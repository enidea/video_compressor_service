use std::{collections::HashMap, path::Path};

use anyhow::Ok;
use ffmpeg_next::{codec, encoder, media};
use ffmpeg_next::{format, Rational};
use shared::app;

use super::transcoder::Transcoder;

pub struct CommandProcessor;

impl CommandProcessor {
    pub fn process(
        command: app::Command,
        input_file_path: &Path,
        output_file_path: &Path,
    ) -> anyhow::Result<()> {
        ffmpeg_next::init()?;

        match command {
            app::Command::Compress => Self::compress(input_file_path, output_file_path),
            app::Command::Resize => {
                println!("Resizing file...");
                Ok(())
            }
            app::Command::ChangeAspectRatio => {
                println!("Changing aspect ratio...");
                Ok(())
            }
            app::Command::ConvertToAudio => {
                println!("Converting file to audio...");
                Ok(())
            }
            app::Command::ConvertToGifOrWebmWithTimeRange => {
                println!("Converting file to GIF or WebM with time range...");
                Ok(())
            }
        }
    }

    fn compress(input_file_path: &Path, output_file_path: &Path) -> anyhow::Result<()> {
        let mut ictx = format::input(&input_file_path)?;
        let mut octx = format::output(&output_file_path)?;

        format::context::input::dump(&ictx, 0, input_file_path.to_str());

        let mut stream_mapping: Vec<isize> = vec![0; ictx.nb_streams() as _];
        let mut ist_time_bases = vec![Rational(0, 0); ictx.nb_streams() as _];
        let mut ost_time_bases = vec![Rational(0, 0); ictx.nb_streams() as _];
        let mut transcoders = HashMap::new();
        let mut ost_index = 0;

        for (ist_index, ist) in ictx.streams().enumerate() {
            let ist_medium = ist.parameters().medium();

            if ist_medium != media::Type::Audio
                && ist_medium != media::Type::Video
                && ist_medium != media::Type::Subtitle
            {
                stream_mapping[ist_index] = -1;
                continue;
            }

            stream_mapping[ist_index] = ost_index;
            ist_time_bases[ist_index] = ist.time_base();

            if ist_medium == media::Type::Video {
                transcoders.insert(ist_index, Transcoder::new(&ist, &mut octx, ost_index as _)?);
            } else {
                let mut ost = octx.add_stream(encoder::find(codec::Id::None))?;
                ost.set_parameters(ist.parameters());

                unsafe {
                    (*ost.parameters().as_mut_ptr()).codec_tag = 0;
                }
            }

            ost_index += 1;
        }

        octx.set_metadata(ictx.metadata().to_owned());
        format::context::output::dump(&octx, 0, output_file_path.to_str());
        octx.write_header()?;

        for (ost_index, _) in octx.streams().enumerate() {
            ost_time_bases[ost_index] = octx
                .stream(ost_index as _)
                .ok_or(anyhow::anyhow!("Could not get stream"))?
                .time_base();
        }

        for (stream, mut packet) in ictx.packets() {
            let ist_index = stream.index();
            let ost_index = stream_mapping[ist_index];
            if ost_index < 0 {
                continue;
            }
            let ost_time_base = ost_time_bases[ost_index as usize];
            match transcoders.get_mut(&ist_index) {
                Some(transcoder) => {
                    transcoder.send_packet_to_decoder(&packet)?;
                    transcoder.receive_and_process_decoded_frames(&mut octx, ost_time_base)?;
                }
                None => {
                    packet.rescale_ts(ist_time_bases[ist_index], ost_time_base);
                    packet.set_position(-1);
                    packet.set_stream(ost_index as _);
                    packet.write_interleaved(&mut octx)?;
                }
            }
        }

        for (ost_index, transcoder) in transcoders.iter_mut() {
            let ost_time_base = ost_time_bases[*ost_index];
            transcoder.send_eof_to_decoder()?;
            transcoder.receive_and_process_decoded_frames(&mut octx, ost_time_base)?;
            transcoder.send_eof_to_encoder()?;
            transcoder.receive_and_process_encoded_packets(&mut octx, ost_time_base)?;
        }

        octx.write_trailer()?;

        Ok(())
    }
}
