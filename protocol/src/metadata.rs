// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct TopTracks {
    // message fields
    country: ::protobuf::SingularField<::std::string::String>,
    track: ::protobuf::RepeatedField<Track>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TopTracks {}

impl TopTracks {
    pub fn new() -> TopTracks {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TopTracks {
        static mut instance: ::protobuf::lazy::Lazy<TopTracks> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TopTracks,
        };
        unsafe {
            instance.get(TopTracks::new)
        }
    }

    // optional string country = 1;

    pub fn clear_country(&mut self) {
        self.country.clear();
    }

    pub fn has_country(&self) -> bool {
        self.country.is_some()
    }

    // Param is passed by value, moved
    pub fn set_country(&mut self, v: ::std::string::String) {
        self.country = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_country(&mut self) -> &mut ::std::string::String {
        if self.country.is_none() {
            self.country.set_default();
        }
        self.country.as_mut().unwrap()
    }

    // Take field
    pub fn take_country(&mut self) -> ::std::string::String {
        self.country.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_country(&self) -> &str {
        match self.country.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_country_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.country
    }

    fn mut_country_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.country
    }

    // repeated .Track track = 2;

    pub fn clear_track(&mut self) {
        self.track.clear();
    }

    // Param is passed by value, moved
    pub fn set_track(&mut self, v: ::protobuf::RepeatedField<Track>) {
        self.track = v;
    }

    // Mutable pointer to the field.
    pub fn mut_track(&mut self) -> &mut ::protobuf::RepeatedField<Track> {
        &mut self.track
    }

    // Take field
    pub fn take_track(&mut self) -> ::protobuf::RepeatedField<Track> {
        ::std::mem::replace(&mut self.track, ::protobuf::RepeatedField::new())
    }

    pub fn get_track(&self) -> &[Track] {
        &self.track
    }

    fn get_track_for_reflect(&self) -> &::protobuf::RepeatedField<Track> {
        &self.track
    }

    fn mut_track_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Track> {
        &mut self.track
    }
}

impl ::protobuf::Message for TopTracks {
    fn is_initialized(&self) -> bool {
        for v in &self.track {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.country)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.track)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.country.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.track {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.country.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.track {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TopTracks {
    fn new() -> TopTracks {
        TopTracks::new()
    }

    fn descriptor_static(_: ::std::option::Option<TopTracks>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "country",
                    TopTracks::get_country_for_reflect,
                    TopTracks::mut_country_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Track>>(
                    "track",
                    TopTracks::get_track_for_reflect,
                    TopTracks::mut_track_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TopTracks>(
                    "TopTracks",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TopTracks {
    fn clear(&mut self) {
        self.clear_country();
        self.clear_track();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TopTracks {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TopTracks {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ActivityPeriod {
    // message fields
    start_year: ::std::option::Option<i32>,
    end_year: ::std::option::Option<i32>,
    decade: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActivityPeriod {}

impl ActivityPeriod {
    pub fn new() -> ActivityPeriod {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActivityPeriod {
        static mut instance: ::protobuf::lazy::Lazy<ActivityPeriod> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActivityPeriod,
        };
        unsafe {
            instance.get(ActivityPeriod::new)
        }
    }

    // optional sint32 start_year = 1;

    pub fn clear_start_year(&mut self) {
        self.start_year = ::std::option::Option::None;
    }

    pub fn has_start_year(&self) -> bool {
        self.start_year.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_year(&mut self, v: i32) {
        self.start_year = ::std::option::Option::Some(v);
    }

    pub fn get_start_year(&self) -> i32 {
        self.start_year.unwrap_or(0)
    }

    fn get_start_year_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.start_year
    }

    fn mut_start_year_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.start_year
    }

    // optional sint32 end_year = 2;

    pub fn clear_end_year(&mut self) {
        self.end_year = ::std::option::Option::None;
    }

    pub fn has_end_year(&self) -> bool {
        self.end_year.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end_year(&mut self, v: i32) {
        self.end_year = ::std::option::Option::Some(v);
    }

    pub fn get_end_year(&self) -> i32 {
        self.end_year.unwrap_or(0)
    }

    fn get_end_year_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.end_year
    }

    fn mut_end_year_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.end_year
    }

    // optional sint32 decade = 3;

    pub fn clear_decade(&mut self) {
        self.decade = ::std::option::Option::None;
    }

    pub fn has_decade(&self) -> bool {
        self.decade.is_some()
    }

    // Param is passed by value, moved
    pub fn set_decade(&mut self, v: i32) {
        self.decade = ::std::option::Option::Some(v);
    }

    pub fn get_decade(&self) -> i32 {
        self.decade.unwrap_or(0)
    }

    fn get_decade_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.decade
    }

    fn mut_decade_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.decade
    }
}

impl ::protobuf::Message for ActivityPeriod {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.start_year = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.end_year = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.decade = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.start_year {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, v);
        }
        if let Some(v) = self.end_year {
            my_size += ::protobuf::rt::value_varint_zigzag_size(2, v);
        }
        if let Some(v) = self.decade {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start_year {
            os.write_sint32(1, v)?;
        }
        if let Some(v) = self.end_year {
            os.write_sint32(2, v)?;
        }
        if let Some(v) = self.decade {
            os.write_sint32(3, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ActivityPeriod {
    fn new() -> ActivityPeriod {
        ActivityPeriod::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActivityPeriod>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "start_year",
                    ActivityPeriod::get_start_year_for_reflect,
                    ActivityPeriod::mut_start_year_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "end_year",
                    ActivityPeriod::get_end_year_for_reflect,
                    ActivityPeriod::mut_end_year_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "decade",
                    ActivityPeriod::get_decade_for_reflect,
                    ActivityPeriod::mut_decade_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActivityPeriod>(
                    "ActivityPeriod",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActivityPeriod {
    fn clear(&mut self) {
        self.clear_start_year();
        self.clear_end_year();
        self.clear_decade();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActivityPeriod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActivityPeriod {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Artist {
    // message fields
    gid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    name: ::protobuf::SingularField<::std::string::String>,
    popularity: ::std::option::Option<i32>,
    top_track: ::protobuf::RepeatedField<TopTracks>,
    album_group: ::protobuf::RepeatedField<AlbumGroup>,
    single_group: ::protobuf::RepeatedField<AlbumGroup>,
    compilation_group: ::protobuf::RepeatedField<AlbumGroup>,
    appears_on_group: ::protobuf::RepeatedField<AlbumGroup>,
    genre: ::protobuf::RepeatedField<::std::string::String>,
    external_id: ::protobuf::RepeatedField<ExternalId>,
    portrait: ::protobuf::RepeatedField<Image>,
    biography: ::protobuf::RepeatedField<Biography>,
    activity_period: ::protobuf::RepeatedField<ActivityPeriod>,
    restriction: ::protobuf::RepeatedField<Restriction>,
    related: ::protobuf::RepeatedField<Artist>,
    is_portrait_album_cover: ::std::option::Option<bool>,
    portrait_group: ::protobuf::SingularPtrField<ImageGroup>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Artist {}

impl Artist {
    pub fn new() -> Artist {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Artist {
        static mut instance: ::protobuf::lazy::Lazy<Artist> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Artist,
        };
        unsafe {
            instance.get(Artist::new)
        }
    }

    // optional bytes gid = 1;

    pub fn clear_gid(&mut self) {
        self.gid.clear();
    }

    pub fn has_gid(&self) -> bool {
        self.gid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gid(&mut self, v: ::std::vec::Vec<u8>) {
        self.gid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gid(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.gid.is_none() {
            self.gid.set_default();
        }
        self.gid.as_mut().unwrap()
    }

    // Take field
    pub fn take_gid(&mut self) -> ::std::vec::Vec<u8> {
        self.gid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_gid(&self) -> &[u8] {
        match self.gid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_gid_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.gid
    }

    fn mut_gid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.gid
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional sint32 popularity = 3;

    pub fn clear_popularity(&mut self) {
        self.popularity = ::std::option::Option::None;
    }

    pub fn has_popularity(&self) -> bool {
        self.popularity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_popularity(&mut self, v: i32) {
        self.popularity = ::std::option::Option::Some(v);
    }

    pub fn get_popularity(&self) -> i32 {
        self.popularity.unwrap_or(0)
    }

    fn get_popularity_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.popularity
    }

    fn mut_popularity_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.popularity
    }

    // repeated .TopTracks top_track = 4;

    pub fn clear_top_track(&mut self) {
        self.top_track.clear();
    }

    // Param is passed by value, moved
    pub fn set_top_track(&mut self, v: ::protobuf::RepeatedField<TopTracks>) {
        self.top_track = v;
    }

    // Mutable pointer to the field.
    pub fn mut_top_track(&mut self) -> &mut ::protobuf::RepeatedField<TopTracks> {
        &mut self.top_track
    }

    // Take field
    pub fn take_top_track(&mut self) -> ::protobuf::RepeatedField<TopTracks> {
        ::std::mem::replace(&mut self.top_track, ::protobuf::RepeatedField::new())
    }

    pub fn get_top_track(&self) -> &[TopTracks] {
        &self.top_track
    }

    fn get_top_track_for_reflect(&self) -> &::protobuf::RepeatedField<TopTracks> {
        &self.top_track
    }

    fn mut_top_track_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TopTracks> {
        &mut self.top_track
    }

    // repeated .AlbumGroup album_group = 5;

    pub fn clear_album_group(&mut self) {
        self.album_group.clear();
    }

    // Param is passed by value, moved
    pub fn set_album_group(&mut self, v: ::protobuf::RepeatedField<AlbumGroup>) {
        self.album_group = v;
    }

    // Mutable pointer to the field.
    pub fn mut_album_group(&mut self) -> &mut ::protobuf::RepeatedField<AlbumGroup> {
        &mut self.album_group
    }

    // Take field
    pub fn take_album_group(&mut self) -> ::protobuf::RepeatedField<AlbumGroup> {
        ::std::mem::replace(&mut self.album_group, ::protobuf::RepeatedField::new())
    }

    pub fn get_album_group(&self) -> &[AlbumGroup] {
        &self.album_group
    }

    fn get_album_group_for_reflect(&self) -> &::protobuf::RepeatedField<AlbumGroup> {
        &self.album_group
    }

    fn mut_album_group_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<AlbumGroup> {
        &mut self.album_group
    }

    // repeated .AlbumGroup single_group = 6;

    pub fn clear_single_group(&mut self) {
        self.single_group.clear();
    }

    // Param is passed by value, moved
    pub fn set_single_group(&mut self, v: ::protobuf::RepeatedField<AlbumGroup>) {
        self.single_group = v;
    }

    // Mutable pointer to the field.
    pub fn mut_single_group(&mut self) -> &mut ::protobuf::RepeatedField<AlbumGroup> {
        &mut self.single_group
    }

    // Take field
    pub fn take_single_group(&mut self) -> ::protobuf::RepeatedField<AlbumGroup> {
        ::std::mem::replace(&mut self.single_group, ::protobuf::RepeatedField::new())
    }

    pub fn get_single_group(&self) -> &[AlbumGroup] {
        &self.single_group
    }

    fn get_single_group_for_reflect(&self) -> &::protobuf::RepeatedField<AlbumGroup> {
        &self.single_group
    }

    fn mut_single_group_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<AlbumGroup> {
        &mut self.single_group
    }

    // repeated .AlbumGroup compilation_group = 7;

    pub fn clear_compilation_group(&mut self) {
        self.compilation_group.clear();
    }

    // Param is passed by value, moved
    pub fn set_compilation_group(&mut self, v: ::protobuf::RepeatedField<AlbumGroup>) {
        self.compilation_group = v;
    }

    // Mutable pointer to the field.
    pub fn mut_compilation_group(&mut self) -> &mut ::protobuf::RepeatedField<AlbumGroup> {
        &mut self.compilation_group
    }

    // Take field
    pub fn take_compilation_group(&mut self) -> ::protobuf::RepeatedField<AlbumGroup> {
        ::std::mem::replace(&mut self.compilation_group, ::protobuf::RepeatedField::new())
    }

    pub fn get_compilation_group(&self) -> &[AlbumGroup] {
        &self.compilation_group
    }

    fn get_compilation_group_for_reflect(&self) -> &::protobuf::RepeatedField<AlbumGroup> {
        &self.compilation_group
    }

    fn mut_compilation_group_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<AlbumGroup> {
        &mut self.compilation_group
    }

    // repeated .AlbumGroup appears_on_group = 8;

    pub fn clear_appears_on_group(&mut self) {
        self.appears_on_group.clear();
    }

    // Param is passed by value, moved
    pub fn set_appears_on_group(&mut self, v: ::protobuf::RepeatedField<AlbumGroup>) {
        self.appears_on_group = v;
    }

    // Mutable pointer to the field.
    pub fn mut_appears_on_group(&mut self) -> &mut ::protobuf::RepeatedField<AlbumGroup> {
        &mut self.appears_on_group
    }

    // Take field
    pub fn take_appears_on_group(&mut self) -> ::protobuf::RepeatedField<AlbumGroup> {
        ::std::mem::replace(&mut self.appears_on_group, ::protobuf::RepeatedField::new())
    }

    pub fn get_appears_on_group(&self) -> &[AlbumGroup] {
        &self.appears_on_group
    }

    fn get_appears_on_group_for_reflect(&self) -> &::protobuf::RepeatedField<AlbumGroup> {
        &self.appears_on_group
    }

    fn mut_appears_on_group_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<AlbumGroup> {
        &mut self.appears_on_group
    }

    // repeated string genre = 9;

    pub fn clear_genre(&mut self) {
        self.genre.clear();
    }

    // Param is passed by value, moved
    pub fn set_genre(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.genre = v;
    }

    // Mutable pointer to the field.
    pub fn mut_genre(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.genre
    }

    // Take field
    pub fn take_genre(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.genre, ::protobuf::RepeatedField::new())
    }

    pub fn get_genre(&self) -> &[::std::string::String] {
        &self.genre
    }

    fn get_genre_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.genre
    }

    fn mut_genre_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.genre
    }

    // repeated .ExternalId external_id = 10;

    pub fn clear_external_id(&mut self) {
        self.external_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_external_id(&mut self, v: ::protobuf::RepeatedField<ExternalId>) {
        self.external_id = v;
    }

    // Mutable pointer to the field.
    pub fn mut_external_id(&mut self) -> &mut ::protobuf::RepeatedField<ExternalId> {
        &mut self.external_id
    }

    // Take field
    pub fn take_external_id(&mut self) -> ::protobuf::RepeatedField<ExternalId> {
        ::std::mem::replace(&mut self.external_id, ::protobuf::RepeatedField::new())
    }

    pub fn get_external_id(&self) -> &[ExternalId] {
        &self.external_id
    }

    fn get_external_id_for_reflect(&self) -> &::protobuf::RepeatedField<ExternalId> {
        &self.external_id
    }

    fn mut_external_id_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ExternalId> {
        &mut self.external_id
    }

    // repeated .Image portrait = 11;

    pub fn clear_portrait(&mut self) {
        self.portrait.clear();
    }

    // Param is passed by value, moved
    pub fn set_portrait(&mut self, v: ::protobuf::RepeatedField<Image>) {
        self.portrait = v;
    }

    // Mutable pointer to the field.
    pub fn mut_portrait(&mut self) -> &mut ::protobuf::RepeatedField<Image> {
        &mut self.portrait
    }

    // Take field
    pub fn take_portrait(&mut self) -> ::protobuf::RepeatedField<Image> {
        ::std::mem::replace(&mut self.portrait, ::protobuf::RepeatedField::new())
    }

    pub fn get_portrait(&self) -> &[Image] {
        &self.portrait
    }

    fn get_portrait_for_reflect(&self) -> &::protobuf::RepeatedField<Image> {
        &self.portrait
    }

    fn mut_portrait_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Image> {
        &mut self.portrait
    }

    // repeated .Biography biography = 12;

    pub fn clear_biography(&mut self) {
        self.biography.clear();
    }

    // Param is passed by value, moved
    pub fn set_biography(&mut self, v: ::protobuf::RepeatedField<Biography>) {
        self.biography = v;
    }

    // Mutable pointer to the field.
    pub fn mut_biography(&mut self) -> &mut ::protobuf::RepeatedField<Biography> {
        &mut self.biography
    }

    // Take field
    pub fn take_biography(&mut self) -> ::protobuf::RepeatedField<Biography> {
        ::std::mem::replace(&mut self.biography, ::protobuf::RepeatedField::new())
    }

    pub fn get_biography(&self) -> &[Biography] {
        &self.biography
    }

    fn get_biography_for_reflect(&self) -> &::protobuf::RepeatedField<Biography> {
        &self.biography
    }

    fn mut_biography_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Biography> {
        &mut self.biography
    }

    // repeated .ActivityPeriod activity_period = 13;

    pub fn clear_activity_period(&mut self) {
        self.activity_period.clear();
    }

    // Param is passed by value, moved
    pub fn set_activity_period(&mut self, v: ::protobuf::RepeatedField<ActivityPeriod>) {
        self.activity_period = v;
    }

    // Mutable pointer to the field.
    pub fn mut_activity_period(&mut self) -> &mut ::protobuf::RepeatedField<ActivityPeriod> {
        &mut self.activity_period
    }

    // Take field
    pub fn take_activity_period(&mut self) -> ::protobuf::RepeatedField<ActivityPeriod> {
        ::std::mem::replace(&mut self.activity_period, ::protobuf::RepeatedField::new())
    }

    pub fn get_activity_period(&self) -> &[ActivityPeriod] {
        &self.activity_period
    }

    fn get_activity_period_for_reflect(&self) -> &::protobuf::RepeatedField<ActivityPeriod> {
        &self.activity_period
    }

    fn mut_activity_period_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ActivityPeriod> {
        &mut self.activity_period
    }

    // repeated .Restriction restriction = 14;

    pub fn clear_restriction(&mut self) {
        self.restriction.clear();
    }

    // Param is passed by value, moved
    pub fn set_restriction(&mut self, v: ::protobuf::RepeatedField<Restriction>) {
        self.restriction = v;
    }

    // Mutable pointer to the field.
    pub fn mut_restriction(&mut self) -> &mut ::protobuf::RepeatedField<Restriction> {
        &mut self.restriction
    }

    // Take field
    pub fn take_restriction(&mut self) -> ::protobuf::RepeatedField<Restriction> {
        ::std::mem::replace(&mut self.restriction, ::protobuf::RepeatedField::new())
    }

    pub fn get_restriction(&self) -> &[Restriction] {
        &self.restriction
    }

    fn get_restriction_for_reflect(&self) -> &::protobuf::RepeatedField<Restriction> {
        &self.restriction
    }

    fn mut_restriction_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Restriction> {
        &mut self.restriction
    }

    // repeated .Artist related = 15;

    pub fn clear_related(&mut self) {
        self.related.clear();
    }

    // Param is passed by value, moved
    pub fn set_related(&mut self, v: ::protobuf::RepeatedField<Artist>) {
        self.related = v;
    }

    // Mutable pointer to the field.
    pub fn mut_related(&mut self) -> &mut ::protobuf::RepeatedField<Artist> {
        &mut self.related
    }

    // Take field
    pub fn take_related(&mut self) -> ::protobuf::RepeatedField<Artist> {
        ::std::mem::replace(&mut self.related, ::protobuf::RepeatedField::new())
    }

    pub fn get_related(&self) -> &[Artist] {
        &self.related
    }

    fn get_related_for_reflect(&self) -> &::protobuf::RepeatedField<Artist> {
        &self.related
    }

    fn mut_related_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Artist> {
        &mut self.related
    }

    // optional bool is_portrait_album_cover = 16;

    pub fn clear_is_portrait_album_cover(&mut self) {
        self.is_portrait_album_cover = ::std::option::Option::None;
    }

    pub fn has_is_portrait_album_cover(&self) -> bool {
        self.is_portrait_album_cover.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_portrait_album_cover(&mut self, v: bool) {
        self.is_portrait_album_cover = ::std::option::Option::Some(v);
    }

    pub fn get_is_portrait_album_cover(&self) -> bool {
        self.is_portrait_album_cover.unwrap_or(false)
    }

    fn get_is_portrait_album_cover_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_portrait_album_cover
    }

    fn mut_is_portrait_album_cover_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_portrait_album_cover
    }

    // optional .ImageGroup portrait_group = 17;

    pub fn clear_portrait_group(&mut self) {
        self.portrait_group.clear();
    }

    pub fn has_portrait_group(&self) -> bool {
        self.portrait_group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_portrait_group(&mut self, v: ImageGroup) {
        self.portrait_group = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_portrait_group(&mut self) -> &mut ImageGroup {
        if self.portrait_group.is_none() {
            self.portrait_group.set_default();
        }
        self.portrait_group.as_mut().unwrap()
    }

    // Take field
    pub fn take_portrait_group(&mut self) -> ImageGroup {
        self.portrait_group.take().unwrap_or_else(|| ImageGroup::new())
    }

    pub fn get_portrait_group(&self) -> &ImageGroup {
        self.portrait_group.as_ref().unwrap_or_else(|| ImageGroup::default_instance())
    }

    fn get_portrait_group_for_reflect(&self) -> &::protobuf::SingularPtrField<ImageGroup> {
        &self.portrait_group
    }

    fn mut_portrait_group_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ImageGroup> {
        &mut self.portrait_group
    }
}

impl ::protobuf::Message for Artist {
    fn is_initialized(&self) -> bool {
        for v in &self.top_track {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.album_group {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.single_group {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.compilation_group {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.appears_on_group {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.external_id {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.portrait {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.biography {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.activity_period {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.restriction {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.related {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.portrait_group {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.gid)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.popularity = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.top_track)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.album_group)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.single_group)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.compilation_group)?;
                },
                8 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.appears_on_group)?;
                },
                9 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.genre)?;
                },
                10 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.external_id)?;
                },
                11 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.portrait)?;
                },
                12 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.biography)?;
                },
                13 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.activity_period)?;
                },
                14 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.restriction)?;
                },
                15 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.related)?;
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_portrait_album_cover = ::std::option::Option::Some(tmp);
                },
                17 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.portrait_group)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.gid.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.popularity {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, v);
        }
        for value in &self.top_track {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.album_group {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.single_group {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.compilation_group {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.appears_on_group {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.genre {
            my_size += ::protobuf::rt::string_size(9, &value);
        };
        for value in &self.external_id {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.portrait {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.biography {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.activity_period {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.restriction {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.related {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.is_portrait_album_cover {
            my_size += 3;
        }
        if let Some(ref v) = self.portrait_group.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.gid.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.popularity {
            os.write_sint32(3, v)?;
        }
        for v in &self.top_track {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.album_group {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.single_group {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.compilation_group {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.appears_on_group {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.genre {
            os.write_string(9, &v)?;
        };
        for v in &self.external_id {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.portrait {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.biography {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.activity_period {
            os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.restriction {
            os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.related {
            os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.is_portrait_album_cover {
            os.write_bool(16, v)?;
        }
        if let Some(ref v) = self.portrait_group.as_ref() {
            os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Artist {
    fn new() -> Artist {
        Artist::new()
    }

    fn descriptor_static(_: ::std::option::Option<Artist>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "gid",
                    Artist::get_gid_for_reflect,
                    Artist::mut_gid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Artist::get_name_for_reflect,
                    Artist::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "popularity",
                    Artist::get_popularity_for_reflect,
                    Artist::mut_popularity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TopTracks>>(
                    "top_track",
                    Artist::get_top_track_for_reflect,
                    Artist::mut_top_track_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AlbumGroup>>(
                    "album_group",
                    Artist::get_album_group_for_reflect,
                    Artist::mut_album_group_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AlbumGroup>>(
                    "single_group",
                    Artist::get_single_group_for_reflect,
                    Artist::mut_single_group_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AlbumGroup>>(
                    "compilation_group",
                    Artist::get_compilation_group_for_reflect,
                    Artist::mut_compilation_group_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AlbumGroup>>(
                    "appears_on_group",
                    Artist::get_appears_on_group_for_reflect,
                    Artist::mut_appears_on_group_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "genre",
                    Artist::get_genre_for_reflect,
                    Artist::mut_genre_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ExternalId>>(
                    "external_id",
                    Artist::get_external_id_for_reflect,
                    Artist::mut_external_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Image>>(
                    "portrait",
                    Artist::get_portrait_for_reflect,
                    Artist::mut_portrait_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Biography>>(
                    "biography",
                    Artist::get_biography_for_reflect,
                    Artist::mut_biography_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ActivityPeriod>>(
                    "activity_period",
                    Artist::get_activity_period_for_reflect,
                    Artist::mut_activity_period_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Restriction>>(
                    "restriction",
                    Artist::get_restriction_for_reflect,
                    Artist::mut_restriction_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Artist>>(
                    "related",
                    Artist::get_related_for_reflect,
                    Artist::mut_related_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_portrait_album_cover",
                    Artist::get_is_portrait_album_cover_for_reflect,
                    Artist::mut_is_portrait_album_cover_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ImageGroup>>(
                    "portrait_group",
                    Artist::get_portrait_group_for_reflect,
                    Artist::mut_portrait_group_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Artist>(
                    "Artist",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Artist {
    fn clear(&mut self) {
        self.clear_gid();
        self.clear_name();
        self.clear_popularity();
        self.clear_top_track();
        self.clear_album_group();
        self.clear_single_group();
        self.clear_compilation_group();
        self.clear_appears_on_group();
        self.clear_genre();
        self.clear_external_id();
        self.clear_portrait();
        self.clear_biography();
        self.clear_activity_period();
        self.clear_restriction();
        self.clear_related();
        self.clear_is_portrait_album_cover();
        self.clear_portrait_group();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Artist {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Artist {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AlbumGroup {
    // message fields
    album: ::protobuf::RepeatedField<Album>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AlbumGroup {}

impl AlbumGroup {
    pub fn new() -> AlbumGroup {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AlbumGroup {
        static mut instance: ::protobuf::lazy::Lazy<AlbumGroup> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AlbumGroup,
        };
        unsafe {
            instance.get(AlbumGroup::new)
        }
    }

    // repeated .Album album = 1;

    pub fn clear_album(&mut self) {
        self.album.clear();
    }

    // Param is passed by value, moved
    pub fn set_album(&mut self, v: ::protobuf::RepeatedField<Album>) {
        self.album = v;
    }

    // Mutable pointer to the field.
    pub fn mut_album(&mut self) -> &mut ::protobuf::RepeatedField<Album> {
        &mut self.album
    }

    // Take field
    pub fn take_album(&mut self) -> ::protobuf::RepeatedField<Album> {
        ::std::mem::replace(&mut self.album, ::protobuf::RepeatedField::new())
    }

    pub fn get_album(&self) -> &[Album] {
        &self.album
    }

    fn get_album_for_reflect(&self) -> &::protobuf::RepeatedField<Album> {
        &self.album
    }

    fn mut_album_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Album> {
        &mut self.album
    }
}

impl ::protobuf::Message for AlbumGroup {
    fn is_initialized(&self) -> bool {
        for v in &self.album {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.album)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.album {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.album {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AlbumGroup {
    fn new() -> AlbumGroup {
        AlbumGroup::new()
    }

    fn descriptor_static(_: ::std::option::Option<AlbumGroup>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Album>>(
                    "album",
                    AlbumGroup::get_album_for_reflect,
                    AlbumGroup::mut_album_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AlbumGroup>(
                    "AlbumGroup",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AlbumGroup {
    fn clear(&mut self) {
        self.clear_album();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AlbumGroup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AlbumGroup {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Date {
    // message fields
    year: ::std::option::Option<i32>,
    month: ::std::option::Option<i32>,
    day: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Date {}

impl Date {
    pub fn new() -> Date {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Date {
        static mut instance: ::protobuf::lazy::Lazy<Date> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Date,
        };
        unsafe {
            instance.get(Date::new)
        }
    }

    // optional sint32 year = 1;

    pub fn clear_year(&mut self) {
        self.year = ::std::option::Option::None;
    }

    pub fn has_year(&self) -> bool {
        self.year.is_some()
    }

    // Param is passed by value, moved
    pub fn set_year(&mut self, v: i32) {
        self.year = ::std::option::Option::Some(v);
    }

    pub fn get_year(&self) -> i32 {
        self.year.unwrap_or(0)
    }

    fn get_year_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.year
    }

    fn mut_year_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.year
    }

    // optional sint32 month = 2;

    pub fn clear_month(&mut self) {
        self.month = ::std::option::Option::None;
    }

    pub fn has_month(&self) -> bool {
        self.month.is_some()
    }

    // Param is passed by value, moved
    pub fn set_month(&mut self, v: i32) {
        self.month = ::std::option::Option::Some(v);
    }

    pub fn get_month(&self) -> i32 {
        self.month.unwrap_or(0)
    }

    fn get_month_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.month
    }

    fn mut_month_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.month
    }

    // optional sint32 day = 3;

    pub fn clear_day(&mut self) {
        self.day = ::std::option::Option::None;
    }

    pub fn has_day(&self) -> bool {
        self.day.is_some()
    }

    // Param is passed by value, moved
    pub fn set_day(&mut self, v: i32) {
        self.day = ::std::option::Option::Some(v);
    }

    pub fn get_day(&self) -> i32 {
        self.day.unwrap_or(0)
    }

    fn get_day_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.day
    }

    fn mut_day_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.day
    }
}

impl ::protobuf::Message for Date {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.year = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.month = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.day = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.year {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, v);
        }
        if let Some(v) = self.month {
            my_size += ::protobuf::rt::value_varint_zigzag_size(2, v);
        }
        if let Some(v) = self.day {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.year {
            os.write_sint32(1, v)?;
        }
        if let Some(v) = self.month {
            os.write_sint32(2, v)?;
        }
        if let Some(v) = self.day {
            os.write_sint32(3, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Date {
    fn new() -> Date {
        Date::new()
    }

    fn descriptor_static(_: ::std::option::Option<Date>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "year",
                    Date::get_year_for_reflect,
                    Date::mut_year_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "month",
                    Date::get_month_for_reflect,
                    Date::mut_month_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "day",
                    Date::get_day_for_reflect,
                    Date::mut_day_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Date>(
                    "Date",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Date {
    fn clear(&mut self) {
        self.clear_year();
        self.clear_month();
        self.clear_day();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Date {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Date {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Album {
    // message fields
    gid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    name: ::protobuf::SingularField<::std::string::String>,
    artist: ::protobuf::RepeatedField<Artist>,
    typ: ::std::option::Option<Album_Type>,
    label: ::protobuf::SingularField<::std::string::String>,
    date: ::protobuf::SingularPtrField<Date>,
    popularity: ::std::option::Option<i32>,
    genre: ::protobuf::RepeatedField<::std::string::String>,
    cover: ::protobuf::RepeatedField<Image>,
    external_id: ::protobuf::RepeatedField<ExternalId>,
    disc: ::protobuf::RepeatedField<Disc>,
    review: ::protobuf::RepeatedField<::std::string::String>,
    copyright: ::protobuf::RepeatedField<Copyright>,
    restriction: ::protobuf::RepeatedField<Restriction>,
    related: ::protobuf::RepeatedField<Album>,
    sale_period: ::protobuf::RepeatedField<SalePeriod>,
    cover_group: ::protobuf::SingularPtrField<ImageGroup>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Album {}

impl Album {
    pub fn new() -> Album {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Album {
        static mut instance: ::protobuf::lazy::Lazy<Album> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Album,
        };
        unsafe {
            instance.get(Album::new)
        }
    }

    // optional bytes gid = 1;

    pub fn clear_gid(&mut self) {
        self.gid.clear();
    }

    pub fn has_gid(&self) -> bool {
        self.gid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gid(&mut self, v: ::std::vec::Vec<u8>) {
        self.gid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gid(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.gid.is_none() {
            self.gid.set_default();
        }
        self.gid.as_mut().unwrap()
    }

    // Take field
    pub fn take_gid(&mut self) -> ::std::vec::Vec<u8> {
        self.gid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_gid(&self) -> &[u8] {
        match self.gid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_gid_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.gid
    }

    fn mut_gid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.gid
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // repeated .Artist artist = 3;

    pub fn clear_artist(&mut self) {
        self.artist.clear();
    }

    // Param is passed by value, moved
    pub fn set_artist(&mut self, v: ::protobuf::RepeatedField<Artist>) {
        self.artist = v;
    }

    // Mutable pointer to the field.
    pub fn mut_artist(&mut self) -> &mut ::protobuf::RepeatedField<Artist> {
        &mut self.artist
    }

    // Take field
    pub fn take_artist(&mut self) -> ::protobuf::RepeatedField<Artist> {
        ::std::mem::replace(&mut self.artist, ::protobuf::RepeatedField::new())
    }

    pub fn get_artist(&self) -> &[Artist] {
        &self.artist
    }

    fn get_artist_for_reflect(&self) -> &::protobuf::RepeatedField<Artist> {
        &self.artist
    }

    fn mut_artist_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Artist> {
        &mut self.artist
    }

    // optional .Album.Type typ = 4;

    pub fn clear_typ(&mut self) {
        self.typ = ::std::option::Option::None;
    }

    pub fn has_typ(&self) -> bool {
        self.typ.is_some()
    }

    // Param is passed by value, moved
    pub fn set_typ(&mut self, v: Album_Type) {
        self.typ = ::std::option::Option::Some(v);
    }

    pub fn get_typ(&self) -> Album_Type {
        self.typ.unwrap_or(Album_Type::ALBUM)
    }

    fn get_typ_for_reflect(&self) -> &::std::option::Option<Album_Type> {
        &self.typ
    }

    fn mut_typ_for_reflect(&mut self) -> &mut ::std::option::Option<Album_Type> {
        &mut self.typ
    }

    // optional string label = 5;

    pub fn clear_label(&mut self) {
        self.label.clear();
    }

    pub fn has_label(&self) -> bool {
        self.label.is_some()
    }

    // Param is passed by value, moved
    pub fn set_label(&mut self, v: ::std::string::String) {
        self.label = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_label(&mut self) -> &mut ::std::string::String {
        if self.label.is_none() {
            self.label.set_default();
        }
        self.label.as_mut().unwrap()
    }

    // Take field
    pub fn take_label(&mut self) -> ::std::string::String {
        self.label.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_label(&self) -> &str {
        match self.label.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_label_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.label
    }

    fn mut_label_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.label
    }

    // optional .Date date = 6;

    pub fn clear_date(&mut self) {
        self.date.clear();
    }

    pub fn has_date(&self) -> bool {
        self.date.is_some()
    }

    // Param is passed by value, moved
    pub fn set_date(&mut self, v: Date) {
        self.date = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_date(&mut self) -> &mut Date {
        if self.date.is_none() {
            self.date.set_default();
        }
        self.date.as_mut().unwrap()
    }

    // Take field
    pub fn take_date(&mut self) -> Date {
        self.date.take().unwrap_or_else(|| Date::new())
    }

    pub fn get_date(&self) -> &Date {
        self.date.as_ref().unwrap_or_else(|| Date::default_instance())
    }

    fn get_date_for_reflect(&self) -> &::protobuf::SingularPtrField<Date> {
        &self.date
    }

    fn mut_date_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Date> {
        &mut self.date
    }

    // optional sint32 popularity = 7;

    pub fn clear_popularity(&mut self) {
        self.popularity = ::std::option::Option::None;
    }

    pub fn has_popularity(&self) -> bool {
        self.popularity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_popularity(&mut self, v: i32) {
        self.popularity = ::std::option::Option::Some(v);
    }

    pub fn get_popularity(&self) -> i32 {
        self.popularity.unwrap_or(0)
    }

    fn get_popularity_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.popularity
    }

    fn mut_popularity_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.popularity
    }

    // repeated string genre = 8;

    pub fn clear_genre(&mut self) {
        self.genre.clear();
    }

    // Param is passed by value, moved
    pub fn set_genre(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.genre = v;
    }

    // Mutable pointer to the field.
    pub fn mut_genre(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.genre
    }

    // Take field
    pub fn take_genre(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.genre, ::protobuf::RepeatedField::new())
    }

    pub fn get_genre(&self) -> &[::std::string::String] {
        &self.genre
    }

    fn get_genre_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.genre
    }

    fn mut_genre_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.genre
    }

    // repeated .Image cover = 9;

    pub fn clear_cover(&mut self) {
        self.cover.clear();
    }

    // Param is passed by value, moved
    pub fn set_cover(&mut self, v: ::protobuf::RepeatedField<Image>) {
        self.cover = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cover(&mut self) -> &mut ::protobuf::RepeatedField<Image> {
        &mut self.cover
    }

    // Take field
    pub fn take_cover(&mut self) -> ::protobuf::RepeatedField<Image> {
        ::std::mem::replace(&mut self.cover, ::protobuf::RepeatedField::new())
    }

    pub fn get_cover(&self) -> &[Image] {
        &self.cover
    }

    fn get_cover_for_reflect(&self) -> &::protobuf::RepeatedField<Image> {
        &self.cover
    }

    fn mut_cover_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Image> {
        &mut self.cover
    }

    // repeated .ExternalId external_id = 10;

    pub fn clear_external_id(&mut self) {
        self.external_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_external_id(&mut self, v: ::protobuf::RepeatedField<ExternalId>) {
        self.external_id = v;
    }

    // Mutable pointer to the field.
    pub fn mut_external_id(&mut self) -> &mut ::protobuf::RepeatedField<ExternalId> {
        &mut self.external_id
    }

    // Take field
    pub fn take_external_id(&mut self) -> ::protobuf::RepeatedField<ExternalId> {
        ::std::mem::replace(&mut self.external_id, ::protobuf::RepeatedField::new())
    }

    pub fn get_external_id(&self) -> &[ExternalId] {
        &self.external_id
    }

    fn get_external_id_for_reflect(&self) -> &::protobuf::RepeatedField<ExternalId> {
        &self.external_id
    }

    fn mut_external_id_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ExternalId> {
        &mut self.external_id
    }

    // repeated .Disc disc = 11;

    pub fn clear_disc(&mut self) {
        self.disc.clear();
    }

    // Param is passed by value, moved
    pub fn set_disc(&mut self, v: ::protobuf::RepeatedField<Disc>) {
        self.disc = v;
    }

    // Mutable pointer to the field.
    pub fn mut_disc(&mut self) -> &mut ::protobuf::RepeatedField<Disc> {
        &mut self.disc
    }

    // Take field
    pub fn take_disc(&mut self) -> ::protobuf::RepeatedField<Disc> {
        ::std::mem::replace(&mut self.disc, ::protobuf::RepeatedField::new())
    }

    pub fn get_disc(&self) -> &[Disc] {
        &self.disc
    }

    fn get_disc_for_reflect(&self) -> &::protobuf::RepeatedField<Disc> {
        &self.disc
    }

    fn mut_disc_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Disc> {
        &mut self.disc
    }

    // repeated string review = 12;

    pub fn clear_review(&mut self) {
        self.review.clear();
    }

    // Param is passed by value, moved
    pub fn set_review(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.review = v;
    }

    // Mutable pointer to the field.
    pub fn mut_review(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.review
    }

    // Take field
    pub fn take_review(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.review, ::protobuf::RepeatedField::new())
    }

    pub fn get_review(&self) -> &[::std::string::String] {
        &self.review
    }

    fn get_review_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.review
    }

    fn mut_review_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.review
    }

    // repeated .Copyright copyright = 13;

    pub fn clear_copyright(&mut self) {
        self.copyright.clear();
    }

    // Param is passed by value, moved
    pub fn set_copyright(&mut self, v: ::protobuf::RepeatedField<Copyright>) {
        self.copyright = v;
    }

    // Mutable pointer to the field.
    pub fn mut_copyright(&mut self) -> &mut ::protobuf::RepeatedField<Copyright> {
        &mut self.copyright
    }

    // Take field
    pub fn take_copyright(&mut self) -> ::protobuf::RepeatedField<Copyright> {
        ::std::mem::replace(&mut self.copyright, ::protobuf::RepeatedField::new())
    }

    pub fn get_copyright(&self) -> &[Copyright] {
        &self.copyright
    }

    fn get_copyright_for_reflect(&self) -> &::protobuf::RepeatedField<Copyright> {
        &self.copyright
    }

    fn mut_copyright_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Copyright> {
        &mut self.copyright
    }

    // repeated .Restriction restriction = 14;

    pub fn clear_restriction(&mut self) {
        self.restriction.clear();
    }

    // Param is passed by value, moved
    pub fn set_restriction(&mut self, v: ::protobuf::RepeatedField<Restriction>) {
        self.restriction = v;
    }

    // Mutable pointer to the field.
    pub fn mut_restriction(&mut self) -> &mut ::protobuf::RepeatedField<Restriction> {
        &mut self.restriction
    }

    // Take field
    pub fn take_restriction(&mut self) -> ::protobuf::RepeatedField<Restriction> {
        ::std::mem::replace(&mut self.restriction, ::protobuf::RepeatedField::new())
    }

    pub fn get_restriction(&self) -> &[Restriction] {
        &self.restriction
    }

    fn get_restriction_for_reflect(&self) -> &::protobuf::RepeatedField<Restriction> {
        &self.restriction
    }

    fn mut_restriction_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Restriction> {
        &mut self.restriction
    }

    // repeated .Album related = 15;

    pub fn clear_related(&mut self) {
        self.related.clear();
    }

    // Param is passed by value, moved
    pub fn set_related(&mut self, v: ::protobuf::RepeatedField<Album>) {
        self.related = v;
    }

    // Mutable pointer to the field.
    pub fn mut_related(&mut self) -> &mut ::protobuf::RepeatedField<Album> {
        &mut self.related
    }

    // Take field
    pub fn take_related(&mut self) -> ::protobuf::RepeatedField<Album> {
        ::std::mem::replace(&mut self.related, ::protobuf::RepeatedField::new())
    }

    pub fn get_related(&self) -> &[Album] {
        &self.related
    }

    fn get_related_for_reflect(&self) -> &::protobuf::RepeatedField<Album> {
        &self.related
    }

    fn mut_related_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Album> {
        &mut self.related
    }

    // repeated .SalePeriod sale_period = 16;

    pub fn clear_sale_period(&mut self) {
        self.sale_period.clear();
    }

    // Param is passed by value, moved
    pub fn set_sale_period(&mut self, v: ::protobuf::RepeatedField<SalePeriod>) {
        self.sale_period = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sale_period(&mut self) -> &mut ::protobuf::RepeatedField<SalePeriod> {
        &mut self.sale_period
    }

    // Take field
    pub fn take_sale_period(&mut self) -> ::protobuf::RepeatedField<SalePeriod> {
        ::std::mem::replace(&mut self.sale_period, ::protobuf::RepeatedField::new())
    }

    pub fn get_sale_period(&self) -> &[SalePeriod] {
        &self.sale_period
    }

    fn get_sale_period_for_reflect(&self) -> &::protobuf::RepeatedField<SalePeriod> {
        &self.sale_period
    }

    fn mut_sale_period_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<SalePeriod> {
        &mut self.sale_period
    }

    // optional .ImageGroup cover_group = 17;

    pub fn clear_cover_group(&mut self) {
        self.cover_group.clear();
    }

    pub fn has_cover_group(&self) -> bool {
        self.cover_group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cover_group(&mut self, v: ImageGroup) {
        self.cover_group = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cover_group(&mut self) -> &mut ImageGroup {
        if self.cover_group.is_none() {
            self.cover_group.set_default();
        }
        self.cover_group.as_mut().unwrap()
    }

    // Take field
    pub fn take_cover_group(&mut self) -> ImageGroup {
        self.cover_group.take().unwrap_or_else(|| ImageGroup::new())
    }

    pub fn get_cover_group(&self) -> &ImageGroup {
        self.cover_group.as_ref().unwrap_or_else(|| ImageGroup::default_instance())
    }

    fn get_cover_group_for_reflect(&self) -> &::protobuf::SingularPtrField<ImageGroup> {
        &self.cover_group
    }

    fn mut_cover_group_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ImageGroup> {
        &mut self.cover_group
    }
}

impl ::protobuf::Message for Album {
    fn is_initialized(&self) -> bool {
        for v in &self.artist {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.date {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.cover {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.external_id {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.disc {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.copyright {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.restriction {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.related {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.sale_period {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.cover_group {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.gid)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.artist)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.typ = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.label)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.date)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.popularity = ::std::option::Option::Some(tmp);
                },
                8 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.genre)?;
                },
                9 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.cover)?;
                },
                10 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.external_id)?;
                },
                11 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.disc)?;
                },
                12 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.review)?;
                },
                13 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.copyright)?;
                },
                14 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.restriction)?;
                },
                15 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.related)?;
                },
                16 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.sale_period)?;
                },
                17 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cover_group)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.gid.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        for value in &self.artist {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.typ {
            my_size += ::protobuf::rt::enum_size(4, v);
        }
        if let Some(ref v) = self.label.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(ref v) = self.date.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.popularity {
            my_size += ::protobuf::rt::value_varint_zigzag_size(7, v);
        }
        for value in &self.genre {
            my_size += ::protobuf::rt::string_size(8, &value);
        };
        for value in &self.cover {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.external_id {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.disc {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.review {
            my_size += ::protobuf::rt::string_size(12, &value);
        };
        for value in &self.copyright {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.restriction {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.related {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.sale_period {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.cover_group.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.gid.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        }
        for v in &self.artist {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.typ {
            os.write_enum(4, v.value())?;
        }
        if let Some(ref v) = self.label.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(ref v) = self.date.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.popularity {
            os.write_sint32(7, v)?;
        }
        for v in &self.genre {
            os.write_string(8, &v)?;
        };
        for v in &self.cover {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.external_id {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.disc {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.review {
            os.write_string(12, &v)?;
        };
        for v in &self.copyright {
            os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.restriction {
            os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.related {
            os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.sale_period {
            os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.cover_group.as_ref() {
            os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Album {
    fn new() -> Album {
        Album::new()
    }

    fn descriptor_static(_: ::std::option::Option<Album>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "gid",
                    Album::get_gid_for_reflect,
                    Album::mut_gid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Album::get_name_for_reflect,
                    Album::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Artist>>(
                    "artist",
                    Album::get_artist_for_reflect,
                    Album::mut_artist_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Album_Type>>(
                    "typ",
                    Album::get_typ_for_reflect,
                    Album::mut_typ_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "label",
                    Album::get_label_for_reflect,
                    Album::mut_label_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Date>>(
                    "date",
                    Album::get_date_for_reflect,
                    Album::mut_date_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "popularity",
                    Album::get_popularity_for_reflect,
                    Album::mut_popularity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "genre",
                    Album::get_genre_for_reflect,
                    Album::mut_genre_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Image>>(
                    "cover",
                    Album::get_cover_for_reflect,
                    Album::mut_cover_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ExternalId>>(
                    "external_id",
                    Album::get_external_id_for_reflect,
                    Album::mut_external_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Disc>>(
                    "disc",
                    Album::get_disc_for_reflect,
                    Album::mut_disc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "review",
                    Album::get_review_for_reflect,
                    Album::mut_review_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Copyright>>(
                    "copyright",
                    Album::get_copyright_for_reflect,
                    Album::mut_copyright_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Restriction>>(
                    "restriction",
                    Album::get_restriction_for_reflect,
                    Album::mut_restriction_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Album>>(
                    "related",
                    Album::get_related_for_reflect,
                    Album::mut_related_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SalePeriod>>(
                    "sale_period",
                    Album::get_sale_period_for_reflect,
                    Album::mut_sale_period_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ImageGroup>>(
                    "cover_group",
                    Album::get_cover_group_for_reflect,
                    Album::mut_cover_group_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Album>(
                    "Album",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Album {
    fn clear(&mut self) {
        self.clear_gid();
        self.clear_name();
        self.clear_artist();
        self.clear_typ();
        self.clear_label();
        self.clear_date();
        self.clear_popularity();
        self.clear_genre();
        self.clear_cover();
        self.clear_external_id();
        self.clear_disc();
        self.clear_review();
        self.clear_copyright();
        self.clear_restriction();
        self.clear_related();
        self.clear_sale_period();
        self.clear_cover_group();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Album {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Album {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Album_Type {
    ALBUM = 1,
    SINGLE = 2,
    COMPILATION = 3,
    EP = 4,
}

impl ::protobuf::ProtobufEnum for Album_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Album_Type> {
        match value {
            1 => ::std::option::Option::Some(Album_Type::ALBUM),
            2 => ::std::option::Option::Some(Album_Type::SINGLE),
            3 => ::std::option::Option::Some(Album_Type::COMPILATION),
            4 => ::std::option::Option::Some(Album_Type::EP),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Album_Type] = &[
            Album_Type::ALBUM,
            Album_Type::SINGLE,
            Album_Type::COMPILATION,
            Album_Type::EP,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Album_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Album_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Album_Type {
}

impl ::protobuf::reflect::ProtobufValue for Album_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Track {
    // message fields
    gid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    name: ::protobuf::SingularField<::std::string::String>,
    album: ::protobuf::SingularPtrField<Album>,
    artist: ::protobuf::RepeatedField<Artist>,
    number: ::std::option::Option<i32>,
    disc_number: ::std::option::Option<i32>,
    duration: ::std::option::Option<i32>,
    popularity: ::std::option::Option<i32>,
    explicit: ::std::option::Option<bool>,
    external_id: ::protobuf::RepeatedField<ExternalId>,
    restriction: ::protobuf::RepeatedField<Restriction>,
    file: ::protobuf::RepeatedField<AudioFile>,
    alternative: ::protobuf::RepeatedField<Track>,
    sale_period: ::protobuf::RepeatedField<SalePeriod>,
    preview: ::protobuf::RepeatedField<AudioFile>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Track {}

impl Track {
    pub fn new() -> Track {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Track {
        static mut instance: ::protobuf::lazy::Lazy<Track> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Track,
        };
        unsafe {
            instance.get(Track::new)
        }
    }

    // optional bytes gid = 1;

    pub fn clear_gid(&mut self) {
        self.gid.clear();
    }

    pub fn has_gid(&self) -> bool {
        self.gid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gid(&mut self, v: ::std::vec::Vec<u8>) {
        self.gid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gid(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.gid.is_none() {
            self.gid.set_default();
        }
        self.gid.as_mut().unwrap()
    }

    // Take field
    pub fn take_gid(&mut self) -> ::std::vec::Vec<u8> {
        self.gid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_gid(&self) -> &[u8] {
        match self.gid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_gid_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.gid
    }

    fn mut_gid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.gid
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional .Album album = 3;

    pub fn clear_album(&mut self) {
        self.album.clear();
    }

    pub fn has_album(&self) -> bool {
        self.album.is_some()
    }

    // Param is passed by value, moved
    pub fn set_album(&mut self, v: Album) {
        self.album = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_album(&mut self) -> &mut Album {
        if self.album.is_none() {
            self.album.set_default();
        }
        self.album.as_mut().unwrap()
    }

    // Take field
    pub fn take_album(&mut self) -> Album {
        self.album.take().unwrap_or_else(|| Album::new())
    }

    pub fn get_album(&self) -> &Album {
        self.album.as_ref().unwrap_or_else(|| Album::default_instance())
    }

    fn get_album_for_reflect(&self) -> &::protobuf::SingularPtrField<Album> {
        &self.album
    }

    fn mut_album_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Album> {
        &mut self.album
    }

    // repeated .Artist artist = 4;

    pub fn clear_artist(&mut self) {
        self.artist.clear();
    }

    // Param is passed by value, moved
    pub fn set_artist(&mut self, v: ::protobuf::RepeatedField<Artist>) {
        self.artist = v;
    }

    // Mutable pointer to the field.
    pub fn mut_artist(&mut self) -> &mut ::protobuf::RepeatedField<Artist> {
        &mut self.artist
    }

    // Take field
    pub fn take_artist(&mut self) -> ::protobuf::RepeatedField<Artist> {
        ::std::mem::replace(&mut self.artist, ::protobuf::RepeatedField::new())
    }

    pub fn get_artist(&self) -> &[Artist] {
        &self.artist
    }

    fn get_artist_for_reflect(&self) -> &::protobuf::RepeatedField<Artist> {
        &self.artist
    }

    fn mut_artist_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Artist> {
        &mut self.artist
    }

    // optional sint32 number = 5;

    pub fn clear_number(&mut self) {
        self.number = ::std::option::Option::None;
    }

    pub fn has_number(&self) -> bool {
        self.number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_number(&mut self, v: i32) {
        self.number = ::std::option::Option::Some(v);
    }

    pub fn get_number(&self) -> i32 {
        self.number.unwrap_or(0)
    }

    fn get_number_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.number
    }

    fn mut_number_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.number
    }

    // optional sint32 disc_number = 6;

    pub fn clear_disc_number(&mut self) {
        self.disc_number = ::std::option::Option::None;
    }

    pub fn has_disc_number(&self) -> bool {
        self.disc_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_disc_number(&mut self, v: i32) {
        self.disc_number = ::std::option::Option::Some(v);
    }

    pub fn get_disc_number(&self) -> i32 {
        self.disc_number.unwrap_or(0)
    }

    fn get_disc_number_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.disc_number
    }

    fn mut_disc_number_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.disc_number
    }

    // optional sint32 duration = 7;

    pub fn clear_duration(&mut self) {
        self.duration = ::std::option::Option::None;
    }

    pub fn has_duration(&self) -> bool {
        self.duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration(&mut self, v: i32) {
        self.duration = ::std::option::Option::Some(v);
    }

    pub fn get_duration(&self) -> i32 {
        self.duration.unwrap_or(0)
    }

    fn get_duration_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.duration
    }

    fn mut_duration_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.duration
    }

    // optional sint32 popularity = 8;

    pub fn clear_popularity(&mut self) {
        self.popularity = ::std::option::Option::None;
    }

    pub fn has_popularity(&self) -> bool {
        self.popularity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_popularity(&mut self, v: i32) {
        self.popularity = ::std::option::Option::Some(v);
    }

    pub fn get_popularity(&self) -> i32 {
        self.popularity.unwrap_or(0)
    }

    fn get_popularity_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.popularity
    }

    fn mut_popularity_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.popularity
    }

    // optional bool explicit = 9;

    pub fn clear_explicit(&mut self) {
        self.explicit = ::std::option::Option::None;
    }

    pub fn has_explicit(&self) -> bool {
        self.explicit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_explicit(&mut self, v: bool) {
        self.explicit = ::std::option::Option::Some(v);
    }

    pub fn get_explicit(&self) -> bool {
        self.explicit.unwrap_or(false)
    }

    fn get_explicit_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.explicit
    }

    fn mut_explicit_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.explicit
    }

    // repeated .ExternalId external_id = 10;

    pub fn clear_external_id(&mut self) {
        self.external_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_external_id(&mut self, v: ::protobuf::RepeatedField<ExternalId>) {
        self.external_id = v;
    }

    // Mutable pointer to the field.
    pub fn mut_external_id(&mut self) -> &mut ::protobuf::RepeatedField<ExternalId> {
        &mut self.external_id
    }

    // Take field
    pub fn take_external_id(&mut self) -> ::protobuf::RepeatedField<ExternalId> {
        ::std::mem::replace(&mut self.external_id, ::protobuf::RepeatedField::new())
    }

    pub fn get_external_id(&self) -> &[ExternalId] {
        &self.external_id
    }

    fn get_external_id_for_reflect(&self) -> &::protobuf::RepeatedField<ExternalId> {
        &self.external_id
    }

    fn mut_external_id_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ExternalId> {
        &mut self.external_id
    }

    // repeated .Restriction restriction = 11;

    pub fn clear_restriction(&mut self) {
        self.restriction.clear();
    }

    // Param is passed by value, moved
    pub fn set_restriction(&mut self, v: ::protobuf::RepeatedField<Restriction>) {
        self.restriction = v;
    }

    // Mutable pointer to the field.
    pub fn mut_restriction(&mut self) -> &mut ::protobuf::RepeatedField<Restriction> {
        &mut self.restriction
    }

    // Take field
    pub fn take_restriction(&mut self) -> ::protobuf::RepeatedField<Restriction> {
        ::std::mem::replace(&mut self.restriction, ::protobuf::RepeatedField::new())
    }

    pub fn get_restriction(&self) -> &[Restriction] {
        &self.restriction
    }

    fn get_restriction_for_reflect(&self) -> &::protobuf::RepeatedField<Restriction> {
        &self.restriction
    }

    fn mut_restriction_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Restriction> {
        &mut self.restriction
    }

    // repeated .AudioFile file = 12;

    pub fn clear_file(&mut self) {
        self.file.clear();
    }

    // Param is passed by value, moved
    pub fn set_file(&mut self, v: ::protobuf::RepeatedField<AudioFile>) {
        self.file = v;
    }

    // Mutable pointer to the field.
    pub fn mut_file(&mut self) -> &mut ::protobuf::RepeatedField<AudioFile> {
        &mut self.file
    }

    // Take field
    pub fn take_file(&mut self) -> ::protobuf::RepeatedField<AudioFile> {
        ::std::mem::replace(&mut self.file, ::protobuf::RepeatedField::new())
    }

    pub fn get_file(&self) -> &[AudioFile] {
        &self.file
    }

    fn get_file_for_reflect(&self) -> &::protobuf::RepeatedField<AudioFile> {
        &self.file
    }

    fn mut_file_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<AudioFile> {
        &mut self.file
    }

    // repeated .Track alternative = 13;

    pub fn clear_alternative(&mut self) {
        self.alternative.clear();
    }

    // Param is passed by value, moved
    pub fn set_alternative(&mut self, v: ::protobuf::RepeatedField<Track>) {
        self.alternative = v;
    }

    // Mutable pointer to the field.
    pub fn mut_alternative(&mut self) -> &mut ::protobuf::RepeatedField<Track> {
        &mut self.alternative
    }

    // Take field
    pub fn take_alternative(&mut self) -> ::protobuf::RepeatedField<Track> {
        ::std::mem::replace(&mut self.alternative, ::protobuf::RepeatedField::new())
    }

    pub fn get_alternative(&self) -> &[Track] {
        &self.alternative
    }

    fn get_alternative_for_reflect(&self) -> &::protobuf::RepeatedField<Track> {
        &self.alternative
    }

    fn mut_alternative_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Track> {
        &mut self.alternative
    }

    // repeated .SalePeriod sale_period = 14;

    pub fn clear_sale_period(&mut self) {
        self.sale_period.clear();
    }

    // Param is passed by value, moved
    pub fn set_sale_period(&mut self, v: ::protobuf::RepeatedField<SalePeriod>) {
        self.sale_period = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sale_period(&mut self) -> &mut ::protobuf::RepeatedField<SalePeriod> {
        &mut self.sale_period
    }

    // Take field
    pub fn take_sale_period(&mut self) -> ::protobuf::RepeatedField<SalePeriod> {
        ::std::mem::replace(&mut self.sale_period, ::protobuf::RepeatedField::new())
    }

    pub fn get_sale_period(&self) -> &[SalePeriod] {
        &self.sale_period
    }

    fn get_sale_period_for_reflect(&self) -> &::protobuf::RepeatedField<SalePeriod> {
        &self.sale_period
    }

    fn mut_sale_period_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<SalePeriod> {
        &mut self.sale_period
    }

    // repeated .AudioFile preview = 15;

    pub fn clear_preview(&mut self) {
        self.preview.clear();
    }

    // Param is passed by value, moved
    pub fn set_preview(&mut self, v: ::protobuf::RepeatedField<AudioFile>) {
        self.preview = v;
    }

    // Mutable pointer to the field.
    pub fn mut_preview(&mut self) -> &mut ::protobuf::RepeatedField<AudioFile> {
        &mut self.preview
    }

    // Take field
    pub fn take_preview(&mut self) -> ::protobuf::RepeatedField<AudioFile> {
        ::std::mem::replace(&mut self.preview, ::protobuf::RepeatedField::new())
    }

    pub fn get_preview(&self) -> &[AudioFile] {
        &self.preview
    }

    fn get_preview_for_reflect(&self) -> &::protobuf::RepeatedField<AudioFile> {
        &self.preview
    }

    fn mut_preview_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<AudioFile> {
        &mut self.preview
    }
}

impl ::protobuf::Message for Track {
    fn is_initialized(&self) -> bool {
        for v in &self.album {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.artist {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.external_id {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.restriction {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.file {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.alternative {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.sale_period {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.preview {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.gid)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.album)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.artist)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.number = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.disc_number = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.duration = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.popularity = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.explicit = ::std::option::Option::Some(tmp);
                },
                10 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.external_id)?;
                },
                11 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.restriction)?;
                },
                12 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.file)?;
                },
                13 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.alternative)?;
                },
                14 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.sale_period)?;
                },
                15 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.preview)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.gid.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.album.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.artist {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.number {
            my_size += ::protobuf::rt::value_varint_zigzag_size(5, v);
        }
        if let Some(v) = self.disc_number {
            my_size += ::protobuf::rt::value_varint_zigzag_size(6, v);
        }
        if let Some(v) = self.duration {
            my_size += ::protobuf::rt::value_varint_zigzag_size(7, v);
        }
        if let Some(v) = self.popularity {
            my_size += ::protobuf::rt::value_varint_zigzag_size(8, v);
        }
        if let Some(v) = self.explicit {
            my_size += 2;
        }
        for value in &self.external_id {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.restriction {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.file {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.alternative {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.sale_period {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.preview {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.gid.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.album.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.artist {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.number {
            os.write_sint32(5, v)?;
        }
        if let Some(v) = self.disc_number {
            os.write_sint32(6, v)?;
        }
        if let Some(v) = self.duration {
            os.write_sint32(7, v)?;
        }
        if let Some(v) = self.popularity {
            os.write_sint32(8, v)?;
        }
        if let Some(v) = self.explicit {
            os.write_bool(9, v)?;
        }
        for v in &self.external_id {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.restriction {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.file {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.alternative {
            os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.sale_period {
            os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.preview {
            os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Track {
    fn new() -> Track {
        Track::new()
    }

    fn descriptor_static(_: ::std::option::Option<Track>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "gid",
                    Track::get_gid_for_reflect,
                    Track::mut_gid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Track::get_name_for_reflect,
                    Track::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Album>>(
                    "album",
                    Track::get_album_for_reflect,
                    Track::mut_album_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Artist>>(
                    "artist",
                    Track::get_artist_for_reflect,
                    Track::mut_artist_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "number",
                    Track::get_number_for_reflect,
                    Track::mut_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "disc_number",
                    Track::get_disc_number_for_reflect,
                    Track::mut_disc_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "duration",
                    Track::get_duration_for_reflect,
                    Track::mut_duration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "popularity",
                    Track::get_popularity_for_reflect,
                    Track::mut_popularity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "explicit",
                    Track::get_explicit_for_reflect,
                    Track::mut_explicit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ExternalId>>(
                    "external_id",
                    Track::get_external_id_for_reflect,
                    Track::mut_external_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Restriction>>(
                    "restriction",
                    Track::get_restriction_for_reflect,
                    Track::mut_restriction_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AudioFile>>(
                    "file",
                    Track::get_file_for_reflect,
                    Track::mut_file_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Track>>(
                    "alternative",
                    Track::get_alternative_for_reflect,
                    Track::mut_alternative_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SalePeriod>>(
                    "sale_period",
                    Track::get_sale_period_for_reflect,
                    Track::mut_sale_period_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AudioFile>>(
                    "preview",
                    Track::get_preview_for_reflect,
                    Track::mut_preview_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Track>(
                    "Track",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Track {
    fn clear(&mut self) {
        self.clear_gid();
        self.clear_name();
        self.clear_album();
        self.clear_artist();
        self.clear_number();
        self.clear_disc_number();
        self.clear_duration();
        self.clear_popularity();
        self.clear_explicit();
        self.clear_external_id();
        self.clear_restriction();
        self.clear_file();
        self.clear_alternative();
        self.clear_sale_period();
        self.clear_preview();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Track {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Track {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Image {
    // message fields
    file_id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    size: ::std::option::Option<Image_Size>,
    width: ::std::option::Option<i32>,
    height: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Image {}

impl Image {
    pub fn new() -> Image {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Image {
        static mut instance: ::protobuf::lazy::Lazy<Image> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Image,
        };
        unsafe {
            instance.get(Image::new)
        }
    }

    // optional bytes file_id = 1;

    pub fn clear_file_id(&mut self) {
        self.file_id.clear();
    }

    pub fn has_file_id(&self) -> bool {
        self.file_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_file_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.file_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_file_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.file_id.is_none() {
            self.file_id.set_default();
        }
        self.file_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_file_id(&mut self) -> ::std::vec::Vec<u8> {
        self.file_id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_file_id(&self) -> &[u8] {
        match self.file_id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_file_id_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.file_id
    }

    fn mut_file_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.file_id
    }

    // optional .Image.Size size = 2;

    pub fn clear_size(&mut self) {
        self.size = ::std::option::Option::None;
    }

    pub fn has_size(&self) -> bool {
        self.size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: Image_Size) {
        self.size = ::std::option::Option::Some(v);
    }

    pub fn get_size(&self) -> Image_Size {
        self.size.unwrap_or(Image_Size::DEFAULT)
    }

    fn get_size_for_reflect(&self) -> &::std::option::Option<Image_Size> {
        &self.size
    }

    fn mut_size_for_reflect(&mut self) -> &mut ::std::option::Option<Image_Size> {
        &mut self.size
    }

    // optional sint32 width = 3;

    pub fn clear_width(&mut self) {
        self.width = ::std::option::Option::None;
    }

    pub fn has_width(&self) -> bool {
        self.width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_width(&mut self, v: i32) {
        self.width = ::std::option::Option::Some(v);
    }

    pub fn get_width(&self) -> i32 {
        self.width.unwrap_or(0)
    }

    fn get_width_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.width
    }

    fn mut_width_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.width
    }

    // optional sint32 height = 4;

    pub fn clear_height(&mut self) {
        self.height = ::std::option::Option::None;
    }

    pub fn has_height(&self) -> bool {
        self.height.is_some()
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: i32) {
        self.height = ::std::option::Option::Some(v);
    }

    pub fn get_height(&self) -> i32 {
        self.height.unwrap_or(0)
    }

    fn get_height_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.height
    }

    fn mut_height_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.height
    }
}

impl ::protobuf::Message for Image {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.file_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.size = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.width = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.height = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.file_id.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(v) = self.size {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(v) = self.width {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, v);
        }
        if let Some(v) = self.height {
            my_size += ::protobuf::rt::value_varint_zigzag_size(4, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.file_id.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(v) = self.size {
            os.write_enum(2, v.value())?;
        }
        if let Some(v) = self.width {
            os.write_sint32(3, v)?;
        }
        if let Some(v) = self.height {
            os.write_sint32(4, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Image {
    fn new() -> Image {
        Image::new()
    }

    fn descriptor_static(_: ::std::option::Option<Image>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "file_id",
                    Image::get_file_id_for_reflect,
                    Image::mut_file_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Image_Size>>(
                    "size",
                    Image::get_size_for_reflect,
                    Image::mut_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "width",
                    Image::get_width_for_reflect,
                    Image::mut_width_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "height",
                    Image::get_height_for_reflect,
                    Image::mut_height_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Image>(
                    "Image",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Image {
    fn clear(&mut self) {
        self.clear_file_id();
        self.clear_size();
        self.clear_width();
        self.clear_height();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Image {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Image {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Image_Size {
    DEFAULT = 0,
    SMALL = 1,
    LARGE = 2,
    XLARGE = 3,
}

impl ::protobuf::ProtobufEnum for Image_Size {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Image_Size> {
        match value {
            0 => ::std::option::Option::Some(Image_Size::DEFAULT),
            1 => ::std::option::Option::Some(Image_Size::SMALL),
            2 => ::std::option::Option::Some(Image_Size::LARGE),
            3 => ::std::option::Option::Some(Image_Size::XLARGE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Image_Size] = &[
            Image_Size::DEFAULT,
            Image_Size::SMALL,
            Image_Size::LARGE,
            Image_Size::XLARGE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Image_Size>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Image_Size", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Image_Size {
}

impl ::protobuf::reflect::ProtobufValue for Image_Size {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ImageGroup {
    // message fields
    image: ::protobuf::RepeatedField<Image>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ImageGroup {}

impl ImageGroup {
    pub fn new() -> ImageGroup {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ImageGroup {
        static mut instance: ::protobuf::lazy::Lazy<ImageGroup> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ImageGroup,
        };
        unsafe {
            instance.get(ImageGroup::new)
        }
    }

    // repeated .Image image = 1;

    pub fn clear_image(&mut self) {
        self.image.clear();
    }

    // Param is passed by value, moved
    pub fn set_image(&mut self, v: ::protobuf::RepeatedField<Image>) {
        self.image = v;
    }

    // Mutable pointer to the field.
    pub fn mut_image(&mut self) -> &mut ::protobuf::RepeatedField<Image> {
        &mut self.image
    }

    // Take field
    pub fn take_image(&mut self) -> ::protobuf::RepeatedField<Image> {
        ::std::mem::replace(&mut self.image, ::protobuf::RepeatedField::new())
    }

    pub fn get_image(&self) -> &[Image] {
        &self.image
    }

    fn get_image_for_reflect(&self) -> &::protobuf::RepeatedField<Image> {
        &self.image
    }

    fn mut_image_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Image> {
        &mut self.image
    }
}

impl ::protobuf::Message for ImageGroup {
    fn is_initialized(&self) -> bool {
        for v in &self.image {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.image)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.image {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.image {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ImageGroup {
    fn new() -> ImageGroup {
        ImageGroup::new()
    }

    fn descriptor_static(_: ::std::option::Option<ImageGroup>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Image>>(
                    "image",
                    ImageGroup::get_image_for_reflect,
                    ImageGroup::mut_image_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ImageGroup>(
                    "ImageGroup",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ImageGroup {
    fn clear(&mut self) {
        self.clear_image();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ImageGroup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ImageGroup {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Biography {
    // message fields
    text: ::protobuf::SingularField<::std::string::String>,
    portrait: ::protobuf::RepeatedField<Image>,
    portrait_group: ::protobuf::RepeatedField<ImageGroup>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Biography {}

impl Biography {
    pub fn new() -> Biography {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Biography {
        static mut instance: ::protobuf::lazy::Lazy<Biography> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Biography,
        };
        unsafe {
            instance.get(Biography::new)
        }
    }

    // optional string text = 1;

    pub fn clear_text(&mut self) {
        self.text.clear();
    }

    pub fn has_text(&self) -> bool {
        self.text.is_some()
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: ::std::string::String) {
        self.text = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text(&mut self) -> &mut ::std::string::String {
        if self.text.is_none() {
            self.text.set_default();
        }
        self.text.as_mut().unwrap()
    }

    // Take field
    pub fn take_text(&mut self) -> ::std::string::String {
        self.text.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_text(&self) -> &str {
        match self.text.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_text_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.text
    }

    fn mut_text_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.text
    }

    // repeated .Image portrait = 2;

    pub fn clear_portrait(&mut self) {
        self.portrait.clear();
    }

    // Param is passed by value, moved
    pub fn set_portrait(&mut self, v: ::protobuf::RepeatedField<Image>) {
        self.portrait = v;
    }

    // Mutable pointer to the field.
    pub fn mut_portrait(&mut self) -> &mut ::protobuf::RepeatedField<Image> {
        &mut self.portrait
    }

    // Take field
    pub fn take_portrait(&mut self) -> ::protobuf::RepeatedField<Image> {
        ::std::mem::replace(&mut self.portrait, ::protobuf::RepeatedField::new())
    }

    pub fn get_portrait(&self) -> &[Image] {
        &self.portrait
    }

    fn get_portrait_for_reflect(&self) -> &::protobuf::RepeatedField<Image> {
        &self.portrait
    }

    fn mut_portrait_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Image> {
        &mut self.portrait
    }

    // repeated .ImageGroup portrait_group = 3;

    pub fn clear_portrait_group(&mut self) {
        self.portrait_group.clear();
    }

    // Param is passed by value, moved
    pub fn set_portrait_group(&mut self, v: ::protobuf::RepeatedField<ImageGroup>) {
        self.portrait_group = v;
    }

    // Mutable pointer to the field.
    pub fn mut_portrait_group(&mut self) -> &mut ::protobuf::RepeatedField<ImageGroup> {
        &mut self.portrait_group
    }

    // Take field
    pub fn take_portrait_group(&mut self) -> ::protobuf::RepeatedField<ImageGroup> {
        ::std::mem::replace(&mut self.portrait_group, ::protobuf::RepeatedField::new())
    }

    pub fn get_portrait_group(&self) -> &[ImageGroup] {
        &self.portrait_group
    }

    fn get_portrait_group_for_reflect(&self) -> &::protobuf::RepeatedField<ImageGroup> {
        &self.portrait_group
    }

    fn mut_portrait_group_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ImageGroup> {
        &mut self.portrait_group
    }
}

impl ::protobuf::Message for Biography {
    fn is_initialized(&self) -> bool {
        for v in &self.portrait {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.portrait_group {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.text)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.portrait)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.portrait_group)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.text.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.portrait {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.portrait_group {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.text.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.portrait {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.portrait_group {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Biography {
    fn new() -> Biography {
        Biography::new()
    }

    fn descriptor_static(_: ::std::option::Option<Biography>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "text",
                    Biography::get_text_for_reflect,
                    Biography::mut_text_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Image>>(
                    "portrait",
                    Biography::get_portrait_for_reflect,
                    Biography::mut_portrait_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ImageGroup>>(
                    "portrait_group",
                    Biography::get_portrait_group_for_reflect,
                    Biography::mut_portrait_group_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Biography>(
                    "Biography",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Biography {
    fn clear(&mut self) {
        self.clear_text();
        self.clear_portrait();
        self.clear_portrait_group();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Biography {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Biography {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Disc {
    // message fields
    number: ::std::option::Option<i32>,
    name: ::protobuf::SingularField<::std::string::String>,
    track: ::protobuf::RepeatedField<Track>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Disc {}

impl Disc {
    pub fn new() -> Disc {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Disc {
        static mut instance: ::protobuf::lazy::Lazy<Disc> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Disc,
        };
        unsafe {
            instance.get(Disc::new)
        }
    }

    // optional sint32 number = 1;

    pub fn clear_number(&mut self) {
        self.number = ::std::option::Option::None;
    }

    pub fn has_number(&self) -> bool {
        self.number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_number(&mut self, v: i32) {
        self.number = ::std::option::Option::Some(v);
    }

    pub fn get_number(&self) -> i32 {
        self.number.unwrap_or(0)
    }

    fn get_number_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.number
    }

    fn mut_number_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.number
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // repeated .Track track = 3;

    pub fn clear_track(&mut self) {
        self.track.clear();
    }

    // Param is passed by value, moved
    pub fn set_track(&mut self, v: ::protobuf::RepeatedField<Track>) {
        self.track = v;
    }

    // Mutable pointer to the field.
    pub fn mut_track(&mut self) -> &mut ::protobuf::RepeatedField<Track> {
        &mut self.track
    }

    // Take field
    pub fn take_track(&mut self) -> ::protobuf::RepeatedField<Track> {
        ::std::mem::replace(&mut self.track, ::protobuf::RepeatedField::new())
    }

    pub fn get_track(&self) -> &[Track] {
        &self.track
    }

    fn get_track_for_reflect(&self) -> &::protobuf::RepeatedField<Track> {
        &self.track
    }

    fn mut_track_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Track> {
        &mut self.track
    }
}

impl ::protobuf::Message for Disc {
    fn is_initialized(&self) -> bool {
        for v in &self.track {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.number = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.track)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.number {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        for value in &self.track {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.number {
            os.write_sint32(1, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        }
        for v in &self.track {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Disc {
    fn new() -> Disc {
        Disc::new()
    }

    fn descriptor_static(_: ::std::option::Option<Disc>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "number",
                    Disc::get_number_for_reflect,
                    Disc::mut_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Disc::get_name_for_reflect,
                    Disc::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Track>>(
                    "track",
                    Disc::get_track_for_reflect,
                    Disc::mut_track_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Disc>(
                    "Disc",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Disc {
    fn clear(&mut self) {
        self.clear_number();
        self.clear_name();
        self.clear_track();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Disc {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Disc {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Copyright {
    // message fields
    typ: ::std::option::Option<Copyright_Type>,
    text: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Copyright {}

impl Copyright {
    pub fn new() -> Copyright {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Copyright {
        static mut instance: ::protobuf::lazy::Lazy<Copyright> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Copyright,
        };
        unsafe {
            instance.get(Copyright::new)
        }
    }

    // optional .Copyright.Type typ = 1;

    pub fn clear_typ(&mut self) {
        self.typ = ::std::option::Option::None;
    }

    pub fn has_typ(&self) -> bool {
        self.typ.is_some()
    }

    // Param is passed by value, moved
    pub fn set_typ(&mut self, v: Copyright_Type) {
        self.typ = ::std::option::Option::Some(v);
    }

    pub fn get_typ(&self) -> Copyright_Type {
        self.typ.unwrap_or(Copyright_Type::P)
    }

    fn get_typ_for_reflect(&self) -> &::std::option::Option<Copyright_Type> {
        &self.typ
    }

    fn mut_typ_for_reflect(&mut self) -> &mut ::std::option::Option<Copyright_Type> {
        &mut self.typ
    }

    // optional string text = 2;

    pub fn clear_text(&mut self) {
        self.text.clear();
    }

    pub fn has_text(&self) -> bool {
        self.text.is_some()
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: ::std::string::String) {
        self.text = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text(&mut self) -> &mut ::std::string::String {
        if self.text.is_none() {
            self.text.set_default();
        }
        self.text.as_mut().unwrap()
    }

    // Take field
    pub fn take_text(&mut self) -> ::std::string::String {
        self.text.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_text(&self) -> &str {
        match self.text.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_text_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.text
    }

    fn mut_text_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.text
    }
}

impl ::protobuf::Message for Copyright {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.typ = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.text)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.typ {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.text.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.typ {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.text.as_ref() {
            os.write_string(2, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Copyright {
    fn new() -> Copyright {
        Copyright::new()
    }

    fn descriptor_static(_: ::std::option::Option<Copyright>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Copyright_Type>>(
                    "typ",
                    Copyright::get_typ_for_reflect,
                    Copyright::mut_typ_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "text",
                    Copyright::get_text_for_reflect,
                    Copyright::mut_text_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Copyright>(
                    "Copyright",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Copyright {
    fn clear(&mut self) {
        self.clear_typ();
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Copyright {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Copyright {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Copyright_Type {
    P = 0,
    C = 1,
}

impl ::protobuf::ProtobufEnum for Copyright_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Copyright_Type> {
        match value {
            0 => ::std::option::Option::Some(Copyright_Type::P),
            1 => ::std::option::Option::Some(Copyright_Type::C),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Copyright_Type] = &[
            Copyright_Type::P,
            Copyright_Type::C,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Copyright_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Copyright_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Copyright_Type {
}

impl ::protobuf::reflect::ProtobufValue for Copyright_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Restriction {
    // message fields
    countries_allowed: ::protobuf::SingularField<::std::string::String>,
    countries_forbidden: ::protobuf::SingularField<::std::string::String>,
    typ: ::std::option::Option<Restriction_Type>,
    catalogue_str: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Restriction {}

impl Restriction {
    pub fn new() -> Restriction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Restriction {
        static mut instance: ::protobuf::lazy::Lazy<Restriction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Restriction,
        };
        unsafe {
            instance.get(Restriction::new)
        }
    }

    // optional string countries_allowed = 2;

    pub fn clear_countries_allowed(&mut self) {
        self.countries_allowed.clear();
    }

    pub fn has_countries_allowed(&self) -> bool {
        self.countries_allowed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_countries_allowed(&mut self, v: ::std::string::String) {
        self.countries_allowed = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_countries_allowed(&mut self) -> &mut ::std::string::String {
        if self.countries_allowed.is_none() {
            self.countries_allowed.set_default();
        }
        self.countries_allowed.as_mut().unwrap()
    }

    // Take field
    pub fn take_countries_allowed(&mut self) -> ::std::string::String {
        self.countries_allowed.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_countries_allowed(&self) -> &str {
        match self.countries_allowed.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_countries_allowed_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.countries_allowed
    }

    fn mut_countries_allowed_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.countries_allowed
    }

    // optional string countries_forbidden = 3;

    pub fn clear_countries_forbidden(&mut self) {
        self.countries_forbidden.clear();
    }

    pub fn has_countries_forbidden(&self) -> bool {
        self.countries_forbidden.is_some()
    }

    // Param is passed by value, moved
    pub fn set_countries_forbidden(&mut self, v: ::std::string::String) {
        self.countries_forbidden = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_countries_forbidden(&mut self) -> &mut ::std::string::String {
        if self.countries_forbidden.is_none() {
            self.countries_forbidden.set_default();
        }
        self.countries_forbidden.as_mut().unwrap()
    }

    // Take field
    pub fn take_countries_forbidden(&mut self) -> ::std::string::String {
        self.countries_forbidden.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_countries_forbidden(&self) -> &str {
        match self.countries_forbidden.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_countries_forbidden_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.countries_forbidden
    }

    fn mut_countries_forbidden_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.countries_forbidden
    }

    // optional .Restriction.Type typ = 4;

    pub fn clear_typ(&mut self) {
        self.typ = ::std::option::Option::None;
    }

    pub fn has_typ(&self) -> bool {
        self.typ.is_some()
    }

    // Param is passed by value, moved
    pub fn set_typ(&mut self, v: Restriction_Type) {
        self.typ = ::std::option::Option::Some(v);
    }

    pub fn get_typ(&self) -> Restriction_Type {
        self.typ.unwrap_or(Restriction_Type::STREAMING)
    }

    fn get_typ_for_reflect(&self) -> &::std::option::Option<Restriction_Type> {
        &self.typ
    }

    fn mut_typ_for_reflect(&mut self) -> &mut ::std::option::Option<Restriction_Type> {
        &mut self.typ
    }

    // repeated string catalogue_str = 5;

    pub fn clear_catalogue_str(&mut self) {
        self.catalogue_str.clear();
    }

    // Param is passed by value, moved
    pub fn set_catalogue_str(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.catalogue_str = v;
    }

    // Mutable pointer to the field.
    pub fn mut_catalogue_str(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.catalogue_str
    }

    // Take field
    pub fn take_catalogue_str(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.catalogue_str, ::protobuf::RepeatedField::new())
    }

    pub fn get_catalogue_str(&self) -> &[::std::string::String] {
        &self.catalogue_str
    }

    fn get_catalogue_str_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.catalogue_str
    }

    fn mut_catalogue_str_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.catalogue_str
    }
}

impl ::protobuf::Message for Restriction {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.countries_allowed)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.countries_forbidden)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.typ = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.catalogue_str)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.countries_allowed.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.countries_forbidden.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.typ {
            my_size += ::protobuf::rt::enum_size(4, v);
        }
        for value in &self.catalogue_str {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.countries_allowed.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.countries_forbidden.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(v) = self.typ {
            os.write_enum(4, v.value())?;
        }
        for v in &self.catalogue_str {
            os.write_string(5, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Restriction {
    fn new() -> Restriction {
        Restriction::new()
    }

    fn descriptor_static(_: ::std::option::Option<Restriction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "countries_allowed",
                    Restriction::get_countries_allowed_for_reflect,
                    Restriction::mut_countries_allowed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "countries_forbidden",
                    Restriction::get_countries_forbidden_for_reflect,
                    Restriction::mut_countries_forbidden_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Restriction_Type>>(
                    "typ",
                    Restriction::get_typ_for_reflect,
                    Restriction::mut_typ_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "catalogue_str",
                    Restriction::get_catalogue_str_for_reflect,
                    Restriction::mut_catalogue_str_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Restriction>(
                    "Restriction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Restriction {
    fn clear(&mut self) {
        self.clear_countries_allowed();
        self.clear_countries_forbidden();
        self.clear_typ();
        self.clear_catalogue_str();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Restriction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Restriction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Restriction_Type {
    STREAMING = 0,
}

impl ::protobuf::ProtobufEnum for Restriction_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Restriction_Type> {
        match value {
            0 => ::std::option::Option::Some(Restriction_Type::STREAMING),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Restriction_Type] = &[
            Restriction_Type::STREAMING,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Restriction_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Restriction_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Restriction_Type {
}

impl ::protobuf::reflect::ProtobufValue for Restriction_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SalePeriod {
    // message fields
    restriction: ::protobuf::RepeatedField<Restriction>,
    start: ::protobuf::SingularPtrField<Date>,
    end: ::protobuf::SingularPtrField<Date>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SalePeriod {}

impl SalePeriod {
    pub fn new() -> SalePeriod {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SalePeriod {
        static mut instance: ::protobuf::lazy::Lazy<SalePeriod> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SalePeriod,
        };
        unsafe {
            instance.get(SalePeriod::new)
        }
    }

    // repeated .Restriction restriction = 1;

    pub fn clear_restriction(&mut self) {
        self.restriction.clear();
    }

    // Param is passed by value, moved
    pub fn set_restriction(&mut self, v: ::protobuf::RepeatedField<Restriction>) {
        self.restriction = v;
    }

    // Mutable pointer to the field.
    pub fn mut_restriction(&mut self) -> &mut ::protobuf::RepeatedField<Restriction> {
        &mut self.restriction
    }

    // Take field
    pub fn take_restriction(&mut self) -> ::protobuf::RepeatedField<Restriction> {
        ::std::mem::replace(&mut self.restriction, ::protobuf::RepeatedField::new())
    }

    pub fn get_restriction(&self) -> &[Restriction] {
        &self.restriction
    }

    fn get_restriction_for_reflect(&self) -> &::protobuf::RepeatedField<Restriction> {
        &self.restriction
    }

    fn mut_restriction_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Restriction> {
        &mut self.restriction
    }

    // optional .Date start = 2;

    pub fn clear_start(&mut self) {
        self.start.clear();
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: Date) {
        self.start = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start(&mut self) -> &mut Date {
        if self.start.is_none() {
            self.start.set_default();
        }
        self.start.as_mut().unwrap()
    }

    // Take field
    pub fn take_start(&mut self) -> Date {
        self.start.take().unwrap_or_else(|| Date::new())
    }

    pub fn get_start(&self) -> &Date {
        self.start.as_ref().unwrap_or_else(|| Date::default_instance())
    }

    fn get_start_for_reflect(&self) -> &::protobuf::SingularPtrField<Date> {
        &self.start
    }

    fn mut_start_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Date> {
        &mut self.start
    }

    // optional .Date end = 3;

    pub fn clear_end(&mut self) {
        self.end.clear();
    }

    pub fn has_end(&self) -> bool {
        self.end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end(&mut self, v: Date) {
        self.end = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end(&mut self) -> &mut Date {
        if self.end.is_none() {
            self.end.set_default();
        }
        self.end.as_mut().unwrap()
    }

    // Take field
    pub fn take_end(&mut self) -> Date {
        self.end.take().unwrap_or_else(|| Date::new())
    }

    pub fn get_end(&self) -> &Date {
        self.end.as_ref().unwrap_or_else(|| Date::default_instance())
    }

    fn get_end_for_reflect(&self) -> &::protobuf::SingularPtrField<Date> {
        &self.end
    }

    fn mut_end_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Date> {
        &mut self.end
    }
}

impl ::protobuf::Message for SalePeriod {
    fn is_initialized(&self) -> bool {
        for v in &self.restriction {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.start {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.end {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.restriction)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.start)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.end)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.restriction {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.start.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.end.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.restriction {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.start.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.end.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SalePeriod {
    fn new() -> SalePeriod {
        SalePeriod::new()
    }

    fn descriptor_static(_: ::std::option::Option<SalePeriod>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Restriction>>(
                    "restriction",
                    SalePeriod::get_restriction_for_reflect,
                    SalePeriod::mut_restriction_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Date>>(
                    "start",
                    SalePeriod::get_start_for_reflect,
                    SalePeriod::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Date>>(
                    "end",
                    SalePeriod::get_end_for_reflect,
                    SalePeriod::mut_end_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SalePeriod>(
                    "SalePeriod",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SalePeriod {
    fn clear(&mut self) {
        self.clear_restriction();
        self.clear_start();
        self.clear_end();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SalePeriod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SalePeriod {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ExternalId {
    // message fields
    typ: ::protobuf::SingularField<::std::string::String>,
    id: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExternalId {}

impl ExternalId {
    pub fn new() -> ExternalId {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExternalId {
        static mut instance: ::protobuf::lazy::Lazy<ExternalId> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExternalId,
        };
        unsafe {
            instance.get(ExternalId::new)
        }
    }

    // optional string typ = 1;

    pub fn clear_typ(&mut self) {
        self.typ.clear();
    }

    pub fn has_typ(&self) -> bool {
        self.typ.is_some()
    }

    // Param is passed by value, moved
    pub fn set_typ(&mut self, v: ::std::string::String) {
        self.typ = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_typ(&mut self) -> &mut ::std::string::String {
        if self.typ.is_none() {
            self.typ.set_default();
        }
        self.typ.as_mut().unwrap()
    }

    // Take field
    pub fn take_typ(&mut self) -> ::std::string::String {
        self.typ.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_typ(&self) -> &str {
        match self.typ.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_typ_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.typ
    }

    fn mut_typ_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.typ
    }

    // optional string id = 2;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        if self.id.is_none() {
            self.id.set_default();
        }
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        self.id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_id(&self) -> &str {
        match self.id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_id_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.id
    }
}

impl ::protobuf::Message for ExternalId {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.typ)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.typ.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.id.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.typ.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.id.as_ref() {
            os.write_string(2, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ExternalId {
    fn new() -> ExternalId {
        ExternalId::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExternalId>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "typ",
                    ExternalId::get_typ_for_reflect,
                    ExternalId::mut_typ_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "id",
                    ExternalId::get_id_for_reflect,
                    ExternalId::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExternalId>(
                    "ExternalId",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExternalId {
    fn clear(&mut self) {
        self.clear_typ();
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExternalId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExternalId {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AudioFile {
    // message fields
    file_id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    format: ::std::option::Option<AudioFile_Format>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AudioFile {}

impl AudioFile {
    pub fn new() -> AudioFile {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AudioFile {
        static mut instance: ::protobuf::lazy::Lazy<AudioFile> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AudioFile,
        };
        unsafe {
            instance.get(AudioFile::new)
        }
    }

    // optional bytes file_id = 1;

    pub fn clear_file_id(&mut self) {
        self.file_id.clear();
    }

    pub fn has_file_id(&self) -> bool {
        self.file_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_file_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.file_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_file_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.file_id.is_none() {
            self.file_id.set_default();
        }
        self.file_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_file_id(&mut self) -> ::std::vec::Vec<u8> {
        self.file_id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_file_id(&self) -> &[u8] {
        match self.file_id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_file_id_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.file_id
    }

    fn mut_file_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.file_id
    }

    // optional .AudioFile.Format format = 2;

    pub fn clear_format(&mut self) {
        self.format = ::std::option::Option::None;
    }

    pub fn has_format(&self) -> bool {
        self.format.is_some()
    }

    // Param is passed by value, moved
    pub fn set_format(&mut self, v: AudioFile_Format) {
        self.format = ::std::option::Option::Some(v);
    }

    pub fn get_format(&self) -> AudioFile_Format {
        self.format.unwrap_or(AudioFile_Format::OGG_VORBIS_96)
    }

    fn get_format_for_reflect(&self) -> &::std::option::Option<AudioFile_Format> {
        &self.format
    }

    fn mut_format_for_reflect(&mut self) -> &mut ::std::option::Option<AudioFile_Format> {
        &mut self.format
    }
}

impl ::protobuf::Message for AudioFile {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.file_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.format = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.file_id.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(v) = self.format {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.file_id.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(v) = self.format {
            os.write_enum(2, v.value())?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AudioFile {
    fn new() -> AudioFile {
        AudioFile::new()
    }

    fn descriptor_static(_: ::std::option::Option<AudioFile>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "file_id",
                    AudioFile::get_file_id_for_reflect,
                    AudioFile::mut_file_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<AudioFile_Format>>(
                    "format",
                    AudioFile::get_format_for_reflect,
                    AudioFile::mut_format_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AudioFile>(
                    "AudioFile",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AudioFile {
    fn clear(&mut self) {
        self.clear_file_id();
        self.clear_format();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AudioFile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AudioFile {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AudioFile_Format {
    OGG_VORBIS_96 = 0,
    OGG_VORBIS_160 = 1,
    OGG_VORBIS_320 = 2,
    MP3_256 = 3,
    MP3_320 = 4,
    MP3_160 = 5,
    MP3_96 = 6,
    MP3_160_ENC = 7,
    OTHER2 = 8,
    OTHER3 = 9,
    AAC_160 = 10,
    AAC_320 = 11,
    OTHER4 = 12,
    OTHER5 = 13,
}

impl ::protobuf::ProtobufEnum for AudioFile_Format {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AudioFile_Format> {
        match value {
            0 => ::std::option::Option::Some(AudioFile_Format::OGG_VORBIS_96),
            1 => ::std::option::Option::Some(AudioFile_Format::OGG_VORBIS_160),
            2 => ::std::option::Option::Some(AudioFile_Format::OGG_VORBIS_320),
            3 => ::std::option::Option::Some(AudioFile_Format::MP3_256),
            4 => ::std::option::Option::Some(AudioFile_Format::MP3_320),
            5 => ::std::option::Option::Some(AudioFile_Format::MP3_160),
            6 => ::std::option::Option::Some(AudioFile_Format::MP3_96),
            7 => ::std::option::Option::Some(AudioFile_Format::MP3_160_ENC),
            8 => ::std::option::Option::Some(AudioFile_Format::OTHER2),
            9 => ::std::option::Option::Some(AudioFile_Format::OTHER3),
            10 => ::std::option::Option::Some(AudioFile_Format::AAC_160),
            11 => ::std::option::Option::Some(AudioFile_Format::AAC_320),
            12 => ::std::option::Option::Some(AudioFile_Format::OTHER4),
            13 => ::std::option::Option::Some(AudioFile_Format::OTHER5),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AudioFile_Format] = &[
            AudioFile_Format::OGG_VORBIS_96,
            AudioFile_Format::OGG_VORBIS_160,
            AudioFile_Format::OGG_VORBIS_320,
            AudioFile_Format::MP3_256,
            AudioFile_Format::MP3_320,
            AudioFile_Format::MP3_160,
            AudioFile_Format::MP3_96,
            AudioFile_Format::MP3_160_ENC,
            AudioFile_Format::OTHER2,
            AudioFile_Format::OTHER3,
            AudioFile_Format::AAC_160,
            AudioFile_Format::AAC_320,
            AudioFile_Format::OTHER4,
            AudioFile_Format::OTHER5,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<AudioFile_Format>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("AudioFile_Format", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for AudioFile_Format {
}

impl ::protobuf::reflect::ProtobufValue for AudioFile_Format {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0emetadata.proto\"C\n\tTopTracks\x12\x18\n\x07country\x18\x01\x20\
    \x01(\tR\x07country\x12\x1c\n\x05track\x18\x02\x20\x03(\x0b2\x06.TrackR\
    \x05track\"b\n\x0eActivityPeriod\x12\x1d\n\nstart_year\x18\x01\x20\x01(\
    \x11R\tstartYear\x12\x19\n\x08end_year\x18\x02\x20\x01(\x11R\x07endYear\
    \x12\x16\n\x06decade\x18\x03\x20\x01(\x11R\x06decade\"\xd0\x05\n\x06Arti\
    st\x12\x10\n\x03gid\x18\x01\x20\x01(\x0cR\x03gid\x12\x12\n\x04name\x18\
    \x02\x20\x01(\tR\x04name\x12\x1e\n\npopularity\x18\x03\x20\x01(\x11R\npo\
    pularity\x12'\n\ttop_track\x18\x04\x20\x03(\x0b2\n.TopTracksR\x08topTrac\
    k\x12,\n\x0balbum_group\x18\x05\x20\x03(\x0b2\x0b.AlbumGroupR\nalbumGrou\
    p\x12.\n\x0csingle_group\x18\x06\x20\x03(\x0b2\x0b.AlbumGroupR\x0bsingle\
    Group\x128\n\x11compilation_group\x18\x07\x20\x03(\x0b2\x0b.AlbumGroupR\
    \x10compilationGroup\x125\n\x10appears_on_group\x18\x08\x20\x03(\x0b2\
    \x0b.AlbumGroupR\x0eappearsOnGroup\x12\x14\n\x05genre\x18\t\x20\x03(\tR\
    \x05genre\x12,\n\x0bexternal_id\x18\n\x20\x03(\x0b2\x0b.ExternalIdR\next\
    ernalId\x12\"\n\x08portrait\x18\x0b\x20\x03(\x0b2\x06.ImageR\x08portrait\
    \x12(\n\tbiography\x18\x0c\x20\x03(\x0b2\n.BiographyR\tbiography\x128\n\
    \x0factivity_period\x18\r\x20\x03(\x0b2\x0f.ActivityPeriodR\x0eactivityP\
    eriod\x12.\n\x0brestriction\x18\x0e\x20\x03(\x0b2\x0c.RestrictionR\x0bre\
    striction\x12!\n\x07related\x18\x0f\x20\x03(\x0b2\x07.ArtistR\x07related\
    \x125\n\x17is_portrait_album_cover\x18\x10\x20\x01(\x08R\x14isPortraitAl\
    bumCover\x122\n\x0eportrait_group\x18\x11\x20\x01(\x0b2\x0b.ImageGroupR\
    \rportraitGroup\"*\n\nAlbumGroup\x12\x1c\n\x05album\x18\x01\x20\x03(\x0b\
    2\x06.AlbumR\x05album\"B\n\x04Date\x12\x12\n\x04year\x18\x01\x20\x01(\
    \x11R\x04year\x12\x14\n\x05month\x18\x02\x20\x01(\x11R\x05month\x12\x10\
    \n\x03day\x18\x03\x20\x01(\x11R\x03day\"\xe3\x04\n\x05Album\x12\x10\n\
    \x03gid\x18\x01\x20\x01(\x0cR\x03gid\x12\x12\n\x04name\x18\x02\x20\x01(\
    \tR\x04name\x12\x1f\n\x06artist\x18\x03\x20\x03(\x0b2\x07.ArtistR\x06art\
    ist\x12\x1d\n\x03typ\x18\x04\x20\x01(\x0e2\x0b.Album.TypeR\x03typ\x12\
    \x14\n\x05label\x18\x05\x20\x01(\tR\x05label\x12\x19\n\x04date\x18\x06\
    \x20\x01(\x0b2\x05.DateR\x04date\x12\x1e\n\npopularity\x18\x07\x20\x01(\
    \x11R\npopularity\x12\x14\n\x05genre\x18\x08\x20\x03(\tR\x05genre\x12\
    \x1c\n\x05cover\x18\t\x20\x03(\x0b2\x06.ImageR\x05cover\x12,\n\x0bextern\
    al_id\x18\n\x20\x03(\x0b2\x0b.ExternalIdR\nexternalId\x12\x19\n\x04disc\
    \x18\x0b\x20\x03(\x0b2\x05.DiscR\x04disc\x12\x16\n\x06review\x18\x0c\x20\
    \x03(\tR\x06review\x12(\n\tcopyright\x18\r\x20\x03(\x0b2\n.CopyrightR\tc\
    opyright\x12.\n\x0brestriction\x18\x0e\x20\x03(\x0b2\x0c.RestrictionR\
    \x0brestriction\x12\x20\n\x07related\x18\x0f\x20\x03(\x0b2\x06.AlbumR\
    \x07related\x12,\n\x0bsale_period\x18\x10\x20\x03(\x0b2\x0b.SalePeriodR\
    \nsalePeriod\x12,\n\x0bcover_group\x18\x11\x20\x01(\x0b2\x0b.ImageGroupR\
    \ncoverGroup\"6\n\x04Type\x12\t\n\x05ALBUM\x10\x01\x12\n\n\x06SINGLE\x10\
    \x02\x12\x0f\n\x0bCOMPILATION\x10\x03\x12\x06\n\x02EP\x10\x04\"\xf9\x03\
    \n\x05Track\x12\x10\n\x03gid\x18\x01\x20\x01(\x0cR\x03gid\x12\x12\n\x04n\
    ame\x18\x02\x20\x01(\tR\x04name\x12\x1c\n\x05album\x18\x03\x20\x01(\x0b2\
    \x06.AlbumR\x05album\x12\x1f\n\x06artist\x18\x04\x20\x03(\x0b2\x07.Artis\
    tR\x06artist\x12\x16\n\x06number\x18\x05\x20\x01(\x11R\x06number\x12\x1f\
    \n\x0bdisc_number\x18\x06\x20\x01(\x11R\ndiscNumber\x12\x1a\n\x08duratio\
    n\x18\x07\x20\x01(\x11R\x08duration\x12\x1e\n\npopularity\x18\x08\x20\
    \x01(\x11R\npopularity\x12\x1a\n\x08explicit\x18\t\x20\x01(\x08R\x08expl\
    icit\x12,\n\x0bexternal_id\x18\n\x20\x03(\x0b2\x0b.ExternalIdR\nexternal\
    Id\x12.\n\x0brestriction\x18\x0b\x20\x03(\x0b2\x0c.RestrictionR\x0brestr\
    iction\x12\x1e\n\x04file\x18\x0c\x20\x03(\x0b2\n.AudioFileR\x04file\x12(\
    \n\x0balternative\x18\r\x20\x03(\x0b2\x06.TrackR\x0balternative\x12,\n\
    \x0bsale_period\x18\x0e\x20\x03(\x0b2\x0b.SalePeriodR\nsalePeriod\x12$\n\
    \x07preview\x18\x0f\x20\x03(\x0b2\n.AudioFileR\x07preview\"\xa6\x01\n\
    \x05Image\x12\x17\n\x07file_id\x18\x01\x20\x01(\x0cR\x06fileId\x12\x1f\n\
    \x04size\x18\x02\x20\x01(\x0e2\x0b.Image.SizeR\x04size\x12\x14\n\x05widt\
    h\x18\x03\x20\x01(\x11R\x05width\x12\x16\n\x06height\x18\x04\x20\x01(\
    \x11R\x06height\"5\n\x04Size\x12\x0b\n\x07DEFAULT\x10\0\x12\t\n\x05SMALL\
    \x10\x01\x12\t\n\x05LARGE\x10\x02\x12\n\n\x06XLARGE\x10\x03\"*\n\nImageG\
    roup\x12\x1c\n\x05image\x18\x01\x20\x03(\x0b2\x06.ImageR\x05image\"w\n\t\
    Biography\x12\x12\n\x04text\x18\x01\x20\x01(\tR\x04text\x12\"\n\x08portr\
    ait\x18\x02\x20\x03(\x0b2\x06.ImageR\x08portrait\x122\n\x0eportrait_grou\
    p\x18\x03\x20\x03(\x0b2\x0b.ImageGroupR\rportraitGroup\"P\n\x04Disc\x12\
    \x16\n\x06number\x18\x01\x20\x01(\x11R\x06number\x12\x12\n\x04name\x18\
    \x02\x20\x01(\tR\x04name\x12\x1c\n\x05track\x18\x03\x20\x03(\x0b2\x06.Tr\
    ackR\x05track\"X\n\tCopyright\x12!\n\x03typ\x18\x01\x20\x01(\x0e2\x0f.Co\
    pyright.TypeR\x03typ\x12\x12\n\x04text\x18\x02\x20\x01(\tR\x04text\"\x14\
    \n\x04Type\x12\x05\n\x01P\x10\0\x12\x05\n\x01C\x10\x01\"\xcc\x01\n\x0bRe\
    striction\x12+\n\x11countries_allowed\x18\x02\x20\x01(\tR\x10countriesAl\
    lowed\x12/\n\x13countries_forbidden\x18\x03\x20\x01(\tR\x12countriesForb\
    idden\x12#\n\x03typ\x18\x04\x20\x01(\x0e2\x11.Restriction.TypeR\x03typ\
    \x12#\n\rcatalogue_str\x18\x05\x20\x03(\tR\x0ccatalogueStr\"\x15\n\x04Ty\
    pe\x12\r\n\tSTREAMING\x10\0\"r\n\nSalePeriod\x12.\n\x0brestriction\x18\
    \x01\x20\x03(\x0b2\x0c.RestrictionR\x0brestriction\x12\x1b\n\x05start\
    \x18\x02\x20\x01(\x0b2\x05.DateR\x05start\x12\x17\n\x03end\x18\x03\x20\
    \x01(\x0b2\x05.DateR\x03end\".\n\nExternalId\x12\x10\n\x03typ\x18\x01\
    \x20\x01(\tR\x03typ\x12\x0e\n\x02id\x18\x02\x20\x01(\tR\x02id\"\xa3\x02\
    \n\tAudioFile\x12\x17\n\x07file_id\x18\x01\x20\x01(\x0cR\x06fileId\x12)\
    \n\x06format\x18\x02\x20\x01(\x0e2\x11.AudioFile.FormatR\x06format\"\xd1\
    \x01\n\x06Format\x12\x11\n\rOGG_VORBIS_96\x10\0\x12\x12\n\x0eOGG_VORBIS_\
    160\x10\x01\x12\x12\n\x0eOGG_VORBIS_320\x10\x02\x12\x0b\n\x07MP3_256\x10\
    \x03\x12\x0b\n\x07MP3_320\x10\x04\x12\x0b\n\x07MP3_160\x10\x05\x12\n\n\
    \x06MP3_96\x10\x06\x12\x0f\n\x0bMP3_160_ENC\x10\x07\x12\n\n\x06OTHER2\
    \x10\x08\x12\n\n\x06OTHER3\x10\t\x12\x0b\n\x07AAC_160\x10\n\x12\x0b\n\
    \x07AAC_320\x10\x0b\x12\n\n\x06OTHER4\x10\x0c\x12\n\n\x06OTHER5\x10\rJ\
    \xba:\n\x07\x12\x05\0\0\xa5\x01\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\
    \n\x02\x04\0\x12\x04\x02\0\x05\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\
    \x11\n\x0b\n\x04\x04\0\x02\0\x12\x03\x03\x04\"\n\x0c\n\x05\x04\0\x02\0\
    \x04\x12\x03\x03\x04\x0c\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x03\r\x13\n\
    \x0c\n\x05\x04\0\x02\0\x01\x12\x03\x03\x14\x1b\n\x0c\n\x05\x04\0\x02\0\
    \x03\x12\x03\x03\x1e!\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x04\x04\x1f\n\
    \x0c\n\x05\x04\0\x02\x01\x04\x12\x03\x04\x04\x0c\n\x0c\n\x05\x04\0\x02\
    \x01\x06\x12\x03\x04\r\x12\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x04\x13\
    \x18\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x04\x1b\x1e\n\n\n\x02\x04\x01\
    \x12\x04\x07\0\x0b\x01\n\n\n\x03\x04\x01\x01\x12\x03\x07\x08\x16\n\x0b\n\
    \x04\x04\x01\x02\0\x12\x03\x08\x04%\n\x0c\n\x05\x04\x01\x02\0\x04\x12\
    \x03\x08\x04\x0c\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x08\r\x13\n\x0c\n\
    \x05\x04\x01\x02\0\x01\x12\x03\x08\x14\x1e\n\x0c\n\x05\x04\x01\x02\0\x03\
    \x12\x03\x08!$\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\t\x04#\n\x0c\n\x05\
    \x04\x01\x02\x01\x04\x12\x03\t\x04\x0c\n\x0c\n\x05\x04\x01\x02\x01\x05\
    \x12\x03\t\r\x13\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\t\x14\x1c\n\x0c\
    \n\x05\x04\x01\x02\x01\x03\x12\x03\t\x1f\"\n\x0b\n\x04\x04\x01\x02\x02\
    \x12\x03\n\x04!\n\x0c\n\x05\x04\x01\x02\x02\x04\x12\x03\n\x04\x0c\n\x0c\
    \n\x05\x04\x01\x02\x02\x05\x12\x03\n\r\x13\n\x0c\n\x05\x04\x01\x02\x02\
    \x01\x12\x03\n\x14\x1a\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\n\x1d\x20\
    \n\n\n\x02\x04\x02\x12\x04\r\0\x1f\x01\n\n\n\x03\x04\x02\x01\x12\x03\r\
    \x08\x0e\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x0e\x04\x1d\n\x0c\n\x05\x04\
    \x02\x02\0\x04\x12\x03\x0e\x04\x0c\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\
    \x0e\r\x12\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x0e\x13\x16\n\x0c\n\x05\
    \x04\x02\x02\0\x03\x12\x03\x0e\x19\x1c\n\x0b\n\x04\x04\x02\x02\x01\x12\
    \x03\x0f\x04\x1f\n\x0c\n\x05\x04\x02\x02\x01\x04\x12\x03\x0f\x04\x0c\n\
    \x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x0f\r\x13\n\x0c\n\x05\x04\x02\x02\
    \x01\x01\x12\x03\x0f\x14\x18\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x0f\
    \x1b\x1e\n\x0b\n\x04\x04\x02\x02\x02\x12\x03\x10\x04%\n\x0c\n\x05\x04\
    \x02\x02\x02\x04\x12\x03\x10\x04\x0c\n\x0c\n\x05\x04\x02\x02\x02\x05\x12\
    \x03\x10\r\x13\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03\x10\x14\x1e\n\x0c\
    \n\x05\x04\x02\x02\x02\x03\x12\x03\x10!$\n\x0b\n\x04\x04\x02\x02\x03\x12\
    \x03\x11\x04'\n\x0c\n\x05\x04\x02\x02\x03\x04\x12\x03\x11\x04\x0c\n\x0c\
    \n\x05\x04\x02\x02\x03\x06\x12\x03\x11\r\x16\n\x0c\n\x05\x04\x02\x02\x03\
    \x01\x12\x03\x11\x17\x20\n\x0c\n\x05\x04\x02\x02\x03\x03\x12\x03\x11#&\n\
    \x0b\n\x04\x04\x02\x02\x04\x12\x03\x12\x04*\n\x0c\n\x05\x04\x02\x02\x04\
    \x04\x12\x03\x12\x04\x0c\n\x0c\n\x05\x04\x02\x02\x04\x06\x12\x03\x12\r\
    \x17\n\x0c\n\x05\x04\x02\x02\x04\x01\x12\x03\x12\x18#\n\x0c\n\x05\x04\
    \x02\x02\x04\x03\x12\x03\x12&)\n\x0b\n\x04\x04\x02\x02\x05\x12\x03\x13\
    \x04+\n\x0c\n\x05\x04\x02\x02\x05\x04\x12\x03\x13\x04\x0c\n\x0c\n\x05\
    \x04\x02\x02\x05\x06\x12\x03\x13\r\x17\n\x0c\n\x05\x04\x02\x02\x05\x01\
    \x12\x03\x13\x18$\n\x0c\n\x05\x04\x02\x02\x05\x03\x12\x03\x13'*\n\x0b\n\
    \x04\x04\x02\x02\x06\x12\x03\x14\x040\n\x0c\n\x05\x04\x02\x02\x06\x04\
    \x12\x03\x14\x04\x0c\n\x0c\n\x05\x04\x02\x02\x06\x06\x12\x03\x14\r\x17\n\
    \x0c\n\x05\x04\x02\x02\x06\x01\x12\x03\x14\x18)\n\x0c\n\x05\x04\x02\x02\
    \x06\x03\x12\x03\x14,/\n\x0b\n\x04\x04\x02\x02\x07\x12\x03\x15\x04/\n\
    \x0c\n\x05\x04\x02\x02\x07\x04\x12\x03\x15\x04\x0c\n\x0c\n\x05\x04\x02\
    \x02\x07\x06\x12\x03\x15\r\x17\n\x0c\n\x05\x04\x02\x02\x07\x01\x12\x03\
    \x15\x18(\n\x0c\n\x05\x04\x02\x02\x07\x03\x12\x03\x15+.\n\x0b\n\x04\x04\
    \x02\x02\x08\x12\x03\x16\x04\x20\n\x0c\n\x05\x04\x02\x02\x08\x04\x12\x03\
    \x16\x04\x0c\n\x0c\n\x05\x04\x02\x02\x08\x05\x12\x03\x16\r\x13\n\x0c\n\
    \x05\x04\x02\x02\x08\x01\x12\x03\x16\x14\x19\n\x0c\n\x05\x04\x02\x02\x08\
    \x03\x12\x03\x16\x1c\x1f\n\x0b\n\x04\x04\x02\x02\t\x12\x03\x17\x04*\n\
    \x0c\n\x05\x04\x02\x02\t\x04\x12\x03\x17\x04\x0c\n\x0c\n\x05\x04\x02\x02\
    \t\x06\x12\x03\x17\r\x17\n\x0c\n\x05\x04\x02\x02\t\x01\x12\x03\x17\x18#\
    \n\x0c\n\x05\x04\x02\x02\t\x03\x12\x03\x17&)\n\x0b\n\x04\x04\x02\x02\n\
    \x12\x03\x18\x04\"\n\x0c\n\x05\x04\x02\x02\n\x04\x12\x03\x18\x04\x0c\n\
    \x0c\n\x05\x04\x02\x02\n\x06\x12\x03\x18\r\x12\n\x0c\n\x05\x04\x02\x02\n\
    \x01\x12\x03\x18\x13\x1b\n\x0c\n\x05\x04\x02\x02\n\x03\x12\x03\x18\x1e!\
    \n\x0b\n\x04\x04\x02\x02\x0b\x12\x03\x19\x04'\n\x0c\n\x05\x04\x02\x02\
    \x0b\x04\x12\x03\x19\x04\x0c\n\x0c\n\x05\x04\x02\x02\x0b\x06\x12\x03\x19\
    \r\x16\n\x0c\n\x05\x04\x02\x02\x0b\x01\x12\x03\x19\x17\x20\n\x0c\n\x05\
    \x04\x02\x02\x0b\x03\x12\x03\x19#&\n\x0b\n\x04\x04\x02\x02\x0c\x12\x03\
    \x1a\x042\n\x0c\n\x05\x04\x02\x02\x0c\x04\x12\x03\x1a\x04\x0c\n\x0c\n\
    \x05\x04\x02\x02\x0c\x06\x12\x03\x1a\r\x1b\n\x0c\n\x05\x04\x02\x02\x0c\
    \x01\x12\x03\x1a\x1c+\n\x0c\n\x05\x04\x02\x02\x0c\x03\x12\x03\x1a.1\n\
    \x0b\n\x04\x04\x02\x02\r\x12\x03\x1b\x04+\n\x0c\n\x05\x04\x02\x02\r\x04\
    \x12\x03\x1b\x04\x0c\n\x0c\n\x05\x04\x02\x02\r\x06\x12\x03\x1b\r\x18\n\
    \x0c\n\x05\x04\x02\x02\r\x01\x12\x03\x1b\x19$\n\x0c\n\x05\x04\x02\x02\r\
    \x03\x12\x03\x1b'*\n\x0b\n\x04\x04\x02\x02\x0e\x12\x03\x1c\x04\"\n\x0c\n\
    \x05\x04\x02\x02\x0e\x04\x12\x03\x1c\x04\x0c\n\x0c\n\x05\x04\x02\x02\x0e\
    \x06\x12\x03\x1c\r\x13\n\x0c\n\x05\x04\x02\x02\x0e\x01\x12\x03\x1c\x14\
    \x1b\n\x0c\n\x05\x04\x02\x02\x0e\x03\x12\x03\x1c\x1e!\n\x0b\n\x04\x04\
    \x02\x02\x0f\x12\x03\x1d\x041\n\x0c\n\x05\x04\x02\x02\x0f\x04\x12\x03\
    \x1d\x04\x0c\n\x0c\n\x05\x04\x02\x02\x0f\x05\x12\x03\x1d\r\x11\n\x0c\n\
    \x05\x04\x02\x02\x0f\x01\x12\x03\x1d\x12)\n\x0c\n\x05\x04\x02\x02\x0f\
    \x03\x12\x03\x1d,0\n\x0b\n\x04\x04\x02\x02\x10\x12\x03\x1e\x04.\n\x0c\n\
    \x05\x04\x02\x02\x10\x04\x12\x03\x1e\x04\x0c\n\x0c\n\x05\x04\x02\x02\x10\
    \x06\x12\x03\x1e\r\x17\n\x0c\n\x05\x04\x02\x02\x10\x01\x12\x03\x1e\x18&\
    \n\x0c\n\x05\x04\x02\x02\x10\x03\x12\x03\x1e)-\n\n\n\x02\x04\x03\x12\x04\
    !\0#\x01\n\n\n\x03\x04\x03\x01\x12\x03!\x08\x12\n\x0b\n\x04\x04\x03\x02\
    \0\x12\x03\"\x04\x1f\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03\"\x04\x0c\n\
    \x0c\n\x05\x04\x03\x02\0\x06\x12\x03\"\r\x12\n\x0c\n\x05\x04\x03\x02\0\
    \x01\x12\x03\"\x13\x18\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\"\x1b\x1e\n\
    \n\n\x02\x04\x04\x12\x04%\0)\x01\n\n\n\x03\x04\x04\x01\x12\x03%\x08\x0c\
    \n\x0b\n\x04\x04\x04\x02\0\x12\x03&\x04\x1f\n\x0c\n\x05\x04\x04\x02\0\
    \x04\x12\x03&\x04\x0c\n\x0c\n\x05\x04\x04\x02\0\x05\x12\x03&\r\x13\n\x0c\
    \n\x05\x04\x04\x02\0\x01\x12\x03&\x14\x18\n\x0c\n\x05\x04\x04\x02\0\x03\
    \x12\x03&\x1b\x1e\n\x0b\n\x04\x04\x04\x02\x01\x12\x03'\x04\x20\n\x0c\n\
    \x05\x04\x04\x02\x01\x04\x12\x03'\x04\x0c\n\x0c\n\x05\x04\x04\x02\x01\
    \x05\x12\x03'\r\x13\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03'\x14\x19\n\
    \x0c\n\x05\x04\x04\x02\x01\x03\x12\x03'\x1c\x1f\n\x0b\n\x04\x04\x04\x02\
    \x02\x12\x03(\x04\x1e\n\x0c\n\x05\x04\x04\x02\x02\x04\x12\x03(\x04\x0c\n\
    \x0c\n\x05\x04\x04\x02\x02\x05\x12\x03(\r\x13\n\x0c\n\x05\x04\x04\x02\
    \x02\x01\x12\x03(\x14\x17\n\x0c\n\x05\x04\x04\x02\x02\x03\x12\x03(\x1a\
    \x1d\n\n\n\x02\x04\x05\x12\x04+\0C\x01\n\n\n\x03\x04\x05\x01\x12\x03+\
    \x08\r\n\x0b\n\x04\x04\x05\x02\0\x12\x03,\x04\x1d\n\x0c\n\x05\x04\x05\
    \x02\0\x04\x12\x03,\x04\x0c\n\x0c\n\x05\x04\x05\x02\0\x05\x12\x03,\r\x12\
    \n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03,\x13\x16\n\x0c\n\x05\x04\x05\x02\
    \0\x03\x12\x03,\x19\x1c\n\x0b\n\x04\x04\x05\x02\x01\x12\x03-\x04\x1f\n\
    \x0c\n\x05\x04\x05\x02\x01\x04\x12\x03-\x04\x0c\n\x0c\n\x05\x04\x05\x02\
    \x01\x05\x12\x03-\r\x13\n\x0c\n\x05\x04\x05\x02\x01\x01\x12\x03-\x14\x18\
    \n\x0c\n\x05\x04\x05\x02\x01\x03\x12\x03-\x1b\x1e\n\x0b\n\x04\x04\x05\
    \x02\x02\x12\x03.\x04!\n\x0c\n\x05\x04\x05\x02\x02\x04\x12\x03.\x04\x0c\
    \n\x0c\n\x05\x04\x05\x02\x02\x06\x12\x03.\r\x13\n\x0c\n\x05\x04\x05\x02\
    \x02\x01\x12\x03.\x14\x1a\n\x0c\n\x05\x04\x05\x02\x02\x03\x12\x03.\x1d\
    \x20\n\x0b\n\x04\x04\x05\x02\x03\x12\x03/\x04\x1c\n\x0c\n\x05\x04\x05\
    \x02\x03\x04\x12\x03/\x04\x0c\n\x0c\n\x05\x04\x05\x02\x03\x06\x12\x03/\r\
    \x11\n\x0c\n\x05\x04\x05\x02\x03\x01\x12\x03/\x12\x15\n\x0c\n\x05\x04\
    \x05\x02\x03\x03\x12\x03/\x18\x1b\n\x0c\n\x04\x04\x05\x04\0\x12\x040\x04\
    5\x05\n\x0c\n\x05\x04\x05\x04\0\x01\x12\x030\t\r\n\r\n\x06\x04\x05\x04\0\
    \x02\0\x12\x031\x08\x14\n\x0e\n\x07\x04\x05\x04\0\x02\0\x01\x12\x031\x08\
    \r\n\x0e\n\x07\x04\x05\x04\0\x02\0\x02\x12\x031\x10\x13\n\r\n\x06\x04\
    \x05\x04\0\x02\x01\x12\x032\x08\x15\n\x0e\n\x07\x04\x05\x04\0\x02\x01\
    \x01\x12\x032\x08\x0e\n\x0e\n\x07\x04\x05\x04\0\x02\x01\x02\x12\x032\x11\
    \x14\n\r\n\x06\x04\x05\x04\0\x02\x02\x12\x033\x08\x1a\n\x0e\n\x07\x04\
    \x05\x04\0\x02\x02\x01\x12\x033\x08\x13\n\x0e\n\x07\x04\x05\x04\0\x02\
    \x02\x02\x12\x033\x16\x19\n\r\n\x06\x04\x05\x04\0\x02\x03\x12\x034\x08\
    \x11\n\x0e\n\x07\x04\x05\x04\0\x02\x03\x01\x12\x034\x08\n\n\x0e\n\x07\
    \x04\x05\x04\0\x02\x03\x02\x12\x034\r\x10\n\x0b\n\x04\x04\x05\x02\x04\
    \x12\x036\x04\x20\n\x0c\n\x05\x04\x05\x02\x04\x04\x12\x036\x04\x0c\n\x0c\
    \n\x05\x04\x05\x02\x04\x05\x12\x036\r\x13\n\x0c\n\x05\x04\x05\x02\x04\
    \x01\x12\x036\x14\x19\n\x0c\n\x05\x04\x05\x02\x04\x03\x12\x036\x1c\x1f\n\
    \x0b\n\x04\x04\x05\x02\x05\x12\x037\x04\x1d\n\x0c\n\x05\x04\x05\x02\x05\
    \x04\x12\x037\x04\x0c\n\x0c\n\x05\x04\x05\x02\x05\x06\x12\x037\r\x11\n\
    \x0c\n\x05\x04\x05\x02\x05\x01\x12\x037\x12\x16\n\x0c\n\x05\x04\x05\x02\
    \x05\x03\x12\x037\x19\x1c\n\x0b\n\x04\x04\x05\x02\x06\x12\x038\x04%\n\
    \x0c\n\x05\x04\x05\x02\x06\x04\x12\x038\x04\x0c\n\x0c\n\x05\x04\x05\x02\
    \x06\x05\x12\x038\r\x13\n\x0c\n\x05\x04\x05\x02\x06\x01\x12\x038\x14\x1e\
    \n\x0c\n\x05\x04\x05\x02\x06\x03\x12\x038!$\n\x0b\n\x04\x04\x05\x02\x07\
    \x12\x039\x04\x20\n\x0c\n\x05\x04\x05\x02\x07\x04\x12\x039\x04\x0c\n\x0c\
    \n\x05\x04\x05\x02\x07\x05\x12\x039\r\x13\n\x0c\n\x05\x04\x05\x02\x07\
    \x01\x12\x039\x14\x19\n\x0c\n\x05\x04\x05\x02\x07\x03\x12\x039\x1c\x1f\n\
    \x0b\n\x04\x04\x05\x02\x08\x12\x03:\x04\x1f\n\x0c\n\x05\x04\x05\x02\x08\
    \x04\x12\x03:\x04\x0c\n\x0c\n\x05\x04\x05\x02\x08\x06\x12\x03:\r\x12\n\
    \x0c\n\x05\x04\x05\x02\x08\x01\x12\x03:\x13\x18\n\x0c\n\x05\x04\x05\x02\
    \x08\x03\x12\x03:\x1b\x1e\n\x0b\n\x04\x04\x05\x02\t\x12\x03;\x04*\n\x0c\
    \n\x05\x04\x05\x02\t\x04\x12\x03;\x04\x0c\n\x0c\n\x05\x04\x05\x02\t\x06\
    \x12\x03;\r\x17\n\x0c\n\x05\x04\x05\x02\t\x01\x12\x03;\x18#\n\x0c\n\x05\
    \x04\x05\x02\t\x03\x12\x03;&)\n\x0b\n\x04\x04\x05\x02\n\x12\x03<\x04\x1d\
    \n\x0c\n\x05\x04\x05\x02\n\x04\x12\x03<\x04\x0c\n\x0c\n\x05\x04\x05\x02\
    \n\x06\x12\x03<\r\x11\n\x0c\n\x05\x04\x05\x02\n\x01\x12\x03<\x12\x16\n\
    \x0c\n\x05\x04\x05\x02\n\x03\x12\x03<\x19\x1c\n\x0b\n\x04\x04\x05\x02\
    \x0b\x12\x03=\x04!\n\x0c\n\x05\x04\x05\x02\x0b\x04\x12\x03=\x04\x0c\n\
    \x0c\n\x05\x04\x05\x02\x0b\x05\x12\x03=\r\x13\n\x0c\n\x05\x04\x05\x02\
    \x0b\x01\x12\x03=\x14\x1a\n\x0c\n\x05\x04\x05\x02\x0b\x03\x12\x03=\x1d\
    \x20\n\x0b\n\x04\x04\x05\x02\x0c\x12\x03>\x04'\n\x0c\n\x05\x04\x05\x02\
    \x0c\x04\x12\x03>\x04\x0c\n\x0c\n\x05\x04\x05\x02\x0c\x06\x12\x03>\r\x16\
    \n\x0c\n\x05\x04\x05\x02\x0c\x01\x12\x03>\x17\x20\n\x0c\n\x05\x04\x05\
    \x02\x0c\x03\x12\x03>#&\n\x0b\n\x04\x04\x05\x02\r\x12\x03?\x04+\n\x0c\n\
    \x05\x04\x05\x02\r\x04\x12\x03?\x04\x0c\n\x0c\n\x05\x04\x05\x02\r\x06\
    \x12\x03?\r\x18\n\x0c\n\x05\x04\x05\x02\r\x01\x12\x03?\x19$\n\x0c\n\x05\
    \x04\x05\x02\r\x03\x12\x03?'*\n\x0b\n\x04\x04\x05\x02\x0e\x12\x03@\x04!\
    \n\x0c\n\x05\x04\x05\x02\x0e\x04\x12\x03@\x04\x0c\n\x0c\n\x05\x04\x05\
    \x02\x0e\x06\x12\x03@\r\x12\n\x0c\n\x05\x04\x05\x02\x0e\x01\x12\x03@\x13\
    \x1a\n\x0c\n\x05\x04\x05\x02\x0e\x03\x12\x03@\x1d\x20\n\x0b\n\x04\x04\
    \x05\x02\x0f\x12\x03A\x04+\n\x0c\n\x05\x04\x05\x02\x0f\x04\x12\x03A\x04\
    \x0c\n\x0c\n\x05\x04\x05\x02\x0f\x06\x12\x03A\r\x17\n\x0c\n\x05\x04\x05\
    \x02\x0f\x01\x12\x03A\x18#\n\x0c\n\x05\x04\x05\x02\x0f\x03\x12\x03A&*\n\
    \x0b\n\x04\x04\x05\x02\x10\x12\x03B\x04+\n\x0c\n\x05\x04\x05\x02\x10\x04\
    \x12\x03B\x04\x0c\n\x0c\n\x05\x04\x05\x02\x10\x06\x12\x03B\r\x17\n\x0c\n\
    \x05\x04\x05\x02\x10\x01\x12\x03B\x18#\n\x0c\n\x05\x04\x05\x02\x10\x03\
    \x12\x03B&*\n\n\n\x02\x04\x06\x12\x04E\0U\x01\n\n\n\x03\x04\x06\x01\x12\
    \x03E\x08\r\n\x0b\n\x04\x04\x06\x02\0\x12\x03F\x04\x1d\n\x0c\n\x05\x04\
    \x06\x02\0\x04\x12\x03F\x04\x0c\n\x0c\n\x05\x04\x06\x02\0\x05\x12\x03F\r\
    \x12\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x03F\x13\x16\n\x0c\n\x05\x04\x06\
    \x02\0\x03\x12\x03F\x19\x1c\n\x0b\n\x04\x04\x06\x02\x01\x12\x03G\x04\x1f\
    \n\x0c\n\x05\x04\x06\x02\x01\x04\x12\x03G\x04\x0c\n\x0c\n\x05\x04\x06\
    \x02\x01\x05\x12\x03G\r\x13\n\x0c\n\x05\x04\x06\x02\x01\x01\x12\x03G\x14\
    \x18\n\x0c\n\x05\x04\x06\x02\x01\x03\x12\x03G\x1b\x1e\n\x0b\n\x04\x04\
    \x06\x02\x02\x12\x03H\x04\x1f\n\x0c\n\x05\x04\x06\x02\x02\x04\x12\x03H\
    \x04\x0c\n\x0c\n\x05\x04\x06\x02\x02\x06\x12\x03H\r\x12\n\x0c\n\x05\x04\
    \x06\x02\x02\x01\x12\x03H\x13\x18\n\x0c\n\x05\x04\x06\x02\x02\x03\x12\
    \x03H\x1b\x1e\n\x0b\n\x04\x04\x06\x02\x03\x12\x03I\x04!\n\x0c\n\x05\x04\
    \x06\x02\x03\x04\x12\x03I\x04\x0c\n\x0c\n\x05\x04\x06\x02\x03\x06\x12\
    \x03I\r\x13\n\x0c\n\x05\x04\x06\x02\x03\x01\x12\x03I\x14\x1a\n\x0c\n\x05\
    \x04\x06\x02\x03\x03\x12\x03I\x1d\x20\n\x0b\n\x04\x04\x06\x02\x04\x12\
    \x03J\x04!\n\x0c\n\x05\x04\x06\x02\x04\x04\x12\x03J\x04\x0c\n\x0c\n\x05\
    \x04\x06\x02\x04\x05\x12\x03J\r\x13\n\x0c\n\x05\x04\x06\x02\x04\x01\x12\
    \x03J\x14\x1a\n\x0c\n\x05\x04\x06\x02\x04\x03\x12\x03J\x1d\x20\n\x0b\n\
    \x04\x04\x06\x02\x05\x12\x03K\x04&\n\x0c\n\x05\x04\x06\x02\x05\x04\x12\
    \x03K\x04\x0c\n\x0c\n\x05\x04\x06\x02\x05\x05\x12\x03K\r\x13\n\x0c\n\x05\
    \x04\x06\x02\x05\x01\x12\x03K\x14\x1f\n\x0c\n\x05\x04\x06\x02\x05\x03\
    \x12\x03K\"%\n\x0b\n\x04\x04\x06\x02\x06\x12\x03L\x04#\n\x0c\n\x05\x04\
    \x06\x02\x06\x04\x12\x03L\x04\x0c\n\x0c\n\x05\x04\x06\x02\x06\x05\x12\
    \x03L\r\x13\n\x0c\n\x05\x04\x06\x02\x06\x01\x12\x03L\x14\x1c\n\x0c\n\x05\
    \x04\x06\x02\x06\x03\x12\x03L\x1f\"\n\x0b\n\x04\x04\x06\x02\x07\x12\x03M\
    \x04%\n\x0c\n\x05\x04\x06\x02\x07\x04\x12\x03M\x04\x0c\n\x0c\n\x05\x04\
    \x06\x02\x07\x05\x12\x03M\r\x13\n\x0c\n\x05\x04\x06\x02\x07\x01\x12\x03M\
    \x14\x1e\n\x0c\n\x05\x04\x06\x02\x07\x03\x12\x03M!$\n\x0b\n\x04\x04\x06\
    \x02\x08\x12\x03N\x04!\n\x0c\n\x05\x04\x06\x02\x08\x04\x12\x03N\x04\x0c\
    \n\x0c\n\x05\x04\x06\x02\x08\x05\x12\x03N\r\x11\n\x0c\n\x05\x04\x06\x02\
    \x08\x01\x12\x03N\x12\x1a\n\x0c\n\x05\x04\x06\x02\x08\x03\x12\x03N\x1d\
    \x20\n\x0b\n\x04\x04\x06\x02\t\x12\x03O\x04*\n\x0c\n\x05\x04\x06\x02\t\
    \x04\x12\x03O\x04\x0c\n\x0c\n\x05\x04\x06\x02\t\x06\x12\x03O\r\x17\n\x0c\
    \n\x05\x04\x06\x02\t\x01\x12\x03O\x18#\n\x0c\n\x05\x04\x06\x02\t\x03\x12\
    \x03O&)\n\x0b\n\x04\x04\x06\x02\n\x12\x03P\x04+\n\x0c\n\x05\x04\x06\x02\
    \n\x04\x12\x03P\x04\x0c\n\x0c\n\x05\x04\x06\x02\n\x06\x12\x03P\r\x18\n\
    \x0c\n\x05\x04\x06\x02\n\x01\x12\x03P\x19$\n\x0c\n\x05\x04\x06\x02\n\x03\
    \x12\x03P'*\n\x0b\n\x04\x04\x06\x02\x0b\x12\x03Q\x04\"\n\x0c\n\x05\x04\
    \x06\x02\x0b\x04\x12\x03Q\x04\x0c\n\x0c\n\x05\x04\x06\x02\x0b\x06\x12\
    \x03Q\r\x16\n\x0c\n\x05\x04\x06\x02\x0b\x01\x12\x03Q\x17\x1b\n\x0c\n\x05\
    \x04\x06\x02\x0b\x03\x12\x03Q\x1e!\n\x0b\n\x04\x04\x06\x02\x0c\x12\x03R\
    \x04%\n\x0c\n\x05\x04\x06\x02\x0c\x04\x12\x03R\x04\x0c\n\x0c\n\x05\x04\
    \x06\x02\x0c\x06\x12\x03R\r\x12\n\x0c\n\x05\x04\x06\x02\x0c\x01\x12\x03R\
    \x13\x1e\n\x0c\n\x05\x04\x06\x02\x0c\x03\x12\x03R!$\n\x0b\n\x04\x04\x06\
    \x02\r\x12\x03S\x04*\n\x0c\n\x05\x04\x06\x02\r\x04\x12\x03S\x04\x0c\n\
    \x0c\n\x05\x04\x06\x02\r\x06\x12\x03S\r\x17\n\x0c\n\x05\x04\x06\x02\r\
    \x01\x12\x03S\x18#\n\x0c\n\x05\x04\x06\x02\r\x03\x12\x03S&)\n\x0b\n\x04\
    \x04\x06\x02\x0e\x12\x03T\x04%\n\x0c\n\x05\x04\x06\x02\x0e\x04\x12\x03T\
    \x04\x0c\n\x0c\n\x05\x04\x06\x02\x0e\x06\x12\x03T\r\x16\n\x0c\n\x05\x04\
    \x06\x02\x0e\x01\x12\x03T\x17\x1e\n\x0c\n\x05\x04\x06\x02\x0e\x03\x12\
    \x03T!$\n\n\n\x02\x04\x07\x12\x04W\0b\x01\n\n\n\x03\x04\x07\x01\x12\x03W\
    \x08\r\n\x0b\n\x04\x04\x07\x02\0\x12\x03X\x04!\n\x0c\n\x05\x04\x07\x02\0\
    \x04\x12\x03X\x04\x0c\n\x0c\n\x05\x04\x07\x02\0\x05\x12\x03X\r\x12\n\x0c\
    \n\x05\x04\x07\x02\0\x01\x12\x03X\x13\x1a\n\x0c\n\x05\x04\x07\x02\0\x03\
    \x12\x03X\x1d\x20\n\x0b\n\x04\x04\x07\x02\x01\x12\x03Y\x04\x1d\n\x0c\n\
    \x05\x04\x07\x02\x01\x04\x12\x03Y\x04\x0c\n\x0c\n\x05\x04\x07\x02\x01\
    \x06\x12\x03Y\r\x11\n\x0c\n\x05\x04\x07\x02\x01\x01\x12\x03Y\x12\x16\n\
    \x0c\n\x05\x04\x07\x02\x01\x03\x12\x03Y\x19\x1c\n\x0c\n\x04\x04\x07\x04\
    \0\x12\x04Z\x04_\x05\n\x0c\n\x05\x04\x07\x04\0\x01\x12\x03Z\t\r\n\r\n\
    \x06\x04\x07\x04\0\x02\0\x12\x03[\x08\x16\n\x0e\n\x07\x04\x07\x04\0\x02\
    \0\x01\x12\x03[\x08\x0f\n\x0e\n\x07\x04\x07\x04\0\x02\0\x02\x12\x03[\x12\
    \x15\n\r\n\x06\x04\x07\x04\0\x02\x01\x12\x03\\\x08\x14\n\x0e\n\x07\x04\
    \x07\x04\0\x02\x01\x01\x12\x03\\\x08\r\n\x0e\n\x07\x04\x07\x04\0\x02\x01\
    \x02\x12\x03\\\x10\x13\n\r\n\x06\x04\x07\x04\0\x02\x02\x12\x03]\x08\x14\
    \n\x0e\n\x07\x04\x07\x04\0\x02\x02\x01\x12\x03]\x08\r\n\x0e\n\x07\x04\
    \x07\x04\0\x02\x02\x02\x12\x03]\x10\x13\n\r\n\x06\x04\x07\x04\0\x02\x03\
    \x12\x03^\x08\x15\n\x0e\n\x07\x04\x07\x04\0\x02\x03\x01\x12\x03^\x08\x0e\
    \n\x0e\n\x07\x04\x07\x04\0\x02\x03\x02\x12\x03^\x11\x14\n\x0b\n\x04\x04\
    \x07\x02\x02\x12\x03`\x04\x20\n\x0c\n\x05\x04\x07\x02\x02\x04\x12\x03`\
    \x04\x0c\n\x0c\n\x05\x04\x07\x02\x02\x05\x12\x03`\r\x13\n\x0c\n\x05\x04\
    \x07\x02\x02\x01\x12\x03`\x14\x19\n\x0c\n\x05\x04\x07\x02\x02\x03\x12\
    \x03`\x1c\x1f\n\x0b\n\x04\x04\x07\x02\x03\x12\x03a\x04!\n\x0c\n\x05\x04\
    \x07\x02\x03\x04\x12\x03a\x04\x0c\n\x0c\n\x05\x04\x07\x02\x03\x05\x12\
    \x03a\r\x13\n\x0c\n\x05\x04\x07\x02\x03\x01\x12\x03a\x14\x1a\n\x0c\n\x05\
    \x04\x07\x02\x03\x03\x12\x03a\x1d\x20\n\n\n\x02\x04\x08\x12\x04d\0f\x01\
    \n\n\n\x03\x04\x08\x01\x12\x03d\x08\x12\n\x0b\n\x04\x04\x08\x02\0\x12\
    \x03e\x04\x1f\n\x0c\n\x05\x04\x08\x02\0\x04\x12\x03e\x04\x0c\n\x0c\n\x05\
    \x04\x08\x02\0\x06\x12\x03e\r\x12\n\x0c\n\x05\x04\x08\x02\0\x01\x12\x03e\
    \x13\x18\n\x0c\n\x05\x04\x08\x02\0\x03\x12\x03e\x1b\x1e\n\n\n\x02\x04\t\
    \x12\x04h\0l\x01\n\n\n\x03\x04\t\x01\x12\x03h\x08\x11\n\x0b\n\x04\x04\t\
    \x02\0\x12\x03i\x04\x1f\n\x0c\n\x05\x04\t\x02\0\x04\x12\x03i\x04\x0c\n\
    \x0c\n\x05\x04\t\x02\0\x05\x12\x03i\r\x13\n\x0c\n\x05\x04\t\x02\0\x01\
    \x12\x03i\x14\x18\n\x0c\n\x05\x04\t\x02\0\x03\x12\x03i\x1b\x1e\n\x0b\n\
    \x04\x04\t\x02\x01\x12\x03j\x04\"\n\x0c\n\x05\x04\t\x02\x01\x04\x12\x03j\
    \x04\x0c\n\x0c\n\x05\x04\t\x02\x01\x06\x12\x03j\r\x12\n\x0c\n\x05\x04\t\
    \x02\x01\x01\x12\x03j\x13\x1b\n\x0c\n\x05\x04\t\x02\x01\x03\x12\x03j\x1e\
    !\n\x0b\n\x04\x04\t\x02\x02\x12\x03k\x04-\n\x0c\n\x05\x04\t\x02\x02\x04\
    \x12\x03k\x04\x0c\n\x0c\n\x05\x04\t\x02\x02\x06\x12\x03k\r\x17\n\x0c\n\
    \x05\x04\t\x02\x02\x01\x12\x03k\x18&\n\x0c\n\x05\x04\t\x02\x02\x03\x12\
    \x03k),\n\n\n\x02\x04\n\x12\x04n\0r\x01\n\n\n\x03\x04\n\x01\x12\x03n\x08\
    \x0c\n\x0b\n\x04\x04\n\x02\0\x12\x03o\x04!\n\x0c\n\x05\x04\n\x02\0\x04\
    \x12\x03o\x04\x0c\n\x0c\n\x05\x04\n\x02\0\x05\x12\x03o\r\x13\n\x0c\n\x05\
    \x04\n\x02\0\x01\x12\x03o\x14\x1a\n\x0c\n\x05\x04\n\x02\0\x03\x12\x03o\
    \x1d\x20\n\x0b\n\x04\x04\n\x02\x01\x12\x03p\x04\x1f\n\x0c\n\x05\x04\n\
    \x02\x01\x04\x12\x03p\x04\x0c\n\x0c\n\x05\x04\n\x02\x01\x05\x12\x03p\r\
    \x13\n\x0c\n\x05\x04\n\x02\x01\x01\x12\x03p\x14\x18\n\x0c\n\x05\x04\n\
    \x02\x01\x03\x12\x03p\x1b\x1e\n\x0b\n\x04\x04\n\x02\x02\x12\x03q\x04\x1f\
    \n\x0c\n\x05\x04\n\x02\x02\x04\x12\x03q\x04\x0c\n\x0c\n\x05\x04\n\x02\
    \x02\x06\x12\x03q\r\x12\n\x0c\n\x05\x04\n\x02\x02\x01\x12\x03q\x13\x18\n\
    \x0c\n\x05\x04\n\x02\x02\x03\x12\x03q\x1b\x1e\n\n\n\x02\x04\x0b\x12\x04t\
    \0{\x01\n\n\n\x03\x04\x0b\x01\x12\x03t\x08\x11\n\x0b\n\x04\x04\x0b\x02\0\
    \x12\x03u\x04\x1c\n\x0c\n\x05\x04\x0b\x02\0\x04\x12\x03u\x04\x0c\n\x0c\n\
    \x05\x04\x0b\x02\0\x06\x12\x03u\r\x11\n\x0c\n\x05\x04\x0b\x02\0\x01\x12\
    \x03u\x12\x15\n\x0c\n\x05\x04\x0b\x02\0\x03\x12\x03u\x18\x1b\n\x0c\n\x04\
    \x04\x0b\x04\0\x12\x04v\x04y\x05\n\x0c\n\x05\x04\x0b\x04\0\x01\x12\x03v\
    \t\r\n\r\n\x06\x04\x0b\x04\0\x02\0\x12\x03w\x08\x10\n\x0e\n\x07\x04\x0b\
    \x04\0\x02\0\x01\x12\x03w\x08\t\n\x0e\n\x07\x04\x0b\x04\0\x02\0\x02\x12\
    \x03w\x0c\x0f\n\r\n\x06\x04\x0b\x04\0\x02\x01\x12\x03x\x08\x10\n\x0e\n\
    \x07\x04\x0b\x04\0\x02\x01\x01\x12\x03x\x08\t\n\x0e\n\x07\x04\x0b\x04\0\
    \x02\x01\x02\x12\x03x\x0c\x0f\n\x0b\n\x04\x04\x0b\x02\x01\x12\x03z\x04\
    \x1f\n\x0c\n\x05\x04\x0b\x02\x01\x04\x12\x03z\x04\x0c\n\x0c\n\x05\x04\
    \x0b\x02\x01\x05\x12\x03z\r\x13\n\x0c\n\x05\x04\x0b\x02\x01\x01\x12\x03z\
    \x14\x18\n\x0c\n\x05\x04\x0b\x02\x01\x03\x12\x03z\x1b\x1e\n\x0b\n\x02\
    \x04\x0c\x12\x05}\0\x85\x01\x01\n\n\n\x03\x04\x0c\x01\x12\x03}\x08\x13\n\
    \x0b\n\x04\x04\x0c\x02\0\x12\x03~\x04,\n\x0c\n\x05\x04\x0c\x02\0\x04\x12\
    \x03~\x04\x0c\n\x0c\n\x05\x04\x0c\x02\0\x05\x12\x03~\r\x13\n\x0c\n\x05\
    \x04\x0c\x02\0\x01\x12\x03~\x14%\n\x0c\n\x05\x04\x0c\x02\0\x03\x12\x03~(\
    +\n\x0b\n\x04\x04\x0c\x02\x01\x12\x03\x7f\x04.\n\x0c\n\x05\x04\x0c\x02\
    \x01\x04\x12\x03\x7f\x04\x0c\n\x0c\n\x05\x04\x0c\x02\x01\x05\x12\x03\x7f\
    \r\x13\n\x0c\n\x05\x04\x0c\x02\x01\x01\x12\x03\x7f\x14'\n\x0c\n\x05\x04\
    \x0c\x02\x01\x03\x12\x03\x7f*-\n\x0c\n\x04\x04\x0c\x02\x02\x12\x04\x80\
    \x01\x04\x1c\n\r\n\x05\x04\x0c\x02\x02\x04\x12\x04\x80\x01\x04\x0c\n\r\n\
    \x05\x04\x0c\x02\x02\x06\x12\x04\x80\x01\r\x11\n\r\n\x05\x04\x0c\x02\x02\
    \x01\x12\x04\x80\x01\x12\x15\n\r\n\x05\x04\x0c\x02\x02\x03\x12\x04\x80\
    \x01\x18\x1b\n\x0e\n\x04\x04\x0c\x04\0\x12\x06\x81\x01\x04\x83\x01\x05\n\
    \r\n\x05\x04\x0c\x04\0\x01\x12\x04\x81\x01\t\r\n\x0e\n\x06\x04\x0c\x04\0\
    \x02\0\x12\x04\x82\x01\x08\x18\n\x0f\n\x07\x04\x0c\x04\0\x02\0\x01\x12\
    \x04\x82\x01\x08\x11\n\x0f\n\x07\x04\x0c\x04\0\x02\0\x02\x12\x04\x82\x01\
    \x14\x17\n\x0c\n\x04\x04\x0c\x02\x03\x12\x04\x84\x01\x04(\n\r\n\x05\x04\
    \x0c\x02\x03\x04\x12\x04\x84\x01\x04\x0c\n\r\n\x05\x04\x0c\x02\x03\x05\
    \x12\x04\x84\x01\r\x13\n\r\n\x05\x04\x0c\x02\x03\x01\x12\x04\x84\x01\x14\
    !\n\r\n\x05\x04\x0c\x02\x03\x03\x12\x04\x84\x01$'\n\x0c\n\x02\x04\r\x12\
    \x06\x87\x01\0\x8b\x01\x01\n\x0b\n\x03\x04\r\x01\x12\x04\x87\x01\x08\x12\
    \n\x0c\n\x04\x04\r\x02\0\x12\x04\x88\x01\x04+\n\r\n\x05\x04\r\x02\0\x04\
    \x12\x04\x88\x01\x04\x0c\n\r\n\x05\x04\r\x02\0\x06\x12\x04\x88\x01\r\x18\
    \n\r\n\x05\x04\r\x02\0\x01\x12\x04\x88\x01\x19$\n\r\n\x05\x04\r\x02\0\
    \x03\x12\x04\x88\x01'*\n\x0c\n\x04\x04\r\x02\x01\x12\x04\x89\x01\x04\x1e\
    \n\r\n\x05\x04\r\x02\x01\x04\x12\x04\x89\x01\x04\x0c\n\r\n\x05\x04\r\x02\
    \x01\x06\x12\x04\x89\x01\r\x11\n\r\n\x05\x04\r\x02\x01\x01\x12\x04\x89\
    \x01\x12\x17\n\r\n\x05\x04\r\x02\x01\x03\x12\x04\x89\x01\x1a\x1d\n\x0c\n\
    \x04\x04\r\x02\x02\x12\x04\x8a\x01\x04\x1c\n\r\n\x05\x04\r\x02\x02\x04\
    \x12\x04\x8a\x01\x04\x0c\n\r\n\x05\x04\r\x02\x02\x06\x12\x04\x8a\x01\r\
    \x11\n\r\n\x05\x04\r\x02\x02\x01\x12\x04\x8a\x01\x12\x15\n\r\n\x05\x04\r\
    \x02\x02\x03\x12\x04\x8a\x01\x18\x1b\n\x0c\n\x02\x04\x0e\x12\x06\x8d\x01\
    \0\x90\x01\x01\n\x0b\n\x03\x04\x0e\x01\x12\x04\x8d\x01\x08\x12\n\x0c\n\
    \x04\x04\x0e\x02\0\x12\x04\x8e\x01\x04\x1e\n\r\n\x05\x04\x0e\x02\0\x04\
    \x12\x04\x8e\x01\x04\x0c\n\r\n\x05\x04\x0e\x02\0\x05\x12\x04\x8e\x01\r\
    \x13\n\r\n\x05\x04\x0e\x02\0\x01\x12\x04\x8e\x01\x14\x17\n\r\n\x05\x04\
    \x0e\x02\0\x03\x12\x04\x8e\x01\x1a\x1d\n\x0c\n\x04\x04\x0e\x02\x01\x12\
    \x04\x8f\x01\x04\x1d\n\r\n\x05\x04\x0e\x02\x01\x04\x12\x04\x8f\x01\x04\
    \x0c\n\r\n\x05\x04\x0e\x02\x01\x05\x12\x04\x8f\x01\r\x13\n\r\n\x05\x04\
    \x0e\x02\x01\x01\x12\x04\x8f\x01\x14\x16\n\r\n\x05\x04\x0e\x02\x01\x03\
    \x12\x04\x8f\x01\x19\x1c\n\x0c\n\x02\x04\x0f\x12\x06\x92\x01\0\xa5\x01\
    \x01\n\x0b\n\x03\x04\x0f\x01\x12\x04\x92\x01\x08\x11\n\x0c\n\x04\x04\x0f\
    \x02\0\x12\x04\x93\x01\x04!\n\r\n\x05\x04\x0f\x02\0\x04\x12\x04\x93\x01\
    \x04\x0c\n\r\n\x05\x04\x0f\x02\0\x05\x12\x04\x93\x01\r\x12\n\r\n\x05\x04\
    \x0f\x02\0\x01\x12\x04\x93\x01\x13\x1a\n\r\n\x05\x04\x0f\x02\0\x03\x12\
    \x04\x93\x01\x1d\x20\n\x0c\n\x04\x04\x0f\x02\x01\x12\x04\x94\x01\x04!\n\
    \r\n\x05\x04\x0f\x02\x01\x04\x12\x04\x94\x01\x04\x0c\n\r\n\x05\x04\x0f\
    \x02\x01\x06\x12\x04\x94\x01\r\x13\n\r\n\x05\x04\x0f\x02\x01\x01\x12\x04\
    \x94\x01\x14\x1a\n\r\n\x05\x04\x0f\x02\x01\x03\x12\x04\x94\x01\x1d\x20\n\
    \x0e\n\x04\x04\x0f\x04\0\x12\x06\x95\x01\x04\xa4\x01\x05\n\r\n\x05\x04\
    \x0f\x04\0\x01\x12\x04\x95\x01\t\x0f\n\x0e\n\x06\x04\x0f\x04\0\x02\0\x12\
    \x04\x96\x01\x08\x1c\n\x0f\n\x07\x04\x0f\x04\0\x02\0\x01\x12\x04\x96\x01\
    \x08\x15\n\x0f\n\x07\x04\x0f\x04\0\x02\0\x02\x12\x04\x96\x01\x18\x1b\n\
    \x0e\n\x06\x04\x0f\x04\0\x02\x01\x12\x04\x97\x01\x08\x1d\n\x0f\n\x07\x04\
    \x0f\x04\0\x02\x01\x01\x12\x04\x97\x01\x08\x16\n\x0f\n\x07\x04\x0f\x04\0\
    \x02\x01\x02\x12\x04\x97\x01\x19\x1c\n\x0e\n\x06\x04\x0f\x04\0\x02\x02\
    \x12\x04\x98\x01\x08\x1d\n\x0f\n\x07\x04\x0f\x04\0\x02\x02\x01\x12\x04\
    \x98\x01\x08\x16\n\x0f\n\x07\x04\x0f\x04\0\x02\x02\x02\x12\x04\x98\x01\
    \x19\x1c\n\x0e\n\x06\x04\x0f\x04\0\x02\x03\x12\x04\x99\x01\x08\x16\n\x0f\
    \n\x07\x04\x0f\x04\0\x02\x03\x01\x12\x04\x99\x01\x08\x0f\n\x0f\n\x07\x04\
    \x0f\x04\0\x02\x03\x02\x12\x04\x99\x01\x12\x15\n\x0e\n\x06\x04\x0f\x04\0\
    \x02\x04\x12\x04\x9a\x01\x08\x16\n\x0f\n\x07\x04\x0f\x04\0\x02\x04\x01\
    \x12\x04\x9a\x01\x08\x0f\n\x0f\n\x07\x04\x0f\x04\0\x02\x04\x02\x12\x04\
    \x9a\x01\x12\x15\n\x0e\n\x06\x04\x0f\x04\0\x02\x05\x12\x04\x9b\x01\x08\
    \x16\n\x0f\n\x07\x04\x0f\x04\0\x02\x05\x01\x12\x04\x9b\x01\x08\x0f\n\x0f\
    \n\x07\x04\x0f\x04\0\x02\x05\x02\x12\x04\x9b\x01\x12\x15\n\x0e\n\x06\x04\
    \x0f\x04\0\x02\x06\x12\x04\x9c\x01\x08\x15\n\x0f\n\x07\x04\x0f\x04\0\x02\
    \x06\x01\x12\x04\x9c\x01\x08\x0e\n\x0f\n\x07\x04\x0f\x04\0\x02\x06\x02\
    \x12\x04\x9c\x01\x11\x14\n\x0e\n\x06\x04\x0f\x04\0\x02\x07\x12\x04\x9d\
    \x01\x08\x1a\n\x0f\n\x07\x04\x0f\x04\0\x02\x07\x01\x12\x04\x9d\x01\x08\
    \x13\n\x0f\n\x07\x04\x0f\x04\0\x02\x07\x02\x12\x04\x9d\x01\x16\x19\n\x0e\
    \n\x06\x04\x0f\x04\0\x02\x08\x12\x04\x9e\x01\x08\x15\n\x0f\n\x07\x04\x0f\
    \x04\0\x02\x08\x01\x12\x04\x9e\x01\x08\x0e\n\x0f\n\x07\x04\x0f\x04\0\x02\
    \x08\x02\x12\x04\x9e\x01\x11\x14\n\x0e\n\x06\x04\x0f\x04\0\x02\t\x12\x04\
    \x9f\x01\x08\x15\n\x0f\n\x07\x04\x0f\x04\0\x02\t\x01\x12\x04\x9f\x01\x08\
    \x0e\n\x0f\n\x07\x04\x0f\x04\0\x02\t\x02\x12\x04\x9f\x01\x11\x14\n\x0e\n\
    \x06\x04\x0f\x04\0\x02\n\x12\x04\xa0\x01\x08\x16\n\x0f\n\x07\x04\x0f\x04\
    \0\x02\n\x01\x12\x04\xa0\x01\x08\x0f\n\x0f\n\x07\x04\x0f\x04\0\x02\n\x02\
    \x12\x04\xa0\x01\x12\x15\n\x0e\n\x06\x04\x0f\x04\0\x02\x0b\x12\x04\xa1\
    \x01\x08\x16\n\x0f\n\x07\x04\x0f\x04\0\x02\x0b\x01\x12\x04\xa1\x01\x08\
    \x0f\n\x0f\n\x07\x04\x0f\x04\0\x02\x0b\x02\x12\x04\xa1\x01\x12\x15\n\x0e\
    \n\x06\x04\x0f\x04\0\x02\x0c\x12\x04\xa2\x01\x08\x15\n\x0f\n\x07\x04\x0f\
    \x04\0\x02\x0c\x01\x12\x04\xa2\x01\x08\x0e\n\x0f\n\x07\x04\x0f\x04\0\x02\
    \x0c\x02\x12\x04\xa2\x01\x11\x14\n\x0e\n\x06\x04\x0f\x04\0\x02\r\x12\x04\
    \xa3\x01\x08\x15\n\x0f\n\x07\x04\x0f\x04\0\x02\r\x01\x12\x04\xa3\x01\x08\
    \x0e\n\x0f\n\x07\x04\x0f\x04\0\x02\r\x02\x12\x04\xa3\x01\x11\x14\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
