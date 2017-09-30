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
pub struct ClientHello {
    // message fields
    build_info: ::protobuf::SingularPtrField<BuildInfo>,
    fingerprints_supported: ::std::vec::Vec<Fingerprint>,
    cryptosuites_supported: ::std::vec::Vec<Cryptosuite>,
    powschemes_supported: ::std::vec::Vec<Powscheme>,
    login_crypto_hello: ::protobuf::SingularPtrField<LoginCryptoHelloUnion>,
    client_nonce: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    padding: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    feature_set: ::protobuf::SingularPtrField<FeatureSet>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientHello {}

impl ClientHello {
    pub fn new() -> ClientHello {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientHello {
        static mut instance: ::protobuf::lazy::Lazy<ClientHello> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientHello,
        };
        unsafe {
            instance.get(ClientHello::new)
        }
    }

    // required .BuildInfo build_info = 10;

    pub fn clear_build_info(&mut self) {
        self.build_info.clear();
    }

    pub fn has_build_info(&self) -> bool {
        self.build_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_build_info(&mut self, v: BuildInfo) {
        self.build_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_build_info(&mut self) -> &mut BuildInfo {
        if self.build_info.is_none() {
            self.build_info.set_default();
        }
        self.build_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_build_info(&mut self) -> BuildInfo {
        self.build_info.take().unwrap_or_else(|| BuildInfo::new())
    }

    pub fn get_build_info(&self) -> &BuildInfo {
        self.build_info.as_ref().unwrap_or_else(|| BuildInfo::default_instance())
    }

    fn get_build_info_for_reflect(&self) -> &::protobuf::SingularPtrField<BuildInfo> {
        &self.build_info
    }

    fn mut_build_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BuildInfo> {
        &mut self.build_info
    }

    // repeated .Fingerprint fingerprints_supported = 20;

    pub fn clear_fingerprints_supported(&mut self) {
        self.fingerprints_supported.clear();
    }

    // Param is passed by value, moved
    pub fn set_fingerprints_supported(&mut self, v: ::std::vec::Vec<Fingerprint>) {
        self.fingerprints_supported = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fingerprints_supported(&mut self) -> &mut ::std::vec::Vec<Fingerprint> {
        &mut self.fingerprints_supported
    }

    // Take field
    pub fn take_fingerprints_supported(&mut self) -> ::std::vec::Vec<Fingerprint> {
        ::std::mem::replace(&mut self.fingerprints_supported, ::std::vec::Vec::new())
    }

    pub fn get_fingerprints_supported(&self) -> &[Fingerprint] {
        &self.fingerprints_supported
    }

    fn get_fingerprints_supported_for_reflect(&self) -> &::std::vec::Vec<Fingerprint> {
        &self.fingerprints_supported
    }

    fn mut_fingerprints_supported_for_reflect(&mut self) -> &mut ::std::vec::Vec<Fingerprint> {
        &mut self.fingerprints_supported
    }

    // repeated .Cryptosuite cryptosuites_supported = 30;

    pub fn clear_cryptosuites_supported(&mut self) {
        self.cryptosuites_supported.clear();
    }

    // Param is passed by value, moved
    pub fn set_cryptosuites_supported(&mut self, v: ::std::vec::Vec<Cryptosuite>) {
        self.cryptosuites_supported = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cryptosuites_supported(&mut self) -> &mut ::std::vec::Vec<Cryptosuite> {
        &mut self.cryptosuites_supported
    }

    // Take field
    pub fn take_cryptosuites_supported(&mut self) -> ::std::vec::Vec<Cryptosuite> {
        ::std::mem::replace(&mut self.cryptosuites_supported, ::std::vec::Vec::new())
    }

    pub fn get_cryptosuites_supported(&self) -> &[Cryptosuite] {
        &self.cryptosuites_supported
    }

    fn get_cryptosuites_supported_for_reflect(&self) -> &::std::vec::Vec<Cryptosuite> {
        &self.cryptosuites_supported
    }

    fn mut_cryptosuites_supported_for_reflect(&mut self) -> &mut ::std::vec::Vec<Cryptosuite> {
        &mut self.cryptosuites_supported
    }

    // repeated .Powscheme powschemes_supported = 40;

    pub fn clear_powschemes_supported(&mut self) {
        self.powschemes_supported.clear();
    }

    // Param is passed by value, moved
    pub fn set_powschemes_supported(&mut self, v: ::std::vec::Vec<Powscheme>) {
        self.powschemes_supported = v;
    }

    // Mutable pointer to the field.
    pub fn mut_powschemes_supported(&mut self) -> &mut ::std::vec::Vec<Powscheme> {
        &mut self.powschemes_supported
    }

    // Take field
    pub fn take_powschemes_supported(&mut self) -> ::std::vec::Vec<Powscheme> {
        ::std::mem::replace(&mut self.powschemes_supported, ::std::vec::Vec::new())
    }

    pub fn get_powschemes_supported(&self) -> &[Powscheme] {
        &self.powschemes_supported
    }

    fn get_powschemes_supported_for_reflect(&self) -> &::std::vec::Vec<Powscheme> {
        &self.powschemes_supported
    }

    fn mut_powschemes_supported_for_reflect(&mut self) -> &mut ::std::vec::Vec<Powscheme> {
        &mut self.powschemes_supported
    }

    // required .LoginCryptoHelloUnion login_crypto_hello = 50;

    pub fn clear_login_crypto_hello(&mut self) {
        self.login_crypto_hello.clear();
    }

    pub fn has_login_crypto_hello(&self) -> bool {
        self.login_crypto_hello.is_some()
    }

    // Param is passed by value, moved
    pub fn set_login_crypto_hello(&mut self, v: LoginCryptoHelloUnion) {
        self.login_crypto_hello = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_login_crypto_hello(&mut self) -> &mut LoginCryptoHelloUnion {
        if self.login_crypto_hello.is_none() {
            self.login_crypto_hello.set_default();
        }
        self.login_crypto_hello.as_mut().unwrap()
    }

    // Take field
    pub fn take_login_crypto_hello(&mut self) -> LoginCryptoHelloUnion {
        self.login_crypto_hello.take().unwrap_or_else(|| LoginCryptoHelloUnion::new())
    }

    pub fn get_login_crypto_hello(&self) -> &LoginCryptoHelloUnion {
        self.login_crypto_hello.as_ref().unwrap_or_else(|| LoginCryptoHelloUnion::default_instance())
    }

    fn get_login_crypto_hello_for_reflect(&self) -> &::protobuf::SingularPtrField<LoginCryptoHelloUnion> {
        &self.login_crypto_hello
    }

    fn mut_login_crypto_hello_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LoginCryptoHelloUnion> {
        &mut self.login_crypto_hello
    }

    // required bytes client_nonce = 60;

    pub fn clear_client_nonce(&mut self) {
        self.client_nonce.clear();
    }

    pub fn has_client_nonce(&self) -> bool {
        self.client_nonce.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_nonce(&mut self, v: ::std::vec::Vec<u8>) {
        self.client_nonce = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client_nonce(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.client_nonce.is_none() {
            self.client_nonce.set_default();
        }
        self.client_nonce.as_mut().unwrap()
    }

    // Take field
    pub fn take_client_nonce(&mut self) -> ::std::vec::Vec<u8> {
        self.client_nonce.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_client_nonce(&self) -> &[u8] {
        match self.client_nonce.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_client_nonce_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.client_nonce
    }

    fn mut_client_nonce_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.client_nonce
    }

    // optional bytes padding = 70;

    pub fn clear_padding(&mut self) {
        self.padding.clear();
    }

    pub fn has_padding(&self) -> bool {
        self.padding.is_some()
    }

    // Param is passed by value, moved
    pub fn set_padding(&mut self, v: ::std::vec::Vec<u8>) {
        self.padding = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_padding(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.padding.is_none() {
            self.padding.set_default();
        }
        self.padding.as_mut().unwrap()
    }

    // Take field
    pub fn take_padding(&mut self) -> ::std::vec::Vec<u8> {
        self.padding.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_padding(&self) -> &[u8] {
        match self.padding.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_padding_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.padding
    }

    fn mut_padding_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.padding
    }

    // optional .FeatureSet feature_set = 80;

    pub fn clear_feature_set(&mut self) {
        self.feature_set.clear();
    }

    pub fn has_feature_set(&self) -> bool {
        self.feature_set.is_some()
    }

    // Param is passed by value, moved
    pub fn set_feature_set(&mut self, v: FeatureSet) {
        self.feature_set = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_feature_set(&mut self) -> &mut FeatureSet {
        if self.feature_set.is_none() {
            self.feature_set.set_default();
        }
        self.feature_set.as_mut().unwrap()
    }

    // Take field
    pub fn take_feature_set(&mut self) -> FeatureSet {
        self.feature_set.take().unwrap_or_else(|| FeatureSet::new())
    }

    pub fn get_feature_set(&self) -> &FeatureSet {
        self.feature_set.as_ref().unwrap_or_else(|| FeatureSet::default_instance())
    }

    fn get_feature_set_for_reflect(&self) -> &::protobuf::SingularPtrField<FeatureSet> {
        &self.feature_set
    }

    fn mut_feature_set_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FeatureSet> {
        &mut self.feature_set
    }
}

impl ::protobuf::Message for ClientHello {
    fn is_initialized(&self) -> bool {
        if self.build_info.is_none() {
            return false;
        }
        if self.login_crypto_hello.is_none() {
            return false;
        }
        if self.client_nonce.is_none() {
            return false;
        }
        for v in &self.build_info {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.login_crypto_hello {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.feature_set {
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
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.build_info)?;
                },
                20 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.fingerprints_supported)?;
                },
                30 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.cryptosuites_supported)?;
                },
                40 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.powschemes_supported)?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.login_crypto_hello)?;
                },
                60 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.client_nonce)?;
                },
                70 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.padding)?;
                },
                80 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.feature_set)?;
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
        if let Some(ref v) = self.build_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.fingerprints_supported {
            my_size += ::protobuf::rt::enum_size(20, *value);
        };
        for value in &self.cryptosuites_supported {
            my_size += ::protobuf::rt::enum_size(30, *value);
        };
        for value in &self.powschemes_supported {
            my_size += ::protobuf::rt::enum_size(40, *value);
        };
        if let Some(ref v) = self.login_crypto_hello.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.client_nonce.as_ref() {
            my_size += ::protobuf::rt::bytes_size(60, &v);
        }
        if let Some(ref v) = self.padding.as_ref() {
            my_size += ::protobuf::rt::bytes_size(70, &v);
        }
        if let Some(ref v) = self.feature_set.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.build_info.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.fingerprints_supported {
            os.write_enum(20, v.value())?;
        };
        for v in &self.cryptosuites_supported {
            os.write_enum(30, v.value())?;
        };
        for v in &self.powschemes_supported {
            os.write_enum(40, v.value())?;
        };
        if let Some(ref v) = self.login_crypto_hello.as_ref() {
            os.write_tag(50, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.client_nonce.as_ref() {
            os.write_bytes(60, &v)?;
        }
        if let Some(ref v) = self.padding.as_ref() {
            os.write_bytes(70, &v)?;
        }
        if let Some(ref v) = self.feature_set.as_ref() {
            os.write_tag(80, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ClientHello {
    fn new() -> ClientHello {
        ClientHello::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientHello>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BuildInfo>>(
                    "build_info",
                    ClientHello::get_build_info_for_reflect,
                    ClientHello::mut_build_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Fingerprint>>(
                    "fingerprints_supported",
                    ClientHello::get_fingerprints_supported_for_reflect,
                    ClientHello::mut_fingerprints_supported_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Cryptosuite>>(
                    "cryptosuites_supported",
                    ClientHello::get_cryptosuites_supported_for_reflect,
                    ClientHello::mut_cryptosuites_supported_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Powscheme>>(
                    "powschemes_supported",
                    ClientHello::get_powschemes_supported_for_reflect,
                    ClientHello::mut_powschemes_supported_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LoginCryptoHelloUnion>>(
                    "login_crypto_hello",
                    ClientHello::get_login_crypto_hello_for_reflect,
                    ClientHello::mut_login_crypto_hello_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "client_nonce",
                    ClientHello::get_client_nonce_for_reflect,
                    ClientHello::mut_client_nonce_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "padding",
                    ClientHello::get_padding_for_reflect,
                    ClientHello::mut_padding_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FeatureSet>>(
                    "feature_set",
                    ClientHello::get_feature_set_for_reflect,
                    ClientHello::mut_feature_set_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientHello>(
                    "ClientHello",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientHello {
    fn clear(&mut self) {
        self.clear_build_info();
        self.clear_fingerprints_supported();
        self.clear_cryptosuites_supported();
        self.clear_powschemes_supported();
        self.clear_login_crypto_hello();
        self.clear_client_nonce();
        self.clear_padding();
        self.clear_feature_set();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClientHello {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientHello {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BuildInfo {
    // message fields
    product: ::std::option::Option<Product>,
    product_flags: ::std::vec::Vec<ProductFlags>,
    platform: ::std::option::Option<Platform>,
    version: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BuildInfo {}

impl BuildInfo {
    pub fn new() -> BuildInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BuildInfo {
        static mut instance: ::protobuf::lazy::Lazy<BuildInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BuildInfo,
        };
        unsafe {
            instance.get(BuildInfo::new)
        }
    }

    // required .Product product = 10;

    pub fn clear_product(&mut self) {
        self.product = ::std::option::Option::None;
    }

    pub fn has_product(&self) -> bool {
        self.product.is_some()
    }

    // Param is passed by value, moved
    pub fn set_product(&mut self, v: Product) {
        self.product = ::std::option::Option::Some(v);
    }

    pub fn get_product(&self) -> Product {
        self.product.unwrap_or(Product::PRODUCT_CLIENT)
    }

    fn get_product_for_reflect(&self) -> &::std::option::Option<Product> {
        &self.product
    }

    fn mut_product_for_reflect(&mut self) -> &mut ::std::option::Option<Product> {
        &mut self.product
    }

    // repeated .ProductFlags product_flags = 20;

    pub fn clear_product_flags(&mut self) {
        self.product_flags.clear();
    }

    // Param is passed by value, moved
    pub fn set_product_flags(&mut self, v: ::std::vec::Vec<ProductFlags>) {
        self.product_flags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_product_flags(&mut self) -> &mut ::std::vec::Vec<ProductFlags> {
        &mut self.product_flags
    }

    // Take field
    pub fn take_product_flags(&mut self) -> ::std::vec::Vec<ProductFlags> {
        ::std::mem::replace(&mut self.product_flags, ::std::vec::Vec::new())
    }

    pub fn get_product_flags(&self) -> &[ProductFlags] {
        &self.product_flags
    }

    fn get_product_flags_for_reflect(&self) -> &::std::vec::Vec<ProductFlags> {
        &self.product_flags
    }

    fn mut_product_flags_for_reflect(&mut self) -> &mut ::std::vec::Vec<ProductFlags> {
        &mut self.product_flags
    }

    // required .Platform platform = 30;

    pub fn clear_platform(&mut self) {
        self.platform = ::std::option::Option::None;
    }

    pub fn has_platform(&self) -> bool {
        self.platform.is_some()
    }

    // Param is passed by value, moved
    pub fn set_platform(&mut self, v: Platform) {
        self.platform = ::std::option::Option::Some(v);
    }

    pub fn get_platform(&self) -> Platform {
        self.platform.unwrap_or(Platform::PLATFORM_WIN32_X86)
    }

    fn get_platform_for_reflect(&self) -> &::std::option::Option<Platform> {
        &self.platform
    }

    fn mut_platform_for_reflect(&mut self) -> &mut ::std::option::Option<Platform> {
        &mut self.platform
    }

    // required uint64 version = 40;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u64 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.version
    }
}

impl ::protobuf::Message for BuildInfo {
    fn is_initialized(&self) -> bool {
        if self.product.is_none() {
            return false;
        }
        if self.platform.is_none() {
            return false;
        }
        if self.version.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.product = ::std::option::Option::Some(tmp);
                },
                20 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.product_flags)?;
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.platform = ::std::option::Option::Some(tmp);
                },
                40 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.version = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.product {
            my_size += ::protobuf::rt::enum_size(10, v);
        }
        for value in &self.product_flags {
            my_size += ::protobuf::rt::enum_size(20, *value);
        };
        if let Some(v) = self.platform {
            my_size += ::protobuf::rt::enum_size(30, v);
        }
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(40, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.product {
            os.write_enum(10, v.value())?;
        }
        for v in &self.product_flags {
            os.write_enum(20, v.value())?;
        };
        if let Some(v) = self.platform {
            os.write_enum(30, v.value())?;
        }
        if let Some(v) = self.version {
            os.write_uint64(40, v)?;
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

impl ::protobuf::MessageStatic for BuildInfo {
    fn new() -> BuildInfo {
        BuildInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<BuildInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Product>>(
                    "product",
                    BuildInfo::get_product_for_reflect,
                    BuildInfo::mut_product_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ProductFlags>>(
                    "product_flags",
                    BuildInfo::get_product_flags_for_reflect,
                    BuildInfo::mut_product_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Platform>>(
                    "platform",
                    BuildInfo::get_platform_for_reflect,
                    BuildInfo::mut_platform_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "version",
                    BuildInfo::get_version_for_reflect,
                    BuildInfo::mut_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BuildInfo>(
                    "BuildInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BuildInfo {
    fn clear(&mut self) {
        self.clear_product();
        self.clear_product_flags();
        self.clear_platform();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BuildInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BuildInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LoginCryptoHelloUnion {
    // message fields
    diffie_hellman: ::protobuf::SingularPtrField<LoginCryptoDiffieHellmanHello>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LoginCryptoHelloUnion {}

impl LoginCryptoHelloUnion {
    pub fn new() -> LoginCryptoHelloUnion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LoginCryptoHelloUnion {
        static mut instance: ::protobuf::lazy::Lazy<LoginCryptoHelloUnion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LoginCryptoHelloUnion,
        };
        unsafe {
            instance.get(LoginCryptoHelloUnion::new)
        }
    }

    // optional .LoginCryptoDiffieHellmanHello diffie_hellman = 10;

    pub fn clear_diffie_hellman(&mut self) {
        self.diffie_hellman.clear();
    }

    pub fn has_diffie_hellman(&self) -> bool {
        self.diffie_hellman.is_some()
    }

    // Param is passed by value, moved
    pub fn set_diffie_hellman(&mut self, v: LoginCryptoDiffieHellmanHello) {
        self.diffie_hellman = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_diffie_hellman(&mut self) -> &mut LoginCryptoDiffieHellmanHello {
        if self.diffie_hellman.is_none() {
            self.diffie_hellman.set_default();
        }
        self.diffie_hellman.as_mut().unwrap()
    }

    // Take field
    pub fn take_diffie_hellman(&mut self) -> LoginCryptoDiffieHellmanHello {
        self.diffie_hellman.take().unwrap_or_else(|| LoginCryptoDiffieHellmanHello::new())
    }

    pub fn get_diffie_hellman(&self) -> &LoginCryptoDiffieHellmanHello {
        self.diffie_hellman.as_ref().unwrap_or_else(|| LoginCryptoDiffieHellmanHello::default_instance())
    }

    fn get_diffie_hellman_for_reflect(&self) -> &::protobuf::SingularPtrField<LoginCryptoDiffieHellmanHello> {
        &self.diffie_hellman
    }

    fn mut_diffie_hellman_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LoginCryptoDiffieHellmanHello> {
        &mut self.diffie_hellman
    }
}

impl ::protobuf::Message for LoginCryptoHelloUnion {
    fn is_initialized(&self) -> bool {
        for v in &self.diffie_hellman {
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
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.diffie_hellman)?;
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
        if let Some(ref v) = self.diffie_hellman.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.diffie_hellman.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for LoginCryptoHelloUnion {
    fn new() -> LoginCryptoHelloUnion {
        LoginCryptoHelloUnion::new()
    }

    fn descriptor_static(_: ::std::option::Option<LoginCryptoHelloUnion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LoginCryptoDiffieHellmanHello>>(
                    "diffie_hellman",
                    LoginCryptoHelloUnion::get_diffie_hellman_for_reflect,
                    LoginCryptoHelloUnion::mut_diffie_hellman_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LoginCryptoHelloUnion>(
                    "LoginCryptoHelloUnion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LoginCryptoHelloUnion {
    fn clear(&mut self) {
        self.clear_diffie_hellman();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LoginCryptoHelloUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoginCryptoHelloUnion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LoginCryptoDiffieHellmanHello {
    // message fields
    gc: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    server_keys_known: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LoginCryptoDiffieHellmanHello {}

impl LoginCryptoDiffieHellmanHello {
    pub fn new() -> LoginCryptoDiffieHellmanHello {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LoginCryptoDiffieHellmanHello {
        static mut instance: ::protobuf::lazy::Lazy<LoginCryptoDiffieHellmanHello> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LoginCryptoDiffieHellmanHello,
        };
        unsafe {
            instance.get(LoginCryptoDiffieHellmanHello::new)
        }
    }

    // required bytes gc = 10;

    pub fn clear_gc(&mut self) {
        self.gc.clear();
    }

    pub fn has_gc(&self) -> bool {
        self.gc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gc(&mut self, v: ::std::vec::Vec<u8>) {
        self.gc = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gc(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.gc.is_none() {
            self.gc.set_default();
        }
        self.gc.as_mut().unwrap()
    }

    // Take field
    pub fn take_gc(&mut self) -> ::std::vec::Vec<u8> {
        self.gc.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_gc(&self) -> &[u8] {
        match self.gc.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_gc_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.gc
    }

    fn mut_gc_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.gc
    }

    // required uint32 server_keys_known = 20;

    pub fn clear_server_keys_known(&mut self) {
        self.server_keys_known = ::std::option::Option::None;
    }

    pub fn has_server_keys_known(&self) -> bool {
        self.server_keys_known.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_keys_known(&mut self, v: u32) {
        self.server_keys_known = ::std::option::Option::Some(v);
    }

    pub fn get_server_keys_known(&self) -> u32 {
        self.server_keys_known.unwrap_or(0)
    }

    fn get_server_keys_known_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.server_keys_known
    }

    fn mut_server_keys_known_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.server_keys_known
    }
}

impl ::protobuf::Message for LoginCryptoDiffieHellmanHello {
    fn is_initialized(&self) -> bool {
        if self.gc.is_none() {
            return false;
        }
        if self.server_keys_known.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.gc)?;
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.server_keys_known = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.gc.as_ref() {
            my_size += ::protobuf::rt::bytes_size(10, &v);
        }
        if let Some(v) = self.server_keys_known {
            my_size += ::protobuf::rt::value_size(20, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.gc.as_ref() {
            os.write_bytes(10, &v)?;
        }
        if let Some(v) = self.server_keys_known {
            os.write_uint32(20, v)?;
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

impl ::protobuf::MessageStatic for LoginCryptoDiffieHellmanHello {
    fn new() -> LoginCryptoDiffieHellmanHello {
        LoginCryptoDiffieHellmanHello::new()
    }

    fn descriptor_static(_: ::std::option::Option<LoginCryptoDiffieHellmanHello>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "gc",
                    LoginCryptoDiffieHellmanHello::get_gc_for_reflect,
                    LoginCryptoDiffieHellmanHello::mut_gc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "server_keys_known",
                    LoginCryptoDiffieHellmanHello::get_server_keys_known_for_reflect,
                    LoginCryptoDiffieHellmanHello::mut_server_keys_known_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LoginCryptoDiffieHellmanHello>(
                    "LoginCryptoDiffieHellmanHello",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LoginCryptoDiffieHellmanHello {
    fn clear(&mut self) {
        self.clear_gc();
        self.clear_server_keys_known();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LoginCryptoDiffieHellmanHello {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoginCryptoDiffieHellmanHello {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FeatureSet {
    // message fields
    autoupdate2: ::std::option::Option<bool>,
    current_location: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FeatureSet {}

impl FeatureSet {
    pub fn new() -> FeatureSet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FeatureSet {
        static mut instance: ::protobuf::lazy::Lazy<FeatureSet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FeatureSet,
        };
        unsafe {
            instance.get(FeatureSet::new)
        }
    }

    // optional bool autoupdate2 = 1;

    pub fn clear_autoupdate2(&mut self) {
        self.autoupdate2 = ::std::option::Option::None;
    }

    pub fn has_autoupdate2(&self) -> bool {
        self.autoupdate2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_autoupdate2(&mut self, v: bool) {
        self.autoupdate2 = ::std::option::Option::Some(v);
    }

    pub fn get_autoupdate2(&self) -> bool {
        self.autoupdate2.unwrap_or(false)
    }

    fn get_autoupdate2_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.autoupdate2
    }

    fn mut_autoupdate2_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.autoupdate2
    }

    // optional bool current_location = 2;

    pub fn clear_current_location(&mut self) {
        self.current_location = ::std::option::Option::None;
    }

    pub fn has_current_location(&self) -> bool {
        self.current_location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_current_location(&mut self, v: bool) {
        self.current_location = ::std::option::Option::Some(v);
    }

    pub fn get_current_location(&self) -> bool {
        self.current_location.unwrap_or(false)
    }

    fn get_current_location_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.current_location
    }

    fn mut_current_location_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.current_location
    }
}

impl ::protobuf::Message for FeatureSet {
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
                    let tmp = is.read_bool()?;
                    self.autoupdate2 = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.current_location = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.autoupdate2 {
            my_size += 2;
        }
        if let Some(v) = self.current_location {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.autoupdate2 {
            os.write_bool(1, v)?;
        }
        if let Some(v) = self.current_location {
            os.write_bool(2, v)?;
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

impl ::protobuf::MessageStatic for FeatureSet {
    fn new() -> FeatureSet {
        FeatureSet::new()
    }

    fn descriptor_static(_: ::std::option::Option<FeatureSet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "autoupdate2",
                    FeatureSet::get_autoupdate2_for_reflect,
                    FeatureSet::mut_autoupdate2_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "current_location",
                    FeatureSet::get_current_location_for_reflect,
                    FeatureSet::mut_current_location_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FeatureSet>(
                    "FeatureSet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FeatureSet {
    fn clear(&mut self) {
        self.clear_autoupdate2();
        self.clear_current_location();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FeatureSet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FeatureSet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct APResponseMessage {
    // message fields
    challenge: ::protobuf::SingularPtrField<APChallenge>,
    upgrade: ::protobuf::SingularPtrField<UpgradeRequiredMessage>,
    login_failed: ::protobuf::SingularPtrField<APLoginFailed>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for APResponseMessage {}

impl APResponseMessage {
    pub fn new() -> APResponseMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static APResponseMessage {
        static mut instance: ::protobuf::lazy::Lazy<APResponseMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const APResponseMessage,
        };
        unsafe {
            instance.get(APResponseMessage::new)
        }
    }

    // optional .APChallenge challenge = 10;

    pub fn clear_challenge(&mut self) {
        self.challenge.clear();
    }

    pub fn has_challenge(&self) -> bool {
        self.challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_challenge(&mut self, v: APChallenge) {
        self.challenge = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_challenge(&mut self) -> &mut APChallenge {
        if self.challenge.is_none() {
            self.challenge.set_default();
        }
        self.challenge.as_mut().unwrap()
    }

    // Take field
    pub fn take_challenge(&mut self) -> APChallenge {
        self.challenge.take().unwrap_or_else(|| APChallenge::new())
    }

    pub fn get_challenge(&self) -> &APChallenge {
        self.challenge.as_ref().unwrap_or_else(|| APChallenge::default_instance())
    }

    fn get_challenge_for_reflect(&self) -> &::protobuf::SingularPtrField<APChallenge> {
        &self.challenge
    }

    fn mut_challenge_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<APChallenge> {
        &mut self.challenge
    }

    // optional .UpgradeRequiredMessage upgrade = 20;

    pub fn clear_upgrade(&mut self) {
        self.upgrade.clear();
    }

    pub fn has_upgrade(&self) -> bool {
        self.upgrade.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upgrade(&mut self, v: UpgradeRequiredMessage) {
        self.upgrade = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_upgrade(&mut self) -> &mut UpgradeRequiredMessage {
        if self.upgrade.is_none() {
            self.upgrade.set_default();
        }
        self.upgrade.as_mut().unwrap()
    }

    // Take field
    pub fn take_upgrade(&mut self) -> UpgradeRequiredMessage {
        self.upgrade.take().unwrap_or_else(|| UpgradeRequiredMessage::new())
    }

    pub fn get_upgrade(&self) -> &UpgradeRequiredMessage {
        self.upgrade.as_ref().unwrap_or_else(|| UpgradeRequiredMessage::default_instance())
    }

    fn get_upgrade_for_reflect(&self) -> &::protobuf::SingularPtrField<UpgradeRequiredMessage> {
        &self.upgrade
    }

    fn mut_upgrade_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<UpgradeRequiredMessage> {
        &mut self.upgrade
    }

    // optional .APLoginFailed login_failed = 30;

    pub fn clear_login_failed(&mut self) {
        self.login_failed.clear();
    }

    pub fn has_login_failed(&self) -> bool {
        self.login_failed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_login_failed(&mut self, v: APLoginFailed) {
        self.login_failed = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_login_failed(&mut self) -> &mut APLoginFailed {
        if self.login_failed.is_none() {
            self.login_failed.set_default();
        }
        self.login_failed.as_mut().unwrap()
    }

    // Take field
    pub fn take_login_failed(&mut self) -> APLoginFailed {
        self.login_failed.take().unwrap_or_else(|| APLoginFailed::new())
    }

    pub fn get_login_failed(&self) -> &APLoginFailed {
        self.login_failed.as_ref().unwrap_or_else(|| APLoginFailed::default_instance())
    }

    fn get_login_failed_for_reflect(&self) -> &::protobuf::SingularPtrField<APLoginFailed> {
        &self.login_failed
    }

    fn mut_login_failed_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<APLoginFailed> {
        &mut self.login_failed
    }
}

impl ::protobuf::Message for APResponseMessage {
    fn is_initialized(&self) -> bool {
        for v in &self.challenge {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.upgrade {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.login_failed {
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
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.challenge)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.upgrade)?;
                },
                30 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.login_failed)?;
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
        if let Some(ref v) = self.challenge.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.upgrade.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.login_failed.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.challenge.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.upgrade.as_ref() {
            os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.login_failed.as_ref() {
            os.write_tag(30, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for APResponseMessage {
    fn new() -> APResponseMessage {
        APResponseMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<APResponseMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<APChallenge>>(
                    "challenge",
                    APResponseMessage::get_challenge_for_reflect,
                    APResponseMessage::mut_challenge_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UpgradeRequiredMessage>>(
                    "upgrade",
                    APResponseMessage::get_upgrade_for_reflect,
                    APResponseMessage::mut_upgrade_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<APLoginFailed>>(
                    "login_failed",
                    APResponseMessage::get_login_failed_for_reflect,
                    APResponseMessage::mut_login_failed_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<APResponseMessage>(
                    "APResponseMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for APResponseMessage {
    fn clear(&mut self) {
        self.clear_challenge();
        self.clear_upgrade();
        self.clear_login_failed();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for APResponseMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for APResponseMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct APChallenge {
    // message fields
    login_crypto_challenge: ::protobuf::SingularPtrField<LoginCryptoChallengeUnion>,
    fingerprint_challenge: ::protobuf::SingularPtrField<FingerprintChallengeUnion>,
    pow_challenge: ::protobuf::SingularPtrField<PoWChallengeUnion>,
    crypto_challenge: ::protobuf::SingularPtrField<CryptoChallengeUnion>,
    server_nonce: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    padding: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for APChallenge {}

impl APChallenge {
    pub fn new() -> APChallenge {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static APChallenge {
        static mut instance: ::protobuf::lazy::Lazy<APChallenge> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const APChallenge,
        };
        unsafe {
            instance.get(APChallenge::new)
        }
    }

    // required .LoginCryptoChallengeUnion login_crypto_challenge = 10;

    pub fn clear_login_crypto_challenge(&mut self) {
        self.login_crypto_challenge.clear();
    }

    pub fn has_login_crypto_challenge(&self) -> bool {
        self.login_crypto_challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_login_crypto_challenge(&mut self, v: LoginCryptoChallengeUnion) {
        self.login_crypto_challenge = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_login_crypto_challenge(&mut self) -> &mut LoginCryptoChallengeUnion {
        if self.login_crypto_challenge.is_none() {
            self.login_crypto_challenge.set_default();
        }
        self.login_crypto_challenge.as_mut().unwrap()
    }

    // Take field
    pub fn take_login_crypto_challenge(&mut self) -> LoginCryptoChallengeUnion {
        self.login_crypto_challenge.take().unwrap_or_else(|| LoginCryptoChallengeUnion::new())
    }

    pub fn get_login_crypto_challenge(&self) -> &LoginCryptoChallengeUnion {
        self.login_crypto_challenge.as_ref().unwrap_or_else(|| LoginCryptoChallengeUnion::default_instance())
    }

    fn get_login_crypto_challenge_for_reflect(&self) -> &::protobuf::SingularPtrField<LoginCryptoChallengeUnion> {
        &self.login_crypto_challenge
    }

    fn mut_login_crypto_challenge_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LoginCryptoChallengeUnion> {
        &mut self.login_crypto_challenge
    }

    // required .FingerprintChallengeUnion fingerprint_challenge = 20;

    pub fn clear_fingerprint_challenge(&mut self) {
        self.fingerprint_challenge.clear();
    }

    pub fn has_fingerprint_challenge(&self) -> bool {
        self.fingerprint_challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fingerprint_challenge(&mut self, v: FingerprintChallengeUnion) {
        self.fingerprint_challenge = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fingerprint_challenge(&mut self) -> &mut FingerprintChallengeUnion {
        if self.fingerprint_challenge.is_none() {
            self.fingerprint_challenge.set_default();
        }
        self.fingerprint_challenge.as_mut().unwrap()
    }

    // Take field
    pub fn take_fingerprint_challenge(&mut self) -> FingerprintChallengeUnion {
        self.fingerprint_challenge.take().unwrap_or_else(|| FingerprintChallengeUnion::new())
    }

    pub fn get_fingerprint_challenge(&self) -> &FingerprintChallengeUnion {
        self.fingerprint_challenge.as_ref().unwrap_or_else(|| FingerprintChallengeUnion::default_instance())
    }

    fn get_fingerprint_challenge_for_reflect(&self) -> &::protobuf::SingularPtrField<FingerprintChallengeUnion> {
        &self.fingerprint_challenge
    }

    fn mut_fingerprint_challenge_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FingerprintChallengeUnion> {
        &mut self.fingerprint_challenge
    }

    // required .PoWChallengeUnion pow_challenge = 30;

    pub fn clear_pow_challenge(&mut self) {
        self.pow_challenge.clear();
    }

    pub fn has_pow_challenge(&self) -> bool {
        self.pow_challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pow_challenge(&mut self, v: PoWChallengeUnion) {
        self.pow_challenge = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pow_challenge(&mut self) -> &mut PoWChallengeUnion {
        if self.pow_challenge.is_none() {
            self.pow_challenge.set_default();
        }
        self.pow_challenge.as_mut().unwrap()
    }

    // Take field
    pub fn take_pow_challenge(&mut self) -> PoWChallengeUnion {
        self.pow_challenge.take().unwrap_or_else(|| PoWChallengeUnion::new())
    }

    pub fn get_pow_challenge(&self) -> &PoWChallengeUnion {
        self.pow_challenge.as_ref().unwrap_or_else(|| PoWChallengeUnion::default_instance())
    }

    fn get_pow_challenge_for_reflect(&self) -> &::protobuf::SingularPtrField<PoWChallengeUnion> {
        &self.pow_challenge
    }

    fn mut_pow_challenge_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PoWChallengeUnion> {
        &mut self.pow_challenge
    }

    // required .CryptoChallengeUnion crypto_challenge = 40;

    pub fn clear_crypto_challenge(&mut self) {
        self.crypto_challenge.clear();
    }

    pub fn has_crypto_challenge(&self) -> bool {
        self.crypto_challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_crypto_challenge(&mut self, v: CryptoChallengeUnion) {
        self.crypto_challenge = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_crypto_challenge(&mut self) -> &mut CryptoChallengeUnion {
        if self.crypto_challenge.is_none() {
            self.crypto_challenge.set_default();
        }
        self.crypto_challenge.as_mut().unwrap()
    }

    // Take field
    pub fn take_crypto_challenge(&mut self) -> CryptoChallengeUnion {
        self.crypto_challenge.take().unwrap_or_else(|| CryptoChallengeUnion::new())
    }

    pub fn get_crypto_challenge(&self) -> &CryptoChallengeUnion {
        self.crypto_challenge.as_ref().unwrap_or_else(|| CryptoChallengeUnion::default_instance())
    }

    fn get_crypto_challenge_for_reflect(&self) -> &::protobuf::SingularPtrField<CryptoChallengeUnion> {
        &self.crypto_challenge
    }

    fn mut_crypto_challenge_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CryptoChallengeUnion> {
        &mut self.crypto_challenge
    }

    // required bytes server_nonce = 50;

    pub fn clear_server_nonce(&mut self) {
        self.server_nonce.clear();
    }

    pub fn has_server_nonce(&self) -> bool {
        self.server_nonce.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_nonce(&mut self, v: ::std::vec::Vec<u8>) {
        self.server_nonce = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_server_nonce(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.server_nonce.is_none() {
            self.server_nonce.set_default();
        }
        self.server_nonce.as_mut().unwrap()
    }

    // Take field
    pub fn take_server_nonce(&mut self) -> ::std::vec::Vec<u8> {
        self.server_nonce.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_server_nonce(&self) -> &[u8] {
        match self.server_nonce.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_server_nonce_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.server_nonce
    }

    fn mut_server_nonce_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.server_nonce
    }

    // optional bytes padding = 60;

    pub fn clear_padding(&mut self) {
        self.padding.clear();
    }

    pub fn has_padding(&self) -> bool {
        self.padding.is_some()
    }

    // Param is passed by value, moved
    pub fn set_padding(&mut self, v: ::std::vec::Vec<u8>) {
        self.padding = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_padding(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.padding.is_none() {
            self.padding.set_default();
        }
        self.padding.as_mut().unwrap()
    }

    // Take field
    pub fn take_padding(&mut self) -> ::std::vec::Vec<u8> {
        self.padding.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_padding(&self) -> &[u8] {
        match self.padding.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_padding_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.padding
    }

    fn mut_padding_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.padding
    }
}

impl ::protobuf::Message for APChallenge {
    fn is_initialized(&self) -> bool {
        if self.login_crypto_challenge.is_none() {
            return false;
        }
        if self.fingerprint_challenge.is_none() {
            return false;
        }
        if self.pow_challenge.is_none() {
            return false;
        }
        if self.crypto_challenge.is_none() {
            return false;
        }
        if self.server_nonce.is_none() {
            return false;
        }
        for v in &self.login_crypto_challenge {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.fingerprint_challenge {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.pow_challenge {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.crypto_challenge {
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
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.login_crypto_challenge)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fingerprint_challenge)?;
                },
                30 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pow_challenge)?;
                },
                40 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.crypto_challenge)?;
                },
                50 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.server_nonce)?;
                },
                60 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.padding)?;
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
        if let Some(ref v) = self.login_crypto_challenge.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.fingerprint_challenge.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.pow_challenge.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.crypto_challenge.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.server_nonce.as_ref() {
            my_size += ::protobuf::rt::bytes_size(50, &v);
        }
        if let Some(ref v) = self.padding.as_ref() {
            my_size += ::protobuf::rt::bytes_size(60, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.login_crypto_challenge.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.fingerprint_challenge.as_ref() {
            os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.pow_challenge.as_ref() {
            os.write_tag(30, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.crypto_challenge.as_ref() {
            os.write_tag(40, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.server_nonce.as_ref() {
            os.write_bytes(50, &v)?;
        }
        if let Some(ref v) = self.padding.as_ref() {
            os.write_bytes(60, &v)?;
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

impl ::protobuf::MessageStatic for APChallenge {
    fn new() -> APChallenge {
        APChallenge::new()
    }

    fn descriptor_static(_: ::std::option::Option<APChallenge>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LoginCryptoChallengeUnion>>(
                    "login_crypto_challenge",
                    APChallenge::get_login_crypto_challenge_for_reflect,
                    APChallenge::mut_login_crypto_challenge_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FingerprintChallengeUnion>>(
                    "fingerprint_challenge",
                    APChallenge::get_fingerprint_challenge_for_reflect,
                    APChallenge::mut_fingerprint_challenge_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PoWChallengeUnion>>(
                    "pow_challenge",
                    APChallenge::get_pow_challenge_for_reflect,
                    APChallenge::mut_pow_challenge_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CryptoChallengeUnion>>(
                    "crypto_challenge",
                    APChallenge::get_crypto_challenge_for_reflect,
                    APChallenge::mut_crypto_challenge_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "server_nonce",
                    APChallenge::get_server_nonce_for_reflect,
                    APChallenge::mut_server_nonce_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "padding",
                    APChallenge::get_padding_for_reflect,
                    APChallenge::mut_padding_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<APChallenge>(
                    "APChallenge",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for APChallenge {
    fn clear(&mut self) {
        self.clear_login_crypto_challenge();
        self.clear_fingerprint_challenge();
        self.clear_pow_challenge();
        self.clear_crypto_challenge();
        self.clear_server_nonce();
        self.clear_padding();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for APChallenge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for APChallenge {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LoginCryptoChallengeUnion {
    // message fields
    diffie_hellman: ::protobuf::SingularPtrField<LoginCryptoDiffieHellmanChallenge>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LoginCryptoChallengeUnion {}

impl LoginCryptoChallengeUnion {
    pub fn new() -> LoginCryptoChallengeUnion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LoginCryptoChallengeUnion {
        static mut instance: ::protobuf::lazy::Lazy<LoginCryptoChallengeUnion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LoginCryptoChallengeUnion,
        };
        unsafe {
            instance.get(LoginCryptoChallengeUnion::new)
        }
    }

    // optional .LoginCryptoDiffieHellmanChallenge diffie_hellman = 10;

    pub fn clear_diffie_hellman(&mut self) {
        self.diffie_hellman.clear();
    }

    pub fn has_diffie_hellman(&self) -> bool {
        self.diffie_hellman.is_some()
    }

    // Param is passed by value, moved
    pub fn set_diffie_hellman(&mut self, v: LoginCryptoDiffieHellmanChallenge) {
        self.diffie_hellman = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_diffie_hellman(&mut self) -> &mut LoginCryptoDiffieHellmanChallenge {
        if self.diffie_hellman.is_none() {
            self.diffie_hellman.set_default();
        }
        self.diffie_hellman.as_mut().unwrap()
    }

    // Take field
    pub fn take_diffie_hellman(&mut self) -> LoginCryptoDiffieHellmanChallenge {
        self.diffie_hellman.take().unwrap_or_else(|| LoginCryptoDiffieHellmanChallenge::new())
    }

    pub fn get_diffie_hellman(&self) -> &LoginCryptoDiffieHellmanChallenge {
        self.diffie_hellman.as_ref().unwrap_or_else(|| LoginCryptoDiffieHellmanChallenge::default_instance())
    }

    fn get_diffie_hellman_for_reflect(&self) -> &::protobuf::SingularPtrField<LoginCryptoDiffieHellmanChallenge> {
        &self.diffie_hellman
    }

    fn mut_diffie_hellman_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LoginCryptoDiffieHellmanChallenge> {
        &mut self.diffie_hellman
    }
}

impl ::protobuf::Message for LoginCryptoChallengeUnion {
    fn is_initialized(&self) -> bool {
        for v in &self.diffie_hellman {
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
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.diffie_hellman)?;
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
        if let Some(ref v) = self.diffie_hellman.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.diffie_hellman.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for LoginCryptoChallengeUnion {
    fn new() -> LoginCryptoChallengeUnion {
        LoginCryptoChallengeUnion::new()
    }

    fn descriptor_static(_: ::std::option::Option<LoginCryptoChallengeUnion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LoginCryptoDiffieHellmanChallenge>>(
                    "diffie_hellman",
                    LoginCryptoChallengeUnion::get_diffie_hellman_for_reflect,
                    LoginCryptoChallengeUnion::mut_diffie_hellman_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LoginCryptoChallengeUnion>(
                    "LoginCryptoChallengeUnion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LoginCryptoChallengeUnion {
    fn clear(&mut self) {
        self.clear_diffie_hellman();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LoginCryptoChallengeUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoginCryptoChallengeUnion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LoginCryptoDiffieHellmanChallenge {
    // message fields
    gs: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    server_signature_key: ::std::option::Option<i32>,
    gs_signature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LoginCryptoDiffieHellmanChallenge {}

impl LoginCryptoDiffieHellmanChallenge {
    pub fn new() -> LoginCryptoDiffieHellmanChallenge {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LoginCryptoDiffieHellmanChallenge {
        static mut instance: ::protobuf::lazy::Lazy<LoginCryptoDiffieHellmanChallenge> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LoginCryptoDiffieHellmanChallenge,
        };
        unsafe {
            instance.get(LoginCryptoDiffieHellmanChallenge::new)
        }
    }

    // required bytes gs = 10;

    pub fn clear_gs(&mut self) {
        self.gs.clear();
    }

    pub fn has_gs(&self) -> bool {
        self.gs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gs(&mut self, v: ::std::vec::Vec<u8>) {
        self.gs = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gs(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.gs.is_none() {
            self.gs.set_default();
        }
        self.gs.as_mut().unwrap()
    }

    // Take field
    pub fn take_gs(&mut self) -> ::std::vec::Vec<u8> {
        self.gs.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_gs(&self) -> &[u8] {
        match self.gs.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_gs_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.gs
    }

    fn mut_gs_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.gs
    }

    // required int32 server_signature_key = 20;

    pub fn clear_server_signature_key(&mut self) {
        self.server_signature_key = ::std::option::Option::None;
    }

    pub fn has_server_signature_key(&self) -> bool {
        self.server_signature_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_signature_key(&mut self, v: i32) {
        self.server_signature_key = ::std::option::Option::Some(v);
    }

    pub fn get_server_signature_key(&self) -> i32 {
        self.server_signature_key.unwrap_or(0)
    }

    fn get_server_signature_key_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.server_signature_key
    }

    fn mut_server_signature_key_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.server_signature_key
    }

    // required bytes gs_signature = 30;

    pub fn clear_gs_signature(&mut self) {
        self.gs_signature.clear();
    }

    pub fn has_gs_signature(&self) -> bool {
        self.gs_signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gs_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.gs_signature = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gs_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.gs_signature.is_none() {
            self.gs_signature.set_default();
        }
        self.gs_signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_gs_signature(&mut self) -> ::std::vec::Vec<u8> {
        self.gs_signature.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_gs_signature(&self) -> &[u8] {
        match self.gs_signature.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_gs_signature_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.gs_signature
    }

    fn mut_gs_signature_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.gs_signature
    }
}

impl ::protobuf::Message for LoginCryptoDiffieHellmanChallenge {
    fn is_initialized(&self) -> bool {
        if self.gs.is_none() {
            return false;
        }
        if self.server_signature_key.is_none() {
            return false;
        }
        if self.gs_signature.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.gs)?;
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.server_signature_key = ::std::option::Option::Some(tmp);
                },
                30 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.gs_signature)?;
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
        if let Some(ref v) = self.gs.as_ref() {
            my_size += ::protobuf::rt::bytes_size(10, &v);
        }
        if let Some(v) = self.server_signature_key {
            my_size += ::protobuf::rt::value_size(20, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.gs_signature.as_ref() {
            my_size += ::protobuf::rt::bytes_size(30, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.gs.as_ref() {
            os.write_bytes(10, &v)?;
        }
        if let Some(v) = self.server_signature_key {
            os.write_int32(20, v)?;
        }
        if let Some(ref v) = self.gs_signature.as_ref() {
            os.write_bytes(30, &v)?;
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

impl ::protobuf::MessageStatic for LoginCryptoDiffieHellmanChallenge {
    fn new() -> LoginCryptoDiffieHellmanChallenge {
        LoginCryptoDiffieHellmanChallenge::new()
    }

    fn descriptor_static(_: ::std::option::Option<LoginCryptoDiffieHellmanChallenge>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "gs",
                    LoginCryptoDiffieHellmanChallenge::get_gs_for_reflect,
                    LoginCryptoDiffieHellmanChallenge::mut_gs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "server_signature_key",
                    LoginCryptoDiffieHellmanChallenge::get_server_signature_key_for_reflect,
                    LoginCryptoDiffieHellmanChallenge::mut_server_signature_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "gs_signature",
                    LoginCryptoDiffieHellmanChallenge::get_gs_signature_for_reflect,
                    LoginCryptoDiffieHellmanChallenge::mut_gs_signature_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LoginCryptoDiffieHellmanChallenge>(
                    "LoginCryptoDiffieHellmanChallenge",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LoginCryptoDiffieHellmanChallenge {
    fn clear(&mut self) {
        self.clear_gs();
        self.clear_server_signature_key();
        self.clear_gs_signature();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LoginCryptoDiffieHellmanChallenge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoginCryptoDiffieHellmanChallenge {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FingerprintChallengeUnion {
    // message fields
    grain: ::protobuf::SingularPtrField<FingerprintGrainChallenge>,
    hmac_ripemd: ::protobuf::SingularPtrField<FingerprintHmacRipemdChallenge>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FingerprintChallengeUnion {}

impl FingerprintChallengeUnion {
    pub fn new() -> FingerprintChallengeUnion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FingerprintChallengeUnion {
        static mut instance: ::protobuf::lazy::Lazy<FingerprintChallengeUnion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FingerprintChallengeUnion,
        };
        unsafe {
            instance.get(FingerprintChallengeUnion::new)
        }
    }

    // optional .FingerprintGrainChallenge grain = 10;

    pub fn clear_grain(&mut self) {
        self.grain.clear();
    }

    pub fn has_grain(&self) -> bool {
        self.grain.is_some()
    }

    // Param is passed by value, moved
    pub fn set_grain(&mut self, v: FingerprintGrainChallenge) {
        self.grain = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_grain(&mut self) -> &mut FingerprintGrainChallenge {
        if self.grain.is_none() {
            self.grain.set_default();
        }
        self.grain.as_mut().unwrap()
    }

    // Take field
    pub fn take_grain(&mut self) -> FingerprintGrainChallenge {
        self.grain.take().unwrap_or_else(|| FingerprintGrainChallenge::new())
    }

    pub fn get_grain(&self) -> &FingerprintGrainChallenge {
        self.grain.as_ref().unwrap_or_else(|| FingerprintGrainChallenge::default_instance())
    }

    fn get_grain_for_reflect(&self) -> &::protobuf::SingularPtrField<FingerprintGrainChallenge> {
        &self.grain
    }

    fn mut_grain_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FingerprintGrainChallenge> {
        &mut self.grain
    }

    // optional .FingerprintHmacRipemdChallenge hmac_ripemd = 20;

    pub fn clear_hmac_ripemd(&mut self) {
        self.hmac_ripemd.clear();
    }

    pub fn has_hmac_ripemd(&self) -> bool {
        self.hmac_ripemd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hmac_ripemd(&mut self, v: FingerprintHmacRipemdChallenge) {
        self.hmac_ripemd = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hmac_ripemd(&mut self) -> &mut FingerprintHmacRipemdChallenge {
        if self.hmac_ripemd.is_none() {
            self.hmac_ripemd.set_default();
        }
        self.hmac_ripemd.as_mut().unwrap()
    }

    // Take field
    pub fn take_hmac_ripemd(&mut self) -> FingerprintHmacRipemdChallenge {
        self.hmac_ripemd.take().unwrap_or_else(|| FingerprintHmacRipemdChallenge::new())
    }

    pub fn get_hmac_ripemd(&self) -> &FingerprintHmacRipemdChallenge {
        self.hmac_ripemd.as_ref().unwrap_or_else(|| FingerprintHmacRipemdChallenge::default_instance())
    }

    fn get_hmac_ripemd_for_reflect(&self) -> &::protobuf::SingularPtrField<FingerprintHmacRipemdChallenge> {
        &self.hmac_ripemd
    }

    fn mut_hmac_ripemd_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FingerprintHmacRipemdChallenge> {
        &mut self.hmac_ripemd
    }
}

impl ::protobuf::Message for FingerprintChallengeUnion {
    fn is_initialized(&self) -> bool {
        for v in &self.grain {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.hmac_ripemd {
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
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.grain)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.hmac_ripemd)?;
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
        if let Some(ref v) = self.grain.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.hmac_ripemd.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.grain.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.hmac_ripemd.as_ref() {
            os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for FingerprintChallengeUnion {
    fn new() -> FingerprintChallengeUnion {
        FingerprintChallengeUnion::new()
    }

    fn descriptor_static(_: ::std::option::Option<FingerprintChallengeUnion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FingerprintGrainChallenge>>(
                    "grain",
                    FingerprintChallengeUnion::get_grain_for_reflect,
                    FingerprintChallengeUnion::mut_grain_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FingerprintHmacRipemdChallenge>>(
                    "hmac_ripemd",
                    FingerprintChallengeUnion::get_hmac_ripemd_for_reflect,
                    FingerprintChallengeUnion::mut_hmac_ripemd_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FingerprintChallengeUnion>(
                    "FingerprintChallengeUnion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FingerprintChallengeUnion {
    fn clear(&mut self) {
        self.clear_grain();
        self.clear_hmac_ripemd();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FingerprintChallengeUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FingerprintChallengeUnion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FingerprintGrainChallenge {
    // message fields
    kek: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FingerprintGrainChallenge {}

impl FingerprintGrainChallenge {
    pub fn new() -> FingerprintGrainChallenge {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FingerprintGrainChallenge {
        static mut instance: ::protobuf::lazy::Lazy<FingerprintGrainChallenge> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FingerprintGrainChallenge,
        };
        unsafe {
            instance.get(FingerprintGrainChallenge::new)
        }
    }

    // required bytes kek = 10;

    pub fn clear_kek(&mut self) {
        self.kek.clear();
    }

    pub fn has_kek(&self) -> bool {
        self.kek.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kek(&mut self, v: ::std::vec::Vec<u8>) {
        self.kek = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_kek(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.kek.is_none() {
            self.kek.set_default();
        }
        self.kek.as_mut().unwrap()
    }

    // Take field
    pub fn take_kek(&mut self) -> ::std::vec::Vec<u8> {
        self.kek.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_kek(&self) -> &[u8] {
        match self.kek.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_kek_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.kek
    }

    fn mut_kek_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.kek
    }
}

impl ::protobuf::Message for FingerprintGrainChallenge {
    fn is_initialized(&self) -> bool {
        if self.kek.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.kek)?;
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
        if let Some(ref v) = self.kek.as_ref() {
            my_size += ::protobuf::rt::bytes_size(10, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.kek.as_ref() {
            os.write_bytes(10, &v)?;
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

impl ::protobuf::MessageStatic for FingerprintGrainChallenge {
    fn new() -> FingerprintGrainChallenge {
        FingerprintGrainChallenge::new()
    }

    fn descriptor_static(_: ::std::option::Option<FingerprintGrainChallenge>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "kek",
                    FingerprintGrainChallenge::get_kek_for_reflect,
                    FingerprintGrainChallenge::mut_kek_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FingerprintGrainChallenge>(
                    "FingerprintGrainChallenge",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FingerprintGrainChallenge {
    fn clear(&mut self) {
        self.clear_kek();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FingerprintGrainChallenge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FingerprintGrainChallenge {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FingerprintHmacRipemdChallenge {
    // message fields
    challenge: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FingerprintHmacRipemdChallenge {}

impl FingerprintHmacRipemdChallenge {
    pub fn new() -> FingerprintHmacRipemdChallenge {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FingerprintHmacRipemdChallenge {
        static mut instance: ::protobuf::lazy::Lazy<FingerprintHmacRipemdChallenge> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FingerprintHmacRipemdChallenge,
        };
        unsafe {
            instance.get(FingerprintHmacRipemdChallenge::new)
        }
    }

    // required bytes challenge = 10;

    pub fn clear_challenge(&mut self) {
        self.challenge.clear();
    }

    pub fn has_challenge(&self) -> bool {
        self.challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_challenge(&mut self, v: ::std::vec::Vec<u8>) {
        self.challenge = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_challenge(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.challenge.is_none() {
            self.challenge.set_default();
        }
        self.challenge.as_mut().unwrap()
    }

    // Take field
    pub fn take_challenge(&mut self) -> ::std::vec::Vec<u8> {
        self.challenge.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_challenge(&self) -> &[u8] {
        match self.challenge.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_challenge_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.challenge
    }

    fn mut_challenge_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.challenge
    }
}

impl ::protobuf::Message for FingerprintHmacRipemdChallenge {
    fn is_initialized(&self) -> bool {
        if self.challenge.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.challenge)?;
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
        if let Some(ref v) = self.challenge.as_ref() {
            my_size += ::protobuf::rt::bytes_size(10, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.challenge.as_ref() {
            os.write_bytes(10, &v)?;
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

impl ::protobuf::MessageStatic for FingerprintHmacRipemdChallenge {
    fn new() -> FingerprintHmacRipemdChallenge {
        FingerprintHmacRipemdChallenge::new()
    }

    fn descriptor_static(_: ::std::option::Option<FingerprintHmacRipemdChallenge>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "challenge",
                    FingerprintHmacRipemdChallenge::get_challenge_for_reflect,
                    FingerprintHmacRipemdChallenge::mut_challenge_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FingerprintHmacRipemdChallenge>(
                    "FingerprintHmacRipemdChallenge",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FingerprintHmacRipemdChallenge {
    fn clear(&mut self) {
        self.clear_challenge();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FingerprintHmacRipemdChallenge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FingerprintHmacRipemdChallenge {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PoWChallengeUnion {
    // message fields
    hash_cash: ::protobuf::SingularPtrField<PoWHashCashChallenge>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PoWChallengeUnion {}

impl PoWChallengeUnion {
    pub fn new() -> PoWChallengeUnion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PoWChallengeUnion {
        static mut instance: ::protobuf::lazy::Lazy<PoWChallengeUnion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PoWChallengeUnion,
        };
        unsafe {
            instance.get(PoWChallengeUnion::new)
        }
    }

    // optional .PoWHashCashChallenge hash_cash = 10;

    pub fn clear_hash_cash(&mut self) {
        self.hash_cash.clear();
    }

    pub fn has_hash_cash(&self) -> bool {
        self.hash_cash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hash_cash(&mut self, v: PoWHashCashChallenge) {
        self.hash_cash = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash_cash(&mut self) -> &mut PoWHashCashChallenge {
        if self.hash_cash.is_none() {
            self.hash_cash.set_default();
        }
        self.hash_cash.as_mut().unwrap()
    }

    // Take field
    pub fn take_hash_cash(&mut self) -> PoWHashCashChallenge {
        self.hash_cash.take().unwrap_or_else(|| PoWHashCashChallenge::new())
    }

    pub fn get_hash_cash(&self) -> &PoWHashCashChallenge {
        self.hash_cash.as_ref().unwrap_or_else(|| PoWHashCashChallenge::default_instance())
    }

    fn get_hash_cash_for_reflect(&self) -> &::protobuf::SingularPtrField<PoWHashCashChallenge> {
        &self.hash_cash
    }

    fn mut_hash_cash_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PoWHashCashChallenge> {
        &mut self.hash_cash
    }
}

impl ::protobuf::Message for PoWChallengeUnion {
    fn is_initialized(&self) -> bool {
        for v in &self.hash_cash {
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
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.hash_cash)?;
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
        if let Some(ref v) = self.hash_cash.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.hash_cash.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for PoWChallengeUnion {
    fn new() -> PoWChallengeUnion {
        PoWChallengeUnion::new()
    }

    fn descriptor_static(_: ::std::option::Option<PoWChallengeUnion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PoWHashCashChallenge>>(
                    "hash_cash",
                    PoWChallengeUnion::get_hash_cash_for_reflect,
                    PoWChallengeUnion::mut_hash_cash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PoWChallengeUnion>(
                    "PoWChallengeUnion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PoWChallengeUnion {
    fn clear(&mut self) {
        self.clear_hash_cash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PoWChallengeUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PoWChallengeUnion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PoWHashCashChallenge {
    // message fields
    prefix: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    length: ::std::option::Option<i32>,
    target: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PoWHashCashChallenge {}

impl PoWHashCashChallenge {
    pub fn new() -> PoWHashCashChallenge {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PoWHashCashChallenge {
        static mut instance: ::protobuf::lazy::Lazy<PoWHashCashChallenge> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PoWHashCashChallenge,
        };
        unsafe {
            instance.get(PoWHashCashChallenge::new)
        }
    }

    // optional bytes prefix = 10;

    pub fn clear_prefix(&mut self) {
        self.prefix.clear();
    }

    pub fn has_prefix(&self) -> bool {
        self.prefix.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prefix(&mut self, v: ::std::vec::Vec<u8>) {
        self.prefix = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_prefix(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.prefix.is_none() {
            self.prefix.set_default();
        }
        self.prefix.as_mut().unwrap()
    }

    // Take field
    pub fn take_prefix(&mut self) -> ::std::vec::Vec<u8> {
        self.prefix.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_prefix(&self) -> &[u8] {
        match self.prefix.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_prefix_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.prefix
    }

    fn mut_prefix_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.prefix
    }

    // optional int32 length = 20;

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: i32) {
        self.length = ::std::option::Option::Some(v);
    }

    pub fn get_length(&self) -> i32 {
        self.length.unwrap_or(0)
    }

    fn get_length_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.length
    }

    fn mut_length_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.length
    }

    // optional int32 target = 30;

    pub fn clear_target(&mut self) {
        self.target = ::std::option::Option::None;
    }

    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target(&mut self, v: i32) {
        self.target = ::std::option::Option::Some(v);
    }

    pub fn get_target(&self) -> i32 {
        self.target.unwrap_or(0)
    }

    fn get_target_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.target
    }

    fn mut_target_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.target
    }
}

impl ::protobuf::Message for PoWHashCashChallenge {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.prefix)?;
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.length = ::std::option::Option::Some(tmp);
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.target = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.prefix.as_ref() {
            my_size += ::protobuf::rt::bytes_size(10, &v);
        }
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::value_size(20, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.target {
            my_size += ::protobuf::rt::value_size(30, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.prefix.as_ref() {
            os.write_bytes(10, &v)?;
        }
        if let Some(v) = self.length {
            os.write_int32(20, v)?;
        }
        if let Some(v) = self.target {
            os.write_int32(30, v)?;
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

impl ::protobuf::MessageStatic for PoWHashCashChallenge {
    fn new() -> PoWHashCashChallenge {
        PoWHashCashChallenge::new()
    }

    fn descriptor_static(_: ::std::option::Option<PoWHashCashChallenge>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "prefix",
                    PoWHashCashChallenge::get_prefix_for_reflect,
                    PoWHashCashChallenge::mut_prefix_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "length",
                    PoWHashCashChallenge::get_length_for_reflect,
                    PoWHashCashChallenge::mut_length_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "target",
                    PoWHashCashChallenge::get_target_for_reflect,
                    PoWHashCashChallenge::mut_target_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PoWHashCashChallenge>(
                    "PoWHashCashChallenge",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PoWHashCashChallenge {
    fn clear(&mut self) {
        self.clear_prefix();
        self.clear_length();
        self.clear_target();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PoWHashCashChallenge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PoWHashCashChallenge {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CryptoChallengeUnion {
    // message fields
    shannon: ::protobuf::SingularPtrField<CryptoShannonChallenge>,
    rc4_sha1_hmac: ::protobuf::SingularPtrField<CryptoRc4Sha1HmacChallenge>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CryptoChallengeUnion {}

impl CryptoChallengeUnion {
    pub fn new() -> CryptoChallengeUnion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CryptoChallengeUnion {
        static mut instance: ::protobuf::lazy::Lazy<CryptoChallengeUnion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CryptoChallengeUnion,
        };
        unsafe {
            instance.get(CryptoChallengeUnion::new)
        }
    }

    // optional .CryptoShannonChallenge shannon = 10;

    pub fn clear_shannon(&mut self) {
        self.shannon.clear();
    }

    pub fn has_shannon(&self) -> bool {
        self.shannon.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shannon(&mut self, v: CryptoShannonChallenge) {
        self.shannon = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_shannon(&mut self) -> &mut CryptoShannonChallenge {
        if self.shannon.is_none() {
            self.shannon.set_default();
        }
        self.shannon.as_mut().unwrap()
    }

    // Take field
    pub fn take_shannon(&mut self) -> CryptoShannonChallenge {
        self.shannon.take().unwrap_or_else(|| CryptoShannonChallenge::new())
    }

    pub fn get_shannon(&self) -> &CryptoShannonChallenge {
        self.shannon.as_ref().unwrap_or_else(|| CryptoShannonChallenge::default_instance())
    }

    fn get_shannon_for_reflect(&self) -> &::protobuf::SingularPtrField<CryptoShannonChallenge> {
        &self.shannon
    }

    fn mut_shannon_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CryptoShannonChallenge> {
        &mut self.shannon
    }

    // optional .CryptoRc4Sha1HmacChallenge rc4_sha1_hmac = 20;

    pub fn clear_rc4_sha1_hmac(&mut self) {
        self.rc4_sha1_hmac.clear();
    }

    pub fn has_rc4_sha1_hmac(&self) -> bool {
        self.rc4_sha1_hmac.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rc4_sha1_hmac(&mut self, v: CryptoRc4Sha1HmacChallenge) {
        self.rc4_sha1_hmac = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rc4_sha1_hmac(&mut self) -> &mut CryptoRc4Sha1HmacChallenge {
        if self.rc4_sha1_hmac.is_none() {
            self.rc4_sha1_hmac.set_default();
        }
        self.rc4_sha1_hmac.as_mut().unwrap()
    }

    // Take field
    pub fn take_rc4_sha1_hmac(&mut self) -> CryptoRc4Sha1HmacChallenge {
        self.rc4_sha1_hmac.take().unwrap_or_else(|| CryptoRc4Sha1HmacChallenge::new())
    }

    pub fn get_rc4_sha1_hmac(&self) -> &CryptoRc4Sha1HmacChallenge {
        self.rc4_sha1_hmac.as_ref().unwrap_or_else(|| CryptoRc4Sha1HmacChallenge::default_instance())
    }

    fn get_rc4_sha1_hmac_for_reflect(&self) -> &::protobuf::SingularPtrField<CryptoRc4Sha1HmacChallenge> {
        &self.rc4_sha1_hmac
    }

    fn mut_rc4_sha1_hmac_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CryptoRc4Sha1HmacChallenge> {
        &mut self.rc4_sha1_hmac
    }
}

impl ::protobuf::Message for CryptoChallengeUnion {
    fn is_initialized(&self) -> bool {
        for v in &self.shannon {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.rc4_sha1_hmac {
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
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.shannon)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.rc4_sha1_hmac)?;
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
        if let Some(ref v) = self.shannon.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.rc4_sha1_hmac.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.shannon.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.rc4_sha1_hmac.as_ref() {
            os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CryptoChallengeUnion {
    fn new() -> CryptoChallengeUnion {
        CryptoChallengeUnion::new()
    }

    fn descriptor_static(_: ::std::option::Option<CryptoChallengeUnion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CryptoShannonChallenge>>(
                    "shannon",
                    CryptoChallengeUnion::get_shannon_for_reflect,
                    CryptoChallengeUnion::mut_shannon_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CryptoRc4Sha1HmacChallenge>>(
                    "rc4_sha1_hmac",
                    CryptoChallengeUnion::get_rc4_sha1_hmac_for_reflect,
                    CryptoChallengeUnion::mut_rc4_sha1_hmac_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CryptoChallengeUnion>(
                    "CryptoChallengeUnion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CryptoChallengeUnion {
    fn clear(&mut self) {
        self.clear_shannon();
        self.clear_rc4_sha1_hmac();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CryptoChallengeUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CryptoChallengeUnion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CryptoShannonChallenge {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CryptoShannonChallenge {}

impl CryptoShannonChallenge {
    pub fn new() -> CryptoShannonChallenge {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CryptoShannonChallenge {
        static mut instance: ::protobuf::lazy::Lazy<CryptoShannonChallenge> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CryptoShannonChallenge,
        };
        unsafe {
            instance.get(CryptoShannonChallenge::new)
        }
    }
}

impl ::protobuf::Message for CryptoShannonChallenge {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for CryptoShannonChallenge {
    fn new() -> CryptoShannonChallenge {
        CryptoShannonChallenge::new()
    }

    fn descriptor_static(_: ::std::option::Option<CryptoShannonChallenge>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CryptoShannonChallenge>(
                    "CryptoShannonChallenge",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CryptoShannonChallenge {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CryptoShannonChallenge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CryptoShannonChallenge {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CryptoRc4Sha1HmacChallenge {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CryptoRc4Sha1HmacChallenge {}

impl CryptoRc4Sha1HmacChallenge {
    pub fn new() -> CryptoRc4Sha1HmacChallenge {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CryptoRc4Sha1HmacChallenge {
        static mut instance: ::protobuf::lazy::Lazy<CryptoRc4Sha1HmacChallenge> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CryptoRc4Sha1HmacChallenge,
        };
        unsafe {
            instance.get(CryptoRc4Sha1HmacChallenge::new)
        }
    }
}

impl ::protobuf::Message for CryptoRc4Sha1HmacChallenge {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for CryptoRc4Sha1HmacChallenge {
    fn new() -> CryptoRc4Sha1HmacChallenge {
        CryptoRc4Sha1HmacChallenge::new()
    }

    fn descriptor_static(_: ::std::option::Option<CryptoRc4Sha1HmacChallenge>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CryptoRc4Sha1HmacChallenge>(
                    "CryptoRc4Sha1HmacChallenge",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CryptoRc4Sha1HmacChallenge {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CryptoRc4Sha1HmacChallenge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CryptoRc4Sha1HmacChallenge {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UpgradeRequiredMessage {
    // message fields
    upgrade_signed_part: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    signature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    http_suffix: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UpgradeRequiredMessage {}

impl UpgradeRequiredMessage {
    pub fn new() -> UpgradeRequiredMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UpgradeRequiredMessage {
        static mut instance: ::protobuf::lazy::Lazy<UpgradeRequiredMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UpgradeRequiredMessage,
        };
        unsafe {
            instance.get(UpgradeRequiredMessage::new)
        }
    }

    // required bytes upgrade_signed_part = 10;

    pub fn clear_upgrade_signed_part(&mut self) {
        self.upgrade_signed_part.clear();
    }

    pub fn has_upgrade_signed_part(&self) -> bool {
        self.upgrade_signed_part.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upgrade_signed_part(&mut self, v: ::std::vec::Vec<u8>) {
        self.upgrade_signed_part = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_upgrade_signed_part(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.upgrade_signed_part.is_none() {
            self.upgrade_signed_part.set_default();
        }
        self.upgrade_signed_part.as_mut().unwrap()
    }

    // Take field
    pub fn take_upgrade_signed_part(&mut self) -> ::std::vec::Vec<u8> {
        self.upgrade_signed_part.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_upgrade_signed_part(&self) -> &[u8] {
        match self.upgrade_signed_part.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_upgrade_signed_part_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.upgrade_signed_part
    }

    fn mut_upgrade_signed_part_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.upgrade_signed_part
    }

    // required bytes signature = 20;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    pub fn has_signature(&self) -> bool {
        self.signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.signature = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.signature.is_none() {
            self.signature.set_default();
        }
        self.signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_signature(&mut self) -> ::std::vec::Vec<u8> {
        self.signature.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_signature(&self) -> &[u8] {
        match self.signature.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_signature_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.signature
    }

    // optional string http_suffix = 30;

    pub fn clear_http_suffix(&mut self) {
        self.http_suffix.clear();
    }

    pub fn has_http_suffix(&self) -> bool {
        self.http_suffix.is_some()
    }

    // Param is passed by value, moved
    pub fn set_http_suffix(&mut self, v: ::std::string::String) {
        self.http_suffix = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_http_suffix(&mut self) -> &mut ::std::string::String {
        if self.http_suffix.is_none() {
            self.http_suffix.set_default();
        }
        self.http_suffix.as_mut().unwrap()
    }

    // Take field
    pub fn take_http_suffix(&mut self) -> ::std::string::String {
        self.http_suffix.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_http_suffix(&self) -> &str {
        match self.http_suffix.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_http_suffix_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.http_suffix
    }

    fn mut_http_suffix_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.http_suffix
    }
}

impl ::protobuf::Message for UpgradeRequiredMessage {
    fn is_initialized(&self) -> bool {
        if self.upgrade_signed_part.is_none() {
            return false;
        }
        if self.signature.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.upgrade_signed_part)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.signature)?;
                },
                30 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.http_suffix)?;
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
        if let Some(ref v) = self.upgrade_signed_part.as_ref() {
            my_size += ::protobuf::rt::bytes_size(10, &v);
        }
        if let Some(ref v) = self.signature.as_ref() {
            my_size += ::protobuf::rt::bytes_size(20, &v);
        }
        if let Some(ref v) = self.http_suffix.as_ref() {
            my_size += ::protobuf::rt::string_size(30, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.upgrade_signed_part.as_ref() {
            os.write_bytes(10, &v)?;
        }
        if let Some(ref v) = self.signature.as_ref() {
            os.write_bytes(20, &v)?;
        }
        if let Some(ref v) = self.http_suffix.as_ref() {
            os.write_string(30, &v)?;
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

impl ::protobuf::MessageStatic for UpgradeRequiredMessage {
    fn new() -> UpgradeRequiredMessage {
        UpgradeRequiredMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<UpgradeRequiredMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "upgrade_signed_part",
                    UpgradeRequiredMessage::get_upgrade_signed_part_for_reflect,
                    UpgradeRequiredMessage::mut_upgrade_signed_part_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    UpgradeRequiredMessage::get_signature_for_reflect,
                    UpgradeRequiredMessage::mut_signature_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "http_suffix",
                    UpgradeRequiredMessage::get_http_suffix_for_reflect,
                    UpgradeRequiredMessage::mut_http_suffix_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UpgradeRequiredMessage>(
                    "UpgradeRequiredMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UpgradeRequiredMessage {
    fn clear(&mut self) {
        self.clear_upgrade_signed_part();
        self.clear_signature();
        self.clear_http_suffix();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UpgradeRequiredMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpgradeRequiredMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct APLoginFailed {
    // message fields
    error_code: ::std::option::Option<ErrorCode>,
    retry_delay: ::std::option::Option<i32>,
    expiry: ::std::option::Option<i32>,
    error_description: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for APLoginFailed {}

impl APLoginFailed {
    pub fn new() -> APLoginFailed {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static APLoginFailed {
        static mut instance: ::protobuf::lazy::Lazy<APLoginFailed> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const APLoginFailed,
        };
        unsafe {
            instance.get(APLoginFailed::new)
        }
    }

    // required .ErrorCode error_code = 10;

    pub fn clear_error_code(&mut self) {
        self.error_code = ::std::option::Option::None;
    }

    pub fn has_error_code(&self) -> bool {
        self.error_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error_code(&mut self, v: ErrorCode) {
        self.error_code = ::std::option::Option::Some(v);
    }

    pub fn get_error_code(&self) -> ErrorCode {
        self.error_code.unwrap_or(ErrorCode::ProtocolError)
    }

    fn get_error_code_for_reflect(&self) -> &::std::option::Option<ErrorCode> {
        &self.error_code
    }

    fn mut_error_code_for_reflect(&mut self) -> &mut ::std::option::Option<ErrorCode> {
        &mut self.error_code
    }

    // optional int32 retry_delay = 20;

    pub fn clear_retry_delay(&mut self) {
        self.retry_delay = ::std::option::Option::None;
    }

    pub fn has_retry_delay(&self) -> bool {
        self.retry_delay.is_some()
    }

    // Param is passed by value, moved
    pub fn set_retry_delay(&mut self, v: i32) {
        self.retry_delay = ::std::option::Option::Some(v);
    }

    pub fn get_retry_delay(&self) -> i32 {
        self.retry_delay.unwrap_or(0)
    }

    fn get_retry_delay_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.retry_delay
    }

    fn mut_retry_delay_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.retry_delay
    }

    // optional int32 expiry = 30;

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

    // optional string error_description = 40;

    pub fn clear_error_description(&mut self) {
        self.error_description.clear();
    }

    pub fn has_error_description(&self) -> bool {
        self.error_description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error_description(&mut self, v: ::std::string::String) {
        self.error_description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error_description(&mut self) -> &mut ::std::string::String {
        if self.error_description.is_none() {
            self.error_description.set_default();
        }
        self.error_description.as_mut().unwrap()
    }

    // Take field
    pub fn take_error_description(&mut self) -> ::std::string::String {
        self.error_description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error_description(&self) -> &str {
        match self.error_description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_error_description_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.error_description
    }

    fn mut_error_description_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.error_description
    }
}

impl ::protobuf::Message for APLoginFailed {
    fn is_initialized(&self) -> bool {
        if self.error_code.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.error_code = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.retry_delay = ::std::option::Option::Some(tmp);
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.expiry = ::std::option::Option::Some(tmp);
                },
                40 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error_description)?;
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
        if let Some(v) = self.error_code {
            my_size += ::protobuf::rt::enum_size(10, v);
        }
        if let Some(v) = self.retry_delay {
            my_size += ::protobuf::rt::value_size(20, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.expiry {
            my_size += ::protobuf::rt::value_size(30, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.error_description.as_ref() {
            my_size += ::protobuf::rt::string_size(40, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error_code {
            os.write_enum(10, v.value())?;
        }
        if let Some(v) = self.retry_delay {
            os.write_int32(20, v)?;
        }
        if let Some(v) = self.expiry {
            os.write_int32(30, v)?;
        }
        if let Some(ref v) = self.error_description.as_ref() {
            os.write_string(40, &v)?;
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

impl ::protobuf::MessageStatic for APLoginFailed {
    fn new() -> APLoginFailed {
        APLoginFailed::new()
    }

    fn descriptor_static(_: ::std::option::Option<APLoginFailed>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ErrorCode>>(
                    "error_code",
                    APLoginFailed::get_error_code_for_reflect,
                    APLoginFailed::mut_error_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "retry_delay",
                    APLoginFailed::get_retry_delay_for_reflect,
                    APLoginFailed::mut_retry_delay_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "expiry",
                    APLoginFailed::get_expiry_for_reflect,
                    APLoginFailed::mut_expiry_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error_description",
                    APLoginFailed::get_error_description_for_reflect,
                    APLoginFailed::mut_error_description_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<APLoginFailed>(
                    "APLoginFailed",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for APLoginFailed {
    fn clear(&mut self) {
        self.clear_error_code();
        self.clear_retry_delay();
        self.clear_expiry();
        self.clear_error_description();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for APLoginFailed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for APLoginFailed {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClientResponsePlaintext {
    // message fields
    login_crypto_response: ::protobuf::SingularPtrField<LoginCryptoResponseUnion>,
    pow_response: ::protobuf::SingularPtrField<PoWResponseUnion>,
    crypto_response: ::protobuf::SingularPtrField<CryptoResponseUnion>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientResponsePlaintext {}

impl ClientResponsePlaintext {
    pub fn new() -> ClientResponsePlaintext {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientResponsePlaintext {
        static mut instance: ::protobuf::lazy::Lazy<ClientResponsePlaintext> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientResponsePlaintext,
        };
        unsafe {
            instance.get(ClientResponsePlaintext::new)
        }
    }

    // required .LoginCryptoResponseUnion login_crypto_response = 10;

    pub fn clear_login_crypto_response(&mut self) {
        self.login_crypto_response.clear();
    }

    pub fn has_login_crypto_response(&self) -> bool {
        self.login_crypto_response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_login_crypto_response(&mut self, v: LoginCryptoResponseUnion) {
        self.login_crypto_response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_login_crypto_response(&mut self) -> &mut LoginCryptoResponseUnion {
        if self.login_crypto_response.is_none() {
            self.login_crypto_response.set_default();
        }
        self.login_crypto_response.as_mut().unwrap()
    }

    // Take field
    pub fn take_login_crypto_response(&mut self) -> LoginCryptoResponseUnion {
        self.login_crypto_response.take().unwrap_or_else(|| LoginCryptoResponseUnion::new())
    }

    pub fn get_login_crypto_response(&self) -> &LoginCryptoResponseUnion {
        self.login_crypto_response.as_ref().unwrap_or_else(|| LoginCryptoResponseUnion::default_instance())
    }

    fn get_login_crypto_response_for_reflect(&self) -> &::protobuf::SingularPtrField<LoginCryptoResponseUnion> {
        &self.login_crypto_response
    }

    fn mut_login_crypto_response_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LoginCryptoResponseUnion> {
        &mut self.login_crypto_response
    }

    // required .PoWResponseUnion pow_response = 20;

    pub fn clear_pow_response(&mut self) {
        self.pow_response.clear();
    }

    pub fn has_pow_response(&self) -> bool {
        self.pow_response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pow_response(&mut self, v: PoWResponseUnion) {
        self.pow_response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pow_response(&mut self) -> &mut PoWResponseUnion {
        if self.pow_response.is_none() {
            self.pow_response.set_default();
        }
        self.pow_response.as_mut().unwrap()
    }

    // Take field
    pub fn take_pow_response(&mut self) -> PoWResponseUnion {
        self.pow_response.take().unwrap_or_else(|| PoWResponseUnion::new())
    }

    pub fn get_pow_response(&self) -> &PoWResponseUnion {
        self.pow_response.as_ref().unwrap_or_else(|| PoWResponseUnion::default_instance())
    }

    fn get_pow_response_for_reflect(&self) -> &::protobuf::SingularPtrField<PoWResponseUnion> {
        &self.pow_response
    }

    fn mut_pow_response_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PoWResponseUnion> {
        &mut self.pow_response
    }

    // required .CryptoResponseUnion crypto_response = 30;

    pub fn clear_crypto_response(&mut self) {
        self.crypto_response.clear();
    }

    pub fn has_crypto_response(&self) -> bool {
        self.crypto_response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_crypto_response(&mut self, v: CryptoResponseUnion) {
        self.crypto_response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_crypto_response(&mut self) -> &mut CryptoResponseUnion {
        if self.crypto_response.is_none() {
            self.crypto_response.set_default();
        }
        self.crypto_response.as_mut().unwrap()
    }

    // Take field
    pub fn take_crypto_response(&mut self) -> CryptoResponseUnion {
        self.crypto_response.take().unwrap_or_else(|| CryptoResponseUnion::new())
    }

    pub fn get_crypto_response(&self) -> &CryptoResponseUnion {
        self.crypto_response.as_ref().unwrap_or_else(|| CryptoResponseUnion::default_instance())
    }

    fn get_crypto_response_for_reflect(&self) -> &::protobuf::SingularPtrField<CryptoResponseUnion> {
        &self.crypto_response
    }

    fn mut_crypto_response_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CryptoResponseUnion> {
        &mut self.crypto_response
    }
}

impl ::protobuf::Message for ClientResponsePlaintext {
    fn is_initialized(&self) -> bool {
        if self.login_crypto_response.is_none() {
            return false;
        }
        if self.pow_response.is_none() {
            return false;
        }
        if self.crypto_response.is_none() {
            return false;
        }
        for v in &self.login_crypto_response {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.pow_response {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.crypto_response {
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
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.login_crypto_response)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pow_response)?;
                },
                30 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.crypto_response)?;
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
        if let Some(ref v) = self.login_crypto_response.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.pow_response.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.crypto_response.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.login_crypto_response.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.pow_response.as_ref() {
            os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.crypto_response.as_ref() {
            os.write_tag(30, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ClientResponsePlaintext {
    fn new() -> ClientResponsePlaintext {
        ClientResponsePlaintext::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientResponsePlaintext>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LoginCryptoResponseUnion>>(
                    "login_crypto_response",
                    ClientResponsePlaintext::get_login_crypto_response_for_reflect,
                    ClientResponsePlaintext::mut_login_crypto_response_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PoWResponseUnion>>(
                    "pow_response",
                    ClientResponsePlaintext::get_pow_response_for_reflect,
                    ClientResponsePlaintext::mut_pow_response_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CryptoResponseUnion>>(
                    "crypto_response",
                    ClientResponsePlaintext::get_crypto_response_for_reflect,
                    ClientResponsePlaintext::mut_crypto_response_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientResponsePlaintext>(
                    "ClientResponsePlaintext",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientResponsePlaintext {
    fn clear(&mut self) {
        self.clear_login_crypto_response();
        self.clear_pow_response();
        self.clear_crypto_response();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClientResponsePlaintext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientResponsePlaintext {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LoginCryptoResponseUnion {
    // message fields
    diffie_hellman: ::protobuf::SingularPtrField<LoginCryptoDiffieHellmanResponse>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LoginCryptoResponseUnion {}

impl LoginCryptoResponseUnion {
    pub fn new() -> LoginCryptoResponseUnion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LoginCryptoResponseUnion {
        static mut instance: ::protobuf::lazy::Lazy<LoginCryptoResponseUnion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LoginCryptoResponseUnion,
        };
        unsafe {
            instance.get(LoginCryptoResponseUnion::new)
        }
    }

    // optional .LoginCryptoDiffieHellmanResponse diffie_hellman = 10;

    pub fn clear_diffie_hellman(&mut self) {
        self.diffie_hellman.clear();
    }

    pub fn has_diffie_hellman(&self) -> bool {
        self.diffie_hellman.is_some()
    }

    // Param is passed by value, moved
    pub fn set_diffie_hellman(&mut self, v: LoginCryptoDiffieHellmanResponse) {
        self.diffie_hellman = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_diffie_hellman(&mut self) -> &mut LoginCryptoDiffieHellmanResponse {
        if self.diffie_hellman.is_none() {
            self.diffie_hellman.set_default();
        }
        self.diffie_hellman.as_mut().unwrap()
    }

    // Take field
    pub fn take_diffie_hellman(&mut self) -> LoginCryptoDiffieHellmanResponse {
        self.diffie_hellman.take().unwrap_or_else(|| LoginCryptoDiffieHellmanResponse::new())
    }

    pub fn get_diffie_hellman(&self) -> &LoginCryptoDiffieHellmanResponse {
        self.diffie_hellman.as_ref().unwrap_or_else(|| LoginCryptoDiffieHellmanResponse::default_instance())
    }

    fn get_diffie_hellman_for_reflect(&self) -> &::protobuf::SingularPtrField<LoginCryptoDiffieHellmanResponse> {
        &self.diffie_hellman
    }

    fn mut_diffie_hellman_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LoginCryptoDiffieHellmanResponse> {
        &mut self.diffie_hellman
    }
}

impl ::protobuf::Message for LoginCryptoResponseUnion {
    fn is_initialized(&self) -> bool {
        for v in &self.diffie_hellman {
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
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.diffie_hellman)?;
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
        if let Some(ref v) = self.diffie_hellman.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.diffie_hellman.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for LoginCryptoResponseUnion {
    fn new() -> LoginCryptoResponseUnion {
        LoginCryptoResponseUnion::new()
    }

    fn descriptor_static(_: ::std::option::Option<LoginCryptoResponseUnion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LoginCryptoDiffieHellmanResponse>>(
                    "diffie_hellman",
                    LoginCryptoResponseUnion::get_diffie_hellman_for_reflect,
                    LoginCryptoResponseUnion::mut_diffie_hellman_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LoginCryptoResponseUnion>(
                    "LoginCryptoResponseUnion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LoginCryptoResponseUnion {
    fn clear(&mut self) {
        self.clear_diffie_hellman();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LoginCryptoResponseUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoginCryptoResponseUnion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LoginCryptoDiffieHellmanResponse {
    // message fields
    hmac: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LoginCryptoDiffieHellmanResponse {}

impl LoginCryptoDiffieHellmanResponse {
    pub fn new() -> LoginCryptoDiffieHellmanResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LoginCryptoDiffieHellmanResponse {
        static mut instance: ::protobuf::lazy::Lazy<LoginCryptoDiffieHellmanResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LoginCryptoDiffieHellmanResponse,
        };
        unsafe {
            instance.get(LoginCryptoDiffieHellmanResponse::new)
        }
    }

    // required bytes hmac = 10;

    pub fn clear_hmac(&mut self) {
        self.hmac.clear();
    }

    pub fn has_hmac(&self) -> bool {
        self.hmac.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hmac(&mut self, v: ::std::vec::Vec<u8>) {
        self.hmac = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hmac(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.hmac.is_none() {
            self.hmac.set_default();
        }
        self.hmac.as_mut().unwrap()
    }

    // Take field
    pub fn take_hmac(&mut self) -> ::std::vec::Vec<u8> {
        self.hmac.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_hmac(&self) -> &[u8] {
        match self.hmac.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_hmac_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.hmac
    }

    fn mut_hmac_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.hmac
    }
}

impl ::protobuf::Message for LoginCryptoDiffieHellmanResponse {
    fn is_initialized(&self) -> bool {
        if self.hmac.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.hmac)?;
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
        if let Some(ref v) = self.hmac.as_ref() {
            my_size += ::protobuf::rt::bytes_size(10, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.hmac.as_ref() {
            os.write_bytes(10, &v)?;
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

impl ::protobuf::MessageStatic for LoginCryptoDiffieHellmanResponse {
    fn new() -> LoginCryptoDiffieHellmanResponse {
        LoginCryptoDiffieHellmanResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<LoginCryptoDiffieHellmanResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hmac",
                    LoginCryptoDiffieHellmanResponse::get_hmac_for_reflect,
                    LoginCryptoDiffieHellmanResponse::mut_hmac_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LoginCryptoDiffieHellmanResponse>(
                    "LoginCryptoDiffieHellmanResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LoginCryptoDiffieHellmanResponse {
    fn clear(&mut self) {
        self.clear_hmac();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LoginCryptoDiffieHellmanResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LoginCryptoDiffieHellmanResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PoWResponseUnion {
    // message fields
    hash_cash: ::protobuf::SingularPtrField<PoWHashCashResponse>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PoWResponseUnion {}

impl PoWResponseUnion {
    pub fn new() -> PoWResponseUnion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PoWResponseUnion {
        static mut instance: ::protobuf::lazy::Lazy<PoWResponseUnion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PoWResponseUnion,
        };
        unsafe {
            instance.get(PoWResponseUnion::new)
        }
    }

    // optional .PoWHashCashResponse hash_cash = 10;

    pub fn clear_hash_cash(&mut self) {
        self.hash_cash.clear();
    }

    pub fn has_hash_cash(&self) -> bool {
        self.hash_cash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hash_cash(&mut self, v: PoWHashCashResponse) {
        self.hash_cash = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash_cash(&mut self) -> &mut PoWHashCashResponse {
        if self.hash_cash.is_none() {
            self.hash_cash.set_default();
        }
        self.hash_cash.as_mut().unwrap()
    }

    // Take field
    pub fn take_hash_cash(&mut self) -> PoWHashCashResponse {
        self.hash_cash.take().unwrap_or_else(|| PoWHashCashResponse::new())
    }

    pub fn get_hash_cash(&self) -> &PoWHashCashResponse {
        self.hash_cash.as_ref().unwrap_or_else(|| PoWHashCashResponse::default_instance())
    }

    fn get_hash_cash_for_reflect(&self) -> &::protobuf::SingularPtrField<PoWHashCashResponse> {
        &self.hash_cash
    }

    fn mut_hash_cash_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PoWHashCashResponse> {
        &mut self.hash_cash
    }
}

impl ::protobuf::Message for PoWResponseUnion {
    fn is_initialized(&self) -> bool {
        for v in &self.hash_cash {
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
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.hash_cash)?;
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
        if let Some(ref v) = self.hash_cash.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.hash_cash.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for PoWResponseUnion {
    fn new() -> PoWResponseUnion {
        PoWResponseUnion::new()
    }

    fn descriptor_static(_: ::std::option::Option<PoWResponseUnion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PoWHashCashResponse>>(
                    "hash_cash",
                    PoWResponseUnion::get_hash_cash_for_reflect,
                    PoWResponseUnion::mut_hash_cash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PoWResponseUnion>(
                    "PoWResponseUnion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PoWResponseUnion {
    fn clear(&mut self) {
        self.clear_hash_cash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PoWResponseUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PoWResponseUnion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PoWHashCashResponse {
    // message fields
    hash_suffix: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PoWHashCashResponse {}

impl PoWHashCashResponse {
    pub fn new() -> PoWHashCashResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PoWHashCashResponse {
        static mut instance: ::protobuf::lazy::Lazy<PoWHashCashResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PoWHashCashResponse,
        };
        unsafe {
            instance.get(PoWHashCashResponse::new)
        }
    }

    // required bytes hash_suffix = 10;

    pub fn clear_hash_suffix(&mut self) {
        self.hash_suffix.clear();
    }

    pub fn has_hash_suffix(&self) -> bool {
        self.hash_suffix.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hash_suffix(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash_suffix = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash_suffix(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.hash_suffix.is_none() {
            self.hash_suffix.set_default();
        }
        self.hash_suffix.as_mut().unwrap()
    }

    // Take field
    pub fn take_hash_suffix(&mut self) -> ::std::vec::Vec<u8> {
        self.hash_suffix.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_hash_suffix(&self) -> &[u8] {
        match self.hash_suffix.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_hash_suffix_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.hash_suffix
    }

    fn mut_hash_suffix_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.hash_suffix
    }
}

impl ::protobuf::Message for PoWHashCashResponse {
    fn is_initialized(&self) -> bool {
        if self.hash_suffix.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                10 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.hash_suffix)?;
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
        if let Some(ref v) = self.hash_suffix.as_ref() {
            my_size += ::protobuf::rt::bytes_size(10, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.hash_suffix.as_ref() {
            os.write_bytes(10, &v)?;
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

impl ::protobuf::MessageStatic for PoWHashCashResponse {
    fn new() -> PoWHashCashResponse {
        PoWHashCashResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<PoWHashCashResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash_suffix",
                    PoWHashCashResponse::get_hash_suffix_for_reflect,
                    PoWHashCashResponse::mut_hash_suffix_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PoWHashCashResponse>(
                    "PoWHashCashResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PoWHashCashResponse {
    fn clear(&mut self) {
        self.clear_hash_suffix();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PoWHashCashResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PoWHashCashResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CryptoResponseUnion {
    // message fields
    shannon: ::protobuf::SingularPtrField<CryptoShannonResponse>,
    rc4_sha1_hmac: ::protobuf::SingularPtrField<CryptoRc4Sha1HmacResponse>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CryptoResponseUnion {}

impl CryptoResponseUnion {
    pub fn new() -> CryptoResponseUnion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CryptoResponseUnion {
        static mut instance: ::protobuf::lazy::Lazy<CryptoResponseUnion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CryptoResponseUnion,
        };
        unsafe {
            instance.get(CryptoResponseUnion::new)
        }
    }

    // optional .CryptoShannonResponse shannon = 10;

    pub fn clear_shannon(&mut self) {
        self.shannon.clear();
    }

    pub fn has_shannon(&self) -> bool {
        self.shannon.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shannon(&mut self, v: CryptoShannonResponse) {
        self.shannon = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_shannon(&mut self) -> &mut CryptoShannonResponse {
        if self.shannon.is_none() {
            self.shannon.set_default();
        }
        self.shannon.as_mut().unwrap()
    }

    // Take field
    pub fn take_shannon(&mut self) -> CryptoShannonResponse {
        self.shannon.take().unwrap_or_else(|| CryptoShannonResponse::new())
    }

    pub fn get_shannon(&self) -> &CryptoShannonResponse {
        self.shannon.as_ref().unwrap_or_else(|| CryptoShannonResponse::default_instance())
    }

    fn get_shannon_for_reflect(&self) -> &::protobuf::SingularPtrField<CryptoShannonResponse> {
        &self.shannon
    }

    fn mut_shannon_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CryptoShannonResponse> {
        &mut self.shannon
    }

    // optional .CryptoRc4Sha1HmacResponse rc4_sha1_hmac = 20;

    pub fn clear_rc4_sha1_hmac(&mut self) {
        self.rc4_sha1_hmac.clear();
    }

    pub fn has_rc4_sha1_hmac(&self) -> bool {
        self.rc4_sha1_hmac.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rc4_sha1_hmac(&mut self, v: CryptoRc4Sha1HmacResponse) {
        self.rc4_sha1_hmac = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rc4_sha1_hmac(&mut self) -> &mut CryptoRc4Sha1HmacResponse {
        if self.rc4_sha1_hmac.is_none() {
            self.rc4_sha1_hmac.set_default();
        }
        self.rc4_sha1_hmac.as_mut().unwrap()
    }

    // Take field
    pub fn take_rc4_sha1_hmac(&mut self) -> CryptoRc4Sha1HmacResponse {
        self.rc4_sha1_hmac.take().unwrap_or_else(|| CryptoRc4Sha1HmacResponse::new())
    }

    pub fn get_rc4_sha1_hmac(&self) -> &CryptoRc4Sha1HmacResponse {
        self.rc4_sha1_hmac.as_ref().unwrap_or_else(|| CryptoRc4Sha1HmacResponse::default_instance())
    }

    fn get_rc4_sha1_hmac_for_reflect(&self) -> &::protobuf::SingularPtrField<CryptoRc4Sha1HmacResponse> {
        &self.rc4_sha1_hmac
    }

    fn mut_rc4_sha1_hmac_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CryptoRc4Sha1HmacResponse> {
        &mut self.rc4_sha1_hmac
    }
}

impl ::protobuf::Message for CryptoResponseUnion {
    fn is_initialized(&self) -> bool {
        for v in &self.shannon {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.rc4_sha1_hmac {
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
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.shannon)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.rc4_sha1_hmac)?;
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
        if let Some(ref v) = self.shannon.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.rc4_sha1_hmac.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.shannon.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.rc4_sha1_hmac.as_ref() {
            os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CryptoResponseUnion {
    fn new() -> CryptoResponseUnion {
        CryptoResponseUnion::new()
    }

    fn descriptor_static(_: ::std::option::Option<CryptoResponseUnion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CryptoShannonResponse>>(
                    "shannon",
                    CryptoResponseUnion::get_shannon_for_reflect,
                    CryptoResponseUnion::mut_shannon_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CryptoRc4Sha1HmacResponse>>(
                    "rc4_sha1_hmac",
                    CryptoResponseUnion::get_rc4_sha1_hmac_for_reflect,
                    CryptoResponseUnion::mut_rc4_sha1_hmac_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CryptoResponseUnion>(
                    "CryptoResponseUnion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CryptoResponseUnion {
    fn clear(&mut self) {
        self.clear_shannon();
        self.clear_rc4_sha1_hmac();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CryptoResponseUnion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CryptoResponseUnion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CryptoShannonResponse {
    // message fields
    dummy: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CryptoShannonResponse {}

impl CryptoShannonResponse {
    pub fn new() -> CryptoShannonResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CryptoShannonResponse {
        static mut instance: ::protobuf::lazy::Lazy<CryptoShannonResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CryptoShannonResponse,
        };
        unsafe {
            instance.get(CryptoShannonResponse::new)
        }
    }

    // optional int32 dummy = 1;

    pub fn clear_dummy(&mut self) {
        self.dummy = ::std::option::Option::None;
    }

    pub fn has_dummy(&self) -> bool {
        self.dummy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dummy(&mut self, v: i32) {
        self.dummy = ::std::option::Option::Some(v);
    }

    pub fn get_dummy(&self) -> i32 {
        self.dummy.unwrap_or(0)
    }

    fn get_dummy_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dummy
    }

    fn mut_dummy_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dummy
    }
}

impl ::protobuf::Message for CryptoShannonResponse {
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
                    let tmp = is.read_int32()?;
                    self.dummy = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.dummy {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dummy {
            os.write_int32(1, v)?;
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

impl ::protobuf::MessageStatic for CryptoShannonResponse {
    fn new() -> CryptoShannonResponse {
        CryptoShannonResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CryptoShannonResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "dummy",
                    CryptoShannonResponse::get_dummy_for_reflect,
                    CryptoShannonResponse::mut_dummy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CryptoShannonResponse>(
                    "CryptoShannonResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CryptoShannonResponse {
    fn clear(&mut self) {
        self.clear_dummy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CryptoShannonResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CryptoShannonResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CryptoRc4Sha1HmacResponse {
    // message fields
    dummy: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CryptoRc4Sha1HmacResponse {}

impl CryptoRc4Sha1HmacResponse {
    pub fn new() -> CryptoRc4Sha1HmacResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CryptoRc4Sha1HmacResponse {
        static mut instance: ::protobuf::lazy::Lazy<CryptoRc4Sha1HmacResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CryptoRc4Sha1HmacResponse,
        };
        unsafe {
            instance.get(CryptoRc4Sha1HmacResponse::new)
        }
    }

    // optional int32 dummy = 1;

    pub fn clear_dummy(&mut self) {
        self.dummy = ::std::option::Option::None;
    }

    pub fn has_dummy(&self) -> bool {
        self.dummy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dummy(&mut self, v: i32) {
        self.dummy = ::std::option::Option::Some(v);
    }

    pub fn get_dummy(&self) -> i32 {
        self.dummy.unwrap_or(0)
    }

    fn get_dummy_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dummy
    }

    fn mut_dummy_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dummy
    }
}

impl ::protobuf::Message for CryptoRc4Sha1HmacResponse {
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
                    let tmp = is.read_int32()?;
                    self.dummy = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.dummy {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dummy {
            os.write_int32(1, v)?;
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

impl ::protobuf::MessageStatic for CryptoRc4Sha1HmacResponse {
    fn new() -> CryptoRc4Sha1HmacResponse {
        CryptoRc4Sha1HmacResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CryptoRc4Sha1HmacResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "dummy",
                    CryptoRc4Sha1HmacResponse::get_dummy_for_reflect,
                    CryptoRc4Sha1HmacResponse::mut_dummy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CryptoRc4Sha1HmacResponse>(
                    "CryptoRc4Sha1HmacResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CryptoRc4Sha1HmacResponse {
    fn clear(&mut self) {
        self.clear_dummy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CryptoRc4Sha1HmacResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CryptoRc4Sha1HmacResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Product {
    PRODUCT_CLIENT = 0,
    PRODUCT_LIBSPOTIFY = 1,
    PRODUCT_MOBILE = 2,
    PRODUCT_PARTNER = 3,
    PRODUCT_LIBSPOTIFY_EMBEDDED = 5,
}

impl ::protobuf::ProtobufEnum for Product {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Product> {
        match value {
            0 => ::std::option::Option::Some(Product::PRODUCT_CLIENT),
            1 => ::std::option::Option::Some(Product::PRODUCT_LIBSPOTIFY),
            2 => ::std::option::Option::Some(Product::PRODUCT_MOBILE),
            3 => ::std::option::Option::Some(Product::PRODUCT_PARTNER),
            5 => ::std::option::Option::Some(Product::PRODUCT_LIBSPOTIFY_EMBEDDED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Product] = &[
            Product::PRODUCT_CLIENT,
            Product::PRODUCT_LIBSPOTIFY,
            Product::PRODUCT_MOBILE,
            Product::PRODUCT_PARTNER,
            Product::PRODUCT_LIBSPOTIFY_EMBEDDED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Product>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Product", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Product {
}

impl ::protobuf::reflect::ProtobufValue for Product {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ProductFlags {
    PRODUCT_FLAG_NONE = 0,
    PRODUCT_FLAG_DEV_BUILD = 1,
}

impl ::protobuf::ProtobufEnum for ProductFlags {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ProductFlags> {
        match value {
            0 => ::std::option::Option::Some(ProductFlags::PRODUCT_FLAG_NONE),
            1 => ::std::option::Option::Some(ProductFlags::PRODUCT_FLAG_DEV_BUILD),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ProductFlags] = &[
            ProductFlags::PRODUCT_FLAG_NONE,
            ProductFlags::PRODUCT_FLAG_DEV_BUILD,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ProductFlags>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ProductFlags", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ProductFlags {
}

impl ::protobuf::reflect::ProtobufValue for ProductFlags {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Platform {
    PLATFORM_WIN32_X86 = 0,
    PLATFORM_OSX_X86 = 1,
    PLATFORM_LINUX_X86 = 2,
    PLATFORM_IPHONE_ARM = 3,
    PLATFORM_S60_ARM = 4,
    PLATFORM_OSX_PPC = 5,
    PLATFORM_ANDROID_ARM = 6,
    PLATFORM_WINDOWS_CE_ARM = 7,
    PLATFORM_LINUX_X86_64 = 8,
    PLATFORM_OSX_X86_64 = 9,
    PLATFORM_PALM_ARM = 10,
    PLATFORM_LINUX_SH = 11,
    PLATFORM_FREEBSD_X86 = 12,
    PLATFORM_FREEBSD_X86_64 = 13,
    PLATFORM_BLACKBERRY_ARM = 14,
    PLATFORM_SONOS = 15,
    PLATFORM_LINUX_MIPS = 16,
    PLATFORM_LINUX_ARM = 17,
    PLATFORM_LOGITECH_ARM = 18,
    PLATFORM_LINUX_BLACKFIN = 19,
    PLATFORM_WP7_ARM = 20,
    PLATFORM_ONKYO_ARM = 21,
    PLATFORM_QNXNTO_ARM = 22,
    PLATFORM_BCO_ARM = 23,
}

impl ::protobuf::ProtobufEnum for Platform {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Platform> {
        match value {
            0 => ::std::option::Option::Some(Platform::PLATFORM_WIN32_X86),
            1 => ::std::option::Option::Some(Platform::PLATFORM_OSX_X86),
            2 => ::std::option::Option::Some(Platform::PLATFORM_LINUX_X86),
            3 => ::std::option::Option::Some(Platform::PLATFORM_IPHONE_ARM),
            4 => ::std::option::Option::Some(Platform::PLATFORM_S60_ARM),
            5 => ::std::option::Option::Some(Platform::PLATFORM_OSX_PPC),
            6 => ::std::option::Option::Some(Platform::PLATFORM_ANDROID_ARM),
            7 => ::std::option::Option::Some(Platform::PLATFORM_WINDOWS_CE_ARM),
            8 => ::std::option::Option::Some(Platform::PLATFORM_LINUX_X86_64),
            9 => ::std::option::Option::Some(Platform::PLATFORM_OSX_X86_64),
            10 => ::std::option::Option::Some(Platform::PLATFORM_PALM_ARM),
            11 => ::std::option::Option::Some(Platform::PLATFORM_LINUX_SH),
            12 => ::std::option::Option::Some(Platform::PLATFORM_FREEBSD_X86),
            13 => ::std::option::Option::Some(Platform::PLATFORM_FREEBSD_X86_64),
            14 => ::std::option::Option::Some(Platform::PLATFORM_BLACKBERRY_ARM),
            15 => ::std::option::Option::Some(Platform::PLATFORM_SONOS),
            16 => ::std::option::Option::Some(Platform::PLATFORM_LINUX_MIPS),
            17 => ::std::option::Option::Some(Platform::PLATFORM_LINUX_ARM),
            18 => ::std::option::Option::Some(Platform::PLATFORM_LOGITECH_ARM),
            19 => ::std::option::Option::Some(Platform::PLATFORM_LINUX_BLACKFIN),
            20 => ::std::option::Option::Some(Platform::PLATFORM_WP7_ARM),
            21 => ::std::option::Option::Some(Platform::PLATFORM_ONKYO_ARM),
            22 => ::std::option::Option::Some(Platform::PLATFORM_QNXNTO_ARM),
            23 => ::std::option::Option::Some(Platform::PLATFORM_BCO_ARM),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Platform] = &[
            Platform::PLATFORM_WIN32_X86,
            Platform::PLATFORM_OSX_X86,
            Platform::PLATFORM_LINUX_X86,
            Platform::PLATFORM_IPHONE_ARM,
            Platform::PLATFORM_S60_ARM,
            Platform::PLATFORM_OSX_PPC,
            Platform::PLATFORM_ANDROID_ARM,
            Platform::PLATFORM_WINDOWS_CE_ARM,
            Platform::PLATFORM_LINUX_X86_64,
            Platform::PLATFORM_OSX_X86_64,
            Platform::PLATFORM_PALM_ARM,
            Platform::PLATFORM_LINUX_SH,
            Platform::PLATFORM_FREEBSD_X86,
            Platform::PLATFORM_FREEBSD_X86_64,
            Platform::PLATFORM_BLACKBERRY_ARM,
            Platform::PLATFORM_SONOS,
            Platform::PLATFORM_LINUX_MIPS,
            Platform::PLATFORM_LINUX_ARM,
            Platform::PLATFORM_LOGITECH_ARM,
            Platform::PLATFORM_LINUX_BLACKFIN,
            Platform::PLATFORM_WP7_ARM,
            Platform::PLATFORM_ONKYO_ARM,
            Platform::PLATFORM_QNXNTO_ARM,
            Platform::PLATFORM_BCO_ARM,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Platform>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Platform", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Platform {
}

impl ::protobuf::reflect::ProtobufValue for Platform {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Fingerprint {
    FINGERPRINT_GRAIN = 0,
    FINGERPRINT_HMAC_RIPEMD = 1,
}

impl ::protobuf::ProtobufEnum for Fingerprint {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Fingerprint> {
        match value {
            0 => ::std::option::Option::Some(Fingerprint::FINGERPRINT_GRAIN),
            1 => ::std::option::Option::Some(Fingerprint::FINGERPRINT_HMAC_RIPEMD),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Fingerprint] = &[
            Fingerprint::FINGERPRINT_GRAIN,
            Fingerprint::FINGERPRINT_HMAC_RIPEMD,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Fingerprint>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Fingerprint", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Fingerprint {
}

impl ::protobuf::reflect::ProtobufValue for Fingerprint {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Cryptosuite {
    CRYPTO_SUITE_SHANNON = 0,
    CRYPTO_SUITE_RC4_SHA1_HMAC = 1,
}

impl ::protobuf::ProtobufEnum for Cryptosuite {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Cryptosuite> {
        match value {
            0 => ::std::option::Option::Some(Cryptosuite::CRYPTO_SUITE_SHANNON),
            1 => ::std::option::Option::Some(Cryptosuite::CRYPTO_SUITE_RC4_SHA1_HMAC),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Cryptosuite] = &[
            Cryptosuite::CRYPTO_SUITE_SHANNON,
            Cryptosuite::CRYPTO_SUITE_RC4_SHA1_HMAC,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Cryptosuite>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Cryptosuite", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Cryptosuite {
}

impl ::protobuf::reflect::ProtobufValue for Cryptosuite {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Powscheme {
    POW_HASH_CASH = 0,
}

impl ::protobuf::ProtobufEnum for Powscheme {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Powscheme> {
        match value {
            0 => ::std::option::Option::Some(Powscheme::POW_HASH_CASH),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Powscheme] = &[
            Powscheme::POW_HASH_CASH,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Powscheme>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Powscheme", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Powscheme {
}

impl ::protobuf::reflect::ProtobufValue for Powscheme {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ErrorCode {
    ProtocolError = 0,
    TryAnotherAP = 2,
    BadConnectionId = 5,
    TravelRestriction = 9,
    PremiumAccountRequired = 11,
    BadCredentials = 12,
    CouldNotValidateCredentials = 13,
    AccountExists = 14,
    ExtraVerificationRequired = 15,
    InvalidAppKey = 16,
    ApplicationBanned = 17,
}

impl ::protobuf::ProtobufEnum for ErrorCode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ErrorCode> {
        match value {
            0 => ::std::option::Option::Some(ErrorCode::ProtocolError),
            2 => ::std::option::Option::Some(ErrorCode::TryAnotherAP),
            5 => ::std::option::Option::Some(ErrorCode::BadConnectionId),
            9 => ::std::option::Option::Some(ErrorCode::TravelRestriction),
            11 => ::std::option::Option::Some(ErrorCode::PremiumAccountRequired),
            12 => ::std::option::Option::Some(ErrorCode::BadCredentials),
            13 => ::std::option::Option::Some(ErrorCode::CouldNotValidateCredentials),
            14 => ::std::option::Option::Some(ErrorCode::AccountExists),
            15 => ::std::option::Option::Some(ErrorCode::ExtraVerificationRequired),
            16 => ::std::option::Option::Some(ErrorCode::InvalidAppKey),
            17 => ::std::option::Option::Some(ErrorCode::ApplicationBanned),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ErrorCode] = &[
            ErrorCode::ProtocolError,
            ErrorCode::TryAnotherAP,
            ErrorCode::BadConnectionId,
            ErrorCode::TravelRestriction,
            ErrorCode::PremiumAccountRequired,
            ErrorCode::BadCredentials,
            ErrorCode::CouldNotValidateCredentials,
            ErrorCode::AccountExists,
            ErrorCode::ExtraVerificationRequired,
            ErrorCode::InvalidAppKey,
            ErrorCode::ApplicationBanned,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ErrorCode>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ErrorCode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ErrorCode {
}

impl ::protobuf::reflect::ProtobufValue for ErrorCode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11keyexchange.proto\"\xb2\x03\n\x0bClientHello\x12)\n\nbuild_info\
    \x18\n\x20\x02(\x0b2\n.BuildInfoR\tbuildInfo\x12C\n\x16fingerprints_supp\
    orted\x18\x14\x20\x03(\x0e2\x0c.FingerprintR\x15fingerprintsSupported\
    \x12C\n\x16cryptosuites_supported\x18\x1e\x20\x03(\x0e2\x0c.CryptosuiteR\
    \x15cryptosuitesSupported\x12=\n\x14powschemes_supported\x18(\x20\x03(\
    \x0e2\n.PowschemeR\x13powschemesSupported\x12D\n\x12login_crypto_hello\
    \x182\x20\x02(\x0b2\x16.LoginCryptoHelloUnionR\x10loginCryptoHello\x12!\
    \n\x0cclient_nonce\x18<\x20\x02(\x0cR\x0bclientNonce\x12\x18\n\x07paddin\
    g\x18F\x20\x01(\x0cR\x07padding\x12,\n\x0bfeature_set\x18P\x20\x01(\x0b2\
    \x0b.FeatureSetR\nfeatureSet\"\xa4\x01\n\tBuildInfo\x12\"\n\x07product\
    \x18\n\x20\x02(\x0e2\x08.ProductR\x07product\x122\n\rproduct_flags\x18\
    \x14\x20\x03(\x0e2\r.ProductFlagsR\x0cproductFlags\x12%\n\x08platform\
    \x18\x1e\x20\x02(\x0e2\t.PlatformR\x08platform\x12\x18\n\x07version\x18(\
    \x20\x02(\x04R\x07version\"^\n\x15LoginCryptoHelloUnion\x12E\n\x0ediffie\
    _hellman\x18\n\x20\x01(\x0b2\x1e.LoginCryptoDiffieHellmanHelloR\rdiffieH\
    ellman\"[\n\x1dLoginCryptoDiffieHellmanHello\x12\x0e\n\x02gc\x18\n\x20\
    \x02(\x0cR\x02gc\x12*\n\x11server_keys_known\x18\x14\x20\x02(\rR\x0fserv\
    erKeysKnown\"Y\n\nFeatureSet\x12\x20\n\x0bautoupdate2\x18\x01\x20\x01(\
    \x08R\x0bautoupdate2\x12)\n\x10current_location\x18\x02\x20\x01(\x08R\
    \x0fcurrentLocation\"\xa5\x01\n\x11APResponseMessage\x12*\n\tchallenge\
    \x18\n\x20\x01(\x0b2\x0c.APChallengeR\tchallenge\x121\n\x07upgrade\x18\
    \x14\x20\x01(\x0b2\x17.UpgradeRequiredMessageR\x07upgrade\x121\n\x0clogi\
    n_failed\x18\x1e\x20\x01(\x0b2\x0e.APLoginFailedR\x0bloginFailed\"\xe8\
    \x02\n\x0bAPChallenge\x12P\n\x16login_crypto_challenge\x18\n\x20\x02(\
    \x0b2\x1a.LoginCryptoChallengeUnionR\x14loginCryptoChallenge\x12O\n\x15f\
    ingerprint_challenge\x18\x14\x20\x02(\x0b2\x1a.FingerprintChallengeUnion\
    R\x14fingerprintChallenge\x127\n\rpow_challenge\x18\x1e\x20\x02(\x0b2\
    \x12.PoWChallengeUnionR\x0cpowChallenge\x12@\n\x10crypto_challenge\x18(\
    \x20\x02(\x0b2\x15.CryptoChallengeUnionR\x0fcryptoChallenge\x12!\n\x0cse\
    rver_nonce\x182\x20\x02(\x0cR\x0bserverNonce\x12\x18\n\x07padding\x18<\
    \x20\x01(\x0cR\x07padding\"f\n\x19LoginCryptoChallengeUnion\x12I\n\x0edi\
    ffie_hellman\x18\n\x20\x01(\x0b2\".LoginCryptoDiffieHellmanChallengeR\rd\
    iffieHellman\"\x88\x01\n!LoginCryptoDiffieHellmanChallenge\x12\x0e\n\x02\
    gs\x18\n\x20\x02(\x0cR\x02gs\x120\n\x14server_signature_key\x18\x14\x20\
    \x02(\x05R\x12serverSignatureKey\x12!\n\x0cgs_signature\x18\x1e\x20\x02(\
    \x0cR\x0bgsSignature\"\x8f\x01\n\x19FingerprintChallengeUnion\x120\n\x05\
    grain\x18\n\x20\x01(\x0b2\x1a.FingerprintGrainChallengeR\x05grain\x12@\n\
    \x0bhmac_ripemd\x18\x14\x20\x01(\x0b2\x1f.FingerprintHmacRipemdChallenge\
    R\nhmacRipemd\"-\n\x19FingerprintGrainChallenge\x12\x10\n\x03kek\x18\n\
    \x20\x02(\x0cR\x03kek\">\n\x1eFingerprintHmacRipemdChallenge\x12\x1c\n\t\
    challenge\x18\n\x20\x02(\x0cR\tchallenge\"G\n\x11PoWChallengeUnion\x122\
    \n\thash_cash\x18\n\x20\x01(\x0b2\x15.PoWHashCashChallengeR\x08hashCash\
    \"^\n\x14PoWHashCashChallenge\x12\x16\n\x06prefix\x18\n\x20\x01(\x0cR\
    \x06prefix\x12\x16\n\x06length\x18\x14\x20\x01(\x05R\x06length\x12\x16\n\
    \x06target\x18\x1e\x20\x01(\x05R\x06target\"\x8a\x01\n\x14CryptoChalleng\
    eUnion\x121\n\x07shannon\x18\n\x20\x01(\x0b2\x17.CryptoShannonChallengeR\
    \x07shannon\x12?\n\rrc4_sha1_hmac\x18\x14\x20\x01(\x0b2\x1b.CryptoRc4Sha\
    1HmacChallengeR\x0brc4Sha1Hmac\"\x18\n\x16CryptoShannonChallenge\"\x1c\n\
    \x1aCryptoRc4Sha1HmacChallenge\"\x87\x01\n\x16UpgradeRequiredMessage\x12\
    .\n\x13upgrade_signed_part\x18\n\x20\x02(\x0cR\x11upgradeSignedPart\x12\
    \x1c\n\tsignature\x18\x14\x20\x02(\x0cR\tsignature\x12\x1f\n\x0bhttp_suf\
    fix\x18\x1e\x20\x01(\tR\nhttpSuffix\"\xa0\x01\n\rAPLoginFailed\x12)\n\ne\
    rror_code\x18\n\x20\x02(\x0e2\n.ErrorCodeR\terrorCode\x12\x1f\n\x0bretry\
    _delay\x18\x14\x20\x01(\x05R\nretryDelay\x12\x16\n\x06expiry\x18\x1e\x20\
    \x01(\x05R\x06expiry\x12+\n\x11error_description\x18(\x20\x01(\tR\x10err\
    orDescription\"\xdd\x01\n\x17ClientResponsePlaintext\x12M\n\x15login_cry\
    pto_response\x18\n\x20\x02(\x0b2\x19.LoginCryptoResponseUnionR\x13loginC\
    ryptoResponse\x124\n\x0cpow_response\x18\x14\x20\x02(\x0b2\x11.PoWRespon\
    seUnionR\x0bpowResponse\x12=\n\x0fcrypto_response\x18\x1e\x20\x02(\x0b2\
    \x14.CryptoResponseUnionR\x0ecryptoResponse\"d\n\x18LoginCryptoResponseU\
    nion\x12H\n\x0ediffie_hellman\x18\n\x20\x01(\x0b2!.LoginCryptoDiffieHell\
    manResponseR\rdiffieHellman\"6\n\x20LoginCryptoDiffieHellmanResponse\x12\
    \x12\n\x04hmac\x18\n\x20\x02(\x0cR\x04hmac\"E\n\x10PoWResponseUnion\x121\
    \n\thash_cash\x18\n\x20\x01(\x0b2\x14.PoWHashCashResponseR\x08hashCash\"\
    6\n\x13PoWHashCashResponse\x12\x1f\n\x0bhash_suffix\x18\n\x20\x02(\x0cR\
    \nhashSuffix\"\x87\x01\n\x13CryptoResponseUnion\x120\n\x07shannon\x18\n\
    \x20\x01(\x0b2\x16.CryptoShannonResponseR\x07shannon\x12>\n\rrc4_sha1_hm\
    ac\x18\x14\x20\x01(\x0b2\x1a.CryptoRc4Sha1HmacResponseR\x0brc4Sha1Hmac\"\
    -\n\x15CryptoShannonResponse\x12\x14\n\x05dummy\x18\x01\x20\x01(\x05R\
    \x05dummy\"1\n\x19CryptoRc4Sha1HmacResponse\x12\x14\n\x05dummy\x18\x01\
    \x20\x01(\x05R\x05dummy*\x7f\n\x07Product\x12\x12\n\x0ePRODUCT_CLIENT\
    \x10\0\x12\x16\n\x12PRODUCT_LIBSPOTIFY\x10\x01\x12\x12\n\x0ePRODUCT_MOBI\
    LE\x10\x02\x12\x13\n\x0fPRODUCT_PARTNER\x10\x03\x12\x1f\n\x1bPRODUCT_LIB\
    SPOTIFY_EMBEDDED\x10\x05*A\n\x0cProductFlags\x12\x15\n\x11PRODUCT_FLAG_N\
    ONE\x10\0\x12\x1a\n\x16PRODUCT_FLAG_DEV_BUILD\x10\x01*\xdc\x04\n\x08Plat\
    form\x12\x16\n\x12PLATFORM_WIN32_X86\x10\0\x12\x14\n\x10PLATFORM_OSX_X86\
    \x10\x01\x12\x16\n\x12PLATFORM_LINUX_X86\x10\x02\x12\x17\n\x13PLATFORM_I\
    PHONE_ARM\x10\x03\x12\x14\n\x10PLATFORM_S60_ARM\x10\x04\x12\x14\n\x10PLA\
    TFORM_OSX_PPC\x10\x05\x12\x18\n\x14PLATFORM_ANDROID_ARM\x10\x06\x12\x1b\
    \n\x17PLATFORM_WINDOWS_CE_ARM\x10\x07\x12\x19\n\x15PLATFORM_LINUX_X86_64\
    \x10\x08\x12\x17\n\x13PLATFORM_OSX_X86_64\x10\t\x12\x15\n\x11PLATFORM_PA\
    LM_ARM\x10\n\x12\x15\n\x11PLATFORM_LINUX_SH\x10\x0b\x12\x18\n\x14PLATFOR\
    M_FREEBSD_X86\x10\x0c\x12\x1b\n\x17PLATFORM_FREEBSD_X86_64\x10\r\x12\x1b\
    \n\x17PLATFORM_BLACKBERRY_ARM\x10\x0e\x12\x12\n\x0ePLATFORM_SONOS\x10\
    \x0f\x12\x17\n\x13PLATFORM_LINUX_MIPS\x10\x10\x12\x16\n\x12PLATFORM_LINU\
    X_ARM\x10\x11\x12\x19\n\x15PLATFORM_LOGITECH_ARM\x10\x12\x12\x1b\n\x17PL\
    ATFORM_LINUX_BLACKFIN\x10\x13\x12\x14\n\x10PLATFORM_WP7_ARM\x10\x14\x12\
    \x16\n\x12PLATFORM_ONKYO_ARM\x10\x15\x12\x17\n\x13PLATFORM_QNXNTO_ARM\
    \x10\x16\x12\x14\n\x10PLATFORM_BCO_ARM\x10\x17*A\n\x0bFingerprint\x12\
    \x15\n\x11FINGERPRINT_GRAIN\x10\0\x12\x1b\n\x17FINGERPRINT_HMAC_RIPEMD\
    \x10\x01*G\n\x0bCryptosuite\x12\x18\n\x14CRYPTO_SUITE_SHANNON\x10\0\x12\
    \x1e\n\x1aCRYPTO_SUITE_RC4_SHA1_HMAC\x10\x01*\x1e\n\tPowscheme\x12\x11\n\
    \rPOW_HASH_CASH\x10\0*\x89\x02\n\tErrorCode\x12\x11\n\rProtocolError\x10\
    \0\x12\x10\n\x0cTryAnotherAP\x10\x02\x12\x13\n\x0fBadConnectionId\x10\
    \x05\x12\x15\n\x11TravelRestriction\x10\t\x12\x1a\n\x16PremiumAccountReq\
    uired\x10\x0b\x12\x12\n\x0eBadCredentials\x10\x0c\x12\x1f\n\x1bCouldNotV\
    alidateCredentials\x10\r\x12\x11\n\rAccountExists\x10\x0e\x12\x1d\n\x19E\
    xtraVerificationRequired\x10\x0f\x12\x11\n\rInvalidAppKey\x10\x10\x12\
    \x15\n\x11ApplicationBanned\x10\x11J\xbd6\n\x07\x12\x05\0\0\xe2\x01\x01\
    \n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\x0b\x01\n\
    \n\n\x03\x04\0\x01\x12\x03\x02\x08\x13\n\x0b\n\x04\x04\0\x02\0\x12\x03\
    \x03\x04(\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x03\x04\x0c\n\x0c\n\x05\
    \x04\0\x02\0\x06\x12\x03\x03\r\x16\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \x03\x17!\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03$'\n\x0b\n\x04\x04\0\
    \x02\x01\x12\x03\x04\x047\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\x04\x04\
    \x0c\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03\x04\r\x18\n\x0c\n\x05\x04\0\
    \x02\x01\x01\x12\x03\x04\x19/\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x042\
    6\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x05\x047\n\x0c\n\x05\x04\0\x02\x02\
    \x04\x12\x03\x05\x04\x0c\n\x0c\n\x05\x04\0\x02\x02\x06\x12\x03\x05\r\x18\
    \n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x05\x19/\n\x0c\n\x05\x04\0\x02\
    \x02\x03\x12\x03\x0526\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x06\x043\n\x0c\
    \n\x05\x04\0\x02\x03\x04\x12\x03\x06\x04\x0c\n\x0c\n\x05\x04\0\x02\x03\
    \x06\x12\x03\x06\r\x16\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x06\x17+\n\
    \x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x06.2\n\x0b\n\x04\x04\0\x02\x04\x12\
    \x03\x07\x04=\n\x0c\n\x05\x04\0\x02\x04\x04\x12\x03\x07\x04\x0c\n\x0c\n\
    \x05\x04\0\x02\x04\x06\x12\x03\x07\r\"\n\x0c\n\x05\x04\0\x02\x04\x01\x12\
    \x03\x07#5\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x078<\n\x0b\n\x04\x04\0\
    \x02\x05\x12\x03\x08\x04'\n\x0c\n\x05\x04\0\x02\x05\x04\x12\x03\x08\x04\
    \x0c\n\x0c\n\x05\x04\0\x02\x05\x05\x12\x03\x08\r\x12\n\x0c\n\x05\x04\0\
    \x02\x05\x01\x12\x03\x08\x13\x1f\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03\
    \x08\"&\n\x0b\n\x04\x04\0\x02\x06\x12\x03\t\x04\"\n\x0c\n\x05\x04\0\x02\
    \x06\x04\x12\x03\t\x04\x0c\n\x0c\n\x05\x04\0\x02\x06\x05\x12\x03\t\r\x12\
    \n\x0c\n\x05\x04\0\x02\x06\x01\x12\x03\t\x13\x1a\n\x0c\n\x05\x04\0\x02\
    \x06\x03\x12\x03\t\x1d!\n\x0b\n\x04\x04\0\x02\x07\x12\x03\n\x04+\n\x0c\n\
    \x05\x04\0\x02\x07\x04\x12\x03\n\x04\x0c\n\x0c\n\x05\x04\0\x02\x07\x06\
    \x12\x03\n\r\x17\n\x0c\n\x05\x04\0\x02\x07\x01\x12\x03\n\x18#\n\x0c\n\
    \x05\x04\0\x02\x07\x03\x12\x03\n&*\n\n\n\x02\x04\x01\x12\x04\x0e\0\x13\
    \x01\n\n\n\x03\x04\x01\x01\x12\x03\x0e\x08\x11\n\x0b\n\x04\x04\x01\x02\0\
    \x12\x03\x0f\x04#\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x0f\x04\x0c\n\
    \x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x0f\r\x14\n\x0c\n\x05\x04\x01\x02\0\
    \x01\x12\x03\x0f\x15\x1c\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x0f\x1f\"\
    \n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x10\x04/\n\x0c\n\x05\x04\x01\x02\
    \x01\x04\x12\x03\x10\x04\x0c\n\x0c\n\x05\x04\x01\x02\x01\x06\x12\x03\x10\
    \r\x19\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x10\x1a'\n\x0c\n\x05\x04\
    \x01\x02\x01\x03\x12\x03\x10*.\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x11\
    \x04&\n\x0c\n\x05\x04\x01\x02\x02\x04\x12\x03\x11\x04\x0c\n\x0c\n\x05\
    \x04\x01\x02\x02\x06\x12\x03\x11\r\x15\n\x0c\n\x05\x04\x01\x02\x02\x01\
    \x12\x03\x11\x16\x1e\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x11!%\n\x0b\
    \n\x04\x04\x01\x02\x03\x12\x03\x12\x04#\n\x0c\n\x05\x04\x01\x02\x03\x04\
    \x12\x03\x12\x04\x0c\n\x0c\n\x05\x04\x01\x02\x03\x05\x12\x03\x12\r\x13\n\
    \x0c\n\x05\x04\x01\x02\x03\x01\x12\x03\x12\x14\x1b\n\x0c\n\x05\x04\x01\
    \x02\x03\x03\x12\x03\x12\x1e\"\n\n\n\x02\x05\0\x12\x04\x15\0\x1b\x01\n\n\
    \n\x03\x05\0\x01\x12\x03\x15\x05\x0c\n\x0b\n\x04\x05\0\x02\0\x12\x03\x16\
    \x04\x19\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x16\x04\x12\n\x0c\n\x05\x05\
    \0\x02\0\x02\x12\x03\x16\x15\x18\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x17\
    \x04\x1c\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x17\x04\x16\n\x0c\n\x05\
    \x05\0\x02\x01\x02\x12\x03\x17\x18\x1b\n\x0b\n\x04\x05\0\x02\x02\x12\x03\
    \x18\x04\x19\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03\x18\x04\x12\n\x0c\n\
    \x05\x05\0\x02\x02\x02\x12\x03\x18\x15\x18\n\x0b\n\x04\x05\0\x02\x03\x12\
    \x03\x19\x04\x1a\n\x0c\n\x05\x05\0\x02\x03\x01\x12\x03\x19\x04\x13\n\x0c\
    \n\x05\x05\0\x02\x03\x02\x12\x03\x19\x16\x19\n\x0b\n\x04\x05\0\x02\x04\
    \x12\x03\x1a\x04&\n\x0c\n\x05\x05\0\x02\x04\x01\x12\x03\x1a\x04\x1f\n\
    \x0c\n\x05\x05\0\x02\x04\x02\x12\x03\x1a\"%\n\n\n\x02\x05\x01\x12\x04\
    \x1d\0\x20\x01\n\n\n\x03\x05\x01\x01\x12\x03\x1d\x05\x11\n\x0b\n\x04\x05\
    \x01\x02\0\x12\x03\x1e\x04\x1c\n\x0c\n\x05\x05\x01\x02\0\x01\x12\x03\x1e\
    \x04\x15\n\x0c\n\x05\x05\x01\x02\0\x02\x12\x03\x1e\x18\x1b\n\x0b\n\x04\
    \x05\x01\x02\x01\x12\x03\x1f\x04!\n\x0c\n\x05\x05\x01\x02\x01\x01\x12\
    \x03\x1f\x04\x1a\n\x0c\n\x05\x05\x01\x02\x01\x02\x12\x03\x1f\x1d\x20\n\n\
    \n\x02\x05\x02\x12\x04\"\0;\x01\n\n\n\x03\x05\x02\x01\x12\x03\"\x05\r\n\
    \x0b\n\x04\x05\x02\x02\0\x12\x03#\x04\x1d\n\x0c\n\x05\x05\x02\x02\0\x01\
    \x12\x03#\x04\x16\n\x0c\n\x05\x05\x02\x02\0\x02\x12\x03#\x19\x1c\n\x0b\n\
    \x04\x05\x02\x02\x01\x12\x03$\x04\x1b\n\x0c\n\x05\x05\x02\x02\x01\x01\
    \x12\x03$\x04\x14\n\x0c\n\x05\x05\x02\x02\x01\x02\x12\x03$\x17\x1a\n\x0b\
    \n\x04\x05\x02\x02\x02\x12\x03%\x04\x1d\n\x0c\n\x05\x05\x02\x02\x02\x01\
    \x12\x03%\x04\x16\n\x0c\n\x05\x05\x02\x02\x02\x02\x12\x03%\x19\x1c\n\x0b\
    \n\x04\x05\x02\x02\x03\x12\x03&\x04\x1e\n\x0c\n\x05\x05\x02\x02\x03\x01\
    \x12\x03&\x04\x17\n\x0c\n\x05\x05\x02\x02\x03\x02\x12\x03&\x1a\x1d\n\x0b\
    \n\x04\x05\x02\x02\x04\x12\x03'\x04\x1b\n\x0c\n\x05\x05\x02\x02\x04\x01\
    \x12\x03'\x04\x14\n\x0c\n\x05\x05\x02\x02\x04\x02\x12\x03'\x17\x1a\n\x0b\
    \n\x04\x05\x02\x02\x05\x12\x03(\x04\x1b\n\x0c\n\x05\x05\x02\x02\x05\x01\
    \x12\x03(\x04\x14\n\x0c\n\x05\x05\x02\x02\x05\x02\x12\x03(\x17\x1a\n\x0b\
    \n\x04\x05\x02\x02\x06\x12\x03)\x04\x1f\n\x0c\n\x05\x05\x02\x02\x06\x01\
    \x12\x03)\x04\x18\n\x0c\n\x05\x05\x02\x02\x06\x02\x12\x03)\x1b\x1e\n\x0b\
    \n\x04\x05\x02\x02\x07\x12\x03*\x04\"\n\x0c\n\x05\x05\x02\x02\x07\x01\
    \x12\x03*\x04\x1b\n\x0c\n\x05\x05\x02\x02\x07\x02\x12\x03*\x1e!\n\x0b\n\
    \x04\x05\x02\x02\x08\x12\x03+\x04\x20\n\x0c\n\x05\x05\x02\x02\x08\x01\
    \x12\x03+\x04\x19\n\x0c\n\x05\x05\x02\x02\x08\x02\x12\x03+\x1c\x1f\n\x0b\
    \n\x04\x05\x02\x02\t\x12\x03,\x04\x1e\n\x0c\n\x05\x05\x02\x02\t\x01\x12\
    \x03,\x04\x17\n\x0c\n\x05\x05\x02\x02\t\x02\x12\x03,\x1a\x1d\n\x0b\n\x04\
    \x05\x02\x02\n\x12\x03-\x04\x1c\n\x0c\n\x05\x05\x02\x02\n\x01\x12\x03-\
    \x04\x15\n\x0c\n\x05\x05\x02\x02\n\x02\x12\x03-\x18\x1b\n\x0b\n\x04\x05\
    \x02\x02\x0b\x12\x03.\x04\x1c\n\x0c\n\x05\x05\x02\x02\x0b\x01\x12\x03.\
    \x04\x15\n\x0c\n\x05\x05\x02\x02\x0b\x02\x12\x03.\x18\x1b\n\x0b\n\x04\
    \x05\x02\x02\x0c\x12\x03/\x04\x1f\n\x0c\n\x05\x05\x02\x02\x0c\x01\x12\
    \x03/\x04\x18\n\x0c\n\x05\x05\x02\x02\x0c\x02\x12\x03/\x1b\x1e\n\x0b\n\
    \x04\x05\x02\x02\r\x12\x030\x04\"\n\x0c\n\x05\x05\x02\x02\r\x01\x12\x030\
    \x04\x1b\n\x0c\n\x05\x05\x02\x02\r\x02\x12\x030\x1e!\n\x0b\n\x04\x05\x02\
    \x02\x0e\x12\x031\x04\"\n\x0c\n\x05\x05\x02\x02\x0e\x01\x12\x031\x04\x1b\
    \n\x0c\n\x05\x05\x02\x02\x0e\x02\x12\x031\x1e!\n\x0b\n\x04\x05\x02\x02\
    \x0f\x12\x032\x04\x19\n\x0c\n\x05\x05\x02\x02\x0f\x01\x12\x032\x04\x12\n\
    \x0c\n\x05\x05\x02\x02\x0f\x02\x12\x032\x15\x18\n\x0b\n\x04\x05\x02\x02\
    \x10\x12\x033\x04\x1f\n\x0c\n\x05\x05\x02\x02\x10\x01\x12\x033\x04\x17\n\
    \x0c\n\x05\x05\x02\x02\x10\x02\x12\x033\x1a\x1e\n\x0b\n\x04\x05\x02\x02\
    \x11\x12\x034\x04\x1e\n\x0c\n\x05\x05\x02\x02\x11\x01\x12\x034\x04\x16\n\
    \x0c\n\x05\x05\x02\x02\x11\x02\x12\x034\x19\x1d\n\x0b\n\x04\x05\x02\x02\
    \x12\x12\x035\x04!\n\x0c\n\x05\x05\x02\x02\x12\x01\x12\x035\x04\x19\n\
    \x0c\n\x05\x05\x02\x02\x12\x02\x12\x035\x1c\x20\n\x0b\n\x04\x05\x02\x02\
    \x13\x12\x036\x04#\n\x0c\n\x05\x05\x02\x02\x13\x01\x12\x036\x04\x1b\n\
    \x0c\n\x05\x05\x02\x02\x13\x02\x12\x036\x1e\"\n\x0b\n\x04\x05\x02\x02\
    \x14\x12\x037\x04\x1c\n\x0c\n\x05\x05\x02\x02\x14\x01\x12\x037\x04\x14\n\
    \x0c\n\x05\x05\x02\x02\x14\x02\x12\x037\x17\x1b\n\x0b\n\x04\x05\x02\x02\
    \x15\x12\x038\x04\x1e\n\x0c\n\x05\x05\x02\x02\x15\x01\x12\x038\x04\x16\n\
    \x0c\n\x05\x05\x02\x02\x15\x02\x12\x038\x19\x1d\n\x0b\n\x04\x05\x02\x02\
    \x16\x12\x039\x04\x1f\n\x0c\n\x05\x05\x02\x02\x16\x01\x12\x039\x04\x17\n\
    \x0c\n\x05\x05\x02\x02\x16\x02\x12\x039\x1a\x1e\n\x0b\n\x04\x05\x02\x02\
    \x17\x12\x03:\x04\x1c\n\x0c\n\x05\x05\x02\x02\x17\x01\x12\x03:\x04\x14\n\
    \x0c\n\x05\x05\x02\x02\x17\x02\x12\x03:\x17\x1b\n\n\n\x02\x05\x03\x12\
    \x04=\0@\x01\n\n\n\x03\x05\x03\x01\x12\x03=\x05\x10\n\x0b\n\x04\x05\x03\
    \x02\0\x12\x03>\x04\x1c\n\x0c\n\x05\x05\x03\x02\0\x01\x12\x03>\x04\x15\n\
    \x0c\n\x05\x05\x03\x02\0\x02\x12\x03>\x18\x1b\n\x0b\n\x04\x05\x03\x02\
    \x01\x12\x03?\x04\"\n\x0c\n\x05\x05\x03\x02\x01\x01\x12\x03?\x04\x1b\n\
    \x0c\n\x05\x05\x03\x02\x01\x02\x12\x03?\x1e!\n\n\n\x02\x05\x04\x12\x04B\
    \0E\x01\n\n\n\x03\x05\x04\x01\x12\x03B\x05\x10\n\x0b\n\x04\x05\x04\x02\0\
    \x12\x03C\x04\x1f\n\x0c\n\x05\x05\x04\x02\0\x01\x12\x03C\x04\x18\n\x0c\n\
    \x05\x05\x04\x02\0\x02\x12\x03C\x1b\x1e\n\x0b\n\x04\x05\x04\x02\x01\x12\
    \x03D\x04%\n\x0c\n\x05\x05\x04\x02\x01\x01\x12\x03D\x04\x1e\n\x0c\n\x05\
    \x05\x04\x02\x01\x02\x12\x03D!$\n\n\n\x02\x05\x05\x12\x04G\0I\x01\n\n\n\
    \x03\x05\x05\x01\x12\x03G\x05\x0e\n\x0b\n\x04\x05\x05\x02\0\x12\x03H\x04\
    \x18\n\x0c\n\x05\x05\x05\x02\0\x01\x12\x03H\x04\x11\n\x0c\n\x05\x05\x05\
    \x02\0\x02\x12\x03H\x14\x17\n\n\n\x02\x04\x02\x12\x04L\0N\x01\n\n\n\x03\
    \x04\x02\x01\x12\x03L\x08\x1d\n\x0b\n\x04\x04\x02\x02\0\x12\x03M\x04@\n\
    \x0c\n\x05\x04\x02\x02\0\x04\x12\x03M\x04\x0c\n\x0c\n\x05\x04\x02\x02\0\
    \x06\x12\x03M\r*\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03M+9\n\x0c\n\x05\
    \x04\x02\x02\0\x03\x12\x03M<?\n\n\n\x02\x04\x03\x12\x04Q\0T\x01\n\n\n\
    \x03\x04\x03\x01\x12\x03Q\x08%\n\x0b\n\x04\x04\x03\x02\0\x12\x03R\x04\
    \x1c\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03R\x04\x0c\n\x0c\n\x05\x04\x03\
    \x02\0\x05\x12\x03R\r\x12\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03R\x13\x15\
    \n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03R\x18\x1b\n\x0b\n\x04\x04\x03\x02\
    \x01\x12\x03S\x04-\n\x0c\n\x05\x04\x03\x02\x01\x04\x12\x03S\x04\x0c\n\
    \x0c\n\x05\x04\x03\x02\x01\x05\x12\x03S\r\x13\n\x0c\n\x05\x04\x03\x02\
    \x01\x01\x12\x03S\x14%\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03S(,\n\n\n\
    \x02\x04\x04\x12\x04W\0Z\x01\n\n\n\x03\x04\x04\x01\x12\x03W\x08\x12\n\
    \x0b\n\x04\x04\x04\x02\0\x12\x03X\x04$\n\x0c\n\x05\x04\x04\x02\0\x04\x12\
    \x03X\x04\x0c\n\x0c\n\x05\x04\x04\x02\0\x05\x12\x03X\r\x11\n\x0c\n\x05\
    \x04\x04\x02\0\x01\x12\x03X\x12\x1d\n\x0c\n\x05\x04\x04\x02\0\x03\x12\
    \x03X\x20#\n\x0b\n\x04\x04\x04\x02\x01\x12\x03Y\x04)\n\x0c\n\x05\x04\x04\
    \x02\x01\x04\x12\x03Y\x04\x0c\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03Y\r\
    \x11\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03Y\x12\"\n\x0c\n\x05\x04\x04\
    \x02\x01\x03\x12\x03Y%(\n\n\n\x02\x04\x05\x12\x04]\0a\x01\n\n\n\x03\x04\
    \x05\x01\x12\x03]\x08\x19\n\x0b\n\x04\x04\x05\x02\0\x12\x03^\x04)\n\x0c\
    \n\x05\x04\x05\x02\0\x04\x12\x03^\x04\x0c\n\x0c\n\x05\x04\x05\x02\0\x06\
    \x12\x03^\r\x18\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03^\x19\"\n\x0c\n\x05\
    \x04\x05\x02\0\x03\x12\x03^%(\n\x0b\n\x04\x04\x05\x02\x01\x12\x03_\x043\
    \n\x0c\n\x05\x04\x05\x02\x01\x04\x12\x03_\x04\x0c\n\x0c\n\x05\x04\x05\
    \x02\x01\x06\x12\x03_\r#\n\x0c\n\x05\x04\x05\x02\x01\x01\x12\x03_$+\n\
    \x0c\n\x05\x04\x05\x02\x01\x03\x12\x03_.2\n\x0b\n\x04\x04\x05\x02\x02\
    \x12\x03`\x04/\n\x0c\n\x05\x04\x05\x02\x02\x04\x12\x03`\x04\x0c\n\x0c\n\
    \x05\x04\x05\x02\x02\x06\x12\x03`\r\x1a\n\x0c\n\x05\x04\x05\x02\x02\x01\
    \x12\x03`\x1b'\n\x0c\n\x05\x04\x05\x02\x02\x03\x12\x03`*.\n\n\n\x02\x04\
    \x06\x12\x04c\0j\x01\n\n\n\x03\x04\x06\x01\x12\x03c\x08\x13\n\x0b\n\x04\
    \x04\x06\x02\0\x12\x03d\x04D\n\x0c\n\x05\x04\x06\x02\0\x04\x12\x03d\x04\
    \x0c\n\x0c\n\x05\x04\x06\x02\0\x06\x12\x03d\r&\n\x0c\n\x05\x04\x06\x02\0\
    \x01\x12\x03d'=\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x03d@C\n\x0b\n\x04\x04\
    \x06\x02\x01\x12\x03e\x04D\n\x0c\n\x05\x04\x06\x02\x01\x04\x12\x03e\x04\
    \x0c\n\x0c\n\x05\x04\x06\x02\x01\x06\x12\x03e\r&\n\x0c\n\x05\x04\x06\x02\
    \x01\x01\x12\x03e'<\n\x0c\n\x05\x04\x06\x02\x01\x03\x12\x03e?C\n\x0b\n\
    \x04\x04\x06\x02\x02\x12\x03f\x044\n\x0c\n\x05\x04\x06\x02\x02\x04\x12\
    \x03f\x04\x0c\n\x0c\n\x05\x04\x06\x02\x02\x06\x12\x03f\r\x1e\n\x0c\n\x05\
    \x04\x06\x02\x02\x01\x12\x03f\x1f,\n\x0c\n\x05\x04\x06\x02\x02\x03\x12\
    \x03f/3\n\x0b\n\x04\x04\x06\x02\x03\x12\x03g\x04:\n\x0c\n\x05\x04\x06\
    \x02\x03\x04\x12\x03g\x04\x0c\n\x0c\n\x05\x04\x06\x02\x03\x06\x12\x03g\r\
    !\n\x0c\n\x05\x04\x06\x02\x03\x01\x12\x03g\"2\n\x0c\n\x05\x04\x06\x02\
    \x03\x03\x12\x03g59\n\x0b\n\x04\x04\x06\x02\x04\x12\x03h\x04'\n\x0c\n\
    \x05\x04\x06\x02\x04\x04\x12\x03h\x04\x0c\n\x0c\n\x05\x04\x06\x02\x04\
    \x05\x12\x03h\r\x12\n\x0c\n\x05\x04\x06\x02\x04\x01\x12\x03h\x13\x1f\n\
    \x0c\n\x05\x04\x06\x02\x04\x03\x12\x03h\"&\n\x0b\n\x04\x04\x06\x02\x05\
    \x12\x03i\x04\"\n\x0c\n\x05\x04\x06\x02\x05\x04\x12\x03i\x04\x0c\n\x0c\n\
    \x05\x04\x06\x02\x05\x05\x12\x03i\r\x12\n\x0c\n\x05\x04\x06\x02\x05\x01\
    \x12\x03i\x13\x1a\n\x0c\n\x05\x04\x06\x02\x05\x03\x12\x03i\x1d!\n\n\n\
    \x02\x04\x07\x12\x04l\0n\x01\n\n\n\x03\x04\x07\x01\x12\x03l\x08!\n\x0b\n\
    \x04\x04\x07\x02\0\x12\x03m\x04D\n\x0c\n\x05\x04\x07\x02\0\x04\x12\x03m\
    \x04\x0c\n\x0c\n\x05\x04\x07\x02\0\x06\x12\x03m\r.\n\x0c\n\x05\x04\x07\
    \x02\0\x01\x12\x03m/=\n\x0c\n\x05\x04\x07\x02\0\x03\x12\x03m@C\n\n\n\x02\
    \x04\x08\x12\x04p\0t\x01\n\n\n\x03\x04\x08\x01\x12\x03p\x08)\n\x0b\n\x04\
    \x04\x08\x02\0\x12\x03q\x04\x1c\n\x0c\n\x05\x04\x08\x02\0\x04\x12\x03q\
    \x04\x0c\n\x0c\n\x05\x04\x08\x02\0\x05\x12\x03q\r\x12\n\x0c\n\x05\x04\
    \x08\x02\0\x01\x12\x03q\x13\x15\n\x0c\n\x05\x04\x08\x02\0\x03\x12\x03q\
    \x18\x1b\n\x0b\n\x04\x04\x08\x02\x01\x12\x03r\x04/\n\x0c\n\x05\x04\x08\
    \x02\x01\x04\x12\x03r\x04\x0c\n\x0c\n\x05\x04\x08\x02\x01\x05\x12\x03r\r\
    \x12\n\x0c\n\x05\x04\x08\x02\x01\x01\x12\x03r\x13'\n\x0c\n\x05\x04\x08\
    \x02\x01\x03\x12\x03r*.\n\x0b\n\x04\x04\x08\x02\x02\x12\x03s\x04'\n\x0c\
    \n\x05\x04\x08\x02\x02\x04\x12\x03s\x04\x0c\n\x0c\n\x05\x04\x08\x02\x02\
    \x05\x12\x03s\r\x12\n\x0c\n\x05\x04\x08\x02\x02\x01\x12\x03s\x13\x1f\n\
    \x0c\n\x05\x04\x08\x02\x02\x03\x12\x03s\"&\n\n\n\x02\x04\t\x12\x04v\0y\
    \x01\n\n\n\x03\x04\t\x01\x12\x03v\x08!\n\x0b\n\x04\x04\t\x02\0\x12\x03w\
    \x043\n\x0c\n\x05\x04\t\x02\0\x04\x12\x03w\x04\x0c\n\x0c\n\x05\x04\t\x02\
    \0\x06\x12\x03w\r&\n\x0c\n\x05\x04\t\x02\0\x01\x12\x03w',\n\x0c\n\x05\
    \x04\t\x02\0\x03\x12\x03w/2\n\x0b\n\x04\x04\t\x02\x01\x12\x03x\x04?\n\
    \x0c\n\x05\x04\t\x02\x01\x04\x12\x03x\x04\x0c\n\x0c\n\x05\x04\t\x02\x01\
    \x06\x12\x03x\r+\n\x0c\n\x05\x04\t\x02\x01\x01\x12\x03x,7\n\x0c\n\x05\
    \x04\t\x02\x01\x03\x12\x03x:>\n\n\n\x02\x04\n\x12\x04|\0~\x01\n\n\n\x03\
    \x04\n\x01\x12\x03|\x08!\n\x0b\n\x04\x04\n\x02\0\x12\x03}\x04\x1d\n\x0c\
    \n\x05\x04\n\x02\0\x04\x12\x03}\x04\x0c\n\x0c\n\x05\x04\n\x02\0\x05\x12\
    \x03}\r\x12\n\x0c\n\x05\x04\n\x02\0\x01\x12\x03}\x13\x16\n\x0c\n\x05\x04\
    \n\x02\0\x03\x12\x03}\x19\x1c\n\x0c\n\x02\x04\x0b\x12\x06\x81\x01\0\x83\
    \x01\x01\n\x0b\n\x03\x04\x0b\x01\x12\x04\x81\x01\x08&\n\x0c\n\x04\x04\
    \x0b\x02\0\x12\x04\x82\x01\x04#\n\r\n\x05\x04\x0b\x02\0\x04\x12\x04\x82\
    \x01\x04\x0c\n\r\n\x05\x04\x0b\x02\0\x05\x12\x04\x82\x01\r\x12\n\r\n\x05\
    \x04\x0b\x02\0\x01\x12\x04\x82\x01\x13\x1c\n\r\n\x05\x04\x0b\x02\0\x03\
    \x12\x04\x82\x01\x1f\"\n\x0c\n\x02\x04\x0c\x12\x06\x86\x01\0\x88\x01\x01\
    \n\x0b\n\x03\x04\x0c\x01\x12\x04\x86\x01\x08\x19\n\x0c\n\x04\x04\x0c\x02\
    \0\x12\x04\x87\x01\x042\n\r\n\x05\x04\x0c\x02\0\x04\x12\x04\x87\x01\x04\
    \x0c\n\r\n\x05\x04\x0c\x02\0\x06\x12\x04\x87\x01\r!\n\r\n\x05\x04\x0c\
    \x02\0\x01\x12\x04\x87\x01\"+\n\r\n\x05\x04\x0c\x02\0\x03\x12\x04\x87\
    \x01.1\n\x0c\n\x02\x04\r\x12\x06\x8a\x01\0\x8e\x01\x01\n\x0b\n\x03\x04\r\
    \x01\x12\x04\x8a\x01\x08\x1c\n\x0c\n\x04\x04\r\x02\0\x12\x04\x8b\x01\x04\
    \x20\n\r\n\x05\x04\r\x02\0\x04\x12\x04\x8b\x01\x04\x0c\n\r\n\x05\x04\r\
    \x02\0\x05\x12\x04\x8b\x01\r\x12\n\r\n\x05\x04\r\x02\0\x01\x12\x04\x8b\
    \x01\x13\x19\n\r\n\x05\x04\r\x02\0\x03\x12\x04\x8b\x01\x1c\x1f\n\x0c\n\
    \x04\x04\r\x02\x01\x12\x04\x8c\x01\x04!\n\r\n\x05\x04\r\x02\x01\x04\x12\
    \x04\x8c\x01\x04\x0c\n\r\n\x05\x04\r\x02\x01\x05\x12\x04\x8c\x01\r\x12\n\
    \r\n\x05\x04\r\x02\x01\x01\x12\x04\x8c\x01\x13\x19\n\r\n\x05\x04\r\x02\
    \x01\x03\x12\x04\x8c\x01\x1c\x20\n\x0c\n\x04\x04\r\x02\x02\x12\x04\x8d\
    \x01\x04!\n\r\n\x05\x04\r\x02\x02\x04\x12\x04\x8d\x01\x04\x0c\n\r\n\x05\
    \x04\r\x02\x02\x05\x12\x04\x8d\x01\r\x12\n\r\n\x05\x04\r\x02\x02\x01\x12\
    \x04\x8d\x01\x13\x19\n\r\n\x05\x04\r\x02\x02\x03\x12\x04\x8d\x01\x1c\x20\
    \n\x0c\n\x02\x04\x0e\x12\x06\x91\x01\0\x94\x01\x01\n\x0b\n\x03\x04\x0e\
    \x01\x12\x04\x91\x01\x08\x1c\n\x0c\n\x04\x04\x0e\x02\0\x12\x04\x92\x01\
    \x042\n\r\n\x05\x04\x0e\x02\0\x04\x12\x04\x92\x01\x04\x0c\n\r\n\x05\x04\
    \x0e\x02\0\x06\x12\x04\x92\x01\r#\n\r\n\x05\x04\x0e\x02\0\x01\x12\x04\
    \x92\x01$+\n\r\n\x05\x04\x0e\x02\0\x03\x12\x04\x92\x01.1\n\x0c\n\x04\x04\
    \x0e\x02\x01\x12\x04\x93\x01\x04=\n\r\n\x05\x04\x0e\x02\x01\x04\x12\x04\
    \x93\x01\x04\x0c\n\r\n\x05\x04\x0e\x02\x01\x06\x12\x04\x93\x01\r'\n\r\n\
    \x05\x04\x0e\x02\x01\x01\x12\x04\x93\x01(5\n\r\n\x05\x04\x0e\x02\x01\x03\
    \x12\x04\x93\x018<\n\x0c\n\x02\x04\x0f\x12\x06\x97\x01\0\x98\x01\x01\n\
    \x0b\n\x03\x04\x0f\x01\x12\x04\x97\x01\x08\x1e\n\x0c\n\x02\x04\x10\x12\
    \x06\x9b\x01\0\x9c\x01\x01\n\x0b\n\x03\x04\x10\x01\x12\x04\x9b\x01\x08\"\
    \n\x0c\n\x02\x04\x11\x12\x06\x9f\x01\0\xa3\x01\x01\n\x0b\n\x03\x04\x11\
    \x01\x12\x04\x9f\x01\x08\x1e\n\x0c\n\x04\x04\x11\x02\0\x12\x04\xa0\x01\
    \x04-\n\r\n\x05\x04\x11\x02\0\x04\x12\x04\xa0\x01\x04\x0c\n\r\n\x05\x04\
    \x11\x02\0\x05\x12\x04\xa0\x01\r\x12\n\r\n\x05\x04\x11\x02\0\x01\x12\x04\
    \xa0\x01\x13&\n\r\n\x05\x04\x11\x02\0\x03\x12\x04\xa0\x01),\n\x0c\n\x04\
    \x04\x11\x02\x01\x12\x04\xa1\x01\x04$\n\r\n\x05\x04\x11\x02\x01\x04\x12\
    \x04\xa1\x01\x04\x0c\n\r\n\x05\x04\x11\x02\x01\x05\x12\x04\xa1\x01\r\x12\
    \n\r\n\x05\x04\x11\x02\x01\x01\x12\x04\xa1\x01\x13\x1c\n\r\n\x05\x04\x11\
    \x02\x01\x03\x12\x04\xa1\x01\x1f#\n\x0c\n\x04\x04\x11\x02\x02\x12\x04\
    \xa2\x01\x04'\n\r\n\x05\x04\x11\x02\x02\x04\x12\x04\xa2\x01\x04\x0c\n\r\
    \n\x05\x04\x11\x02\x02\x05\x12\x04\xa2\x01\r\x13\n\r\n\x05\x04\x11\x02\
    \x02\x01\x12\x04\xa2\x01\x14\x1f\n\r\n\x05\x04\x11\x02\x02\x03\x12\x04\
    \xa2\x01\"&\n\x0c\n\x02\x04\x12\x12\x06\xa5\x01\0\xaa\x01\x01\n\x0b\n\
    \x03\x04\x12\x01\x12\x04\xa5\x01\x08\x15\n\x0c\n\x04\x04\x12\x02\0\x12\
    \x04\xa6\x01\x04(\n\r\n\x05\x04\x12\x02\0\x04\x12\x04\xa6\x01\x04\x0c\n\
    \r\n\x05\x04\x12\x02\0\x06\x12\x04\xa6\x01\r\x16\n\r\n\x05\x04\x12\x02\0\
    \x01\x12\x04\xa6\x01\x17!\n\r\n\x05\x04\x12\x02\0\x03\x12\x04\xa6\x01$'\
    \n\x0c\n\x04\x04\x12\x02\x01\x12\x04\xa7\x01\x04&\n\r\n\x05\x04\x12\x02\
    \x01\x04\x12\x04\xa7\x01\x04\x0c\n\r\n\x05\x04\x12\x02\x01\x05\x12\x04\
    \xa7\x01\r\x12\n\r\n\x05\x04\x12\x02\x01\x01\x12\x04\xa7\x01\x13\x1e\n\r\
    \n\x05\x04\x12\x02\x01\x03\x12\x04\xa7\x01!%\n\x0c\n\x04\x04\x12\x02\x02\
    \x12\x04\xa8\x01\x04!\n\r\n\x05\x04\x12\x02\x02\x04\x12\x04\xa8\x01\x04\
    \x0c\n\r\n\x05\x04\x12\x02\x02\x05\x12\x04\xa8\x01\r\x12\n\r\n\x05\x04\
    \x12\x02\x02\x01\x12\x04\xa8\x01\x13\x19\n\r\n\x05\x04\x12\x02\x02\x03\
    \x12\x04\xa8\x01\x1c\x20\n\x0c\n\x04\x04\x12\x02\x03\x12\x04\xa9\x01\x04\
    -\n\r\n\x05\x04\x12\x02\x03\x04\x12\x04\xa9\x01\x04\x0c\n\r\n\x05\x04\
    \x12\x02\x03\x05\x12\x04\xa9\x01\r\x13\n\r\n\x05\x04\x12\x02\x03\x01\x12\
    \x04\xa9\x01\x14%\n\r\n\x05\x04\x12\x02\x03\x03\x12\x04\xa9\x01(,\n\x0c\
    \n\x02\x05\x06\x12\x06\xac\x01\0\xb8\x01\x01\n\x0b\n\x03\x05\x06\x01\x12\
    \x04\xac\x01\x05\x0e\n\x0c\n\x04\x05\x06\x02\0\x12\x04\xad\x01\x04\x18\n\
    \r\n\x05\x05\x06\x02\0\x01\x12\x04\xad\x01\x04\x11\n\r\n\x05\x05\x06\x02\
    \0\x02\x12\x04\xad\x01\x14\x17\n\x0c\n\x04\x05\x06\x02\x01\x12\x04\xae\
    \x01\x04\x17\n\r\n\x05\x05\x06\x02\x01\x01\x12\x04\xae\x01\x04\x10\n\r\n\
    \x05\x05\x06\x02\x01\x02\x12\x04\xae\x01\x13\x16\n\x0c\n\x04\x05\x06\x02\
    \x02\x12\x04\xaf\x01\x04\x1a\n\r\n\x05\x05\x06\x02\x02\x01\x12\x04\xaf\
    \x01\x04\x13\n\r\n\x05\x05\x06\x02\x02\x02\x12\x04\xaf\x01\x16\x19\n\x0c\
    \n\x04\x05\x06\x02\x03\x12\x04\xb0\x01\x04\x1c\n\r\n\x05\x05\x06\x02\x03\
    \x01\x12\x04\xb0\x01\x04\x15\n\r\n\x05\x05\x06\x02\x03\x02\x12\x04\xb0\
    \x01\x18\x1b\n\x0c\n\x04\x05\x06\x02\x04\x12\x04\xb1\x01\x04!\n\r\n\x05\
    \x05\x06\x02\x04\x01\x12\x04\xb1\x01\x04\x1a\n\r\n\x05\x05\x06\x02\x04\
    \x02\x12\x04\xb1\x01\x1d\x20\n\x0c\n\x04\x05\x06\x02\x05\x12\x04\xb2\x01\
    \x04\x19\n\r\n\x05\x05\x06\x02\x05\x01\x12\x04\xb2\x01\x04\x12\n\r\n\x05\
    \x05\x06\x02\x05\x02\x12\x04\xb2\x01\x15\x18\n\x0c\n\x04\x05\x06\x02\x06\
    \x12\x04\xb3\x01\x04&\n\r\n\x05\x05\x06\x02\x06\x01\x12\x04\xb3\x01\x04\
    \x1f\n\r\n\x05\x05\x06\x02\x06\x02\x12\x04\xb3\x01\"%\n\x0c\n\x04\x05\
    \x06\x02\x07\x12\x04\xb4\x01\x04\x18\n\r\n\x05\x05\x06\x02\x07\x01\x12\
    \x04\xb4\x01\x04\x11\n\r\n\x05\x05\x06\x02\x07\x02\x12\x04\xb4\x01\x14\
    \x17\n\x0c\n\x04\x05\x06\x02\x08\x12\x04\xb5\x01\x04$\n\r\n\x05\x05\x06\
    \x02\x08\x01\x12\x04\xb5\x01\x04\x1d\n\r\n\x05\x05\x06\x02\x08\x02\x12\
    \x04\xb5\x01\x20#\n\x0c\n\x04\x05\x06\x02\t\x12\x04\xb6\x01\x04\x19\n\r\
    \n\x05\x05\x06\x02\t\x01\x12\x04\xb6\x01\x04\x11\n\r\n\x05\x05\x06\x02\t\
    \x02\x12\x04\xb6\x01\x14\x18\n\x0c\n\x04\x05\x06\x02\n\x12\x04\xb7\x01\
    \x04\x1d\n\r\n\x05\x05\x06\x02\n\x01\x12\x04\xb7\x01\x04\x15\n\r\n\x05\
    \x05\x06\x02\n\x02\x12\x04\xb7\x01\x18\x1c\n\x0c\n\x02\x04\x13\x12\x06\
    \xba\x01\0\xbe\x01\x01\n\x0b\n\x03\x04\x13\x01\x12\x04\xba\x01\x08\x1f\n\
    \x0c\n\x04\x04\x13\x02\0\x12\x04\xbb\x01\x04B\n\r\n\x05\x04\x13\x02\0\
    \x04\x12\x04\xbb\x01\x04\x0c\n\r\n\x05\x04\x13\x02\0\x06\x12\x04\xbb\x01\
    \r%\n\r\n\x05\x04\x13\x02\0\x01\x12\x04\xbb\x01&;\n\r\n\x05\x04\x13\x02\
    \0\x03\x12\x04\xbb\x01>A\n\x0c\n\x04\x04\x13\x02\x01\x12\x04\xbc\x01\x04\
    2\n\r\n\x05\x04\x13\x02\x01\x04\x12\x04\xbc\x01\x04\x0c\n\r\n\x05\x04\
    \x13\x02\x01\x06\x12\x04\xbc\x01\r\x1d\n\r\n\x05\x04\x13\x02\x01\x01\x12\
    \x04\xbc\x01\x1e*\n\r\n\x05\x04\x13\x02\x01\x03\x12\x04\xbc\x01-1\n\x0c\
    \n\x04\x04\x13\x02\x02\x12\x04\xbd\x01\x048\n\r\n\x05\x04\x13\x02\x02\
    \x04\x12\x04\xbd\x01\x04\x0c\n\r\n\x05\x04\x13\x02\x02\x06\x12\x04\xbd\
    \x01\r\x20\n\r\n\x05\x04\x13\x02\x02\x01\x12\x04\xbd\x01!0\n\r\n\x05\x04\
    \x13\x02\x02\x03\x12\x04\xbd\x0137\n\x0c\n\x02\x04\x14\x12\x06\xc1\x01\0\
    \xc3\x01\x01\n\x0b\n\x03\x04\x14\x01\x12\x04\xc1\x01\x08\x20\n\x0c\n\x04\
    \x04\x14\x02\0\x12\x04\xc2\x01\x04C\n\r\n\x05\x04\x14\x02\0\x04\x12\x04\
    \xc2\x01\x04\x0c\n\r\n\x05\x04\x14\x02\0\x06\x12\x04\xc2\x01\r-\n\r\n\
    \x05\x04\x14\x02\0\x01\x12\x04\xc2\x01.<\n\r\n\x05\x04\x14\x02\0\x03\x12\
    \x04\xc2\x01?B\n\x0c\n\x02\x04\x15\x12\x06\xc6\x01\0\xc8\x01\x01\n\x0b\n\
    \x03\x04\x15\x01\x12\x04\xc6\x01\x08(\n\x0c\n\x04\x04\x15\x02\0\x12\x04\
    \xc7\x01\x04\x1e\n\r\n\x05\x04\x15\x02\0\x04\x12\x04\xc7\x01\x04\x0c\n\r\
    \n\x05\x04\x15\x02\0\x05\x12\x04\xc7\x01\r\x12\n\r\n\x05\x04\x15\x02\0\
    \x01\x12\x04\xc7\x01\x13\x17\n\r\n\x05\x04\x15\x02\0\x03\x12\x04\xc7\x01\
    \x1a\x1d\n\x0c\n\x02\x04\x16\x12\x06\xcb\x01\0\xcd\x01\x01\n\x0b\n\x03\
    \x04\x16\x01\x12\x04\xcb\x01\x08\x18\n\x0c\n\x04\x04\x16\x02\0\x12\x04\
    \xcc\x01\x041\n\r\n\x05\x04\x16\x02\0\x04\x12\x04\xcc\x01\x04\x0c\n\r\n\
    \x05\x04\x16\x02\0\x06\x12\x04\xcc\x01\r\x20\n\r\n\x05\x04\x16\x02\0\x01\
    \x12\x04\xcc\x01!*\n\r\n\x05\x04\x16\x02\0\x03\x12\x04\xcc\x01-0\n\x0c\n\
    \x02\x04\x17\x12\x06\xd0\x01\0\xd2\x01\x01\n\x0b\n\x03\x04\x17\x01\x12\
    \x04\xd0\x01\x08\x1b\n\x0c\n\x04\x04\x17\x02\0\x12\x04\xd1\x01\x04%\n\r\
    \n\x05\x04\x17\x02\0\x04\x12\x04\xd1\x01\x04\x0c\n\r\n\x05\x04\x17\x02\0\
    \x05\x12\x04\xd1\x01\r\x12\n\r\n\x05\x04\x17\x02\0\x01\x12\x04\xd1\x01\
    \x13\x1e\n\r\n\x05\x04\x17\x02\0\x03\x12\x04\xd1\x01!$\n\x0c\n\x02\x04\
    \x18\x12\x06\xd5\x01\0\xd8\x01\x01\n\x0b\n\x03\x04\x18\x01\x12\x04\xd5\
    \x01\x08\x1b\n\x0c\n\x04\x04\x18\x02\0\x12\x04\xd6\x01\x041\n\r\n\x05\
    \x04\x18\x02\0\x04\x12\x04\xd6\x01\x04\x0c\n\r\n\x05\x04\x18\x02\0\x06\
    \x12\x04\xd6\x01\r\"\n\r\n\x05\x04\x18\x02\0\x01\x12\x04\xd6\x01#*\n\r\n\
    \x05\x04\x18\x02\0\x03\x12\x04\xd6\x01-0\n\x0c\n\x04\x04\x18\x02\x01\x12\
    \x04\xd7\x01\x04<\n\r\n\x05\x04\x18\x02\x01\x04\x12\x04\xd7\x01\x04\x0c\
    \n\r\n\x05\x04\x18\x02\x01\x06\x12\x04\xd7\x01\r&\n\r\n\x05\x04\x18\x02\
    \x01\x01\x12\x04\xd7\x01'4\n\r\n\x05\x04\x18\x02\x01\x03\x12\x04\xd7\x01\
    7;\n\x0c\n\x02\x04\x19\x12\x06\xdb\x01\0\xdd\x01\x01\n\x0b\n\x03\x04\x19\
    \x01\x12\x04\xdb\x01\x08\x1d\n\x0c\n\x04\x04\x19\x02\0\x12\x04\xdc\x01\
    \x04\x1f\n\r\n\x05\x04\x19\x02\0\x04\x12\x04\xdc\x01\x04\x0c\n\r\n\x05\
    \x04\x19\x02\0\x05\x12\x04\xdc\x01\r\x12\n\r\n\x05\x04\x19\x02\0\x01\x12\
    \x04\xdc\x01\x13\x18\n\r\n\x05\x04\x19\x02\0\x03\x12\x04\xdc\x01\x1b\x1e\
    \n\x0c\n\x02\x04\x1a\x12\x06\xe0\x01\0\xe2\x01\x01\n\x0b\n\x03\x04\x1a\
    \x01\x12\x04\xe0\x01\x08!\n\x0c\n\x04\x04\x1a\x02\0\x12\x04\xe1\x01\x04\
    \x1f\n\r\n\x05\x04\x1a\x02\0\x04\x12\x04\xe1\x01\x04\x0c\n\r\n\x05\x04\
    \x1a\x02\0\x05\x12\x04\xe1\x01\r\x12\n\r\n\x05\x04\x1a\x02\0\x01\x12\x04\
    \xe1\x01\x13\x18\n\r\n\x05\x04\x1a\x02\0\x03\x12\x04\xe1\x01\x1b\x1e\
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
