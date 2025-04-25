use std::{fmt::Display, fs, path::Path};

#[derive(Debug)]
pub struct SrtTime {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub millis: u32,
}

impl SrtTime {
    pub fn in_seconds(&self) -> f64 {
        let seconds = (self.hours * 60 * 60 + self.minutes * 60 + self.seconds) as f64;
        let millis = self.millis as f64 / 1000.0;
        seconds + millis
    }
}

impl Display for SrtTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02},{:03}",
            self.hours, self.minutes, self.seconds, self.millis
        )
    }
}

#[derive(Debug)]
pub struct SrtSegment {
    pub id: u32,
    pub start: SrtTime,
    pub end: SrtTime,
    pub text: String,
}

impl Display for SrtSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{} --> {}\n{}\n\n",
            self.id, self.start, self.end, self.text
        )
    }
}

pub struct SrtFile {
    pub segments: Vec<SrtSegment>,
}

impl Display for SrtFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for seg in &self.segments {
            write!(f, "{}", seg)?;
        }
        Ok(())
    }
}

impl SrtFile {
    pub fn write_to(&self, path: impl AsRef<Path>) -> crate::Result<()> {
        let contents = self.to_string();
        fs::write(path, contents)?;
        Ok(())
    }
}
