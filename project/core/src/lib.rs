extern crate hyper;
extern crate futures_util;

pub type GenericError = Box<dyn std::error::Error + Send + Sync>;

pub mod static_data;
pub mod api;
