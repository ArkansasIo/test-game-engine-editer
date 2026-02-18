//! Engine resource and asset manager.

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetHandle(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssetKind {
    Texture,
    Material,
    Mesh,
    Sound,
    Effect,
    Script,
    Animation,
    Font,
}

#[derive(Debug, Clone)]
pub struct AssetRecord {
    pub handle: AssetHandle,
    pub name: String,
    pub path: String,
    pub kind: AssetKind,
    pub loaded: bool,
}

#[derive(Debug, Default, Clone)]
pub struct ResourceManager {
    next_handle: u64,
    assets: HashMap<AssetHandle, AssetRecord>,
}

impl ResourceManager {
    pub fn register(
        &mut self,
        name: impl Into<String>,
        path: impl Into<String>,
        kind: AssetKind,
    ) -> AssetHandle {
        let handle = AssetHandle(self.next_handle.max(1));
        self.next_handle = handle.0 + 1;
        self.assets.insert(
            handle,
            AssetRecord {
                handle,
                name: name.into(),
                path: path.into(),
                kind,
                loaded: false,
            },
        );
        handle
    }

    pub fn set_loaded(&mut self, handle: AssetHandle, loaded: bool) {
        if let Some(rec) = self.assets.get_mut(&handle) {
            rec.loaded = loaded;
        }
    }

    pub fn get(&self, handle: AssetHandle) -> Option<&AssetRecord> {
        self.assets.get(&handle)
    }

    pub fn iter(&self) -> impl Iterator<Item = &AssetRecord> {
        self.assets.values()
    }
}
