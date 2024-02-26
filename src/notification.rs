use notify_rust::{error::Error, Notification, NotificationHandle, Timeout};

use crate::app::PomodoroPhase;

pub fn send_notification(phase: &PomodoroPhase) -> Result<NotificationHandle, Error> {
    let (summary,  body) = match phase {
        PomodoroPhase::Work => ("Break Over", "Time to get back to work!"),
        PomodoroPhase::ShortBreak => ("Pomodoro Finished", "Time to take a break!"),
        PomodoroPhase::LongBreak => ("Pomodoro Finished", "Time for an extra-long break! You deserve it!")
    };
    
    Notification::new().summary(summary).body(body).timeout(Timeout::Milliseconds(5000)).show()
}