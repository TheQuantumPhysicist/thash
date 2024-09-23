# thash

A terminal hashing program that prints the hash digest to stdout, with many hashing algorithms, many output formats, and support to request the number of iterations for hashing.

## Why?

Basically I wanted something more flexible than `sha256sum`, `md5sum` and the other similar terminal tools. It always helps to have a tool that can do multiple things with hashing, so I spent some time writing a good, reliable program.

## Installation

You can install this with [cargo](https://www.rust-lang.org/tools/install).

```
cargo install thash
```

Currently it's only available on crates.io with cargo. If it gets popular enough, I'll happily upload it elsewhere.

## Usage

Run `thash --help`, to see all available options, algorithms, etc.

The data input is passed through stdin. This program can be seen as a drop-in replacement for all `XXXsum` programs on Linux, usage-wise, assuming the defaults are desired.

Anyway, here are some examples on how to use this program, with the expected outcome:

- Hash the string "abc" using blake2b (the default hashing algorithm), and output the hash digest as hex (default output).

```
$ echo -n "abc" | thash
ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d17d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923
```

Notice that we use `-n` with `echo` to avoid adding a new line at the end.

- Hash the string "abc" using sha256, and output the hash digest as hex. Remember: The list of all available algorithms can be viewed with `--help`.

```
$ echo -n "abc" | thash -a sha256
ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad
```

- Same, but now as different types of outputs

```
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

```
$ echo -n "abc" | thash -a sha256 -i2
4f8b42c22dd3729b519ba6f68d2da7cc5b2d606d05daed5ad5128cc03e6c6358
```

This will take the hash digest output, and hash it again with sha256.

- In fact, this is equivalent to this:

```
$ echo -n "abc" | thash -a sha256 -f binary | thash -a sha256
4f8b42c22dd3729b519ba6f68d2da7cc5b2d606d05daed5ad5128cc03e6c6358
```

Notice how outputting binary is now useful. You can pipe your data through multiple hash functions.

```
$ echo -n "abc" | thash -a sha256 -f binary | thash -a blake2s -f binary | thash -a sha3-224
c287a7a4abc221b50aa406b6b6e47017f0bb5bc354870912fc00588d
```

This will hash "abc" with sha256, hash the binary result with blake2s, then hash the binary result with sha3-224, and print the final result.
