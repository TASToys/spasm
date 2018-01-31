
use hyper;
use hyper::Client;
use hyper::{Request, Response};
use hyper_rustls::HttpsConnector;

use std::rc::Rc;

use tokio_core;

const TWITCH_API: &'static str = "https://api.twitch.tv/kraken";

#[derive(Debug)]
pub struct Twitch {
    pub _id:String,
    pub token:String,
    // pub client: Client<HttpsConnector>,
}

impl Twitch {
    pub fn Twitch(_id:&str, token:&str) -> Self {
        let mut tan = tokio_core::reactor::Core::new().unwrap();

        Twitch {
            _id: _id.to_owned(),
            token: token.to_owned(),
            // client: Client::new(&tan)
        }
    }
}