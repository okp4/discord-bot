//! Components declaration
use abscissa_core::{Component, FrameworkError};
use abscissa_tokio::TokioComponent;

#[derive(Component, Debug)]
#[component(inject = "init_tokio(abscissa_tokio::TokioComponent)")]
pub struct TokioComponent {}

impl TokioComponent {
    pub fn new() -> Result<Self, FrameworkError> {
        Ok(Self {})
    }

    /// Called automatically after `TokioComponent` is initialized
    pub fn init_tokio(&mut self, tokio_cmp: &TokioComponent) -> Result<(), FrameworkError> {
        Ok(())
    }
}
