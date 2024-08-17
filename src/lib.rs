#![allow(dead_code)]

use siphasher::sip::SipHasher;
use std::{hash::BuildHasherDefault, sync::Mutex};

static SEED: Mutex<Option<[u8; 16]>> = Mutex::new(None);

type OwnHasher = BuildHasherDefault<Hasher>;

struct Hasher {
    inner: SipHasher,
}

impl Default for Hasher {
    fn default() -> Self {
        let inner = if let Some(seed) = SEED.lock().unwrap().as_ref() {
            SipHasher::new_with_key(seed)
        } else {
            SipHasher::default()
        };

        Self { inner }
    }
}

impl std::hash::Hasher for Hasher {
    fn finish(&self) -> u64 {
        self.inner.finish()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.inner.write(bytes)
    }
}

pub type Move = [usize; 2];
pub mod board;
pub mod config;
pub mod program;
pub mod validator;
pub mod vector_util;
