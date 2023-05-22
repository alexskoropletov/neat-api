pub async fn check(uid: String) -> Result<impl warp::Reply, warp::Rejection> {
    println!("health check");
    println!("uid {:?}", uid);
    Ok(warp::reply::json(&"Ok"))
}
