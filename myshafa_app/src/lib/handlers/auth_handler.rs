use std::convert::Infallible;

pub async fn signin() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html("This is the signin page!".to_owned()))
}

pub async fn signup() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html("This is the signup page!".to_owned()))
}
