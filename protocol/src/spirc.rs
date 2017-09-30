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
pub struct Frame {
    // message fields
    version: ::std::option::Option<u32>,
    ident: ::protobuf::SingularField<::std::string::String>,
    protocol_version: ::protobuf::SingularField<::std::string::String>,
    seq_nr: ::std::option::Option<u32>,
    typ: ::std::option::Option<MessageType>,
    device_state: ::protobuf::SingularPtrField<DeviceState>,
    goodbye: ::protobuf::SingularPtrField<Goodbye>,
    state: ::protobuf::SingularPtrField<State>,
    position: ::std::option::Option<u32>,
    volume: ::std::option::Option<u32>,
    state_update_id: ::std::option::Option<i64>,
    recipient: ::protobuf::RepeatedField<::std::string::String>,
    context_player_state: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    new_name: ::protobuf::SingularField<::std::string::String>,
    metadata: ::protobuf::SingularPtrField<Metadata>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Frame {}

impl Frame {
    pub fn new() -> Frame {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Frame {
        static mut instance: ::protobuf::lazy::Lazy<Frame> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Frame,
        };
        unsafe {
            instance.get(Frame::new)
        }
    }

    // optional uint32 version = 1;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u32) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u32 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.version
    }

    // optional string ident = 2;

    pub fn clear_ident(&mut self) {
        self.ident.clear();
    }

    pub fn has_ident(&self) -> bool {
        self.ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ident(&mut self, v: ::std::string::String) {
        self.ident = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ident(&mut self) -> &mut ::std::string::String {
        if self.ident.is_none() {
            self.ident.set_default();
        }
        self.ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_ident(&mut self) -> ::std::string::String {
        self.ident.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ident(&self) -> &str {
        match self.ident.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_ident_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.ident
    }

    fn mut_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.ident
    }

    // optional string protocol_version = 3;

    pub fn clear_protocol_version(&mut self) {
        self.protocol_version.clear();
    }

    pub fn has_protocol_version(&self) -> bool {
        self.protocol_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_protocol_version(&mut self, v: ::std::string::String) {
        self.protocol_version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_protocol_version(&mut self) -> &mut ::std::string::String {
        if self.protocol_version.is_none() {
            self.protocol_version.set_default();
        }
        self.protocol_version.as_mut().unwrap()
    }

    // Take field
    pub fn take_protocol_version(&mut self) -> ::std::string::String {
        self.protocol_version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_protocol_version(&self) -> &str {
        match self.protocol_version.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_protocol_version_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.protocol_version
    }

    fn mut_protocol_version_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.protocol_version
    }

    // optional uint32 seq_nr = 4;

    pub fn clear_seq_nr(&mut self) {
        self.seq_nr = ::std::option::Option::None;
    }

    pub fn has_seq_nr(&self) -> bool {
        self.seq_nr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_nr(&mut self, v: u32) {
        self.seq_nr = ::std::option::Option::Some(v);
    }

    pub fn get_seq_nr(&self) -> u32 {
        self.seq_nr.unwrap_or(0)
    }

    fn get_seq_nr_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seq_nr
    }

    fn mut_seq_nr_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seq_nr
    }

    // optional .MessageType typ = 5;

    pub fn clear_typ(&mut self) {
        self.typ = ::std::option::Option::None;
    }

    pub fn has_typ(&self) -> bool {
        self.typ.is_some()
    }

    // Param is passed by value, moved
    pub fn set_typ(&mut self, v: MessageType) {
        self.typ = ::std::option::Option::Some(v);
    }

    pub fn get_typ(&self) -> MessageType {
        self.typ.unwrap_or(MessageType::kMessageTypeHello)
    }

    fn get_typ_for_reflect(&self) -> &::std::option::Option<MessageType> {
        &self.typ
    }

    fn mut_typ_for_reflect(&mut self) -> &mut ::std::option::Option<MessageType> {
        &mut self.typ
    }

    // optional .DeviceState device_state = 7;

    pub fn clear_device_state(&mut self) {
        self.device_state.clear();
    }

    pub fn has_device_state(&self) -> bool {
        self.device_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_device_state(&mut self, v: DeviceState) {
        self.device_state = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device_state(&mut self) -> &mut DeviceState {
        if self.device_state.is_none() {
            self.device_state.set_default();
        }
        self.device_state.as_mut().unwrap()
    }

    // Take field
    pub fn take_device_state(&mut self) -> DeviceState {
        self.device_state.take().unwrap_or_else(|| DeviceState::new())
    }

    pub fn get_device_state(&self) -> &DeviceState {
        self.device_state.as_ref().unwrap_or_else(|| DeviceState::default_instance())
    }

    fn get_device_state_for_reflect(&self) -> &::protobuf::SingularPtrField<DeviceState> {
        &self.device_state
    }

    fn mut_device_state_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DeviceState> {
        &mut self.device_state
    }

    // optional .Goodbye goodbye = 11;

    pub fn clear_goodbye(&mut self) {
        self.goodbye.clear();
    }

    pub fn has_goodbye(&self) -> bool {
        self.goodbye.is_some()
    }

    // Param is passed by value, moved
    pub fn set_goodbye(&mut self, v: Goodbye) {
        self.goodbye = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_goodbye(&mut self) -> &mut Goodbye {
        if self.goodbye.is_none() {
            self.goodbye.set_default();
        }
        self.goodbye.as_mut().unwrap()
    }

    // Take field
    pub fn take_goodbye(&mut self) -> Goodbye {
        self.goodbye.take().unwrap_or_else(|| Goodbye::new())
    }

    pub fn get_goodbye(&self) -> &Goodbye {
        self.goodbye.as_ref().unwrap_or_else(|| Goodbye::default_instance())
    }

    fn get_goodbye_for_reflect(&self) -> &::protobuf::SingularPtrField<Goodbye> {
        &self.goodbye
    }

    fn mut_goodbye_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Goodbye> {
        &mut self.goodbye
    }

    // optional .State state = 12;

    pub fn clear_state(&mut self) {
        self.state.clear();
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: State) {
        self.state = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_state(&mut self) -> &mut State {
        if self.state.is_none() {
            self.state.set_default();
        }
        self.state.as_mut().unwrap()
    }

    // Take field
    pub fn take_state(&mut self) -> State {
        self.state.take().unwrap_or_else(|| State::new())
    }

    pub fn get_state(&self) -> &State {
        self.state.as_ref().unwrap_or_else(|| State::default_instance())
    }

    fn get_state_for_reflect(&self) -> &::protobuf::SingularPtrField<State> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<State> {
        &mut self.state
    }

    // optional uint32 position = 13;

    pub fn clear_position(&mut self) {
        self.position = ::std::option::Option::None;
    }

    pub fn has_position(&self) -> bool {
        self.position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_position(&mut self, v: u32) {
        self.position = ::std::option::Option::Some(v);
    }

    pub fn get_position(&self) -> u32 {
        self.position.unwrap_or(0)
    }

    fn get_position_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.position
    }

    fn mut_position_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.position
    }

    // optional uint32 volume = 14;

    pub fn clear_volume(&mut self) {
        self.volume = ::std::option::Option::None;
    }

    pub fn has_volume(&self) -> bool {
        self.volume.is_some()
    }

    // Param is passed by value, moved
    pub fn set_volume(&mut self, v: u32) {
        self.volume = ::std::option::Option::Some(v);
    }

    pub fn get_volume(&self) -> u32 {
        self.volume.unwrap_or(0)
    }

    fn get_volume_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.volume
    }

    fn mut_volume_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.volume
    }

    // optional int64 state_update_id = 17;

    pub fn clear_state_update_id(&mut self) {
        self.state_update_id = ::std::option::Option::None;
    }

    pub fn has_state_update_id(&self) -> bool {
        self.state_update_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state_update_id(&mut self, v: i64) {
        self.state_update_id = ::std::option::Option::Some(v);
    }

    pub fn get_state_update_id(&self) -> i64 {
        self.state_update_id.unwrap_or(0)
    }

    fn get_state_update_id_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.state_update_id
    }

    fn mut_state_update_id_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.state_update_id
    }

    // repeated string recipient = 18;

    pub fn clear_recipient(&mut self) {
        self.recipient.clear();
    }

    // Param is passed by value, moved
    pub fn set_recipient(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.recipient = v;
    }

    // Mutable pointer to the field.
    pub fn mut_recipient(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.recipient
    }

    // Take field
    pub fn take_recipient(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.recipient, ::protobuf::RepeatedField::new())
    }

    pub fn get_recipient(&self) -> &[::std::string::String] {
        &self.recipient
    }

    fn get_recipient_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.recipient
    }

    fn mut_recipient_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.recipient
    }

    // optional bytes context_player_state = 19;

    pub fn clear_context_player_state(&mut self) {
        self.context_player_state.clear();
    }

    pub fn has_context_player_state(&self) -> bool {
        self.context_player_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context_player_state(&mut self, v: ::std::vec::Vec<u8>) {
        self.context_player_state = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context_player_state(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.context_player_state.is_none() {
            self.context_player_state.set_default();
        }
        self.context_player_state.as_mut().unwrap()
    }

    // Take field
    pub fn take_context_player_state(&mut self) -> ::std::vec::Vec<u8> {
        self.context_player_state.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_context_player_state(&self) -> &[u8] {
        match self.context_player_state.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_context_player_state_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.context_player_state
    }

    fn mut_context_player_state_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.context_player_state
    }

    // optional string new_name = 20;

    pub fn clear_new_name(&mut self) {
        self.new_name.clear();
    }

    pub fn has_new_name(&self) -> bool {
        self.new_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_name(&mut self, v: ::std::string::String) {
        self.new_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_new_name(&mut self) -> &mut ::std::string::String {
        if self.new_name.is_none() {
            self.new_name.set_default();
        }
        self.new_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_new_name(&mut self) -> ::std::string::String {
        self.new_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_new_name(&self) -> &str {
        match self.new_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_new_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.new_name
    }

    fn mut_new_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.new_name
    }

    // optional .Metadata metadata = 25;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: Metadata) {
        self.metadata = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut Metadata {
        if self.metadata.is_none() {
            self.metadata.set_default();
        }
        self.metadata.as_mut().unwrap()
    }

    // Take field
    pub fn take_metadata(&mut self) -> Metadata {
        self.metadata.take().unwrap_or_else(|| Metadata::new())
    }

    pub fn get_metadata(&self) -> &Metadata {
        self.metadata.as_ref().unwrap_or_else(|| Metadata::default_instance())
    }

    fn get_metadata_for_reflect(&self) -> &::protobuf::SingularPtrField<Metadata> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Metadata> {
        &mut self.metadata
    }
}

impl ::protobuf::Message for Frame {
    fn is_initialized(&self) -> bool {
        for v in &self.device_state {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.goodbye {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.state {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.metadata {
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
                    let tmp = is.read_uint32()?;
                    self.version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ident)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.protocol_version)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.seq_nr = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.typ = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.device_state)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.goodbye)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.state)?;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.position = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.volume = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.state_update_id = ::std::option::Option::Some(tmp);
                },
                18 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.recipient)?;
                },
                19 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.context_player_state)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.new_name)?;
                },
                25 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.metadata)?;
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
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.ident.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.protocol_version.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.seq_nr {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.typ {
            my_size += ::protobuf::rt::enum_size(5, v);
        }
        if let Some(ref v) = self.device_state.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.goodbye.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.state.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.position {
            my_size += ::protobuf::rt::value_size(13, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.volume {
            my_size += ::protobuf::rt::value_size(14, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.state_update_id {
            my_size += ::protobuf::rt::value_size(17, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.recipient {
            my_size += ::protobuf::rt::string_size(18, &value);
        };
        if let Some(ref v) = self.context_player_state.as_ref() {
            my_size += ::protobuf::rt::bytes_size(19, &v);
        }
        if let Some(ref v) = self.new_name.as_ref() {
            my_size += ::protobuf::rt::string_size(20, &v);
        }
        if let Some(ref v) = self.metadata.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.ident.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.protocol_version.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(v) = self.seq_nr {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.typ {
            os.write_enum(5, v.value())?;
        }
        if let Some(ref v) = self.device_state.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.goodbye.as_ref() {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.state.as_ref() {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.position {
            os.write_uint32(13, v)?;
        }
        if let Some(v) = self.volume {
            os.write_uint32(14, v)?;
        }
        if let Some(v) = self.state_update_id {
            os.write_int64(17, v)?;
        }
        for v in &self.recipient {
            os.write_string(18, &v)?;
        };
        if let Some(ref v) = self.context_player_state.as_ref() {
            os.write_bytes(19, &v)?;
        }
        if let Some(ref v) = self.new_name.as_ref() {
            os.write_string(20, &v)?;
        }
        if let Some(ref v) = self.metadata.as_ref() {
            os.write_tag(25, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Frame {
    fn new() -> Frame {
        Frame::new()
    }

    fn descriptor_static(_: ::std::option::Option<Frame>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "version",
                    Frame::get_version_for_reflect,
                    Frame::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ident",
                    Frame::get_ident_for_reflect,
                    Frame::mut_ident_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "protocol_version",
                    Frame::get_protocol_version_for_reflect,
                    Frame::mut_protocol_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq_nr",
                    Frame::get_seq_nr_for_reflect,
                    Frame::mut_seq_nr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<MessageType>>(
                    "typ",
                    Frame::get_typ_for_reflect,
                    Frame::mut_typ_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DeviceState>>(
                    "device_state",
                    Frame::get_device_state_for_reflect,
                    Frame::mut_device_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Goodbye>>(
                    "goodbye",
                    Frame::get_goodbye_for_reflect,
                    Frame::mut_goodbye_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<State>>(
                    "state",
                    Frame::get_state_for_reflect,
                    Frame::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "position",
                    Frame::get_position_for_reflect,
                    Frame::mut_position_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "volume",
                    Frame::get_volume_for_reflect,
                    Frame::mut_volume_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "state_update_id",
                    Frame::get_state_update_id_for_reflect,
                    Frame::mut_state_update_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "recipient",
                    Frame::get_recipient_for_reflect,
                    Frame::mut_recipient_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "context_player_state",
                    Frame::get_context_player_state_for_reflect,
                    Frame::mut_context_player_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "new_name",
                    Frame::get_new_name_for_reflect,
                    Frame::mut_new_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Metadata>>(
                    "metadata",
                    Frame::get_metadata_for_reflect,
                    Frame::mut_metadata_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Frame>(
                    "Frame",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Frame {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_ident();
        self.clear_protocol_version();
        self.clear_seq_nr();
        self.clear_typ();
        self.clear_device_state();
        self.clear_goodbye();
        self.clear_state();
        self.clear_position();
        self.clear_volume();
        self.clear_state_update_id();
        self.clear_recipient();
        self.clear_context_player_state();
        self.clear_new_name();
        self.clear_metadata();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Frame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Frame {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeviceState {
    // message fields
    sw_version: ::protobuf::SingularField<::std::string::String>,
    is_active: ::std::option::Option<bool>,
    can_play: ::std::option::Option<bool>,
    volume: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    error_code: ::std::option::Option<u32>,
    became_active_at: ::std::option::Option<i64>,
    error_message: ::protobuf::SingularField<::std::string::String>,
    capabilities: ::protobuf::RepeatedField<Capability>,
    context_player_error: ::protobuf::SingularField<::std::string::String>,
    metadata: ::protobuf::RepeatedField<Metadata>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeviceState {}

impl DeviceState {
    pub fn new() -> DeviceState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeviceState {
        static mut instance: ::protobuf::lazy::Lazy<DeviceState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeviceState,
        };
        unsafe {
            instance.get(DeviceState::new)
        }
    }

    // optional string sw_version = 1;

    pub fn clear_sw_version(&mut self) {
        self.sw_version.clear();
    }

    pub fn has_sw_version(&self) -> bool {
        self.sw_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sw_version(&mut self, v: ::std::string::String) {
        self.sw_version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sw_version(&mut self) -> &mut ::std::string::String {
        if self.sw_version.is_none() {
            self.sw_version.set_default();
        }
        self.sw_version.as_mut().unwrap()
    }

    // Take field
    pub fn take_sw_version(&mut self) -> ::std::string::String {
        self.sw_version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_sw_version(&self) -> &str {
        match self.sw_version.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_sw_version_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.sw_version
    }

    fn mut_sw_version_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.sw_version
    }

    // optional bool is_active = 10;

    pub fn clear_is_active(&mut self) {
        self.is_active = ::std::option::Option::None;
    }

    pub fn has_is_active(&self) -> bool {
        self.is_active.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_active(&mut self, v: bool) {
        self.is_active = ::std::option::Option::Some(v);
    }

    pub fn get_is_active(&self) -> bool {
        self.is_active.unwrap_or(false)
    }

    fn get_is_active_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_active
    }

    fn mut_is_active_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_active
    }

    // optional bool can_play = 11;

    pub fn clear_can_play(&mut self) {
        self.can_play = ::std::option::Option::None;
    }

    pub fn has_can_play(&self) -> bool {
        self.can_play.is_some()
    }

    // Param is passed by value, moved
    pub fn set_can_play(&mut self, v: bool) {
        self.can_play = ::std::option::Option::Some(v);
    }

    pub fn get_can_play(&self) -> bool {
        self.can_play.unwrap_or(false)
    }

    fn get_can_play_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.can_play
    }

    fn mut_can_play_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.can_play
    }

    // optional uint32 volume = 12;

    pub fn clear_volume(&mut self) {
        self.volume = ::std::option::Option::None;
    }

    pub fn has_volume(&self) -> bool {
        self.volume.is_some()
    }

    // Param is passed by value, moved
    pub fn set_volume(&mut self, v: u32) {
        self.volume = ::std::option::Option::Some(v);
    }

    pub fn get_volume(&self) -> u32 {
        self.volume.unwrap_or(0)
    }

    fn get_volume_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.volume
    }

    fn mut_volume_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.volume
    }

    // optional string name = 13;

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

    // optional uint32 error_code = 14;

    pub fn clear_error_code(&mut self) {
        self.error_code = ::std::option::Option::None;
    }

    pub fn has_error_code(&self) -> bool {
        self.error_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error_code(&mut self, v: u32) {
        self.error_code = ::std::option::Option::Some(v);
    }

    pub fn get_error_code(&self) -> u32 {
        self.error_code.unwrap_or(0)
    }

    fn get_error_code_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.error_code
    }

    fn mut_error_code_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.error_code
    }

    // optional int64 became_active_at = 15;

    pub fn clear_became_active_at(&mut self) {
        self.became_active_at = ::std::option::Option::None;
    }

    pub fn has_became_active_at(&self) -> bool {
        self.became_active_at.is_some()
    }

    // Param is passed by value, moved
    pub fn set_became_active_at(&mut self, v: i64) {
        self.became_active_at = ::std::option::Option::Some(v);
    }

    pub fn get_became_active_at(&self) -> i64 {
        self.became_active_at.unwrap_or(0)
    }

    fn get_became_active_at_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.became_active_at
    }

    fn mut_became_active_at_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.became_active_at
    }

    // optional string error_message = 16;

    pub fn clear_error_message(&mut self) {
        self.error_message.clear();
    }

    pub fn has_error_message(&self) -> bool {
        self.error_message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error_message(&mut self, v: ::std::string::String) {
        self.error_message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error_message(&mut self) -> &mut ::std::string::String {
        if self.error_message.is_none() {
            self.error_message.set_default();
        }
        self.error_message.as_mut().unwrap()
    }

    // Take field
    pub fn take_error_message(&mut self) -> ::std::string::String {
        self.error_message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error_message(&self) -> &str {
        match self.error_message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_error_message_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.error_message
    }

    fn mut_error_message_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.error_message
    }

    // repeated .Capability capabilities = 17;

    pub fn clear_capabilities(&mut self) {
        self.capabilities.clear();
    }

    // Param is passed by value, moved
    pub fn set_capabilities(&mut self, v: ::protobuf::RepeatedField<Capability>) {
        self.capabilities = v;
    }

    // Mutable pointer to the field.
    pub fn mut_capabilities(&mut self) -> &mut ::protobuf::RepeatedField<Capability> {
        &mut self.capabilities
    }

    // Take field
    pub fn take_capabilities(&mut self) -> ::protobuf::RepeatedField<Capability> {
        ::std::mem::replace(&mut self.capabilities, ::protobuf::RepeatedField::new())
    }

    pub fn get_capabilities(&self) -> &[Capability] {
        &self.capabilities
    }

    fn get_capabilities_for_reflect(&self) -> &::protobuf::RepeatedField<Capability> {
        &self.capabilities
    }

    fn mut_capabilities_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Capability> {
        &mut self.capabilities
    }

    // optional string context_player_error = 20;

    pub fn clear_context_player_error(&mut self) {
        self.context_player_error.clear();
    }

    pub fn has_context_player_error(&self) -> bool {
        self.context_player_error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context_player_error(&mut self, v: ::std::string::String) {
        self.context_player_error = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context_player_error(&mut self) -> &mut ::std::string::String {
        if self.context_player_error.is_none() {
            self.context_player_error.set_default();
        }
        self.context_player_error.as_mut().unwrap()
    }

    // Take field
    pub fn take_context_player_error(&mut self) -> ::std::string::String {
        self.context_player_error.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_context_player_error(&self) -> &str {
        match self.context_player_error.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_context_player_error_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.context_player_error
    }

    fn mut_context_player_error_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.context_player_error
    }

    // repeated .Metadata metadata = 25;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: ::protobuf::RepeatedField<Metadata>) {
        self.metadata = v;
    }

    // Mutable pointer to the field.
    pub fn mut_metadata(&mut self) -> &mut ::protobuf::RepeatedField<Metadata> {
        &mut self.metadata
    }

    // Take field
    pub fn take_metadata(&mut self) -> ::protobuf::RepeatedField<Metadata> {
        ::std::mem::replace(&mut self.metadata, ::protobuf::RepeatedField::new())
    }

    pub fn get_metadata(&self) -> &[Metadata] {
        &self.metadata
    }

    fn get_metadata_for_reflect(&self) -> &::protobuf::RepeatedField<Metadata> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Metadata> {
        &mut self.metadata
    }
}

impl ::protobuf::Message for DeviceState {
    fn is_initialized(&self) -> bool {
        for v in &self.capabilities {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.metadata {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.sw_version)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_active = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.can_play = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.volume = ::std::option::Option::Some(tmp);
                },
                13 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.error_code = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.became_active_at = ::std::option::Option::Some(tmp);
                },
                16 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error_message)?;
                },
                17 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.capabilities)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.context_player_error)?;
                },
                25 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.metadata)?;
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
        if let Some(ref v) = self.sw_version.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.is_active {
            my_size += 2;
        }
        if let Some(v) = self.can_play {
            my_size += 2;
        }
        if let Some(v) = self.volume {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(13, &v);
        }
        if let Some(v) = self.error_code {
            my_size += ::protobuf::rt::value_size(14, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.became_active_at {
            my_size += ::protobuf::rt::value_size(15, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.error_message.as_ref() {
            my_size += ::protobuf::rt::string_size(16, &v);
        }
        for value in &self.capabilities {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.context_player_error.as_ref() {
            my_size += ::protobuf::rt::string_size(20, &v);
        }
        for value in &self.metadata {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.sw_version.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.is_active {
            os.write_bool(10, v)?;
        }
        if let Some(v) = self.can_play {
            os.write_bool(11, v)?;
        }
        if let Some(v) = self.volume {
            os.write_uint32(12, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(13, &v)?;
        }
        if let Some(v) = self.error_code {
            os.write_uint32(14, v)?;
        }
        if let Some(v) = self.became_active_at {
            os.write_int64(15, v)?;
        }
        if let Some(ref v) = self.error_message.as_ref() {
            os.write_string(16, &v)?;
        }
        for v in &self.capabilities {
            os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.context_player_error.as_ref() {
            os.write_string(20, &v)?;
        }
        for v in &self.metadata {
            os.write_tag(25, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for DeviceState {
    fn new() -> DeviceState {
        DeviceState::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeviceState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "sw_version",
                    DeviceState::get_sw_version_for_reflect,
                    DeviceState::mut_sw_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_active",
                    DeviceState::get_is_active_for_reflect,
                    DeviceState::mut_is_active_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "can_play",
                    DeviceState::get_can_play_for_reflect,
                    DeviceState::mut_can_play_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "volume",
                    DeviceState::get_volume_for_reflect,
                    DeviceState::mut_volume_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    DeviceState::get_name_for_reflect,
                    DeviceState::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "error_code",
                    DeviceState::get_error_code_for_reflect,
                    DeviceState::mut_error_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "became_active_at",
                    DeviceState::get_became_active_at_for_reflect,
                    DeviceState::mut_became_active_at_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error_message",
                    DeviceState::get_error_message_for_reflect,
                    DeviceState::mut_error_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Capability>>(
                    "capabilities",
                    DeviceState::get_capabilities_for_reflect,
                    DeviceState::mut_capabilities_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "context_player_error",
                    DeviceState::get_context_player_error_for_reflect,
                    DeviceState::mut_context_player_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Metadata>>(
                    "metadata",
                    DeviceState::get_metadata_for_reflect,
                    DeviceState::mut_metadata_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeviceState>(
                    "DeviceState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeviceState {
    fn clear(&mut self) {
        self.clear_sw_version();
        self.clear_is_active();
        self.clear_can_play();
        self.clear_volume();
        self.clear_name();
        self.clear_error_code();
        self.clear_became_active_at();
        self.clear_error_message();
        self.clear_capabilities();
        self.clear_context_player_error();
        self.clear_metadata();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeviceState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeviceState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Capability {
    // message fields
    typ: ::std::option::Option<CapabilityType>,
    intValue: ::std::vec::Vec<i64>,
    stringValue: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Capability {}

impl Capability {
    pub fn new() -> Capability {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Capability {
        static mut instance: ::protobuf::lazy::Lazy<Capability> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Capability,
        };
        unsafe {
            instance.get(Capability::new)
        }
    }

    // optional .CapabilityType typ = 1;

    pub fn clear_typ(&mut self) {
        self.typ = ::std::option::Option::None;
    }

    pub fn has_typ(&self) -> bool {
        self.typ.is_some()
    }

    // Param is passed by value, moved
    pub fn set_typ(&mut self, v: CapabilityType) {
        self.typ = ::std::option::Option::Some(v);
    }

    pub fn get_typ(&self) -> CapabilityType {
        self.typ.unwrap_or(CapabilityType::kSupportedContexts)
    }

    fn get_typ_for_reflect(&self) -> &::std::option::Option<CapabilityType> {
        &self.typ
    }

    fn mut_typ_for_reflect(&mut self) -> &mut ::std::option::Option<CapabilityType> {
        &mut self.typ
    }

    // repeated int64 intValue = 2;

    pub fn clear_intValue(&mut self) {
        self.intValue.clear();
    }

    // Param is passed by value, moved
    pub fn set_intValue(&mut self, v: ::std::vec::Vec<i64>) {
        self.intValue = v;
    }

    // Mutable pointer to the field.
    pub fn mut_intValue(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.intValue
    }

    // Take field
    pub fn take_intValue(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.intValue, ::std::vec::Vec::new())
    }

    pub fn get_intValue(&self) -> &[i64] {
        &self.intValue
    }

    fn get_intValue_for_reflect(&self) -> &::std::vec::Vec<i64> {
        &self.intValue
    }

    fn mut_intValue_for_reflect(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.intValue
    }

    // repeated string stringValue = 3;

    pub fn clear_stringValue(&mut self) {
        self.stringValue.clear();
    }

    // Param is passed by value, moved
    pub fn set_stringValue(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.stringValue = v;
    }

    // Mutable pointer to the field.
    pub fn mut_stringValue(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.stringValue
    }

    // Take field
    pub fn take_stringValue(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.stringValue, ::protobuf::RepeatedField::new())
    }

    pub fn get_stringValue(&self) -> &[::std::string::String] {
        &self.stringValue
    }

    fn get_stringValue_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.stringValue
    }

    fn mut_stringValue_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.stringValue
    }
}

impl ::protobuf::Message for Capability {
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
                    ::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.intValue)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.stringValue)?;
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
        for value in &self.intValue {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.stringValue {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.typ {
            os.write_enum(1, v.value())?;
        }
        for v in &self.intValue {
            os.write_int64(2, *v)?;
        };
        for v in &self.stringValue {
            os.write_string(3, &v)?;
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

impl ::protobuf::MessageStatic for Capability {
    fn new() -> Capability {
        Capability::new()
    }

    fn descriptor_static(_: ::std::option::Option<Capability>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CapabilityType>>(
                    "typ",
                    Capability::get_typ_for_reflect,
                    Capability::mut_typ_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "intValue",
                    Capability::get_intValue_for_reflect,
                    Capability::mut_intValue_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "stringValue",
                    Capability::get_stringValue_for_reflect,
                    Capability::mut_stringValue_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Capability>(
                    "Capability",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Capability {
    fn clear(&mut self) {
        self.clear_typ();
        self.clear_intValue();
        self.clear_stringValue();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Capability {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Capability {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Goodbye {
    // message fields
    reason: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Goodbye {}

impl Goodbye {
    pub fn new() -> Goodbye {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Goodbye {
        static mut instance: ::protobuf::lazy::Lazy<Goodbye> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Goodbye,
        };
        unsafe {
            instance.get(Goodbye::new)
        }
    }

    // optional string reason = 1;

    pub fn clear_reason(&mut self) {
        self.reason.clear();
    }

    pub fn has_reason(&self) -> bool {
        self.reason.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reason(&mut self, v: ::std::string::String) {
        self.reason = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reason(&mut self) -> &mut ::std::string::String {
        if self.reason.is_none() {
            self.reason.set_default();
        }
        self.reason.as_mut().unwrap()
    }

    // Take field
    pub fn take_reason(&mut self) -> ::std::string::String {
        self.reason.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_reason(&self) -> &str {
        match self.reason.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_reason_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.reason
    }

    fn mut_reason_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.reason
    }
}

impl ::protobuf::Message for Goodbye {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.reason)?;
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
        if let Some(ref v) = self.reason.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.reason.as_ref() {
            os.write_string(1, &v)?;
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

impl ::protobuf::MessageStatic for Goodbye {
    fn new() -> Goodbye {
        Goodbye::new()
    }

    fn descriptor_static(_: ::std::option::Option<Goodbye>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "reason",
                    Goodbye::get_reason_for_reflect,
                    Goodbye::mut_reason_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Goodbye>(
                    "Goodbye",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Goodbye {
    fn clear(&mut self) {
        self.clear_reason();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Goodbye {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Goodbye {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct State {
    // message fields
    context_uri: ::protobuf::SingularField<::std::string::String>,
    index: ::std::option::Option<u32>,
    position_ms: ::std::option::Option<u32>,
    status: ::std::option::Option<PlayStatus>,
    position_measured_at: ::std::option::Option<u64>,
    context_description: ::protobuf::SingularField<::std::string::String>,
    shuffle: ::std::option::Option<bool>,
    repeat: ::std::option::Option<bool>,
    last_command_ident: ::protobuf::SingularField<::std::string::String>,
    last_command_msgid: ::std::option::Option<u32>,
    playing_from_fallback: ::std::option::Option<bool>,
    row: ::std::option::Option<u32>,
    playing_track_index: ::std::option::Option<u32>,
    track: ::protobuf::RepeatedField<TrackRef>,
    ad: ::protobuf::SingularPtrField<Ad>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for State {}

impl State {
    pub fn new() -> State {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static State {
        static mut instance: ::protobuf::lazy::Lazy<State> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const State,
        };
        unsafe {
            instance.get(State::new)
        }
    }

    // optional string context_uri = 2;

    pub fn clear_context_uri(&mut self) {
        self.context_uri.clear();
    }

    pub fn has_context_uri(&self) -> bool {
        self.context_uri.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context_uri(&mut self, v: ::std::string::String) {
        self.context_uri = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context_uri(&mut self) -> &mut ::std::string::String {
        if self.context_uri.is_none() {
            self.context_uri.set_default();
        }
        self.context_uri.as_mut().unwrap()
    }

    // Take field
    pub fn take_context_uri(&mut self) -> ::std::string::String {
        self.context_uri.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_context_uri(&self) -> &str {
        match self.context_uri.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_context_uri_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.context_uri
    }

    fn mut_context_uri_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.context_uri
    }

    // optional uint32 index = 3;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: u32) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index(&self) -> u32 {
        self.index.unwrap_or(0)
    }

    fn get_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.index
    }

    // optional uint32 position_ms = 4;

    pub fn clear_position_ms(&mut self) {
        self.position_ms = ::std::option::Option::None;
    }

    pub fn has_position_ms(&self) -> bool {
        self.position_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_position_ms(&mut self, v: u32) {
        self.position_ms = ::std::option::Option::Some(v);
    }

    pub fn get_position_ms(&self) -> u32 {
        self.position_ms.unwrap_or(0)
    }

    fn get_position_ms_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.position_ms
    }

    fn mut_position_ms_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.position_ms
    }

    // optional .PlayStatus status = 5;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: PlayStatus) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> PlayStatus {
        self.status.unwrap_or(PlayStatus::kPlayStatusStop)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<PlayStatus> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<PlayStatus> {
        &mut self.status
    }

    // optional uint64 position_measured_at = 7;

    pub fn clear_position_measured_at(&mut self) {
        self.position_measured_at = ::std::option::Option::None;
    }

    pub fn has_position_measured_at(&self) -> bool {
        self.position_measured_at.is_some()
    }

    // Param is passed by value, moved
    pub fn set_position_measured_at(&mut self, v: u64) {
        self.position_measured_at = ::std::option::Option::Some(v);
    }

    pub fn get_position_measured_at(&self) -> u64 {
        self.position_measured_at.unwrap_or(0)
    }

    fn get_position_measured_at_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.position_measured_at
    }

    fn mut_position_measured_at_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.position_measured_at
    }

    // optional string context_description = 8;

    pub fn clear_context_description(&mut self) {
        self.context_description.clear();
    }

    pub fn has_context_description(&self) -> bool {
        self.context_description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context_description(&mut self, v: ::std::string::String) {
        self.context_description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context_description(&mut self) -> &mut ::std::string::String {
        if self.context_description.is_none() {
            self.context_description.set_default();
        }
        self.context_description.as_mut().unwrap()
    }

    // Take field
    pub fn take_context_description(&mut self) -> ::std::string::String {
        self.context_description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_context_description(&self) -> &str {
        match self.context_description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_context_description_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.context_description
    }

    fn mut_context_description_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.context_description
    }

    // optional bool shuffle = 13;

    pub fn clear_shuffle(&mut self) {
        self.shuffle = ::std::option::Option::None;
    }

    pub fn has_shuffle(&self) -> bool {
        self.shuffle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shuffle(&mut self, v: bool) {
        self.shuffle = ::std::option::Option::Some(v);
    }

    pub fn get_shuffle(&self) -> bool {
        self.shuffle.unwrap_or(false)
    }

    fn get_shuffle_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.shuffle
    }

    fn mut_shuffle_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.shuffle
    }

    // optional bool repeat = 14;

    pub fn clear_repeat(&mut self) {
        self.repeat = ::std::option::Option::None;
    }

    pub fn has_repeat(&self) -> bool {
        self.repeat.is_some()
    }

    // Param is passed by value, moved
    pub fn set_repeat(&mut self, v: bool) {
        self.repeat = ::std::option::Option::Some(v);
    }

    pub fn get_repeat(&self) -> bool {
        self.repeat.unwrap_or(false)
    }

    fn get_repeat_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.repeat
    }

    fn mut_repeat_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.repeat
    }

    // optional string last_command_ident = 20;

    pub fn clear_last_command_ident(&mut self) {
        self.last_command_ident.clear();
    }

    pub fn has_last_command_ident(&self) -> bool {
        self.last_command_ident.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_command_ident(&mut self, v: ::std::string::String) {
        self.last_command_ident = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_last_command_ident(&mut self) -> &mut ::std::string::String {
        if self.last_command_ident.is_none() {
            self.last_command_ident.set_default();
        }
        self.last_command_ident.as_mut().unwrap()
    }

    // Take field
    pub fn take_last_command_ident(&mut self) -> ::std::string::String {
        self.last_command_ident.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_last_command_ident(&self) -> &str {
        match self.last_command_ident.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_last_command_ident_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.last_command_ident
    }

    fn mut_last_command_ident_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.last_command_ident
    }

    // optional uint32 last_command_msgid = 21;

    pub fn clear_last_command_msgid(&mut self) {
        self.last_command_msgid = ::std::option::Option::None;
    }

    pub fn has_last_command_msgid(&self) -> bool {
        self.last_command_msgid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_command_msgid(&mut self, v: u32) {
        self.last_command_msgid = ::std::option::Option::Some(v);
    }

    pub fn get_last_command_msgid(&self) -> u32 {
        self.last_command_msgid.unwrap_or(0)
    }

    fn get_last_command_msgid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.last_command_msgid
    }

    fn mut_last_command_msgid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.last_command_msgid
    }

    // optional bool playing_from_fallback = 24;

    pub fn clear_playing_from_fallback(&mut self) {
        self.playing_from_fallback = ::std::option::Option::None;
    }

    pub fn has_playing_from_fallback(&self) -> bool {
        self.playing_from_fallback.is_some()
    }

    // Param is passed by value, moved
    pub fn set_playing_from_fallback(&mut self, v: bool) {
        self.playing_from_fallback = ::std::option::Option::Some(v);
    }

    pub fn get_playing_from_fallback(&self) -> bool {
        self.playing_from_fallback.unwrap_or(false)
    }

    fn get_playing_from_fallback_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.playing_from_fallback
    }

    fn mut_playing_from_fallback_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.playing_from_fallback
    }

    // optional uint32 row = 25;

    pub fn clear_row(&mut self) {
        self.row = ::std::option::Option::None;
    }

    pub fn has_row(&self) -> bool {
        self.row.is_some()
    }

    // Param is passed by value, moved
    pub fn set_row(&mut self, v: u32) {
        self.row = ::std::option::Option::Some(v);
    }

    pub fn get_row(&self) -> u32 {
        self.row.unwrap_or(0)
    }

    fn get_row_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.row
    }

    fn mut_row_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.row
    }

    // optional uint32 playing_track_index = 26;

    pub fn clear_playing_track_index(&mut self) {
        self.playing_track_index = ::std::option::Option::None;
    }

    pub fn has_playing_track_index(&self) -> bool {
        self.playing_track_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_playing_track_index(&mut self, v: u32) {
        self.playing_track_index = ::std::option::Option::Some(v);
    }

    pub fn get_playing_track_index(&self) -> u32 {
        self.playing_track_index.unwrap_or(0)
    }

    fn get_playing_track_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.playing_track_index
    }

    fn mut_playing_track_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.playing_track_index
    }

    // repeated .TrackRef track = 27;

    pub fn clear_track(&mut self) {
        self.track.clear();
    }

    // Param is passed by value, moved
    pub fn set_track(&mut self, v: ::protobuf::RepeatedField<TrackRef>) {
        self.track = v;
    }

    // Mutable pointer to the field.
    pub fn mut_track(&mut self) -> &mut ::protobuf::RepeatedField<TrackRef> {
        &mut self.track
    }

    // Take field
    pub fn take_track(&mut self) -> ::protobuf::RepeatedField<TrackRef> {
        ::std::mem::replace(&mut self.track, ::protobuf::RepeatedField::new())
    }

    pub fn get_track(&self) -> &[TrackRef] {
        &self.track
    }

    fn get_track_for_reflect(&self) -> &::protobuf::RepeatedField<TrackRef> {
        &self.track
    }

    fn mut_track_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TrackRef> {
        &mut self.track
    }

    // optional .Ad ad = 28;

    pub fn clear_ad(&mut self) {
        self.ad.clear();
    }

    pub fn has_ad(&self) -> bool {
        self.ad.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ad(&mut self, v: Ad) {
        self.ad = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ad(&mut self) -> &mut Ad {
        if self.ad.is_none() {
            self.ad.set_default();
        }
        self.ad.as_mut().unwrap()
    }

    // Take field
    pub fn take_ad(&mut self) -> Ad {
        self.ad.take().unwrap_or_else(|| Ad::new())
    }

    pub fn get_ad(&self) -> &Ad {
        self.ad.as_ref().unwrap_or_else(|| Ad::default_instance())
    }

    fn get_ad_for_reflect(&self) -> &::protobuf::SingularPtrField<Ad> {
        &self.ad
    }

    fn mut_ad_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Ad> {
        &mut self.ad
    }
}

impl ::protobuf::Message for State {
    fn is_initialized(&self) -> bool {
        for v in &self.track {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.ad {
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
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.context_uri)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.index = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.position_ms = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.status = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.position_measured_at = ::std::option::Option::Some(tmp);
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.context_description)?;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.shuffle = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.repeat = ::std::option::Option::Some(tmp);
                },
                20 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.last_command_ident)?;
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.last_command_msgid = ::std::option::Option::Some(tmp);
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.playing_from_fallback = ::std::option::Option::Some(tmp);
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.row = ::std::option::Option::Some(tmp);
                },
                26 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.playing_track_index = ::std::option::Option::Some(tmp);
                },
                27 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.track)?;
                },
                28 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ad)?;
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
        if let Some(ref v) = self.context_uri.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.position_ms {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::enum_size(5, v);
        }
        if let Some(v) = self.position_measured_at {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.context_description.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        }
        if let Some(v) = self.shuffle {
            my_size += 2;
        }
        if let Some(v) = self.repeat {
            my_size += 2;
        }
        if let Some(ref v) = self.last_command_ident.as_ref() {
            my_size += ::protobuf::rt::string_size(20, &v);
        }
        if let Some(v) = self.last_command_msgid {
            my_size += ::protobuf::rt::value_size(21, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.playing_from_fallback {
            my_size += 3;
        }
        if let Some(v) = self.row {
            my_size += ::protobuf::rt::value_size(25, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.playing_track_index {
            my_size += ::protobuf::rt::value_size(26, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.track {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.ad.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.context_uri.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.index {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.position_ms {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.status {
            os.write_enum(5, v.value())?;
        }
        if let Some(v) = self.position_measured_at {
            os.write_uint64(7, v)?;
        }
        if let Some(ref v) = self.context_description.as_ref() {
            os.write_string(8, &v)?;
        }
        if let Some(v) = self.shuffle {
            os.write_bool(13, v)?;
        }
        if let Some(v) = self.repeat {
            os.write_bool(14, v)?;
        }
        if let Some(ref v) = self.last_command_ident.as_ref() {
            os.write_string(20, &v)?;
        }
        if let Some(v) = self.last_command_msgid {
            os.write_uint32(21, v)?;
        }
        if let Some(v) = self.playing_from_fallback {
            os.write_bool(24, v)?;
        }
        if let Some(v) = self.row {
            os.write_uint32(25, v)?;
        }
        if let Some(v) = self.playing_track_index {
            os.write_uint32(26, v)?;
        }
        for v in &self.track {
            os.write_tag(27, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.ad.as_ref() {
            os.write_tag(28, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for State {
    fn new() -> State {
        State::new()
    }

    fn descriptor_static(_: ::std::option::Option<State>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "context_uri",
                    State::get_context_uri_for_reflect,
                    State::mut_context_uri_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "index",
                    State::get_index_for_reflect,
                    State::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "position_ms",
                    State::get_position_ms_for_reflect,
                    State::mut_position_ms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<PlayStatus>>(
                    "status",
                    State::get_status_for_reflect,
                    State::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "position_measured_at",
                    State::get_position_measured_at_for_reflect,
                    State::mut_position_measured_at_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "context_description",
                    State::get_context_description_for_reflect,
                    State::mut_context_description_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "shuffle",
                    State::get_shuffle_for_reflect,
                    State::mut_shuffle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "repeat",
                    State::get_repeat_for_reflect,
                    State::mut_repeat_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "last_command_ident",
                    State::get_last_command_ident_for_reflect,
                    State::mut_last_command_ident_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "last_command_msgid",
                    State::get_last_command_msgid_for_reflect,
                    State::mut_last_command_msgid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "playing_from_fallback",
                    State::get_playing_from_fallback_for_reflect,
                    State::mut_playing_from_fallback_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "row",
                    State::get_row_for_reflect,
                    State::mut_row_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "playing_track_index",
                    State::get_playing_track_index_for_reflect,
                    State::mut_playing_track_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TrackRef>>(
                    "track",
                    State::get_track_for_reflect,
                    State::mut_track_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Ad>>(
                    "ad",
                    State::get_ad_for_reflect,
                    State::mut_ad_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<State>(
                    "State",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for State {
    fn clear(&mut self) {
        self.clear_context_uri();
        self.clear_index();
        self.clear_position_ms();
        self.clear_status();
        self.clear_position_measured_at();
        self.clear_context_description();
        self.clear_shuffle();
        self.clear_repeat();
        self.clear_last_command_ident();
        self.clear_last_command_msgid();
        self.clear_playing_from_fallback();
        self.clear_row();
        self.clear_playing_track_index();
        self.clear_track();
        self.clear_ad();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for State {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for State {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TrackRef {
    // message fields
    gid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    uri: ::protobuf::SingularField<::std::string::String>,
    queued: ::std::option::Option<bool>,
    context: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TrackRef {}

impl TrackRef {
    pub fn new() -> TrackRef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TrackRef {
        static mut instance: ::protobuf::lazy::Lazy<TrackRef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TrackRef,
        };
        unsafe {
            instance.get(TrackRef::new)
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

    // optional string uri = 2;

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

    // optional bool queued = 3;

    pub fn clear_queued(&mut self) {
        self.queued = ::std::option::Option::None;
    }

    pub fn has_queued(&self) -> bool {
        self.queued.is_some()
    }

    // Param is passed by value, moved
    pub fn set_queued(&mut self, v: bool) {
        self.queued = ::std::option::Option::Some(v);
    }

    pub fn get_queued(&self) -> bool {
        self.queued.unwrap_or(false)
    }

    fn get_queued_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.queued
    }

    fn mut_queued_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.queued
    }

    // optional string context = 4;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: ::std::string::String) {
        self.context = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut ::std::string::String {
        if self.context.is_none() {
            self.context.set_default();
        }
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> ::std::string::String {
        self.context.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_context(&self) -> &str {
        match self.context.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.context
    }
}

impl ::protobuf::Message for TrackRef {
    fn is_initialized(&self) -> bool {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.uri)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.queued = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.context)?;
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
        if let Some(ref v) = self.uri.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.queued {
            my_size += 2;
        }
        if let Some(ref v) = self.context.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.gid.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(ref v) = self.uri.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.queued {
            os.write_bool(3, v)?;
        }
        if let Some(ref v) = self.context.as_ref() {
            os.write_string(4, &v)?;
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

impl ::protobuf::MessageStatic for TrackRef {
    fn new() -> TrackRef {
        TrackRef::new()
    }

    fn descriptor_static(_: ::std::option::Option<TrackRef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "gid",
                    TrackRef::get_gid_for_reflect,
                    TrackRef::mut_gid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "uri",
                    TrackRef::get_uri_for_reflect,
                    TrackRef::mut_uri_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "queued",
                    TrackRef::get_queued_for_reflect,
                    TrackRef::mut_queued_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "context",
                    TrackRef::get_context_for_reflect,
                    TrackRef::mut_context_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TrackRef>(
                    "TrackRef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TrackRef {
    fn clear(&mut self) {
        self.clear_gid();
        self.clear_uri();
        self.clear_queued();
        self.clear_context();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TrackRef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TrackRef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Ad {
    // message fields
    next: ::std::option::Option<i32>,
    ogg_fid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    image_fid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    duration: ::std::option::Option<i32>,
    click_url: ::protobuf::SingularField<::std::string::String>,
    impression_url: ::protobuf::SingularField<::std::string::String>,
    product: ::protobuf::SingularField<::std::string::String>,
    advertiser: ::protobuf::SingularField<::std::string::String>,
    gid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Ad {}

impl Ad {
    pub fn new() -> Ad {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Ad {
        static mut instance: ::protobuf::lazy::Lazy<Ad> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Ad,
        };
        unsafe {
            instance.get(Ad::new)
        }
    }

    // optional int32 next = 1;

    pub fn clear_next(&mut self) {
        self.next = ::std::option::Option::None;
    }

    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }

    // Param is passed by value, moved
    pub fn set_next(&mut self, v: i32) {
        self.next = ::std::option::Option::Some(v);
    }

    pub fn get_next(&self) -> i32 {
        self.next.unwrap_or(0)
    }

    fn get_next_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.next
    }

    fn mut_next_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.next
    }

    // optional bytes ogg_fid = 2;

    pub fn clear_ogg_fid(&mut self) {
        self.ogg_fid.clear();
    }

    pub fn has_ogg_fid(&self) -> bool {
        self.ogg_fid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ogg_fid(&mut self, v: ::std::vec::Vec<u8>) {
        self.ogg_fid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ogg_fid(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.ogg_fid.is_none() {
            self.ogg_fid.set_default();
        }
        self.ogg_fid.as_mut().unwrap()
    }

    // Take field
    pub fn take_ogg_fid(&mut self) -> ::std::vec::Vec<u8> {
        self.ogg_fid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_ogg_fid(&self) -> &[u8] {
        match self.ogg_fid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_ogg_fid_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.ogg_fid
    }

    fn mut_ogg_fid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.ogg_fid
    }

    // optional bytes image_fid = 3;

    pub fn clear_image_fid(&mut self) {
        self.image_fid.clear();
    }

    pub fn has_image_fid(&self) -> bool {
        self.image_fid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_image_fid(&mut self, v: ::std::vec::Vec<u8>) {
        self.image_fid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_image_fid(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.image_fid.is_none() {
            self.image_fid.set_default();
        }
        self.image_fid.as_mut().unwrap()
    }

    // Take field
    pub fn take_image_fid(&mut self) -> ::std::vec::Vec<u8> {
        self.image_fid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_image_fid(&self) -> &[u8] {
        match self.image_fid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_image_fid_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.image_fid
    }

    fn mut_image_fid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.image_fid
    }

    // optional int32 duration = 4;

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

    // optional string click_url = 5;

    pub fn clear_click_url(&mut self) {
        self.click_url.clear();
    }

    pub fn has_click_url(&self) -> bool {
        self.click_url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_click_url(&mut self, v: ::std::string::String) {
        self.click_url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_click_url(&mut self) -> &mut ::std::string::String {
        if self.click_url.is_none() {
            self.click_url.set_default();
        }
        self.click_url.as_mut().unwrap()
    }

    // Take field
    pub fn take_click_url(&mut self) -> ::std::string::String {
        self.click_url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_click_url(&self) -> &str {
        match self.click_url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_click_url_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.click_url
    }

    fn mut_click_url_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.click_url
    }

    // optional string impression_url = 6;

    pub fn clear_impression_url(&mut self) {
        self.impression_url.clear();
    }

    pub fn has_impression_url(&self) -> bool {
        self.impression_url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_impression_url(&mut self, v: ::std::string::String) {
        self.impression_url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_impression_url(&mut self) -> &mut ::std::string::String {
        if self.impression_url.is_none() {
            self.impression_url.set_default();
        }
        self.impression_url.as_mut().unwrap()
    }

    // Take field
    pub fn take_impression_url(&mut self) -> ::std::string::String {
        self.impression_url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_impression_url(&self) -> &str {
        match self.impression_url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_impression_url_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.impression_url
    }

    fn mut_impression_url_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.impression_url
    }

    // optional string product = 7;

    pub fn clear_product(&mut self) {
        self.product.clear();
    }

    pub fn has_product(&self) -> bool {
        self.product.is_some()
    }

    // Param is passed by value, moved
    pub fn set_product(&mut self, v: ::std::string::String) {
        self.product = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_product(&mut self) -> &mut ::std::string::String {
        if self.product.is_none() {
            self.product.set_default();
        }
        self.product.as_mut().unwrap()
    }

    // Take field
    pub fn take_product(&mut self) -> ::std::string::String {
        self.product.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_product(&self) -> &str {
        match self.product.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_product_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.product
    }

    fn mut_product_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.product
    }

    // optional string advertiser = 8;

    pub fn clear_advertiser(&mut self) {
        self.advertiser.clear();
    }

    pub fn has_advertiser(&self) -> bool {
        self.advertiser.is_some()
    }

    // Param is passed by value, moved
    pub fn set_advertiser(&mut self, v: ::std::string::String) {
        self.advertiser = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_advertiser(&mut self) -> &mut ::std::string::String {
        if self.advertiser.is_none() {
            self.advertiser.set_default();
        }
        self.advertiser.as_mut().unwrap()
    }

    // Take field
    pub fn take_advertiser(&mut self) -> ::std::string::String {
        self.advertiser.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_advertiser(&self) -> &str {
        match self.advertiser.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_advertiser_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.advertiser
    }

    fn mut_advertiser_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.advertiser
    }

    // optional bytes gid = 9;

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
}

impl ::protobuf::Message for Ad {
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
                    self.next = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.ogg_fid)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.image_fid)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.duration = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.click_url)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.impression_url)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.product)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.advertiser)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.gid)?;
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
        if let Some(v) = self.next {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.ogg_fid.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(ref v) = self.image_fid.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        if let Some(v) = self.duration {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.click_url.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(ref v) = self.impression_url.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        if let Some(ref v) = self.product.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        }
        if let Some(ref v) = self.advertiser.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        }
        if let Some(ref v) = self.gid.as_ref() {
            my_size += ::protobuf::rt::bytes_size(9, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.next {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.ogg_fid.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(ref v) = self.image_fid.as_ref() {
            os.write_bytes(3, &v)?;
        }
        if let Some(v) = self.duration {
            os.write_int32(4, v)?;
        }
        if let Some(ref v) = self.click_url.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(ref v) = self.impression_url.as_ref() {
            os.write_string(6, &v)?;
        }
        if let Some(ref v) = self.product.as_ref() {
            os.write_string(7, &v)?;
        }
        if let Some(ref v) = self.advertiser.as_ref() {
            os.write_string(8, &v)?;
        }
        if let Some(ref v) = self.gid.as_ref() {
            os.write_bytes(9, &v)?;
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

impl ::protobuf::MessageStatic for Ad {
    fn new() -> Ad {
        Ad::new()
    }

    fn descriptor_static(_: ::std::option::Option<Ad>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "next",
                    Ad::get_next_for_reflect,
                    Ad::mut_next_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "ogg_fid",
                    Ad::get_ogg_fid_for_reflect,
                    Ad::mut_ogg_fid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "image_fid",
                    Ad::get_image_fid_for_reflect,
                    Ad::mut_image_fid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "duration",
                    Ad::get_duration_for_reflect,
                    Ad::mut_duration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "click_url",
                    Ad::get_click_url_for_reflect,
                    Ad::mut_click_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "impression_url",
                    Ad::get_impression_url_for_reflect,
                    Ad::mut_impression_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "product",
                    Ad::get_product_for_reflect,
                    Ad::mut_product_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "advertiser",
                    Ad::get_advertiser_for_reflect,
                    Ad::mut_advertiser_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "gid",
                    Ad::get_gid_for_reflect,
                    Ad::mut_gid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Ad>(
                    "Ad",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Ad {
    fn clear(&mut self) {
        self.clear_next();
        self.clear_ogg_fid();
        self.clear_image_fid();
        self.clear_duration();
        self.clear_click_url();
        self.clear_impression_url();
        self.clear_product();
        self.clear_advertiser();
        self.clear_gid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Ad {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Ad {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Metadata {
    // message fields
    field_type: ::protobuf::SingularField<::std::string::String>,
    metadata: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Metadata {}

impl Metadata {
    pub fn new() -> Metadata {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Metadata {
        static mut instance: ::protobuf::lazy::Lazy<Metadata> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Metadata,
        };
        unsafe {
            instance.get(Metadata::new)
        }
    }

    // optional string type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        if self.field_type.is_none() {
            self.field_type.set_default();
        }
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        self.field_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_field_type(&self) -> &str {
        match self.field_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_field_type_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.field_type
    }

    // optional string metadata = 2;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: ::std::string::String) {
        self.metadata = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut ::std::string::String {
        if self.metadata.is_none() {
            self.metadata.set_default();
        }
        self.metadata.as_mut().unwrap()
    }

    // Take field
    pub fn take_metadata(&mut self) -> ::std::string::String {
        self.metadata.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_metadata(&self) -> &str {
        match self.metadata.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_metadata_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.metadata
    }
}

impl ::protobuf::Message for Metadata {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.field_type)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.metadata)?;
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
        if let Some(ref v) = self.field_type.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.metadata.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.field_type.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.metadata.as_ref() {
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

impl ::protobuf::MessageStatic for Metadata {
    fn new() -> Metadata {
        Metadata::new()
    }

    fn descriptor_static(_: ::std::option::Option<Metadata>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    Metadata::get_field_type_for_reflect,
                    Metadata::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "metadata",
                    Metadata::get_metadata_for_reflect,
                    Metadata::mut_metadata_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Metadata>(
                    "Metadata",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Metadata {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_metadata();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Metadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Metadata {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MessageType {
    kMessageTypeHello = 1,
    kMessageTypeGoodbye = 2,
    kMessageTypeProbe = 3,
    kMessageTypeNotify = 10,
    kMessageTypeLoad = 20,
    kMessageTypePlay = 21,
    kMessageTypePause = 22,
    kMessageTypePlayPause = 23,
    kMessageTypeSeek = 24,
    kMessageTypePrev = 25,
    kMessageTypeNext = 26,
    kMessageTypeVolume = 27,
    kMessageTypeShuffle = 28,
    kMessageTypeRepeat = 29,
    kMessageTypeVolumeDown = 31,
    kMessageTypeVolumeUp = 32,
    kMessageTypeReplace = 33,
    kMessageTypeLogout = 34,
    kMessageTypeAction = 35,
    kMessageTypeRename = 36,
    kMessageTypeUpdateMetadata = 128,
}

impl ::protobuf::ProtobufEnum for MessageType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MessageType> {
        match value {
            1 => ::std::option::Option::Some(MessageType::kMessageTypeHello),
            2 => ::std::option::Option::Some(MessageType::kMessageTypeGoodbye),
            3 => ::std::option::Option::Some(MessageType::kMessageTypeProbe),
            10 => ::std::option::Option::Some(MessageType::kMessageTypeNotify),
            20 => ::std::option::Option::Some(MessageType::kMessageTypeLoad),
            21 => ::std::option::Option::Some(MessageType::kMessageTypePlay),
            22 => ::std::option::Option::Some(MessageType::kMessageTypePause),
            23 => ::std::option::Option::Some(MessageType::kMessageTypePlayPause),
            24 => ::std::option::Option::Some(MessageType::kMessageTypeSeek),
            25 => ::std::option::Option::Some(MessageType::kMessageTypePrev),
            26 => ::std::option::Option::Some(MessageType::kMessageTypeNext),
            27 => ::std::option::Option::Some(MessageType::kMessageTypeVolume),
            28 => ::std::option::Option::Some(MessageType::kMessageTypeShuffle),
            29 => ::std::option::Option::Some(MessageType::kMessageTypeRepeat),
            31 => ::std::option::Option::Some(MessageType::kMessageTypeVolumeDown),
            32 => ::std::option::Option::Some(MessageType::kMessageTypeVolumeUp),
            33 => ::std::option::Option::Some(MessageType::kMessageTypeReplace),
            34 => ::std::option::Option::Some(MessageType::kMessageTypeLogout),
            35 => ::std::option::Option::Some(MessageType::kMessageTypeAction),
            36 => ::std::option::Option::Some(MessageType::kMessageTypeRename),
            128 => ::std::option::Option::Some(MessageType::kMessageTypeUpdateMetadata),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MessageType] = &[
            MessageType::kMessageTypeHello,
            MessageType::kMessageTypeGoodbye,
            MessageType::kMessageTypeProbe,
            MessageType::kMessageTypeNotify,
            MessageType::kMessageTypeLoad,
            MessageType::kMessageTypePlay,
            MessageType::kMessageTypePause,
            MessageType::kMessageTypePlayPause,
            MessageType::kMessageTypeSeek,
            MessageType::kMessageTypePrev,
            MessageType::kMessageTypeNext,
            MessageType::kMessageTypeVolume,
            MessageType::kMessageTypeShuffle,
            MessageType::kMessageTypeRepeat,
            MessageType::kMessageTypeVolumeDown,
            MessageType::kMessageTypeVolumeUp,
            MessageType::kMessageTypeReplace,
            MessageType::kMessageTypeLogout,
            MessageType::kMessageTypeAction,
            MessageType::kMessageTypeRename,
            MessageType::kMessageTypeUpdateMetadata,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<MessageType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MessageType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MessageType {
}

impl ::protobuf::reflect::ProtobufValue for MessageType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CapabilityType {
    kSupportedContexts = 1,
    kCanBePlayer = 2,
    kRestrictToLocal = 3,
    kDeviceType = 4,
    kGaiaEqConnectId = 5,
    kSupportsLogout = 6,
    kIsObservable = 7,
    kVolumeSteps = 8,
    kSupportedTypes = 9,
    kCommandAcks = 10,
    kSupportsRename = 11,
    kHidden = 12,
}

impl ::protobuf::ProtobufEnum for CapabilityType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CapabilityType> {
        match value {
            1 => ::std::option::Option::Some(CapabilityType::kSupportedContexts),
            2 => ::std::option::Option::Some(CapabilityType::kCanBePlayer),
            3 => ::std::option::Option::Some(CapabilityType::kRestrictToLocal),
            4 => ::std::option::Option::Some(CapabilityType::kDeviceType),
            5 => ::std::option::Option::Some(CapabilityType::kGaiaEqConnectId),
            6 => ::std::option::Option::Some(CapabilityType::kSupportsLogout),
            7 => ::std::option::Option::Some(CapabilityType::kIsObservable),
            8 => ::std::option::Option::Some(CapabilityType::kVolumeSteps),
            9 => ::std::option::Option::Some(CapabilityType::kSupportedTypes),
            10 => ::std::option::Option::Some(CapabilityType::kCommandAcks),
            11 => ::std::option::Option::Some(CapabilityType::kSupportsRename),
            12 => ::std::option::Option::Some(CapabilityType::kHidden),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CapabilityType] = &[
            CapabilityType::kSupportedContexts,
            CapabilityType::kCanBePlayer,
            CapabilityType::kRestrictToLocal,
            CapabilityType::kDeviceType,
            CapabilityType::kGaiaEqConnectId,
            CapabilityType::kSupportsLogout,
            CapabilityType::kIsObservable,
            CapabilityType::kVolumeSteps,
            CapabilityType::kSupportedTypes,
            CapabilityType::kCommandAcks,
            CapabilityType::kSupportsRename,
            CapabilityType::kHidden,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CapabilityType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CapabilityType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CapabilityType {
}

impl ::protobuf::reflect::ProtobufValue for CapabilityType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum PlayStatus {
    kPlayStatusStop = 0,
    kPlayStatusPlay = 1,
    kPlayStatusPause = 2,
    kPlayStatusLoading = 3,
}

impl ::protobuf::ProtobufEnum for PlayStatus {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<PlayStatus> {
        match value {
            0 => ::std::option::Option::Some(PlayStatus::kPlayStatusStop),
            1 => ::std::option::Option::Some(PlayStatus::kPlayStatusPlay),
            2 => ::std::option::Option::Some(PlayStatus::kPlayStatusPause),
            3 => ::std::option::Option::Some(PlayStatus::kPlayStatusLoading),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [PlayStatus] = &[
            PlayStatus::kPlayStatusStop,
            PlayStatus::kPlayStatusPlay,
            PlayStatus::kPlayStatusPause,
            PlayStatus::kPlayStatusLoading,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<PlayStatus>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("PlayStatus", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for PlayStatus {
}

impl ::protobuf::reflect::ProtobufValue for PlayStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bspirc.proto\"\xfa\x03\n\x05Frame\x12\x18\n\x07version\x18\x01\x20\
    \x01(\rR\x07version\x12\x14\n\x05ident\x18\x02\x20\x01(\tR\x05ident\x12)\
    \n\x10protocol_version\x18\x03\x20\x01(\tR\x0fprotocolVersion\x12\x15\n\
    \x06seq_nr\x18\x04\x20\x01(\rR\x05seqNr\x12\x1e\n\x03typ\x18\x05\x20\x01\
    (\x0e2\x0c.MessageTypeR\x03typ\x12/\n\x0cdevice_state\x18\x07\x20\x01(\
    \x0b2\x0c.DeviceStateR\x0bdeviceState\x12\"\n\x07goodbye\x18\x0b\x20\x01\
    (\x0b2\x08.GoodbyeR\x07goodbye\x12\x1c\n\x05state\x18\x0c\x20\x01(\x0b2\
    \x06.StateR\x05state\x12\x1a\n\x08position\x18\r\x20\x01(\rR\x08position\
    \x12\x16\n\x06volume\x18\x0e\x20\x01(\rR\x06volume\x12&\n\x0fstate_updat\
    e_id\x18\x11\x20\x01(\x03R\rstateUpdateId\x12\x1c\n\trecipient\x18\x12\
    \x20\x03(\tR\trecipient\x120\n\x14context_player_state\x18\x13\x20\x01(\
    \x0cR\x12contextPlayerState\x12\x19\n\x08new_name\x18\x14\x20\x01(\tR\
    \x07newName\x12%\n\x08metadata\x18\x19\x20\x01(\x0b2\t.MetadataR\x08meta\
    data\"\x88\x03\n\x0bDeviceState\x12\x1d\n\nsw_version\x18\x01\x20\x01(\t\
    R\tswVersion\x12\x1b\n\tis_active\x18\n\x20\x01(\x08R\x08isActive\x12\
    \x19\n\x08can_play\x18\x0b\x20\x01(\x08R\x07canPlay\x12\x16\n\x06volume\
    \x18\x0c\x20\x01(\rR\x06volume\x12\x12\n\x04name\x18\r\x20\x01(\tR\x04na\
    me\x12\x1d\n\nerror_code\x18\x0e\x20\x01(\rR\terrorCode\x12(\n\x10became\
    _active_at\x18\x0f\x20\x01(\x03R\x0ebecameActiveAt\x12#\n\rerror_message\
    \x18\x10\x20\x01(\tR\x0cerrorMessage\x12/\n\x0ccapabilities\x18\x11\x20\
    \x03(\x0b2\x0b.CapabilityR\x0ccapabilities\x120\n\x14context_player_erro\
    r\x18\x14\x20\x01(\tR\x12contextPlayerError\x12%\n\x08metadata\x18\x19\
    \x20\x03(\x0b2\t.MetadataR\x08metadata\"m\n\nCapability\x12!\n\x03typ\
    \x18\x01\x20\x01(\x0e2\x0f.CapabilityTypeR\x03typ\x12\x1a\n\x08intValue\
    \x18\x02\x20\x03(\x03R\x08intValue\x12\x20\n\x0bstringValue\x18\x03\x20\
    \x03(\tR\x0bstringValue\"!\n\x07Goodbye\x12\x16\n\x06reason\x18\x01\x20\
    \x01(\tR\x06reason\"\xa1\x04\n\x05State\x12\x1f\n\x0bcontext_uri\x18\x02\
    \x20\x01(\tR\ncontextUri\x12\x14\n\x05index\x18\x03\x20\x01(\rR\x05index\
    \x12\x1f\n\x0bposition_ms\x18\x04\x20\x01(\rR\npositionMs\x12#\n\x06stat\
    us\x18\x05\x20\x01(\x0e2\x0b.PlayStatusR\x06status\x120\n\x14position_me\
    asured_at\x18\x07\x20\x01(\x04R\x12positionMeasuredAt\x12/\n\x13context_\
    description\x18\x08\x20\x01(\tR\x12contextDescription\x12\x18\n\x07shuff\
    le\x18\r\x20\x01(\x08R\x07shuffle\x12\x16\n\x06repeat\x18\x0e\x20\x01(\
    \x08R\x06repeat\x12,\n\x12last_command_ident\x18\x14\x20\x01(\tR\x10last\
    CommandIdent\x12,\n\x12last_command_msgid\x18\x15\x20\x01(\rR\x10lastCom\
    mandMsgid\x122\n\x15playing_from_fallback\x18\x18\x20\x01(\x08R\x13playi\
    ngFromFallback\x12\x10\n\x03row\x18\x19\x20\x01(\rR\x03row\x12.\n\x13pla\
    ying_track_index\x18\x1a\x20\x01(\rR\x11playingTrackIndex\x12\x1f\n\x05t\
    rack\x18\x1b\x20\x03(\x0b2\t.TrackRefR\x05track\x12\x13\n\x02ad\x18\x1c\
    \x20\x01(\x0b2\x03.AdR\x02ad\"`\n\x08TrackRef\x12\x10\n\x03gid\x18\x01\
    \x20\x01(\x0cR\x03gid\x12\x10\n\x03uri\x18\x02\x20\x01(\tR\x03uri\x12\
    \x16\n\x06queued\x18\x03\x20\x01(\x08R\x06queued\x12\x18\n\x07context\
    \x18\x04\x20\x01(\tR\x07context\"\xfa\x01\n\x02Ad\x12\x12\n\x04next\x18\
    \x01\x20\x01(\x05R\x04next\x12\x17\n\x07ogg_fid\x18\x02\x20\x01(\x0cR\
    \x06oggFid\x12\x1b\n\timage_fid\x18\x03\x20\x01(\x0cR\x08imageFid\x12\
    \x1a\n\x08duration\x18\x04\x20\x01(\x05R\x08duration\x12\x1b\n\tclick_ur\
    l\x18\x05\x20\x01(\tR\x08clickUrl\x12%\n\x0eimpression_url\x18\x06\x20\
    \x01(\tR\rimpressionUrl\x12\x18\n\x07product\x18\x07\x20\x01(\tR\x07prod\
    uct\x12\x1e\n\nadvertiser\x18\x08\x20\x01(\tR\nadvertiser\x12\x10\n\x03g\
    id\x18\t\x20\x01(\x0cR\x03gid\":\n\x08Metadata\x12\x12\n\x04type\x18\x01\
    \x20\x01(\tR\x04type\x12\x1a\n\x08metadata\x18\x02\x20\x01(\tR\x08metada\
    ta*\x8d\x04\n\x0bMessageType\x12\x15\n\x11kMessageTypeHello\x10\x01\x12\
    \x17\n\x13kMessageTypeGoodbye\x10\x02\x12\x15\n\x11kMessageTypeProbe\x10\
    \x03\x12\x16\n\x12kMessageTypeNotify\x10\n\x12\x14\n\x10kMessageTypeLoad\
    \x10\x14\x12\x14\n\x10kMessageTypePlay\x10\x15\x12\x15\n\x11kMessageType\
    Pause\x10\x16\x12\x19\n\x15kMessageTypePlayPause\x10\x17\x12\x14\n\x10kM\
    essageTypeSeek\x10\x18\x12\x14\n\x10kMessageTypePrev\x10\x19\x12\x14\n\
    \x10kMessageTypeNext\x10\x1a\x12\x16\n\x12kMessageTypeVolume\x10\x1b\x12\
    \x17\n\x13kMessageTypeShuffle\x10\x1c\x12\x16\n\x12kMessageTypeRepeat\
    \x10\x1d\x12\x1a\n\x16kMessageTypeVolumeDown\x10\x1f\x12\x18\n\x14kMessa\
    geTypeVolumeUp\x10\x20\x12\x17\n\x13kMessageTypeReplace\x10!\x12\x16\n\
    \x12kMessageTypeLogout\x10\"\x12\x16\n\x12kMessageTypeAction\x10#\x12\
    \x16\n\x12kMessageTypeRename\x10$\x12\x1f\n\x1akMessageTypeUpdateMetadat\
    a\x10\x80\x01*\xfa\x01\n\x0eCapabilityType\x12\x16\n\x12kSupportedContex\
    ts\x10\x01\x12\x10\n\x0ckCanBePlayer\x10\x02\x12\x14\n\x10kRestrictToLoc\
    al\x10\x03\x12\x0f\n\x0bkDeviceType\x10\x04\x12\x14\n\x10kGaiaEqConnectI\
    d\x10\x05\x12\x13\n\x0fkSupportsLogout\x10\x06\x12\x11\n\rkIsObservable\
    \x10\x07\x12\x10\n\x0ckVolumeSteps\x10\x08\x12\x13\n\x0fkSupportedTypes\
    \x10\t\x12\x10\n\x0ckCommandAcks\x10\n\x12\x13\n\x0fkSupportsRename\x10\
    \x0b\x12\x0b\n\x07kHidden\x10\x0c*d\n\nPlayStatus\x12\x13\n\x0fkPlayStat\
    usStop\x10\0\x12\x13\n\x0fkPlayStatusPlay\x10\x01\x12\x14\n\x10kPlayStat\
    usPause\x10\x02\x12\x16\n\x12kPlayStatusLoading\x10\x03J\xbf.\n\x07\x12\
    \x05\0\0\x82\x01\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\
    \x04\x02\0\x12\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\r\n\x0b\n\x04\x04\
    \0\x02\0\x12\x03\x03\x04\"\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x03\x04\
    \x0c\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x03\r\x13\n\x0c\n\x05\x04\0\x02\
    \0\x01\x12\x03\x03\x14\x1b\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03\x1e!\
    \n\x0b\n\x04\x04\0\x02\x01\x12\x03\x04\x04\x20\n\x0c\n\x05\x04\0\x02\x01\
    \x04\x12\x03\x04\x04\x0c\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x04\r\x13\
    \n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x04\x14\x19\n\x0c\n\x05\x04\0\x02\
    \x01\x03\x12\x03\x04\x1c\x1f\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x05\x04+\
    \n\x0c\n\x05\x04\0\x02\x02\x04\x12\x03\x05\x04\x0c\n\x0c\n\x05\x04\0\x02\
    \x02\x05\x12\x03\x05\r\x13\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x05\x14\
    $\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x05'*\n\x0b\n\x04\x04\0\x02\x03\
    \x12\x03\x06\x04!\n\x0c\n\x05\x04\0\x02\x03\x04\x12\x03\x06\x04\x0c\n\
    \x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x06\r\x13\n\x0c\n\x05\x04\0\x02\x03\
    \x01\x12\x03\x06\x14\x1a\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x06\x1d\
    \x20\n\x0b\n\x04\x04\0\x02\x04\x12\x03\x07\x04#\n\x0c\n\x05\x04\0\x02\
    \x04\x04\x12\x03\x07\x04\x0c\n\x0c\n\x05\x04\0\x02\x04\x06\x12\x03\x07\r\
    \x18\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\x07\x19\x1c\n\x0c\n\x05\x04\0\
    \x02\x04\x03\x12\x03\x07\x1f\"\n\x0b\n\x04\x04\0\x02\x05\x12\x03\x08\x04\
    ,\n\x0c\n\x05\x04\0\x02\x05\x04\x12\x03\x08\x04\x0c\n\x0c\n\x05\x04\0\
    \x02\x05\x06\x12\x03\x08\r\x18\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03\x08\
    \x19%\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03\x08(+\n\x0b\n\x04\x04\0\x02\
    \x06\x12\x03\t\x04#\n\x0c\n\x05\x04\0\x02\x06\x04\x12\x03\t\x04\x0c\n\
    \x0c\n\x05\x04\0\x02\x06\x06\x12\x03\t\r\x14\n\x0c\n\x05\x04\0\x02\x06\
    \x01\x12\x03\t\x15\x1c\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03\t\x1f\"\n\
    \x0b\n\x04\x04\0\x02\x07\x12\x03\n\x04\x1f\n\x0c\n\x05\x04\0\x02\x07\x04\
    \x12\x03\n\x04\x0c\n\x0c\n\x05\x04\0\x02\x07\x06\x12\x03\n\r\x12\n\x0c\n\
    \x05\x04\0\x02\x07\x01\x12\x03\n\x13\x18\n\x0c\n\x05\x04\0\x02\x07\x03\
    \x12\x03\n\x1b\x1e\n\x0b\n\x04\x04\0\x02\x08\x12\x03\x0b\x04#\n\x0c\n\
    \x05\x04\0\x02\x08\x04\x12\x03\x0b\x04\x0c\n\x0c\n\x05\x04\0\x02\x08\x05\
    \x12\x03\x0b\r\x13\n\x0c\n\x05\x04\0\x02\x08\x01\x12\x03\x0b\x14\x1c\n\
    \x0c\n\x05\x04\0\x02\x08\x03\x12\x03\x0b\x1f\"\n\x0b\n\x04\x04\0\x02\t\
    \x12\x03\x0c\x04!\n\x0c\n\x05\x04\0\x02\t\x04\x12\x03\x0c\x04\x0c\n\x0c\
    \n\x05\x04\0\x02\t\x05\x12\x03\x0c\r\x13\n\x0c\n\x05\x04\0\x02\t\x01\x12\
    \x03\x0c\x14\x1a\n\x0c\n\x05\x04\0\x02\t\x03\x12\x03\x0c\x1d\x20\n\x0b\n\
    \x04\x04\0\x02\n\x12\x03\r\x04*\n\x0c\n\x05\x04\0\x02\n\x04\x12\x03\r\
    \x04\x0c\n\x0c\n\x05\x04\0\x02\n\x05\x12\x03\r\r\x12\n\x0c\n\x05\x04\0\
    \x02\n\x01\x12\x03\r\x13\"\n\x0c\n\x05\x04\0\x02\n\x03\x12\x03\r%)\n\x0b\
    \n\x04\x04\0\x02\x0b\x12\x03\x0e\x04%\n\x0c\n\x05\x04\0\x02\x0b\x04\x12\
    \x03\x0e\x04\x0c\n\x0c\n\x05\x04\0\x02\x0b\x05\x12\x03\x0e\r\x13\n\x0c\n\
    \x05\x04\0\x02\x0b\x01\x12\x03\x0e\x14\x1d\n\x0c\n\x05\x04\0\x02\x0b\x03\
    \x12\x03\x0e\x20$\n\x0b\n\x04\x04\0\x02\x0c\x12\x03\x0f\x04/\n\x0c\n\x05\
    \x04\0\x02\x0c\x04\x12\x03\x0f\x04\x0c\n\x0c\n\x05\x04\0\x02\x0c\x05\x12\
    \x03\x0f\r\x12\n\x0c\n\x05\x04\0\x02\x0c\x01\x12\x03\x0f\x13'\n\x0c\n\
    \x05\x04\0\x02\x0c\x03\x12\x03\x0f*.\n\x0b\n\x04\x04\0\x02\r\x12\x03\x10\
    \x04$\n\x0c\n\x05\x04\0\x02\r\x04\x12\x03\x10\x04\x0c\n\x0c\n\x05\x04\0\
    \x02\r\x05\x12\x03\x10\r\x13\n\x0c\n\x05\x04\0\x02\r\x01\x12\x03\x10\x14\
    \x1c\n\x0c\n\x05\x04\0\x02\r\x03\x12\x03\x10\x1f#\n\x0b\n\x04\x04\0\x02\
    \x0e\x12\x03\x11\x04&\n\x0c\n\x05\x04\0\x02\x0e\x04\x12\x03\x11\x04\x0c\
    \n\x0c\n\x05\x04\0\x02\x0e\x06\x12\x03\x11\r\x15\n\x0c\n\x05\x04\0\x02\
    \x0e\x01\x12\x03\x11\x16\x1e\n\x0c\n\x05\x04\0\x02\x0e\x03\x12\x03\x11!%\
    \n\n\n\x02\x05\0\x12\x04\x14\0*\x01\n\n\n\x03\x05\0\x01\x12\x03\x14\x05\
    \x10\n\x0b\n\x04\x05\0\x02\0\x12\x03\x15\x04\x1c\n\x0c\n\x05\x05\0\x02\0\
    \x01\x12\x03\x15\x04\x15\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x15\x18\x1b\
    \n\x0b\n\x04\x05\0\x02\x01\x12\x03\x16\x04\x1e\n\x0c\n\x05\x05\0\x02\x01\
    \x01\x12\x03\x16\x04\x17\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x16\x1a\
    \x1d\n\x0b\n\x04\x05\0\x02\x02\x12\x03\x17\x04\x1c\n\x0c\n\x05\x05\0\x02\
    \x02\x01\x12\x03\x17\x04\x15\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\x17\
    \x18\x1b\n\x0b\n\x04\x05\0\x02\x03\x12\x03\x18\x04\x1d\n\x0c\n\x05\x05\0\
    \x02\x03\x01\x12\x03\x18\x04\x16\n\x0c\n\x05\x05\0\x02\x03\x02\x12\x03\
    \x18\x19\x1c\n\x0b\n\x04\x05\0\x02\x04\x12\x03\x19\x04\x1c\n\x0c\n\x05\
    \x05\0\x02\x04\x01\x12\x03\x19\x04\x14\n\x0c\n\x05\x05\0\x02\x04\x02\x12\
    \x03\x19\x17\x1b\n\x0b\n\x04\x05\0\x02\x05\x12\x03\x1a\x04\x1c\n\x0c\n\
    \x05\x05\0\x02\x05\x01\x12\x03\x1a\x04\x14\n\x0c\n\x05\x05\0\x02\x05\x02\
    \x12\x03\x1a\x17\x1b\n\x0b\n\x04\x05\0\x02\x06\x12\x03\x1b\x04\x1d\n\x0c\
    \n\x05\x05\0\x02\x06\x01\x12\x03\x1b\x04\x15\n\x0c\n\x05\x05\0\x02\x06\
    \x02\x12\x03\x1b\x18\x1c\n\x0b\n\x04\x05\0\x02\x07\x12\x03\x1c\x04!\n\
    \x0c\n\x05\x05\0\x02\x07\x01\x12\x03\x1c\x04\x19\n\x0c\n\x05\x05\0\x02\
    \x07\x02\x12\x03\x1c\x1c\x20\n\x0b\n\x04\x05\0\x02\x08\x12\x03\x1d\x04\
    \x1c\n\x0c\n\x05\x05\0\x02\x08\x01\x12\x03\x1d\x04\x14\n\x0c\n\x05\x05\0\
    \x02\x08\x02\x12\x03\x1d\x17\x1b\n\x0b\n\x04\x05\0\x02\t\x12\x03\x1e\x04\
    \x1c\n\x0c\n\x05\x05\0\x02\t\x01\x12\x03\x1e\x04\x14\n\x0c\n\x05\x05\0\
    \x02\t\x02\x12\x03\x1e\x17\x1b\n\x0b\n\x04\x05\0\x02\n\x12\x03\x1f\x04\
    \x1c\n\x0c\n\x05\x05\0\x02\n\x01\x12\x03\x1f\x04\x14\n\x0c\n\x05\x05\0\
    \x02\n\x02\x12\x03\x1f\x17\x1b\n\x0b\n\x04\x05\0\x02\x0b\x12\x03\x20\x04\
    \x1e\n\x0c\n\x05\x05\0\x02\x0b\x01\x12\x03\x20\x04\x16\n\x0c\n\x05\x05\0\
    \x02\x0b\x02\x12\x03\x20\x19\x1d\n\x0b\n\x04\x05\0\x02\x0c\x12\x03!\x04\
    \x1f\n\x0c\n\x05\x05\0\x02\x0c\x01\x12\x03!\x04\x17\n\x0c\n\x05\x05\0\
    \x02\x0c\x02\x12\x03!\x1a\x1e\n\x0b\n\x04\x05\0\x02\r\x12\x03\"\x04\x1e\
    \n\x0c\n\x05\x05\0\x02\r\x01\x12\x03\"\x04\x16\n\x0c\n\x05\x05\0\x02\r\
    \x02\x12\x03\"\x19\x1d\n\x0b\n\x04\x05\0\x02\x0e\x12\x03#\x04\"\n\x0c\n\
    \x05\x05\0\x02\x0e\x01\x12\x03#\x04\x1a\n\x0c\n\x05\x05\0\x02\x0e\x02\
    \x12\x03#\x1d!\n\x0b\n\x04\x05\0\x02\x0f\x12\x03$\x04\x20\n\x0c\n\x05\
    \x05\0\x02\x0f\x01\x12\x03$\x04\x18\n\x0c\n\x05\x05\0\x02\x0f\x02\x12\
    \x03$\x1b\x1f\n\x0b\n\x04\x05\0\x02\x10\x12\x03%\x04\x1f\n\x0c\n\x05\x05\
    \0\x02\x10\x01\x12\x03%\x04\x17\n\x0c\n\x05\x05\0\x02\x10\x02\x12\x03%\
    \x1a\x1e\n\x0b\n\x04\x05\0\x02\x11\x12\x03&\x04\x1e\n\x0c\n\x05\x05\0\
    \x02\x11\x01\x12\x03&\x04\x16\n\x0c\n\x05\x05\0\x02\x11\x02\x12\x03&\x19\
    \x1d\n\x0b\n\x04\x05\0\x02\x12\x12\x03'\x04\x1e\n\x0c\n\x05\x05\0\x02\
    \x12\x01\x12\x03'\x04\x16\n\x0c\n\x05\x05\0\x02\x12\x02\x12\x03'\x19\x1d\
    \n\x0b\n\x04\x05\0\x02\x13\x12\x03(\x04\x1e\n\x0c\n\x05\x05\0\x02\x13\
    \x01\x12\x03(\x04\x16\n\x0c\n\x05\x05\0\x02\x13\x02\x12\x03(\x19\x1d\n\
    \x0b\n\x04\x05\0\x02\x14\x12\x03)\x04&\n\x0c\n\x05\x05\0\x02\x14\x01\x12\
    \x03)\x04\x1e\n\x0c\n\x05\x05\0\x02\x14\x02\x12\x03)!%\n\n\n\x02\x04\x01\
    \x12\x04,\08\x01\n\n\n\x03\x04\x01\x01\x12\x03,\x08\x13\n\x0b\n\x04\x04\
    \x01\x02\0\x12\x03-\x04%\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03-\x04\x0c\
    \n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03-\r\x13\n\x0c\n\x05\x04\x01\x02\0\
    \x01\x12\x03-\x14\x1e\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03-!$\n\x0b\n\
    \x04\x04\x01\x02\x01\x12\x03.\x04\"\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\
    \x03.\x04\x0c\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03.\r\x11\n\x0c\n\x05\
    \x04\x01\x02\x01\x01\x12\x03.\x12\x1b\n\x0c\n\x05\x04\x01\x02\x01\x03\
    \x12\x03.\x1e!\n\x0b\n\x04\x04\x01\x02\x02\x12\x03/\x04!\n\x0c\n\x05\x04\
    \x01\x02\x02\x04\x12\x03/\x04\x0c\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\
    \x03/\r\x11\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03/\x12\x1a\n\x0c\n\x05\
    \x04\x01\x02\x02\x03\x12\x03/\x1d\x20\n\x0b\n\x04\x04\x01\x02\x03\x12\
    \x030\x04!\n\x0c\n\x05\x04\x01\x02\x03\x04\x12\x030\x04\x0c\n\x0c\n\x05\
    \x04\x01\x02\x03\x05\x12\x030\r\x13\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\
    \x030\x14\x1a\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\x030\x1d\x20\n\x0b\n\
    \x04\x04\x01\x02\x04\x12\x031\x04\x1f\n\x0c\n\x05\x04\x01\x02\x04\x04\
    \x12\x031\x04\x0c\n\x0c\n\x05\x04\x01\x02\x04\x05\x12\x031\r\x13\n\x0c\n\
    \x05\x04\x01\x02\x04\x01\x12\x031\x14\x18\n\x0c\n\x05\x04\x01\x02\x04\
    \x03\x12\x031\x1b\x1e\n\x0b\n\x04\x04\x01\x02\x05\x12\x032\x04%\n\x0c\n\
    \x05\x04\x01\x02\x05\x04\x12\x032\x04\x0c\n\x0c\n\x05\x04\x01\x02\x05\
    \x05\x12\x032\r\x13\n\x0c\n\x05\x04\x01\x02\x05\x01\x12\x032\x14\x1e\n\
    \x0c\n\x05\x04\x01\x02\x05\x03\x12\x032!$\n\x0b\n\x04\x04\x01\x02\x06\
    \x12\x033\x04*\n\x0c\n\x05\x04\x01\x02\x06\x04\x12\x033\x04\x0c\n\x0c\n\
    \x05\x04\x01\x02\x06\x05\x12\x033\r\x12\n\x0c\n\x05\x04\x01\x02\x06\x01\
    \x12\x033\x13#\n\x0c\n\x05\x04\x01\x02\x06\x03\x12\x033&)\n\x0b\n\x04\
    \x04\x01\x02\x07\x12\x034\x04)\n\x0c\n\x05\x04\x01\x02\x07\x04\x12\x034\
    \x04\x0c\n\x0c\n\x05\x04\x01\x02\x07\x05\x12\x034\r\x13\n\x0c\n\x05\x04\
    \x01\x02\x07\x01\x12\x034\x14!\n\x0c\n\x05\x04\x01\x02\x07\x03\x12\x034$\
    (\n\x0b\n\x04\x04\x01\x02\x08\x12\x035\x04,\n\x0c\n\x05\x04\x01\x02\x08\
    \x04\x12\x035\x04\x0c\n\x0c\n\x05\x04\x01\x02\x08\x06\x12\x035\r\x17\n\
    \x0c\n\x05\x04\x01\x02\x08\x01\x12\x035\x18$\n\x0c\n\x05\x04\x01\x02\x08\
    \x03\x12\x035'+\n\x0b\n\x04\x04\x01\x02\t\x12\x036\x040\n\x0c\n\x05\x04\
    \x01\x02\t\x04\x12\x036\x04\x0c\n\x0c\n\x05\x04\x01\x02\t\x05\x12\x036\r\
    \x13\n\x0c\n\x05\x04\x01\x02\t\x01\x12\x036\x14(\n\x0c\n\x05\x04\x01\x02\
    \t\x03\x12\x036+/\n\x0b\n\x04\x04\x01\x02\n\x12\x037\x04&\n\x0c\n\x05\
    \x04\x01\x02\n\x04\x12\x037\x04\x0c\n\x0c\n\x05\x04\x01\x02\n\x06\x12\
    \x037\r\x15\n\x0c\n\x05\x04\x01\x02\n\x01\x12\x037\x16\x1e\n\x0c\n\x05\
    \x04\x01\x02\n\x03\x12\x037!%\n\n\n\x02\x04\x02\x12\x04:\0>\x01\n\n\n\
    \x03\x04\x02\x01\x12\x03:\x08\x12\n\x0b\n\x04\x04\x02\x02\0\x12\x03;\x04\
    &\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03;\x04\x0c\n\x0c\n\x05\x04\x02\x02\
    \0\x06\x12\x03;\r\x1b\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03;\x1c\x1f\n\
    \x0c\n\x05\x04\x02\x02\0\x03\x12\x03;\"%\n\x0b\n\x04\x04\x02\x02\x01\x12\
    \x03<\x04\"\n\x0c\n\x05\x04\x02\x02\x01\x04\x12\x03<\x04\x0c\n\x0c\n\x05\
    \x04\x02\x02\x01\x05\x12\x03<\r\x12\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\
    \x03<\x13\x1b\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03<\x1e!\n\x0b\n\x04\
    \x04\x02\x02\x02\x12\x03=\x04&\n\x0c\n\x05\x04\x02\x02\x02\x04\x12\x03=\
    \x04\x0c\n\x0c\n\x05\x04\x02\x02\x02\x05\x12\x03=\r\x13\n\x0c\n\x05\x04\
    \x02\x02\x02\x01\x12\x03=\x14\x1f\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\
    \x03=\"%\n\n\n\x02\x05\x01\x12\x04@\0M\x01\n\n\n\x03\x05\x01\x01\x12\x03\
    @\x05\x13\n\x0b\n\x04\x05\x01\x02\0\x12\x03A\x04\x1d\n\x0c\n\x05\x05\x01\
    \x02\0\x01\x12\x03A\x04\x16\n\x0c\n\x05\x05\x01\x02\0\x02\x12\x03A\x19\
    \x1c\n\x0b\n\x04\x05\x01\x02\x01\x12\x03B\x04\x17\n\x0c\n\x05\x05\x01\
    \x02\x01\x01\x12\x03B\x04\x10\n\x0c\n\x05\x05\x01\x02\x01\x02\x12\x03B\
    \x13\x16\n\x0b\n\x04\x05\x01\x02\x02\x12\x03C\x04\x1b\n\x0c\n\x05\x05\
    \x01\x02\x02\x01\x12\x03C\x04\x14\n\x0c\n\x05\x05\x01\x02\x02\x02\x12\
    \x03C\x17\x1a\n\x0b\n\x04\x05\x01\x02\x03\x12\x03D\x04\x16\n\x0c\n\x05\
    \x05\x01\x02\x03\x01\x12\x03D\x04\x0f\n\x0c\n\x05\x05\x01\x02\x03\x02\
    \x12\x03D\x12\x15\n\x0b\n\x04\x05\x01\x02\x04\x12\x03E\x04\x1b\n\x0c\n\
    \x05\x05\x01\x02\x04\x01\x12\x03E\x04\x14\n\x0c\n\x05\x05\x01\x02\x04\
    \x02\x12\x03E\x17\x1a\n\x0b\n\x04\x05\x01\x02\x05\x12\x03F\x04\x1a\n\x0c\
    \n\x05\x05\x01\x02\x05\x01\x12\x03F\x04\x13\n\x0c\n\x05\x05\x01\x02\x05\
    \x02\x12\x03F\x16\x19\n\x0b\n\x04\x05\x01\x02\x06\x12\x03G\x04\x18\n\x0c\
    \n\x05\x05\x01\x02\x06\x01\x12\x03G\x04\x11\n\x0c\n\x05\x05\x01\x02\x06\
    \x02\x12\x03G\x14\x17\n\x0b\n\x04\x05\x01\x02\x07\x12\x03H\x04\x17\n\x0c\
    \n\x05\x05\x01\x02\x07\x01\x12\x03H\x04\x10\n\x0c\n\x05\x05\x01\x02\x07\
    \x02\x12\x03H\x13\x16\n\x0b\n\x04\x05\x01\x02\x08\x12\x03I\x04\x1a\n\x0c\
    \n\x05\x05\x01\x02\x08\x01\x12\x03I\x04\x13\n\x0c\n\x05\x05\x01\x02\x08\
    \x02\x12\x03I\x16\x19\n\x0b\n\x04\x05\x01\x02\t\x12\x03J\x04\x17\n\x0c\n\
    \x05\x05\x01\x02\t\x01\x12\x03J\x04\x10\n\x0c\n\x05\x05\x01\x02\t\x02\
    \x12\x03J\x13\x16\n\x0b\n\x04\x05\x01\x02\n\x12\x03K\x04\x1a\n\x0c\n\x05\
    \x05\x01\x02\n\x01\x12\x03K\x04\x13\n\x0c\n\x05\x05\x01\x02\n\x02\x12\
    \x03K\x16\x19\n\x0b\n\x04\x05\x01\x02\x0b\x12\x03L\x04\x12\n\x0c\n\x05\
    \x05\x01\x02\x0b\x01\x12\x03L\x04\x0b\n\x0c\n\x05\x05\x01\x02\x0b\x02\
    \x12\x03L\x0e\x11\n\n\n\x02\x04\x03\x12\x04O\0Q\x01\n\n\n\x03\x04\x03\
    \x01\x12\x03O\x08\x0f\n\x0b\n\x04\x04\x03\x02\0\x12\x03P\x04!\n\x0c\n\
    \x05\x04\x03\x02\0\x04\x12\x03P\x04\x0c\n\x0c\n\x05\x04\x03\x02\0\x05\
    \x12\x03P\r\x13\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03P\x14\x1a\n\x0c\n\
    \x05\x04\x03\x02\0\x03\x12\x03P\x1d\x20\n\n\n\x02\x04\x04\x12\x04S\0c\
    \x01\n\n\n\x03\x04\x04\x01\x12\x03S\x08\r\n\x0b\n\x04\x04\x04\x02\0\x12\
    \x03T\x04&\n\x0c\n\x05\x04\x04\x02\0\x04\x12\x03T\x04\x0c\n\x0c\n\x05\
    \x04\x04\x02\0\x05\x12\x03T\r\x13\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03T\
    \x14\x1f\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03T\"%\n\x0b\n\x04\x04\x04\
    \x02\x01\x12\x03U\x04\x20\n\x0c\n\x05\x04\x04\x02\x01\x04\x12\x03U\x04\
    \x0c\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03U\r\x13\n\x0c\n\x05\x04\x04\
    \x02\x01\x01\x12\x03U\x14\x19\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03U\
    \x1c\x1f\n\x0b\n\x04\x04\x04\x02\x02\x12\x03V\x04&\n\x0c\n\x05\x04\x04\
    \x02\x02\x04\x12\x03V\x04\x0c\n\x0c\n\x05\x04\x04\x02\x02\x05\x12\x03V\r\
    \x13\n\x0c\n\x05\x04\x04\x02\x02\x01\x12\x03V\x14\x1f\n\x0c\n\x05\x04\
    \x04\x02\x02\x03\x12\x03V\"%\n\x0b\n\x04\x04\x04\x02\x03\x12\x03W\x04%\n\
    \x0c\n\x05\x04\x04\x02\x03\x04\x12\x03W\x04\x0c\n\x0c\n\x05\x04\x04\x02\
    \x03\x06\x12\x03W\r\x17\n\x0c\n\x05\x04\x04\x02\x03\x01\x12\x03W\x18\x1e\
    \n\x0c\n\x05\x04\x04\x02\x03\x03\x12\x03W!$\n\x0b\n\x04\x04\x04\x02\x04\
    \x12\x03X\x04/\n\x0c\n\x05\x04\x04\x02\x04\x04\x12\x03X\x04\x0c\n\x0c\n\
    \x05\x04\x04\x02\x04\x05\x12\x03X\r\x13\n\x0c\n\x05\x04\x04\x02\x04\x01\
    \x12\x03X\x14(\n\x0c\n\x05\x04\x04\x02\x04\x03\x12\x03X+.\n\x0b\n\x04\
    \x04\x04\x02\x05\x12\x03Y\x04.\n\x0c\n\x05\x04\x04\x02\x05\x04\x12\x03Y\
    \x04\x0c\n\x0c\n\x05\x04\x04\x02\x05\x05\x12\x03Y\r\x13\n\x0c\n\x05\x04\
    \x04\x02\x05\x01\x12\x03Y\x14'\n\x0c\n\x05\x04\x04\x02\x05\x03\x12\x03Y*\
    -\n\x0b\n\x04\x04\x04\x02\x06\x12\x03Z\x04\x20\n\x0c\n\x05\x04\x04\x02\
    \x06\x04\x12\x03Z\x04\x0c\n\x0c\n\x05\x04\x04\x02\x06\x05\x12\x03Z\r\x11\
    \n\x0c\n\x05\x04\x04\x02\x06\x01\x12\x03Z\x12\x19\n\x0c\n\x05\x04\x04\
    \x02\x06\x03\x12\x03Z\x1c\x1f\n\x0b\n\x04\x04\x04\x02\x07\x12\x03[\x04\
    \x1f\n\x0c\n\x05\x04\x04\x02\x07\x04\x12\x03[\x04\x0c\n\x0c\n\x05\x04\
    \x04\x02\x07\x05\x12\x03[\r\x11\n\x0c\n\x05\x04\x04\x02\x07\x01\x12\x03[\
    \x12\x18\n\x0c\n\x05\x04\x04\x02\x07\x03\x12\x03[\x1b\x1e\n\x0b\n\x04\
    \x04\x04\x02\x08\x12\x03\\\x04.\n\x0c\n\x05\x04\x04\x02\x08\x04\x12\x03\
    \\\x04\x0c\n\x0c\n\x05\x04\x04\x02\x08\x05\x12\x03\\\r\x13\n\x0c\n\x05\
    \x04\x04\x02\x08\x01\x12\x03\\\x14&\n\x0c\n\x05\x04\x04\x02\x08\x03\x12\
    \x03\\)-\n\x0b\n\x04\x04\x04\x02\t\x12\x03]\x04.\n\x0c\n\x05\x04\x04\x02\
    \t\x04\x12\x03]\x04\x0c\n\x0c\n\x05\x04\x04\x02\t\x05\x12\x03]\r\x13\n\
    \x0c\n\x05\x04\x04\x02\t\x01\x12\x03]\x14&\n\x0c\n\x05\x04\x04\x02\t\x03\
    \x12\x03])-\n\x0b\n\x04\x04\x04\x02\n\x12\x03^\x04/\n\x0c\n\x05\x04\x04\
    \x02\n\x04\x12\x03^\x04\x0c\n\x0c\n\x05\x04\x04\x02\n\x05\x12\x03^\r\x11\
    \n\x0c\n\x05\x04\x04\x02\n\x01\x12\x03^\x12'\n\x0c\n\x05\x04\x04\x02\n\
    \x03\x12\x03^*.\n\x0b\n\x04\x04\x04\x02\x0b\x12\x03_\x04\x1f\n\x0c\n\x05\
    \x04\x04\x02\x0b\x04\x12\x03_\x04\x0c\n\x0c\n\x05\x04\x04\x02\x0b\x05\
    \x12\x03_\r\x13\n\x0c\n\x05\x04\x04\x02\x0b\x01\x12\x03_\x14\x17\n\x0c\n\
    \x05\x04\x04\x02\x0b\x03\x12\x03_\x1a\x1e\n\x0b\n\x04\x04\x04\x02\x0c\
    \x12\x03`\x04/\n\x0c\n\x05\x04\x04\x02\x0c\x04\x12\x03`\x04\x0c\n\x0c\n\
    \x05\x04\x04\x02\x0c\x05\x12\x03`\r\x13\n\x0c\n\x05\x04\x04\x02\x0c\x01\
    \x12\x03`\x14'\n\x0c\n\x05\x04\x04\x02\x0c\x03\x12\x03`*.\n\x0b\n\x04\
    \x04\x04\x02\r\x12\x03a\x04#\n\x0c\n\x05\x04\x04\x02\r\x04\x12\x03a\x04\
    \x0c\n\x0c\n\x05\x04\x04\x02\r\x06\x12\x03a\r\x15\n\x0c\n\x05\x04\x04\
    \x02\r\x01\x12\x03a\x16\x1b\n\x0c\n\x05\x04\x04\x02\r\x03\x12\x03a\x1e\"\
    \n\x0b\n\x04\x04\x04\x02\x0e\x12\x03b\x04\x1a\n\x0c\n\x05\x04\x04\x02\
    \x0e\x04\x12\x03b\x04\x0c\n\x0c\n\x05\x04\x04\x02\x0e\x06\x12\x03b\r\x0f\
    \n\x0c\n\x05\x04\x04\x02\x0e\x01\x12\x03b\x10\x12\n\x0c\n\x05\x04\x04\
    \x02\x0e\x03\x12\x03b\x15\x19\n\n\n\x02\x05\x02\x12\x04e\0j\x01\n\n\n\
    \x03\x05\x02\x01\x12\x03e\x05\x0f\n\x0b\n\x04\x05\x02\x02\0\x12\x03f\x04\
    \x1a\n\x0c\n\x05\x05\x02\x02\0\x01\x12\x03f\x04\x13\n\x0c\n\x05\x05\x02\
    \x02\0\x02\x12\x03f\x16\x19\n\x0b\n\x04\x05\x02\x02\x01\x12\x03g\x04\x1a\
    \n\x0c\n\x05\x05\x02\x02\x01\x01\x12\x03g\x04\x13\n\x0c\n\x05\x05\x02\
    \x02\x01\x02\x12\x03g\x16\x19\n\x0b\n\x04\x05\x02\x02\x02\x12\x03h\x04\
    \x1b\n\x0c\n\x05\x05\x02\x02\x02\x01\x12\x03h\x04\x14\n\x0c\n\x05\x05\
    \x02\x02\x02\x02\x12\x03h\x17\x1a\n\x0b\n\x04\x05\x02\x02\x03\x12\x03i\
    \x04\x1d\n\x0c\n\x05\x05\x02\x02\x03\x01\x12\x03i\x04\x16\n\x0c\n\x05\
    \x05\x02\x02\x03\x02\x12\x03i\x19\x1c\n\n\n\x02\x04\x05\x12\x04l\0q\x01\
    \n\n\n\x03\x04\x05\x01\x12\x03l\x08\x10\n\x0b\n\x04\x04\x05\x02\0\x12\
    \x03m\x04\x1d\n\x0c\n\x05\x04\x05\x02\0\x04\x12\x03m\x04\x0c\n\x0c\n\x05\
    \x04\x05\x02\0\x05\x12\x03m\r\x12\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03m\
    \x13\x16\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03m\x19\x1c\n\x0b\n\x04\x04\
    \x05\x02\x01\x12\x03n\x04\x1e\n\x0c\n\x05\x04\x05\x02\x01\x04\x12\x03n\
    \x04\x0c\n\x0c\n\x05\x04\x05\x02\x01\x05\x12\x03n\r\x13\n\x0c\n\x05\x04\
    \x05\x02\x01\x01\x12\x03n\x14\x17\n\x0c\n\x05\x04\x05\x02\x01\x03\x12\
    \x03n\x1a\x1d\n\x0b\n\x04\x04\x05\x02\x02\x12\x03o\x04\x1f\n\x0c\n\x05\
    \x04\x05\x02\x02\x04\x12\x03o\x04\x0c\n\x0c\n\x05\x04\x05\x02\x02\x05\
    \x12\x03o\r\x11\n\x0c\n\x05\x04\x05\x02\x02\x01\x12\x03o\x12\x18\n\x0c\n\
    \x05\x04\x05\x02\x02\x03\x12\x03o\x1b\x1e\n\x0b\n\x04\x04\x05\x02\x03\
    \x12\x03p\x04\"\n\x0c\n\x05\x04\x05\x02\x03\x04\x12\x03p\x04\x0c\n\x0c\n\
    \x05\x04\x05\x02\x03\x05\x12\x03p\r\x13\n\x0c\n\x05\x04\x05\x02\x03\x01\
    \x12\x03p\x14\x1b\n\x0c\n\x05\x04\x05\x02\x03\x03\x12\x03p\x1e!\n\n\n\
    \x02\x04\x06\x12\x04s\0}\x01\n\n\n\x03\x04\x06\x01\x12\x03s\x08\n\n\x0b\
    \n\x04\x04\x06\x02\0\x12\x03t\x04\x1e\n\x0c\n\x05\x04\x06\x02\0\x04\x12\
    \x03t\x04\x0c\n\x0c\n\x05\x04\x06\x02\0\x05\x12\x03t\r\x12\n\x0c\n\x05\
    \x04\x06\x02\0\x01\x12\x03t\x13\x17\n\x0c\n\x05\x04\x06\x02\0\x03\x12\
    \x03t\x1a\x1d\n\x0b\n\x04\x04\x06\x02\x01\x12\x03u\x04!\n\x0c\n\x05\x04\
    \x06\x02\x01\x04\x12\x03u\x04\x0c\n\x0c\n\x05\x04\x06\x02\x01\x05\x12\
    \x03u\r\x12\n\x0c\n\x05\x04\x06\x02\x01\x01\x12\x03u\x13\x1a\n\x0c\n\x05\
    \x04\x06\x02\x01\x03\x12\x03u\x1d\x20\n\x0b\n\x04\x04\x06\x02\x02\x12\
    \x03v\x04#\n\x0c\n\x05\x04\x06\x02\x02\x04\x12\x03v\x04\x0c\n\x0c\n\x05\
    \x04\x06\x02\x02\x05\x12\x03v\r\x12\n\x0c\n\x05\x04\x06\x02\x02\x01\x12\
    \x03v\x13\x1c\n\x0c\n\x05\x04\x06\x02\x02\x03\x12\x03v\x1f\"\n\x0b\n\x04\
    \x04\x06\x02\x03\x12\x03w\x04\"\n\x0c\n\x05\x04\x06\x02\x03\x04\x12\x03w\
    \x04\x0c\n\x0c\n\x05\x04\x06\x02\x03\x05\x12\x03w\r\x12\n\x0c\n\x05\x04\
    \x06\x02\x03\x01\x12\x03w\x13\x1b\n\x0c\n\x05\x04\x06\x02\x03\x03\x12\
    \x03w\x1e!\n\x0b\n\x04\x04\x06\x02\x04\x12\x03x\x04$\n\x0c\n\x05\x04\x06\
    \x02\x04\x04\x12\x03x\x04\x0c\n\x0c\n\x05\x04\x06\x02\x04\x05\x12\x03x\r\
    \x13\n\x0c\n\x05\x04\x06\x02\x04\x01\x12\x03x\x14\x1d\n\x0c\n\x05\x04\
    \x06\x02\x04\x03\x12\x03x\x20#\n\x0b\n\x04\x04\x06\x02\x05\x12\x03y\x04)\
    \n\x0c\n\x05\x04\x06\x02\x05\x04\x12\x03y\x04\x0c\n\x0c\n\x05\x04\x06\
    \x02\x05\x05\x12\x03y\r\x13\n\x0c\n\x05\x04\x06\x02\x05\x01\x12\x03y\x14\
    \"\n\x0c\n\x05\x04\x06\x02\x05\x03\x12\x03y%(\n\x0b\n\x04\x04\x06\x02\
    \x06\x12\x03z\x04\"\n\x0c\n\x05\x04\x06\x02\x06\x04\x12\x03z\x04\x0c\n\
    \x0c\n\x05\x04\x06\x02\x06\x05\x12\x03z\r\x13\n\x0c\n\x05\x04\x06\x02\
    \x06\x01\x12\x03z\x14\x1b\n\x0c\n\x05\x04\x06\x02\x06\x03\x12\x03z\x1e!\
    \n\x0b\n\x04\x04\x06\x02\x07\x12\x03{\x04%\n\x0c\n\x05\x04\x06\x02\x07\
    \x04\x12\x03{\x04\x0c\n\x0c\n\x05\x04\x06\x02\x07\x05\x12\x03{\r\x13\n\
    \x0c\n\x05\x04\x06\x02\x07\x01\x12\x03{\x14\x1e\n\x0c\n\x05\x04\x06\x02\
    \x07\x03\x12\x03{!$\n\x0b\n\x04\x04\x06\x02\x08\x12\x03|\x04\x1d\n\x0c\n\
    \x05\x04\x06\x02\x08\x04\x12\x03|\x04\x0c\n\x0c\n\x05\x04\x06\x02\x08\
    \x05\x12\x03|\r\x12\n\x0c\n\x05\x04\x06\x02\x08\x01\x12\x03|\x13\x16\n\
    \x0c\n\x05\x04\x06\x02\x08\x03\x12\x03|\x19\x1c\n\x0b\n\x02\x04\x07\x12\
    \x05\x7f\0\x82\x01\x01\n\n\n\x03\x04\x07\x01\x12\x03\x7f\x08\x10\n\x0c\n\
    \x04\x04\x07\x02\0\x12\x04\x80\x01\x04\x1f\n\r\n\x05\x04\x07\x02\0\x04\
    \x12\x04\x80\x01\x04\x0c\n\r\n\x05\x04\x07\x02\0\x05\x12\x04\x80\x01\r\
    \x13\n\r\n\x05\x04\x07\x02\0\x01\x12\x04\x80\x01\x14\x18\n\r\n\x05\x04\
    \x07\x02\0\x03\x12\x04\x80\x01\x1b\x1e\n\x0c\n\x04\x04\x07\x02\x01\x12\
    \x04\x81\x01\x04#\n\r\n\x05\x04\x07\x02\x01\x04\x12\x04\x81\x01\x04\x0c\
    \n\r\n\x05\x04\x07\x02\x01\x05\x12\x04\x81\x01\r\x13\n\r\n\x05\x04\x07\
    \x02\x01\x01\x12\x04\x81\x01\x14\x1c\n\r\n\x05\x04\x07\x02\x01\x03\x12\
    \x04\x81\x01\x1f\"\
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
