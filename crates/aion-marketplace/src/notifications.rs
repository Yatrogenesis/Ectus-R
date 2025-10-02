use crate::{config::NotificationConfig, models::*, errors::*};

pub struct NotificationManager {
    config: NotificationConfig,
}

impl NotificationManager {
    pub async fn new(config: NotificationConfig) -> Result<Self> {
        Ok(Self { config })
    }

    pub async fn notify_package_published(&self, _package: &Package, _publisher: &User) -> Result<()> {
        Ok(())
    }

    pub async fn notify_review_submitted(&self, _package: &Package, _review: &PackageReview, _reviewer: &User, _owner: &User) -> Result<()> {
        Ok(())
    }
}