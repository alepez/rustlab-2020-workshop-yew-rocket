mod components;
mod worker;

use album_db::{Credentials, Images};
use components::Preview;
use std::rc::Rc;
use worker::Worker;
use yew::prelude::*;

pub struct App {
    state: State,
    link: ComponentLink<Self>,
    worker: Box<dyn Bridge<Worker>>,
}

pub enum Msg {
    Login(ChangeData),
    WorkerRes(worker::Response),
}

#[derive(Default)]
struct State {
    images: Option<Rc<Images>>,
    login_needed: bool,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut worker = Worker::bridge(link.callback(Msg::WorkerRes));
        worker.send(worker::Request::GetImages);

        App {
            state: State::default(),
            link,
            worker,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Login(e) => {
                log::debug!("login {:?}", e);
                if let ChangeData::Value(password) = e {
                    let username = "admin".to_string();
                    let credentials = Credentials { username, password };
                    self.worker.send(worker::Request::Login(credentials));
                    true
                } else {
                    false
                }
            }
            Msg::WorkerRes(res) => match res {
                worker::Response::ImagesLoaded(images) => {
                    self.state.images = Some(images);
                    true
                }
                worker::Response::Error(error) => {
                    if error == yew::services::fetch::StatusCode::UNAUTHORIZED {
                        log::info!("unauthorized");
                        self.state.login_needed = true;
                        true
                    } else {
                        log::info!("Error: {}", error);
                        false
                    }
                }
                worker::Response::LoginSuccess(_) => {
                    self.state.login_needed = false;
                    true
                }
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
        } else if self.state.login_needed {
            html! {
            <input type="password"
                onchange=self.link.callback(|e: ChangeData| Msg::Login(e))
                />
            }
        } else {
            html! {
            <div>{ "Loading..."}</div>
            }
        }
    }
}
