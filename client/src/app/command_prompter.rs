use chrono::Timelike;
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
            app::Command::Clip { .. } => app::Command::Clip {
                clip_range: Self::prompt_clip_range()?,
                media_type: Self::prompt_media_type_for_clip()?,
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

    fn prompt_clip_range() -> anyhow::Result<app::ClipRange> {
        let start_time = dialoguer::Input::<String>::new()
            .with_prompt(
                "Enter start time (HH:mm:ss e.g. 01:23:45, mm:ss e.g. 12:34, or seconds e.g. 3600)",
            )
            .interact()?;

        let end_time = dialoguer::Input::<String>::new()
            .with_prompt(
                "Enter end time (HH:mm:ss e.g. 01:23:45, mm:ss e.g. 12:34, or seconds e.g. 3600)",
            )
            .interact()?;

        app::ClipRange::new(Self::parse_time(&start_time)?, Self::parse_time(&end_time)?)
    }

    fn parse_time(input: &str) -> anyhow::Result<u32> {
        if let Ok(seconds) = input.parse::<u32>() {
            return Ok(seconds);
        }

        if let Ok(time) = chrono::NaiveTime::parse_from_str(input, "%H:%M:%S")
            .or_else(|_| chrono::NaiveTime::parse_from_str(&format!("00:{}", input), "%H:%M:%S"))
        {
            return Ok(time.num_seconds_from_midnight());
        }

        anyhow::bail!("Invalid time format")
    }

    fn prompt_media_type_for_clip() -> anyhow::Result<app::MediaTypeForClip> {
        let media_types = app::MediaTypeForClip::iter().collect::<Vec<_>>();

        let selection = dialoguer::Select::new()
            .with_prompt("Select a media type")
            .items(&media_types)
            .interact()?;

        Ok(media_types[selection])
    }
}

#[cfg(test)]
mod tests {
    use super::CommandPrompter;

    fn assert_parse_time(input: &str, expected: u32) {
        assert_eq!(CommandPrompter::parse_time(input).unwrap(), expected);
    }

    fn assert_parse_time_error(input: &str) {
        assert!(CommandPrompter::parse_time(input).is_err());
    }

    #[test]
    fn test_parse_time_valid_inputs() {
        assert_parse_time("01:23:45", 5025);
        assert_parse_time("12:34", 754);
        assert_parse_time("3600", 3600);
        assert_parse_time("01:01:01", 3661);
        assert_parse_time("1:1:1", 3661);
        assert_parse_time("01:01", 61);
        assert_parse_time("1:01", 61);
        assert_parse_time("01", 1);
        assert_parse_time("00:00:00", 0);
        assert_parse_time("00:00", 0);
        assert_parse_time("0", 0);
        assert_parse_time("59", 59);
        assert_parse_time("59:59", 3599);
        assert_parse_time("23:59:59", 86399);
    }

    #[test]
    fn test_parse_time_invalid_inputs() {
        assert_parse_time_error("abc");
        assert_parse_time_error("01:23:45:67");
        assert_parse_time_error(":30");
        assert_parse_time_error("1::30");
        assert_parse_time_error("24:00:00");
    }
}
