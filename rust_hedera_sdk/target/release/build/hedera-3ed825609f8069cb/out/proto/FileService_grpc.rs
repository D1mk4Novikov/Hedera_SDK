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

pub trait FileService {
    fn create_file(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse>;

    fn update_file(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse>;

    fn delete_file(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse>;

    fn append_content(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse>;

    fn get_file_content(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response>;

    fn get_file_info(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response>;

    fn admin_delete(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse>;

    fn admin_undelete(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse>;
}

// client

pub struct FileServiceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_createFile: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Transaction::Transaction, super::TransactionResponse::TransactionResponse>>,
    method_updateFile: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Transaction::Transaction, super::TransactionResponse::TransactionResponse>>,
    method_deleteFile: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Transaction::Transaction, super::TransactionResponse::TransactionResponse>>,
    method_appendContent: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Transaction::Transaction, super::TransactionResponse::TransactionResponse>>,
    method_getFileContent: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Query::Query, super::Response::Response>>,
    method_getFileInfo: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Query::Query, super::Response::Response>>,
    method_adminDelete: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Transaction::Transaction, super::TransactionResponse::TransactionResponse>>,
    method_adminUndelete: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::Transaction::Transaction, super::TransactionResponse::TransactionResponse>>,
}

impl ::grpc::ClientStub for FileServiceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        FileServiceClient {
            grpc_client: grpc_client,
            method_createFile: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.FileService/createFile".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_updateFile: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.FileService/updateFile".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_deleteFile: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.FileService/deleteFile".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_appendContent: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.FileService/appendContent".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_getFileContent: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.FileService/getFileContent".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_getFileInfo: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.FileService/getFileInfo".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_adminDelete: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.FileService/adminDelete".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_adminUndelete: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.FileService/adminUndelete".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl FileService for FileServiceClient {
    fn create_file(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse> {
        self.grpc_client.call_unary(o, p, self.method_createFile.clone())
    }

    fn update_file(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse> {
        self.grpc_client.call_unary(o, p, self.method_updateFile.clone())
    }

    fn delete_file(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse> {
        self.grpc_client.call_unary(o, p, self.method_deleteFile.clone())
    }

    fn append_content(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse> {
        self.grpc_client.call_unary(o, p, self.method_appendContent.clone())
    }

    fn get_file_content(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response> {
        self.grpc_client.call_unary(o, p, self.method_getFileContent.clone())
    }

    fn get_file_info(&self, o: ::grpc::RequestOptions, p: super::Query::Query) -> ::grpc::SingleResponse<super::Response::Response> {
        self.grpc_client.call_unary(o, p, self.method_getFileInfo.clone())
    }

    fn admin_delete(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse> {
        self.grpc_client.call_unary(o, p, self.method_adminDelete.clone())
    }

    fn admin_undelete(&self, o: ::grpc::RequestOptions, p: super::Transaction::Transaction) -> ::grpc::SingleResponse<super::TransactionResponse::TransactionResponse> {
        self.grpc_client.call_unary(o, p, self.method_adminUndelete.clone())
    }
}

// server

pub struct FileServiceServer;


impl FileServiceServer {
    pub fn new_service_def<H : FileService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/proto.FileService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.FileService/createFile".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.create_file(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.FileService/updateFile".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.update_file(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.FileService/deleteFile".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.delete_file(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.FileService/appendContent".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.append_content(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.FileService/getFileContent".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_file_content(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.FileService/getFileInfo".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_file_info(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.FileService/adminDelete".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.admin_delete(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.FileService/adminUndelete".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.admin_undelete(o, p))
                    },
                ),
            ],
        )
    }
}
