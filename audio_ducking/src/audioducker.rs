use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use log::debug;
use crate::cpal::cpal_config::CpalConfiguration;

pub struct Ducker {
    device: Option<String>,

    stop: Arc<AtomicBool>

}

impl Ducker {
    pub fn new() -> Self {
        Self {
            device: None,
            stop: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&self) {
        debug!("Start Audio Ducker");
        //CpalConfiguration::get_device();
    }

    pub fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}