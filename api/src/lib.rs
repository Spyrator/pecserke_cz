use serde::Deserialize;
use spin_sdk::http::conversions::TryFromBody;
use spin_sdk::http::{IntoResponse, Json, Params, Request, Response, Router};
use spin_sdk::http_component;
use spin_sdk::sqlite::{Connection, Value};

#[http_component]
fn handle_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut r = Router::default();
    r.post("/api/users", register);
    // r.get("/api/users", read_users);
    // r.delete("/api/users/:id", delete_account);
    Ok(r.handle(req))
}

#[derive(Deserialize)]
struct User {
    name: String,
    password: String,
}

fn register(req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let body = req.into_body();
    let user: User = Json::try_from_body(body)?.0;

    let connection = Connection::open_default()?;

    let res = connection.execute(
        "INSERT INTO users (name, password) VALUES (?,?)",
        &[Value::Text(user.name), Value::Text(user.password)],
    )?;

    Ok(Response::builder()
        .status(200)
        .header("HX-Trigger", "register")
        .body(format!("{:?}", res))
        .build())
}
