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
            app::Command::Resize { .. } => app::Command::Resize {
                resolution: Self::prompt_resolution()?,
            },
            app::Command::ChangeAspectRatio { .. } => app::Command::ChangeAspectRatio {
                aspect_ratio: Self::prompt_aspect_ratio()?,
                aspect_ratio_fit: Self::prompt_aspect_ratio_fit()?,
            },
            _ => commands[selection],
        })
    }

    fn prompt_resolution() -> anyhow::Result<app::Resolution> {
        let resolutions = app::Resolution::iter().collect::<Vec<_>>();

        let selection = dialoguer::Select::new()
            .with_prompt("Select a resolution")
            .items(&resolutions)
            .interact()?;

        Ok(resolutions[selection])
    }

    fn prompt_aspect_ratio() -> anyhow::Result<app::AspectRatio> {
        let aspect_ratios = app::AspectRatio::iter().collect::<Vec<_>>();

        let selection = dialoguer::Select::new()
            .with_prompt("Select an aspect ratio")
            .items(&aspect_ratios)
            .interact()?;

        Ok(aspect_ratios[selection])
    }

    fn prompt_aspect_ratio_fit() -> anyhow::Result<app::AspectRatioFit> {
        let aspect_ratio_fits = app::AspectRatioFit::iter().collect::<Vec<_>>();

        let selection = dialoguer::Select::new()
            .with_prompt("Select an aspect ratio fit")
            .items(&aspect_ratio_fits)
            .interact()?;

        Ok(aspect_ratio_fits[selection])
    }
}
