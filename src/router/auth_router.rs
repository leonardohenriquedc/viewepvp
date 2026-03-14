use actix_web::web;

use crate::handlers::authentication::{login, new_user};

pub fn config_auth(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(new_user);
}
