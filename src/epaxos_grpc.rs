// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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

pub trait Internal {
    fn pre_accept(&self, o: ::grpc::RequestOptions, p: super::epaxos::Payload) -> ::grpc::SingleResponse<super::epaxos::Payload>;

    fn accept(&self, o: ::grpc::RequestOptions, p: super::epaxos::Payload) -> ::grpc::SingleResponse<super::epaxos::AcceptOKPayload>;

    fn commit(&self, o: ::grpc::RequestOptions, p: super::epaxos::Payload) -> ::grpc::SingleResponse<super::epaxos::Empty>;
}

// client

pub struct InternalClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_PreAccept: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::epaxos::Payload, super::epaxos::Payload>>,
    method_Accept: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::epaxos::Payload, super::epaxos::AcceptOKPayload>>,
    method_Commit: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::epaxos::Payload, super::epaxos::Empty>>,
}

impl ::grpc::ClientStub for InternalClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        InternalClient {
            grpc_client: grpc_client,
            method_PreAccept: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/epaxos.Internal/PreAccept".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Accept: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/epaxos.Internal/Accept".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Commit: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/epaxos.Internal/Commit".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl Internal for InternalClient {
    fn pre_accept(&self, o: ::grpc::RequestOptions, p: super::epaxos::Payload) -> ::grpc::SingleResponse<super::epaxos::Payload> {
        self.grpc_client.call_unary(o, p, self.method_PreAccept.clone())
    }

    fn accept(&self, o: ::grpc::RequestOptions, p: super::epaxos::Payload) -> ::grpc::SingleResponse<super::epaxos::AcceptOKPayload> {
        self.grpc_client.call_unary(o, p, self.method_Accept.clone())
    }

    fn commit(&self, o: ::grpc::RequestOptions, p: super::epaxos::Payload) -> ::grpc::SingleResponse<super::epaxos::Empty> {
        self.grpc_client.call_unary(o, p, self.method_Commit.clone())
    }
}

// server

pub struct InternalServer;


impl InternalServer {
    pub fn new_service_def<H : Internal + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/epaxos.Internal",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/epaxos.Internal/PreAccept".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.pre_accept(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/epaxos.Internal/Accept".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.accept(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/epaxos.Internal/Commit".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.commit(o, p))
                    },
                ),
            ],
        )
    }
}

// interface

pub trait External {
    fn write(&self, o: ::grpc::RequestOptions, p: super::epaxos::WriteRequest) -> ::grpc::SingleResponse<super::epaxos::WriteResponse>;

    fn read(&self, o: ::grpc::RequestOptions, p: super::epaxos::ReadRequest) -> ::grpc::SingleResponse<super::epaxos::ReadResponse>;
}

// client

pub struct ExternalClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_Write: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::epaxos::WriteRequest, super::epaxos::WriteResponse>>,
    method_Read: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::epaxos::ReadRequest, super::epaxos::ReadResponse>>,
}

impl ::grpc::ClientStub for ExternalClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        ExternalClient {
            grpc_client: grpc_client,
            method_Write: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/epaxos.External/Write".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Read: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/epaxos.External/Read".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl External for ExternalClient {
    fn write(&self, o: ::grpc::RequestOptions, p: super::epaxos::WriteRequest) -> ::grpc::SingleResponse<super::epaxos::WriteResponse> {
        self.grpc_client.call_unary(o, p, self.method_Write.clone())
    }

    fn read(&self, o: ::grpc::RequestOptions, p: super::epaxos::ReadRequest) -> ::grpc::SingleResponse<super::epaxos::ReadResponse> {
        self.grpc_client.call_unary(o, p, self.method_Read.clone())
    }
}

// server

pub struct ExternalServer;


impl ExternalServer {
    pub fn new_service_def<H : External + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/epaxos.External",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/epaxos.External/Write".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.write(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/epaxos.External/Read".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.read(o, p))
                    },
                ),
            ],
        )
    }
}
