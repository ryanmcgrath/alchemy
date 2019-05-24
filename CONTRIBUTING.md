# Contributing

Thanks for your interest in contributing to Alchemy! We very much look forward to
your suggestions, bug reports, and pull requests and so on.

*Note:* Anyone who interacts with Alchemy in any space, including but not
limited to this GitHub repository, must follow our [code of
conduct](https://github.com/ryanmcgrath/alchemy/blob/master/code_of_conduct.md).


## Submitting bug reports

Have a look at our [issue tracker](https://github.com/ryanmcgrath/alchemy/issues). If you can't find an issue (open or closed)
describing your problem (or a very similar one) there, please open a new issue with
the following details:

- Which versions of Rust and Alchemy are you using?
- Which feature flags are you using?
- What are you trying to accomplish?
- What is the full error you are seeing?
- How can we reproduce this?
  - Please quote as much of your code as needed to reproduce (best link to a
    public repository or [Gist])
  - Please post as much of your database schema as is relevant to your error

[issue tracker]: https://github.com/ryanmcgrath/alchemy/issues
[Gist]: https://gist.github.com

Thank you! We'll try to respond as quickly as possible.


## Submitting feature requests

If you can't find an issue (open or closed) describing your idea on our [issue
tracker], open an issue. Adding answers to the following
questions in your description is +1:

- What do you want to do, and how do you expect Alchemy to support you with that?
- How might this be added to Alchemy?
- What are possible alternatives?
- Are there any disadvantages?

Thank you! We'll try to respond as quickly as possible.


## Contribute code to Alchemy

### Setting up Alchemy locally

1. Install Rust using [rustup], which allows you to easily switch between Rust
   versions. Alchemy currently supports Rust Stable, but it's worthwhile in case
   we want to test against beta versions.

2. Clone this repository and open it in your favorite editor.
3. Now, try running the test suite to confirm everything works for you locally
   by executing `bin/test`. (Initially, this will take a while to compile
   everything.)

[rustup]: https://www.rustup.rs

### Coding Style

We follow the [Rust Style Guide](https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/guide/guide.md), enforced using [rustfmt](https://github.com/rust-lang-nursery/rustfmt).
In a few cases, though, it's fine to deviate - a good example is branching match trees, like the CSS parsing.

To run rustfmt tests locally:

1. Use rustup to set rust toolchain to the version specified in the
   [rust-toolchain file](./rust-toolchain).

2. Install the rustfmt and clippy by running
   ```
   rustup component add rustfmt-preview
   rustup component add clippy-preview
   ```

3. Run clippy using cargo from the root of your alchemy repo.
   ```
   cargo clippy
   ```
   Each PR needs to compile without warning.

4. Run rustfmt using cargo from the root of your alchemy repo.

   To see changes that need to be made, run

   ```
   cargo fmt --all -- --check
   ```

   If all code is properly formatted (e.g. if you have not made any changes),
   this should run without error or output.
   If your code needs to be reformatted,
   you will see a diff between your code and properly formatted code.
   If you see code here that you didn't make any changes to
   then you are probably running the wrong version of rustfmt.
   Once you are ready to apply the formatting changes, run

   ```
   cargo fmt --all
   ```

   You won't see any output, but all your files will be corrected.

You can also use rustfmt to make corrections or highlight issues in your editor.
Check out [their README](https://github.com/rust-lang-nursery/rustfmt) for details.


### Notes
This project prefers verbose naming, to a certain degree - UI code is read more often than written, so it's 
worthwhile to ensure that it scans well.
