//! Application level configuration.
//!
//! Provides infrastructure for application level configuration. Configuration
//! values may be set and retrieved by type.

use std::any::{Any, TypeId};
use std::sync::Arc;
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};
use std::fmt;

type AnyMap = HashMap<TypeId, Box<dyn Any + Send + Sync>, BuildHasherDefault<IdHasher>>;

#[derive(Debug, Default)]
struct IdHasher(u64);

impl Hasher for IdHasher {
    fn write(&mut self, _: &[u8]) {
        unreachable!("TypeId calls write_u64");
    }

    #[inline]
    fn write_u64(&mut self, id: u64) {
        self.0 = id;
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
}

pub(crate) struct ConfigBuilder {
    inner: AnyMap,
}

impl ConfigBuilder {
    pub(crate) fn new() -> Self {
        Self { inner: AnyMap::default() }
    }

    pub(crate) fn insert<T: Send + Sync + 'static>(mut self, val: T) -> Self {
        self.inner.insert(TypeId::of::<T>(), Box::new(val));
        self
    }

    pub(crate) fn into_config(self) -> Config {
        Config { inner: Arc::new(self.inner) }
    }
}

impl fmt::Debug for ConfigBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConfigBuilder")
            .finish()
    }
}

/// A type of application level configuration.
#[derive(Clone)]
pub struct Config {
    inner: Arc<AnyMap>,
}

impl Config {
    /// Get the configuration value of the specified type.
    ///
    /// If a configuration value of type `T` is stored in `Config`, it is
    /// returned. Otherwise, `None` is returned.
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.inner
            .get(&TypeId::of::<T>())
            .and_then(|boxed| {
                (&**boxed as &dyn Any).downcast_ref()
            })
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .finish()
    }
}
