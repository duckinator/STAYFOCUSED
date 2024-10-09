use std::time::{Duration, Instant};

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct Task {
    pub elapsed_time: Duration,
    pub description: String,
    pub note: String,
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

    pub fn is_tracking_time(&self) -> bool {
        self.start_instant.is_some()
    }

    pub fn tick(&mut self) {
        if let Some(start_instant) = self.start_instant {
            let now = Instant::now();
            let difference = now.duration_since(start_instant);

            // Mostly just so it's not updating every fucking frame.
            if difference.as_secs() < 1 {
                return;
            }

            self.elapsed_time += difference;

            self.start_instant = Some(now);
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_functionality() {
        let mut task = Task::default();
        assert_eq!("".to_string(), task.description);
        task.description.push_str("test task");

        assert_eq!("test task".to_string(), task.description);
        assert_eq!(Duration::new(0, 0), task.elapsed_time);
    }

    #[test]
    fn task_tick() {
        let mut task = Task::default();

        let five_secs_ago = Instant::now() - Duration::new(5, 0);
        task.start_instant = Some(five_secs_ago);

        task.tick();

        // Time occured, so look for 5-6 seconds instead of exactly 5 seconds.
        assert!(Duration::new(5, 0) <= task.elapsed_time);
        assert!(task.elapsed_time <= Duration::new(6, 0));
    }

    #[test]
    fn time_tracking_functionality() {
        let five_secs = Duration::new(5, 0);

        let mut task = Task::default();
        assert!(task.elapsed_time == Duration::new(0, 0));

        assert_eq!(false, task.is_tracking_time());
        task.start();

        assert_eq!(true, task.is_tracking_time());

        // Go 5 seconds into the past.
        task.start_instant = Some(task.start_instant.unwrap() - five_secs);

        task.stop();
        assert_eq!(false, task.is_tracking_time());

        assert!(task.elapsed_time >= five_secs);
    }
}
