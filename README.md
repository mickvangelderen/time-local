In order to obtain the local time offset, [time](https://crates.io/crates/time) calls out to `libc`s `localtime_r` function.
Implementations of `localtime_r`, like `glibc` and `musl`, call `getenv("TZ")` to obtain the current value for the `TZ` environment variable.
Unfortunately, values returned by `getenv()` can be invalidated by calls that modify the environment, like `setenv()`, `unsetenv()`, or `putenv()`.

For example, the following single-threaded application has a potential use after free bug:

```c
char * value = getenv("KEY"); // obtain pointer
setenv("KEY", "new value"); // potential free
printf("KEY = %s", value); // potential use after free
```

The functions in Rust's `std::env` module synchronize access to the environment through a lock.
However, any foreign code (including `libc` implementations) is free to modify the environment without acquiring that lock.
This has led to discussion about whether Rust's [`std::env::set_var` should be marked unsafe](https://internals.rust-lang.org/t/synchronized-ffi-access-to-posix-environment-variable-functions/15475/19).

Under the assumption that accessing the environment is implemented correctly everywhere for single-threaded programs, there can only be issues in multi-threaded programs.
This is why the [time](https://crates.io/crates/time) crate lets you obtain the UTC offset while the number of threads is 1.

This crate provides a solution for applications that can accept using a cached value of the UTC offset by doing exactly that: caching the UTC offset at the time of invocation.
Here is an example:

```rust
use time_local::{OffsetDateTimeExt, UtcOffsetExt};

fn main() {
    time_local::init().expect("initialization should succeed before spawning threads");

    let date = std::thread::spawn(|| {
        // We can not convert a date time to it's local representation.
        assert!(time::OffsetDateTime::now_utc()
            .to_local()
            .is_err(), "to_local should fail");
        
        // We can use the cached UTC offset computed at application startup. Note that this is computing something
        // different entirely, but it may be good enough for your application.
        time::OffsetDateTime::now_utc().to_offset(time::UtcOffset::cached_local_offset())
    })
    .join()
    .expect("thread should not panic");

    println!("{date:?}")
}
```

Note that a UTC offset depends on both the timezone and a particular date and time.
The cached UTC offset is computed from the current machine's timezone and time.
Changes to the system's local time and/or the `TZ` environment variable will not be reflected by the cached UTC offset, and the cached UTC offset used in `.to_local()` does not depend on the `OffsetDateTime`.

See https://github.com/time-rs/time/issues/688#issue-2346267822 for origins.
