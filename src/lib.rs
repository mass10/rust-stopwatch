//! A simple stopwatch for Rust.

/// Custom formatter for std::time::Duration
trait FormatterForDuration {
	/// Format elapsed time.
	fn to_string(&self) -> String;
}

impl FormatterForDuration for std::time::Duration {
	fn to_string(&self) -> String {
		let mut millis = self.as_millis();
		let mut sec = 0;
		let mut min = 0;
		let mut hour = 0;
		while 1000 <= millis {
			sec += 1;
			millis -= 1000;
		}
		while 60 <= sec {
			min += 1;
			sec -= 60;
		}
		while 60 <= min {
			hour += 1;
			min -= 60;
		}
		if 10 <= hour {
			let s = format!("{}:{:02}:{:02}:{:03}", hour, min, sec, millis);
			return s;
		} else {
			let s = format!("{:02}:{:02}:{:02}:{:03}", hour, min, sec, millis);
			return s;
		}
	}
	// fn to_string(&self) -> String {
	// 	let millis = self.as_millis();
	// 	let sec = millis / 1000;
	// 	let millis = millis % 1000;
	// 	let min = sec / 60;
	// 	let sec = sec % 60;
	// 	let hour = min / 60;
	// 	let sec = sec % 60;
	// 	if 10 <= hour {
	// 		let s = format!("{}:{:02}:{:02}:{:03}", hour, min, sec, millis);
	// 		return s;
	// 	} else {
	// 		let s = format!("{:02}:{:02}:{:02}:{:03}", hour, min, sec, millis);
	// 		return s;
	// 	}
	// }
}

/// Stopwatch
///
/// Displays in default style "hh:mm:ss.SSS".
pub struct Stopwatch {
	time: std::time::Instant,
}

impl Stopwatch {
	/// Return a new instance.
	pub fn new() -> Stopwatch {
		return Stopwatch { time: std::time::Instant::now() };
	}

	/// As seconds.
	pub fn as_seconds_short(&self) -> f64 {
		let duration = std::time::Instant::now() - self.time;
		let secs = duration.as_secs();
		let millis = duration.subsec_millis();
		let total = secs * 1000 + millis as u64;
		return total as f64 / 1000.0;
	}

	/// Returns duration from start.
	pub fn as_duration(&self) -> std::time::Duration {
		return std::time::Instant::now() - self.time;
	}

	/// Returns string formatted in default style "hh24:mm:ss.SSS".
	pub fn to_string(&self) -> String {
		let duration = std::time::Instant::now() - self.time;
		return duration.to_string();
	}
}

impl std::fmt::Display for Stopwatch {
	/// Format elapsed time.
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let elapsed = std::time::Instant::now() - self.time;
		write!(f, "{}", elapsed.to_string())?;
		return Ok(());
	}
}
