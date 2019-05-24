# Alchemy-Macros
This crate holds macros for two things, primarily:

- `rsx! {}`, which transforms `<View></<View>` tags into their proper `RSX` calls. Much of this is forked from the awesome work done by [Bodil Stokke in typed-html](https://github.com/bodil/typed-html).
- `styles! {}`, which transforms CSS style nodes into `Vec<Styles>`, which the rendering engine uses to theme and style nodes. This relies on the [CSS Parser from Servo](https://github.com/servo/rust-cssparser). Styles do not support cascading; this is a design decision, as inheritance is already a bit of a taboo in Rust, so to do it in styling code feels really odd and involves a mental shift the deeper you go. Opt to apply successive style keys, conditionally if need be, to achieve the same thing with a compositional approach.

## Questions, Comments?
Open an issue, or hit me up on [Twitter](https://twitter.com/ryanmcgrath/).
