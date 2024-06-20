use time_local::{OffsetDateTimeExt, UtcOffsetExt};

fn main() {
    time_local::init().expect("initialization should succeed before spawning threads");

    let date = std::thread::spawn(|| {
        // We can not convert a date time to it's local representation.
        assert!(
            time::OffsetDateTime::now_utc().to_local().is_err(),
            "to_local should fail"
        );

        // We can use the cached UTC offset computed at application startup. Note that this is computing something
        // different entirely, but it may be good enough for your application.
        time::OffsetDateTime::now_utc().to_offset(time::UtcOffset::cached_local_offset())
    })
    .join()
    .expect("thread should not panic");

    println!("{date:?}")
}
