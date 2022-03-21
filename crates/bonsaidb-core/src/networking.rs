use arc_bytes::serde::Bytes;
use schema::SchemaName;
use serde::{Deserialize, Serialize};

use crate::{
    api::Api,
    connection::{AccessPolicy, Database, Identity, QueryKey, Range, Session, SessionId, Sort},
    document::{DocumentId, OwnedDocument},
    keyvalue::{KeyOperation, Output},
    schema::{
        self,
        view::map::{self, MappedSerializedDocuments},
        CollectionName, Name, NamedReference, ViewName,
    },
    transaction::{Executed, OperationResult, Transaction},
};

/// The current protocol version.
pub const CURRENT_PROTOCOL_VERSION: &str = "bonsai/pre/0";

/// A payload with an associated id.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Payload {
    /// The authentication session id for this payload.
    pub session_id: Option<SessionId>,
    /// The unique id for this payload.
    pub id: Option<u32>,
    /// The unique name of the api
    pub name: Name,
    /// The payload
    pub value: Result<Bytes, crate::Error>,
}

/// Creates a database.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateDatabase {
    /// The database to create.
    pub database: Database,
    /// Only attempts to create the database if it doesn't already exist.
    pub only_if_needed: bool,
}

impl Api for CreateDatabase {
    type Response = ();
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("CreateDatabase")
    }
}

/// Deletes the database named `name`
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct DeleteDatabase {
    /// The name of the database to delete.
    pub name: String,
}

impl Api for DeleteDatabase {
    type Response = ();
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("DeleteDatabase")
    }
}

/// Lists all databases.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ListDatabases;

impl Api for ListDatabases {
    type Response = Vec<Database>;
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("ListDatabases")
    }
}

/// Lists available schemas.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ListAvailableSchemas;

impl Api for ListAvailableSchemas {
    type Response = Vec<SchemaName>;
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("ListAvailableSchemas")
    }
}

/// Creates a user.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateUser {
    /// The unique username of the user to create.
    pub username: String,
}

impl Api for CreateUser {
    type Response = u64;
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("CreateUser")
    }
}

/// Deletes a user.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct DeleteUser {
    /// The unique primary key of the user to be deleted.
    pub user: NamedReference<'static, u64>,
}

impl Api for DeleteUser {
    type Response = ();
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("DeleteUser")
    }
}

/// Set's a user's password.
#[cfg(feature = "password-hashing")]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct SetUserPassword {
    /// The username or id of the user.
    pub user: NamedReference<'static, u64>,
    /// The user's new password.
    pub password: crate::connection::SensitiveString,
}

#[cfg(feature = "password-hashing")]
impl Api for SetUserPassword {
    type Response = ();
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("SetUserPassword")
    }
}

/// Authenticate as a user.
#[cfg(feature = "password-hashing")]
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Authenticate {
    /// The username or id of the user.
    pub user: NamedReference<'static, u64>,
    /// The method of authentication.
    pub authentication: crate::connection::Authentication,
}

#[cfg(feature = "password-hashing")]
impl Api for Authenticate {
    type Response = Session;
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("Authenticate")
    }
}

/// Assume an identity.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct AssumeIdentity(pub Identity);

impl Api for AssumeIdentity {
    type Response = Session;
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("AssumeIdentity")
    }
}

/// Alter's a user's membership in a permission group.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct AlterUserPermissionGroupMembership {
    /// The username or id of the user.
    pub user: NamedReference<'static, u64>,

    /// The name or id of the group.
    pub group: NamedReference<'static, u64>,

    /// Whether the user should be in the group.
    pub should_be_member: bool,
}

impl Api for AlterUserPermissionGroupMembership {
    type Response = ();
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("AlterUserPermissionGroupMembership")
    }
}

/// Alter's a user's role
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct AlterUserRoleMembership {
    /// The username or id of the user.
    pub user: NamedReference<'static, u64>,

    /// The name or id of the role.
    pub role: NamedReference<'static, u64>,

    /// Whether the user should have the role.
    pub should_be_member: bool,
}

impl Api for AlterUserRoleMembership {
    type Response = ();
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("AlterUserRoleMembership")
    }
}

/// Retrieve a single document.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Get {
    /// The name of the database.
    pub database: String,
    /// The collection of the document.
    pub collection: CollectionName,
    /// The id of the document.
    pub id: DocumentId,
}

impl Api for Get {
    type Response = Option<OwnedDocument>;
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("Get")
    }
}

/// Retrieve multiple documents.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct GetMultiple {
    /// The name of the database.
    pub database: String,
    /// The collection of the documents.
    pub collection: CollectionName,
    /// The ids of the documents.
    pub ids: Vec<DocumentId>,
}

impl Api for GetMultiple {
    type Response = Vec<OwnedDocument>;
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("GetMultiple")
    }
}

/// Retrieve multiple documents.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct List {
    /// The name of the database.
    pub database: String,
    /// The collection of the documents.
    pub collection: CollectionName,
    /// The range of ids to list.
    pub ids: Range<DocumentId>,
    /// The order for the query into the collection.
    pub order: Sort,
    /// The maximum number of results to return.
    pub limit: Option<u32>,
}

impl Api for List {
    type Response = Vec<OwnedDocument>;
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("List")
    }
}

/// Counts the number of documents in the specified range.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Count {
    /// The name of the database.
    pub database: String,
    /// The collection of the documents.
    pub collection: CollectionName,
    /// The range of ids to count.
    pub ids: Range<DocumentId>,
}

impl Api for Count {
    type Response = u64;
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("Count")
    }
}

/// Queries a view.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Query {
    /// The name of the database.
    pub database: String,
    /// The name of the view.
    pub view: ViewName,
    /// The filter for the view.
    pub key: Option<QueryKey<Bytes>>,
    /// The order for the query into the view.
    pub order: Sort,
    /// The maximum number of results to return.
    pub limit: Option<u32>,
    /// The access policy for the query.
    pub access_policy: AccessPolicy,
}

impl Api for Query {
    type Response = Vec<map::Serialized>;
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("Query")
    }
}

/// Queries a view with the associated documents.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct QueryWithDocs(pub Query);

impl Api for QueryWithDocs {
    type Response = MappedSerializedDocuments;
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("QueryWithDocs")
    }
}

/// Reduces a view.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Reduce {
    /// The name of the database.
    pub database: String,
    /// The name of the view.
    pub view: ViewName,
    /// The filter for the view.
    pub key: Option<QueryKey<Bytes>>,
    /// The access policy for the query.
    pub access_policy: AccessPolicy,
}

impl Api for Reduce {
    type Response = Bytes;
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("Reduce")
    }
}

/// Reduces a view, grouping the reduced values by key.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ReduceGrouped(pub Reduce);

impl Api for ReduceGrouped {
    type Response = Vec<map::MappedSerializedValue>;
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("ReduceGrouped")
    }
}

/// Deletes the associated documents resulting from the view query.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct DeleteDocs {
    /// The name of the database.
    pub database: String,
    /// The name of the view.
    pub view: ViewName,
    /// The filter for the view.
    pub key: Option<QueryKey<Bytes>>,
    /// The access policy for the query.
    pub access_policy: AccessPolicy,
}

impl Api for DeleteDocs {
    type Response = u64;
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("DeleteDocs")
    }
}

/// Applies a transaction.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ApplyTransaction {
    /// The name of the database.
    pub database: String,
    /// The trasnaction to apply.
    pub transaction: Transaction,
}

impl Api for ApplyTransaction {
    type Response = Vec<OperationResult>;
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("ApplyTransaction")
    }
}

/// Lists executed transactions.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ListExecutedTransactions {
    /// The name of the database.
    pub database: String,
    /// The starting transaction id.
    pub starting_id: Option<u64>,
    /// The maximum number of results.
    pub result_limit: Option<u32>,
}

impl Api for ListExecutedTransactions {
    type Response = Vec<Executed>;
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("ListExecutedTransactions")
    }
}

/// Queries the last transaction id.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct LastTransactionId {
    /// The name of the database.
    pub database: String,
}

impl Api for LastTransactionId {
    type Response = Option<u64>;
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("LastTransactionId")
    }
}

/// Creates a `PubSub` [`Subscriber`](crate::pubsub::Subscriber)
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateSubscriber {
    /// The name of the database.
    pub database: String,
}

impl Api for CreateSubscriber {
    type Response = u64;
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("CreateSubscriber")
    }
}

/// Publishes `payload` to all subscribers of `topic`.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Publish {
    /// The name of the database.
    pub database: String,
    /// The topics to publish to.
    pub topic: String,
    /// The payload to publish.
    pub payload: Bytes,
}

impl Api for Publish {
    type Response = ();
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("Publish")
    }
}

/// Publishes `payload` to all subscribers of all `topics`.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct PublishToAll {
    /// The name of the database.
    pub database: String,
    /// The topics to publish to.
    pub topics: Vec<String>,
    /// The payload to publish.
    pub payload: Bytes,
}

impl Api for PublishToAll {
    type Response = ();
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("PublishToAll")
    }
}

/// Subscribes `subscriber_id` to messages for `topic`.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct SubscribeTo {
    /// The name of the database.
    pub database: String,
    /// The id of the [`Subscriber`](crate::pubsub::Subscriber).
    pub subscriber_id: u64,
    /// The topic to subscribe to.
    pub topic: String,
}

impl Api for SubscribeTo {
    type Response = ();
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("SubscribeTo")
    }
}

/// A PubSub message was received.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct MessageReceived {
    /// The ID of the subscriber receiving the message.
    pub subscriber_id: u64,
    /// The topic the payload was received on.
    pub topic: String,
    /// The message payload.
    pub payload: Bytes,
}

impl Api for MessageReceived {
    type Response = Self;
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("MessageReceived")
    }
}

/// Unsubscribes `subscriber_id` from messages for `topic`.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UnsubscribeFrom {
    /// The name of the database.
    pub database: String,
    /// The id of the [`Subscriber`](crate::pubsub::Subscriber).
    pub subscriber_id: u64,
    /// The topic to unsubscribe from.
    pub topic: String,
}

impl Api for UnsubscribeFrom {
    type Response = ();
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("UnsubscribeFrom")
    }
}

/// Unregisters the subscriber.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UnregisterSubscriber {
    /// The name of the database.
    pub database: String,
    /// The id of the [`Subscriber`](crate::pubsub::Subscriber).
    pub subscriber_id: u64,
}

impl Api for UnregisterSubscriber {
    type Response = ();
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("UnregisterSubscriber")
    }
}

/// Excutes a key-value store operation.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ExecuteKeyOperation {
    /// The name of the database.
    pub database: String,
    /// The operation to execute.
    pub op: KeyOperation,
}

impl Api for ExecuteKeyOperation {
    type Response = Output;
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("ExecuteKeyOperation")
    }
}

/// Compacts the collection.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CompactCollection {
    /// The name of the database.
    pub database: String,
    /// The name of the collection to compact.
    pub name: CollectionName,
}

impl Api for CompactCollection {
    type Response = ();
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("CompactCollection")
    }
}

/// Compacts the key-value store.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CompactKeyValueStore {
    /// The name of the database.
    pub database: String,
}

impl Api for CompactKeyValueStore {
    type Response = ();
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("CompactKeyValueStore")
    }
}

/// Compacts the entire database.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Compact {
    /// The name of the database.
    pub database: String,
}

impl Api for Compact {
    type Response = ();
    type Error = crate::Error;

    fn name() -> Name {
        Name::from("Compact")
    }
}

/// A networking error.
#[derive(Clone, thiserror::Error, Debug, Serialize, Deserialize)]
pub enum Error {
    /// The server responded with a message that wasn't expected for the request
    /// sent.
    #[error("unexpected response: {0}")]
    UnexpectedResponse(String),

    /// The connection was interrupted.
    #[error("unexpected disconnection")]
    Disconnected,
}
