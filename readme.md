# ValgrindParser
A simple and limited parser for valgrind output.

I created this project when I ran into some undefined behavoir while doing my [winter](https://github.com/fourrth/winter) project.
It found it with very limited effort so I am quite fond of this project.

## Features
This project is quite simple and contains only a basic executable for sifting through large valgrind files.
It is very incomplete and will stay that way becasue it has reached the *good enough point*. 
One thing I may or may not add is terminal input instead of only file input as being able to pipe in valgrind would nice.

## Usage
You can use the program like follows: 

```ValgrindParser --input_dir='my_file.txt' --output_dir='output_log.txt' --glob='glob1,glob2' --pattern_pair=({REGEX})[match1,match2]```

Or for example:
```ValgrindParser --input_dir='my_file.txt' --output_dir='output_log.txt' --glob=*dl-open*,*swrast_dri* --pattern_pair=(\\(.+\\))[*wsl*]```

the ```--input_dir='my_file.txt'``` and ```--output_dir='output_log.txt``` specify the valgrind output for the program and the resulting file

the ```--glob=glob1,glob2``` just removes any lines which match that glob

the ```--pattern_pair=({REGEX})[match1,match2]``` matches some supplied regex and then looks for the specified matches, removing the line if found.

## Build
This uses no libraries outside cargo so simply:

```
git clone https://github.com/fourrth/ValgrindParser.git
cd ValgrindParser
cargo build
```