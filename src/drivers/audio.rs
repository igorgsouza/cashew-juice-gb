pub struct Driver {}

impl Driver {
    pub fn new(sample_rate: u32) -> Driver {
        Driver {}
    }

    pub fn clear(&self) {}

    pub fn set_rates(&self, clocks_per_second: f64, sample_rate: f64) {}

    pub fn read_samples(&mut self, buf: &mut [i16], stereo: bool) -> usize {
        0
    }

    pub fn samples_avail(&self) -> u32 {
        0
    }

    pub fn end_frame(&self, time: u32) {}

    pub fn add_delta(&self, clock_time: u32, delta: i32) {}
}
