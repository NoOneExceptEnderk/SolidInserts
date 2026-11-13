use serde::Deserialize;

#[derive(Clone, Copy, Default, Deserialize, PartialEq, Eq)]
pub enum Series {
    #[default]
    #[serde(alias = "standard")]
    Standard,
    #[serde(rename = "Short Version", alias = "Corto", alias = "corto")]
    Corto,
    #[serde(rename = "VORON M3 × 5 × 4", alias = "Voron", alias = "voron")]
    Voron,
}

impl Series {
    pub fn label(self) -> &'static str {
        match self {
            Self::Standard => "Standard",
            Self::Corto => "Short Version",
            Self::Voron => "VORON M3 × 5 × 4",
        }
    }
}

pub const AUTO: &str = "Automatica (Ø nominale)";
const AUTO_TOLERANCE_MM: f64 = 0.05;

struct Thread {
    label: &'static str,
    nominal: f64,
}

const THREADS: [Thread; 8] = [
    Thread {
        label: "M2 × 0,40",
        nominal: 2.0,
    },
    Thread {
        label: "M2,5 × 0,45",
        nominal: 2.5,
    },
    Thread {
        label: "M3 × 0,50",
        nominal: 3.0,
    },
    Thread {
        label: "M4 × 0,70",
        nominal: 4.0,
    },
    Thread {
        label: "M5 × 0,80",
        nominal: 5.0,
    },
    Thread {
        label: "M6 × 1,00",
        nominal: 6.0,
    },
    Thread {
        label: "M8 × 1,25",
        nominal: 8.0,
    },
    Thread {
        label: "M10 × 1,50",
        nominal: 10.0,
    },
];

pub struct Insert {
    pub thread: &'static str,
    pub product_name: String,
    pub length_mm: f64,
    pub bore_mm: f64,
    pub wall_mm: f64,
}

impl Insert {
    pub fn depth_mm(&self) -> f64 {
        self.length_mm + 1.0
    }
}

// CNC Kitchen, Dimensions & Design Guidelines. Values are L, D2 and W in mm.
use crate::locale::{Language, local_format};

pub fn choose(
    series: Series,
    thread: &str,
    pilot_mm: f64,
    lang: Language,
) -> Result<Insert, String> {
    let thread = if thread == AUTO {
        THREADS
            .iter()
            .find(|size| (size.nominal - pilot_mm).abs() <= AUTO_TOLERANCE_MM + 1e-8)
    } else {
        THREADS.iter().find(|size| size.label == thread)
    }
    .ok_or_else(|| {
        local_format!(
            lang,
            [
                "Foro Ø{pilot_mm:.2} mm. Scegli esplicitamente la filettatura della vite.",
                "Hole Ø{pilot_mm:.2} mm. Select the screw thread explicitly.",
                "Bohrung Ø{pilot_mm:.2} mm. Wähle das Schraubengewinde manuell.",
                "Agujero de Ø{pilot_mm:.2} mm. Selecciona la rosca del tornillo manualmente.",
                "Trou de Ø{pilot_mm:.2} mm. Sélectionne le filetage de la vis manuellement.",
                "Furo de Ø{pilot_mm:.2} mm. Selecione a rosca do parafuso manualmente."
            ]
        )
    })?;

    let (length_mm, bore_mm, wall_mm) = match (series, thread.nominal) {
        (Series::Standard, 2.0) => (3.0, 3.2, 1.3),
        (Series::Standard, 2.5) => (4.0, 4.0, 1.6),
        (Series::Standard, 3.0) => (5.7, 4.0, 1.6),
        (Series::Standard, 4.0) => (8.1, 5.7, 2.1),
        (Series::Standard, 5.0) => (9.5, 6.5, 2.6),
        (Series::Standard, 6.0) => (12.7, 8.1, 3.3),
        (Series::Standard, 8.0) => (12.7, 9.7, 4.5),
        (Series::Standard, 10.0) => (12.7, 12.0, 6.0),
        (Series::Corto, 3.0) => (3.0, 4.0, 1.6),
        (Series::Corto, 4.0) => (4.0, 5.7, 2.1),
        (Series::Corto, 5.0) => (5.8, 6.5, 2.6),
        (Series::Voron, 3.0) => (4.0, 4.4, 1.6),
        _ => {
            return Err(local_format!(
                lang,
                [
                    "{} non è disponibile nella serie CNC Kitchen {}.",
                    "{} is not available in the CNC Kitchen {} series.",
                    "{} ist in der CNC-Kitchen-Serie {} nicht verfügbar.",
                    "{} no está disponible en la serie CNC Kitchen {}.",
                    "{} n’est pas disponible dans la série CNC Kitchen {}.",
                    "{} não está disponível na série CNC Kitchen {}."
                ],
                thread.label,
                series.label()
            ));
        }
    };
    let size = thread
        .label
        .split_once(" × ")
        .map_or(thread.label, |(size, _)| size);
    let product_name = match series {
        Series::Standard => format!("{size} × {length_mm}"),
        Series::Corto => format!("{size} × {length_mm} (Short Version)"),
        Series::Voron => series.label().to_owned(),
    };
    Ok(Insert {
        thread: thread.label,
        product_name,
        length_mm,
        bore_mm,
        wall_mm,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pilot_selects_the_thread_but_series_sets_the_seat() {
        for (series, diameter, depth) in [
            (Series::Standard, 4.0, 6.7),
            (Series::Corto, 4.0, 4.0),
            (Series::Voron, 4.4, 5.0),
        ] {
            let insert = choose(series, AUTO, 3.0 - 1e-12, Language::Italian).unwrap();
            assert_eq!(insert.thread, "M3 × 0,50");
            assert_eq!(insert.bore_mm, diameter);
            assert_eq!(insert.depth_mm(), depth);
        }
        assert_eq!(
            choose(Series::Standard, AUTO, 2.55, Language::Italian)
                .unwrap()
                .thread,
            "M2,5 × 0,45"
        );
        assert!(choose(Series::Standard, AUTO, 3.2, Language::Italian).is_err());
        assert!(choose(Series::Standard, AUTO, 4.5, Language::Italian).is_err());
        assert!(choose(Series::Voron, "M4 × 0,70", 4.0, Language::Italian).is_err());
    }

    #[test]
    fn the_single_desktop_dialog_offers_the_catalog_threads() {
        let manifest: serde_json::Value =
            serde_json::from_str(include_str!("../plugin.json")).unwrap();
        let options: Vec<_> = std::iter::once(AUTO)
            .chain(THREADS.iter().map(|size| size.label))
            .collect();
        let entries = manifest["runtime"]["entries"].as_array().unwrap();
        assert_eq!(manifest["version"], env!("CARGO_PKG_VERSION"));
        let [apply] = entries.as_slice() else {
            panic!("SolidInserts must expose one entry");
        };
        assert_eq!(
            apply["slots"],
            serde_json::json!([{"kind": "menu", "name": "Sedi per inserti"}])
        );
        assert_eq!(apply["name"], "applica");
        assert_eq!(apply["effect"], "edit");
        let fields = &apply["input_schema"]["properties"];
        assert_eq!(fields.as_object().unwrap().len(), 3);
        assert_eq!(apply["input_schema"]["x-solidinserts-select-holes"], 2);
        assert_eq!(fields["filettatura"]["enum"], serde_json::json!(options));
        assert_eq!(fields["filettatura"]["default"], AUTO);
        let series = [Series::Standard, Series::Corto, Series::Voron];
        assert_eq!(
            fields["serie"]["enum"],
            serde_json::json!(series.map(Series::label))
        );
        for label in series
            .map(Series::label)
            .into_iter()
            .chain(["Corto", "Voron"])
        {
            assert!(serde_json::from_value::<Series>(serde_json::json!(label)).is_ok());
        }
    }
}
