use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct TimeConfig {
    pub milliseconds: u64,
}

impl TimeConfig {
    pub fn from_str(time: &str) -> Result<Self, String> {
        let time = time.trim();
        if time.is_empty() {
            return Err("Time argument cannot be an empty string".into());
        }
        let first = time.chars().next().unwrap();
        if !first.is_ascii_digit() {
            return Err(format!("Time must start with an integer, was '{first}'"));
        }

        let mut time_end = 0;
        for ch in time.chars() {
            if !ch.is_ascii_digit() {
                break;
            }
            time_end += 1;
        }

        let mut unit_start = time_end;
        for ch in time[unit_start..].chars() {
            if !ch.is_whitespace() {
                break;
            }
            unit_start += 1;
        }

        // TODO: fix unwraps
        let time_value = time[0..time_end].parse::<u64>().unwrap();
        if time_value == 0 {
            return Err("Time cannot be zero.".into());
        }

        let unit_lower = time[unit_start..].to_lowercase();
        let unit_str = unit_lower.as_str();
        let unit = match unit_str {
            // second is 1000 milliseconds
            "s" | "sec" | "second" => 1000,
            // minute is 60_000 milliseconds
            "m" | "min" | "minute" => 60_000,
            // hour is 3_600_000 milliseconds
            "h" | "hour" => 3_600_000,
            _ => {
                return Err(format!(
                "Invalid unit '{unit_str}'. Must be one of s, sec, second, m, min, minute, h, hour"
            ))
            }
        };

        Ok(Self {
            milliseconds: time_value * unit,
        })
    }
}

impl FromStr for TimeConfig {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seconds_ok() {
        let time = TimeConfig::from_str("1s").unwrap();
        assert_eq!(time.milliseconds, 1_000);
        let time = TimeConfig::from_str("1sec").unwrap();
        assert_eq!(time.milliseconds, 1_000);
        let time = TimeConfig::from_str("1second").unwrap();
        assert_eq!(time.milliseconds, 1_000);

        let time = TimeConfig::from_str("1S").unwrap();
        assert_eq!(time.milliseconds, 1_000);
        let time = TimeConfig::from_str("1Sec").unwrap();
        assert_eq!(time.milliseconds, 1_000);
        let time = TimeConfig::from_str("1SeCoNd").unwrap();
        assert_eq!(time.milliseconds, 1_000);

        let time = TimeConfig::from_str("1 S").unwrap();
        assert_eq!(time.milliseconds, 1_000);
        let time = TimeConfig::from_str("1123 sec").unwrap();
        assert_eq!(time.milliseconds, 1123_000);
        let time = TimeConfig::from_str("99     sec").unwrap();
        assert_eq!(time.milliseconds, 99_000);
    }

    #[test]
    fn test_minutes_ok() {
        let time = TimeConfig::from_str("1m").unwrap();
        assert_eq!(time.milliseconds, 60_000);
        let time = TimeConfig::from_str("1min").unwrap();
        assert_eq!(time.milliseconds, 60_000);
        let time = TimeConfig::from_str("1minute").unwrap();
        assert_eq!(time.milliseconds, 60_000);

        let time = TimeConfig::from_str("1M").unwrap();
        assert_eq!(time.milliseconds, 60_000);
        let time = TimeConfig::from_str("1MiN").unwrap();
        assert_eq!(time.milliseconds, 60_000);
        let time = TimeConfig::from_str("1MiNuTe").unwrap();
        assert_eq!(time.milliseconds, 60_000);

        let time = TimeConfig::from_str("1 M").unwrap();
        assert_eq!(time.milliseconds, 60_000);
        let time = TimeConfig::from_str("1123 Minute").unwrap();
        assert_eq!(time.milliseconds, 1123 * 60_000);
        let time = TimeConfig::from_str("99     min").unwrap();
        assert_eq!(time.milliseconds, 99 * 60_000);
    }

    #[test]
    fn test_hours_ok() {
        let time = TimeConfig::from_str("1h").unwrap();
        assert_eq!(time.milliseconds, 60_000 * 60);
        let time = TimeConfig::from_str("1hour").unwrap();
        assert_eq!(time.milliseconds, 60_000 * 60);

        let time = TimeConfig::from_str("1H").unwrap();
        assert_eq!(time.milliseconds, 60_000 * 60);
        let time = TimeConfig::from_str("1HouR").unwrap();
        assert_eq!(time.milliseconds, 60_000 * 60);

        let time = TimeConfig::from_str("1 H").unwrap();
        assert_eq!(time.milliseconds, 60_000 * 60);
        let time = TimeConfig::from_str("1123 hoUr").unwrap();
        assert_eq!(time.milliseconds, 1123 * 60_000 * 60);
        let time = TimeConfig::from_str("99     hour").unwrap();
        assert_eq!(time.milliseconds, 99 * 60_000 * 60);
    }

    #[test]
    fn test_parse_failures() {
        // no empty values
        let time = TimeConfig::from_str("");
        assert!(time.is_err());
        // even with whitespace
        let time = TimeConfig::from_str("   ");
        assert!(time.is_err());
        // no negative values
        let time = TimeConfig::from_str("-1h");
        assert!(time.is_err());
        // unit is required
        let time = TimeConfig::from_str("1");
        assert!(time.is_err());
        // time value is required
        let time = TimeConfig::from_str("minute");
        assert!(time.is_err());
        // no zeros
        let time = TimeConfig::from_str("0minute");
        assert!(time.is_err());
    }
}
