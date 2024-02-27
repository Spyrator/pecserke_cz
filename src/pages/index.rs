use askama::Template;
use spin_sdk::http::{IntoResponse, Params, Request, Response};

#[derive(Template)]
#[template(path = "index.html")]
pub struct WebsiteTemplate {}

pub fn get_index(_: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let res = Response::builder()
        .status(200)
        .body(WebsiteTemplate {}.to_string())
        .build();

    return Ok(res);
}
