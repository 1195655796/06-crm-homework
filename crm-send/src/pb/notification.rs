// This file is @generated by prost-build.
/// email message to be sent
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmailMessage {
    /// unique identifier of the message
    #[prost(string, tag = "1")]
    pub message_id: ::prost::alloc::string::String,
    /// subject of the email
    #[prost(string, tag = "2")]
    pub subject: ::prost::alloc::string::String,
    /// sender of the email
    #[prost(string, tag = "3")]
    pub sender: ::prost::alloc::string::String,
    /// recipients of the email
    #[prost(string, repeated, tag = "4")]
    pub recipients: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// body of the email
    #[prost(string, tag = "5")]
    pub body: ::prost::alloc::string::String,
}
/// sms message to be sent
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmsMessage {
    /// unique identifier of the message
    #[prost(string, tag = "1")]
    pub message_id: ::prost::alloc::string::String,
    /// sender of the sms
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    /// recipients of the sms
    #[prost(string, repeated, tag = "3")]
    pub recipients: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// body of the sms
    #[prost(string, tag = "4")]
    pub body: ::prost::alloc::string::String,
}
/// in-app message to be sent
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InAppMessage {
    /// unique identifier of the message
    #[prost(string, tag = "1")]
    pub message_id: ::prost::alloc::string::String,
    /// device id of the device to send the in-app message
    #[prost(string, tag = "2")]
    pub device_id: ::prost::alloc::string::String,
    /// title of the in-app message
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    /// body of the in-app message
    #[prost(string, tag = "4")]
    pub body: ::prost::alloc::string::String,
}
/// request to send a message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendRequest {
    /// one of the message types to send
    #[prost(oneof = "send_request::Msg", tags = "2, 3, 4")]
    pub msg: ::core::option::Option<send_request::Msg>,
}
/// Nested message and enum types in `SendRequest`.
pub mod send_request {
    /// one of the message types to send
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        #[prost(message, tag = "2")]
        Email(super::EmailMessage),
        #[prost(message, tag = "3")]
        Sms(super::SmsMessage),
        #[prost(message, tag = "4")]
        InApp(super::InAppMessage),
    }
}
/// response to a send request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendResponse {
    /// unique identifier of the message
    #[prost(string, tag = "1")]
    pub message_id: ::prost::alloc::string::String,
    /// timestamp of when the message was sent
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
/// Generated client implementations.
pub mod notification_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The Notification service provides a way to send notifications to users.
    #[derive(Debug, Clone)]
    pub struct NotificationClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl NotificationClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> NotificationClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> NotificationClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            NotificationClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Send a notification to a user.
        pub async fn send(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::SendRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SendResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/notification.Notification/Send",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("notification.Notification", "Send"));
            self.inner.streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod notification_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with NotificationServer.
    #[async_trait]
    pub trait Notification: Send + Sync + 'static {
        /// Server streaming response type for the Send method.
        type SendStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::SendResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Send a notification to a user.
        async fn send(
            &self,
            request: tonic::Request<tonic::Streaming<super::SendRequest>>,
        ) -> std::result::Result<tonic::Response<Self::SendStream>, tonic::Status>;
    }
    /// The Notification service provides a way to send notifications to users.
    #[derive(Debug)]
    pub struct NotificationServer<T: Notification> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Notification> NotificationServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for NotificationServer<T>
    where
        T: Notification,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/notification.Notification/Send" => {
                    #[allow(non_camel_case_types)]
                    struct SendSvc<T: Notification>(pub Arc<T>);
                    impl<
                        T: Notification,
                    > tonic::server::StreamingService<super::SendRequest>
                    for SendSvc<T> {
                        type Response = super::SendResponse;
                        type ResponseStream = T::SendStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::SendRequest>>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Notification>::send(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Notification> Clone for NotificationServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Notification> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Notification> tonic::server::NamedService for NotificationServer<T> {
        const NAME: &'static str = "notification.Notification";
    }
}
