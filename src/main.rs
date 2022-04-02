use clap::Parser;
use eyre::ContextCompat;

mod cli;
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

    dbg!(screen);

    Ok(())
}
