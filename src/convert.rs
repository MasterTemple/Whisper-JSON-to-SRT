use crate::{
    srt::{SrtFile, SrtSegment, SrtTime},
    whisper::{UnalignedWhisperXFile, UnalignedWhisperXSegment},
};

impl SrtTime {
    pub fn from_seconds(seconds: f64) -> Self {
        let total_millis = (seconds * 1000.0).round() as u64;

        let hours = (total_millis / 3_600_000) as u32;
        let minutes = ((total_millis % 3_600_000) / 60_000) as u32;
        let seconds = ((total_millis % 60_000) / 1000) as u32;
        let millis = (total_millis % 1000) as u32;

        Self {
            hours,
            minutes,
            seconds,
            millis,
        }
    }
}

impl UnalignedWhisperXSegment {
    pub fn into_srt_segment(self, idx: usize) -> SrtSegment {
        SrtSegment {
            id: (idx + 1) as u32,
            start: SrtTime::from_seconds(self.start),
            end: SrtTime::from_seconds(self.end),
            text: self.text,
        }
    }
}

impl Into<SrtFile> for UnalignedWhisperXFile {
    fn into(self) -> SrtFile {
        SrtFile {
            segments: self
                .segments
                .into_iter()
                .enumerate()
                .map(|(idx, seg)| seg.into_srt_segment(idx))
                .collect(),
        }
    }
}

impl UnalignedWhisperXFile {
    pub fn to_srt(self) -> SrtFile {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::srt::SrtTime;

    #[test]
    fn srt_time_from_seconds() {
        assert_eq!(
            SrtTime::from_seconds(0.031).to_string().as_str(),
            "00:00:00,031"
        );
        assert_eq!(
            SrtTime::from_seconds(29.967).to_string().as_str(),
            "00:00:29,967"
        );

        assert_eq!(
            SrtTime::from_seconds(58.756).to_string().as_str(),
            "00:00:58,756"
        );
        assert_eq!(
            SrtTime::from_seconds(87.983).to_string().as_str(),
            "00:01:27,983"
        );

        assert_eq!(
            SrtTime::from_seconds(2992.998).to_string().as_str(),
            "00:49:52,998"
        );
        assert_eq!(
            SrtTime::from_seconds(3004.557).to_string().as_str(),
            "00:50:04,557"
        );

        assert_eq!(
            SrtTime::from_seconds(8073.335).to_string().as_str(),
            "02:14:33,335"
        );
        assert_eq!(
            SrtTime::from_seconds(8089.552).to_string().as_str(),
            "02:14:49,552"
        );
    }
}
