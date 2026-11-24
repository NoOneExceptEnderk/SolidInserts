wit_bindgen::generate!({ path: "wit", world: "command-dialog-plugin" });

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

impl exports::craft::plugin::dialog_provider::Guest for SolidInserts {
    fn update_fields(entry: String, values: String) -> Result<String, PluginError> {
        use crate::{
            locale::{Language, local_format},
            nut_pockets,
        };
        use serde_json::json;

        let Ok(mut values) = serde_json::from_str::<Value>(&values) else {
            return Ok("{}".into());
        };
        if entry != nut_pockets::ENTRY {
            return Ok("{}".into());
        }
        let locale = values
            .as_object_mut()
            .and_then(|values| values.remove("locale"));
        let lang = Language::from_tag(locale.as_ref().and_then(Value::as_str));
        let mut sizes = json!({"enabled": false, "value": ""});
        let mut pauses = sizes.clone();
        if values["hole_picks"]
            .as_array()
            .is_some_and(|picks| !picks.is_empty())
        {
            match nut_pockets::run(&mut SolidCraft, values, lang, true) {
                Ok(report) => {
                    sizes["value"] = report["matched_sizes"].clone();
                    if report["holes_with_undersize_bore"]
                        .as_array()
                        .is_some_and(|holes| !holes.is_empty())
                    {
                        let note = local_format!(
                            lang,
                            [
                                "Alcuni fori sono più stretti del diametro nominale della vite. Verificane il passaggio.",
                                "Some holes are smaller than the nominal screw diameter. Check screw clearance."
                            ]
                        );
                        let marker = local_format!(lang, ["verifica Ø", "check bore Ø"]);
                        sizes["hint"] = json!(note);
                        sizes["value"] = json!(format!(
                            "{} · {marker}",
                            sizes["value"].as_str().unwrap_or_default()
                        ));
                    }
                    if let Some(display) = report["display"].as_str() {
                        pauses["value"] = json!(display);
                        pauses["hint"] = report["summary"].clone();
                    }
                }
                Err(reason) => {
                    sizes["value"] = json!(local_format!(
                        lang,
                        ["Controlla la selezione", "Check selection"]
                    ));
                    sizes["hint"] = json!(reason);
                    pauses["hint"] = sizes["hint"].clone();
                }
            };
        };
        Ok(json!({"matched_sizes": sizes, "pause_report": pauses}).to_string())
    }
}

export!(SolidInserts);
