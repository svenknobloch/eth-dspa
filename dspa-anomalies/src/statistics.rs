use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct RollingStatistic {
    queue: VecDeque<f32>,
    mean: f32,
    running_variance: f32,
}

impl RollingStatistic {
    pub fn new(size: usize) -> Self {
        RollingStatistic {
            queue: VecDeque::with_capacity(size),
            mean: 0f32,
            running_variance: 0f32,
        }
    }

    pub fn update(&mut self, value: f32) {
        if self.queue.len() < self.queue.capacity() {
            self.queue.push_back(value);
            let delta = value - self.mean;
            self.mean += delta / self.queue.len() as f32;
            self.running_variance += delta * (value - self.mean);
        } else {
            let old = self.queue.pop_front().unwrap();
            let old_mean = self.mean;
            self.queue.push_back(value);
            self.mean += (value - old) / self.queue.capacity() as f32;
            self.running_variance += (value + old - old_mean - self.mean) * (value - old);
        }
    }

    pub fn saturated(&self) -> bool {
        self.queue.capacity() == self.queue.len()
    }

    #[inline]
    pub fn mean(&self) -> f32 {
        self.mean
    }

    #[inline]
    pub fn variance(&self) -> f32 {
        self.running_variance / self.queue.len().max(1) as f32
    }

    #[inline]
    pub fn stddev(&self) -> f32 {
        self.variance().sqrt()
    }

    pub fn is_anomaly(&self, threshold: f32, value: f32) -> bool {
        let lower_bound = self.mean - threshold * self.stddev();
        let upper_bound = self.mean + threshold * self.stddev();

        value < lower_bound || upper_bound < value
    }
}

#[derive(Clone, Debug)]
pub struct OnlineStatistic {
    min_count: usize,
    count: usize,
    mean: f32,
    running_variance: f32,
}

impl OnlineStatistic {
    pub fn new(min_count: usize) -> Self {
        OnlineStatistic {
            min_count: min_count,
            count: 0,
            mean: 0f32,
            running_variance: 0f32,
        }
    }

    pub fn update(&mut self, value: f32) {
        self.count += 1;
        let delta = value - self.mean;
        self.mean += delta / self.count as f32;
        self.running_variance += delta * (value - self.mean);
    }

    pub fn mean(&self) -> f32 {
        self.mean
    }

    #[inline]
    pub fn variance(&self) -> f32 {
        self.running_variance / (self.count.max(1) as f32)
    }

    #[inline]
    pub fn stddev(&self) -> f32 {
        self.variance().sqrt()
    }

    #[inline]
    pub fn saturated(&self) -> bool {
        self.count >= self.min_count
    }

    pub fn is_anomaly(&self, threshold: f32, value: f32) -> Option<f32> {
        let stddevs = (value - self.mean) / self.stddev();

        if stddevs.abs() > threshold {
            Some(stddevs)
        } else {
            None
        }
    }
}
