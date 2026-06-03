mod app;
mod ui;

use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();
    let app_result = app::App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}
