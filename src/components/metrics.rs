//! A prometheus endpoint.
use std::slice::Iter;
use std::time::Duration;

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
        if let Some(addr) = config.metrics.endpoint {
            metrics_exporter_prometheus::PrometheusBuilder::new()
                .with_http_listener(addr)
                .install()
                .map_err(|e| FrameworkErrorKind::ComponentError.context(e))?;

            self.install_metrics(config.metrics.refresh);

            info!("👂 Prometheus endpoint: {}", addr);
        }
        Ok(())
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

    #[cfg(target_os = "linux")]
    fn install_metrics(&self, duration: Duration) {
        use tracing::warn;

        metrics_process_promstyle::describe();

        info!(
            "⚙️ Start metrics process (duration: {} ms)",
            duration.as_millis()
        );
        std::thread::spawn(move || {
            std::thread::sleep(duration);

            // #[cfg(target_os = "linux")]
            if let Err(why) = metrics_process_promstyle::emit_now() {
                warn!("❌ Failed to emit process metrics: {}", why);
            }
        });
    }

    #[cfg(not(target_os = "linux"))]
    fn install_metrics(&self, _: Duration) {
        info!("⚠️️ Process metrics are not supported for the architecture.");
    }
}
