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
pub struct Subscription {
    // message fields
    uri: ::protobuf::SingularField<::std::string::String>,
    expiry: ::std::option::Option<i32>,
    status_code: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Subscription {}

impl Subscription {
    pub fn new() -> Subscription {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Subscription {
        static mut instance: ::protobuf::lazy::Lazy<Subscription> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Subscription,
        };
        unsafe {
            instance.get(Subscription::new)
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

    // optional int32 expiry = 2;

    pub fn clear_expiry(&mut self) {
        self.expiry = ::std::option::Option::None;
    }

    pub fn has_expiry(&self) -> bool {
        self.expiry.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expiry(&mut self, v: i32) {
        self.expiry = ::std::option::Option::Some(v);
    }

    pub fn get_expiry(&self) -> i32 {
        self.expiry.unwrap_or(0)
    }

    fn get_expiry_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.expiry
    }

    fn mut_expiry_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.expiry
    }

    // optional int32 status_code = 3;

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
}

impl ::protobuf::Message for Subscription {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.expiry = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.status_code = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.expiry {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.status_code {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.uri.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.expiry {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.status_code {
            os.write_int32(3, v)?;
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

impl ::protobuf::MessageStatic for Subscription {
    fn new() -> Subscription {
        Subscription::new()
    }

    fn descriptor_static(_: ::std::option::Option<Subscription>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "uri",
                    Subscription::get_uri_for_reflect,
                    Subscription::mut_uri_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "expiry",
                    Subscription::get_expiry_for_reflect,
                    Subscription::mut_expiry_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "status_code",
                    Subscription::get_status_code_for_reflect,
                    Subscription::mut_status_code_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Subscription>(
                    "Subscription",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Subscription {
    fn clear(&mut self) {
        self.clear_uri();
        self.clear_expiry();
        self.clear_status_code();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Subscription {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Subscription {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cpubsub.proto\"Y\n\x0cSubscription\x12\x10\n\x03uri\x18\x01\x20\x01\
    (\tR\x03uri\x12\x16\n\x06expiry\x18\x02\x20\x01(\x05R\x06expiry\x12\x1f\
    \n\x0bstatus_code\x18\x03\x20\x01(\x05R\nstatusCodeJ\xf9\x01\n\x06\x12\
    \x04\0\0\x06\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\
    \x02\0\x06\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x14\n\x0b\n\x04\x04\0\
    \x02\0\x12\x03\x03\x04\x1e\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x03\x04\
    \x0c\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x03\r\x13\n\x0c\n\x05\x04\0\x02\
    \0\x01\x12\x03\x03\x14\x17\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03\x1a\
    \x1d\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x04\x04\x20\n\x0c\n\x05\x04\0\x02\
    \x01\x04\x12\x03\x04\x04\x0c\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x04\r\
    \x12\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x04\x13\x19\n\x0c\n\x05\x04\0\
    \x02\x01\x03\x12\x03\x04\x1c\x1f\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x05\
    \x04%\n\x0c\n\x05\x04\0\x02\x02\x04\x12\x03\x05\x04\x0c\n\x0c\n\x05\x04\
    \0\x02\x02\x05\x12\x03\x05\r\x12\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\
    \x05\x13\x1e\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x05!$\
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
