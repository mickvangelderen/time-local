use std::sync::OnceLock;

use time::UtcOffset;

pub trait UtcOffsetExt {
    /// Cached result of [`time::UtcOffset::current_local_offset`].
    ///
    /// # Panicks
    ///
    /// Panicks if [`crate::init`] has not been called with a succesful return value.
    fn cached_local_offset() -> time::UtcOffset;
}

static UTC_OFFSET: OnceLock<time::UtcOffset> = OnceLock::new();

impl UtcOffsetExt for time::UtcOffset {
    fn cached_local_offset() -> time::UtcOffset {
        *UTC_OFFSET.get().unwrap_or_else(|| utc_offset_init_error())
    }
}

pub trait OffsetDateTimeExt {
    /// Convenience method that calls [`time::OffsetDateTime::to_offset`] with the [`time::UtcOffset`] for `self`.
    fn to_local(self) -> Result<time::OffsetDateTime, time::error::IndeterminateOffset>;
}

impl OffsetDateTimeExt for time::OffsetDateTime {
    fn to_local(self) -> Result<time::OffsetDateTime, time::error::IndeterminateOffset> {
        Ok(self.to_offset(time::UtcOffset::local_offset_at(self)?))
    }
}

/// Call this function before your program spawns threads and before you use [`UtcOffsetExt::cached_local_offset`].
pub fn init() -> Result<(), time::error::IndeterminateOffset> {
    let utc_offset = UtcOffset::current_local_offset()?;
    UTC_OFFSET
        .set(utc_offset)
        .unwrap_or_else(|_| utc_offset_init_error());
    Ok(())
}

fn utc_offset_init_error() -> ! {
    panic!(
        "call `time_local::init()` once during application initialization before spawning threads"
    )
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
        let guard = UnsoundGuard::new();
        init().unwrap();
        drop(guard);
        let offset = time::UtcOffset::cached_local_offset();
        let date = time::OffsetDateTime::from_unix_timestamp(1718785511).unwrap();
        assert_eq!(date.to_offset(offset).offset(), offset);
    }
}
