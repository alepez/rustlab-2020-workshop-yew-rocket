use album_db::{Image, ImageId, Images};
use std::collections::HashSet;
use std::rc::Rc;
use yew::format::{Json, Nothing, Text};
use yew::services::fetch;
use yew::services::fetch::FetchTask;
use yew::worker::*;

#[derive(Debug)]
pub enum Request {
    GetImages,
    DeleteImage(Image),
    UpdateImage(Image),
}

#[derive(Debug, Clone)]
pub enum Response {
    ImagesLoaded(Rc<Images>),
    Error(String),
}

pub enum Msg {
    Response(Response),
}

impl From<Response> for Msg {
    fn from(res: Response) -> Self {
        Msg::Response(res)
    }
}

pub struct Worker {
    link: AgentLink<Worker>,
    subscribers: HashSet<HandlerId>,
    fetch_task: Option<FetchTask>,
}

impl Agent for Worker {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        Worker {
            link,
            subscribers: HashSet::default(),
            fetch_task: None,
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Response(res) => {
                self.publish(res);
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _who: HandlerId) {
        request(self, msg)
    }
}

impl Worker {
    fn publish(&self, res: Response) {
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, res.clone());
        }
    }
}

#[cfg(feature = "mock_http")]
pub fn request(worker: &mut Worker, msg: Request) {
    let res = match msg {
        Request::GetImages => Response::ImagesLoaded(Rc::new(Images(vec![Image {
            id: ImageId(1),
            tags: Vec::default(),
        }]))),
        Request::DeleteImage(_image) => Response::ImagesLoaded(Rc::new(Images::default())),
        Request::UpdateImage(image) => Response::ImagesLoaded(Rc::new(Images(vec![image]))),
    };

    worker.link.send_message(res);
}

#[cfg(not(feature = "mock_http"))]
pub fn request(worker: &mut Worker, msg: Request) {
    let task = match msg {
        Request::GetImages => {
            let req = get("/api/images");
            task(worker, req, Response::ImagesLoaded)
        }
        Request::DeleteImage(image) => {
            let req = delete(format!("/api/images/{}", image.id).as_str());
            task(worker, req, Response::ImagesLoaded)
        }
        Request::UpdateImage(image) => {
            let req = put(format!("/api/images/{}", image.id).as_str(), json(image));
            task(worker, req, Response::ImagesLoaded)
        }
    };

    worker.fetch_task = task;
}

fn get(url: &str) -> fetch::Request<Nothing> {
    fetch::Request::get(url).body(Nothing).unwrap()
}

fn delete(url: &str) -> fetch::Request<Nothing> {
    fetch::Request::delete(url).body(Nothing).unwrap()
}

fn put<IN>(url: &str, body: IN) -> fetch::Request<IN>
where
    IN: Into<Text>,
{
    fetch::Request::put(url).body(body).unwrap()
}

fn json<T: serde::ser::Serialize>(data: T) -> Result<String, anyhow::Error> {
    serde_json::to_string(&data).map_err(|e| anyhow::anyhow!(e))
}


fn task<T, IN>(
    worker: &Worker,
    req: fetch::Request<IN>,
    ctor: impl Fn(Rc<T>) -> Response + 'static,
) -> Option<fetch::FetchTask>
where
    IN: Into<Text>,
    T: serde::de::DeserializeOwned + 'static,
{
    let link = worker.link.clone();

    let handler = move |response: fetch::Response<Json<Result<T, anyhow::Error>>>| {
        let (meta, Json(data)) = response.into_parts();

        if let Ok(data) = data {
            let res = ctor(Rc::new(data));
            link.send_message(res);
        } else {
            let res = Response::Error(meta.status.to_string());
            link.send_message(res);
        }
    };

    fetch::FetchService::fetch(req, handler.into()).ok()
}
