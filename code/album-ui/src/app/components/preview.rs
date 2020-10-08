use crate::app::worker::{self, Worker};
use album_db::Image;
use yew::prelude::*;
use yewtil::NeqAssign;

pub struct Preview {
    props: Props,
    link: ComponentLink<Preview>,
    worker: Box<dyn Bridge<Worker>>,
}

pub enum Msg {
    DeleteClicked,

    WorkerRes(worker::Response),
}

#[derive(Properties, Clone, Eq, PartialEq)]
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
            link,
            worker,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DeleteClicked => {
                log::info!("Delete {:?}", self.props.image);
                self.worker
                    .send(worker::Request::DeleteImage(self.props.image));
                false
            }
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
                <button onclick=self.link.callback(|_| Msg::DeleteClicked)>{ "Delete" }</button>
            </div>
            <img src=src />
        </div>
        }
    }
}
