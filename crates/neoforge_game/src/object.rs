//! Base object registry for engine-level object lifecycle.

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectId(pub u64);

#[derive(Debug, Clone)]
pub struct ObjectMeta {
    pub name: String,
    pub class_name: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Default, Clone)]
pub struct ObjectRegistry {
    next_id: u64,
    objects: HashMap<ObjectId, ObjectMeta>,
}

impl ObjectRegistry {
    pub fn create(&mut self, name: impl Into<String>, class_name: impl Into<String>) -> ObjectId {
        let id = ObjectId(self.next_id.max(1));
        self.next_id = id.0 + 1;
        self.objects.insert(
            id,
            ObjectMeta {
                name: name.into(),
                class_name: class_name.into(),
                tags: Vec::new(),
            },
        );
        id
    }

    pub fn destroy(&mut self, id: ObjectId) -> Option<ObjectMeta> {
        self.objects.remove(&id)
    }

    pub fn get(&self, id: ObjectId) -> Option<&ObjectMeta> {
        self.objects.get(&id)
    }

    pub fn get_mut(&mut self, id: ObjectId) -> Option<&mut ObjectMeta> {
        self.objects.get_mut(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (ObjectId, &ObjectMeta)> {
        self.objects.iter().map(|(id, meta)| (*id, meta))
    }
}
