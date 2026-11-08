use crate::app::App;

pub mod app;
pub mod event;
pub mod ui;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let mut app = App::new();
    app.load_folders(std::path::Path::new("."))?;

    let result = app.run(terminal);
    ratatui::restore();
    result
}

