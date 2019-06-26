use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Page{}

pub enum Msg{}

impl Component for Page{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Page{}
    }
    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<Page> for Page {
    fn view(&self) -> Html<Page> {
        html! {
            <div>
                <p>{"Page page"}</p>
            </div>
        }
    }
}