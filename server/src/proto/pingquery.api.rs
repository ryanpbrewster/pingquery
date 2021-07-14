#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConfigRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConfigResponse {
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<Config>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetConfigRequest {
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<Config>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetConfigResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecRequest {
    #[prost(string, tag = "1")]
    pub raw_sql: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecResponse {
    #[prost(message, repeated, tag = "1")]
    pub rows: ::prost::alloc::vec::Vec<Row>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractRequest {
    //// An identifier that the server will echo back with any response related to this request.
    //// Must be monotonically increasing.
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(oneof = "interact_request::Type", tags = "2, 3, 4")]
    pub r#type: ::core::option::Option<interact_request::Type>,
}
/// Nested message and enum types in `InteractRequest`.
pub mod interact_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag = "2")]
        Mutate(super::Statement),
        #[prost(message, tag = "3")]
        Query(super::Statement),
        #[prost(message, tag = "4")]
        Listen(super::Statement),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractResponse {
    //// The identifier of the request that generated this response.
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(message, repeated, tag = "2")]
    pub rows: ::prost::alloc::vec::Vec<Row>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    #[prost(message, repeated, tag = "1")]
    pub queries: ::prost::alloc::vec::Vec<QueryConfig>,
    #[prost(message, repeated, tag = "2")]
    pub mutates: ::prost::alloc::vec::Vec<MutateConfig>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConfig {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub sql_template: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub listen: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateConfig {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub sql_template: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub notify: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Statement {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Row>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(oneof = "value::Type", tags = "1, 2")]
    pub r#type: ::core::option::Option<value::Type>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(int64, tag = "1")]
        Integer(i64),
        #[prost(string, tag = "2")]
        Text(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Row {
    #[prost(map = "string, message", tag = "1")]
    pub columns: ::std::collections::HashMap<::prost::alloc::string::String, Value>,
}
#[doc = r" Generated client implementations."]
pub mod ping_query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct PingQueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PingQueryClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PingQueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PingQueryClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            PingQueryClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn initialize(
            &mut self,
            request: impl tonic::IntoRequest<super::InitializeRequest>,
        ) -> Result<tonic::Response<super::InitializeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pingquery.api.PingQuery/Initialize");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConfigRequest>,
        ) -> Result<tonic::Response<super::GetConfigResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pingquery.api.PingQuery/GetConfig");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set_config(
            &mut self,
            request: impl tonic::IntoRequest<super::SetConfigRequest>,
        ) -> Result<tonic::Response<super::SetConfigResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pingquery.api.PingQuery/SetConfig");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn exec(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecRequest>,
        ) -> Result<tonic::Response<super::ExecResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pingquery.api.PingQuery/Exec");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn interact(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::InteractRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::InteractResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pingquery.api.PingQuery/Interact");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod ping_query_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with PingQueryServer."]
    #[async_trait]
    pub trait PingQuery: Send + Sync + 'static {
        async fn initialize(
            &self,
            request: tonic::Request<super::InitializeRequest>,
        ) -> Result<tonic::Response<super::InitializeResponse>, tonic::Status>;
        async fn get_config(
            &self,
            request: tonic::Request<super::GetConfigRequest>,
        ) -> Result<tonic::Response<super::GetConfigResponse>, tonic::Status>;
        async fn set_config(
            &self,
            request: tonic::Request<super::SetConfigRequest>,
        ) -> Result<tonic::Response<super::SetConfigResponse>, tonic::Status>;
        async fn exec(
            &self,
            request: tonic::Request<super::ExecRequest>,
        ) -> Result<tonic::Response<super::ExecResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the Interact method."]
        type InteractStream: futures_core::Stream<Item = Result<super::InteractResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn interact(
            &self,
            request: tonic::Request<tonic::Streaming<super::InteractRequest>>,
        ) -> Result<tonic::Response<Self::InteractStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct PingQueryServer<T: PingQuery> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PingQuery> PingQueryServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PingQueryServer<T>
    where
        T: PingQuery,
        B: Body + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/pingquery.api.PingQuery/Initialize" => {
                    #[allow(non_camel_case_types)]
                    struct InitializeSvc<T: PingQuery>(pub Arc<T>);
                    impl<T: PingQuery> tonic::server::UnaryService<super::InitializeRequest> for InitializeSvc<T> {
                        type Response = super::InitializeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InitializeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).initialize(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = InitializeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pingquery.api.PingQuery/GetConfig" => {
                    #[allow(non_camel_case_types)]
                    struct GetConfigSvc<T: PingQuery>(pub Arc<T>);
                    impl<T: PingQuery> tonic::server::UnaryService<super::GetConfigRequest> for GetConfigSvc<T> {
                        type Response = super::GetConfigResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetConfigSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pingquery.api.PingQuery/SetConfig" => {
                    #[allow(non_camel_case_types)]
                    struct SetConfigSvc<T: PingQuery>(pub Arc<T>);
                    impl<T: PingQuery> tonic::server::UnaryService<super::SetConfigRequest> for SetConfigSvc<T> {
                        type Response = super::SetConfigResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetConfigSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pingquery.api.PingQuery/Exec" => {
                    #[allow(non_camel_case_types)]
                    struct ExecSvc<T: PingQuery>(pub Arc<T>);
                    impl<T: PingQuery> tonic::server::UnaryService<super::ExecRequest> for ExecSvc<T> {
                        type Response = super::ExecResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).exec(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExecSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/pingquery.api.PingQuery/Interact" => {
                    #[allow(non_camel_case_types)]
                    struct InteractSvc<T: PingQuery>(pub Arc<T>);
                    impl<T: PingQuery> tonic::server::StreamingService<super::InteractRequest> for InteractSvc<T> {
                        type Response = super::InteractResponse;
                        type ResponseStream = T::InteractStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::InteractRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).interact(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = InteractSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: PingQuery> Clone for PingQueryServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: PingQuery> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PingQuery> tonic::transport::NamedService for PingQueryServer<T> {
        const NAME: &'static str = "pingquery.api.PingQuery";
    }
}
