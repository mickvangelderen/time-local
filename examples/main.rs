use time_local::OffsetDateTimeExt;

fn main() {
    time_local::init();

    let date = std::thread::spawn(|| {
        // `time::OffsetDateTime::now_local()` will fail because it queries `time::UtcOffset::current_local_time`, instead we can use:
        time::OffsetDateTime::now_utc()
            .to_local()
            .expect("conversion to local offset with cached value should succeed")
    })
    .join()
    .expect("thread should not panic");

    println!("{date:?}")
}
