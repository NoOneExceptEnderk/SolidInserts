wit_bindgen::generate!({ path: "wit", world: "command-plugin" });

use crate::Host;
use craft::plugin::{
    commands,
    types::{FieldFailure, PluginError},
};
use serde_json::Value;

struct SolidCraft;

impl Host for SolidCraft {
    fn run(&mut self, command: &str, args: Value) -> Result<Value, String> {
        let response = commands::run(command, &args.to_string()).map_err(|error| match error {
            PluginError::CommandFailed(failure) => {
                format!("{}: {}", failure.command, failure.reason)
            }
            other => other.to_string(),
        })?;
        serde_json::from_str(&response)
            .map_err(|error| format!("Risposta di {command} non valida: {error}"))
    }
}

struct SolidInserts;

impl exports::craft::plugin::entry::Guest for SolidInserts {
    fn execute(entry: String, params: String) -> Result<String, PluginError> {
        let invalid = |reason| {
            PluginError::InvalidInput(FieldFailure {
                field: "SolidInserts".into(),
                reason,
            })
        };
        let params = serde_json::from_str(&params).map_err(|error| invalid(error.to_string()))?;
        crate::execute(&mut SolidCraft, &entry, params)
            .map(|result| result.to_string())
            .map_err(invalid)
    }
}

export!(SolidInserts);
