use tuono_lib::Request;
use tuono_lib::axum::http::StatusCode;

#[tuono_lib::api(GET)]
async fn health_check(_req: Request) -> StatusCode {
    StatusCode::OK
}
