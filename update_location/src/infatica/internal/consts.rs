//! Shared Infatica constants and defaults.

use std::time::Duration;

/// Default per-request timeout for all Infatica API calls.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Common form field names used by Infatica’s PHP API.
pub const EMAIL_FIELD: &str = "email";
pub const PASSWORD_FIELD: &str = "password";
pub const EXCLUDE_CORPORATE_FIELD: &str = "excludeCorporate";

/// Endpoint paths (relative to Infatica base URL).
pub const GEO_NODES_ENDPOINT: &str = "includes/api/client/geo_nodes.php";
pub const ISP_CODES_ENDPOINT: &str = "includes/api/client/isp_codes.php";
pub const REGION_CODES_ENDPOINT: &str = "includes/api/client/subdivision_codes.php";
pub const ZIP_CODES_ENDPOINT: &str = "includes/api/client/zip-codes.php";
