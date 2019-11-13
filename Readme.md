# Rolling logs for rust
This lib will provide rolling logs for rust as a simple io::Writer so it will be compatible with a lot 
of logging libraries

## Features
These features should be supported
- [x] Splitting into files of similar (configurable) size
- [x] Retiring files of a certain (configurable) age
- [x] Retiring files if a certain amount of files is reached in the directory

These features might be supported
- [] Compression