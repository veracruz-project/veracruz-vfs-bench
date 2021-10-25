//! Benchmark of filesystem operations over one large file
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
    fs::File,
    hint,
    io::Write,
    io::Read,
    io::Seek,
    io::SeekFrom,
    iter,
    mem,
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


/// Write a large file in-order
pub fn write_inorder(size: u64, block_size: usize, run: u32) -> Duration {
    let path = format!("/scratch/write_inorder_{}_{}_{}.txt", size, block_size, run);
    let mut file = File::create(&path).unwrap();
    let mut prng = xorshift64(42);
    let mut buffer = vec![0u8; block_size];

    let stopwatch = Instant::now();

    for i in (0..size).step_by(block_size) {
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
            let input = hint::black_box(&buffer);
            file.write_all(input).unwrap();
        });
    }

    hint::black_box({
        file.flush().unwrap();
    });

    let duration = stopwatch.elapsed();

    // Truncate the file! Otherwise Veracruz may try to copy it back over
    // into the user's fs, which is a waste of (significant) time...
    //
    file.set_len(0).unwrap();

    duration
}

/// Update a large file in-order
pub fn update_inorder(size: u64, block_size: usize, run: u32) -> Duration {
    let path = format!("/scratch/update_inorder_{}_{}_{}.txt", size, block_size, run);
    let mut file = File::create(&path).unwrap();
    let mut prng = xorshift64(42);
    let mut buffer = vec![0u8; block_size];

    // first create/fill the file
    for i in (0..size).step_by(block_size) {
        for (j, x) in
            (&mut prng)
                .take(usize::try_from(
                    min(i+u64::try_from(block_size).unwrap(), size) - i
                ).unwrap())
                .enumerate()
        {
            buffer[j] = x as u8;
        }

        file.write_all(&buffer).unwrap();
    }

    mem::drop(file);
    let mut file = File::create(&path).unwrap();

    // now measure updates
    let stopwatch = Instant::now();

    for i in (0..size).step_by(block_size) {
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
            let input = hint::black_box(&buffer);
            file.write_all(input).unwrap();
        });
    }

    hint::black_box({
        file.flush().unwrap();
    });

    let duration = stopwatch.elapsed();

    // Truncate the file! Otherwise Veracruz may try to copy it back over
    // into the user's fs, which is a waste of (significant) time...
    //
    file.set_len(0).unwrap();

    duration
}

/// Read a large file in-order
pub fn read_inorder(size: u64, block_size: usize, run: u32) -> Duration {
    let path = format!("/scratch/read_inorder_{}_{}_{}.txt", size, block_size, run);
    let mut file = File::create(&path).unwrap();
    let mut prng = xorshift64(42);
    let mut buffer = vec![0u8; block_size];

    // first create/fill the file
    for i in (0..size).step_by(block_size) {
        for (j, x) in
            (&mut prng)
                .take(usize::try_from(
                    min(i+u64::try_from(block_size).unwrap(), size) - i
                ).unwrap())
                .enumerate()
        {
            buffer[j] = x as u8;
        }

        file.write_all(&buffer).unwrap();
    }

    mem::drop(file);
    let mut file = File::open(&path).unwrap();

    // Now measure reads
    let stopwatch = Instant::now();

    for i in (0..size).step_by(block_size) {
        let step_size = usize::try_from(
            min(i+u64::try_from(block_size).unwrap(), size) - i
        ).unwrap();
        
        hint::black_box({
            file.read_exact(hint::black_box(&mut buffer[..step_size])).unwrap();
            &buffer
        });
    }

    let duration = stopwatch.elapsed();

    mem::drop(file);
    let file = File::create(&path).unwrap();

    // Truncate the file! Otherwise Veracruz may try to copy it back over
    // into the user's fs, which is a waste of (significant) time...
    //
    file.set_len(0).unwrap();

    duration
}

/// Write a large file in reverse-order
pub fn write_reversed(size: u64, block_size: usize, run: u32) -> Duration {
    let path = format!("/scratch/write_reversed_{}_{}_{}.txt", size, block_size, run);
    let mut file = File::create(&path).unwrap();
    let mut prng = xorshift64(42);
    let mut buffer = vec![0u8; block_size];

    let stopwatch = Instant::now();

    // this division is a workaround for Range<u64> limitations
    for i in
        (0..size/u64::try_from(block_size).unwrap())
            .rev()
            .map(|i| i*u64::try_from(block_size).unwrap())
    {
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
            file.seek(SeekFrom::Start(i)).unwrap();

            let input = hint::black_box(&buffer);
            file.write_all(input).unwrap();
        });
    }

    hint::black_box({
        file.flush().unwrap();
    });

    let duration = stopwatch.elapsed();

    // Truncate the file! Otherwise Veracruz may try to copy it back over
    // into the user's fs, which is a waste of (significant) time...
    //
    file.set_len(0).unwrap();

    duration
}

/// Update a large file in reverse-order
pub fn update_reversed(size: u64, block_size: usize, run: u32) -> Duration {
    let path = format!("/scratch/update_reversed_{}_{}_{}.txt", size, block_size, run);
    let mut file = File::create(&path).unwrap();
    let mut prng = xorshift64(42);
    let mut buffer = vec![0u8; block_size];

    // first create/fill the file
    for i in (0..size).step_by(block_size) {
        for (j, x) in
            (&mut prng)
                .take(usize::try_from(
                    min(i+u64::try_from(block_size).unwrap(), size) - i
                ).unwrap())
                .enumerate()
        {
            buffer[j] = x as u8;
        }

        file.write_all(&buffer).unwrap();
    }

    mem::drop(file);
    let mut file = File::create(&path).unwrap();

    // now measure updates
    let stopwatch = Instant::now();

    // this division is a workaround for Range<u64> limitations
    for i in
        (0..size/u64::try_from(block_size).unwrap())
            .rev()
            .map(|i| i*u64::try_from(block_size).unwrap())
    {
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
            file.seek(SeekFrom::Start(i)).unwrap();

            let input = hint::black_box(&buffer);
            file.write_all(input).unwrap();
        });
    }

    hint::black_box({
        file.flush().unwrap();
    });

    let duration = stopwatch.elapsed();

    // Truncate the file! Otherwise Veracruz may try to copy it back over
    // into the user's fs, which is a waste of (significant) time...
    //
    file.set_len(0).unwrap();

    duration
}

/// Read a large file in reverse-order
pub fn read_reversed(size: u64, block_size: usize, run: u32) -> Duration {
    let path = format!("/scratch/read_reversed_{}_{}_{}.txt", size, block_size, run);
    let mut file = File::create(&path).unwrap();
    let mut prng = xorshift64(42);
    let mut buffer = vec![0u8; block_size];

    // first create/fill the file
    for i in (0..size).step_by(block_size) {
        for (j, x) in
            (&mut prng)
                .take(usize::try_from(
                    min(i+u64::try_from(block_size).unwrap(), size) - i
                ).unwrap())
                .enumerate()
        {
            buffer[j] = x as u8;
        }

        file.write_all(&buffer).unwrap();
    }

    mem::drop(file);
    let mut file = File::open(&path).unwrap();

    // Now measure reads
    let stopwatch = Instant::now();

    // this division is a workaround for Range<u64> limitations
    for i in
        (0..size/u64::try_from(block_size).unwrap())
            .rev()
            .map(|i| i*u64::try_from(block_size).unwrap())
    {
        let step_size = usize::try_from(
            min(i+u64::try_from(block_size).unwrap(), size) - i
        ).unwrap();
        
        hint::black_box({
            file.seek(SeekFrom::Start(i)).unwrap();

            file.read_exact(hint::black_box(&mut buffer[..step_size])).unwrap();
            &buffer
        });
    }

    let duration = stopwatch.elapsed();

    mem::drop(file);
    let file = File::create(&path).unwrap();

    // Truncate the file! Otherwise Veracruz may try to copy it back over
    // into the user's fs, which is a waste of (significant) time...
    //
    file.set_len(0).unwrap();

    duration
}

/// Write a large file in reverse-order
pub fn write_random(size: u64, block_size: usize, run: u32) -> Duration {
    let path = format!("/scratch/write_random_{}_{}_{}.txt", size, block_size, run);
    let mut file = File::create(&path).unwrap();
    let prng = RefCell::new(xorshift64(42));
    let mut buffer = vec![0u8; block_size];

    let stopwatch = Instant::now();

    // this may not touch every block, but that's ok
    let count = size/u64::try_from(block_size).unwrap();
    for i in 
        (0..count)
            .map(|_| prng.borrow_mut().next().unwrap() % count)
            .map(|i| i*u64::try_from(block_size).unwrap())
    {
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
            file.seek(SeekFrom::Start(i)).unwrap();

            let input = hint::black_box(&buffer);
            file.write_all(input).unwrap();
        });
    }

    hint::black_box({
        file.flush().unwrap();
    });

    let duration = stopwatch.elapsed();

    // Truncate the file! Otherwise Veracruz may try to copy it back over
    // into the user's fs, which is a waste of (significant) time...
    //
    file.set_len(0).unwrap();

    duration
}

/// Update a large file in reverse-order
pub fn update_random(size: u64, block_size: usize, run: u32) -> Duration {
    let path = format!("/scratch/update_random_{}_{}_{}.txt", size, block_size, run);
    let mut file = File::create(&path).unwrap();
    let prng = RefCell::new(xorshift64(42));
    let mut buffer = vec![0u8; block_size];

    // first create/fill the file
    for i in (0..size).step_by(block_size) {
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

        file.write_all(&buffer).unwrap();
    }

    mem::drop(file);
    let mut file = File::create(&path).unwrap();

    // now measure updates
    let stopwatch = Instant::now();

    // this may not touch every block, but that's ok
    let count = size/u64::try_from(block_size).unwrap();
    for i in 
        (0..count)
            .map(|_| prng.borrow_mut().next().unwrap() % count)
            .map(|i| i*u64::try_from(block_size).unwrap())
    {
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
            file.seek(SeekFrom::Start(i)).unwrap();

            let input = hint::black_box(&buffer);
            file.write_all(input).unwrap();
        });
    }

    hint::black_box({
        file.flush().unwrap();
    });

    let duration = stopwatch.elapsed();

    // Truncate the file! Otherwise Veracruz may try to copy it back over
    // into the user's fs, which is a waste of (significant) time...
    //
    file.set_len(0).unwrap();

    duration
}

/// Read a large file in reverse-order
pub fn read_random(size: u64, block_size: usize, run: u32) -> Duration {
    let path = format!("/scratch/read_random_{}_{}_{}.txt", size, block_size, run);
    let mut file = File::create(&path).unwrap();
    let mut prng = xorshift64(42);
    let mut buffer = vec![0u8; block_size];

    // first create/fill the file
    for i in (0..size).step_by(block_size) {
        for (j, x) in
            (&mut prng)
                .take(usize::try_from(
                    min(i+u64::try_from(block_size).unwrap(), size) - i
                ).unwrap())
                .enumerate()
        {
            buffer[j] = x as u8;
        }

        file.write_all(&buffer).unwrap();
    }

    mem::drop(file);
    let mut file = File::open(&path).unwrap();

    // Now measure reads
    let stopwatch = Instant::now();

    // this may not touch every block, but that's ok
    let count = size/u64::try_from(block_size).unwrap();
    for i in 
        (0..count)
            .map(|_| (&mut prng).next().unwrap() % count)
            .map(|i| i*u64::try_from(block_size).unwrap())
    {
        let step_size = usize::try_from(
            min(i+u64::try_from(block_size).unwrap(), size) - i
        ).unwrap();
        
        hint::black_box({
            file.seek(SeekFrom::Start(i)).unwrap();

            file.read_exact(hint::black_box(&mut buffer[..step_size])).unwrap();
            &buffer
        });
    }

    let duration = stopwatch.elapsed();

    mem::drop(file);
    let file = File::create(&path).unwrap();

    // Truncate the file! Otherwise Veracruz may try to copy it back over
    // into the user's fs, which is a waste of (significant) time...
    //
    file.set_len(0).unwrap();

    duration
}
