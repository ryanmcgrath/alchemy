#![recursion_limit="256"]

/// demo/main.rs
///
/// Used to sketch out application structure/feel/etc.
///
/// @author Ryan McGrath <ryan@rymc.io>
/// @created March 26th, 2019

use alchemy::{
    App, AppDelegate, Error, rsx, 
    RSX, text, Text, View, Window, WindowDelegate
};

pub struct AppState {
    window: Option<Window>
}

impl AppDelegate for AppState {
    fn did_finish_launching(&mut self) {
        let mut window = Window::new(WindowState {});
        window.set_title("Layout Test");
        window.set_dimensions(100., 100., 600., 600.);
        window.show();
        self.window = Some(window);
        println!("Should be showing");
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
            </View>
        })
    }
}

fn main() {
    App::new(AppState {
        window: None
    }).run()
}
