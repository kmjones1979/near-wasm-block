// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockAndReceipts {
    #[prost(message, optional, tag="1")]
    pub block: ::core::option::Option<super::super::sf::near::codec::v1::Block>,
    #[prost(message, repeated, tag="2")]
    pub outcome: ::prost::alloc::vec::Vec<super::super::sf::near::codec::v1::ExecutionOutcomeWithId>,
    #[prost(message, repeated, tag="3")]
    pub receipt: ::prost::alloc::vec::Vec<super::super::sf::near::codec::v1::Receipt>,
}
// @@protoc_insertion_point(module)
