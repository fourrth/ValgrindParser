# ValgrindParser

A simple, yet limited parser for valgrind output.

I created this project when I ran into some undefined behavoir while doing my [winter](https://github.com/fourrth/winter) project.
It found it with very limited effort so I am quite fond of this project.

## Usage

First, clone the repository,

```
https://github.com/fourrth/ValgrindParser.git
```

and then install

```
cd ValgrindParser
cargo install 
```

It is used in the form:

```
ValgrindParser --input_dir='my_file.txt' --output_dir='output_log.txt' --glob='glob1,glob2' --pattern_pair=({REGEX})[match1,match2]
```
In more detail:

### --input_dir (*required*)
Specifies the input file

### --output_dir (*required*)
Specifies the output file

<br>
One of the following is required

### --glob
This removes any lines which match that glob

### --pattern_pair
This matches some supplied regex and then looks for the specified matches, removing the line if found.

## Examples
You can use the program like follows: 

```
ValgrindParser --input_dir='my_file.txt' --output_dir='output_log.txt' --glob=*dl-open*,*swrast_dri* --pattern_pair=(\\(.+\\))[*wsl*]
```

## License

This crate — along with any subsequent additions or revisions — are all dual licensed under [MIT License](LICENSE-MIT) or [Apache License](LICENSE-APACHE) at your option.