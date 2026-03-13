use actix_web::web;

use crate::handlers::groups::get_groups;

pub fn config_groups(cfg: &mut web::ServiceConfig) {
    cfg.service(get_groups);
}
