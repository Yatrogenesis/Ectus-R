pub mod aws;
pub mod gcp;
pub mod azure;

pub use aws::AWSProvider;
pub use gcp::GCPProvider;
pub use azure::AzureProvider;

use crate::{CloudProvider, CloudProviderInterface};
use std::sync::Arc;

pub struct CloudProviderFactory;

impl CloudProviderFactory {
    pub fn create_provider(provider: CloudProvider) -> Arc<dyn CloudProviderInterface + Send + Sync> {
        match provider {
            CloudProvider::AWS => Arc::new(AWSProvider::new()),
            CloudProvider::GCP => Arc::new(GCPProvider::new()),
            CloudProvider::Azure => Arc::new(AzureProvider::new()),
            _ => panic!("Unsupported provider: {:?}", provider),
        }
    }
}