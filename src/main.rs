use std::ffi::c_int;
use std::time::Instant;

use seedcracker::check_seed;
use seedcracker::random::mth::get_mask;

fn main() {
    let chunk_x: c_int = -28;
    let chunk_z: c_int = -73;

    // let mut g: MaybeUninit<Generator> = MaybeUninit::zeroed();
    // unsafe { setupGenerator(g.as_mut_ptr(), MC, 0) };
    let time = Instant::now();
    for lower48 in 0..get_mask(26) as u64 {
        // if lower48 % 2_u64.pow(24) == 0 {
        //     println!(
        //         "Progress: {:.2}%",
        //         (lower48 as f64 / get_mask(48) as f64) * 100.0
        //     );
        // }
        if check_seed(lower48, chunk_x, chunk_z) {
            println!("Found seed `{lower48}` in {:.3?}", time.elapsed());
            break;
        }
    }
}
