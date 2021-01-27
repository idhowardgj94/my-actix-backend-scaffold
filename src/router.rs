use actix_web::HttpResponse;

pub async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().body(json::object! {
        "status" => "not found",
        "msg" => "please check the api doc and try again",
    }.dump())
}