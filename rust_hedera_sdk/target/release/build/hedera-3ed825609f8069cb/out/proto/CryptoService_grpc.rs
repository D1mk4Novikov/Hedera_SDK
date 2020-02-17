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

pub trait CryptoService {
    fn create_account(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse>;

    fn update_account(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse>;

    fn crypto_transfer(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse>;

    fn crypto_delete(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse>;

    fn add_claim(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse>;

    fn delete_claim(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse>;

    fn get_claim(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response>;

    fn get_account_records(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response>;

    fn crypto_get_balance(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response>;

    fn get_account_info(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response>;

    fn get_transaction_receipts(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response>;

    fn get_fast_transaction_record(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response>;

    fn get_tx_record_by_tx_id(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response>;

    fn get_stakers_by_account_id(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response>;
}

// client

pub struct CryptoServiceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_createAccount: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Transaction::Transaction, super::TransactionResponse::TransactionResponse>>,
    method_updateAccount: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Transaction::Transaction, super::TransactionResponse::TransactionResponse>>,
    method_cryptoTransfer: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Transaction::Transaction, super::TransactionResponse::TransactionResponse>>,
    method_cryptoDelete: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Transaction::Transaction, super::TransactionResponse::TransactionResponse>>,
    method_addClaim: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Transaction::Transaction, super::TransactionResponse::TransactionResponse>>,
    method_deleteClaim: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Transaction::Transaction, super::TransactionResponse::TransactionResponse>>,
    method_getClaim: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Query::Query, super::Response::Response>>,
    method_getAccountRecords: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Query::Query, super::Response::Response>>,
    method_cryptoGetBalance: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Query::Query, super::Response::Response>>,
    method_getAccountInfo: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Query::Query, super::Response::Response>>,
    method_getTransactionReceipts: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Query::Query, super::Response::Response>>,
    method_getFastTransactionRecord: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Query::Query, super::Response::Response>>,
    method_getTxRecordByTxID: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Query::Query, super::Response::Response>>,
    method_getStakersByAccountID: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Query::Query, super::Response::Response>>,
}

impl ::grpc::ClientStub for CryptoServiceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        CryptoServiceClient {
            grpc_client: grpc_client,
            method_createAccount: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.CryptoService/createAccount".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_updateAccount: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.CryptoService/updateAccount".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_cryptoTransfer: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.CryptoService/cryptoTransfer".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_cryptoDelete: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.CryptoService/cryptoDelete".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_addClaim: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.CryptoService/addClaim".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_deleteClaim: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.CryptoService/deleteClaim".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_getClaim: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.CryptoService/getClaim".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_getAccountRecords: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.CryptoService/getAccountRecords".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_cryptoGetBalance: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.CryptoService/cryptoGetBalance".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_getAccountInfo: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.CryptoService/getAccountInfo".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_getTransactionReceipts: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.CryptoService/getTransactionReceipts".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_getFastTransactionRecord: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.CryptoService/getFastTransactionRecord".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_getTxRecordByTxID: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.CryptoService/getTxRecordByTxID".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_getStakersByAccountID: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.CryptoService/getStakersByAccountID".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl CryptoService for CryptoServiceClient {
    fn create_account(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse> {
        self.grpc_client.call_unary(o, p, self.method_createAccount.clone())
    }

    fn update_account(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse> {
        self.grpc_client.call_unary(o, p, self.method_updateAccount.clone())
    }

    fn crypto_transfer(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse> {
        self.grpc_client.call_unary(o, p, self.method_cryptoTransfer.clone())
    }

    fn crypto_delete(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse> {
        self.grpc_client.call_unary(o, p, self.method_cryptoDelete.clone())
    }

    fn add_claim(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse> {
        self.grpc_client.call_unary(o, p, self.method_addClaim.clone())
    }

    fn delete_claim(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse> {
        self.grpc_client.call_unary(o, p, self.method_deleteClaim.clone())
    }

    fn get_claim(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response> {
        self.grpc_client.call_unary(o, p, self.method_getClaim.clone())
    }

    fn get_account_records(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response> {
        self.grpc_client.call_unary(o, p, self.method_getAccountRecords.clone())
    }

    fn crypto_get_balance(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response> {
        self.grpc_client.call_unary(o, p, self.method_cryptoGetBalance.clone())
    }

    fn get_account_info(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response> {
        self.grpc_client.call_unary(o, p, self.method_getAccountInfo.clone())
    }

    fn get_transaction_receipts(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response> {
        self.grpc_client.call_unary(o, p, self.method_getTransactionReceipts.clone())
    }

    fn get_fast_transaction_record(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response> {
        self.grpc_client.call_unary(o, p, self.method_getFastTransactionRecord.clone())
    }

    fn get_tx_record_by_tx_id(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response> {
        self.grpc_client.call_unary(o, p, self.method_getTxRecordByTxID.clone())
    }

    fn get_stakers_by_account_id(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response> {
        self.grpc_client.call_unary(o, p, self.method_getStakersByAccountID.clone())
    }
}

// server

pub struct CryptoServiceServer;


impl CryptoServiceServer {
    pub fn new_service_def<H : CryptoService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/proto.CryptoService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.CryptoService/createAccount".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.create_account(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.CryptoService/updateAccount".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.update_account(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.CryptoService/cryptoTransfer".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.crypto_transfer(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.CryptoService/cryptoDelete".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.crypto_delete(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.CryptoService/addClaim".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.add_claim(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.CryptoService/deleteClaim".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.delete_claim(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.CryptoService/getClaim".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_claim(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.CryptoService/getAccountRecords".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_account_records(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.CryptoService/cryptoGetBalance".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.crypto_get_balance(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.CryptoService/getAccountInfo".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_account_info(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.CryptoService/getTransactionReceipts".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_transaction_receipts(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.CryptoService/getFastTransactionRecord".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_fast_transaction_record(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.CryptoService/getTxRecordByTxID".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_tx_record_by_tx_id(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.CryptoService/getStakersByAccountID".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_stakers_by_account_id(o, p))
                    },
                ),
            ],
        )
    }
}
