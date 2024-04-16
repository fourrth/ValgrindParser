use lazy_static::lazy_static;

use std::collections::HashMap;

use regex::Regex;

pub enum Args {
    InputDir,
    OutputDir,
    PatternPair,
    GlobalGlob,
}

/*
 * okay, so what is the format for the program args
 * I think just basic stuff, where we have --Thing_[Thingextra]='shell evaled string as next arg'
 * I don't think it requires anything complicated,
 * what flags are going to exist?
 * --input_dir=`relative or abosolute directory`
 * --output_dir=`relative or absolute directory`
 * defaults to log.txt in cwd
 * --pattern_pair='(`a custom line, regex pattern, checked by matches not captures`)[`a list of glob patterns to check in that line pattern, seperated by ,`]'
 * --glob=`global glob pattern to check on every line`
 *
 * Then a list of possible arguments which are default filters
 * However, I don't really know how valgrind works and everything that it can put out,
 * so I won't do any of this right now
 * The following are not implimented, but may be implemented in the future
 */

// Change later to something actually baked into the binary
// and not this lazy static stuff
lazy_static! {

    pub static ref RE_PATTERNPAIR_REGEX: Regex = Regex::new(r"\s?\((.+)\)\s?\[(.+)\]$").unwrap();

    // this one used matches instead of captures because it
    // was too fidily to figure out how
    // to make it work similarly to the others
    pub static ref RE_PATTERPAIR_GLOBLIST: Regex = Regex::new(r"[^,]+").unwrap();

    pub static ref HASHMAP: HashMap<&'static str, Args> = {
        let mut m = HashMap::new();

        m.insert("input_dir",Args::InputDir);
        m.insert("output_dir", Args::OutputDir);
        m.insert("pattern_pair",Args::PatternPair);
        m.insert("glob",Args::GlobalGlob);
        m
    };

    pub static ref HASHMAP_VALID_ARGS: String = {
        let mut ret = String::new();
        for ca in HASHMAP.iter().map(|(&s, _)| s) {
            ret.push_str(ca);
            ret.push_str(", ");
        }
        ret
    };



}
