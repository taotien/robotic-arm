use iced::{Application, Settings};
use robo_ui::Controls;

fn main() -> anyhow::Result<()> {
    Controls::run(Settings::default())?;

    Ok(())
}
