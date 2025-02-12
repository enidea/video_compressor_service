use shared::app;
use strum::IntoEnumIterator;

pub struct CommandPrompter;

impl CommandPrompter {
    pub fn prompt() -> anyhow::Result<app::Command> {
        let commands = app::Command::iter().collect::<Vec<app::Command>>();

        let selection = dialoguer::Select::new()
            .with_prompt("Select a command")
            .items(&commands)
            .interact()?;

        Ok(commands[selection])
    }
}
