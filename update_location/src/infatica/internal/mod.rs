//! # Infatica Internal Client
//!
//! This module provides low-level access to Infatica’s HTTP endpoints:
//! - [`geo_nodes`] — returns geo-node listings (country, region, city, ISP, ASN, ZIP, node count)
//! - [`region_codes`] — returns the dictionary of region/subdivision codes
//! - [`zip_codes`] — returns the dictionary of postal/ZIP codes
//! - [`isp_codes`] — returns the dictionary of ISP codes
//!
//! Each function performs a single API query using `reqwest` with form-encoded fields,
//! deserializes the JSON result into strongly-typed Rust structs, and flattens the legacy
//! “array-of-arrays” Infatica format into a simple `Vec<T>`.

pub mod geo_nodes;
pub mod models;
pub mod isp_codes;
mod query_infatica;
mod consts;
mod helpers;
pub mod errors;
pub mod region_codes;
pub mod zip_codes;
