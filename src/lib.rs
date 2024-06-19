use std::sync::OnceLock;

use time::UtcOffset;

pub trait UtcOffsetExt {
    /// Cached result of [`time::UtcOffset::current_local_offset`].
    fn cached_local_offset() -> Result<time::UtcOffset, time::error::IndeterminateOffset>;
}

impl UtcOffsetExt for time::UtcOffset {
    fn cached_local_offset() -> Result<time::UtcOffset, time::error::IndeterminateOffset> {
        static CACHE: OnceLock<Result<time::UtcOffset, time::error::IndeterminateOffset>> =
            OnceLock::new();
        *CACHE.get_or_init(time::UtcOffset::current_local_offset)
    }
}

pub trait OffsetDateTimeExt {
    /// Convenience method that calls [`time::OffsetDateTime::to_offset`] with the return value of
    /// [`time::UtcOffset::current_local_offset`]. The current local offset is cached upon the first call. This call is
    /// more likely to succeed before the program spawns threads. Browse the source code of
    /// [`time::UtcOffset::current_local_offset`] to understand why.
    fn to_local(self) -> time::Result<time::OffsetDateTime>;
}

impl OffsetDateTimeExt for time::OffsetDateTime {
    fn to_local(self) -> time::Result<time::OffsetDateTime> {
        Ok(self.to_offset(time::UtcOffset::cached_local_offset()?))
    }
}

/// Call this function before your program spawns threads. Only necessary if you program spawns threads.
///
/// The function [`time::UtcOffset::current_local_offset`] is more likely to succeed before the program spawns threads.
/// Browse the source code of [`time::UtcOffset::current_local_offset`] to understand why.
pub fn init() {
    let _ = UtcOffset::cached_local_offset();
}

#[cfg(test)]
mod tests {
    use super::*;

    struct UnsoundGuard {
        #[allow(unused)]
        guard: std::sync::MutexGuard<'static, ()>,
    }

    impl UnsoundGuard {
        pub fn new() -> Self {
            use std::sync::Mutex;
            use time::util::local_offset::{set_soundness, Soundness};

            static SOUNDNESS_LOCK: Mutex<()> = Mutex::new(());
            let guard = SOUNDNESS_LOCK.lock().expect("lock is poisoned");
            unsafe { set_soundness(Soundness::Unsound) };
            Self { guard }
        }
    }

    impl Drop for UnsoundGuard {
        fn drop(&mut self) {
            use time::util::local_offset::{set_soundness, Soundness};

            unsafe { set_soundness(Soundness::Sound) };
        }
    }

    #[test]
    fn it_works() {
        let _guard = UnsoundGuard::new();
        init();
        let offset = time::UtcOffset::cached_local_offset().unwrap();
        let date = time::OffsetDateTime::from_unix_timestamp(1718785511).unwrap();
        assert_eq!(date.to_local().unwrap().offset(), offset);
    }
}
