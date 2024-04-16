use std::{fs::File, path::PathBuf, str::FromStr};

use globset::{Glob, GlobSet};
use regex::Regex;

mod input;

/// This basically just handles all of the user
/// stuff and makes sure the binary is good as that
/// your actually able to use it.
pub struct Interface {
    // The default things to pass to the context
    pub input_file: File,
    pub output_file: File,

    pub patternpair_pattern_lines: Vec<Regex>,
    pub patternpair_glob_lists: Vec<GlobSet>,

    pub global_glob: GlobSet,
}

impl Interface {
    /// sets everything up for you based on inputted kind
    pub fn new() -> Self {
        // assume the first is the path/cwd of executable
        let args_in = std::env::args().skip(1);

        let mut input_file: Option<PathBuf> = None;
        let mut output_file: Option<PathBuf> = None;

        let mut pattern_lines = vec![];
        let mut pattern_globsets = vec![];

        let mut global_globs = GlobSet::builder();

        for argument in args_in {
            // we now parse the arguments
            // every argument must start with a `--`
            if let Some((_, care)) = argument.split_once("--") {
                if let Some((action, arg)) = care.split_once("=") {
                    // action is now what we want to do

                    if let Some(mat) = input::HASHMAP.get(action) {
                        // now we have what the user wanted to do as our Args enum
                        match mat {
                            &input::Args::InputDir => {
                                if input_file.is_some() {
                                    panic!("Cannot have two input files");
                                } else {
                                    input_file = Some(PathBuf::from_str(arg).unwrap());
                                }
                            }
                            &input::Args::OutputDir => {
                                if output_file.is_some() {
                                    panic!("Cannot have two output files");
                                } else {
                                    output_file = Some(PathBuf::from_str(arg).unwrap());
                                }
                            }
                            &input::Args::PatternPair => {
                                if let Some(captures) = input::RE_PATTERNPAIR_REGEX.captures(arg) {
                                    assert!(captures.len() == 3); // 1 for full match, 2 for rest

                                    let glob_list = captures.get(2).unwrap().as_str();
                                    // this is just like `*wsl*,*thingy.exe,lib*.lib,` or whatever
                                    let mut globs_out = GlobSet::builder();
                                    for glob in input::RE_PATTERPAIR_GLOBLIST
                                        .find_iter(glob_list)
                                        .map(|mat| mat.as_str())
                                        .map(|mat| Glob::new(mat).unwrap())
                                    {
                                        globs_out.add(glob);
                                    }

                                    let regex = captures.get(1).unwrap().as_str();

                                    // regex should just be valid regex, so lets just assume that
                                    // regex is without \xxxxxxx\ and is just *blah*

                                    match Regex::from_str(regex) {
                                        Ok(re_out) => {
                                            //
                                            pattern_lines.push(re_out);
                                            pattern_globsets.push(globs_out.build().unwrap());
                                        }
                                        Err(e) => {
                                            //
                                            panic!("Could not create regex: {e}");
                                        }
                                    }
                                } else {
                                    unimplemented!("Regex did not match PatternPair Captures");
                                }
                            }
                            &input::Args::GlobalGlob => {
                                // okay let's seperate it into globs
                                for ca in arg.split(",") {
                                    match Glob::new(ca) {
                                        Ok(glob) => {
                                            global_globs.add(glob);
                                        }
                                        Err(e) => {
                                            panic!("Could not create glob: {e}");
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        // Now, if it is not in here,
                        // then we are either doing something wrong,
                        // or we were given an incorrect input
                        // we will just panic because why not: we are testing afterall
                        // these panics will 100% be fixed and will not be here forever, right?

                        unimplemented!(
                            "I think you gave an incorrect argument. Valid arguments are: {}",
                            input::HASHMAP_VALID_ARGS.as_str()
                        );
                    }
                } else {
                    // same as below I think
                    unimplemented!("after the -- we did not find a == so we just panic");
                }
            } else {
                // in the future, we will just
                // look for other things,
                // but idc right now so lets just panic
                unimplemented!("Did not have a -- so we just panic");
            }
        }
        // at the end of everything,
        // the top variables of vec s should be filled

        // we should have an output_file
        if output_file.is_none() {
            // means we will do the default
            // TODO: logging that we are doing the default (and logging in general)
            output_file = Some(PathBuf::from_str("./log.txt").unwrap());
        }

        // we should have an input_file as well
        if input_file.is_none() {
            // means we just panic for now,
            panic!("You must provide an input file");
        }

        // at this point, everything that is panic-like
        // should have panicked by now or we are all good
        let fin = match File::open(input_file.unwrap()) {
            Ok(fin) => fin,
            Err(e) => {
                panic!("Could not find file: {e}");
            }
        };

        let fout = match File::create(output_file.unwrap()) {
            Ok(fout) => fout,
            Err(e) => {
                panic!("Could not create file: {e}");
            }
        };
        let global_glob_out = global_globs.build().unwrap();
        Self {
            input_file: fin,
            output_file: fout,
            global_glob: global_glob_out,
            patternpair_glob_lists: pattern_globsets,
            patternpair_pattern_lines: pattern_lines,
        }
    }
}
