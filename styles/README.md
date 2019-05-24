# Alchemy-Styles
This crate implements CSS parsing and Flexbox layout. CSS parsing relies on the [CSS Parser from Servo](https://github.com/servo/rust-cssparser). Flexbox is implemented with [Stretch](https://github.com/vislyhq/stretch), albeit currently [a fork by msilgreith](https://github.com/msiglreith/stretch/tree/index), cloned into here to serve a few small changes (a change for more thread safety, and to push appearance based styles that Flexbox doesn't concern itself with). Down the road, I could see this not including Stretch inline.

## Questions, Comments?
Open an issue, or hit me up on [Twitter](https://twitter.com/ryanmcgrath/).
