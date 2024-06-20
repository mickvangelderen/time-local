use std::sync::{Mutex, MutexGuard};
use time::util::local_offset::{set_soundness, Soundness};

/// Helper to set and unset [`time::util::local_offset::Soundness`].
/// Doing so is necessary because the test framework spawns threads.
pub struct UnsoundGuard {
    #[allow(unused)]
    guard: MutexGuard<'static, ()>,
}

impl UnsoundGuard {
    pub fn new() -> Self {
        static SOUNDNESS_LOCK: Mutex<()> = Mutex::new(());
        let guard = SOUNDNESS_LOCK.lock().expect("lock is poisoned");
        unsafe { set_soundness(Soundness::Unsound) };
        Self { guard }
    }
}

impl Drop for UnsoundGuard {
    fn drop(&mut self) {
        unsafe { set_soundness(Soundness::Sound) };
    }
}
