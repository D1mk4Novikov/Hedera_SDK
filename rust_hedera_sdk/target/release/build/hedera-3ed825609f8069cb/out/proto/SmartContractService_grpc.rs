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


// interface

pub trait SmartContractService {
    fn create_contract(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse>;

    fn update_contract(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse>;

    fn contract_call_method(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse>;

    fn get_contract_info(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response>;

    fn contract_call_local_method(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response>;

    fn contract_get_bytecode(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response>;

    fn get_by_solidity_id(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response>;

    fn get_tx_record_by_contract_id(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response>;
}

// client

pub struct SmartContractServiceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_createContract: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Transaction::Transaction, super::TransactionResponse::TransactionResponse>>,
    method_updateContract: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Transaction::Transaction, super::TransactionResponse::TransactionResponse>>,
    method_contractCallMethod: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Transaction::Transaction, super::TransactionResponse::TransactionResponse>>,
    method_getContractInfo: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Query::Query, super::Response::Response>>,
    method_contractCallLocalMethod: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Query::Query, super::Response::Response>>,
    method_ContractGetBytecode: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Query::Query, super::Response::Response>>,
    method_getBySolidityID: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Query::Query, super::Response::Response>>,
    method_getTxRecordByContractID: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Query::Query, super::Response::Response>>,
}

impl ::grpc::ClientStub for SmartContractServiceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        SmartContractServiceClient {
            grpc_client: grpc_client,
            method_createContract: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.SmartContractService/createContract".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_updateContract: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.SmartContractService/updateContract".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_contractCallMethod: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.SmartContractService/contractCallMethod".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_getContractInfo: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.SmartContractService/getContractInfo".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_contractCallLocalMethod: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.SmartContractService/contractCallLocalMethod".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ContractGetBytecode: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.SmartContractService/ContractGetBytecode".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_getBySolidityID: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.SmartContractService/getBySolidityID".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_getTxRecordByContractID: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.SmartContractService/getTxRecordByContractID".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl SmartContractService for SmartContractServiceClient {
    fn create_contract(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse> {
        self.grpc_client.call_unary(o, p, self.method_createContract.clone())
    }

    fn update_contract(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse> {
        self.grpc_client.call_unary(o, p, self.method_updateContract.clone())
    }

    fn contract_call_method(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse> {
        self.grpc_client.call_unary(o, p, self.method_contractCallMethod.clone())
    }

    fn get_contract_info(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response> {
        self.grpc_client.call_unary(o, p, self.method_getContractInfo.clone())
    }

    fn contract_call_local_method(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response> {
        self.grpc_client.call_unary(o, p, self.method_contractCallLocalMethod.clone())
    }

    fn contract_get_bytecode(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response> {
        self.grpc_client.call_unary(o, p, self.method_ContractGetBytecode.clone())
    }

    fn get_by_solidity_id(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response> {
        self.grpc_client.call_unary(o, p, self.method_getBySolidityID.clone())
    }

    fn get_tx_record_by_contract_id(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response> {
        self.grpc_client.call_unary(o, p, self.method_getTxRecordByContractID.clone())
    }
}

// server

pub struct SmartContractServiceServer;


impl SmartContractServiceServer {
    pub fn new_service_def<H : SmartContractService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/proto.SmartContractService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.SmartContractService/createContract".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.create_contract(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.SmartContractService/updateContract".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.update_contract(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.SmartContractService/contractCallMethod".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.contract_call_method(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.SmartContractService/getContractInfo".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_contract_info(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.SmartContractService/contractCallLocalMethod".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.contract_call_local_method(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.SmartContractService/ContractGetBytecode".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.contract_get_bytecode(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.SmartContractService/getBySolidityID".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_by_solidity_id(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.SmartContractService/getTxRecordByContractID".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_tx_record_by_contract_id(o, p))
                    },
                ),
            ],
        )
    }
}
