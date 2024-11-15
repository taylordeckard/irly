use serde::Serialize;
use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};

mod proto {
    tonic::include_proto!("irly.v1");
}

use proto::irly_client::IrlyClient;

async fn request_file(
    client: &mut IrlyClient<tonic::transport::channel::Channel>,
    path: &str,
) -> Result<tonic::Response<proto::GetFileResponse>, Box<dyn std::error::Error>> {
    let req = proto::GetFileRequest {
        file_path: path.to_string(),
    };

    let request = tonic::Request::new(req);

    let response = match client.get_file(request).await {
        Ok(response) => response,
        Err(e) => return Err(Box::new(e)),
    };

    Ok(response)
}

async fn web_handler(path: warp::path::FullPath) -> Result<impl warp::Reply, warp::Rejection> {
    let addr = "http://localhost:50051";
    let mut client = IrlyClient::connect(addr).await.unwrap();
    let Ok(response) = request_file(&mut client, path.as_str()).await else {
        return Err(warp::reject::not_found());
    };

    let res_ref = response.get_ref();

    Ok(warp::reply::with_header(
        res_ref.file_content.clone(),
        "content-type",
        mime_guess::from_path(&res_ref.file_path)
            .first_or_octet_stream()
            .to_string(),
    ))
}

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    let code;
    let message;
    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else {
        eprintln!("unhandled rejection: {err:?}");
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }
    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });
    Ok(warp::reply::with_status(json, code))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proxy_route = warp::path::full().and(warp::get()).and_then(web_handler);

    let routes = proxy_route.recover(handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;

    Ok(())
}
