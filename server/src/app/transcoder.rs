use anyhow::{Context, Ok};
use ffmpeg_next::{codec, decoder, encoder, format, frame, picture, Dictionary, Packet, Rational};

use super::transcoder_options::TranscoderOptions;

pub struct Transcoder {
    ost_index: usize,
    decoder: decoder::Video,
    input_time_base: Rational,
    encoder: encoder::Video,
}

impl Transcoder {
    const DEFAULT_FORMAT: format::Pixel = format::Pixel::YUV420P;
    const DEFAULT_PRESET: &str = "medium";
    const DEFAULT_CRF: &str = "23";

    pub fn new(
        ist: &format::stream::Stream,
        octx: &mut format::context::Output,
        ost_index: usize,
        transcoder_options: &TranscoderOptions,
    ) -> anyhow::Result<Self> {
        let global_header = octx.format().flags().contains(format::Flags::GLOBAL_HEADER);
        let decoder = codec::Context::from_parameters(ist.parameters())?
            .decoder()
            .video()?;

        // codec is an algorithm for encoding (compressing) and decoding (decompressing) video and audio data.
        let codec =
            encoder::find(codec::Id::H264).ok_or(anyhow::anyhow!("H.264 codec not found"))?;

        let mut ost = octx.add_stream(codec)?;

        let mut encoder = codec::Context::new_with_codec(codec).encoder().video()?;

        ost.set_parameters(&encoder);
        encoder.set_height(decoder.height());
        encoder.set_width(decoder.width());
        encoder.set_aspect_ratio(decoder.aspect_ratio());
        encoder.set_format(Self::DEFAULT_FORMAT);
        encoder.set_frame_rate(decoder.frame_rate());
        encoder.set_time_base(ist.time_base());
        encoder.set_bit_rate(transcoder_options.bitrate.unwrap_or(decoder.bit_rate()));

        if global_header {
            encoder.set_flags(codec::Flags::GLOBAL_HEADER);
        }

        let mut dict = Dictionary::new();

        dict.set(
            "preset",
            transcoder_options
                .preset
                .as_deref()
                .unwrap_or(Self::DEFAULT_PRESET),
        );

        let opened_encoder = encoder.open_with(dict)?;

        ost.set_parameters(&opened_encoder);

        Ok(Self {
            ost_index,
            decoder,
            input_time_base: ist.time_base(),
            encoder: opened_encoder,
        })
    }

    pub fn send_packet_to_decoder(&mut self, packet: &Packet) -> anyhow::Result<()> {
        self.decoder
            .send_packet(packet)
            .context("Error sending packet to decoder")
    }

    pub fn send_eof_to_decoder(&mut self) -> anyhow::Result<()> {
        self.decoder
            .send_eof()
            .context("Error sending EOF to decoder")
    }

    pub fn receive_and_process_decoded_frames(
        &mut self,
        octx: &mut format::context::Output,
        ost_time_base: Rational,
    ) -> anyhow::Result<()> {
        let mut frame = frame::Video::empty();

        while self.decoder.receive_frame(&mut frame).is_ok() {
            let timestamp = frame.timestamp();
            frame.set_pts(timestamp);
            frame.set_kind(picture::Type::None);
            self.send_frame_to_encoder(&frame)?;
            self.receive_and_process_encoded_packets(octx, ost_time_base)?;
        }

        Ok(())
    }

    pub fn send_frame_to_encoder(&mut self, frame: &frame::Video) -> anyhow::Result<()> {
        self.encoder
            .send_frame(frame)
            .context("Error sending frame to encoder")
    }

    pub fn send_eof_to_encoder(&mut self) -> anyhow::Result<()> {
        self.encoder
            .send_eof()
            .context("Error sending EOF to encoder")
    }

    pub fn receive_and_process_encoded_packets(
        &mut self,
        octx: &mut format::context::Output,
        ost_time_base: Rational,
    ) -> anyhow::Result<()> {
        let mut packet = Packet::empty();

        while self.encoder.receive_packet(&mut packet).is_ok() {
            packet.set_stream(self.ost_index as _);
            packet.rescale_ts(self.input_time_base, ost_time_base);
            packet.write_interleaved(octx)?;
        }

        Ok(())
    }
}
