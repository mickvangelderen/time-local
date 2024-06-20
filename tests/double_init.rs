mod support;

#[test]
#[should_panic(
    expected = "call `time_local::init()` once during application initialization before spawning threads"
)]
fn panic_on_double_init() {
    let _guard = support::UnsoundGuard::new();
    time_local::init().unwrap();
    time_local::init().unwrap();
}
