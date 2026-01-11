pub struct ThreadSchedulingData {
    virtual_time: i64,
}
pub struct ProcessSchedulingData {}
impl Default for ThreadSchedulingData {
    fn default() -> Self {
        Self { virtual_time: 0 }
    }
}
impl Default for ProcessSchedulingData {
    fn default() -> Self {
        Self {}
    }
}
