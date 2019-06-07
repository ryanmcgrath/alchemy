+++
title = "Alchemy - A Rust GUI Framework"
template = "index.html"
+++

# A New Rust GUI Framework
Alchemy is a Rust GUI Framework, backed by native widgets on each platform it supports, with an API that's a blend of those found in AppKit, UIKit, and React Native. It supports a JSX-ish syntax (RSX), styling with CSS, the safety of building in Rust, and a familiar API for many developers who build UI on a daily basis. The goal is to provide an API that feels at home in Rust, while striving to provide a visual appearance that's easy to scan and parse. It does not, and will never, require nightly. It's still early stages, but feedback and contributions are welcome.


## What's It Look Like?
``` rust
use alchemy::{
    AppDelegate, Error, RSX, rsx, 
    styles, View, Window, WindowDelegate
};

struct AppState {
    window: Window
}

impl AppDelegate for AppState {
    fn did_finish_launching(&mut self) {
        self.window.set_title("LOL");
        self.window.set_dimensions(10., 10., 600., 600.);
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
            margin-top: 10;
            padding-top: 10;
        }

        innerbox {
            background-color: #003366;
            width: 200;
            height: 200;
        }
    });

    app.run(AppState {
        window: Window::new(WindowState {
        
        })
    });
}
```

<div id="tempGetStarted">
<h2>Get Started</h2>
<a href="https://github.com/ryanmcgrath/alchemy/" title="Browse the Alchemy Source Code on GitHub" class="getStartedBtn gh">GitHub</a>
<a href="https://docs.rs/alchemy/" title="Read the Alchemy Documentation on docs.rs" class="getStartedBtn">Docs</a>
</div>
