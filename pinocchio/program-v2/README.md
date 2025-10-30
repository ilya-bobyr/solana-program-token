# `p2-token`

A `pinocchio`-based Token program that uses Pinocchio Account API v2.

## Overview

`p2-token` is a reimplementation of the `p-token` program, which, in turn, is a
reimplementation of the SPL Token program.

As of now, this is done to see how the Account API v2 stacks against the current
Pinocchio Accounts API that is used in the `p-token` program.

We want to measure CU usage and compare it against SPL Token and p-token, as
well as compare the amount of unsafe code compared to the p-token version.

## Features

- `no_std` crate
- Same instruction and account layout as SPL Token
- Minimal CU usage


## License

The code is licensed under the [Apache License Version 2.0](LICENSE)
