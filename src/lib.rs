use worker::*;

#[event(fetch)]
async fn fetch(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    Response::ok(match req.path().as_str() {
        "/" => "Hello, World!".to_string(),
        _ => format!("Hello, {}", req.path().trim_start_matches('/')),
    })
}

