#![allow(dead_code)]
use uuid::Uuid;

pub struct Sphere {
    id: Uuid,
}

impl Sphere {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }

    pub fn id(self) -> Uuid {
        self.id
    }
}
