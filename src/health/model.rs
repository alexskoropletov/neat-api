use crate::common::stdout;

pub async fn check() -> Result<impl warp::Reply, warp::Rejection> {
    stdout::success("health check", ());
    Ok(warp::reply::json(&"Ok"))
}
