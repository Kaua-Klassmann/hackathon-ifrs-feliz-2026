use sea_orm::entity::prelude::Date;
use validator::ValidationError;

pub fn validate_iso_date(value: &str) -> Result<(), ValidationError> {
    value
        .parse::<Date>()
        .map(|_| ())
        .map_err(|_| ValidationError::new("invalid_iso_date"))
}
