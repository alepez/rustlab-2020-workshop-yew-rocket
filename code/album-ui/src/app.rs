use album_db::Images;
use yew::prelude::*;

pub struct App {
    state: State,
}

pub enum Msg {}

#[derive(Default)]
struct State {
    images: Option<Images>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        App {
            state: State::default(),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if let Some(images) = &self.state.images {
            html! {
            <div>{ format!("There are {} images", images.0.len() )}</div>
            }
        } else {
            html! {
            <div>{ "Loading..."}</div>
            }
        }
    }
}
