#[macro_use] extern crate stdweb;
extern crate yew;

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use crate::stdweb::web::IEventTarget;

mod header;
mod home;
mod page;

use header::Header;
use home::Home;
use page::Page;

enum View{
    None,
    Home,
    Page,
}

struct Model {
    content: View
}

enum Msg {
    URLUpdate(String)
}

impl Component for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            content: View::Home
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg{
            Msg::URLUpdate(new_url) => {
                self.content = match new_url.split("/").last().unwrap() {
                    "" => View::Home,
                    "page" => View::Page,
                    _ => View::None,
                }
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <>
                <Header:/>
                {self.content.view()}
            </>
        }
    }
}

impl Renderable<Model> for View{
    fn view(&self) -> Html<Model> {
        match self{
            View::None => html!{
                <></>
            },
            View::Home => html!{
                <>
                    <Home:/>
                </>
            },
            View::Page => html!{
                <>
                    <Page:/>
                </>
            }
        }
    }
}

fn main() {
    // yew::start_app::<Model>();

    yew::initialize();
    let mut scope = yew::App::<Model>::new().mount_to_body();
    yew::run_loop();

    scope.send_message(Msg::URLUpdate(stdweb::web::window().location().unwrap().hash().unwrap()));

    stdweb::web::window().add_event_listener(move |e:stdweb::web::event::HashChangeEvent| {
        scope.send_message(Msg::URLUpdate(e.new_url()));
    });
}