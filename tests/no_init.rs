use time_local::UtcOffsetExt;

#[test]
#[should_panic(
    expected = "call `time_local::init()` once during application initialization before spawning threads"
)]
fn panic_on_no_init() {
    time::UtcOffset::cached_local_offset();
}
