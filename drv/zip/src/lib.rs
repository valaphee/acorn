// Copyright 2024 Kevin Ludwig
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

#![no_std]

use acorn_api::{Error, FileDevice, Result};
use zerocopy::little_endian::{U16, U32, U64};

pub struct FileSystem {}

impl FileSystem {}

impl FileDevice for FileSystem {
    fn stat(&self, index: u64) -> Result<()> {
        Err(Error::Unimplemented)
    }

    fn ctrl(&self, index: u64) -> Result<()> {
        Err(Error::Unimplemented)
    }

    fn read(&self, index: u64, offset: u64, buffer: &mut [u8]) -> Result<()> {
        Err(Error::Unimplemented)
    }

    fn write(&self, index: u64, offset: u64, buffer: &[u8]) -> Result<()> {
        Err(Error::Unimplemented)
    }
}

/// See §4.3.7
struct LocalFileHeader {
    signature: [u8; 4],
    version: U16,
    flags: U16,
    compression_method: U16,
    last_mod_file_time: U16,
    last_mod_file_date: U16,
    checksum: U32,
    compressed_size: U32,
    uncompressed_size: U32,
    file_name_length: U16,
    extra_field_length: U16,
}

/// See §4.3.12
struct CentralDirectoryFileHeader {
    signature: [u8; 4],
    version_used: U16,
    version: U16,
    flags: U16,
    compression_method: U16,
    last_mod_file_time: U16,
    last_mod_file_date: U16,
    checksum: U32,
    compressed_size: U32,
    uncompressed_size: U32,
    file_name_length: U16,
    extra_field_length: U16,
    file_comment_length: U16,
    disk_number_start: U16,
    internal_file_attributes: U16,
    external_file_attributes: U32,
    local_header_offset: U32,
}

/// See §4.3.14
struct EndOfCentralDirectoryRecord64 {
    signature: [u8; 4],
    size: U64,
    version_used: U16,
    version: U16,
    disk_current: U32,
    disk: U32,
    entries: U64,
    entries_total: U64,
    size2: U64,
    offset: U64,
}

/// See §4.3.15
struct EndOfCentralDirectoryLocator64 {
    signature: [u8; 4],
    disk: U32,
    offset: U64,
    disk_count: U32,
}

/// See §4.3.16
struct EndOfCentralDirectoryRecord {
    signature: [u8; 4],
    disk_current: U16,
    disk: U16,
    entries: U16,
    entries_total: U16,
    size: U32,
    offset: U32,
    comment_length: U16,
}
