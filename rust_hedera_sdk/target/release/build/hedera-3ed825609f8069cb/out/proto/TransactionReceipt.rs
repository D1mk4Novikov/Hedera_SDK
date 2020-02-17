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
pub struct TransactionReceipt {
    // message fields
    pub status: super::ResponseCode::ResponseCodeEnum,
    pub accountID: ::protobuf::SingularPtrField<super::BasicTypes::AccountID>,
    pub fileID: ::protobuf::SingularPtrField<super::BasicTypes::FileID>,
    pub contractID: ::protobuf::SingularPtrField<super::BasicTypes::ContractID>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl TransactionReceipt {
    pub fn new() -> TransactionReceipt {
        ::std::default::Default::default()
    }

    // .proto.ResponseCodeEnum status = 1;

    pub fn clear_status(&mut self) {
        self.status = super::ResponseCode::ResponseCodeEnum::OK;
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: super::ResponseCode::ResponseCodeEnum) {
        self.status = v;
    }

    pub fn get_status(&self) -> super::ResponseCode::ResponseCodeEnum {
        self.status
    }

    // .proto.AccountID accountID = 2;

    pub fn clear_accountID(&mut self) {
        self.accountID.clear();
    }

    pub fn has_accountID(&self) -> bool {
        self.accountID.is_some()
    }

    // Param is passed by value, moved
    pub fn set_accountID(&mut self, v: super::BasicTypes::AccountID) {
        self.accountID = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_accountID(&mut self) -> &mut super::BasicTypes::AccountID {
        if self.accountID.is_none() {
            self.accountID.set_default();
        }
        self.accountID.as_mut().unwrap()
    }

    // Take field
    pub fn take_accountID(&mut self) -> super::BasicTypes::AccountID {
        self.accountID.take().unwrap_or_else(|| super::BasicTypes::AccountID::new())
    }

    pub fn get_accountID(&self) -> &super::BasicTypes::AccountID {
        self.accountID.as_ref().unwrap_or_else(|| super::BasicTypes::AccountID::default_instance())
    }

    // .proto.FileID fileID = 3;

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

    // .proto.ContractID contractID = 4;

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

impl ::protobuf::Message for TransactionReceipt {
    fn is_initialized(&self) -> bool {
        for v in &self.accountID {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.fileID {
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
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.status, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.accountID)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fileID)?;
                },
                4 => {
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
        if self.status != super::ResponseCode::ResponseCodeEnum::OK {
            my_size += ::protobuf::rt::enum_size(1, self.status);
        }
        if let Some(ref v) = self.accountID.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.fileID.as_ref() {
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
        if self.status != super::ResponseCode::ResponseCodeEnum::OK {
            os.write_enum(1, self.status.value())?;
        }
        if let Some(ref v) = self.accountID.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.fileID.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.contractID.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn new() -> TransactionReceipt {
        TransactionReceipt::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::ResponseCode::ResponseCodeEnum>>(
                    "status",
                    |m: &TransactionReceipt| { &m.status },
                    |m: &mut TransactionReceipt| { &mut m.status },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::BasicTypes::AccountID>>(
                    "accountID",
                    |m: &TransactionReceipt| { &m.accountID },
                    |m: &mut TransactionReceipt| { &mut m.accountID },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::BasicTypes::FileID>>(
                    "fileID",
                    |m: &TransactionReceipt| { &m.fileID },
                    |m: &mut TransactionReceipt| { &mut m.fileID },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::BasicTypes::ContractID>>(
                    "contractID",
                    |m: &TransactionReceipt| { &m.contractID },
                    |m: &mut TransactionReceipt| { &mut m.contractID },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TransactionReceipt>(
                    "TransactionReceipt",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static TransactionReceipt {
        static mut instance: ::protobuf::lazy::Lazy<TransactionReceipt> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TransactionReceipt,
        };
        unsafe {
            instance.get(TransactionReceipt::new)
        }
    }
}

impl ::protobuf::Clear for TransactionReceipt {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_accountID();
        self.clear_fileID();
        self.clear_contractID();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TransactionReceipt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TransactionReceipt {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18TransactionReceipt.proto\x12\x05proto\x1a\x10BasicTypes.proto\x1a\
    \x12ResponseCode.proto\"\xcf\x01\n\x12TransactionReceipt\x12/\n\x06statu\
    s\x18\x01\x20\x01(\x0e2\x17.proto.ResponseCodeEnumR\x06status\x12.\n\tac\
    countID\x18\x02\x20\x01(\x0b2\x10.proto.AccountIDR\taccountID\x12%\n\x06\
    fileID\x18\x03\x20\x01(\x0b2\r.proto.FileIDR\x06fileID\x121\n\ncontractI\
    D\x18\x04\x20\x01(\x0b2\x11.proto.ContractIDR\ncontractIDB&\n\"com.heder\
    ahashgraph.api.proto.javaP\x01b\x06proto3\
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