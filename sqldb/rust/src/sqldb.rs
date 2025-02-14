// This file is generated automatically using wasmcloud/weld-codegen and smithy model definitions
//

#![allow(unused_imports, clippy::ptr_arg, clippy::needless_lifetimes)]
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, io::Write, string::ToString};
use wasmbus_rpc::{
    deserialize, serialize, Context, Message, MessageDispatch, RpcError, RpcResult, SendOpts,
    Timestamp, Transport,
};

pub const SMITHY_VERSION: &str = "1.0";

/// Metadata about a Column in the result set
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Column {
    /// column ordinal
    pub ordinal: u32,
    /// Column name in the result
    #[serde(default)]
    pub name: String,
    /// column data type as reported by the database
    #[serde(rename = "dbType")]
    #[serde(default)]
    pub db_type: String,
}

/// List of columns in the result set returned by a Fetch operation
pub type Columns = Vec<Column>;

/// Result of an Execute operation
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExecuteResult {
    /// the number of rows affected by the query
    #[serde(rename = "rowsAffected")]
    pub rows_affected: u64,
    /// optional error information.
    /// If error is included in the FetchResult, other values should be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<SqlDbError>,
}

/// Result of a fetch query
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct FetchResult {
    /// number of rows returned
    #[serde(rename = "numRows")]
    pub num_rows: u64,
    /// description of columns returned
    pub columns: Columns,
    /// result rows, encoded in CBOR as
    /// an array (rows) of arrays (fields per row)
    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub rows: Vec<u8>,
    /// optional error information.
    /// If error is included in the FetchResult, other values should be ignored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<SqlDbError>,
}

/// A query is a non-empty string containing an SQL query or statement,
/// in the syntax of the back-end database.
pub type Query = String;

/// Detailed error information from the previous operation
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct SqlDbError {
    /// Type of error.
    /// The list of enum variants for this field may be expanded in the future
    /// to provide finer-granularity failure information
    #[serde(default)]
    pub code: String,
    /// error message
    #[serde(default)]
    pub message: String,
}

/// SqlDb - SQL Database connections
/// To use this capability, the actor must be linked
/// with the capability contract "wasmcloud:sqldb"
/// wasmbus.contractId: wasmcloud:sqldb
/// wasmbus.providerReceive
#[async_trait]
pub trait SqlDb {
    /// returns the capability contract id for this interface
    fn contract_id() -> &'static str {
        "wasmcloud:sqldb"
    }
    /// Execute an sql statement
    async fn execute(&self, ctx: &Context, arg: &Query) -> RpcResult<ExecuteResult>;
    /// Perform select query on database, returning all result rows
    async fn fetch(&self, ctx: &Context, arg: &Query) -> RpcResult<FetchResult>;
}

/// SqlDbReceiver receives messages defined in the SqlDb service trait
/// SqlDb - SQL Database connections
/// To use this capability, the actor must be linked
/// with the capability contract "wasmcloud:sqldb"
#[doc(hidden)]
#[async_trait]
pub trait SqlDbReceiver: MessageDispatch + SqlDb {
    async fn dispatch(&self, ctx: &Context, message: &Message<'_>) -> RpcResult<Message<'_>> {
        match message.method {
            "Execute" => {
                let value: Query = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = SqlDb::execute(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "SqlDb.Execute",
                    arg: Cow::Owned(buf),
                })
            }
            "Fetch" => {
                let value: Query = deserialize(message.arg.as_ref())
                    .map_err(|e| RpcError::Deser(format!("message '{}': {}", message.method, e)))?;
                let resp = SqlDb::fetch(self, ctx, &value).await?;
                let buf = serialize(&resp)?;
                Ok(Message {
                    method: "SqlDb.Fetch",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "SqlDb::{}",
                message.method
            ))),
        }
    }
}

/// SqlDbSender sends messages to a SqlDb service
/// SqlDb - SQL Database connections
/// To use this capability, the actor must be linked
/// with the capability contract "wasmcloud:sqldb"
/// client for sending SqlDb messages
#[derive(Debug)]
pub struct SqlDbSender<T: Transport> {
    transport: T,
}

impl<T: Transport> SqlDbSender<T> {
    /// Constructs a SqlDbSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(target_arch = "wasm32")]
impl SqlDbSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for sending to a SqlDb provider
    /// implementing the 'wasmcloud:sqldb' capability contract, with the "default" link
    pub fn new() -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_provider("wasmcloud:sqldb", "default")
                .unwrap();
        Self { transport }
    }

    /// Constructs a client for sending to a SqlDb provider
    /// implementing the 'wasmcloud:sqldb' capability contract, with the specified link name
    pub fn new_with_link(link_name: &str) -> wasmbus_rpc::RpcResult<Self> {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_provider("wasmcloud:sqldb", link_name)?;
        Ok(Self { transport })
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> SqlDb for SqlDbSender<T> {
    #[allow(unused)]
    /// Execute an sql statement
    async fn execute(&self, ctx: &Context, arg: &Query) -> RpcResult<ExecuteResult> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "SqlDb.Execute",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "Execute", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    /// Perform select query on database, returning all result rows
    async fn fetch(&self, ctx: &Context, arg: &Query) -> RpcResult<FetchResult> {
        let buf = serialize(arg)?;
        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "SqlDb.Fetch",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;
        let value = deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("response to {}: {}", "Fetch", e)))?;
        Ok(value)
    }
}
