use warp::Filter;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(serde::Serialize)]
struct OkResponse {
    response: String,
    timestamp: u128

}

#[tokio::main]
async fn main() {
    let ok = warp::path!("ok")
        .map(|| {
            warp::reply::json(&OkResponse {
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
                response: "ok".to_string()
            })
        });


    warp::serve(ok)
        .run(([127, 0, 0, 1], 3030))
        .await;
}