// This file is generated by rust-protobuf 2.3.0. Do not edit
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
pub struct FileCreateTransactionBody {
    // message fields
    pub expirationTime: ::protobuf::SingularPtrField<super::Timestamp::Timestamp>,
    pub keys: ::protobuf::SingularPtrField<super::BasicTypes::KeyList>,
    pub contents: ::std::vec::Vec<u8>,
    pub shardID: ::protobuf::SingularPtrField<super::BasicTypes::ShardID>,
    pub realmID: ::protobuf::SingularPtrField<super::BasicTypes::RealmID>,
    pub newRealmAdminKey: ::protobuf::SingularPtrField<super::BasicTypes::Key>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl FileCreateTransactionBody {
    pub fn new() -> FileCreateTransactionBody {
        ::std::default::Default::default()
    }

    // .proto.Timestamp expirationTime = 2;

    pub fn clear_expirationTime(&mut self) {
        self.expirationTime.clear();
    }

    pub fn has_expirationTime(&self) -> bool {
        self.expirationTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expirationTime(&mut self, v: super::Timestamp::Timestamp) {
        self.expirationTime = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_expirationTime(&mut self) -> &mut super::Timestamp::Timestamp {
        if self.expirationTime.is_none() {
            self.expirationTime.set_default();
        }
        self.expirationTime.as_mut().unwrap()
    }

    // Take field
    pub fn take_expirationTime(&mut self) -> super::Timestamp::Timestamp {
        self.expirationTime.take().unwrap_or_else(|| super::Timestamp::Timestamp::new())
    }

    pub fn get_expirationTime(&self) -> &super::Timestamp::Timestamp {
        self.expirationTime.as_ref().unwrap_or_else(|| super::Timestamp::Timestamp::default_instance())
    }

    // .proto.KeyList keys = 3;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    pub fn has_keys(&self) -> bool {
        self.keys.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: super::BasicTypes::KeyList) {
        self.keys = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_keys(&mut self) -> &mut super::BasicTypes::KeyList {
        if self.keys.is_none() {
            self.keys.set_default();
        }
        self.keys.as_mut().unwrap()
    }

    // Take field
    pub fn take_keys(&mut self) -> super::BasicTypes::KeyList {
        self.keys.take().unwrap_or_else(|| super::BasicTypes::KeyList::new())
    }

    pub fn get_keys(&self) -> &super::BasicTypes::KeyList {
        self.keys.as_ref().unwrap_or_else(|| super::BasicTypes::KeyList::default_instance())
    }

    // bytes contents = 4;

    pub fn clear_contents(&mut self) {
        self.contents.clear();
    }

    // Param is passed by value, moved
    pub fn set_contents(&mut self, v: ::std::vec::Vec<u8>) {
        self.contents = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contents(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.contents
    }

    // Take field
    pub fn take_contents(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.contents, ::std::vec::Vec::new())
    }

    pub fn get_contents(&self) -> &[u8] {
        &self.contents
    }

    // .proto.ShardID shardID = 5;

    pub fn clear_shardID(&mut self) {
        self.shardID.clear();
    }

    pub fn has_shardID(&self) -> bool {
        self.shardID.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shardID(&mut self, v: super::BasicTypes::ShardID) {
        self.shardID = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_shardID(&mut self) -> &mut super::BasicTypes::ShardID {
        if self.shardID.is_none() {
            self.shardID.set_default();
        }
        self.shardID.as_mut().unwrap()
    }

    // Take field
    pub fn take_shardID(&mut self) -> super::BasicTypes::ShardID {
        self.shardID.take().unwrap_or_else(|| super::BasicTypes::ShardID::new())
    }

    pub fn get_shardID(&self) -> &super::BasicTypes::ShardID {
        self.shardID.as_ref().unwrap_or_else(|| super::BasicTypes::ShardID::default_instance())
    }

    // .proto.RealmID realmID = 6;

    pub fn clear_realmID(&mut self) {
        self.realmID.clear();
    }

    pub fn has_realmID(&self) -> bool {
        self.realmID.is_some()
    }

    // Param is passed by value, moved
    pub fn set_realmID(&mut self, v: super::BasicTypes::RealmID) {
        self.realmID = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_realmID(&mut self) -> &mut super::BasicTypes::RealmID {
        if self.realmID.is_none() {
            self.realmID.set_default();
        }
        self.realmID.as_mut().unwrap()
    }

    // Take field
    pub fn take_realmID(&mut self) -> super::BasicTypes::RealmID {
        self.realmID.take().unwrap_or_else(|| super::BasicTypes::RealmID::new())
    }

    pub fn get_realmID(&self) -> &super::BasicTypes::RealmID {
        self.realmID.as_ref().unwrap_or_else(|| super::BasicTypes::RealmID::default_instance())
    }

    // .proto.Key newRealmAdminKey = 7;

    pub fn clear_newRealmAdminKey(&mut self) {
        self.newRealmAdminKey.clear();
    }

    pub fn has_newRealmAdminKey(&self) -> bool {
        self.newRealmAdminKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_newRealmAdminKey(&mut self, v: super::BasicTypes::Key) {
        self.newRealmAdminKey = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_newRealmAdminKey(&mut self) -> &mut super::BasicTypes::Key {
        if self.newRealmAdminKey.is_none() {
            self.newRealmAdminKey.set_default();
        }
        self.newRealmAdminKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_newRealmAdminKey(&mut self) -> super::BasicTypes::Key {
        self.newRealmAdminKey.take().unwrap_or_else(|| super::BasicTypes::Key::new())
    }

    pub fn get_newRealmAdminKey(&self) -> &super::BasicTypes::Key {
        self.newRealmAdminKey.as_ref().unwrap_or_else(|| super::BasicTypes::Key::default_instance())
    }
}

impl ::protobuf::Message for FileCreateTransactionBody {
    fn is_initialized(&self) -> bool {
        for v in &self.expirationTime {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.keys {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.shardID {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.realmID {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.newRealmAdminKey {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.expirationTime)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.keys)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.contents)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.shardID)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.realmID)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.newRealmAdminKey)?;
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
        if let Some(ref v) = self.expirationTime.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.keys.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.contents.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.contents);
        }
        if let Some(ref v) = self.shardID.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.realmID.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.newRealmAdminKey.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.expirationTime.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.keys.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.contents.is_empty() {
            os.write_bytes(4, &self.contents)?;
        }
        if let Some(ref v) = self.shardID.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.realmID.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.newRealmAdminKey.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
        Self::descriptor_static()
    }

    fn new() -> FileCreateTransactionBody {
        FileCreateTransactionBody::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::Timestamp::Timestamp>>(
                    "expirationTime",
                    |m: &FileCreateTransactionBody| { &m.expirationTime },
                    |m: &mut FileCreateTransactionBody| { &mut m.expirationTime },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::BasicTypes::KeyList>>(
                    "keys",
                    |m: &FileCreateTransactionBody| { &m.keys },
                    |m: &mut FileCreateTransactionBody| { &mut m.keys },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "contents",
                    |m: &FileCreateTransactionBody| { &m.contents },
                    |m: &mut FileCreateTransactionBody| { &mut m.contents },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::BasicTypes::ShardID>>(
                    "shardID",
                    |m: &FileCreateTransactionBody| { &m.shardID },
                    |m: &mut FileCreateTransactionBody| { &mut m.shardID },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::BasicTypes::RealmID>>(
                    "realmID",
                    |m: &FileCreateTransactionBody| { &m.realmID },
                    |m: &mut FileCreateTransactionBody| { &mut m.realmID },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::BasicTypes::Key>>(
                    "newRealmAdminKey",
                    |m: &FileCreateTransactionBody| { &m.newRealmAdminKey },
                    |m: &mut FileCreateTransactionBody| { &mut m.newRealmAdminKey },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FileCreateTransactionBody>(
                    "FileCreateTransactionBody",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static FileCreateTransactionBody {
        static mut instance: ::protobuf::lazy::Lazy<FileCreateTransactionBody> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FileCreateTransactionBody,
        };
        unsafe {
            instance.get(FileCreateTransactionBody::new)
        }
    }
}

impl ::protobuf::Clear for FileCreateTransactionBody {
    fn clear(&mut self) {
        self.clear_expirationTime();
        self.clear_keys();
        self.clear_contents();
        self.clear_shardID();
        self.clear_realmID();
        self.clear_newRealmAdminKey();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FileCreateTransactionBody {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FileCreateTransactionBody {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10FileCreate.proto\x12\x05proto\x1a\x10BasicTypes.proto\x1a\x0fTimes\
    tamp.proto\"\xa1\x02\n\x19FileCreateTransactionBody\x128\n\x0eexpiration\
    Time\x18\x02\x20\x01(\x0b2\x10.proto.TimestampR\x0eexpirationTime\x12\"\
    \n\x04keys\x18\x03\x20\x01(\x0b2\x0e.proto.KeyListR\x04keys\x12\x1a\n\
    \x08contents\x18\x04\x20\x01(\x0cR\x08contents\x12(\n\x07shardID\x18\x05\
    \x20\x01(\x0b2\x0e.proto.ShardIDR\x07shardID\x12(\n\x07realmID\x18\x06\
    \x20\x01(\x0b2\x0e.proto.RealmIDR\x07realmID\x126\n\x10newRealmAdminKey\
    \x18\x07\x20\x01(\x0b2\n.proto.KeyR\x10newRealmAdminKeyB&\n\"com.hederah\
    ashgraph.api.proto.javaP\x01b\x06proto3\
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