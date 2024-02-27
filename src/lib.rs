use spin_sdk::http::{IntoResponse, Params, Request, Response, Router};
use spin_sdk::http_component;

#[http_component]
fn handle_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut r = Router::default();
    r.get("/", register);
    Ok(r.handle(req))
}

fn register(_: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let res = "<p>hi :D</p>";
    let res = Response::builder()
        .status(200)
        .header("HX-Trigger", "register")
        .body(res)
        .build();

    return Ok(res);
}
