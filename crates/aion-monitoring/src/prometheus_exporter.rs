use anyhow::Result;

pub struct PrometheusExporter {}

impl PrometheusExporter {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn start(&self) -> Result<()> {
        tracing::info!("Prometheus exporter started");
        Ok(())
    }
}