//! A prometheus endpoint.
use std::slice::Iter;

use abscissa_core::component::Id;
use abscissa_core::{Component, FrameworkError, FrameworkErrorKind, Version};
use tracing::info;

use crate::application::DiscordBotApp;
use crate::config::DiscordBotConfig;

/// Abscissa component which runs a metrics endpoint.
#[derive(Debug)]
pub struct MetricsEndpoint {}

impl Component<DiscordBotApp> for MetricsEndpoint {
    fn id(&self) -> Id {
        Id::new(concat!(module_path!(), "::", stringify!(#name)))
    }

    fn version(&self) -> Version {
        Version::parse(env!("CARGO_PKG_VERSION")).unwrap()
    }

    fn after_config(&mut self, config: &DiscordBotConfig) -> Result<(), FrameworkError> {
        match config.metrics.endpoint {
            Some(addr) => {
                metrics_exporter_prometheus::PrometheusBuilder::new()
                    .with_http_listener(addr)
                    .install()
                    .map_err(|e| FrameworkErrorKind::ComponentError.context(e))?;

                info!("👂 Prometheus endpoint: {}", addr);

                Ok(())
            }
            None => Ok(()),
        }
    }

    fn dependencies(&self) -> Iter<'_, Id> {
        [].iter()
    }
}

impl MetricsEndpoint {
    /// Create the component.
    pub fn new() -> Result<Self, FrameworkError> {
        Ok(Self {})
    }
}
