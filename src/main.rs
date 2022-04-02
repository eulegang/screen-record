use clap::Parser;
use eyre::ContextCompat;

mod cli;
mod ffmpeg;
mod xrandr;

fn main() -> eyre::Result<()> {
    let cli = cli::Cli::parse();

    let screens = xrandr::Screen::list().unwrap();

    let screen = if let Some(display) = cli.display {
        screens
            .iter()
            .find(|screen| screen.name == display)
            .wrap_err(format!("screen '{}' not found", &display))?
    } else {
        screens.first().wrap_err("no screens detected")?
    };

    let mut record = ffmpeg::Record::default();

    let mut screen_grab = ffmpeg::ScreenGrab::default();
    screen_grab.size = Some(screen.dim);
    screen_grab.offset = Some(screen.offset);
    screen_grab.framerate = cli.framerate;
    record.push(screen_grab);

    let output = cli.output.wrap_err("need path for right now")?;
    record.push(ffmpeg::FileOutput { path: &output });

    if !record.run()? {
        eyre::bail!("failed to record");
    }

    Ok(())
}
