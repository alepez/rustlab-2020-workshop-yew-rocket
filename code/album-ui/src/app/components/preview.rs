use crate::app::worker::{self, Worker};
use album_db::Image;
use yew::prelude::*;
use yewtil::NeqAssign;

pub struct Preview {
    props: Props,
    state: State,
    link: ComponentLink<Preview>,
    worker: Box<dyn Bridge<Worker>>,
}

pub enum Msg {
    DeleteClicked,
    AddTagClicked,
    AcceptTagClicked,
    CancelTagClicked,
    UpdateTagText(String),

    WorkerRes(worker::Response),
}

#[derive(Default)]
struct State {
    tag_input_visible: bool,
    tag_text: String,
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
        let state = State::default();

        Preview {
            props,
            state,
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
            Msg::AddTagClicked => {
                self.state.tag_input_visible = true;
                true
            }
            Msg::AcceptTagClicked => {
                let tag = self.state.tag_text.clone();
                self.state.tag_text.clear();
                self.state.tag_input_visible = false;
                let image = self.props.image.clone();
                // FIXME image.tags.push(tag
                self.worker
                    .send(worker::Request::UpdateImage(self.props.image));
                true
            }
            Msg::CancelTagClicked => {
                self.state.tag_input_visible = false;
                true
            }
            Msg::UpdateTagText(tag) => {
                self.state.tag_text = tag;
                true
            }
            Msg::WorkerRes(res) => match res {
                _ => false,
            },
        }
    }

    fn view(&self) -> Html {
        let src = format!("/api/images/{}/preview.jpg", self.props.image.id);

        let tag = {
            if self.state.tag_input_visible {
                html! {
                <>
                    <input
                        type="text"
                        oninput=self.link.callback(|e: InputData| Msg::UpdateTagText(e.value))
                    />
                    <button onclick=self.link.callback(|_| Msg::AcceptTagClicked)>{ "Ok" }</button>
                    <button onclick=self.link.callback(|_| Msg::CancelTagClicked)>{ "Cancel" }</button>
                </>
                }
            } else {
                html! {
                <button onclick=self.link.callback(|_| Msg::AddTagClicked)>{ "Tag" }</button>
                }
            }
        };

        html! {
        <div class="album-preview">
            <div class="album-toolbar">
                <button onclick=self.link.callback(|_| Msg::DeleteClicked)>{ "Delete" }</button>
                { tag }
            </div>
            <img src=src />
        </div>
        }
    }
}
