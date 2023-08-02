use std::fs::File;

use crate::{
    
    node::{UpdateError},
    nodes::node::{Node, InitError, ReadyError, ShutdownError},
};

pub struct ErrorDummyNode {
    name: String, 
}

impl ErrorDummyNode {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into()
        }
    }
}

impl Node for ErrorDummyNode {
    fn on_init(&self)-> Result<(), InitError> {
        let _file = File::open("").map_err(|err| InitError::Other(err.into()))?;

        Ok(())
    }

    fn on_ready(&self)-> Result<(), ReadyError> {
        Ok(())
    }

    fn on_shutdown(&self)-> Result<(), ShutdownError> {
        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn update(&self) -> Result<(), UpdateError> {
        Ok(())
    }
}
