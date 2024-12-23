use std::ffi::c_int;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

use seedcracker::check_seed;
use seedcracker::random::mth::get_mask;

const MAX_THREADS: u64 = 16;

const CHUNK_X: i32 = -28;
const CHUNK_Z: i32 = -73;

pub fn format_num(num: u64) -> String {
    if num < 10_000 {
        format!("{}", num)
    } else if num < 1_000_000 {
        format!("{:.0}k", (num as f64) / 1000.0)
    } else if num < 10_000_000 {
        format!("{:.1}M", (num as f64) / (1000.0 * 1000.0))
    } else if num < 1_000_000_000 {
        format!("{:.0}M", (num as f64) / (1000.0 * 1000.0))
    } else if num < 10_000_000_000 {
        format!("{:.1}B", (num as f64) / (1000.0 * 1000.0 * 1000.0))
    } else if num < 1_000_000_000_000 {
        format!("{:.0}B", (num as f64) / (1000.0 * 1000.0 * 1000.0))
    } else if num < 10_000_000_000_000 {
        format!("{:.1}T", (num as f64) / (1000.0 * 1000.0 * 1000.0 * 1000.0))
    } else if num < 1_000_000_000_000_000 {
        format!("{:.0}T", (num as f64) / (1000.0 * 1000.0 * 1000.0 * 1000.0))
    } else {
        format!(
            "{:.2}Q",
            (num as f64) / (1000.0 * 1000.0 * 1000.0 * 1000.0 * 1000.0)
        )
    }
}

const MAX_SEEDS: u64 = get_mask(36) as u64;

static mut SHOULD_STOP: bool = false;

static FOUND_SEED: AtomicBool = AtomicBool::new(false);
static SEEDS_SCANNED: AtomicU64 = AtomicU64::new(MAX_SEEDS);

fn main() {
    let time = Instant::now();
    let mut threads: Vec<JoinHandle<()>> = vec![];
    for thread_idx in 0..MAX_THREADS {
        threads.push(thread::spawn(move || {
            if unsafe { SHOULD_STOP } {
                return;
            }
            for lower48 in (thread_idx..MAX_SEEDS).step_by(MAX_THREADS as usize) {
                if check_seed(lower48, CHUNK_X as c_int, CHUNK_Z as c_int) {
                    if !FOUND_SEED.load(Ordering::SeqCst) {
                        FOUND_SEED.store(true, Ordering::SeqCst);
                        println!("Found seed `{lower48}`");
                        SEEDS_SCANNED.store(lower48, Ordering::SeqCst);
                        unsafe { SHOULD_STOP = true };
                        break;
                    } else {
                        return
                    }
                }
            }
        }));
    }
    for thread in threads {
        thread.join().unwrap();
    }
    let seeds_scanned = SEEDS_SCANNED.load(Ordering::SeqCst);
    let elapsed = time.elapsed();
    if !FOUND_SEED.load(Ordering::SeqCst) {
        println!("No seed found!");
    }
    println!("Stats:");
    println!("{} seeds scanned in {:.3?}", seeds_scanned, elapsed);
    println!(
        "{} seeds/s",
        format_num((seeds_scanned as f64 / elapsed.as_secs_f64()) as u64)
    );
    println!(
        "{} seeds/h",
        format_num((seeds_scanned as f64 / (elapsed.as_secs_f64() / 3600.0)) as u64)
    );
    println!(
        "{:.3}ns per seed",
        (elapsed.as_nanos() as f64) / seeds_scanned as f64
    );
}
