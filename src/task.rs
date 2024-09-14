use std::time::{Duration, Instant};

// [ pri ] [ duration ] [ task ] [ Edit ] [ Delete ]
#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct Task {
    //priority: Priority,
    pub elapsed_time: Duration,
    pub description: String,
    #[serde(skip)]
    pub start_instant: Option<Instant>,
}

impl Task {
    pub fn start(&mut self) {
        if self.start_instant.is_none() {
            self.start_instant = Some(Instant::now());
        }
    }

    pub fn stop(&mut self) {
        if self.start_instant.is_some() {
            self.tick();
            self.start_instant = None;
        }
    }

    pub fn tick(&mut self) {
        if self.start_instant.is_none() {
            return;
        }

        let now = Instant::now();
        let difference = now.duration_since(self.start_instant.unwrap());

        // Mostly just so it's not updating every fucking frame.
        if difference.as_secs() < 1 {
            return;
        }

        self.elapsed_time += difference;

        self.start_instant = Some(now);
    }

    pub fn elapsed_time_str(&self) -> String {
        let time = self.elapsed_time.as_secs();
        let mins = time / 60;
        let secs = time % 60;

        let hours = mins / 60;
        let mins = mins % 60;

        format!("{:02}:{:02}:{:02}", hours, mins, secs)
    }
}
