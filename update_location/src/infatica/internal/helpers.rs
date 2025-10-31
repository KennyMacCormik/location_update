//! Helper utilities for constructing form field vectors passed to Infatica API.

use crate::infatica::internal::consts::EXCLUDE_CORPORATE_FIELD;
use crate::infatica::internal::models::InfaticaFormFields;

/// Adds `excludeCorporate=1` form field for queries
/// that should filter out corporate data (e.g. residential only).
pub(crate) fn extras_exclude_corporate() -> InfaticaFormFields {
	vec![(EXCLUDE_CORPORATE_FIELD.to_string(), "1".to_string())]
}

/// Returns an empty form field list (for queries with no extra params).
pub(crate) fn extras_empty() -> InfaticaFormFields {
	Vec::new()
}