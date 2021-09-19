use crate::config::Config;

#[derive(Copy, Clone, Debug)]
pub struct ClockResult {
    pub total_elapsed: f32,
    pub last_period: f32,
    pub frame_count: u32,
}

pub trait Clock {
    fn init(config: &Config) -> Self;
    fn update(&mut self);
    fn current(&self) -> ClockResult;
    fn play(&mut self);
    fn pause(&mut self);
    fn set_playing(&mut self, play: bool);
    fn is_playing(&self) -> bool;
}

pub struct PrintClock {
    rate: std::time::Duration,
    time_elapsed: std::time::Duration,
    pub frame_count: u32,
    pub playing: bool,
}

pub struct RenderClock {
    pub total_elapsed: std::time::Duration,
    last_render_time: std::time::Instant,
    last_period: std::time::Duration,
    pub frame_count: u32,
    pub playing: bool,
}

impl Clock for RenderClock {
    fn init(_config: &Config) -> Self {
        Self {
            total_elapsed: std::time::Duration::ZERO,
            last_render_time: std::time::Instant::now(),
            last_period: std::time::Duration::ZERO,
            frame_count: 0,
            playing: false,
        }
    }
    fn update(&mut self) {
        let now = std::time::Instant::now();
        let dt = now - self.last_render_time;
        if self.is_playing() {
            self.total_elapsed += dt;
        }
        self.last_period = dt;
        self.last_render_time = now;
        self.frame_count += 1;
    }

    fn current(&self) -> ClockResult {
        ClockResult {
            last_period: self.last_period.as_secs_f32(),
            total_elapsed: self.total_elapsed.as_secs_f32(),
            frame_count: self.frame_count,
        }
    }

    fn play(&mut self) {
        self.last_render_time = std::time::Instant::now();
        self.playing = true;
    }

    fn pause(&mut self) {
        self.playing = false;
    }

    fn is_playing(&self) -> bool {
        self.playing
    }

    fn set_playing(&mut self, play: bool) {
        self.playing = play
    }
}

impl Clock for PrintClock {
    fn init(_config: &Config) -> Self {
        Self {
            rate: std::time::Duration::from_millis(20),
            time_elapsed: std::time::Duration::ZERO,
            frame_count: 0,
            playing: false,
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

    fn play(&mut self) {
        self.playing = true;
    }

    fn pause(&mut self) {
        self.playing = false;
    }

    fn is_playing(&self) -> bool {
        self.playing
    }

    fn set_playing(&mut self, play: bool) {
        self.playing = play
    }
}
