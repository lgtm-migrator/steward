#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
    #[prost(int32, tag = "1")]
    pub upper_price: i32,
    #[prost(int32, tag = "2")]
    pub lower_price: i32,
    #[prost(uint32, tag = "3")]
    pub weight: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RebalanceRequest {
    #[prost(string, tag = "1")]
    pub cellar_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub data: ::prost::alloc::vec::Vec<Position>,
    #[prost(message, optional, tag = "3")]
    pub created_timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RebalanceResponse {}
#[doc = r" Generated client implementations."]
pub mod uniswap_v3_cellar_allocator_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct UniswapV3CellarAllocatorClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UniswapV3CellarAllocatorClient<tonic::transport::Channel> {
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
    impl<T> UniswapV3CellarAllocatorClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn rebalance(
            &mut self,
            request: impl tonic::IntoRequest<super::RebalanceRequest>,
        ) -> Result<tonic::Response<super::RebalanceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cellars.uniswapv3.v1.UniswapV3CellarAllocator/Rebalance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for UniswapV3CellarAllocatorClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for UniswapV3CellarAllocatorClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "UniswapV3CellarAllocatorClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod uniswap_v3_cellar_allocator_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with UniswapV3CellarAllocatorServer."]
    #[async_trait]
    pub trait UniswapV3CellarAllocator: Send + Sync + 'static {
        async fn rebalance(
            &self,
            request: tonic::Request<super::RebalanceRequest>,
        ) -> Result<tonic::Response<super::RebalanceResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct UniswapV3CellarAllocatorServer<T: UniswapV3CellarAllocator> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: UniswapV3CellarAllocator> UniswapV3CellarAllocatorServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for UniswapV3CellarAllocatorServer<T>
    where
        T: UniswapV3CellarAllocator,
        B: HttpBody + Send + Sync + 'static,
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
                "/cellars.uniswapv3.v1.UniswapV3CellarAllocator/Rebalance" => {
                    #[allow(non_camel_case_types)]
                    struct RebalanceSvc<T: UniswapV3CellarAllocator>(pub Arc<T>);
                    impl<T: UniswapV3CellarAllocator>
                        tonic::server::UnaryService<super::RebalanceRequest> for RebalanceSvc<T>
                    {
                        type Response = super::RebalanceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RebalanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).rebalance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RebalanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: UniswapV3CellarAllocator> Clone for UniswapV3CellarAllocatorServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: UniswapV3CellarAllocator> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: UniswapV3CellarAllocator> tonic::transport::NamedService
        for UniswapV3CellarAllocatorServer<T>
    {
        const NAME: &'static str = "cellars.uniswapv3.v1.UniswapV3CellarAllocator";
    }
}
