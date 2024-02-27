use askama::Template;
use spin_sdk::http::{IntoResponse, Params, Request, Response};

#[derive(Template)]
#[template(path = "qr.html")]
pub struct QrTemplate {}

pub fn get_qr(_: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let res = Response::builder()
        .status(200)
        .body(QrTemplate {}.to_string())
        .build();

    return Ok(res);
}

#[derive(Template)]
#[template(path = "settings.html")]
pub struct SettingsTemplate {}

pub fn get_settings(_: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let res = Response::builder()
        .status(200)
        .body(SettingsTemplate {}.to_string())
        .build();

    return Ok(res);
}

#[derive(Template)]
#[template(path = "cv.html")]
pub struct CvTemplate {}

pub fn get_cv(_: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let res = Response::builder()
        .status(200)
        .body(CvTemplate {}.to_string())
        .build();

    return Ok(res);
}
