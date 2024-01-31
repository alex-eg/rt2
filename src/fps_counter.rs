use std::time::Instant;

use std::collections::VecDeque;

pub struct FpsCounter {
    previous: Instant,
    times: VecDeque<u128>,
}

impl FpsCounter {
    pub fn new(span: usize) -> FpsCounter {
        FpsCounter {
            previous: Instant::now(),
            times: VecDeque::with_capacity(span),
        }
    }

    pub fn restart(&mut self) {
        self.previous = Instant::now();
    }

    pub fn update(&mut self) {
        if self.times.capacity() == self.times.len() {
            self.times.pop_front();
        }

        self.times.push_back(self.previous.elapsed().as_millis());
        self.previous = Instant::now();
    }

    pub fn fps(&self) -> f32 {
        let mut acc: u128 = 0;
        for t in &self.times {
            acc += *t;
        }
        1000. / (acc as f32 / self.times.len() as f32)
    }
}
