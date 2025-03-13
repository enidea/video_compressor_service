mod app;
mod ffmpeg;

fn main() -> anyhow::Result<()> {
    app::run()
}
