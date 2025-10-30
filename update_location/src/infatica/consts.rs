use std::time::Duration;

pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

pub const EMAIL_FIELD: &str = "email";
pub const PASSWORD_FIELD: &str = "password";
pub const EXCLUDE_CORPORATE_FIELD: &str = "excludeCorporate";

pub const GEO_NODES_ENDPOINT: &str = "includes/api/client/geo_nodes.php";
pub const ISP_CODES_ENDPOINT: &str = "includes/api/client/isp_codes.php";
