pub struct Duration(pub(crate) u64);

impl Duration {
    pub fn ticks(ticks: u64) -> Duration {
        Duration(ticks)
    }

    pub fn seconds(seconds: u64) -> Duration {
        Duration(seconds * 20)
    }

    pub fn minutes(minutes: u64) -> Duration {
        Duration(minutes * 20 * 60)
    }
}