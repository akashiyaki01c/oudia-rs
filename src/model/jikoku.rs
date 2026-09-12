const TOTAL_SECONDS_PER_DAY: usize = 24 * 60 * 60;

#[derive(Debug, Default, PartialEq, Clone, Copy, Eq)]
pub struct Jikoku {
    total_seconds: Option<usize>,
}

impl Jikoku {
    pub fn adjust_total_seconds(self) -> Self {
        Self {
            total_seconds: if let Some(total_seconds) = self.total_seconds {
                Some(total_seconds % TOTAL_SECONDS_PER_DAY)
            } else {
                None
            },
        }
    }
}
