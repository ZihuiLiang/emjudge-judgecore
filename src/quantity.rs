use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::{Add, Sub};
use std::time::Duration;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct MemorySize(usize);

impl MemorySize {
    pub fn new(bytes: usize) -> Self {
        MemorySize(bytes)
    }

    pub fn as_bytes(&self) -> usize {
        self.0
    }

    pub fn as_kilobytes(&self) -> usize {
        self.0 / 1024
    }

    pub fn as_megabytes(&self) -> usize {
        self.as_kilobytes() / 1024
    }

    pub fn as_gigabytes(&self) -> usize {
        self.as_megabytes() / 1024
    }
}

impl fmt::Display for MemorySize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 % 1024 == 0 {
            if self.0 % (1024 * 1024) == 0 {
                if self.0 % (1024 * 1024 * 1024) == 0 {
                    write!(f, "{}GB", self.as_gigabytes())
                } else {
                    write!(f, "{}MB", self.as_megabytes())
                }
            } else {
                write!(f, "{}KB", self.as_kilobytes())
            }
        } else {
            write!(f, "{}B", self.0)
        }
    }
}

impl MemorySize {
    pub fn from_bytes(value: usize) -> Self {
        Self::new(value)
    }

    pub fn from_kilobytes(value: usize) -> Self {
        MemorySize::new(value * 1024)
    }

    pub fn from_megabytes(value: usize) -> Self {
        MemorySize::new(value * 1024 * 1024)
    }

    pub fn from_gigabytes(value: usize) -> Self {
        MemorySize::new(value * 1024 * 1024 * 1024)
    }
}

impl Add for MemorySize {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        MemorySize::new(self.0 + other.0)
    }
}

impl Sub for MemorySize {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        if self.0 >= other.0 {
            MemorySize::new(self.0 - other.0)
        } else {
            MemorySize::new(0)
        }
    }
}

impl Serialize for MemorySize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.0 % 1024 == 0 {
            if self.0 % (1024 * 1024) == 0 {
                if self.0 % (1024 * 1024 * 1024) == 0 {
                    serializer.serialize_str(format!("{}GB", self.as_gigabytes()).as_str())
                } else {
                    serializer.serialize_str(format!("{}MB", self.as_megabytes()).as_str())
                }
            } else {
                serializer.serialize_str(format!("{}KB", self.as_kilobytes()).as_str())
            }
        } else {
            serializer.serialize_str(format!("{}B", self.0).as_str())
        }
    }
}

impl<'de> Deserialize<'de> for MemorySize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MemorySizeVisitor;

        impl<'de> serde::de::Visitor<'de> for MemorySizeVisitor {
            type Value = MemorySize;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing MemorySize (e.g., '1024B')")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                // Implement custom parsing logic based on your serialization format
                let (number, unit) =
                    value.split_at(value.trim_end_matches(|c| char::is_alphabetic(c)).len());
                let bytes = match unit.trim() {
                    "B" => 1,
                    "KB" => 1024,
                    "MB" => 1024 * 1024,
                    "GB" => 1024 * 1024 * 1024,
                    _ => return Err(serde::de::Error::custom("Invalid MemorySize unit")),
                };

                if let Ok(value) = number.parse::<usize>() {
                    Ok(MemorySize::new(value * bytes))
                } else {
                    Err(serde::de::Error::custom("Invalid MemorySize format"))
                }
            }
        }
        deserializer.deserialize_str(MemorySizeVisitor)
    }
}

#[derive(Debug, Eq, PartialOrd, Ord, Clone, Copy, Default, PartialEq)]
pub struct TimeSpan(u64);

impl TimeSpan {
    pub fn from_milliseconds(milliseconds: u64) -> Self {
        TimeSpan(milliseconds)
    }

    pub fn from_seconds(seconds: u64) -> Self {
        TimeSpan(seconds * 1_000)
    }

    pub fn from_minutes(minutes: u64) -> Self {
        TimeSpan(minutes * 60 * 1_000)
    }

    pub fn from_hours(hours: u64) -> Self {
        TimeSpan(hours * 60 * 60 * 1_000)
    }

    pub fn as_milliseconds(&self) -> u64 {
        self.0
    }

    pub fn as_seconds(&self) -> u64 {
        self.0 / 1_000
    }

    pub fn as_minutes(&self) -> u64 {
        self.0 / 60 / 1_000
    }

    pub fn as_hours(&self) -> u64 {
        self.0 / 60 / 60 / 1_000
    }
}

impl From<Duration> for TimeSpan {
    fn from(duration: Duration) -> Self {
        TimeSpan(duration.as_millis() as u64)
    }
}

impl From<TimeSpan> for Duration {
    fn from(timespan: TimeSpan) -> Self {
        Duration::from_millis(timespan.0)
    }
}

impl fmt::Display for TimeSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}ms", self.as_milliseconds())
    }
}

impl Serialize for TimeSpan {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.0 % 10 == 0 {
            if self.0 % (10 * 60) == 0 {
                if self.0 % (10 * 60 * 60) == 0 {
                    serializer.serialize_str(format!("{}h", self.as_hours()).as_str())
                } else {
                    serializer.serialize_str(format!("{}m", self.as_minutes()).as_str())
                }
            } else {
                serializer.serialize_str(format!("{}s", self.as_seconds()).as_str())
            }
        } else {
            serializer.serialize_str(format!("{}ms", self.0).as_str())
        }
    }
}

impl<'de> Deserialize<'de> for TimeSpan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeSpanVisitor;

        impl<'de> serde::de::Visitor<'de> for TimeSpanVisitor {
            type Value = TimeSpan; // Change the type to TimeSpan

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(
                    "a string representing TimeSpan (e.g., '60s', '120m', '24h', '500ms')",
                )
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                // Implement custom parsing logic based on your serialization format
                let (number, unit) =
                    value.split_at(value.trim_end_matches(|c| char::is_alphabetic(c)).len());
                let ms = match unit.trim() {
                    "ms" => 1,
                    "s" => 10,
                    "m" => 60 * 10,
                    "h" => 60 * 60 * 10,
                    _ => return Err(serde::de::Error::custom("Invalid TimeSpan unit")),
                };

                if let Ok(value) = number.parse::<u64>() {
                    Ok(TimeSpan::from_milliseconds(value * ms)) // Use TimeSpan::from_milliseconds instead of MemorySize::new
                } else {
                    Err(serde::de::Error::custom("Invalid TimeSpan format"))
                }
            }
        }
        deserializer.deserialize_str(TimeSpanVisitor)
    }
}

impl Add for TimeSpan {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        TimeSpan(self.0 + other.0)
    }
}

impl Sub for TimeSpan {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        TimeSpan(self.0 - other.0)
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProcessResource {
    pub runtime: TimeSpan,
    pub memory: MemorySize,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl ProcessResource {
    pub fn default() -> Self {
        ProcessResource {
            runtime: TimeSpan::default(),
            memory: MemorySize::default(),
            stdout: vec![],
            stderr: vec![],
        }
    }
}

impl fmt::Display for ProcessResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stdout_str = String::from_utf8_lossy(&self.stdout);
        let stderr_str = String::from_utf8_lossy(&self.stderr);

        let stdout_chars: String = stdout_str.chars().take(256).collect();
        let stderr_chars: String = stderr_str.chars().take(256).collect();

        let stdout_escaped: String = stdout_chars
            .bytes()
            .flat_map(std::ascii::escape_default)
            .map(|b| b as char)
            .collect();
        let stderr_escaped: String = stderr_chars
            .bytes()
            .flat_map(std::ascii::escape_default)
            .map(|b| b as char)
            .collect();
        let stdout_escaped = if stdout_str.len() > 256 {
            format!("{}... ({} chars total)", stdout_escaped, stdout_str.len())
        } else {
            stdout_escaped
        };
        let stderr_escaped = if stderr_str.len() > 256 {
            format!("{}... ({} chars total)", stderr_escaped, stderr_str.len())
        } else {
            stderr_escaped
        };

        write!(
            f,
            "Runtime: {}, Memory: {}, Stdout: {}, Stderr: {}",
            self.runtime, self.memory, stdout_escaped, stderr_escaped
        )
    }
}
