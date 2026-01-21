mod clock;
use crate::clock::app::Clock;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    Clock::new().runner(terminal)?;
    ratatui::restore();
    Ok(())
}
