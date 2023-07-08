use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::Command;

use reqwest::Client;
use thiserror::Error;

use crate::types::{
    AddInvoiceRequest, AddInvoiceResponse, LookupInvoiceResponse, SendPaymentSyncRequest,
    SendPaymentSyncResponse,
};

/// Make it easier to handle and propagate errors using the NodeError enum as the error type.
pub type Result<T> = std::result::Result<T, NodeError>;

/// Represent the possible errors that can occur when interacting with a Lightning Network Daemon (LND) node.
#[derive(Error, Debug)]
pub enum NodeError {
    /// An I/O error occurred.
    #[error("I/O error")]
    IoError(#[from] std::io::Error),
    /// An error occurred while making a request.
    #[error("Request error")]
    RequestError(#[from] reqwest::Error),
    /// The request header contained an invalid value.
    #[error("Request header error")]
    RequestHeaderError(#[from] reqwest::header::InvalidHeaderValue),
}

/// Encapsulate data needed to interact with a Lightning Network Daemon (LND) node.
#[derive(Clone, Debug)]
pub struct Node {
    /// The host address of the LND node.
    pub host: String,
    /// The HTTP client used to communicate with the LND node.
    pub client: Client,
}

impl Node {
    /// Initialize a [Node] object using the macaroon and certificate files provided.
    ///
    /// # Arguments
    ///
    /// * `host` - The host address to connect to.
    /// * `macaroon_path` - The path to the macaroon file.
    /// * `cert_path` - The path to the certificate file.
    ///
    pub async fn init<P: AsRef<Path>>(
        host: String,
        macaroon_path: P,
        cert_path: P,
    ) -> Result<Self> {
        let mut cert_file = File::open(cert_path)?;
        let mut cert_raw = Vec::new();
        cert_file.read_to_end(&mut cert_raw)?;
        let cert = reqwest::Certificate::from_pem(&cert_raw)?;

        let cmd_output = Command::new("xxd")
            .args(["-ps", "-u", "-c", "1000"])
            .arg(macaroon_path.as_ref())
            .output()?;

        let mut macaroon = cmd_output.stdout;
        macaroon.retain(|&z| {
            ((z >= b'0' as _) && (z <= b'9' as _)) | ((z >= b'A' as _) && (z <= b'F' as _))
        });

        let mut headers = reqwest::header::HeaderMap::new();
        let mut macaroon_value = reqwest::header::HeaderValue::from_bytes(&macaroon)?;
        macaroon_value.set_sensitive(true);
        headers.insert("Grpc-Metadata-macaroon", macaroon_value);

        let client = reqwest::Client::builder()
            .add_root_certificate(cert)
            .default_headers(headers)
            .build()?;

        Ok(Node { host, client })
    }

    /// Handle the response from an HTTP request and perform error checking based on the response status code.
    ///
    /// # Arguments
    ///
    /// * `response` - An object representing the HTTP response.
    ///
    async fn on_response(response: reqwest::Response) -> Result<reqwest::Response> {
        let status = response.status();

        match status {
            reqwest::StatusCode::OK => Ok(response),
            _ => match response.error_for_status() {
                Ok(res) => Ok(res),
                Err(err) => Err(err.into()),
            },
        }
    }

    /// Send a POST request to add a new invoice.
    ///
    /// # Arguments
    ///
    /// * `invoice` - A reference to a [AddInvoiceRequest] object containing the details of the invoice to be added.
    ///
    pub async fn add_invoice(&self, invoice: &AddInvoiceRequest) -> Result<AddInvoiceResponse> {
        let url = format!("{host}/v1/invoices", host = self.host);

        let mut response = self.client.post(&url).json(invoice).send().await?;

        response = Self::on_response(response).await?;

        let data: AddInvoiceResponse = response.json().await?;

        Ok(data)
    }

    /// Send a GET request to retrieve information about an invoice.
    ///
    /// # Arguments
    ///
    /// * `payment_hash` - A reference to the payment hash of the invoice to lookup.
    ///
    pub async fn lookup_invoice(&self, payment_hash: &String) -> Result<LookupInvoiceResponse> {
        let url = format!(
            "{host}/v1/invoice/{payment_hash}",
            host = self.host,
            payment_hash = payment_hash
        );

        let mut response = self.client.get(&url).send().await?;

        response = Self::on_response(response).await?;

        let data: LookupInvoiceResponse = match response.json().await {
            Ok(data) => data,
            Err(err) => {
                return Err(err.into());
            }
        };

        Ok(data)
    }

    /// Send a POST request to initiate a payment for a given payment request.
    ///
    /// # Arguments
    ///
    /// * `payment_request` - A reference to a [SendPaymentSyncRequest] object containing the details of the payment request.
    ///
    pub async fn pay_invoice(
        &self,
        payment_request: &SendPaymentSyncRequest,
    ) -> Result<SendPaymentSyncResponse> {
        let url = format!("{host}/v1/channels/transactions", host = self.host);

        let mut response = self.client.post(&url).json(payment_request).send().await?;

        response = Self::on_response(response).await?;

        let data: SendPaymentSyncResponse = response.json().await?;

        Ok(data)
    }
}
