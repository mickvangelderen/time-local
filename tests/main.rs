mod support;

use time_local::UtcOffsetExt as _;

#[test]
fn it_works() {
    let guard = support::UnsoundGuard::new();
    time_local::init().unwrap();
    drop(guard);
    let offset = time::UtcOffset::cached_local_offset();
    let date = time::OffsetDateTime::from_unix_timestamp(1718785511).unwrap();
    assert_eq!(date.to_offset(offset).offset(), offset);
}
