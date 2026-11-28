//! Capabilities for meragpt models.
//!
//! This module defines model types and their capabilities for meragpt providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::meragpt::Meragpt;

model_capabilities! {
    provider: Meragpt,
    models: {
        Restyler1 {
            model_name: "restyler-1",
            constructor_name: restyler_1,
            display_name: "Restyler 1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
    }
}
