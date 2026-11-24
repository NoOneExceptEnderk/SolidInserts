use crate::{
    Host,
    catalog::AUTO,
    geometry::{Mouth, Pick, Planner, TOLERANCE_MM, difference, dot},
    locale::{Language, local_format},
    round_mm,
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) const ENTRY: &str = "captive-nut-pockets";
const AUTO_TOLERANCE_MM: f64 = 0.25;
const MATCH_EPSILON: f64 = 1e-8;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Settings {
    hole_picks: Vec<Pick>,
    #[serde(default)]
    print_face: Vec<Pick>,
    #[serde(default = "crate::automatic_thread")]
    nut_size: String,
    inset_mm: f64,
    side_clearance_mm: f64,
    axial_clearance_mm: f64,
    layer_height_mm: f64,
    first_layer_mm: f64,
    #[serde(default)]
    solo_quote: bool,
}

struct Pocket {
    mouth: Mouth,
    nut: Nut,
    flats: f64,
    depth: f64,
    floor_z: f64,
    roof_z: f64,
}

#[derive(Clone, Copy)]
struct Layers {
    first: f64,
    height: f64,
}

impl Layers {
    fn top(self, layer: u32) -> f64 {
        if layer == 0 {
            0.0
        } else {
            self.first + f64::from(layer - 1) * self.height
        }
    }

    // The nominal layer slices are taken at the midpoints of the layers.
    // Return the number of completed layers with a midpoint below the plane.
    fn before(self, z: f64) -> u32 {
        const EPSILON: f64 = 1e-8;
        if z <= self.first / 2.0 + EPSILON {
            return 0;
        }
        let second_slice = self.first + self.height / 2.0;
        1 + (((z - second_slice - EPSILON) / self.height).ceil().max(0.0) as u32)
    }

    fn pause(self, floor_z: f64, roof_z: f64, nut_height: f64) -> Option<u32> {
        let floor = self.before(floor_z);
        let open = self.before(roof_z);
        (floor > 0 && open > floor && self.top(open) - self.top(floor) >= nut_height + TOLERANCE_MM)
            .then_some(open + 1)
    }
}

#[derive(Clone, Copy)]
struct Nut {
    size: &'static str,
    nominal: f64,
    flats: f64,
    height: f64,
}

impl Nut {
    const fn new(size: &'static str, nominal: f64, flats: f64, height: f64) -> Self {
        Self {
            size,
            nominal,
            flats,
            height,
        }
    }
}

// Würth DIN 934 A2 hex nuts.
const NUTS: [Nut; 8] = [
    Nut::new("M2", 2.0, 4.0, 1.6),
    Nut::new("M2.5", 2.5, 5.0, 2.0),
    Nut::new("M3", 3.0, 5.5, 2.4),
    Nut::new("M4", 4.0, 7.0, 3.2),
    Nut::new("M5", 5.0, 8.0, 4.0),
    Nut::new("M6", 6.0, 10.0, 5.0),
    Nut::new("M8", 8.0, 13.0, 6.5),
    Nut::new("M10", 10.0, 17.0, 8.0),
];

fn detect_nut(diameter: f64, lang: Language) -> Result<Nut, String> {
    let difference = |nut: &Nut| (nut.nominal - diameter).abs();
    let mut matches = NUTS
        .into_iter()
        .filter(|nut| difference(nut) <= AUTO_TOLERANCE_MM + MATCH_EPSILON);
    let Some(first) = matches.next() else {
        return Err(local_format!(
            lang,
            [
                "Foro Ø{diameter:.2} mm. Nessuna misura nominale entro ±0,25 mm. Scegli il dado a mano.",
                "Hole Ø{diameter:.2} mm. No nominal size within ±0.25 mm. Choose the nut size manually."
            ]
        ));
    };
    let Some(second) = matches.next() else {
        return Ok(first);
    };
    let delta = difference(&first) - difference(&second);
    if delta.abs() <= MATCH_EPSILON {
        return Err(local_format!(
            lang,
            [
                "Foro Ø{diameter:.2} mm. {} e {} sono equidistanti. Scegli il dado a mano.",
                "Hole Ø{diameter:.2} mm. {} and {} are equally close. Choose the nut size manually."
            ],
            first.size,
            second.size
        ));
    }
    Ok(if delta < 0.0 { first } else { second })
}

pub(crate) fn run(
    host: &mut impl Host,
    mut params: Value,
    lang: Language,
    quote_only: bool,
) -> Result<Value, String> {
    if let Some(values) = params.as_object_mut() {
        values.remove("pause_report");
        values.remove("matched_sizes");
    }
    let settings: Settings = serde_json::from_value(params).map_err(|error| {
        local_format!(
            lang,
            [
                "Parametri della tasca non validi: {error}",
                "Invalid nut pocket parameters: {error}"
            ]
        )
    })?;
    let selected_nut = if settings.nut_size == AUTO {
        None
    } else {
        Some(
            NUTS.into_iter()
                .find(|nut| nut.size == settings.nut_size)
                .ok_or_else(|| {
                    local_format!(
                        lang,
                        [
                            "Misura del dado non disponibile.",
                            "Nut size is not available."
                        ]
                    )
                })?,
        )
    };
    let valid_layer = |height| height == 0.0 || (0.01..=2.0).contains(&height);
    if !(0.05..=1000.0).contains(&settings.inset_mm)
        || !(0.0..=1.0).contains(&settings.side_clearance_mm)
        || !(0.0..=2.0).contains(&settings.axial_clearance_mm)
        || !valid_layer(settings.layer_height_mm)
        || !valid_layer(settings.first_layer_mm)
    {
        return Err(local_format!(
            lang,
            [
                "Controlla la profondità, il gioco e l’altezza degli strati.",
                "Check the rim offset, clearances and layer heights."
            ]
        ));
    }
    let layers = (settings.layer_height_mm > 0.0).then(|| Layers {
        height: settings.layer_height_mm,
        first: if settings.first_layer_mm > 0.0 {
            settings.first_layer_mm
        } else {
            settings.layer_height_mm
        },
    });
    let offset = settings.inset_mm;
    let apply = !quote_only && !settings.solo_quote;
    let mut planner = Planner::new(host, Some(settings.hole_picks), lang)?;
    let mut sized_holes = Vec::<(Mouth, Nut)>::new();
    for pick in planner.take_selection() {
        let mouth = planner.mouth(pick)?;
        if sized_holes
            .iter()
            .any(|(previous, _)| previous.same_opening(&mouth))
        {
            continue;
        }
        let nut = match selected_nut {
            Some(nut) => nut,
            None => detect_nut(mouth.diameter, lang)?,
        };
        sized_holes.push((mouth, nut));
    }
    let sizes: BTreeSet<_> = sized_holes.iter().map(|(_, nut)| nut.size).collect();
    let matched_sizes = sizes.iter().copied().collect::<Vec<_>>().join(", ");
    let undersize: Vec<_> = sized_holes
        .iter()
        .enumerate()
        .filter(|(_, (mouth, nut))| mouth.diameter < nut.nominal - TOLERANCE_MM)
        .map(|(index, _)| index + 1)
        .collect();
    let print_face = match <[Pick; 1]>::try_from(settings.print_face) {
        Ok([face]) => face,
        // The form can show the match while the user is still choosing the orientation.
        Err(faces) if quote_only && faces.is_empty() => {
            return Ok(
                json!({"matched_sizes": matched_sizes, "holes_with_undersize_bore": undersize}),
            );
        }
        Err(_) => {
            return Err(local_format!(
                lang,
                ["Seleziona una faccia di stampa.", "Select one print face."]
            ));
        }
    };
    let (body, bed, up) = planner.print_plane(print_face)?;
    let mut pockets = Vec::<Pocket>::with_capacity(sized_holes.len());
    for (mouth, nut) in sized_holes {
        let size = nut.size;
        let flats = round_mm(nut.flats + 2.0 * settings.side_clearance_mm);
        let depth = round_mm(nut.height + settings.axial_clearance_mm);
        let along_bore = dot(mouth.inward(), up);
        if mouth.body != body || along_bore.abs() < 0.99999999 {
            return Err(local_format!(
                lang,
                [
                    "Scegli una faccia di stampa sullo stesso corpo, perpendicolare a tutti gli assi dei fori.",
                    "Select a print face on the same body, perpendicular to every hole axis."
                ]
            ));
        }
        if (selected_nut.is_some() && mouth.diameter < nut.nominal - TOLERANCE_MM)
            || mouth.diameter >= flats - TOLERANCE_MM
        {
            return Err(local_format!(
                lang,
                [
                    "Il foro Ø{:.2} mm non è adatto alla vite {size} e alla tasca da {flats:.2} mm tra facce.",
                    "The Ø{:.2} mm hole does not fit the {size} screw and the {flats:.2} mm across-flats pocket."
                ],
                mouth.diameter
            ));
        }
        if offset + depth + TOLERANCE_MM >= mouth.pilot_depth {
            return Err(local_format!(
                lang,
                [
                    "La tasca termina a {:.2} mm dall’imbocco e il foro cilindrico a {:.2} mm. Lascia materiale anche oltre la tasca.",
                    "The pocket ends {:.2} mm from its rim and the cylindrical hole ends at {:.2} mm. Leave material beyond the pocket."
                ],
                offset + depth,
                mouth.pilot_depth
            ));
        }
        let mouth_z = dot(difference(mouth.center, bed), up);
        let z1 = mouth_z + along_bore * offset;
        let z2 = mouth_z + along_bore * (offset + depth);
        let floor_z = z1.min(z2);
        let roof_z = z1.max(z2);
        if floor_z <= TOLERANCE_MM || roof_z > 100_000.0 || !roof_z.is_finite() {
            return Err(local_format!(
                lang,
                [
                    "La tasca deve trovarsi interamente sopra la faccia di stampa.",
                    "The pocket must be wholly above the print face."
                ]
            ));
        }
        pockets.push(Pocket {
            mouth,
            nut,
            flats,
            depth,
            floor_z,
            roof_z,
        });
    }

    let mut rows = Vec::with_capacity(pockets.len());
    let mut events = BTreeMap::<(u32, &str), Vec<usize>>::new();
    let mut missing = Vec::new();
    for (index, pocket) in pockets.iter().enumerate() {
        let number = index + 1;
        let size = pocket.nut.size;
        let mut row = json!({
            "hole": number,
            "center_mm": pocket.mouth.center,
            "nut_size": size,
            "across_flats_mm": pocket.flats,
            "pocket_height_mm": pocket.depth,
            "nut_thickness_mm": pocket.nut.height,
            "floor_z_mm": round_mm(pocket.floor_z),
            "roof_z_mm": round_mm(pocket.roof_z)
        });
        if let Some(layers) = layers {
            if let Some(layer) = layers.pause(pocket.floor_z, pocket.roof_z, pocket.nut.height) {
                events.entry((layer, size)).or_default().push(number);
            } else {
                missing.push(number);
            }
        }
        if apply {
            let feature_name =
                local_format!(lang, ["Dado prigioniero {size}", "Captive nut {size}"]);
            row["feature"] = json!(planner.cut_pocket(
                &pocket.mouth,
                pocket.flats,
                offset,
                pocket.depth,
                &feature_name
            )?);
        }
        rows.push(row);
    }
    let pauses: Vec<_> = events
        .into_iter()
        .map(|((layer, size), holes)| {
            let layers = layers.unwrap();
            json!({
                "before_layer": layer,
                "printed_z_mm": round_mm(layers.top(layer - 1)),
                "layer_z_mm": round_mm(layers.top(layer)),
                "nut_size": size,
                "quantity": holes.len(),
                "holes": holes
            })
        })
        .collect();
    let (summary, display) = pause_summary(
        lang,
        &matched_sizes,
        &rows,
        &pauses,
        &missing,
        layers.is_some(),
    );
    if apply {
        planner.set_print_face(&body, bed)?;
        planner.clear_selection()?;
    }
    let mut result = json!({
        "body": body,
        "matched_sizes": matched_sizes,
        "inset_mm": offset,
        "pockets": rows,
        "pauses": pauses,
        "holes_without_pause": missing,
        "summary": summary,
        "display": display,
        "holes_with_undersize_bore": undersize
    });
    if sizes.len() == 1
        && let Some(pocket) = pockets.first()
    {
        result["nut_size"] = json!(pocket.nut.size);
        result["across_flats_mm"] = json!(pocket.flats);
        result["pocket_height_mm"] = json!(pocket.depth);
        result["nut_thickness_mm"] = json!(pocket.nut.height);
    }
    Ok(result)
}

fn pause_summary(
    lang: Language,
    size: &str,
    rows: &[Value],
    pauses: &[Value],
    missing: &[usize],
    fixed: bool,
) -> (String, String) {
    if !fixed {
        let heights: Vec<_> = rows
            .iter()
            .map(|row| row["roof_z_mm"].to_string())
            .collect();
        let display = heights.join(", ");
        let summary = local_format!(
            lang,
            [
                "Quote di chiusura sopra il piatto (mm): {}. Scegli i layer in anteprima.",
                "Pocket closing heights above bed (mm): {}. Choose the layers in Preview."
            ],
            display
        );
        return (summary, display);
    }
    let estimates: Vec<_> = pauses
        .iter()
        .map(|p| {
            format!(
                "{} → {} ({}× {})",
                p["printed_z_mm"],
                p["layer_z_mm"],
                p["quantity"],
                p["nut_size"].as_str().unwrap_or_default()
            )
        })
        .collect();
    let mut display = estimates.join("; ");
    let mut summary = if display.is_empty() {
        String::new()
    } else {
        local_format!(
            lang,
            [
                "{size} · Stima Z già stampata → pausa prima di Z (mm): {}.",
                "{size} · Estimated printed Z → pause before layer at Z (mm): {}."
            ],
            display
        )
    };
    if !missing.is_empty() {
        let numbers: Vec<_> = missing.iter().map(usize::to_string).collect();
        if !summary.is_empty() {
            summary.push(' ');
        }
        summary.push_str(&local_format!(
            lang,
            [
                "Fori {} senza pausa stimabile. Aumenta il gioco assiale o usa strati più sottili.",
                "No estimated pause for holes {}. Increase axial clearance or use thinner layers."
            ],
            numbers.join(", ")
        ));
        if display.is_empty() {
            display = local_format!(lang, ["Nessuna pausa stimabile", "No estimated pause"]);
        }
    }
    (summary, display)
}
