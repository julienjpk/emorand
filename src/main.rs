//! emorand - prints a random emoji to stdout

/* Copyright (C) 2021 Julien JPK (jjpk.me)

 * This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General
 * Public License as published by the Free Software Foundation, either version 3 of the License, or any later version.

 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied
 * warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
 *  details.

 * You should have received a copy of the GNU Affero General Public License along with this program.
 * If not, see <https://www.gnu.org/licenses/>. */

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::ErrorKind::AlreadyExists;

use directories::ProjectDirs;
use rand::prelude::*;
use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use regex::Regex;
use reqwest;

/// Name of the emoji cache file (under the user's emorand cache directory)
static CACHE_FILENAME: &str = "cache.bin";
/// URL to the online plaintext list of emoji sequences
static UNICODE_URL: &str = "https://unicode.org/Public/emoji/13.1/emoji-sequences.txt";

/// Creates the local emoji cache from the online list
fn create_cache(mut cache_rw: std::fs::File) {
    let sequence_regex = Regex::new(r"^[A-F0-9]+(\.\.[A-F0-9]+)? +;").unwrap();
    let response_text = reqwest::blocking::get(UNICODE_URL)
	.expect(&format!("failed to fetch {} to build the emoji cache", UNICODE_URL))
        .text()
        .expect(&format!("failed to parse {} when trying to build the emoji cache", UNICODE_URL));
    let sequence_strings: Vec<&str> = response_text.lines()
        .filter(|line| sequence_regex.is_match(line))
        .map(|line| line.split(";").collect::<Vec<&str>>().get(0).unwrap().trim())
        .collect();

    for sequence_string in sequence_strings.iter() {
	let parts: Vec<&str> = sequence_string.split("..").collect();
	match parts.len() {
	    1 => {
		/* Single code point: one emoji on this line */
		let codepoint = u32::from_str_radix(parts[0], 16)
		    .expect(&format!("invalid code point encountered when building cache: {}", parts[0]));
		cache_rw.write_u32::<NativeEndian>(codepoint)
		    .expect(&format!("failed to write while building emoji cache"));
	    },
	    2 => {
		/* Two code points: this is a code point range */
		let bounds: Vec<u32> = parts.iter().map(
		    |s| u32::from_str_radix(s, 16)
			.expect(&format!("invalid code point interval encountered when building cache: {}",
					 sequence_string)))
		    .collect();

		assert!(bounds[0] <= bounds[1],
			format!("reversed code point interval encountered when building cache: {}", sequence_string));

		for codepoint in bounds[0]..=bounds[1] {
		    cache_rw.write_u32::<NativeEndian>(codepoint)
			.expect(&format!("failed to write while building emoji cache"));
		}
	    },
	    _ => ()
	}
    }
}

fn main() {
    let project_dirs = ProjectDirs::from("me.jjpk", "jjpk", "emorand")
	.expect("failed to determine standard directories on this system");
    let cache_dir = project_dirs.cache_dir();

    std::fs::create_dir_all(cache_dir)
	.expect(&format!("failed to create cache directory {}", cache_dir.to_str().unwrap()));

    let cache_path = cache_dir.join(CACHE_FILENAME);
    let cache_path_str = cache_path.to_str().unwrap();

    /* Create the emoji cache if the file doesn't exist yet */
    match OpenOptions::new().create_new(true).write(true).open(&cache_path) {
	Ok(cache_rw) => create_cache(cache_rw),
	Err(cache_rw_error) => if let AlreadyExists = cache_rw_error.kind() { () } else {
	    panic!("Error while opening cache file at {}: {}", cache_path_str, cache_rw_error)
	}
    }

    /* Get the cache length, check that it's a multiple of 4 (4 bytes per code point) */
    let cache_meta = std::fs::metadata(&cache_path)
	.expect(&format!("failed to get metadata for cache at {}", cache_path_str));
    let cache_bytes = cache_meta.len();

    if cache_bytes == 0 || cache_bytes % 4 != 0 {
	panic!("the emojirand cache file at {} appears to be corrupted (delete it)", cache_path_str)
    }

    /* Pick a random code point index in the file */
    let mut rng = thread_rng();
    let random_byte = rng.gen_range(0..cache_bytes);
    let codepoint_start = random_byte - random_byte % 4;

    let mut cache_ro = std::fs::File::open(&cache_path)
	.expect(&format!("failed to open cache file at {}", cache_path_str));
    cache_ro.seek(SeekFrom::Start(codepoint_start))
	.expect(&format!("failed to seek on emoji cache file at {}", cache_path_str));

    /* Fetch the UTF-32 code point */
    let codepoint = cache_ro.read_u32::<NativeEndian>()
	.expect(&format!("failed to extract emoji from cache file at {}", cache_path_str));

    /* Print the code point (UTF-32 -> UTF-8 in most cases) */
    print!("{}", std::char::from_u32(codepoint).unwrap());
}
