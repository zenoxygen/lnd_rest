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

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/add-invoice#lnrpcinvoice)
#[derive(Debug, Serialize)]
pub struct AddInvoiceRequest {
    pub memo: Option<String>,
    pub r_preimage: Option<String>,
    pub value_msat: u64,
    pub description_hash: Option<String>,
    pub expiry: i32,
    pub fallback_addr: Option<String>,
    pub cltv_expiry: Option<i32>,
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/add-invoice#lnrpcaddinvoiceresponse)
#[derive(Debug, Deserialize)]
pub struct AddInvoiceResponse {
    pub r_hash: Base64String,
    pub payment_request: String,
    pub add_index: String,
    pub payment_addr: Base64String,
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/lookup-invoice#lnrpcinvoice)
#[derive(Debug, Deserialize, Clone)]
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

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/send-payment-sync#lnrpcfeelimit)
#[derive(Debug, Serialize)]
pub struct FeeLimit {
    pub fixed: Option<String>,
    pub fixed_msat: Option<String>,
    pub percent: Option<String>,
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/send-payment-sync#lnrpcsendrequest)
#[derive(Debug, Serialize)]
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

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/send-payment-sync#lnrpcmpprecord)
#[derive(Debug, Deserialize)]
pub struct MppRecord {
    pub payment_addr: Base64String,
    pub total_amt_msat: String,
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/send-payment-sync#lnrpcamprecord)
#[derive(Debug, Deserialize)]
pub struct AmpRecord {
    pub root_share: Base64String,
    pub set_id: Base64String,
    pub child_index: i64,
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/send-payment-sync#lnrpchop)
#[derive(Debug, Deserialize)]
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

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/send-payment-sync#lnrpcroute)
#[derive(Debug, Deserialize)]
pub struct Route {
    pub total_time_lock: i64,
    pub total_amt: String,
    pub total_amt_msat: String,
    pub total_fees: String,
    pub total_fees_msat: String,
    pub hops: Vec<Hop>,
}

/// See [LND API documentation](https://api.lightning.community/api/lnd/lightning/send-payment-sync#lnrpcsendresponse)
#[derive(Debug, Deserialize)]
pub struct SendPaymentSyncResponse {
    pub payment_error: String,
    pub payment_preimage: Base64String,
    pub payment_route: Option<Route>,
    pub payment_hash: Base64String,
}
