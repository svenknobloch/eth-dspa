use std::collections::{HashSet};
use std::fmt;

use serde_derive::{Deserialize, Serialize};

pub mod operators;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ActivePostEvent {
    Comment { post_id: i32, person_id: i32 },
    Like { post_id: i32, person_id: i32 },
}

impl ActivePostEvent {
    pub fn id(&self) -> i32 {
        match self {
            ActivePostEvent::Comment { post_id, .. } => *post_id,
            ActivePostEvent::Like { post_id, .. } => *post_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivePost {
    id: i32,
    users: HashSet<i32>,
    comments: u64,
    likes: u64,
}

impl fmt::Display for ActivePost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Active Post {{ id: {:8}, users: {:8}, comments: {:8}, likes: {:8} }}",
            self.id,
            self.users.len(),
            self.comments,
            self.likes
        )
    }
}

impl ActivePost {
    pub fn new(id: i32) -> Self {
        ActivePost {
            id,
            users: Default::default(),
            comments: Default::default(),
            likes: Default::default(),
        }
    }

    pub fn update(&mut self, event: ActivePostEvent) {
        match event {
            ActivePostEvent::Comment { person_id, .. } => {
                self.comments += 1;
                self.users.insert(person_id);
            }
            ActivePostEvent::Like { person_id, .. } => {
                self.likes += 1;
                self.users.insert(person_id);
            }
            _ => {}
        }
    }
}

