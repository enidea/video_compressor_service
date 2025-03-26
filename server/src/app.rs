mod command_processor;
mod tcp_stream_handler;
mod video_transcoder;

use video_transcoder::VideoTranscoder;

pub fn run() -> anyhow::Result<()> {
    VideoTranscoder::run()
}
