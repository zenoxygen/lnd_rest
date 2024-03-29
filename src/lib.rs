//! # lnd_rest
//!
//! Rust wrapper to interact with the REST API of a Lightning Network Daemon node.
//!
//! ## Usage
//!
//! Add package to your `Cargo.toml` manifest:
//!
//! ```sh
//! cargo add lnd_rest
//! ```
//!
//! ### Add a new invoice
//!
//! ```rust
//! use lnd_rest::node::Node;
//! use lnd_rest::types::AddInvoiceRequest;
//!
//! #[tokio::main]
//! async fn main() {
//!     let host = "your_lnd_host".to_string();
//!     let macaroon_path = "path/to/macaroon".to_string();
//!     let cert_path = "path/to/cert".to_string();
//!
//!     let node = Node::init(host, macaroon_path, cert_path).await.unwrap();
//!
//!     let add_invoice_request = AddInvoiceRequest {
//!         value_msat: 1000, // Set the invoice value in millisatoshis
//!         expiry: 3600, // Set the expiry time in seconds
//!         // Set other fields as needed
//!         ..Default::default()
//!     };
//!
//!     let add_invoice_response = node.add_invoice(&add_invoice_request).await;
//!
//!     match add_invoice_response {
//!         Ok(response) => {
//!             println!("Payment hash: {}", response.r_hash);
//!             // Access other fields as needed
//!         }
//!         Err(err) => {
//!             eprintln!("Error adding invoice: {}", err);
//!         }
//!     }
//! }
//! ```
//!
//! ### Lookup an invoice
//!
//! ```rust
//! use lnd_rest::node::Node;
//!
//! #[tokio::main]
//! async fn main() {
//!     let host = "your_lnd_host".to_string();
//!     let macaroon_path = "path/to/macaroon".to_string();
//!     let cert_path = "path/to/cert".to_string();
//!
//!     let node = Node::init(host, macaroon_path, cert_path).await.unwrap();
//!
//!     let payment_hash = "your_payment_hash".to_string();
//!
//!     let lookup_invoice_response = node.lookup_invoice(&payment_hash).await;
//!
//!     match lookup_invoice_response {
//!         Ok(response) => {
//!             println!("Invoice state: {:?}", response.state);
//!             // Access other fields as needed
//!         }
//!         Err(err) => {
//!             eprintln!("Error looking up invoice: {}", err);
//!         }
//!     }
//! }
//! ```
//!
//! ### Pay an invoice
//!
//! ```rust
//! use lnd_rest::node::Node;
//! use lnd_rest::types::SendPaymentSyncRequest;
//!
//! #[tokio::main]
//! async fn main() {
//!     let host = "your_lnd_host".to_string();
//!     let macaroon_path = "path/to/macaroon".to_string();
//!     let cert_path = "path/to/cert".to_string();
//!
//!     let node = Node::init(host, macaroon_path, cert_path).await.unwrap();
//!
//!     let send_payment_request = SendPaymentSyncRequest {
//!         payment_request: "your_payment_request".to_string(),
//!         // Set other fields as needed
//!         ..Default::default()
//!     };
//!
//!     let send_payment_response = node.pay_invoice(&send_payment_request).await;
//!
//!     match send_payment_response {
//!         Ok(response) => {
//!             println!("Payment preimage: {}", response.payment_preimage);
//!             // Access other fields as needed
//!         }
//!         Err(err) => {
//!             eprintln!("Error paying invoice: {}", err);
//!         }
//!     }
//! }
//! ```
//!
//! ### List payments
//!
//! ```rust
//! use lnd_rest::node::Node;
//! use lnd_rest::types::ListPaymentsRequest;
//!
//! #[tokio::main]
//! async fn main() {
//!     let host = "your_lnd_host".to_string();
//!     let macaroon_path = "path/to/macaroon".to_string();
//!     let cert_path = "path/to/cert".to_string();
//!
//!     let node = Node::init(host, macaroon_path, cert_path).await.unwrap();
//!
//!     let list_payments_request = ListPaymentsRequest {
//!         include_incomplete: true, // Include incomplete payments
//!         // Set other fields as needed
//!         ..Default::default()
//!     };
//!
//!     let list_payments_response = node.list_payments(&list_payments_request).await;
//!
//!     match list_payments_response {
//!         Ok(response) => {
//!             for payment in response.payments {
//!                 println!("Payment hash: {}", payment.payment_hash);
//!                 // Access other fields as needed
//!             }
//!         }
//!         Err(err) => {
//!             eprintln!("Error listing payments: {}", err);
//!         }
//!     }
//! }
//! ```

pub mod node;
pub mod types;
