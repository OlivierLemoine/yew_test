use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Header{}

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Header {}
    }
    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }
}

impl Renderable<Header> for Header {
    fn view(&self) -> Html<Header> {
        html! {
            <nav>
                <div class="nav-wrapper",>
                    <ul id="nav-mobile", class="hide-on-med-and-down",>
                        <li><a href="#/",>{"Home"}</a></li>
                        <li><a href="#/page",>{"Page"}</a></li>
                    </ul>
                </div>
            </nav>
        }
    }
}