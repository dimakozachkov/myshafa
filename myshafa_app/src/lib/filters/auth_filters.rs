use warp::Filter;

use crate::handlers::auth_handler;

pub fn auth() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("auth").and(signup().or(signin()))
}

/// POST /signup
pub fn signup() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("signup")
        .and(warp::post()).and_then(auth_handler::signup)
}

/// POST /signin
pub fn signin() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("signin").and(warp::post()).and_then(auth_handler::signin)
}



