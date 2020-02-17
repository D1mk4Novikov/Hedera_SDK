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
pub struct FileUpdateTransactionBody {
    // message fields
    pub fileID: ::protobuf::SingularPtrField<super::BasicTypes::FileID>,
    pub expirationTime: ::protobuf::SingularPtrField<super::Timestamp::Timestamp>,
    pub keys: ::protobuf::SingularPtrField<super::BasicTypes::KeyList>,
    pub contents: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl FileUpdateTransactionBody {
    pub fn new() -> FileUpdateTransactionBody {
        ::std::default::Default::default()
    }

    // .proto.FileID fileID = 1;

    pub fn clear_fileID(&mut self) {
        self.fileID.clear();
    }

    pub fn has_fileID(&self) -> bool {
        self.fileID.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileID(&mut self, v: super::BasicTypes::FileID) {
        self.fileID = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fileID(&mut self) -> &mut super::BasicTypes::FileID {
        if self.fileID.is_none() {
            self.fileID.set_default();
        }
        self.fileID.as_mut().unwrap()
    }

    // Take field
    pub fn take_fileID(&mut self) -> super::BasicTypes::FileID {
        self.fileID.take().unwrap_or_else(|| super::BasicTypes::FileID::new())
    }

    pub fn get_fileID(&self) -> &super::BasicTypes::FileID {
        self.fileID.as_ref().unwrap_or_else(|| super::BasicTypes::FileID::default_instance())
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
}

impl ::protobuf::Message for FileUpdateTransactionBody {
    fn is_initialized(&self) -> bool {
        for v in &self.fileID {
            if !v.is_initialized() {
                return false;
            }
        };
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
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fileID)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.expirationTime)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.keys)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.contents)?;
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
        if let Some(ref v) = self.fileID.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.fileID.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
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

    fn new() -> FileUpdateTransactionBody {
        FileUpdateTransactionBody::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::BasicTypes::FileID>>(
                    "fileID",
                    |m: &FileUpdateTransactionBody| { &m.fileID },
                    |m: &mut FileUpdateTransactionBody| { &mut m.fileID },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::Timestamp::Timestamp>>(
                    "expirationTime",
                    |m: &FileUpdateTransactionBody| { &m.expirationTime },
                    |m: &mut FileUpdateTransactionBody| { &mut m.expirationTime },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::BasicTypes::KeyList>>(
                    "keys",
                    |m: &FileUpdateTransactionBody| { &m.keys },
                    |m: &mut FileUpdateTransactionBody| { &mut m.keys },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "contents",
                    |m: &FileUpdateTransactionBody| { &m.contents },
                    |m: &mut FileUpdateTransactionBody| { &mut m.contents },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FileUpdateTransactionBody>(
                    "FileUpdateTransactionBody",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static FileUpdateTransactionBody {
        static mut instance: ::protobuf::lazy::Lazy<FileUpdateTransactionBody> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FileUpdateTransactionBody,
        };
        unsafe {
            instance.get(FileUpdateTransactionBody::new)
        }
    }
}

impl ::protobuf::Clear for FileUpdateTransactionBody {
    fn clear(&mut self) {
        self.clear_fileID();
        self.clear_expirationTime();
        self.clear_keys();
        self.clear_contents();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FileUpdateTransactionBody {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FileUpdateTransactionBody {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10FileUpdate.proto\x12\x05proto\x1a\x10BasicTypes.proto\x1a\x0fTimes\
    tamp.proto\"\xbc\x01\n\x19FileUpdateTransactionBody\x12%\n\x06fileID\x18\
    \x01\x20\x01(\x0b2\r.proto.FileIDR\x06fileID\x128\n\x0eexpirationTime\
    \x18\x02\x20\x01(\x0b2\x10.proto.TimestampR\x0eexpirationTime\x12\"\n\
    \x04keys\x18\x03\x20\x01(\x0b2\x0e.proto.KeyListR\x04keys\x12\x1a\n\x08c\
    ontents\x18\x04\x20\x01(\x0cR\x08contentsB&\n\"com.hederahashgraph.api.p\
    roto.javaP\x01b\x06proto3\
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