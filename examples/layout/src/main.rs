#![recursion_limit="256"]

/// demo/main.rs
///
/// Used to sketch out application structure/feel/etc.
///
/// @author Ryan McGrath <ryan@rymc.io>
/// @created March 26th, 2019

use alchemy::{
    AppDelegate, Component, ComponentKey, Fragment, Error, Props, rsx, RSX, styles, text,  
    Text, View, Window, WindowDelegate
};

pub struct AppState {
    window: Window
}

impl AppDelegate for AppState {
    fn did_finish_launching(&mut self) {
        self.window.set_title("Layout Test");
        self.window.set_dimensions(100., 100., 600., 600.);
        self.window.show();
    }
}

#[derive(Default)]
struct BannerProps {}

#[derive(Props)]
struct Banner;

impl Component for Banner {
    fn new(_key: ComponentKey) -> Banner {
        Banner {}
    }

    fn render(&self, children: Vec<RSX>) -> Result<RSX, Error> {
        Ok(rsx! {
            <Fragment>
                <View styles=["wut1"]></View>
                {children}
            </Fragment>
        })
    }
}

pub struct WindowState;

impl WindowDelegate for WindowState {
    fn will_close(&mut self) {
        println!("Closing!?");
    }

    fn render(&self) -> Result<RSX, Error> {
        let messages = vec!["LOL"]; //, "wut", "BERT"];
        Ok(rsx! {
            <View styles={&messages}>
                <Text styles=["message"]>"Hello there, my name is Bert"</Text>
                <View styles=["boxxx"] />
                {messages.iter().map(|message| rsx! {
                    <Text styles=["text"]>{text!("{}", message)}</Text>
                })}
                <View styles=["box1"]>
                    <Banner>
                        <View styles=["innermostBox"] />
                    </Banner>
                </View>
            </View>
        })
    }
}

fn main() {
    let app = alchemy::shared_app();

    app.register_styles("default", styles! {
        root { background-color: #000; }

        LOL {
            background-color: #307ace;
            width: 500;
            height: 230;
            padding-top: 20;
            padding-left: 20;
            padding-right: 40;
        }
        
        message { width: 500; height: 100; background-color: yellow; color: black; }
        text { width: 500; height: 100; background-color: blue; color: white; }

        boxxx {
            background-color: rgba(245, 217, 28, .8);
            width: 100;
            height: 100;
            margin-top: 40;
            margin-right: 20;
        }

        box1 {
            background-color: #f51c69;
            width: 250;
            height: 100;
        }

        wut1 {
            background-color: black;
            width: 50;
            height: 230;
        }

        innermostBox {
            background-color: green;
            width: 20;
            height: 20;
        }
    });
    
    app.run(AppState {
        window: Window::new(WindowState {
        
        })
    });
}
