mod tickterm;
use crate::tickterm::app::TickTerm;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    TickTerm::new().runner(terminal)?;
    ratatui::restore();
    Ok(())
}
