use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

use globset::GlobSet;
use lazy_static::lazy_static;
use regex::Regex;

use crate::interface;

lazy_static! {
    static ref RE_WHITESPACE: Regex = Regex::new(r"(?:==)(?:\d+)(?:==)\W+$").unwrap();
}

pub struct Context {
    reader: BufReader<File>,
    pub writer: BufWriter<File>,

    global_glob: GlobSet,

    pattern_pair_lines: Vec<Regex>,
    pattern_pair_globsets: Vec<GlobSet>,

    string_buf: String,
    string_buf_offset: usize,

    pub cnt: usize,
}

impl Context {
    pub fn new<'a>(interface: interface::Interface) -> Self {
        Self {
            reader: BufReader::new(interface.input_file),
            writer: BufWriter::new(interface.output_file),

            global_glob: interface.global_glob,

            pattern_pair_lines: interface.patternpair_pattern_lines,
            pattern_pair_globsets: interface.patternpair_glob_lists,

            string_buf: String::new(),
            string_buf_offset: 0,
            cnt: 0,
        }
    }
    /// Process the next block
    /// Returns Some(()) if we still have more blocks
    /// None if we are out of blocks to process
    /// Calling after this will just result in more None s
    /// Note that io errors are panics
    pub fn process_next_block(&mut self) -> Option<()> {
        self.string_buf.clear();
        self.string_buf_offset = 0;

        // At this point, we should be primed to enter a block
        // this means that the next time we do read_line, it will
        // load in a header

        // the first line gotten should
        // be the header of the current block
        // we are going to consider all lines after this
        // to be apart of the same block, until we
        // reach an empty, newline block

        let mut should_write = true;
        loop {
            let seek_amt = self.reader.read_line(&mut self.string_buf).unwrap();
            if seek_amt == 0 {
                // means we ran out of file
                // so print what we got left and return None
                if should_write {
                    self.writer.write_all(self.string_buf.as_bytes()).unwrap();
                }
                return None;
            }
            // self.reader.seek_relative(seek_amt as i64).unwrap();
            // Otherwise, we just read an entire line into string_buf
            // Now we could have read either a header, a fullpath, or an empty line
            if self.match_illegals() {
                // means we got an actual fullpath
                // for now, this is no different than
                // anything else and we just keep it
                self.string_buf_offset += seek_amt;
                should_write = false;
            } else {
                // Means that we either got a header or whitespace
                if RE_WHITESPACE.is_match_at(&self.string_buf, self.string_buf_offset) {
                    // if we got white space, then we are done and we should stop looping
                    // so we need to write out (process block cleans for us tho)
                    if should_write {
                        self.writer.write_all(self.string_buf.as_bytes()).unwrap();
                    }
                    return Some(());
                }
                // Then this means that we got a header,
                // so we just increment the offset
                self.string_buf_offset += seek_amt;
            }
        }
    }
    // matches to see if the string_buf has any illegals
    // using self.string_buf_offset
    fn match_illegals(&mut self) -> bool {
        // here, string_buf is just whatever text we are doing currently

        let view = &self.string_buf[self.string_buf_offset..];

        // Okay, behavior...
        // first we check for global globs

        if self.global_glob.is_match(view) {
            self.cnt += 1;
            return true;
        }

        // now lets check for the pattern pairs

        debug_assert_eq!(
            self.pattern_pair_globsets.len(),
            self.pattern_pair_lines.len()
        );
        for cx in 0..self.pattern_pair_globsets.len() {
            if let Some(mat) = self.pattern_pair_lines[cx].find(view) {
                if self.pattern_pair_globsets[cx].is_match(mat.as_str()) {
                    self.cnt += 1;
                    return true;
                }
            }
        }

        /* if let Some(captures) = RE_FULLPATH.captures_at(&self.string_buf, self.string_buf_offset) {
            // fullpath
            let cap = captures.get(1).unwrap().as_str();
            if self.global_glob.is_match(cap) {
                self.cnt += 1;
                return true;
            }
        } */
        false
    }
}
