use actix_web::web;

use crate::handlers::authentication::{create_user, login};

pub fn config_auth(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(create_user);
}
