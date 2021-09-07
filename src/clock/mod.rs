use crate::config::Config;

pub struct ClockResult {
    pub total_elapsed: f32,
    pub last_period: f32,
    pub frame_count: u32,
}

pub trait Clock {
    fn init(config: &Config) -> Self;
    fn update(&mut self);
    fn current(&self) -> ClockResult;
}

pub struct PrintClock {
    rate: std::time::Duration,
    time_elapsed: std::time::Duration,
    pub frame_count: u32,
}

pub struct RenderClock {
    start: std::time::Instant,
    last_render_time: std::time::Instant,
    last_period: std::time::Duration,
    pub frame_count: u32,
}

impl Clock for RenderClock {
    fn init(_config: &Config) -> Self {
        Self {
            start: std::time::Instant::now(),
            last_render_time: std::time::Instant::now(),
            last_period: std::time::Duration::ZERO,
            frame_count: 0,
        }
    }
    fn update(&mut self) {
        let now = std::time::Instant::now();
        let dt = now - self.last_render_time;
        self.last_period = dt;
        self.last_render_time = now;
        self.frame_count += 1;
    }

    fn current(&self) -> ClockResult {
        ClockResult {
            last_period: self.last_period.as_secs_f32(),
            total_elapsed: (self.last_render_time - self.start).as_secs_f32(),
            frame_count: self.frame_count,
        }
    }
}

impl Clock for PrintClock {
    fn init(_config: &Config) -> Self {
        Self {
            rate: std::time::Duration::from_millis(100),
            time_elapsed: std::time::Duration::ZERO,
            frame_count: 0,
        }
    }
    fn update(&mut self) {
        self.time_elapsed += self.rate;
        self.frame_count += 1;
    }

    fn current(&self) -> ClockResult {
        if self.frame_count % 100 == 0 {
            println!("{:?}", self.time_elapsed);
        }
        ClockResult {
            last_period: self.rate.as_secs_f32(),
            total_elapsed: self.time_elapsed.as_secs_f32(),
            frame_count: self.frame_count,
        }
    }
}
