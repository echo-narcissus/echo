//////////////////////////////////////
/// THIS WHOLE THING IS TEMPORARY! ///
//////////////////////////////////////
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub(crate) type SharedMessageStore = Arc<Mutex<MessageStore>>;

#[derive(Debug)]
pub(crate) struct StoredMessage {
    data: Vec<u8>,
    expires_at: Option<Instant>,
}

#[derive(Debug)]
pub(crate) struct MessageStore {
    messages: HashMap<Vec<u8>, StoredMessage>,
}


impl MessageStore {
    pub(crate) fn new() -> Self {
        Self {
            messages: HashMap::new(),
        }
    }

    pub(crate) fn insert(&mut self, id: Vec<u8>, data: Vec<u8>, ttl_seconds: Option<u64>) {
        let expires_at = ttl_seconds.map(|ttl| Instant::now() + Duration::from_secs(ttl));
        let message = StoredMessage { data, expires_at };
        self.messages.insert(id, message);
    }

    pub(crate) fn get(&mut self, id: &[u8]) -> Option<Vec<u8>> {
        if let Some(message) = self.messages.get(id) {
            if let Some(expires_at) = message.expires_at {
                if Instant::now() > expires_at {
                    self.messages.remove(id);
                    return None;
                }
            }
            return Some(message.data.clone());
        }
        None
    }
}
