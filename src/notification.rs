use std::time::Duration;

use notify_rust::{Notification, Timeout};

use crate::app::PomodoroPhase;

pub fn send_phase_notification(phase: &PomodoroPhase) {
    let (summary,  body) = match phase {
        PomodoroPhase::Work => ("Break Over", "Time to get back to work!"),
        PomodoroPhase::ShortBreak => ("Pomodoro Finished", "Time to take a break!"),
        PomodoroPhase::LongBreak => ("Pomodoro Finished", "Time for an extra-long break! You deserve it!")
    };
    
    match Notification::new().summary(summary).body(body).timeout(Timeout::Milliseconds(5000)).show() {
        Ok(_) => (),
        Err(e) => eprintln!("Can't send a notification: {}", e)
    }
}

pub fn send_reminder(paused_time: &Duration) {
    let paused_humantime = humantime::format_duration(*paused_time).to_string();
    let body = format!{"Your timer has been paused for {paused_humantime}."};

    match Notification::new().summary("Keep going!").body(&body).timeout(Timeout::Milliseconds(5000)).show() {
        Ok(_) => (),
        Err(e) => eprintln!("Can't send a notification: {}", e)
    }
}