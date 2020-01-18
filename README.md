# Classy Types

## Description
Classy Types processes files containing TypeScript interfaces and types to generate Class-based equivalents

## Build instructions
* Install Rust toolchain first following instructions [here](https://www.rust-lang.org/tools/install)
* `git clone https://github.com/hgrsd/classy_types`
* `cd classy_types`
* `./build.sh` (or buid manually using `cargo build --release`, which will output to `target/`

## Use
`./classy_types [-o output_folder] <file pattern>`

Examples:
```bash
./classy_types "index.d.ts" # prints processed version of index.d.ts to stdout
./classy_types -o ./ "index.d.ts" # outputs "classes-index.d.ts" to working directory
./classy_types -o ./classes "index.d.ts" # outputs "classes-index.d.ts" to ./classes
./classy_types -o ./classes "*.d.ts" # same as above, processing all *.d.ts files in current directory 
./classy_types -o ./classes "**/*.d.ts" # as above, but recursively processes all .d.ts files starting from current directory 
```

## Tests
Tests can be run using `cargo test` from the project root dir.

