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

use acorn_api::{BlockDevice, Error, FileDevice, Result};
use zerocopy::little_endian::{U16, U32};

pub struct FileSystem<D: BlockDevice> {
    device: D,
}

impl<D: BlockDevice> FileSystem<D> {}

impl<D: BlockDevice> FileDevice for FileSystem<D> {
    fn ctrl(&self, _index: u64, _buffer: &acorn_api::DirEntry) -> Result<()> {
        Err(Error::Unimplemented)
    }

    fn stat(&self, _index: u64, _offset: u64, _buffer: &mut acorn_api::DirEntry) -> Result<()> {
        Err(Error::Unimplemented)
    }

    fn read(&self, _index: u64, _offset: u64, _buffer: &mut [u8]) -> Result<()> {
        Err(Error::Unimplemented)
    }

    fn write(&self, _index: u64, _offset: u64, _buffer: &[u8]) -> Result<()> {
        Err(Error::Unimplemented)
    }
}

/// See §3.2
#[repr(C)]
struct BootSector {
    bs_jmpboot: [u8; 3],
    bs_oemname: [u8; 8],

    bpb_bytspersec: U16,
    bpb_secperclus: u8,
    bpb_rsvdseccnt: U16,
    bpb_numfats: u8,
    bpb_rootentcnt: U16,
    bpb_totsec16: U16,
    bpb_media: u8,
    bpb_fatsz16: U16,

    bpb_secpertrk: U16,
    bpb_numheads: U16,
    bpb_hiddsec: U32,

    bpb_totsec32: U32,

    bpb_fatsz32: U32,
    bpb_extflags: U16,
    bpb_fsver: U16,
    bpb_rootclus: U32,
    bpb_fsinfo: U16,
    bpb_bkbootsec: U16,
    bpb_reserved: [u8; 12],

    bs_drvnum: u8,
    bs_reserved1: u8,
    bs_bootsig: u8,
    bs_volid: U32,
    bs_vollab: [u8; 11],
    bs_filsystype: [u8; 8],

    bs_boot: [u8; 420],
    signature_word: [u8; 2],
}

/// See §6
#[repr(C)]
struct DirEntry {
    dir_name: [u8; 11],
    dir_attr: u8,
    dir_ntres: u8,
    dir_crttimetenth: u8,
    dir_crttime: U16,
    dir_crtdate: U16,
    dir_lstaccdate: U16,
    dir_fstclushi: U16,
    dir_wrttime: U16,
    dir_wrtdate: U16,
    dir_fstcluslo: U16,
    dir_filesize: U32,
}

impl DirEntry {
    fn created_at(&self) -> Option<chrono::NaiveDateTime> {
        let time = self.dir_crttime.get();
        let date = self.dir_crtdate.get();
        Some(chrono::NaiveDateTime::new(
            chrono::NaiveDate::from_ymd_opt(
                i32::from(1980 + (date >> 9)),
                u32::from((date >> 5) & 0xF),
                u32::from(date & 0x1F),
            )?,
            chrono::NaiveTime::from_hms_milli_opt(
                u32::from(time >> 11),
                u32::from((time >> 5) & 0x1F),
                u32::from((time & 0x1F) * 2 + u16::from(self.dir_crttimetenth / 100)),
                u32::from(self.dir_crttimetenth % 100) * 10,
            )?,
        ))
    }

    fn last_accessed(&self) -> Option<chrono::NaiveDateTime> {
        let date = self.dir_lstaccdate.get();
        Some(chrono::NaiveDateTime::new(
            chrono::NaiveDate::from_ymd_opt(
                i32::from(1980 + (date >> 9)),
                u32::from((date >> 5) & 0xF),
                u32::from(date & 0x1F),
            )?,
            chrono::NaiveTime::default(),
        ))
    }

    fn last_modified(&self) -> Option<chrono::NaiveDateTime> {
        let time = self.dir_wrttime.get();
        let date = self.dir_wrtdate.get();
        Some(chrono::NaiveDateTime::new(
            chrono::NaiveDate::from_ymd_opt(
                i32::from(1980 + (date >> 9)),
                u32::from((date >> 5) & 0xF),
                u32::from(date & 0x1F),
            )?,
            chrono::NaiveTime::from_hms_opt(
                u32::from(time >> 11),
                u32::from((time >> 5) & 0x1F),
                u32::from((time & 0x1F) * 2),
            )?,
        ))
    }
}

/// See §7
#[repr(C)]
struct LongNameDirEntry {
    ldir_ord: u8,
    ldir_name1: [U16; 5],
    ldir_attr: u8,
    ldir_type: u8,
    ldir_chksum: u8,
    ldir_name2: [U16; 6],
    ldir_fstcluslo: U16,
    ldir_name3: [U16; 2],
}
