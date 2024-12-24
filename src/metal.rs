use metal::{Device, DeviceRef, FunctionConstantValues, MTLDataType, MTLResourceOptions};
use std::ffi::c_void;
use std::time::Instant;
use std::slice;

const LIB_SOURCE: &[u8] = include_bytes!("metal/find_seed.metallib");

pub const MAX_SEED: u64 = 1 << 38;

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

fn main() {
    let device: &DeviceRef = &Device::system_default().expect("No device found");

    let lib = device
        .new_library_with_data(LIB_SOURCE)
        .unwrap();

    let constants = FunctionConstantValues::new();

    constants.set_constant_value_at_index(&MAX_SEED as *const u64 as *const c_void, MTLDataType::ULong, 0);

    let function = lib.get_function("find_seed", Some(constants)).unwrap();
    let pipeline = device
        .new_compute_pipeline_state_with_function(&function)
        .unwrap();

    let buffer_result = device.new_buffer(
        size_of::<u64>() as u64 * 2,
        MTLResourceOptions::StorageModeShared,
    );

    let command_queue = device.new_command_queue();

    let command_buffer = command_queue.new_command_buffer();

    let compute_encoder = command_buffer.new_compute_command_encoder();
    compute_encoder.set_compute_pipeline_state(&pipeline);
    compute_encoder.set_buffers(
        0,
        &[Some(&buffer_result)],
        &[0; 2],
    );

    let max_threads_per_group = pipeline.max_total_threads_per_threadgroup();
    let num_thread_groups = MAX_SEED.div_ceil(max_threads_per_group);

    let max_threads_per_group = metal::MTLSize::new(max_threads_per_group, 1, 1);
    let num_thread_groups = metal::MTLSize::new(num_thread_groups, 1, 1);
    compute_encoder.dispatch_threads(num_thread_groups, max_threads_per_group);

    compute_encoder.end_encoding();
    command_buffer.commit();

    let now = Instant::now();

    command_buffer.wait_until_completed();

    let elapsed = now.elapsed();

    println!("{} seeds in {:.3?}", format_num(MAX_SEED), elapsed);
    println!("{} seeds/s or {:.3}ns per seed", format_num((MAX_SEED as f64 / elapsed.as_secs_f64()) as u64), elapsed.as_nanos() as f64 / MAX_SEED as f64);

    {
        let ptr = buffer_result.contents() as *const u64;
        let len = buffer_result.length() as usize / size_of::<u64>();
        let slice = unsafe { slice::from_raw_parts(ptr, len) };
        println!("Results: {:?}", slice.to_vec());
    }
}
