# secret-sharing-cli

[![Crate](https://badgen.net/crates/v/secret-sharing-cli)](https://crates.io/crates/secret-sharing-cli)
[![License](https://badgen.net/github/license/dcsunset/secret-sharing-cli)](https://github.com/DCsunset/secret-sharing-cli)

A secret sharing command line tool using Shamir's Secret Sharing.

## Install

Make sure you have cargo installed.

```
cargo install secret-sharing-cli
```

## Usage

Run `secret-sharing-cli -h` to see the help message.

For example, to split a string secret that requires 2 shares to recover and generate 4 shares, run:

```
secret-sharing-cli split --string <secret> --threshold 2 --number 4
```

Then to recover the secret, use any two shares:

```
secret-sharing-cli recover --string <share 1> <share 2>
```

Besides, splitting binary files is also supported using `--file` flags.

## Dependencies

Thanks to the following crates:

* [sharks](https://github.com/c0dearm/sharks)
* [clap](https://github.com/clap-rs/clap)
* [base64](https://github.com/marshallpierce/rust-base64)

## License

GPL-3.0 License
