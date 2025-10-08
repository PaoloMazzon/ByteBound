use std::sync::atomic::{AtomicUsize, Ordering};
use spdlog::prelude::*;
use anyhow::anyhow;
use std::fs;

static G_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Abstraction to work with files for any client
pub struct ClientWorkspace {
    /// Directory where this client's files are
    dir: String,
    
    /// Index of this client
    index: usize
}

impl ClientWorkspace {
    /// Creates a new client workspace, returning it. Might fail if the host does not have the necessary
    /// permissions
    pub fn new() -> Result<Self, anyhow::Error> {
        let index = G_COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir = format!("/app/clients/client_{}", index);
        std::fs::create_dir_all(&dir)?;
        Ok(ClientWorkspace { 
            dir,
            index
        })
    }

    /// Returns the docker volume mapping for this workspace (ie, /client/:/container/)
    pub fn docker_volume_flag(&self) -> String {
        format!("/share_folder/client_{}/:/client/", self.index)
    }

    /// Attempts to write a string to a given filename in the client's unique directory
    pub fn write_file(&self, name: &str, contents: &str) -> Result<(), anyhow::Error> {
        std::fs::write(format!("{}/{}", self.dir, name), contents)
            .map_err(|e| anyhow!("{:?}", e))
    }

    /// Attempts to read a file into a string from the given filename in the client's unique directory
    pub fn read_file(&self, name: &str) -> Result<String, anyhow::Error> {
        std::fs::read_to_string(format!("{}/{}", self.dir, name))
            .map_err(|e| anyhow!("{:?}", e))
    }

    /// Returns the complete file path for a given filename
    pub fn realpath(&self, name: &str) -> String {
        format!("{}/{}", self.dir, name)
    }
}

// Automatically delete the directory when the client workspace leaves scope
impl Drop for ClientWorkspace {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_dir_all(&self.dir) {
            error!("Failed to delete client workspace {}, {:?}", self.dir, e);
        }
    }
}