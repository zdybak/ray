#![allow(dead_code)]
use uuid::Uuid;

//We have to clone/copy sphere objects to store the same object in multiple intersections
#[derive(Debug, Clone, Copy)]
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

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
