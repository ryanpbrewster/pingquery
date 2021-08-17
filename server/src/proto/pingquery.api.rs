#[derive(::serde::Serialize, ::serde::Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeRequest {
}
#[derive(::serde::Serialize, ::serde::Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeResponse {
}
#[derive(::serde::Serialize, ::serde::Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiagnosticsRequest {
}
#[derive(::serde::Serialize, ::serde::Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiagnosticsResponse {
    #[prost(int32, tag="1")]
    pub num_connected_clients: i32,
    #[prost(message, repeated, tag="2")]
    pub queries: ::prost::alloc::vec::Vec<QueryDiagnostics>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDiagnostics {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub num_executions: i64,
}
#[derive(::serde::Serialize, ::serde::Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConfigRequest {
}
#[derive(::serde::Serialize, ::serde::Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConfigResponse {
    #[prost(message, optional, tag="1")]
    pub config: ::core::option::Option<Config>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetConfigRequest {
    #[prost(message, optional, tag="1")]
    pub config: ::core::option::Option<Config>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetConfigResponse {
}
#[derive(::serde::Serialize, ::serde::Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecRequest {
    #[prost(string, tag="1")]
    pub raw_sql: ::prost::alloc::string::String,
}
#[derive(::serde::Serialize, ::serde::Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecResponse {
    #[prost(message, repeated, tag="1")]
    pub rows: ::prost::alloc::vec::Vec<Row>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractRequest {
    //// An identifier that the server will echo back with any response related to this request.
    //// Must be monotonically increasing.
    #[prost(int32, tag="1")]
    pub id: i32,
    #[prost(oneof="interact_request::Type", tags="2, 3, 4")]
    pub r#type: ::core::option::Option<interact_request::Type>,
}
/// Nested message and enum types in `InteractRequest`.
pub mod interact_request {
    #[derive(::serde::Serialize, ::serde::Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag="2")]
        Mutate(super::Statement),
        #[prost(message, tag="3")]
        Query(super::Statement),
        #[prost(message, tag="4")]
        Listen(super::Statement),
    }
}
#[derive(::serde::Serialize, ::serde::Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractResponse {
    //// The identifier of the request that generated this response.
    #[prost(int32, tag="1")]
    pub id: i32,
    #[prost(message, repeated, tag="2")]
    pub rows: ::prost::alloc::vec::Vec<Row>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    #[prost(message, repeated, tag="1")]
    pub queries: ::prost::alloc::vec::Vec<QueryConfig>,
    #[prost(message, repeated, tag="2")]
    pub mutates: ::prost::alloc::vec::Vec<MutateConfig>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConfig {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub sql_template: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub listen: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateConfig {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub sql_template: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub notify: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Statement {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Row>,
}
#[derive(::serde::Serialize, ::serde::Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(oneof="value::Type", tags="1, 2")]
    pub r#type: ::core::option::Option<value::Type>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    #[derive(::serde::Serialize, ::serde::Deserialize)] #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(int64, tag="1")]
        Integer(i64),
        #[prost(string, tag="2")]
        Text(::prost::alloc::string::String),
    }
}
#[derive(::serde::Serialize, ::serde::Deserialize)] #[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Row {
    #[prost(map="string, message", tag="1")]
    pub columns: ::std::collections::HashMap<::prost::alloc::string::String, Value>,
}
