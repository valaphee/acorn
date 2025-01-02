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

#![no_std]

use core::fmt::Debug;

use acorn_api::storage::{BlockStorage, DirEntry, Error, FileStorage, Result};
use zerocopy::{
    Immutable, KnownLayout, TryFromBytes,
    little_endian::{U16, U32, U64},
};

pub struct FileSystem<S: BlockStorage> {
    storage: S,
}

impl<S: BlockStorage> FileSystem<S> {
    pub fn new(storage: S) -> Result<Self> {
        // search for [EndOfCentralDirectoryRecord] starting from the end
        let mut buffer = [0u8; 512];
        let mut offset = 0u64.next_multiple_of(512);
        let end_of_central_directory_record = loop {
            offset = offset.saturating_sub(512);
            storage.read(offset, &mut buffer)?;
            if let Some(end_of_central_directory_record) = buffer
                .windows(size_of::<EndOfCentralDirectoryRecord>())
                .rev()
                .find_map(|window| EndOfCentralDirectoryRecord::try_ref_from_bytes(window).ok())
            {
                break end_of_central_directory_record;
            }
            if offset == 0 {
                return Err(Error::Unimplemented);
            }
        };
        Ok(Self { storage })
    }
}

impl<S: BlockStorage> FileStorage for FileSystem<S> {
    fn ctrl(&self, _index: u64, _buffer: &DirEntry) -> Result<()> {
        Err(Error::Unimplemented)
    }

    fn stat(&self, _index: u64, _offset: u64, _buffer: &mut DirEntry) -> Result<()> {
        Err(Error::Unimplemented)
    }

    fn read(&self, _index: u64, _offset: u64, _buffer: &mut [u8]) -> Result<()> {
        Err(Error::Unimplemented)
    }

    fn write(&self, _index: u64, _offset: u64, _buffer: &[u8]) -> Result<()> {
        Err(Error::Unimplemented)
    }
}

/// See §4.3.7
#[repr(C)]
#[derive(Debug, TryFromBytes, KnownLayout, Immutable)]
struct LocalFileHeader {
    signature: LocalFileHeaderSignature,
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

#[repr(C)]
#[derive(Debug, TryFromBytes, KnownLayout, Immutable)]
struct LocalFileHeaderSignature(P, K, X03, X04);

/// See §4.3.12
#[repr(C)]
#[derive(Debug, TryFromBytes, KnownLayout, Immutable)]
struct CentralDirectoryHeader {
    signature: CentralDirectoryHeaderSignature,
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

#[repr(C)]
#[derive(Debug, TryFromBytes, KnownLayout, Immutable)]
struct CentralDirectoryHeaderSignature(P, K, X01, X02);

/// See §4.3.14
#[repr(C)]
#[derive(Debug, TryFromBytes, KnownLayout, Immutable)]
struct Zip64EndOfCentralDirectoryRecord {
    signature: Zip64EndOfCentralDirectoryRecordSignature,
    _size: U64,
    version_used: U16,
    version: U16,
    disk_current: U32,
    disk: U32,
    entries: U64,
    entries_total: U64,
    size: U64,
    offset: U64,
}

#[repr(C)]
#[derive(Debug, TryFromBytes, KnownLayout, Immutable)]
struct Zip64EndOfCentralDirectoryRecordSignature(P, K, X06, X06);

/// See §4.3.15
#[repr(C)]
#[derive(Debug, TryFromBytes, KnownLayout, Immutable)]
struct Zip64EndOfCentralDirectoryLocator {
    signature: Zip64EndOfCentralDirectoryLocatorSignature,
    disk: U32,
    offset: U64,
    disk_count: U32,
}

#[repr(C)]
#[derive(Debug, TryFromBytes, KnownLayout, Immutable)]
struct Zip64EndOfCentralDirectoryLocatorSignature(P, K, X06, X07);

/// See §4.3.16
#[repr(C)]
#[derive(Debug, TryFromBytes, KnownLayout, Immutable)]
struct EndOfCentralDirectoryRecord {
    signature: EndOfCentralDirectoryRecordSignature,
    disk_current: U16,
    disk: U16,
    entries: U16,
    entries_total: U16,
    size: U32,
    offset: U32,
    comment_length: U16,
}

#[repr(C)]
#[derive(Debug, TryFromBytes, KnownLayout, Immutable)]
struct EndOfCentralDirectoryRecordSignature(P, K, X05, X06);

#[repr(u8)]
#[derive(Debug, TryFromBytes, KnownLayout, Immutable)]
enum P {
    P = b'P',
}

#[repr(u8)]
#[derive(Debug, TryFromBytes, KnownLayout, Immutable)]
enum K {
    K = b'K',
}

#[repr(u8)]
#[derive(Debug, TryFromBytes, KnownLayout, Immutable)]
enum X01 {
    x01 = 0x01,
}

#[repr(u8)]
#[derive(Debug, TryFromBytes, KnownLayout, Immutable)]
enum X02 {
    x02 = 0x02,
}

#[repr(u8)]
#[derive(Debug, TryFromBytes, KnownLayout, Immutable)]
enum X03 {
    x03 = 0x03,
}

#[repr(u8)]
#[derive(Debug, TryFromBytes, KnownLayout, Immutable)]
enum X04 {
    x04 = 0x04,
}

#[repr(u8)]
#[derive(Debug, TryFromBytes, KnownLayout, Immutable)]
enum X05 {
    x05 = 0x05,
}

#[repr(u8)]
#[derive(Debug, TryFromBytes, KnownLayout, Immutable)]
enum X06 {
    x06 = 0x06,
}

#[repr(u8)]
#[derive(Debug, TryFromBytes, KnownLayout, Immutable)]
enum X07 {
    x07 = 0x07,
}
