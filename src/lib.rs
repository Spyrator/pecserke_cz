use spin_sdk::http::{IntoResponse, Request, Router};
use spin_sdk::http_component;

mod api;
mod pages;

#[http_component]
fn handle_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut r = Router::default();

    //routes
    r.get("/", pages::index::get_index);
    r.get("/qr", pages::pages::get_qr);
    r.get("/settings", pages::pages::get_settings);
    r.get("/cv", pages::pages::get_cv);
    r.get("/pay/:amount", api::payme::pay_me_amount);
    r.post("/pay", api::payme::pay_me_json);

    //respond
    Ok(r.handle(req))
}
