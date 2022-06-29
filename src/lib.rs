//! A simple timer for sequential events
//!
//! ```rust
//! use std::{ time::Duration, thread::sleep };
//!
//! let mut timer = seq_timer::Timer::new();
//!
//! // starts the first event
//! timer.start("the first event");
//! sleep(Duration::from_millis(1));
//!
//! // finishes the first event and starts the second one
//! // you can also `.finish()` the current event manually
//! timer.start("the second event");
//! sleep(Duration::from_millis(10));
//!
//! // finishes the last event and prints sorted measurments to stdout:
//! timer.print();
//! ```
//! The output would be similar to:
//! ```text
//! the second event | 10078204 ns |  88%
//!  the first event |  1265423 ns |  11%
//! ```
//! The timer also implements [Display](core::fmt::Display), but you must
//! finish the last event manually in this case: `debug!("{}", timer.finish())`
use std::{
    fmt,
    time::{Duration, Instant},
};

#[derive(Default)]
pub struct Timer {
    /// Event names
    names: Vec<&'static str>,
    /// Event durations
    durations: Vec<Duration>,
    /// Current event started
    started: Option<Instant>,
}

impl Timer {
    pub fn new() -> Self {
        Default::default()
    }

    /// Finishes the current event if necessary and starts a new event
    pub fn start(&mut self, name: &'static str) {
        self.finish();
        self.names.push(name);
        self.started = Some(Instant::now());
    }

    /// Finishes the current event
    pub fn finish(&mut self) -> &Self {
        if let Some(started) = self.started {
            self.durations.push(started.elapsed());
            self.started = None;
        }
        self
    }

    /// Finishes the last event and prints the result
    pub fn print(&mut self) {
        self.finish();
        println!("{self}");
    }

    /// Return the length of the longest name
    fn max_name_len(&self) -> usize {
        self.names.iter().map(|x| x.len()).max().unwrap_or_default()
    }

    /// Return the length of the longest name
    fn max_duration_len(&self) -> usize {
        self.durations
            .iter()
            .max()
            .unwrap_or(&Duration::ZERO)
            .as_nanos()
            .to_string()
            .len()
    }

    /// The total time all events took
    fn total_duration(&self) -> Duration {
        self.durations.iter().sum()
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_name = self.max_name_len();
        let max_duration = self.max_duration_len();
        let total_duration = self.total_duration().as_nanos();
        let mut sorted_durations: Vec<_> = self.names.iter().zip(&self.durations).collect();
        sorted_durations.sort_by(|a, b| b.1.cmp(a.1));
        for (name, duration) in sorted_durations {
            let d = duration.as_nanos();
            let percent = d * 100 / total_duration;
            writeln!(
                f,
                "{:>max_name$} | {:>max_duration$} ns | {:>3}%",
                name, d, percent
            )?;
        }
        if self.started.is_some() {
            writeln!(
                f,
                "WARNING: timer event `{}` has not been finished properly, run `Timer::finish()`",
                self.names.iter().last().unwrap_or(&"")
            )?;
        }
        Ok(())
    }
}
