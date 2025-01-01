// Copyright 2025 Kevin Ludwig
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use core::{fmt, str};

use zerocopy::{FromBytes, Immutable, KnownLayout};

use crate::Result;

pub trait FileDevice {
    fn ctrl(&self, index: u64, buffer: &DirEntry) -> Result<()>;

    fn stat(&self, index: u64, offset: u64, buffer: &mut DirEntry) -> Result<()>;

    fn read(&self, index: u64, offset: u64, buffer: &mut [u8]) -> Result<()>;

    fn write(&self, index: u64, offset: u64, buffer: &[u8]) -> Result<()>;
}

#[repr(C)]
#[derive(FromBytes, KnownLayout, Immutable)]
pub struct DirEntry {
    pub index: u64,
    pub created_at: u64,
    pub last_accessed: u64,
    pub last_modified: u64,
    pub data_length: u64,
    pub name_length: u8,
    pub name: [u8],
}

impl DirEntry {
    pub fn name(&self) -> &str {
        str::from_utf8(&self.name[..self.name_length as usize]).unwrap()
    }
}

impl fmt::Debug for DirEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DirEntry")
            .field("index", &self.index)
            .field("created_at", &self.created_at)
            .field("last_accessed", &self.last_accessed)
            .field("last_modified", &self.last_modified)
            .field("data_length", &self.data_length)
            .field("name", &self.name())
            .finish()
    }
}
