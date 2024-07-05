use crate::db_context::Database;
use std::sync::{Arc, Mutex};

pub mod body;
pub mod controller;
pub mod db_context;
pub mod entity;
pub mod schedule;
pub mod repository;
pub mod service;

pub struct AppState<'a> {
    pub connections: Mutex<u32>,
    pub context: Arc<Database<'a>>,
}