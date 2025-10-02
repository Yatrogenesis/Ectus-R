use crate::{config::SearchConfig, models::*, errors::*};

pub struct SearchEngine {
    config: SearchConfig,
}

impl SearchEngine {
    pub async fn new(config: SearchConfig) -> Result<Self> {
        Ok(Self { config })
    }

    pub async fn index_package(&self, _package: &Package) -> Result<()> {
        Ok(())
    }

    pub async fn search_packages(&self, _params: &SearchParams) -> Result<SearchResults<uuid::Uuid>> {
        Ok(SearchResults {
            items: vec![],
            total_count: 0,
            page: 1,
            per_page: 20,
            total_pages: 0,
        })
    }
}