use core::sync::atomic::{AtomicUsize, Ordering};
#[no_mangle]
static JIFFIES: AtomicUsize = AtomicUsize::new(0);

// Number of ms between each irq0
// This value should be written only onth at the boot
#[no_mangle]
pub static mut SYSTEM_FRACTION: f64 = 1.0;

#[inline(always)]
pub fn get_time_ms() -> u64 {
    // SYSTEM_FRACTION shouldn't be change after boot and should so be safe
    unsafe {
        (JIFFIES.load(Ordering::Relaxed) as f64 * SYSTEM_FRACTION) as u64
    }
}

#[inline(always)]
pub fn get_time_s() -> f64 {
    get_time_ms() as f64 / 1000.0
}

#[inline(always)]
#[no_mangle]
pub fn jiffies_inc() {
    JIFFIES.fetch_add(1, Ordering::Relaxed);
}

#[inline(always)]
pub fn jiffies() -> usize {
    JIFFIES.load(Ordering::Relaxed)
}


pub fn sleep(ms: u64) {
    if ms > 100 {
        sleep_ms(ms);
    } else {
        microsleep(ms * 1000);
    }
}

#[inline]
fn sleep_ms(ms: u64) {
    let saved_time = get_time_ms() as u64;
    while saved_time + ms > get_time_ms() as u64 {
        unsafe {
        crate::wrappers::hlt!();
        }
    }
}

#[inline]
pub fn microsleep(us: u64) {
    nanosleep(us * 1000);
}

// Imprecise benched clock cycle
// Between two jiffies of 10ms
// could loop over 6000000 nop
// Which give 6 nop each nanosecond
#[no_mangle]
pub fn nanosleep(ns: u64) {
    for _ in 0..(ns * 1000) {
        unsafe {
        core::arch::asm!("nop");
        }
    }
}

