[![Alchemy](https://github.com/ryanmcgrath/alchemy/blob/trunk/assets/alchemy_logo_250x.png?raw=true =100x100)](https://alchemy.rs)

A Rust GUI Framework
==========================================================

[![Gitter](https://badges.gitter.im/alchemy-rs/alchemy.svg)](https://gitter.im/alchemy-rs/alchemy)
[![Crates.io](https://img.shields.io/crates/v/alchemy.svg)](https://crates.io/crates/alchemy)

API Documentation: [latest release](https://docs.rs/alchemy) – [trunk branch](https://docs.alchemy.rs)

[Homepage](https://alchemy.rs)

Alchemy is a Rust GUI Framework, backed by native widgets on each platform it supports, with an API that's a blend of those found in AppKit, UIKit, and React Native. It aims to provide an API that feels at home in Rust, while striving to provide a visual appearance that's easy to scan and parse. It does not, and will never, require nightly. It's still early stages, but feedback and contributions are welcome.

## Supported Platforms
Alchemy will, ideally, support the platforms listed below. At the moment, the `Cocoa` backend is the most complete, as I develop on a Mac and know the framework more than I'd care to admit. This list will be updated as more frameworks are added.

- `cocoa`, which provides backing widgets, windows and assorted frameworks for `macOS`.
- `cocoa-touch`, which provides backing widgets, windows and assorted frameworks for `iOS`.
- `gtk`, which affords a `GTK` layer. This is mostly intended for GNOME users; if you'd like to run it elsewhere, you're on your own.
- `qt`, which affords a `Qt` layer. This is mostly indended for `KDE` users; if you'd like to run it elsewhere, you're on your own.
- `uwp`, which affords a `"UWP"` layer for Microsoft platforms that support it. This is a bit of a hack, provided by linking into the [microsoft/WinObjC](https://github.com/Microsoft/WinObjC/) framework, originally inteded for porting `iOS` applications to `UWP`. Down the road, if or when a proper `UWP` library for Rust surfaces, I'd be happy to look at replacing this.

Support for more platforms is desired - for example, I think an [`OrbTk`](https://gitlab.redox-os.org/redox-os/orbtk) or [`Piston`](https://www.piston.rs) backend could be cool to see. A [`winapi-rs`](https://github.com/retep998/winapi-rs) backend could be cool, too!

## What Currently Works...?
At the moment, the following is implemented:

- A basic `cocoa` API, which implements the `Application` and `Window` lifecycles. `View`s are supported as well.
- A basic `reconciliation` module, which handles computing changes to the widget tree and applying them as necessary. It currently follows a design similar to React pre-16; I'm open to changing this if someone wants to collaborate.
- A CSS parser, based on the work done over in [servo/servo](https://github.com/servo/servo). It doesn't support cascading, and follows an API closer to that of React Native's. This is intentional.
- An RSX system, based on work done in [bodil/typed-html](https://github.com/bodil/typed-html) by Bodil Stokke. This was actually the project that made me circle back to the entire thing, too.
- Macros for easy UI construction - `rsx! {}`, which transforms JSX-ish syntax into element trees for the reconciler to work with, and `styles! {}`, which pre-process CSS into their styles.
- A CSS layout system, based off the work done over in [vislyhq/stretch](https://github.com/vislyhq/stretch). At the moment, this project includes a fork with a newer underlying API by [msiglreith](https://github.com/msiglreith/stretch/tree/index). Once the API is merged upstream, it's likely the dependency would change to `stretch` proper.

## What's it look like?
``` rust
use alchemy::{AppDelegate, Error, RSX, rsx, styles, View, Window, WindowDelegate};

struct AppState {
    window: Window
}

impl AppDelegate for AppState {
    fn did_finish_launching(&mut self) {
        self.window.show();
    }
}

struct WindowState;

impl WindowDelegate for WindowState {
    fn render(&self) -> Result<RSX, Error> {
        Ok(rsx! {
            <View styles=["box"]>
                <View styles=["innerbox"] />
            </View>
        })
    }
}

fn main() {
    let app = alchemy::shared_app();

    app.register_styles("default", styles! {
        box {
            background-color: #307ace;
            width: 300;
            height: 300;
            margin: 10;
            padding: 10;
        }

        innerbox {
            background-color: #003366;
            width: 200;
            height: 200;
        }
    });

    alchemy::shared_app().run(AppState {
        window: Window::new("Le Appy App", (0., 0., 600., 600.), WindowState {})
    });
}
```

## Does it support custom Components?
Alchemy implements the React component lifecycle. It does not (currently) implement Hooks, and may or may not implement them in the future. The class-based lifecycle maps fairly well to Rust idioms already, as you really never wanted to subclass in React anyway.

A custom component would look like the following:

``` rust
use alchemy::{Component, Error, rsx, RSX};

pub struct MySpecialWidget {
    your_special_value_or_whatever: i32
}

impl Component for MySpecialWidget {
    fn component_did_mount(&mut self, props: &Props) {
        // Do whatever you want. Fire a network request or something, I dunno.
    }

    fn render(&self, props: &Props) -> Result<RSX, Error> {
        Ok(RSX::None)
    }
}
```

Rust allows the lifecycle to have a few cool guarantees that you can't really get in JavaScript - for instance, props don't actually belong to you... but it was a weird aspect of class-based components in JavaScript where you'd be able to arbitrarily call `this.props.whatever`. Function based components actually communicated it better, in that they were passed in - with Rust, it's very clear that you just get a reference.

Alchemy follows [this diagram of React's lifecycle methods](https://twitter.com/dan_abramov/status/981712092611989509) to a T for the most part. What's cool is that methods that shouldn't have side effects, we can call as straight up borrows... and the ones that are allowed to have mutable side effects, we can call them as `&mut self`. You can, of course, still incur side effects by doing something else, but being able to imply the intention directly in the API is kind of cool.

## License
I'm dual licensing this, due to the licenses that some of the projects it depends on being that. If there's some other (more appropriate) way to do this, please feel free to open an issue.

 * Mozilla Public License, Version 2.0, ([LICENSE-MPL](LICENSE-MPL.md) or https://www.mozilla.org/en-US/MPL/)
 * MIT License ([LICENSE-MIT](LICENSE-MIT.md) or https://opensource.org/licenses/MIT)

### Contributing
Before contributing, please read the [contributors guide](https://github.com/ryanmcgrath/alchemy/blob/master/CONTRIBUTING.md) 
for useful information about setting up Alchemy locally, coding style and common abbreviations.

Unless you explicitly state otherwise, any contribution you intentionally submit
for inclusion in the work, should be dual-licensed as above, without any additional terms or conditions.

## Notes
- Major thanks to [David McNeil](https://github.com/davidMcneil) for graciously allowing me to take the `alchemy` name on crates.io. Hot take, if we had user namespacing, this wouldn't be an issue!
- Cheers to [diesel-rs/diesel](https://github.com/diesel-rs/diesel), who have a very well laid out repository that a bunch of this structure was cribbed from.
- Questions or comments that you don't think warrant an issue? Feel free to [poke me over on Twitter](https://twitter.com/ryanmcgrath/) or email me ([ryan@rymc.io](mailto:ryan@rymc.io)).
