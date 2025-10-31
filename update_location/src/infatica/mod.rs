//! # Infatica Public Module
//!
//! High-level API for fetching Infatica’s data dictionaries in parallel.
//!
//! This module provides a single entrypoint, [`get_all`], which queries all
//! Infatica endpoints concurrently via the internal modules:
//!
//! - [`geo_nodes`] — geographic node data (country, region, city, ISP, ASN, ZIP, node count)
//! - [`region_codes`] — region/subdivision dictionary
//! - [`zip_codes`] — postal/ZIP code dictionary
//! - [`isp_codes`] — ISP dictionary
//!
//! On success, it returns an [`InfaticaQueryResults`] struct containing all four datasets.
//! On failure, it returns a vector of [`InfaticaQueryError`] values, one per failed endpoint.
//!
//! The module isolates all HTTP and schema details inside [`internal`],
//! exposing only strongly typed, high-level methods and result structures.

mod internal;
mod get_all;
mod errors;
mod models;

pub use get_all::get_all;