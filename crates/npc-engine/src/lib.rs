/*
 * niepce - engine/mod.rs
 *
 * Copyright (C) 2017-2022 Hubert Figuière
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

pub mod db;
pub mod library;

use std::ptr;

use db::NiepceProperties;

// must be a tuple for cxx
#[derive(Default)]
pub struct PropertySet(npc_fwk::PropertySet<db::NiepceProperties>);

impl PropertySet {
    fn add(&mut self, v: u32) {
        self.0.insert(NiepceProperties::from(v));
    }
}

impl std::ops::Deref for PropertySet {
    type Target = npc_fwk::PropertySet<db::NiepceProperties>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use npc_fwk::base::PropertyIndex;
use npc_fwk::toolkit::widgets::WrappedPropertyBag;
use npc_fwk::PropertyValue;

/// Delete the %WrappedPropertyBag object
///
/// # Safety
/// Dereference the raw pointer.
#[no_mangle]
pub unsafe extern "C" fn fwk_wrapped_property_bag_delete(bag: *mut WrappedPropertyBag) {
    drop(Box::from_raw(bag));
}

/// Clone the %WrappedPropertyBag object. Use this to take it out of the GValue.
///
/// # Safety
/// Dereference the raw pointer.
#[no_mangle]
pub unsafe extern "C" fn fwk_wrapped_property_bag_clone(
    bag: *const WrappedPropertyBag,
) -> *mut WrappedPropertyBag {
    Box::into_raw(Box::new((*bag).clone()))
}

/// # Safety
/// Dereference the raw pointer.
#[no_mangle]
pub unsafe extern "C" fn fwk_property_bag_len(bag: &WrappedPropertyBag) -> usize {
    (*bag).0.len()
}

/// # Safety
/// Dereference the raw pointer.
#[no_mangle]
pub unsafe extern "C" fn fwk_property_bag_key_by_index(
    bag: &WrappedPropertyBag,
    idx: usize,
) -> u32 {
    (*bag).0.bag[idx]
}

#[no_mangle]
pub extern "C" fn fwk_property_bag_value(
    b: &WrappedPropertyBag,
    key: PropertyIndex,
) -> *mut PropertyValue {
    if b.0.map.contains_key(&key) {
        let value = Box::new(b.0.map[&key].clone());
        Box::into_raw(value)
    } else {
        ptr::null_mut()
    }
}

// must be a tuple for cxx
#[derive(Default)]
pub struct PropertyBag(npc_fwk::PropertyBag<db::NiepceProperties>);

impl PropertyBag {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn key_by_index(&self, idx: usize) -> u32 {
        self.0.bag[idx].into()
    }

    fn contains_key(&self, key: &u32) -> bool {
        let key = db::NiepceProperties::from(*key);
        self.0.contains_key(&key)
    }

    fn value_unchecked(&self, key: u32) -> &PropertyValue {
        self.0
            .map
            .get(&db::NiepceProperties::from(key))
            .expect("no such value")
    }
}

impl std::ops::Deref for PropertyBag {
    type Target = npc_fwk::PropertyBag<db::NiepceProperties>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for PropertyBag {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn property_set_new() -> Box<PropertySet> {
    Box::new(PropertySet::default())
}

pub type NiepcePropertySet = PropertySet;
pub type NiepcePropertyBag = PropertyBag;

use crate::db::{Keyword, Label, LibFile, LibFolder, LibMetadata};
use crate::library::thumbnail_cache::{thumbnail_cache_new, ThumbnailCache};

#[cxx::bridge(namespace = "eng")]
mod ffi {
    #[namespace = "fwk"]
    extern "C++" {
        include!("fwk/cxx_prelude.hpp");
        include!("fwk/cxx_colour_bindings.hpp");

        type RgbColour = npc_fwk::base::rgbcolour::RgbColour;
        type PropertyValue = npc_fwk::PropertyValue;
    }

    #[repr(i32)]
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub enum FileType {
        /// Don't know
        Unknown = 0,
        /// Camera Raw
        Raw = 1,
        /// Bundle of RAW + processed. Don't assume JPEG.
        RawJpeg = 2,
        /// Processed Image
        Image = 3,
        /// Video
        Video = 4,
    }

    extern "Rust" {
        type Keyword;

        fn id(&self) -> i64;
        fn keyword(&self) -> &str;
    }

    impl Box<Keyword> {}

    extern "Rust" {
        type Label;

        fn colour(&self) -> &RgbColour;
        fn label(&self) -> &str;
        fn id(&self) -> i64;
        fn clone_boxed(&self) -> Box<Label>;
    }

    extern "Rust" {
        type LibFile;

        #[cxx_name = "path"]
        fn path_str(&self) -> String;
        fn id(&self) -> i64;
        fn folder_id(&self) -> i64;
        fn orientation(&self) -> i32;
        // The type is `FileType`.
        fn file_type(&self) -> FileType;
        #[cxx_name = "property"]
        fn property_int(&self, idx: u32) -> i32;
        #[cxx_name = "set_property"]
        fn set_property_int(&mut self, idx: u32, v: i32);
    }

    impl Box<LibFile> {}

    #[repr(i32)]
    #[derive(Clone)]
    pub enum FolderVirtualType {
        NONE = 0,
        TRASH = 1,
    }

    extern "Rust" {
        type LibFolder;

        fn id(&self) -> i64;
        fn name(&self) -> &str;
        fn expanded(&self) -> bool;
        fn virtual_type(&self) -> FolderVirtualType;
    }

    extern "Rust" {
        type LibMetadata;

        fn id(&self) -> i64;
        fn to_properties(&self, propset: &PropertySet) -> Box<PropertyBag>;
    }

    #[namespace = "fwk"]
    extern "Rust" {
        type PropertyBag;

        fn is_empty(&self) -> bool;
        fn len(&self) -> usize;
        fn contains_key(&self, key: &u32) -> bool;
        #[cxx_name = "value"]
        fn value_unchecked(&self, key: u32) -> &PropertyValue;
        fn key_by_index(&self, idx: usize) -> u32;
    }

    #[namespace = "fwk"]
    extern "Rust" {
        type PropertySet;

        #[cxx_name = "PropertySet_new"]
        fn property_set_new() -> Box<PropertySet>;
        fn add(&mut self, v: u32);
    }

    extern "C++" {
        type LcChannel;
    }

    extern "Rust" {
        type ThumbnailCache;

        #[cxx_name = "ThumbnailCache_new"]
        pub unsafe fn thumbnail_cache_new(
            dir: &str,
            channel: *const LcChannel,
        ) -> Box<ThumbnailCache>;
    }
}
