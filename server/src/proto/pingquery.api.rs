#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeRequest {}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeResponse {}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiagnosticsRequest {}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiagnosticsResponse {
    #[prost(int32, tag = "1")]
    #[serde(default)]
    pub num_connected_clients: i32,
    #[prost(message, repeated, tag = "2")]
    #[serde(default)]
    pub queries: ::prost::alloc::vec::Vec<QueryDiagnostics>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDiagnostics {
    #[prost(string, tag = "1")]
    #[serde(default)]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    #[serde(default)]
    pub num_executions: i64,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConfigRequest {}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConfigResponse {
    #[prost(message, optional, tag = "1")]
    #[serde(default)]
    pub config: ::core::option::Option<Config>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetConfigRequest {
    #[prost(message, optional, tag = "1")]
    #[serde(default)]
    pub config: ::core::option::Option<Config>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetConfigResponse {}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecRequest {
    #[prost(string, tag = "1")]
    #[serde(default)]
    pub raw_sql: ::prost::alloc::string::String,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecResponse {
    #[prost(message, repeated, tag = "1")]
    #[serde(default)]
    pub rows: ::prost::alloc::vec::Vec<Row>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractRequest {
    //// An identifier that the server will echo back with any response related to this request.
    //// Must be monotonically increasing.
    #[prost(int32, tag = "1")]
    #[serde(default)]
    pub id: i32,
    #[prost(message, optional, tag = "2")]
    #[serde(default)]
    pub mutate: ::core::option::Option<Statement>,
    #[prost(message, optional, tag = "3")]
    #[serde(default)]
    pub query: ::core::option::Option<Statement>,
    #[prost(message, optional, tag = "4")]
    #[serde(default)]
    pub listen: ::core::option::Option<Statement>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractResponse {
    //// The identifier of the request that generated this response.
    #[prost(int32, tag = "1")]
    #[serde(default)]
    pub id: i32,
    #[prost(message, repeated, tag = "2")]
    #[serde(default)]
    pub rows: ::prost::alloc::vec::Vec<Row>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    #[prost(message, repeated, tag = "1")]
    #[serde(default)]
    pub queries: ::prost::alloc::vec::Vec<QueryConfig>,
    #[prost(message, repeated, tag = "2")]
    #[serde(default)]
    pub mutates: ::prost::alloc::vec::Vec<MutateConfig>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConfig {
    #[prost(string, tag = "1")]
    #[serde(default)]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(default)]
    pub sql_template: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    #[serde(default)]
    pub listen: ::prost::alloc::vec::Vec<Path>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateConfig {
    #[prost(string, tag = "1")]
    #[serde(default)]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(default)]
    pub sql_template: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    #[serde(default)]
    pub notify: ::prost::alloc::vec::Vec<Path>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Statement {
    #[prost(string, tag = "1")]
    #[serde(default)]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    #[serde(default)]
    pub params: ::core::option::Option<Row>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(int64, tag = "1")]
    #[serde(default)]
    pub integer: i64,
    #[prost(string, tag = "2")]
    #[serde(default)]
    pub text: ::prost::alloc::string::String,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Row {
    #[prost(map = "string, message", tag = "1")]
    #[serde(default)]
    pub columns: ::std::collections::HashMap<::prost::alloc::string::String, Value>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Path {
    #[prost(string, repeated, tag = "1")]
    #[serde(default)]
    pub segments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
