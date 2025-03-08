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

        Ok(match commands[selection] {
            app::Command::Resize { resolution: _ } => app::Command::Resize {
                resolution: Self::prompt_resolution()?,
            },
            _ => commands[selection],
        })
    }

    pub fn prompt_resolution() -> anyhow::Result<app::Resolution> {
        let resolutions = app::Resolution::iter().collect::<Vec<app::Resolution>>();

        let selection = dialoguer::Select::new()
            .with_prompt("Select a resolution")
            .items(&resolutions)
            .interact()?;

        Ok(resolutions[selection])
    }
}
