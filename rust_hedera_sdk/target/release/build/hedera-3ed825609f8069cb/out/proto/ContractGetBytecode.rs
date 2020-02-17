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
pub struct ContractGetBytecodeQuery {
    // message fields
    pub header: ::protobuf::SingularPtrField<super::QueryHeader::QueryHeader>,
    pub contractID: ::protobuf::SingularPtrField<super::BasicTypes::ContractID>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl ContractGetBytecodeQuery {
    pub fn new() -> ContractGetBytecodeQuery {
        ::std::default::Default::default()
    }

    // .proto.QueryHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: super::QueryHeader::QueryHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut super::QueryHeader::QueryHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> super::QueryHeader::QueryHeader {
        self.header.take().unwrap_or_else(|| super::QueryHeader::QueryHeader::new())
    }

    pub fn get_header(&self) -> &super::QueryHeader::QueryHeader {
        self.header.as_ref().unwrap_or_else(|| super::QueryHeader::QueryHeader::default_instance())
    }

    // .proto.ContractID contractID = 2;

    pub fn clear_contractID(&mut self) {
        self.contractID.clear();
    }

    pub fn has_contractID(&self) -> bool {
        self.contractID.is_some()
    }

    // Param is passed by value, moved
    pub fn set_contractID(&mut self, v: super::BasicTypes::ContractID) {
        self.contractID = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contractID(&mut self) -> &mut super::BasicTypes::ContractID {
        if self.contractID.is_none() {
            self.contractID.set_default();
        }
        self.contractID.as_mut().unwrap()
    }

    // Take field
    pub fn take_contractID(&mut self) -> super::BasicTypes::ContractID {
        self.contractID.take().unwrap_or_else(|| super::BasicTypes::ContractID::new())
    }

    pub fn get_contractID(&self) -> &super::BasicTypes::ContractID {
        self.contractID.as_ref().unwrap_or_else(|| super::BasicTypes::ContractID::default_instance())
    }
}

impl ::protobuf::Message for ContractGetBytecodeQuery {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.contractID {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.contractID)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.contractID.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.contractID.as_ref() {
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

    fn new() -> ContractGetBytecodeQuery {
        ContractGetBytecodeQuery::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::QueryHeader::QueryHeader>>(
                    "header",
                    |m: &ContractGetBytecodeQuery| { &m.header },
                    |m: &mut ContractGetBytecodeQuery| { &mut m.header },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::BasicTypes::ContractID>>(
                    "contractID",
                    |m: &ContractGetBytecodeQuery| { &m.contractID },
                    |m: &mut ContractGetBytecodeQuery| { &mut m.contractID },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ContractGetBytecodeQuery>(
                    "ContractGetBytecodeQuery",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static ContractGetBytecodeQuery {
        static mut instance: ::protobuf::lazy::Lazy<ContractGetBytecodeQuery> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ContractGetBytecodeQuery,
        };
        unsafe {
            instance.get(ContractGetBytecodeQuery::new)
        }
    }
}

impl ::protobuf::Clear for ContractGetBytecodeQuery {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_contractID();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ContractGetBytecodeQuery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ContractGetBytecodeQuery {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ContractGetBytecodeResponse {
    // message fields
    pub header: ::protobuf::SingularPtrField<super::ResponseHeader::ResponseHeader>,
    pub bytecode: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl ContractGetBytecodeResponse {
    pub fn new() -> ContractGetBytecodeResponse {
        ::std::default::Default::default()
    }

    // .proto.ResponseHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: super::ResponseHeader::ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut super::ResponseHeader::ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> super::ResponseHeader::ResponseHeader {
        self.header.take().unwrap_or_else(|| super::ResponseHeader::ResponseHeader::new())
    }

    pub fn get_header(&self) -> &super::ResponseHeader::ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| super::ResponseHeader::ResponseHeader::default_instance())
    }

    // bytes bytecode = 6;

    pub fn clear_bytecode(&mut self) {
        self.bytecode.clear();
    }

    // Param is passed by value, moved
    pub fn set_bytecode(&mut self, v: ::std::vec::Vec<u8>) {
        self.bytecode = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bytecode(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.bytecode
    }

    // Take field
    pub fn take_bytecode(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.bytecode, ::std::vec::Vec::new())
    }

    pub fn get_bytecode(&self) -> &[u8] {
        &self.bytecode
    }
}

impl ::protobuf::Message for ContractGetBytecodeResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.header {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.bytecode)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.bytecode.is_empty() {
            my_size += ::protobuf::rt::bytes_size(6, &self.bytecode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.bytecode.is_empty() {
            os.write_bytes(6, &self.bytecode)?;
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

    fn new() -> ContractGetBytecodeResponse {
        ContractGetBytecodeResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::ResponseHeader::ResponseHeader>>(
                    "header",
                    |m: &ContractGetBytecodeResponse| { &m.header },
                    |m: &mut ContractGetBytecodeResponse| { &mut m.header },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "bytecode",
                    |m: &ContractGetBytecodeResponse| { &m.bytecode },
                    |m: &mut ContractGetBytecodeResponse| { &mut m.bytecode },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ContractGetBytecodeResponse>(
                    "ContractGetBytecodeResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static ContractGetBytecodeResponse {
        static mut instance: ::protobuf::lazy::Lazy<ContractGetBytecodeResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ContractGetBytecodeResponse,
        };
        unsafe {
            instance.get(ContractGetBytecodeResponse::new)
        }
    }
}

impl ::protobuf::Clear for ContractGetBytecodeResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_bytecode();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ContractGetBytecodeResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ContractGetBytecodeResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19ContractGetBytecode.proto\x12\x05proto\x1a\x10BasicTypes.proto\x1a\
    \x11QueryHeader.proto\x1a\x14ResponseHeader.proto\"y\n\x18ContractGetByt\
    ecodeQuery\x12*\n\x06header\x18\x01\x20\x01(\x0b2\x12.proto.QueryHeaderR\
    \x06header\x121\n\ncontractID\x18\x02\x20\x01(\x0b2\x11.proto.ContractID\
    R\ncontractID\"h\n\x1bContractGetBytecodeResponse\x12-\n\x06header\x18\
    \x01\x20\x01(\x0b2\x15.proto.ResponseHeaderR\x06header\x12\x1a\n\x08byte\
    code\x18\x06\x20\x01(\x0cR\x08bytecodeB&\n\"com.hederahashgraph.api.prot\
    o.javaP\x01b\x06proto3\
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