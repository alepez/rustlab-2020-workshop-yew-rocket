mod components;
mod worker;

use album_db::Images;
use components::Preview;
use std::rc::Rc;
use worker::Worker;
use yew::prelude::*;

pub struct App {
    state: State,
    _worker: Box<dyn Bridge<Worker>>,
}

pub enum Msg {
    WorkerRes(worker::Response),
}

#[derive(Default)]
struct State {
    images: Option<Rc<Images>>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut worker = Worker::bridge(link.callback(Msg::WorkerRes));
        worker.send(worker::Request::GetImages);

        App {
            state: State::default(),
            _worker: worker,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::WorkerRes(res) => match res {
                worker::Response::ImagesLoaded(images) => {
                    self.state.images = Some(images);
                    true
                }
                _ => false,
            },
        }
    }

    fn view(&self) -> Html {
        if let Some(images) = &self.state.images {
            html! {
            <>
                <div>{ format!("There are {} images", images.0.len() )}</div>
                { for images.0.iter().map(|image| { html! { <Preview image=image /> }}) }
            </>
            }
        } else {
            html! {
            <div>{ "Loading..."}</div>
            }
        }
    }
}
