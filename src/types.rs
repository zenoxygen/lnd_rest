use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Represent a base64 encoded string.
pub type Base64String = String;

/// Represent the possible states of an invoice.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub enum InvoiceState {
    /// The invoice is open and awaiting payment.
    OPEN = 0,
    /// The invoice has been settled and the payment has been confirmed.
    SETTLED = 1,
    /// The invoice has been canceled and is no longer valid.
    CANCELED = 2,
    /// The invoice has been accepted but not yet settled.
    ACCEPTED = 3,
}

/// Represent the possible statuses of a payment.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[allow(nonstandard_style)]
pub enum PaymentStatus {
    /// The payment status is unknown.
    UNKNOWN = 0,
    /// The payment is currently in flight.
    IN_FLIGHT = 1,
    /// The payment completed successfully.
    SUCCEEDED = 2,
    /// The payment failed.
    FAILED = 3,
}

/// Represent the possible failure reasons of a payment.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[allow(nonstandard_style)]
pub enum PaymentFailureReason {
    /// Payment isn't failed (yet).
    FAILURE_REASON_NONE = 0,
    /// There are more routes to try, but the payment timeout was exceeded.
    FAILURE_REASON_TIMEOUT = 1,
    /// All possible routes were tried and failed permanently. Or were no routes to the destination at all.
    FAILURE_REASON_NO_ROUTE = 2,
    /// A non-recoverable error has occured.
    FAILURE_REASON_ERROR = 3,
    /// Payment details incorrect (unknown hash, invalid amt or invalid final cltv delta).
    FAILURE_REASON_INCORRECT_PAYMENT_DETAILS = 4,
    /// Insufficient local balance.
    FAILURE_REASON_INSUFFICIENT_BALANCE = 5,
}

/// Represent the possible statuses of an HTLCAttempt.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[allow(nonstandard_style)]
pub enum HTLCStatus {
    /// The HTLC is currently in flight.
    IN_FLIGHT = 0,
    /// The HTLC completed successfully.
    SUCCEEDED = 1,
    /// The HTLC failed.
    FAILED = 2,
}

/// Represent the possible failure reasons of an HTLCAttempt.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[allow(nonstandard_style)]
pub enum FailureCode {
    /// Reserved failure reason.
    RESERVED = 0,
    /// Incorrect or unknown payment details.
    INCORRECT_OR_UNKNOWN_PAYMENT_DETAILS = 1,
    /// Incorrect payment amount.
    INCORRECT_PAYMENT_AMOUNT = 2,
    /// Final incorrect CLTV expiry.
    FINAL_INCORRECT_CLTV_EXPIRY = 3,
    /// Final incorrect HTLC amount.
    FINAL_INCORRECT_HTLC_AMOUNT = 4,
    /// Final expiry too soon.
    FINAL_EXPIRY_TOO_SOON = 5,
    /// Invalid realm.
    INVALID_REALM = 6,
    /// Expiry too soon.
    EXPIRY_TOO_SOON = 7,
    /// Invalid onion version.
    INVALID_ONION_VERSION = 8,
    /// Invalid onion HMAC.
    INVALID_ONION_HMAC = 9,
    /// Invalid onion key.
    INVALID_ONION_KEY = 10,
    /// Amount below minimum.
    AMOUNT_BELOW_MINIMUM = 11,
    /// Fee insufficient.
    FEE_INSUFFICIENT = 12,
    /// Incorrect CLTV expiry.
    INCORRECT_CLTV_EXPIRY = 13,
    /// Channel disabled.
    CHANNEL_DISABLED = 14,
    /// Temporary channel failure.
    TEMPORARY_CHANNEL_FAILURE = 15,
    /// Required node feature missing.
    REQUIRED_NODE_FEATURE_MISSING = 16,
    /// Required channel feature missing.
    REQUIRED_CHANNEL_FEATURE_MISSING = 17,
    /// Unknown next peer.
    UNKNOWN_NEXT_PEER = 18,
    /// Temporary node failure.
    TEMPORARY_NODE_FAILURE = 19,
    /// Permanent node failure.
    PERMANENT_NODE_FAILURE = 20,
    /// Permanent channel failure.
    PERMANENT_CHANNEL_FAILURE = 21,
    /// Expiry too far.
    EXPIRY_TOO_FAR = 22,
    /// MPP timeout.
    MPP_TIMEOUT = 23,
    /// Invalid onion payload.
    INVALID_ONION_PAYLOAD = 24,
    /// Internal failure.
    INTERNAL_FAILURE = 997,
    /// Unknown failure.
    UNKNOWN_FAILURE = 998,
    /// An unreadable failure result is returned if the received failure message cannot be decrypted.
    UNREADABLE_FAILURE = 999,
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/add-invoice#lnrpcinvoice).
#[derive(Debug, Default, Serialize)]
pub struct AddInvoiceRequest {
    pub memo: Option<String>,
    pub r_preimage: Option<String>,
    pub value_msat: u64,
    pub description_hash: Option<String>,
    pub expiry: i32,
    pub fallback_addr: Option<String>,
    pub cltv_expiry: Option<i32>,
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/add-invoice#lnrpcaddinvoiceresponse).
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct AddInvoiceResponse {
    pub r_hash: Base64String,
    pub payment_request: String,
    pub add_index: String,
    pub payment_addr: Base64String,
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/lookup-invoice#lnrpcinvoice).
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct LookupInvoiceResponse {
    pub memo: String,
    pub r_preimage: Base64String,
    pub r_hash: Base64String,
    pub value: String,
    pub value_msat: String,
    pub settled: bool,
    pub settle_date: String,
    pub creation_date: String,
    pub payment_request: String,
    pub expiry: String,
    pub state: InvoiceState,
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/send-payment-sync#lnrpcfeelimit).
#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
pub struct FeeLimit {
    pub fixed: Option<String>,
    pub fixed_msat: Option<String>,
    pub percent: Option<String>,
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/send-payment-sync#lnrpcsendrequest).
#[derive(Debug, Default, Serialize)]
pub struct SendPaymentSyncRequest {
    pub dest: Option<Base64String>,
    pub amt: Option<String>,
    pub amt_msat: Option<String>,
    pub payment_hash: Option<Base64String>,
    pub payment_request: String,
    pub final_cltv_delta: Option<i32>,
    pub fee_limit: Option<FeeLimit>,
    pub outgoing_chan_id: Option<String>,
    pub last_hop_pubkey: Option<Base64String>,
    pub cltv_limit: Option<i64>,
    pub allow_self_payment: Option<bool>,
    pub dest_features: Option<Vec<u8>>,
    pub payment_addr: Option<Base64String>,
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/send-payment-sync#lnrpcmpprecord).
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct MppRecord {
    pub payment_addr: Base64String,
    pub total_amt_msat: String,
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/send-payment-sync#lnrpcamprecord).
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct AmpRecord {
    pub root_share: Base64String,
    pub set_id: Base64String,
    pub child_index: i64,
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/send-payment-sync#lnrpchop).
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct Hop {
    pub chan_id: String,
    pub chan_capacity: String,
    pub amt_to_forward: String,
    pub fee: String,
    pub expiry: i64,
    pub amt_to_forward_msat: String,
    pub fee_msat: String,
    pub pub_key: Option<String>,
    pub tlv_payload: bool,
    pub mpp_record: Option<MppRecord>,
    pub amp_record: Option<AmpRecord>,
    pub custom_records: HashMap<String, String>,
    pub metadata: Base64String,
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/send-payment-sync#lnrpcroute).
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct Route {
    pub total_time_lock: i64,
    pub total_amt: String,
    pub total_amt_msat: String,
    pub total_fees: String,
    pub total_fees_msat: String,
    pub hops: Vec<Hop>,
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/send-payment-sync#lnrpcsendresponse).
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct SendPaymentSyncResponse {
    pub payment_error: String,
    pub payment_preimage: Base64String,
    pub payment_route: Option<Route>,
    pub payment_hash: Base64String,
}

/// See [LND API documentation](https://lightning.engineering/api-docs/api/lnd/lightning/list-payments#lnrpclistpaymentsrequest).
#[derive(Debug, Default, Serialize)]
pub struct ListPaymentsRequest {
    pub include_incomplete: bool,
    pub index_offset: u64,
    pub max_payments: u64,
    pub reversed: bool,
    pub count_total_payments: bool,
    pub creation_date_start: u64,
    pub creation_date_end: u64,
}

/// See [LND API documentation](https://lightning.engineering/api-docs/api/lnd/lightning/list-payments/index.html#lnrpcchannelupdate).
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct ChannelUpdate {
    pub signature: String,
    pub chain_hash: String,
    pub chan_id: String,
    pub timestamp: u32,
    pub message_flags: u32,
    pub channel_flags: u32,
    pub time_lock_delta: u32,
    pub htlc_minimum_msat: String,
    pub base_fee: u32,
    pub fee_rate: u32,
    pub htlc_maximum_msat: String,
    pub extra_opaque_data: String,
}

/// See [LND API documentation](https://lightning.engineering/api-docs/api/lnd/lightning/list-payments/index.html#lnrpcfailure).
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct Failure {
    pub code: FailureCode,
    pub channel_update: Option<ChannelUpdate>,
    pub htlc_msat: String,
    pub onion_sha_256: String,
    pub cltv_expiry: u32,
    pub flags: u32,
    pub failure_source_index: u32,
    pub height: u32,
}

/// See [LND API documentation](https://lightning.engineering/api-docs/api/lnd/lightning/list-payments/index.html#lnrpchtlcattempt).
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct HTLCAttempt {
    pub attempt_id: String,
    pub status: HTLCStatus,
    pub route: Route,
    pub attempt_time_ns: String,
    pub resolve_time_ns: String,
    pub failure: Option<Failure>,
    pub preimage: String,
}

/// See [LND API documentation](https://lightning.engineering/api-docs/api/lnd/lightning/list-payments/index.html#lnrpcpayment).
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct Payment {
    pub payment_hash: Base64String,
    pub payment_preimage: Base64String,
    pub payment_request: String,
    pub status: PaymentStatus,
    pub fee_sat: String,
    pub fee_msat: String,
    pub value_sat: String,
    pub value_msat: String,
    pub creation_time_ns: String,
    pub htlcs: Vec<HTLCAttempt>,
    pub payment_index: String,
    pub failure_reason: PaymentFailureReason,
}

/// See [LND API documentation](https://lightning.engineering/api-docs/api/lnd/lightning/list-payments#lnrpclistpaymentsresponse).
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct ListPaymentsResponse {
    pub payments: Vec<Payment>,
    pub first_index_offset: String,
    pub last_index_offset: String,
    pub total_num_payments: String,
}
