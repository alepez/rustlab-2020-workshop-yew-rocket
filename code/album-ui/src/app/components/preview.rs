use crate::app::worker::{self, Worker};
use album_db::Image;
use yew::prelude::*;

pub struct Preview {
    props: Props,
    _worker: Box<dyn Bridge<Worker>>,
}

pub enum Msg {
    WorkerRes(worker::Response),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub image: Image,
}

impl Component for Preview {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let worker = Worker::bridge(link.callback(Msg::WorkerRes));

        Preview {
            props,
            _worker: worker,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::WorkerRes(res) => match res {
                _ => false,
            },
        }
    }

    fn view(&self) -> Html {
        let src = format!("/api/images/{}/preview.jpg", self.props.image.id);
        html! {
        <div class="album-preview">
            <div class="album-toolbar">
                <button>{ "Delete" }</button>
            </div>
            <img src=src />
        </div>
        }
    }
}
