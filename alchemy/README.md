# Alchemy Core
This crate implements the core Alchemy application, which is what users ultimately import. Applications are a singleton; some might not like this, but it enables a design pattern that meshes a bit better with existing GUI framework systems and patterns.

The general pattern for developing with Alchemy is as follows:

``` bash
[Alchemy API] -> [Inner Mutability] -> [Platform Bridge (implemented in other crates)]
    |
    |
    |- [Delegate]
```

The delegate pattern is cribbed from AppKit/UIKit, where it tends to work quite nicely as a way to respond to system level events.

## Questions, Comments?
Open an issue, or hit me up on [Twitter](https://twitter.com/ryanmcgrath/).
