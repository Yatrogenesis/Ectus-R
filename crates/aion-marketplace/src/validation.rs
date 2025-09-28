use crate::{config::ValidationConfig, models::*, errors::*, security::ValidationResult};

pub struct PackageValidator {
    config: ValidationConfig,
}

impl PackageValidator {
    pub async fn new(config: ValidationConfig) -> Result<Self> {
        Ok(Self { config })
    }

    pub async fn validate_package(&self, _data: &[u8], _request: &CreatePackageRequest) -> Result<ValidationResult> {
        Ok(ValidationResult {
            is_valid: true,
            errors: vec![],
        })
    }

    pub async fn validate_review_content(&self, _review: &CreateReviewRequest) -> Result<()> {
        Ok(())
    }
}