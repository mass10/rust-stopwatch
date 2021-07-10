// mod lib;

fn test_my_stopwatch(try_count: u32) {
	// println!("[TRACE] $$$ BEGIN 1:rust_stopwatch $$$");
	let stopwatch = rust_stopwatch::Stopwatch::new();
	for _ in 1..try_count {
		// if 0 < i {
		// 	std::thread::sleep(std::time::Duration::from_millis(1));
		// }
		let _text = format!("[TRACE] elapsed: {}", stopwatch);
		// println!("{}", _text);
	}
	println!("[TRACE] TRY:1 rust_stopwatch {}", stopwatch.as_duration().as_secs_f32());
}

fn test_rust_stopwatch_crate(try_count: u32) {
	extern crate stopwatch;
	// println!("[TRACE] $$$ BEGIN 2:stopwatch $$$");
	let stopwatch = stopwatch::Stopwatch::start_new();
	for _ in 1..try_count {
		// if 0 < i {
		// 	std::thread::sleep(std::time::Duration::from_millis(1));
		// }
		let _text = format!("[TRACE] elapsed: {}", stopwatch);
		// println!("{}", _text);
	}
	println!("[TRACE] TRY:2 stopwatch {}", stopwatch.elapsed().as_secs_f32());
}

fn do_try(f: fn(try_count: u32), try_count: u32, count: u32) {
	for _ in 1..count {
		f(try_count);
	}
}

/// エントリーポイント
fn main() {
	// 自分の crate
	do_try(test_my_stopwatch, 10000, 100);
	// 既存の crate
	do_try(test_rust_stopwatch_crate, 10000, 100);
}
