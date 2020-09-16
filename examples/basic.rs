use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use minibar::Progress;

fn main() {
    let stdout = io::stdout();
    let lock = stdout.lock();

    let mut progress = Progress::new(lock, 100);

    for i in 0..=100 {
        let _ = writeln!(
            progress,
            "{n} small step for man, {n} giant leap for mankind",
            n = i
        );

        progress.pos(i);

        let _ = progress.render();
        let _ = progress.render();

        thread::sleep(Duration::from_millis(10));
    }

    let mut lock = progress.finish();

    let _ = writeln!(lock);
}
