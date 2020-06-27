use core::arch::x86_64::*;

mod file_io;

unsafe fn time_access(addr: &u8) -> u64 {
    let mut __aux: u32 = 0;
    let value: u8;
    let start = __rdtscp(&mut __aux);
    value = *addr;
    _mm_mfence();
    let stop = __rdtscp(&mut __aux);
    return stop - start;
}

fn main() {
    unsafe {
        benchmark_cache(10_000);
        analyze_size(100_000);
        
    }
}

fn access(buffer: &[u8]) {
  let mut count: u8 = 0;
    for i in 0..buffer.len() {
        count += buffer[i];
    }
}

unsafe fn analyze_size(max_size: usize) -> Result<(), &'static str> {
  if max_size > 1_000_000 {
    return Err("Max size can not exceed 1.000.000");
  }
  let mut cycle_samples: Vec<u64> = Vec::with_capacity(max_size);
  let target: u8 = 255;
  let mut buffer: [u8; 1_000_000] = [0; 1_000_000];
  let target_address: &u8 = &target;
  for i in (1..max_size).step_by(100) {
    let value = *target_address;
    access(&buffer[1..i]);
    let cycles = time_access(&target);
    cycle_samples.push(cycles);
  }
  file_io::write_csv(&mut cycle_samples.into_iter())
    .map_err(|_| "Failed to write to file.")?;
  Ok(())
}

unsafe fn benchmark_cache(nb_samples: usize) {
    let target: u8 = 10;
    let flushed_access_time = (0..nb_samples).map(|_| {
        _mm_clflush(&target);
        return time_access(&target);
    }).fold(0, |x, y| x + y);
    let flushed_avg: f64 = (flushed_access_time as f64)/(nb_samples as f64);
    println!("Average cycles per access when present: {}", flushed_avg);

    // Load target in cache;
    let dummy = target + 1;
    let present_access_time = (0..nb_samples).map(|_| {
        return time_access(&target);
    }).fold(0, |x, y| x + y);

    let present_avg: f64 = (present_access_time as f64)/(nb_samples as f64);
    println!("Average cycles per access when present: {}", present_avg);
}
