use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Home{}

pub enum Msg{}

impl Component for Home{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Home{}
    }
    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<Home> for Home {
    fn view(&self) -> Html<Home> {
        html! {
            <div>
                <p>{"Home page"}</p>
            </div>
        }
    }
}