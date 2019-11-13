# Rolling logs for rust
This lib will provide rolling logs for rust as a simple io::Writer so it will be compatible with a lot 
of logging libraries

## Features
These features should be supported
1. Splitting into files of similar (configurable) size
1. Retiring files of a certain (configurable) age
1. Retiring files if a certain amount of files is reached in the directory

These features might be supported
1. Compression