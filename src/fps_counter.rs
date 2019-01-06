use time::precise_time_ns;

use std::collections::VecDeque;

pub struct FpsCounter {
    previous: u64,
    times: VecDeque<u32>,
}

impl FpsCounter {
    pub fn new(span: usize) -> FpsCounter {
        FpsCounter {
            previous: 100_000,
            times: VecDeque::with_capacity(span),
        }
    }

    pub fn restart(&mut self) {
        self.previous = precise_time_ns();
    }

    pub fn update(&mut self) {
        if self.times.capacity() == self.times.len() {
            self.times.pop_front();
        }

        let current = precise_time_ns();
        self.times.push_back((current - self.previous) as u32);
        self.previous = current;
    }

    pub fn fps(&self) -> f32 {
        let mut acc: u64 = 0;
        for t in &self.times {
            acc += u64::from(*t);
        }

        1. / (acc as f32 / self.times.len() as f32) * 1_000_000_000.
    }
}
