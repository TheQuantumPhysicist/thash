# thash

A fast and versatile terminal hashing program that prints the hash digest to stdout, with many hashing algorithms, many output formats, and support to request the number of iterations for hashing.

## Why?

Basically I wanted something more flexible than `sha256sum`, `md5sum` and the other similar terminal tools. It always helps to have a tool that can do multiple things with hashing, so I spent some time writing a good, reliable program.

## Installation

You can install this with [cargo](https://www.rust-lang.org/tools/install).

```bash
cargo install thash
```

Currently it's only available on crates.io with cargo. If it gets popular enough, I'll happily upload it elsewhere.

## Usage

Run `thash --help`, to see all available options, algorithms, etc.

The data input is passed through stdin. This program can be seen as a drop-in replacement for all `XXXsum` programs on Linux, usage-wise, assuming the defaults are desired.

Anyway, here are some examples on how to use this program, with the expected outcome:

- Hash the string "abc" using blake2b (the default hashing algorithm), and output the hash digest as hex (default output).

```bash
$ echo -n "abc" | thash
ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d17d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923
```

Notice that we use `-n` with `echo` to avoid adding a new line at the end.

- Hash the string "abc" using sha256, and output the hash digest as hex. Remember: The list of all available algorithms can be viewed with `--help`.

```bash
$ echo -n "abc" | thash -a sha256
ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad
```

- Same, but now as different types of outputs

```bash
$ echo -n "abc" | thash -a sha256 -f hex-lower
ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad

$ echo -n "abc" | thash -a sha256 -f hex-upper
BA7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD

$ echo -n "abc" | thash -a sha256 -f base64
ungWv48Bz+pBQUDeXa4iI7ADYaOWF3qctBD/YfIAFa0=

$ echo -n "abc" | thash -a sha256 -f base64-no-pad
ungWv48Bz+pBQUDeXa4iI7ADYaOWF3qctBD/YfIAFa0

$ echo -n "abc" | thash -a sha256 -f base64-url-safe
ungWv48Bz-pBQUDeXa4iI7ADYaOWF3qctBD_YfIAFa0=

$ echo -n "abc" | thash -a sha256 -f base64-url-safe-no-pad
ungWv48Bz-pBQUDeXa4iI7ADYaOWF3qctBD_YfIAFa0
```

You can even write the output as binary with `-f binary`! Obviously, this isn't properly visible in a text file. But you can pipe it to other programs.

- How about hashing "abc" twice using sha256, instead of once?

```bash
$ echo -n "abc" | thash -a sha256 -i2
4f8b42c22dd3729b519ba6f68d2da7cc5b2d606d05daed5ad5128cc03e6c6358
```

This will take the hash digest output, and hash it again with sha256.

- In fact, this is equivalent to this:

```bash
$ echo -n "abc" | thash -a sha256 -f binary | thash -a sha256
4f8b42c22dd3729b519ba6f68d2da7cc5b2d606d05daed5ad5128cc03e6c6358
```

Notice how outputting binary is now useful. You can pipe your data through multiple hash functions.

```bash
$ echo -n "abc" | thash -a sha256 -f binary | thash -a blake2s -f binary | thash -a sha3-224
c287a7a4abc221b50aa406b6b6e47017f0bb5bc354870912fc00588d
```

This will hash "abc" with sha256, hash the binary result with blake2s, then hash the binary result with sha3-224, and print the final result.

## Performance

`thash` beats all regular tools, like `sha256`, `b2sum`, etc, in performance.

We use `hyperfine` for benchmarking. You can install it with `cargo install hyperfine`.

To run benchmark, first, create a file with random data, followed by some hashing program to ensure any caching operation is done:

```bash
dd if=/dev/random of=random-file.bin bs=1M count=1024
cat random-file.bin | sha256sum
```

#### SHA256 performance

```bash
$ hyperfine "cat random-file.bin | thash -a sha256"
Benchmark 1: cat random-file.bin | thash -a sha256
  Time (mean ± σ):     670.7 ms ±   9.0 ms    [User: 499.6 ms, System: 589.4 ms]
  Range (min … max):   654.1 ms … 684.2 ms    10 runs

$ hyperfine "cat random-file.bin | sha256sum"
Benchmark 1: cat random-file.bin | sha256sum
  Time (mean ± σ):      2.928 s ±  0.050 s    [User: 2.795 s, System: 0.497 s]
  Range (min … max):    2.841 s …  3.016 s    10 runs
```

#### SHA512 performance

```bash
$ hyperfine "cat random-file.bin | thash -a sha512"
Benchmark 1: cat random-file.bin | thash -a sha512
  Time (mean ± σ):      1.653 s ±  0.053 s    [User: 1.491 s, System: 0.629 s]
  Range (min … max):    1.608 s …  1.785 s    10 runs

$ hyperfine "cat random-file.bin | sha512sum"
Benchmark 1: cat random-file.bin | sha512sum
  Time (mean ± σ):      2.114 s ±  0.018 s    [User: 1.982 s, System: 0.488 s]
  Range (min … max):    2.088 s …  2.143 s    10 runs
```

#### Blake2b performance

```bash
$ hyperfine "cat random-file.bin | thash -a blake2b"
Benchmark 1: cat random-file.bin | thash -a blake2b
  Time (mean ± σ):     970.0 ms ±  18.7 ms    [User: 796.6 ms, System: 610.0 ms]
  Range (min … max):   951.9 ms … 1011.0 ms    10 runs

$ hyperfine "cat random-file.bin | b2sum"
Benchmark 1: cat random-file.bin | b2sum
  Time (mean ± σ):      1.060 s ±  0.020 s    [User: 0.927 s, System: 0.480 s]
  Range (min … max):    1.036 s …  1.093 s    10 runs
```

#### SHA-1 sum

```bash
$ hyperfine "cat random-file.bin | thash -a sha1"
Benchmark 1: cat random-file.bin | thash -a sha1
  Time (mean ± σ):     652.3 ms ±   7.3 ms    [User: 476.8 ms, System: 600.5 ms]
  Range (min … max):   639.8 ms … 662.3 ms    10 runs

$ hyperfine "cat random-file.bin | sha1sum"
Benchmark 1: cat random-file.bin | sha1sum
  Time (mean ± σ):      1.247 s ±  0.029 s    [User: 1.120 s, System: 0.472 s]
  Range (min … max):    1.209 s …  1.288 s    10 runs
```

#### Notes on performance

On Mac, the performance of `thash` is even better, and is faster for everything. But `md5` is the only exception I found, where it is slower on Linux.
