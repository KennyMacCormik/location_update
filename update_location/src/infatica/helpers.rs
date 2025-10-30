use crate::infatica::consts::EXCLUDE_CORPORATE_FIELD;
use crate::infatica::models::InfaticaFormFields;

pub(crate) fn extras_exclude_corporate() -> InfaticaFormFields {
    vec![(EXCLUDE_CORPORATE_FIELD.to_string(), "1".to_string())]
}

pub(crate) fn extras_empty() -> InfaticaFormFields {
    Vec::new()
}