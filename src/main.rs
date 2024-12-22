use std::ffi::c_int;
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

use rs_abbreviation_number::NumericAbbreviate;

use seedcracker::check_seed;
use seedcracker::random::mth::get_mask;

const MAX_THREADS: u64 = 16;

fn main() {
    let chunk_x: c_int = -28;
    let chunk_z: c_int = -73;

    // let mut g: MaybeUninit<Generator> = MaybeUninit::zeroed();
    // unsafe { setupGenerator(g.as_mut_ptr(), MC, 0) };
    let time = Instant::now();
    let mut threads: Vec<JoinHandle<()>> = vec![];
    let max_seeds = get_mask(26) as u64;
    for thread_idx in 0..MAX_THREADS {
        threads.push(thread::spawn(move || {
            for lower48 in (thread_idx..max_seeds).step_by(MAX_THREADS as usize) {
                if check_seed(lower48, chunk_x, chunk_z) {
                    println!("Found seed `{lower48}` in {:.3?}", time.elapsed());
                    break;
                }
            }
        }));
    }
    for thread in threads {
        thread.join().unwrap();
    }
    let elapsed = time.elapsed();
    println!(
        "No seed found. Scanned {} in {:.3?} or {} seeds/s. Each seed took {:.3} ns on average",
        max_seeds.abbreviate_number(&Default::default()),
        elapsed,
        (max_seeds as f64 / elapsed.as_secs_f64()).abbreviate_number(&Default::default()),
        elapsed.as_nanos() as f64 / max_seeds as f64
    );
}
