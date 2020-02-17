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
pub struct FileDeleteTransactionBody {
    // message fields
    pub fileID: ::protobuf::SingularPtrField<super::BasicTypes::FileID>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl FileDeleteTransactionBody {
    pub fn new() -> FileDeleteTransactionBody {
        ::std::default::Default::default()
    }

    // .proto.FileID fileID = 2;

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
}

impl ::protobuf::Message for FileDeleteTransactionBody {
    fn is_initialized(&self) -> bool {
        for v in &self.fileID {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fileID)?;
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.fileID.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> FileDeleteTransactionBody {
        FileDeleteTransactionBody::new()
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
                    |m: &FileDeleteTransactionBody| { &m.fileID },
                    |m: &mut FileDeleteTransactionBody| { &mut m.fileID },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FileDeleteTransactionBody>(
                    "FileDeleteTransactionBody",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static FileDeleteTransactionBody {
        static mut instance: ::protobuf::lazy::Lazy<FileDeleteTransactionBody> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FileDeleteTransactionBody,
        };
        unsafe {
            instance.get(FileDeleteTransactionBody::new)
        }
    }
}

impl ::protobuf::Clear for FileDeleteTransactionBody {
    fn clear(&mut self) {
        self.clear_fileID();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FileDeleteTransactionBody {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FileDeleteTransactionBody {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10FileDelete.proto\x12\x05proto\x1a\x10BasicTypes.proto\"B\n\x19File\
    DeleteTransactionBody\x12%\n\x06fileID\x18\x02\x20\x01(\x0b2\r.proto.Fil\
    eIDR\x06fileIDB&\n\"com.hederahashgraph.api.proto.javaP\x01b\x06proto3\
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