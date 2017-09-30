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
pub struct MercuryMultiGetRequest {
    // message fields
    request: ::protobuf::RepeatedField<MercuryRequest>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MercuryMultiGetRequest {}

impl MercuryMultiGetRequest {
    pub fn new() -> MercuryMultiGetRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MercuryMultiGetRequest {
        static mut instance: ::protobuf::lazy::Lazy<MercuryMultiGetRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MercuryMultiGetRequest,
        };
        unsafe {
            instance.get(MercuryMultiGetRequest::new)
        }
    }

    // repeated .MercuryRequest request = 1;

    pub fn clear_request(&mut self) {
        self.request.clear();
    }

    // Param is passed by value, moved
    pub fn set_request(&mut self, v: ::protobuf::RepeatedField<MercuryRequest>) {
        self.request = v;
    }

    // Mutable pointer to the field.
    pub fn mut_request(&mut self) -> &mut ::protobuf::RepeatedField<MercuryRequest> {
        &mut self.request
    }

    // Take field
    pub fn take_request(&mut self) -> ::protobuf::RepeatedField<MercuryRequest> {
        ::std::mem::replace(&mut self.request, ::protobuf::RepeatedField::new())
    }

    pub fn get_request(&self) -> &[MercuryRequest] {
        &self.request
    }

    fn get_request_for_reflect(&self) -> &::protobuf::RepeatedField<MercuryRequest> {
        &self.request
    }

    fn mut_request_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<MercuryRequest> {
        &mut self.request
    }
}

impl ::protobuf::Message for MercuryMultiGetRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.request {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.request)?;
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
        for value in &self.request {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.request {
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

impl ::protobuf::MessageStatic for MercuryMultiGetRequest {
    fn new() -> MercuryMultiGetRequest {
        MercuryMultiGetRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<MercuryMultiGetRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MercuryRequest>>(
                    "request",
                    MercuryMultiGetRequest::get_request_for_reflect,
                    MercuryMultiGetRequest::mut_request_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MercuryMultiGetRequest>(
                    "MercuryMultiGetRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MercuryMultiGetRequest {
    fn clear(&mut self) {
        self.clear_request();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MercuryMultiGetRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MercuryMultiGetRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MercuryMultiGetReply {
    // message fields
    reply: ::protobuf::RepeatedField<MercuryReply>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MercuryMultiGetReply {}

impl MercuryMultiGetReply {
    pub fn new() -> MercuryMultiGetReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MercuryMultiGetReply {
        static mut instance: ::protobuf::lazy::Lazy<MercuryMultiGetReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MercuryMultiGetReply,
        };
        unsafe {
            instance.get(MercuryMultiGetReply::new)
        }
    }

    // repeated .MercuryReply reply = 1;

    pub fn clear_reply(&mut self) {
        self.reply.clear();
    }

    // Param is passed by value, moved
    pub fn set_reply(&mut self, v: ::protobuf::RepeatedField<MercuryReply>) {
        self.reply = v;
    }

    // Mutable pointer to the field.
    pub fn mut_reply(&mut self) -> &mut ::protobuf::RepeatedField<MercuryReply> {
        &mut self.reply
    }

    // Take field
    pub fn take_reply(&mut self) -> ::protobuf::RepeatedField<MercuryReply> {
        ::std::mem::replace(&mut self.reply, ::protobuf::RepeatedField::new())
    }

    pub fn get_reply(&self) -> &[MercuryReply] {
        &self.reply
    }

    fn get_reply_for_reflect(&self) -> &::protobuf::RepeatedField<MercuryReply> {
        &self.reply
    }

    fn mut_reply_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<MercuryReply> {
        &mut self.reply
    }
}

impl ::protobuf::Message for MercuryMultiGetReply {
    fn is_initialized(&self) -> bool {
        for v in &self.reply {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.reply)?;
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
        for value in &self.reply {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.reply {
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

impl ::protobuf::MessageStatic for MercuryMultiGetReply {
    fn new() -> MercuryMultiGetReply {
        MercuryMultiGetReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<MercuryMultiGetReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MercuryReply>>(
                    "reply",
                    MercuryMultiGetReply::get_reply_for_reflect,
                    MercuryMultiGetReply::mut_reply_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MercuryMultiGetReply>(
                    "MercuryMultiGetReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MercuryMultiGetReply {
    fn clear(&mut self) {
        self.clear_reply();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MercuryMultiGetReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MercuryMultiGetReply {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MercuryRequest {
    // message fields
    uri: ::protobuf::SingularField<::std::string::String>,
    content_type: ::protobuf::SingularField<::std::string::String>,
    body: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    etag: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MercuryRequest {}

impl MercuryRequest {
    pub fn new() -> MercuryRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MercuryRequest {
        static mut instance: ::protobuf::lazy::Lazy<MercuryRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MercuryRequest,
        };
        unsafe {
            instance.get(MercuryRequest::new)
        }
    }

    // optional string uri = 1;

    pub fn clear_uri(&mut self) {
        self.uri.clear();
    }

    pub fn has_uri(&self) -> bool {
        self.uri.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uri(&mut self, v: ::std::string::String) {
        self.uri = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uri(&mut self) -> &mut ::std::string::String {
        if self.uri.is_none() {
            self.uri.set_default();
        }
        self.uri.as_mut().unwrap()
    }

    // Take field
    pub fn take_uri(&mut self) -> ::std::string::String {
        self.uri.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_uri(&self) -> &str {
        match self.uri.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_uri_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.uri
    }

    fn mut_uri_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.uri
    }

    // optional string content_type = 2;

    pub fn clear_content_type(&mut self) {
        self.content_type.clear();
    }

    pub fn has_content_type(&self) -> bool {
        self.content_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content_type(&mut self, v: ::std::string::String) {
        self.content_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content_type(&mut self) -> &mut ::std::string::String {
        if self.content_type.is_none() {
            self.content_type.set_default();
        }
        self.content_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_content_type(&mut self) -> ::std::string::String {
        self.content_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_content_type(&self) -> &str {
        match self.content_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_content_type_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.content_type
    }

    fn mut_content_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.content_type
    }

    // optional bytes body = 3;

    pub fn clear_body(&mut self) {
        self.body.clear();
    }

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_body(&mut self, v: ::std::vec::Vec<u8>) {
        self.body = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.body.is_none() {
            self.body.set_default();
        }
        self.body.as_mut().unwrap()
    }

    // Take field
    pub fn take_body(&mut self) -> ::std::vec::Vec<u8> {
        self.body.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_body(&self) -> &[u8] {
        match self.body.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_body_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.body
    }

    fn mut_body_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.body
    }

    // optional bytes etag = 4;

    pub fn clear_etag(&mut self) {
        self.etag.clear();
    }

    pub fn has_etag(&self) -> bool {
        self.etag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_etag(&mut self, v: ::std::vec::Vec<u8>) {
        self.etag = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_etag(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.etag.is_none() {
            self.etag.set_default();
        }
        self.etag.as_mut().unwrap()
    }

    // Take field
    pub fn take_etag(&mut self) -> ::std::vec::Vec<u8> {
        self.etag.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_etag(&self) -> &[u8] {
        match self.etag.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_etag_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.etag
    }

    fn mut_etag_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.etag
    }
}

impl ::protobuf::Message for MercuryRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.uri)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.content_type)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.body)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.etag)?;
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
        if let Some(ref v) = self.uri.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.content_type.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.body.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        if let Some(ref v) = self.etag.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.uri.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.content_type.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.body.as_ref() {
            os.write_bytes(3, &v)?;
        }
        if let Some(ref v) = self.etag.as_ref() {
            os.write_bytes(4, &v)?;
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

impl ::protobuf::MessageStatic for MercuryRequest {
    fn new() -> MercuryRequest {
        MercuryRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<MercuryRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "uri",
                    MercuryRequest::get_uri_for_reflect,
                    MercuryRequest::mut_uri_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "content_type",
                    MercuryRequest::get_content_type_for_reflect,
                    MercuryRequest::mut_content_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "body",
                    MercuryRequest::get_body_for_reflect,
                    MercuryRequest::mut_body_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "etag",
                    MercuryRequest::get_etag_for_reflect,
                    MercuryRequest::mut_etag_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MercuryRequest>(
                    "MercuryRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MercuryRequest {
    fn clear(&mut self) {
        self.clear_uri();
        self.clear_content_type();
        self.clear_body();
        self.clear_etag();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MercuryRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MercuryRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MercuryReply {
    // message fields
    status_code: ::std::option::Option<i32>,
    status_message: ::protobuf::SingularField<::std::string::String>,
    cache_policy: ::std::option::Option<MercuryReply_CachePolicy>,
    ttl: ::std::option::Option<i32>,
    etag: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    content_type: ::protobuf::SingularField<::std::string::String>,
    body: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MercuryReply {}

impl MercuryReply {
    pub fn new() -> MercuryReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MercuryReply {
        static mut instance: ::protobuf::lazy::Lazy<MercuryReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MercuryReply,
        };
        unsafe {
            instance.get(MercuryReply::new)
        }
    }

    // optional sint32 status_code = 1;

    pub fn clear_status_code(&mut self) {
        self.status_code = ::std::option::Option::None;
    }

    pub fn has_status_code(&self) -> bool {
        self.status_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status_code(&mut self, v: i32) {
        self.status_code = ::std::option::Option::Some(v);
    }

    pub fn get_status_code(&self) -> i32 {
        self.status_code.unwrap_or(0)
    }

    fn get_status_code_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.status_code
    }

    fn mut_status_code_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.status_code
    }

    // optional string status_message = 2;

    pub fn clear_status_message(&mut self) {
        self.status_message.clear();
    }

    pub fn has_status_message(&self) -> bool {
        self.status_message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status_message(&mut self, v: ::std::string::String) {
        self.status_message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status_message(&mut self) -> &mut ::std::string::String {
        if self.status_message.is_none() {
            self.status_message.set_default();
        }
        self.status_message.as_mut().unwrap()
    }

    // Take field
    pub fn take_status_message(&mut self) -> ::std::string::String {
        self.status_message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_status_message(&self) -> &str {
        match self.status_message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_status_message_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.status_message
    }

    fn mut_status_message_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.status_message
    }

    // optional .MercuryReply.CachePolicy cache_policy = 3;

    pub fn clear_cache_policy(&mut self) {
        self.cache_policy = ::std::option::Option::None;
    }

    pub fn has_cache_policy(&self) -> bool {
        self.cache_policy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cache_policy(&mut self, v: MercuryReply_CachePolicy) {
        self.cache_policy = ::std::option::Option::Some(v);
    }

    pub fn get_cache_policy(&self) -> MercuryReply_CachePolicy {
        self.cache_policy.unwrap_or(MercuryReply_CachePolicy::CACHE_NO)
    }

    fn get_cache_policy_for_reflect(&self) -> &::std::option::Option<MercuryReply_CachePolicy> {
        &self.cache_policy
    }

    fn mut_cache_policy_for_reflect(&mut self) -> &mut ::std::option::Option<MercuryReply_CachePolicy> {
        &mut self.cache_policy
    }

    // optional sint32 ttl = 4;

    pub fn clear_ttl(&mut self) {
        self.ttl = ::std::option::Option::None;
    }

    pub fn has_ttl(&self) -> bool {
        self.ttl.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ttl(&mut self, v: i32) {
        self.ttl = ::std::option::Option::Some(v);
    }

    pub fn get_ttl(&self) -> i32 {
        self.ttl.unwrap_or(0)
    }

    fn get_ttl_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ttl
    }

    fn mut_ttl_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ttl
    }

    // optional bytes etag = 5;

    pub fn clear_etag(&mut self) {
        self.etag.clear();
    }

    pub fn has_etag(&self) -> bool {
        self.etag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_etag(&mut self, v: ::std::vec::Vec<u8>) {
        self.etag = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_etag(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.etag.is_none() {
            self.etag.set_default();
        }
        self.etag.as_mut().unwrap()
    }

    // Take field
    pub fn take_etag(&mut self) -> ::std::vec::Vec<u8> {
        self.etag.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_etag(&self) -> &[u8] {
        match self.etag.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_etag_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.etag
    }

    fn mut_etag_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.etag
    }

    // optional string content_type = 6;

    pub fn clear_content_type(&mut self) {
        self.content_type.clear();
    }

    pub fn has_content_type(&self) -> bool {
        self.content_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content_type(&mut self, v: ::std::string::String) {
        self.content_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content_type(&mut self) -> &mut ::std::string::String {
        if self.content_type.is_none() {
            self.content_type.set_default();
        }
        self.content_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_content_type(&mut self) -> ::std::string::String {
        self.content_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_content_type(&self) -> &str {
        match self.content_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_content_type_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.content_type
    }

    fn mut_content_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.content_type
    }

    // optional bytes body = 7;

    pub fn clear_body(&mut self) {
        self.body.clear();
    }

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_body(&mut self, v: ::std::vec::Vec<u8>) {
        self.body = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.body.is_none() {
            self.body.set_default();
        }
        self.body.as_mut().unwrap()
    }

    // Take field
    pub fn take_body(&mut self) -> ::std::vec::Vec<u8> {
        self.body.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_body(&self) -> &[u8] {
        match self.body.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_body_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.body
    }

    fn mut_body_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.body
    }
}

impl ::protobuf::Message for MercuryReply {
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
                    self.status_code = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.status_message)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.cache_policy = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.ttl = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.etag)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.content_type)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.body)?;
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
        if let Some(v) = self.status_code {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, v);
        }
        if let Some(ref v) = self.status_message.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.cache_policy {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        if let Some(v) = self.ttl {
            my_size += ::protobuf::rt::value_varint_zigzag_size(4, v);
        }
        if let Some(ref v) = self.etag.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        }
        if let Some(ref v) = self.content_type.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        if let Some(ref v) = self.body.as_ref() {
            my_size += ::protobuf::rt::bytes_size(7, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status_code {
            os.write_sint32(1, v)?;
        }
        if let Some(ref v) = self.status_message.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.cache_policy {
            os.write_enum(3, v.value())?;
        }
        if let Some(v) = self.ttl {
            os.write_sint32(4, v)?;
        }
        if let Some(ref v) = self.etag.as_ref() {
            os.write_bytes(5, &v)?;
        }
        if let Some(ref v) = self.content_type.as_ref() {
            os.write_string(6, &v)?;
        }
        if let Some(ref v) = self.body.as_ref() {
            os.write_bytes(7, &v)?;
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

impl ::protobuf::MessageStatic for MercuryReply {
    fn new() -> MercuryReply {
        MercuryReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<MercuryReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "status_code",
                    MercuryReply::get_status_code_for_reflect,
                    MercuryReply::mut_status_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "status_message",
                    MercuryReply::get_status_message_for_reflect,
                    MercuryReply::mut_status_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<MercuryReply_CachePolicy>>(
                    "cache_policy",
                    MercuryReply::get_cache_policy_for_reflect,
                    MercuryReply::mut_cache_policy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "ttl",
                    MercuryReply::get_ttl_for_reflect,
                    MercuryReply::mut_ttl_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "etag",
                    MercuryReply::get_etag_for_reflect,
                    MercuryReply::mut_etag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "content_type",
                    MercuryReply::get_content_type_for_reflect,
                    MercuryReply::mut_content_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "body",
                    MercuryReply::get_body_for_reflect,
                    MercuryReply::mut_body_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MercuryReply>(
                    "MercuryReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MercuryReply {
    fn clear(&mut self) {
        self.clear_status_code();
        self.clear_status_message();
        self.clear_cache_policy();
        self.clear_ttl();
        self.clear_etag();
        self.clear_content_type();
        self.clear_body();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MercuryReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MercuryReply {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MercuryReply_CachePolicy {
    CACHE_NO = 1,
    CACHE_PRIVATE = 2,
    CACHE_PUBLIC = 3,
}

impl ::protobuf::ProtobufEnum for MercuryReply_CachePolicy {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MercuryReply_CachePolicy> {
        match value {
            1 => ::std::option::Option::Some(MercuryReply_CachePolicy::CACHE_NO),
            2 => ::std::option::Option::Some(MercuryReply_CachePolicy::CACHE_PRIVATE),
            3 => ::std::option::Option::Some(MercuryReply_CachePolicy::CACHE_PUBLIC),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MercuryReply_CachePolicy] = &[
            MercuryReply_CachePolicy::CACHE_NO,
            MercuryReply_CachePolicy::CACHE_PRIVATE,
            MercuryReply_CachePolicy::CACHE_PUBLIC,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<MercuryReply_CachePolicy>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MercuryReply_CachePolicy", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MercuryReply_CachePolicy {
}

impl ::protobuf::reflect::ProtobufValue for MercuryReply_CachePolicy {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Header {
    // message fields
    uri: ::protobuf::SingularField<::std::string::String>,
    content_type: ::protobuf::SingularField<::std::string::String>,
    method: ::protobuf::SingularField<::std::string::String>,
    status_code: ::std::option::Option<i32>,
    user_fields: ::protobuf::RepeatedField<UserField>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Header {}

impl Header {
    pub fn new() -> Header {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Header {
        static mut instance: ::protobuf::lazy::Lazy<Header> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Header,
        };
        unsafe {
            instance.get(Header::new)
        }
    }

    // optional string uri = 1;

    pub fn clear_uri(&mut self) {
        self.uri.clear();
    }

    pub fn has_uri(&self) -> bool {
        self.uri.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uri(&mut self, v: ::std::string::String) {
        self.uri = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uri(&mut self) -> &mut ::std::string::String {
        if self.uri.is_none() {
            self.uri.set_default();
        }
        self.uri.as_mut().unwrap()
    }

    // Take field
    pub fn take_uri(&mut self) -> ::std::string::String {
        self.uri.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_uri(&self) -> &str {
        match self.uri.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_uri_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.uri
    }

    fn mut_uri_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.uri
    }

    // optional string content_type = 2;

    pub fn clear_content_type(&mut self) {
        self.content_type.clear();
    }

    pub fn has_content_type(&self) -> bool {
        self.content_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_content_type(&mut self, v: ::std::string::String) {
        self.content_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_content_type(&mut self) -> &mut ::std::string::String {
        if self.content_type.is_none() {
            self.content_type.set_default();
        }
        self.content_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_content_type(&mut self) -> ::std::string::String {
        self.content_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_content_type(&self) -> &str {
        match self.content_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_content_type_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.content_type
    }

    fn mut_content_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.content_type
    }

    // optional string method = 3;

    pub fn clear_method(&mut self) {
        self.method.clear();
    }

    pub fn has_method(&self) -> bool {
        self.method.is_some()
    }

    // Param is passed by value, moved
    pub fn set_method(&mut self, v: ::std::string::String) {
        self.method = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_method(&mut self) -> &mut ::std::string::String {
        if self.method.is_none() {
            self.method.set_default();
        }
        self.method.as_mut().unwrap()
    }

    // Take field
    pub fn take_method(&mut self) -> ::std::string::String {
        self.method.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_method(&self) -> &str {
        match self.method.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_method_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.method
    }

    fn mut_method_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.method
    }

    // optional sint32 status_code = 4;

    pub fn clear_status_code(&mut self) {
        self.status_code = ::std::option::Option::None;
    }

    pub fn has_status_code(&self) -> bool {
        self.status_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status_code(&mut self, v: i32) {
        self.status_code = ::std::option::Option::Some(v);
    }

    pub fn get_status_code(&self) -> i32 {
        self.status_code.unwrap_or(0)
    }

    fn get_status_code_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.status_code
    }

    fn mut_status_code_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.status_code
    }

    // repeated .UserField user_fields = 6;

    pub fn clear_user_fields(&mut self) {
        self.user_fields.clear();
    }

    // Param is passed by value, moved
    pub fn set_user_fields(&mut self, v: ::protobuf::RepeatedField<UserField>) {
        self.user_fields = v;
    }

    // Mutable pointer to the field.
    pub fn mut_user_fields(&mut self) -> &mut ::protobuf::RepeatedField<UserField> {
        &mut self.user_fields
    }

    // Take field
    pub fn take_user_fields(&mut self) -> ::protobuf::RepeatedField<UserField> {
        ::std::mem::replace(&mut self.user_fields, ::protobuf::RepeatedField::new())
    }

    pub fn get_user_fields(&self) -> &[UserField] {
        &self.user_fields
    }

    fn get_user_fields_for_reflect(&self) -> &::protobuf::RepeatedField<UserField> {
        &self.user_fields
    }

    fn mut_user_fields_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<UserField> {
        &mut self.user_fields
    }
}

impl ::protobuf::Message for Header {
    fn is_initialized(&self) -> bool {
        for v in &self.user_fields {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.uri)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.content_type)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.method)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.status_code = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.user_fields)?;
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
        if let Some(ref v) = self.uri.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.content_type.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.method.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.status_code {
            my_size += ::protobuf::rt::value_varint_zigzag_size(4, v);
        }
        for value in &self.user_fields {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.uri.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.content_type.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.method.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(v) = self.status_code {
            os.write_sint32(4, v)?;
        }
        for v in &self.user_fields {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Header {
    fn new() -> Header {
        Header::new()
    }

    fn descriptor_static(_: ::std::option::Option<Header>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "uri",
                    Header::get_uri_for_reflect,
                    Header::mut_uri_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "content_type",
                    Header::get_content_type_for_reflect,
                    Header::mut_content_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "method",
                    Header::get_method_for_reflect,
                    Header::mut_method_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "status_code",
                    Header::get_status_code_for_reflect,
                    Header::mut_status_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UserField>>(
                    "user_fields",
                    Header::get_user_fields_for_reflect,
                    Header::mut_user_fields_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Header>(
                    "Header",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Header {
    fn clear(&mut self) {
        self.clear_uri();
        self.clear_content_type();
        self.clear_method();
        self.clear_status_code();
        self.clear_user_fields();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Header {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Header {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UserField {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UserField {}

impl UserField {
    pub fn new() -> UserField {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UserField {
        static mut instance: ::protobuf::lazy::Lazy<UserField> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UserField,
        };
        unsafe {
            instance.get(UserField::new)
        }
    }

    // optional string key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        if self.key.is_none() {
            self.key.set_default();
        }
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        self.key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        match self.key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.key
    }

    // optional bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.value
    }
}

impl ::protobuf::Message for UserField {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value)?;
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
        if let Some(ref v) = self.key.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.key.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.value.as_ref() {
            os.write_bytes(2, &v)?;
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

impl ::protobuf::MessageStatic for UserField {
    fn new() -> UserField {
        UserField::new()
    }

    fn descriptor_static(_: ::std::option::Option<UserField>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "key",
                    UserField::get_key_for_reflect,
                    UserField::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    UserField::get_value_for_reflect,
                    UserField::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UserField>(
                    "UserField",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UserField {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UserField {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UserField {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rmercury.proto\"C\n\x16MercuryMultiGetRequest\x12)\n\x07request\x18\
    \x01\x20\x03(\x0b2\x0f.MercuryRequestR\x07request\";\n\x14MercuryMultiGe\
    tReply\x12#\n\x05reply\x18\x01\x20\x03(\x0b2\r.MercuryReplyR\x05reply\"m\
    \n\x0eMercuryRequest\x12\x10\n\x03uri\x18\x01\x20\x01(\tR\x03uri\x12!\n\
    \x0ccontent_type\x18\x02\x20\x01(\tR\x0bcontentType\x12\x12\n\x04body\
    \x18\x03\x20\x01(\x0cR\x04body\x12\x12\n\x04etag\x18\x04\x20\x01(\x0cR\
    \x04etag\"\xb3\x02\n\x0cMercuryReply\x12\x1f\n\x0bstatus_code\x18\x01\
    \x20\x01(\x11R\nstatusCode\x12%\n\x0estatus_message\x18\x02\x20\x01(\tR\
    \rstatusMessage\x12<\n\x0ccache_policy\x18\x03\x20\x01(\x0e2\x19.Mercury\
    Reply.CachePolicyR\x0bcachePolicy\x12\x10\n\x03ttl\x18\x04\x20\x01(\x11R\
    \x03ttl\x12\x12\n\x04etag\x18\x05\x20\x01(\x0cR\x04etag\x12!\n\x0cconten\
    t_type\x18\x06\x20\x01(\tR\x0bcontentType\x12\x12\n\x04body\x18\x07\x20\
    \x01(\x0cR\x04body\"@\n\x0bCachePolicy\x12\x0c\n\x08CACHE_NO\x10\x01\x12\
    \x11\n\rCACHE_PRIVATE\x10\x02\x12\x10\n\x0cCACHE_PUBLIC\x10\x03\"\xa3\
    \x01\n\x06Header\x12\x10\n\x03uri\x18\x01\x20\x01(\tR\x03uri\x12!\n\x0cc\
    ontent_type\x18\x02\x20\x01(\tR\x0bcontentType\x12\x16\n\x06method\x18\
    \x03\x20\x01(\tR\x06method\x12\x1f\n\x0bstatus_code\x18\x04\x20\x01(\x11\
    R\nstatusCode\x12+\n\x0buser_fields\x18\x06\x20\x03(\x0b2\n.UserFieldR\n\
    userFields\"3\n\tUserField\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\
    \x12\x14\n\x05value\x18\x02\x20\x01(\x0cR\x05valueJ\xaf\r\n\x06\x12\x04\
    \0\0,\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\
    \x04\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x1e\n\x0b\n\x04\x04\0\x02\0\
    \x12\x03\x03\x04*\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x03\x04\x0c\n\x0c\
    \n\x05\x04\0\x02\0\x06\x12\x03\x03\r\x1b\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03\x03\x1c#\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03&)\n\n\n\x02\x04\
    \x01\x12\x04\x06\0\x08\x01\n\n\n\x03\x04\x01\x01\x12\x03\x06\x08\x1c\n\
    \x0b\n\x04\x04\x01\x02\0\x12\x03\x07\x04&\n\x0c\n\x05\x04\x01\x02\0\x04\
    \x12\x03\x07\x04\x0c\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x07\r\x19\n\
    \x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x07\x1a\x1f\n\x0c\n\x05\x04\x01\x02\
    \0\x03\x12\x03\x07\"%\n\n\n\x02\x04\x02\x12\x04\n\0\x0f\x01\n\n\n\x03\
    \x04\x02\x01\x12\x03\n\x08\x16\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x0b\x04\
    \x1e\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03\x0b\x04\x0c\n\x0c\n\x05\x04\
    \x02\x02\0\x05\x12\x03\x0b\r\x13\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\
    \x0b\x14\x17\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x0b\x1a\x1d\n\x0b\n\
    \x04\x04\x02\x02\x01\x12\x03\x0c\x04'\n\x0c\n\x05\x04\x02\x02\x01\x04\
    \x12\x03\x0c\x04\x0c\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x0c\r\x13\n\
    \x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x0c\x14\x20\n\x0c\n\x05\x04\x02\
    \x02\x01\x03\x12\x03\x0c#&\n\x0b\n\x04\x04\x02\x02\x02\x12\x03\r\x04\x1e\
    \n\x0c\n\x05\x04\x02\x02\x02\x04\x12\x03\r\x04\x0c\n\x0c\n\x05\x04\x02\
    \x02\x02\x05\x12\x03\r\r\x12\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03\r\
    \x13\x17\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\r\x1a\x1d\n\x0b\n\x04\
    \x04\x02\x02\x03\x12\x03\x0e\x04\x1e\n\x0c\n\x05\x04\x02\x02\x03\x04\x12\
    \x03\x0e\x04\x0c\n\x0c\n\x05\x04\x02\x02\x03\x05\x12\x03\x0e\r\x12\n\x0c\
    \n\x05\x04\x02\x02\x03\x01\x12\x03\x0e\x13\x17\n\x0c\n\x05\x04\x02\x02\
    \x03\x03\x12\x03\x0e\x1a\x1d\n\n\n\x02\x04\x03\x12\x04\x11\0\x1e\x01\n\n\
    \n\x03\x04\x03\x01\x12\x03\x11\x08\x14\n\x0b\n\x04\x04\x03\x02\0\x12\x03\
    \x12\x04&\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03\x12\x04\x0c\n\x0c\n\x05\
    \x04\x03\x02\0\x05\x12\x03\x12\r\x13\n\x0c\n\x05\x04\x03\x02\0\x01\x12\
    \x03\x12\x14\x1f\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x12\"%\n\x0b\n\
    \x04\x04\x03\x02\x01\x12\x03\x13\x04)\n\x0c\n\x05\x04\x03\x02\x01\x04\
    \x12\x03\x13\x04\x0c\n\x0c\n\x05\x04\x03\x02\x01\x05\x12\x03\x13\r\x13\n\
    \x0c\n\x05\x04\x03\x02\x01\x01\x12\x03\x13\x14\"\n\x0c\n\x05\x04\x03\x02\
    \x01\x03\x12\x03\x13%(\n\x0b\n\x04\x04\x03\x02\x02\x12\x03\x14\x04,\n\
    \x0c\n\x05\x04\x03\x02\x02\x04\x12\x03\x14\x04\x0c\n\x0c\n\x05\x04\x03\
    \x02\x02\x06\x12\x03\x14\r\x18\n\x0c\n\x05\x04\x03\x02\x02\x01\x12\x03\
    \x14\x19%\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x03\x14(+\n\x0c\n\x04\x04\
    \x03\x04\0\x12\x04\x15\x04\x19\x05\n\x0c\n\x05\x04\x03\x04\0\x01\x12\x03\
    \x15\t\x14\n\r\n\x06\x04\x03\x04\0\x02\0\x12\x03\x16\x08\x17\n\x0e\n\x07\
    \x04\x03\x04\0\x02\0\x01\x12\x03\x16\x08\x10\n\x0e\n\x07\x04\x03\x04\0\
    \x02\0\x02\x12\x03\x16\x13\x16\n\r\n\x06\x04\x03\x04\0\x02\x01\x12\x03\
    \x17\x08\x1c\n\x0e\n\x07\x04\x03\x04\0\x02\x01\x01\x12\x03\x17\x08\x15\n\
    \x0e\n\x07\x04\x03\x04\0\x02\x01\x02\x12\x03\x17\x18\x1b\n\r\n\x06\x04\
    \x03\x04\0\x02\x02\x12\x03\x18\x08\x1b\n\x0e\n\x07\x04\x03\x04\0\x02\x02\
    \x01\x12\x03\x18\x08\x14\n\x0e\n\x07\x04\x03\x04\0\x02\x02\x02\x12\x03\
    \x18\x17\x1a\n\x0b\n\x04\x04\x03\x02\x03\x12\x03\x1a\x04\x1e\n\x0c\n\x05\
    \x04\x03\x02\x03\x04\x12\x03\x1a\x04\x0c\n\x0c\n\x05\x04\x03\x02\x03\x05\
    \x12\x03\x1a\r\x13\n\x0c\n\x05\x04\x03\x02\x03\x01\x12\x03\x1a\x14\x17\n\
    \x0c\n\x05\x04\x03\x02\x03\x03\x12\x03\x1a\x1a\x1d\n\x0b\n\x04\x04\x03\
    \x02\x04\x12\x03\x1b\x04\x1e\n\x0c\n\x05\x04\x03\x02\x04\x04\x12\x03\x1b\
    \x04\x0c\n\x0c\n\x05\x04\x03\x02\x04\x05\x12\x03\x1b\r\x12\n\x0c\n\x05\
    \x04\x03\x02\x04\x01\x12\x03\x1b\x13\x17\n\x0c\n\x05\x04\x03\x02\x04\x03\
    \x12\x03\x1b\x1a\x1d\n\x0b\n\x04\x04\x03\x02\x05\x12\x03\x1c\x04'\n\x0c\
    \n\x05\x04\x03\x02\x05\x04\x12\x03\x1c\x04\x0c\n\x0c\n\x05\x04\x03\x02\
    \x05\x05\x12\x03\x1c\r\x13\n\x0c\n\x05\x04\x03\x02\x05\x01\x12\x03\x1c\
    \x14\x20\n\x0c\n\x05\x04\x03\x02\x05\x03\x12\x03\x1c#&\n\x0b\n\x04\x04\
    \x03\x02\x06\x12\x03\x1d\x04\x1e\n\x0c\n\x05\x04\x03\x02\x06\x04\x12\x03\
    \x1d\x04\x0c\n\x0c\n\x05\x04\x03\x02\x06\x05\x12\x03\x1d\r\x12\n\x0c\n\
    \x05\x04\x03\x02\x06\x01\x12\x03\x1d\x13\x17\n\x0c\n\x05\x04\x03\x02\x06\
    \x03\x12\x03\x1d\x1a\x1d\n\n\n\x02\x04\x04\x12\x04!\0'\x01\n\n\n\x03\x04\
    \x04\x01\x12\x03!\x08\x0e\n\x0b\n\x04\x04\x04\x02\0\x12\x03\"\x04\x1f\n\
    \x0c\n\x05\x04\x04\x02\0\x04\x12\x03\"\x04\x0c\n\x0c\n\x05\x04\x04\x02\0\
    \x05\x12\x03\"\r\x13\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03\"\x14\x17\n\
    \x0c\n\x05\x04\x04\x02\0\x03\x12\x03\"\x1a\x1e\n\x0b\n\x04\x04\x04\x02\
    \x01\x12\x03#\x04(\n\x0c\n\x05\x04\x04\x02\x01\x04\x12\x03#\x04\x0c\n\
    \x0c\n\x05\x04\x04\x02\x01\x05\x12\x03#\r\x13\n\x0c\n\x05\x04\x04\x02\
    \x01\x01\x12\x03#\x14\x20\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03##'\n\
    \x0b\n\x04\x04\x04\x02\x02\x12\x03$\x04\"\n\x0c\n\x05\x04\x04\x02\x02\
    \x04\x12\x03$\x04\x0c\n\x0c\n\x05\x04\x04\x02\x02\x05\x12\x03$\r\x13\n\
    \x0c\n\x05\x04\x04\x02\x02\x01\x12\x03$\x14\x1a\n\x0c\n\x05\x04\x04\x02\
    \x02\x03\x12\x03$\x1d!\n\x0b\n\x04\x04\x04\x02\x03\x12\x03%\x04'\n\x0c\n\
    \x05\x04\x04\x02\x03\x04\x12\x03%\x04\x0c\n\x0c\n\x05\x04\x04\x02\x03\
    \x05\x12\x03%\r\x13\n\x0c\n\x05\x04\x04\x02\x03\x01\x12\x03%\x14\x1f\n\
    \x0c\n\x05\x04\x04\x02\x03\x03\x12\x03%\"&\n\x0b\n\x04\x04\x04\x02\x04\
    \x12\x03&\x04*\n\x0c\n\x05\x04\x04\x02\x04\x04\x12\x03&\x04\x0c\n\x0c\n\
    \x05\x04\x04\x02\x04\x06\x12\x03&\r\x16\n\x0c\n\x05\x04\x04\x02\x04\x01\
    \x12\x03&\x17\"\n\x0c\n\x05\x04\x04\x02\x04\x03\x12\x03&%)\n\n\n\x02\x04\
    \x05\x12\x04)\0,\x01\n\n\n\x03\x04\x05\x01\x12\x03)\x08\x11\n\x0b\n\x04\
    \x04\x05\x02\0\x12\x03*\x04\x1f\n\x0c\n\x05\x04\x05\x02\0\x04\x12\x03*\
    \x04\x0c\n\x0c\n\x05\x04\x05\x02\0\x05\x12\x03*\r\x13\n\x0c\n\x05\x04\
    \x05\x02\0\x01\x12\x03*\x14\x17\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03*\
    \x1a\x1e\n\x0b\n\x04\x04\x05\x02\x01\x12\x03+\x04\x20\n\x0c\n\x05\x04\
    \x05\x02\x01\x04\x12\x03+\x04\x0c\n\x0c\n\x05\x04\x05\x02\x01\x05\x12\
    \x03+\r\x12\n\x0c\n\x05\x04\x05\x02\x01\x01\x12\x03+\x13\x18\n\x0c\n\x05\
    \x04\x05\x02\x01\x03\x12\x03+\x1b\x1f\
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
