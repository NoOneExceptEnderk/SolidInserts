mod catalog;
mod geometry;
mod locale;

#[cfg(target_arch = "wasm32")]
mod guest;

use catalog::{AUTO, Insert, Series};
use geometry::{Mouth, Pick, Planner};
use locale::{Language, local_format};
use serde::Deserialize;
use serde_json::{Value, json};

pub trait Host {
    fn run(&mut self, command: &str, args: Value) -> Result<Value, String>;
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Settings {
    #[serde(default)]
    serie: Series,
    #[serde(default = "automatic_thread")]
    filettatura: String,
    #[serde(default)]
    compensazione_mm: f64,
    // Script callers can obtain the report without adding features.
    #[serde(default)]
    solo_quote: bool,
    // Older desktop forms and scripts may still send an explicit action.
    operazione: Option<Operation>,
    // A native viewport can pass its hit without changing the document selection.
    hole_pick: Option<Pick>,
    hole_picks: Option<Vec<Pick>>,
}

#[derive(Deserialize, PartialEq, Eq)]
enum Operation {
    #[serde(rename = "Anteprima quote")]
    Preview,
    #[serde(rename = "Applica sedi")]
    Apply,
}

fn automatic_thread() -> String {
    AUTO.into()
}

struct Seat {
    mouth: Mouth,
    insert: Insert,
    diameter: f64,
    depth: f64,
    name: String,
}

pub fn execute(host: &mut impl Host, entry: &str, mut params: Value) -> Result<Value, String> {
    let tag = params
        .as_object_mut()
        .and_then(|object| object.remove("locale"));
    let lang = Language::from_tag(tag.as_ref().and_then(Value::as_str));
    if entry != "applica" {
        return Err(local_format!(
            lang,
            [
                "Comando SolidInserts sconosciuto: {entry}.",
                "Unknown SolidInserts entry: {entry}."
            ]
        ));
    }
    let settings: Settings = serde_json::from_value(params).map_err(|error| {
        local_format!(
            lang,
            [
                "Parametri della sede non validi: {error}",
                "Invalid seat parameters: {error}"
            ]
        )
    })?;
    let legacy_preview = settings.operazione == Some(Operation::Preview);
    let apply = !legacy_preview && !settings.solo_quote;
    if !(-1.0..=1.0).contains(&settings.compensazione_mm) {
        return Err(local_format!(
            lang,
            [
                "La compensazione deve essere fra −1 e +1 mm.",
                "Diameter compensation must be between −1 and +1 mm.",
                "Die Durchmesserkorrektur muss zwischen −1 und +1 mm liegen.",
                "La compensación debe estar entre −1 y +1 mm.",
                "La compensation doit être comprise entre −1 et +1 mm.",
                "A compensação deve ficar entre −1 e +1 mm."
            ]
        ));
    }

    let picks = match (settings.hole_picks, settings.hole_pick) {
        (Some(_), Some(_)) => {
            return Err(local_format!(
                lang,
                [
                    "Fornisci hole_picks oppure hole_pick.",
                    "Specify either hole_picks or hole_pick."
                ]
            ));
        }
        (Some(picks), None) => Some(picks),
        (None, Some(pick)) => Some(vec![pick]),
        (None, None) => None,
    };
    let mut planner = Planner::new(host, picks, lang)?;
    let mut seats = Vec::<Seat>::new();
    for pick in planner.take_selection() {
        let mouth = planner.mouth(pick)?;
        if seats.iter().any(|seat| seat.mouth.same_opening(&mouth)) {
            continue;
        }
        let insert = catalog::choose(settings.serie, &settings.filettatura, mouth.diameter, lang)?;
        let diameter = round_mm(insert.bore_mm + settings.compensazione_mm);
        let depth = round_mm(insert.depth_mm());
        let name = local_format!(
            lang,
            [
                "Inserto {}",
                "Insert {}",
                "Gewindeeinsatz {}",
                "Inserto {}",
                "Insert {}",
                "Inserto {}"
            ],
            insert.product_name
        );
        seats.push(Seat {
            mouth,
            insert,
            diameter,
            depth,
            name,
        });
    }
    let mut report = Vec::with_capacity(seats.len());
    let mut obstacles = Vec::new();
    for seat in &seats {
        let clearance = planner.entrance_clearance(&seat.mouth)?;
        let wall = round_mm(clearance - seat.diameter / 2.0);
        let mut issues = Vec::new();
        if seat.diameter <= seat.mouth.diameter + geometry::TOLERANCE_MM {
            issues.push(local_format!(
                lang,
                [
                    "La sede Ø{:.2} deve essere più larga del foro Ø{:.2} mm.",
                    "The Ø{:.2} seat must be wider than the Ø{:.2} mm hole.",
                    "Die Aufnahme mit Ø{:.2} muss größer als die Bohrung mit Ø{:.2} mm sein.",
                    "El alojamiento de Ø{:.2} debe ser más ancho que el agujero de Ø{:.2} mm.",
                    "Le logement de Ø{:.2} doit être plus large que le trou de Ø{:.2} mm.",
                    "O alojamento de Ø{:.2} deve ser mais largo que o furo de Ø{:.2} mm."
                ],
                seat.diameter,
                seat.mouth.diameter
            ));
        }
        if wall <= geometry::TOLERANCE_MM {
            issues.push(local_format!(lang, ["La sede Ø{:.2} raggiunge un bordo vicino all’ingresso.", "The Ø{:.2} seat reaches a nearby edge at the opening.", "Die Aufnahme mit Ø{:.2} reicht bis zu einer benachbarten Kante an der Öffnung.", "El alojamiento de Ø{:.2} alcanza un borde cercano a la entrada.", "Le logement de Ø{:.2} atteint un bord proche de l’entrée.", "O alojamento de Ø{:.2} alcança uma borda próxima à entrada."],
                seat.diameter
            ));
        }
        if !seat.mouth.can_deepen && seat.depth + geometry::TOLERANCE_MM >= seat.mouth.pilot_depth {
            issues.push(local_format!(lang, ["La sede richiede {:.2} mm di profondità. La parete cilindrica termina a {:.2} mm; scegli un inserto più corto.", "The seat needs {:.2} mm of depth. The cylindrical wall ends at {:.2} mm; choose a shorter insert.", "Die Aufnahme benötigt {:.2} mm Tiefe. Die Zylinderwand endet bei {:.2} mm. Wähle einen kürzeren Einsatz.", "El alojamiento requiere {:.2} mm de profundidad. La pared cilíndrica termina a {:.2} mm; elige un inserto más corto.", "Le logement nécessite une profondeur de {:.2} mm. La paroi cylindrique se termine à {:.2} mm ; choisis un insert plus court.", "O alojamento precisa de {:.2} mm de profundidade. A parede cilíndrica termina a {:.2} mm; escolha um inserto mais curto."],
                seat.depth, seat.mouth.pilot_depth,
            ));
        }
        let mut notes = Vec::new();
        if wall > geometry::TOLERANCE_MM && wall + geometry::TOLERANCE_MM < seat.insert.wall_mm {
            notes.push(local_format!(lang, ["All’ingresso rimangono {wall:.2} mm verso il bordo più vicino. La scheda indica {:.2} mm.", "The opening has {wall:.2} mm of material to the nearest edge. The catalog specifies {:.2} mm."], seat.insert.wall_mm));
        }
        let ordinal = report.len() + 1;
        obstacles.extend(issues.iter().map(|issue| {
            local_format!(
                lang,
                [
                    "Foro {ordinal} su {}. {issue}",
                    "Hole {ordinal} on {}. {issue}",
                    "Bohrung {ordinal} an {}. {issue}",
                    "Agujero {ordinal} en {}. {issue}",
                    "Trou {ordinal} sur {}. {issue}",
                    "Furo {ordinal} em {}. {issue}"
                ],
                seat.mouth.body
            )
        }));
        report.push(json!({
            "foro": ordinal,
            "corpo": seat.mouth.body,
            "centro_mm": seat.mouth.center,
            "normale": seat.mouth.outward(),
            "diametro_attuale_mm": round_mm(seat.mouth.diameter),
            "profondita_cilindrica_attuale_mm": round_mm(seat.mouth.pilot_depth),
            "filettatura": seat.insert.thread,
            "inserto": seat.insert.product_name,
            "lunghezza_inserto_mm": seat.insert.length_mm,
            "diametro_sede_mm": seat.diameter,
            "profondita_sede_mm": seat.depth,
            "spessore_all_ingresso_mm": wall,
            "spessore_da_scheda_mm": seat.insert.wall_mm,
            "segnalazioni": notes
        }));
    }
    for (index, seat) in seats.iter().enumerate() {
        for (other_index, other) in seats[..index].iter().enumerate() {
            if seat
                .mouth
                .coplanar_distance(&other.mouth)
                .is_some_and(|distance| {
                    distance <= (seat.diameter + other.diameter) / 2.0 + geometry::TOLERANCE_MM
                })
            {
                obstacles.push(local_format!(
                    lang,
                    [
                        "Le sedi dei fori {} e {} si sovrappongono all’ingresso.",
                        "The seats for holes {} and {} overlap at the opening.",
                        "Die Aufnahmen für die Bohrungen {} und {} überlappen sich an der Öffnung.",
                        "Los alojamientos de los agujeros {} y {} se superponen en la entrada.",
                        "Les logements des trous {} et {} se chevauchent à l’entrée.",
                        "Os alojamentos dos furos {} e {} se sobrepõem na entrada."
                    ],
                    other_index + 1,
                    index + 1
                ));
            }
        }
    }

    if apply {
        if !obstacles.is_empty() {
            return Err(obstacles.join(" "));
        }
        for (seat, row) in seats.iter().zip(&mut report) {
            let feature = planner.cut(&seat.mouth, seat.diameter, seat.depth, &seat.name)?;
            row["feature"] = json!(feature);
        }
        planner.clear_selection()?;
    }
    if legacy_preview {
        let proposals: Vec<_> = seats
            .iter()
            .map(|seat| {
                format!(
                    "Foro Ø{} → {} · sede Ø{} × {} mm",
                    quota(round_mm(seat.mouth.diameter)),
                    seat.insert.thread,
                    quota(seat.diameter),
                    quota(seat.depth),
                )
            })
            .collect();
        let warnings: Vec<_> = report
            .iter()
            .flat_map(|row| row["segnalazioni"].as_array().into_iter().flatten())
            .filter_map(Value::as_str)
            .collect();
        let mut lines = vec!["Anteprima quote".to_owned()];
        lines.extend(proposals);
        lines.extend(
            obstacles
                .iter()
                .map(|issue| format!("Impedimento: {issue}")),
        );
        lines.extend(warnings.iter().map(|warning| format!("Avviso: {warning}")));
        return Err(lines.join("\n"));
    }
    let mut result = json!({
        "serie": format!("CNC Kitchen {}", settings.serie.label()),
        "sedi": report,
    });
    if apply {
        result["applicate"] = json!(report.len());
    } else {
        result["quote"] = json!(
            seats
                .iter()
                .map(|seat| {
                    format!(
                        "{}  ·  Ø{:.2} × {:.2} mm",
                        seat.insert.thread, seat.diameter, seat.depth
                    )
                    .replace('.', ",")
                })
                .collect::<Vec<_>>()
        );
        if !obstacles.is_empty() {
            result["impedimenti"] = json!(obstacles);
        }
    }
    Ok(result)
}

fn quota(value: f64) -> String {
    value.to_string().replace('.', ",")
}

fn round_mm(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}
