//! Benchmarks of various filesystem operations
//!
//! ## Authors
//!
//! The Veracruz Development Team.
//!
//! ## Copyright
//!
//! See the file `LICENSING.markdown` in the Veracruz root directory for licensing
//! and copyright information.

// black_box disable optimizations that depend on its value
//
// unfortunately it is only available on nightly
//
#![feature(test)]

#[allow(unused)]
use anyhow;
use std::{
    env,
    fs,
};

mod large_file;
mod incremental_file;
mod small_files;


/// entry point
fn main() {
    // parse arguments
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 4 || args.len() > 5 {
        eprintln!("./{} <mode> <size> [block_size] [run]", args[0]);
        return;
    }

    let mode = &args[1];
    let benchmark = match args[1].as_ref() {
        "write_inorder"                 => large_file::write_inorder,
        "update_inorder"                => large_file::update_inorder,
        "read_inorder"                  => large_file::read_inorder,
        "write_reversed"                => large_file::write_reversed,
        "update_reversed"               => large_file::update_reversed,
        "read_reversed"                 => large_file::read_reversed,
        "write_random"                  => large_file::write_random,
        "update_random"                 => large_file::update_random,
        "read_random"                   => large_file::read_random,
        "incremental_write_inorder"     => incremental_file::write_inorder,
        "incremental_update_inorder"    => incremental_file::update_inorder,
        "incremental_read_inorder"      => incremental_file::read_inorder,
        "incremental_write_reversed"    => incremental_file::write_reversed,
        "incremental_update_reversed"   => incremental_file::update_reversed,
        "incremental_read_reversed"     => incremental_file::read_reversed,
        "incremental_write_random"      => incremental_file::write_random,
        "incremental_update_random"     => incremental_file::update_random,
        "incremental_read_random"       => incremental_file::read_random,
        "small_write_inorder"           => small_files::write_inorder,
        "small_read_inorder"            => small_files::read_inorder,
        "small_update_inorder"          => small_files::update_inorder,
        "small_write_reversed"          => small_files::write_reversed,
        "small_read_reversed"           => small_files::read_reversed,
        "small_update_reversed"         => small_files::update_reversed,
        "small_write_random"            => small_files::write_random,
        "small_read_random"             => small_files::read_random,
        "small_update_random"           => small_files::update_random,
        _ => {
            eprintln!("Unknown mode {:?}", mode);
            return;
        }
    };

    let size = match args[2].parse::<u64>() {
        Ok(size) => size,
        Err(_) => {
            eprintln!("Can't parse size");
            return;
        }
    };

    let block_size = match args[3].parse::<usize>() {
        Ok(block_size) => block_size,
        Err(_) => {
            eprintln!("Can't parse block_size");
            return;
        }
    };

    let run = match args.get(4) {
        Some(run) => match run.parse::<u32>() {
            Ok(run) => run,
            Err(_) => {
                eprintln!("Can't parse run");
                return;
            }
        },
        None => 0,
    };

    // run benchmarks
    println!("benchmarking {}: size={}, block_size={}",
        mode, size, block_size
    );

    let duration = benchmark(size, block_size, run);

    println!("benchmarking {}: runtime={:?}",
        mode, duration
    );

    // write results to file
    fs::write(
        format!("/results/result_{}_{}_{}_{}.json",
            mode, size, block_size, run
        ),
        format!(
            "{{\
                \"name\":{:?},\
                \"size\":{},\
                \"block_size\":{},\
                \"run\":{},\
                \"runtime\":{}\
            }}",
            mode,
            size,
            block_size,
            run,
            duration.as_secs_f64(),
        )
    ).unwrap();
}
