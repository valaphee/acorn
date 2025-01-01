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

use std::{fs, os::unix::fs::FileExt, path::Path};

use acorn_api::{BlockDevice, DirEntry, Error, FileDevice, Result};

pub struct File(fs::File);

impl File {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self(fs::File::open(path).unwrap())
    }
}

impl BlockDevice for File {
    fn read(&self, offset: u64, buffer: &mut [u8]) -> Result<()> {
        self.0.read_exact_at(buffer, offset).unwrap();
        Ok(())
    }

    fn write(&self, offset: u64, buffer: &[u8]) -> Result<()> {
        self.0.write_all_at(buffer, offset).unwrap();
        Ok(())
    }
}

impl FileDevice for File {
    fn ctrl(&self, _index: u64, _buffer: &DirEntry) -> Result<()> {
        Err(Error::Unimplemented)
    }

    fn stat(&self, _index: u64, _offset: u64, buffer: &mut DirEntry) -> Result<()> {
        buffer.index = 0;
        let metadata = self.0.metadata().unwrap();
        /*buffer.created_at = metadata.created().unwrap();
        buffer.last_accessed = metadata.accessed().unwrap();
        buffer.last_modified = metadata.modified().unwrap();*/
        buffer.data_length = metadata.len();
        Ok(())
    }

    fn read(&self, _index: u64, offset: u64, buffer: &mut [u8]) -> Result<()> {
        let _ = self.0.read_exact_at(buffer, offset);
        Ok(())
    }

    fn write(&self, _index: u64, offset: u64, buffer: &[u8]) -> Result<()> {
        let _ = self.0.write_all_at(buffer, offset);
        Ok(())
    }
}
