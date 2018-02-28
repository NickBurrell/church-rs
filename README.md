
# Church
> A minimalist functional programming language

The Church Programming Language is a minimalist programming language based off of the original syntax of Alonzo Churches lambda calculus.

## Installation
### Building from Source
#### Building on MacOS/Linux
 1) Dependencies
	 * `rustup` or `rustc` running on the latest nightly build
	 * GNU `make`
	 * `git`
 2) Clone the [source] with `git`:
 ```sh
$ git clone https://github.com/zero-frost/church-rs/tree/dev.git
$ cd church-rs
```
[source]: https://github.com/zero-frost/church-rs
3) Build:
```bash
$ make
# or if you use cargo/rustup
$ cargo build
```
#### Windows
1) Dependencies 
	* `cygwin`or Windows Subsystem Linux
	* `rustc`/`cargo` or `rustup` (either for Cygwin or WSL)
	* GNU `make` (if on WSL or Cygwin)
2) Clone the [source] with `git`:
 ```sh
$ git clone https://github.com/zero-frost/church-rs/tree/dev.git
$ cd church-rs
```
[source]: https://github.com/zero-frost/church-rs
3) Build
```bash
$ make
# or if you use cargo
$ cargo build
``` 

### Building Documentation
```bash
$ cargo doc
```
## License
This project is licensed under Mozilla Public License 2.0 ([LICENSE])

[LICENSE]: https://github.com/zero-frost/church-rs/blob/dev/LICENSE
