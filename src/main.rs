use core::arch::x86_64::*;

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
        benchmark_cache(1_000_000);
    }
}

fn clear(buffer: &mut [u8]) {
    for i in 0..buffer.len() {
        buffer[i] = 0;
    }
}

unsafe fn benchmark_cache(nb_samples: usize) {
    let target: u8 = 10;
    let mut cycles: u64 = 0;
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


fn write_csv(values: Iterator<Item=u64>) {

}
