//! Benchmark of filesystem operations over many small files
//!
//! ## Authors
//!
//! The Veracruz Development Team.
//!
//! ## Copyright
//!
//! See the file `LICENSING.markdown` in the Veracruz root directory for licensing
//! and copyright information.

use std::{
    cell::RefCell,
    cmp::min,
    convert::TryFrom,
    fs,
    fs::File,
    fs::OpenOptions,
    hint,
    io::Write,
    io::Read,
    iter,
    ops::DerefMut,
    time::Duration,
    time::Instant,
};

/// xorshift64 for providing deterministic pseudo-random numbers
fn xorshift64(seed: u64) -> impl Iterator<Item=u64> {
    let mut x = seed;
    iter::repeat_with(move || {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        x
    })
}


/// Write small files in-order
pub fn write_inorder(size: u64, block_size: usize, run: u32) -> Duration {
    let path = format!("/scratch/small_write_inorder_{}_{}_{}", size, block_size, run);
    let mut prng = xorshift64(42);
    let mut buffer = vec![0u8; block_size];
    fs::create_dir(&path).unwrap();

    let stopwatch = Instant::now();

    for i in 0..size/u64::try_from(block_size).unwrap() {
        let path = format!("{}/{:09x}.txt", path, i);

        for (j, x) in
            (&mut prng)
                .take(usize::try_from(
                    min(i+u64::try_from(block_size).unwrap(), size) - i
                ).unwrap())
                .enumerate()
        {
            buffer[j] = x as u8;
        }
        
        hint::black_box({
            let path = hint::black_box(&path);
            let mut file = File::create(path).unwrap();

            let input = hint::black_box(&buffer);
            file.write_all(input).unwrap();

            file.flush().unwrap();
        });
    }

    let duration = stopwatch.elapsed();

    // Clean up! Otherwise Veracruz may try to copy it back over
    // into the user's fs, which is a waste of (significant) time...
    //
    for i in 0..size/u64::try_from(block_size).unwrap() {
        let path = format!("{}/{:09x}.txt", path, i);
        let file = File::create(path).unwrap();
        file.set_len(0).unwrap();
    }

    duration
}

/// Update small files in-order
pub fn update_inorder(size: u64, block_size: usize, run: u32) -> Duration {
    let path = format!("/scratch/small_write_inorder_{}_{}_{}", size, block_size, run);
    let mut prng = xorshift64(42);
    let mut buffer = vec![0u8; block_size];
    fs::create_dir(&path).unwrap();

    // first create the files
    for i in 0..size/u64::try_from(block_size).unwrap() {
        let path = format!("{}/{:09x}.txt", path, i);

        for (j, x) in
            (&mut prng)
                .take(usize::try_from(
                    min(i+u64::try_from(block_size).unwrap(), size) - i
                ).unwrap())
                .enumerate()
        {
            buffer[j] = x as u8;
        }
        
        hint::black_box({
            // curiously we need to open this file as read here to enable
            // reading later, since the flags to open here affect the persistent
            // capabilities on the filesystem
            let path = hint::black_box(&path);
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(path).unwrap();

            let input = hint::black_box(&buffer);
            file.write_all(input).unwrap();

            file.flush().unwrap();
        });
    }

    // then benchmark
    let stopwatch = Instant::now();

    for i in 0..size/u64::try_from(block_size).unwrap() {
        let path = format!("{}/{:09x}.txt", path, i);

        for (j, x) in
            (&mut prng)
                .take(usize::try_from(
                    min(i+u64::try_from(block_size).unwrap(), size) - i
                ).unwrap())
                .enumerate()
        {
            buffer[j] = x as u8;
        }
        
        hint::black_box({
            let path = hint::black_box(&path);
            let mut file = OpenOptions::new()
                .write(true)
                .open(path).unwrap();

            let input = hint::black_box(&buffer);
            file.write_all(input).unwrap();

            file.flush().unwrap();
        });
    }

    let duration = stopwatch.elapsed();

    // Clean up! Otherwise Veracruz may try to copy it back over
    // into the user's fs, which is a waste of (significant) time...
    //
    for i in 0..size/u64::try_from(block_size).unwrap() {
        let path = format!("{}/{:09x}.txt", path, i);
        let file = File::create(path).unwrap();
        file.set_len(0).unwrap();
    }

    duration
}

/// Read small files in-order
pub fn read_inorder(size: u64, block_size: usize, run: u32) -> Duration {
    let path = format!("/scratch/small_write_inorder_{}_{}_{}", size, block_size, run);
    let mut prng = xorshift64(42);
    let mut buffer = vec![0u8; block_size];
    fs::create_dir(&path).unwrap();

    // first create the files
    for i in 0..size/u64::try_from(block_size).unwrap() {
        let path = format!("{}/{:09x}.txt", path, i);

        for (j, x) in
            (&mut prng)
                .take(usize::try_from(
                    min(i+u64::try_from(block_size).unwrap(), size) - i
                ).unwrap())
                .enumerate()
        {
            buffer[j] = x as u8;
        }
        
        hint::black_box({
            // curiously we need to open this file as read here to enable
            // reading later, since the flags to open here affect the persistent
            // capabilities on the filesystem
            let path = hint::black_box(&path);
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(path).unwrap();

            let input = hint::black_box(&buffer);
            file.write_all(input).unwrap();

            file.flush().unwrap();
        });
    }

    // then benchmark
    let stopwatch = Instant::now();

    for i in 0..size/u64::try_from(block_size).unwrap() {
        let path = format!("{}/{:09x}.txt", path, i);
        
        hint::black_box({
            let path = hint::black_box(&path);
            let mut file = File::open(path).unwrap();

            file.read_exact(hint::black_box(&mut buffer)).unwrap();
            &buffer
        });
    }

    let duration = stopwatch.elapsed();

    // Clean up! Otherwise Veracruz may try to copy it back over
    // into the user's fs, which is a waste of (significant) time...
    //
    for i in 0..size/u64::try_from(block_size).unwrap() {
        let path = format!("{}/{:09x}.txt", path, i);
        let file = File::create(path).unwrap();
        file.set_len(0).unwrap();
    }

    duration
}

/// Write small files in reversed-order
pub fn write_reversed(size: u64, block_size: usize, run: u32) -> Duration {
    let path = format!("/scratch/small_write_reversed_{}_{}_{}", size, block_size, run);
    let mut prng = xorshift64(42);
    let mut buffer = vec![0u8; block_size];
    fs::create_dir(&path).unwrap();

    let stopwatch = Instant::now();

    for i in (0..size/u64::try_from(block_size).unwrap()).rev() {
        let path = format!("{}/{:09x}.txt", path, i);

        for (j, x) in
            (&mut prng)
                .take(usize::try_from(
                    min(i+u64::try_from(block_size).unwrap(), size) - i
                ).unwrap())
                .enumerate()
        {
            buffer[j] = x as u8;
        }
        
        hint::black_box({
            let path = hint::black_box(&path);
            let mut file = File::create(path).unwrap();

            let input = hint::black_box(&buffer);
            file.write_all(input).unwrap();

            file.flush().unwrap();
        });
    }

    let duration = stopwatch.elapsed();

    // Clean up! Otherwise Veracruz may try to copy it back over
    // into the user's fs, which is a waste of (significant) time...
    //
    for i in 0..size/u64::try_from(block_size).unwrap() {
        let path = format!("{}/{:09x}.txt", path, i);
        let file = File::create(path).unwrap();
        file.set_len(0).unwrap();
    }

    duration
}

/// Update small files in reversed-order
pub fn update_reversed(size: u64, block_size: usize, run: u32) -> Duration {
    let path = format!("/scratch/small_write_reversed_{}_{}_{}", size, block_size, run);
    let mut prng = xorshift64(42);
    let mut buffer = vec![0u8; block_size];
    fs::create_dir(&path).unwrap();

    // first create the files
    for i in 0..size/u64::try_from(block_size).unwrap() {
        let path = format!("{}/{:09x}.txt", path, i);

        for (j, x) in
            (&mut prng)
                .take(usize::try_from(
                    min(i+u64::try_from(block_size).unwrap(), size) - i
                ).unwrap())
                .enumerate()
        {
            buffer[j] = x as u8;
        }
        
        hint::black_box({
            // curiously we need to open this file as read here to enable
            // reading later, since the flags to open here affect the persistent
            // capabilities on the filesystem
            let path = hint::black_box(&path);
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(path).unwrap();

            let input = hint::black_box(&buffer);
            file.write_all(input).unwrap();

            file.flush().unwrap();
        });
    }

    // then benchmark
    let stopwatch = Instant::now();

    for i in (0..size/u64::try_from(block_size).unwrap()).rev() {
        let path = format!("{}/{:09x}.txt", path, i);

        for (j, x) in
            (&mut prng)
                .take(usize::try_from(
                    min(i+u64::try_from(block_size).unwrap(), size) - i
                ).unwrap())
                .enumerate()
        {
            buffer[j] = x as u8;
        }
        
        hint::black_box({
            let path = hint::black_box(&path);
            let mut file = OpenOptions::new()
                .write(true)
                .open(path).unwrap();

            let input = hint::black_box(&buffer);
            file.write_all(input).unwrap();

            file.flush().unwrap();
        });
    }

    let duration = stopwatch.elapsed();

    // Clean up! Otherwise Veracruz may try to copy it back over
    // into the user's fs, which is a waste of (significant) time...
    //
    for i in 0..size/u64::try_from(block_size).unwrap() {
        let path = format!("{}/{:09x}.txt", path, i);
        let file = File::create(path).unwrap();
        file.set_len(0).unwrap();
    }

    duration
}

/// Read small files in reversed-order
pub fn read_reversed(size: u64, block_size: usize, run: u32) -> Duration {
    let path = format!("/scratch/small_write_reversed_{}_{}_{}", size, block_size, run);
    let mut prng = xorshift64(42);
    let mut buffer = vec![0u8; block_size];
    fs::create_dir(&path).unwrap();

    // first create the files
    for i in 0..size/u64::try_from(block_size).unwrap() {
        let path = format!("{}/{:09x}.txt", path, i);

        for (j, x) in
            (&mut prng)
                .take(usize::try_from(
                    min(i+u64::try_from(block_size).unwrap(), size) - i
                ).unwrap())
                .enumerate()
        {
            buffer[j] = x as u8;
        }
        
        hint::black_box({
            // curiously we need to open this file as read here to enable
            // reading later, since the flags to open here affect the persistent
            // capabilities on the filesystem
            let path = hint::black_box(&path);
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(path).unwrap();

            let input = hint::black_box(&buffer);
            file.write_all(input).unwrap();

            file.flush().unwrap();
        });
    }

    // then benchmark
    let stopwatch = Instant::now();

    for i in (0..size/u64::try_from(block_size).unwrap()).rev() {
        let path = format!("{}/{:09x}.txt", path, i);
        
        hint::black_box({
            let path = hint::black_box(&path);
            let mut file = File::open(path).unwrap();

            file.read_exact(hint::black_box(&mut buffer)).unwrap();
            &buffer
        });
    }

    let duration = stopwatch.elapsed();

    // Clean up! Otherwise Veracruz may try to copy it back over
    // into the user's fs, which is a waste of (significant) time...
    //
    for i in 0..size/u64::try_from(block_size).unwrap() {
        let path = format!("{}/{:09x}.txt", path, i);
        let file = File::create(path).unwrap();
        file.set_len(0).unwrap();
    }

    duration
}

/// Write small files in random-order
pub fn write_random(size: u64, block_size: usize, run: u32) -> Duration {
    let path = format!("/scratch/small_write_random_{}_{}_{}", size, block_size, run);
    let prng = RefCell::new(xorshift64(42));
    let mut buffer = vec![0u8; block_size];
    fs::create_dir(&path).unwrap();

    let stopwatch = Instant::now();

    let count = size/u64::try_from(block_size).unwrap();
    for i in 
        (0..count)
            .map(|_| prng.borrow_mut().next().unwrap() % count)
    {
        let path = format!("{}/{:09x}.txt", path, i);

        for (j, x) in
            prng
                .borrow_mut()
                .deref_mut()
                .take(usize::try_from(
                    min(i+u64::try_from(block_size).unwrap(), size) - i
                ).unwrap())
                .enumerate()
        {
            buffer[j] = x as u8;
        }
        
        hint::black_box({
            let path = hint::black_box(&path);
            let mut file = File::create(path).unwrap();

            let input = hint::black_box(&buffer);
            file.write_all(input).unwrap();

            file.flush().unwrap();
        });
    }

    let duration = stopwatch.elapsed();

    // Clean up! Otherwise Veracruz may try to copy it back over
    // into the user's fs, which is a waste of (significant) time...
    //
    for i in 0..size/u64::try_from(block_size).unwrap() {
        let path = format!("{}/{:09x}.txt", path, i);
        let file = File::create(path).unwrap();
        file.set_len(0).unwrap();
    }

    duration
}

/// Update small files in random-order
pub fn update_random(size: u64, block_size: usize, run: u32) -> Duration {
    let path = format!("/scratch/small_write_random_{}_{}_{}", size, block_size, run);
    let prng = RefCell::new(xorshift64(42));
    let mut buffer = vec![0u8; block_size];
    fs::create_dir(&path).unwrap();

    // first create the files
    for i in 0..size/u64::try_from(block_size).unwrap() {
        let path = format!("{}/{:09x}.txt", path, i);

        for (j, x) in
            prng
                .borrow_mut()
                .deref_mut()
                .take(usize::try_from(
                    min(i+u64::try_from(block_size).unwrap(), size) - i
                ).unwrap())
                .enumerate()
        {
            buffer[j] = x as u8;
        }
        
        hint::black_box({
            // curiously we need to open this file as read here to enable
            // reading later, since the flags to open here affect the persistent
            // capabilities on the filesystem
            let path = hint::black_box(&path);
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(path).unwrap();

            let input = hint::black_box(&buffer);
            file.write_all(input).unwrap();

            file.flush().unwrap();
        });
    }

    // then benchmark
    let stopwatch = Instant::now();

    let count = size/u64::try_from(block_size).unwrap();
    for i in 
        (0..count)
            .map(|_| prng.borrow_mut().next().unwrap() % count)
    {
        let path = format!("{}/{:09x}.txt", path, i);

        for (j, x) in
            prng
                .borrow_mut()
                .deref_mut()
                .take(usize::try_from(
                    min(i+u64::try_from(block_size).unwrap(), size) - i
                ).unwrap())
                .enumerate()
        {
            buffer[j] = x as u8;
        }
        
        hint::black_box({
            let path = hint::black_box(&path);
            let mut file = OpenOptions::new()
                .write(true)
                .open(path).unwrap();

            let input = hint::black_box(&buffer);
            file.write_all(input).unwrap();

            file.flush().unwrap();
        });
    }

    let duration = stopwatch.elapsed();

    // Clean up! Otherwise Veracruz may try to copy it back over
    // into the user's fs, which is a waste of (significant) time...
    //
    for i in 0..size/u64::try_from(block_size).unwrap() {
        let path = format!("{}/{:09x}.txt", path, i);
        let file = File::create(path).unwrap();
        file.set_len(0).unwrap();
    }

    duration
}

/// Read small files in random-order
pub fn read_random(size: u64, block_size: usize, run: u32) -> Duration {
    let path = format!("/scratch/small_write_random_{}_{}_{}", size, block_size, run);
    let mut prng = xorshift64(42);
    let mut buffer = vec![0u8; block_size];
    fs::create_dir(&path).unwrap();

    // first create the files
    for i in 0..size/u64::try_from(block_size).unwrap() {
        let path = format!("{}/{:09x}.txt", path, i);

        for (j, x) in
            (&mut prng)
                .take(usize::try_from(
                    min(i+u64::try_from(block_size).unwrap(), size) - i
                ).unwrap())
                .enumerate()
        {
            buffer[j] = x as u8;
        }
        
        hint::black_box({
            // curiously we need to open this file as read here to enable
            // reading later, since the flags to open here affect the persistent
            // capabilities on the filesystem
            let path = hint::black_box(&path);
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(path).unwrap();

            let input = hint::black_box(&buffer);
            file.write_all(input).unwrap();

            file.flush().unwrap();
        });
    }

    // then benchmark
    let stopwatch = Instant::now();

    let count = size/u64::try_from(block_size).unwrap();
    for i in 
        (0..count)
            .map(|_| (&mut prng).next().unwrap() % count)
    {
        let path = format!("{}/{:09x}.txt", path, i);
        
        hint::black_box({
            let path = hint::black_box(&path);
            let mut file = File::open(path).unwrap();

            file.read_exact(hint::black_box(&mut buffer)).unwrap();
            &buffer
        });
    }

    let duration = stopwatch.elapsed();

    // Clean up! Otherwise Veracruz may try to copy it back over
    // into the user's fs, which is a waste of (significant) time...
    //
    for i in 0..size/u64::try_from(block_size).unwrap() {
        let path = format!("{}/{:09x}.txt", path, i);
        let file = File::create(path).unwrap();
        file.set_len(0).unwrap();
    }

    duration
}
