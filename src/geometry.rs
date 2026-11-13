use crate::Host;
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    f64::consts::PI,
};

pub const TOLERANCE_MM: f64 = 0.01;
const MAX_SELECTION: usize = 24;
const SEAT_NAME_PREFIX: &str = "SolidInserts · ";

type Point = [f64; 3];

fn difference(a: Point, b: Point) -> Point {
    std::array::from_fn(|i| a[i] - b[i])
}

fn dot(a: Point, b: Point) -> f64 {
    a.into_iter().zip(b).map(|(a, b)| a * b).sum()
}

fn norm(v: Point) -> f64 {
    dot(v, v).sqrt()
}

fn radial_distance(v: Point, axis: Point) -> f64 {
    norm(difference(
        v,
        axis.map(|component| component * dot(v, axis)),
    ))
}

fn cross(a: Point, b: Point) -> Point {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn point(value: &Value) -> Option<Point> {
    serde_json::from_value(value.clone()).ok()
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Pick {
    Edge {
        body: String,
        index: usize,
        point: Point,
    },
    Face {
        body: String,
        index: usize,
    },
    #[serde(other)]
    Other,
}

#[derive(Deserialize)]
struct Edge {
    index: usize,
    mid: Point,
    #[serde(default)]
    name: Option<String>,
}

impl Edge {
    fn incident(&self) -> impl Iterator<Item = &str> {
        let name = self.name.as_deref().unwrap_or_default();
        // SolidCraft numbers edges sharing a pair of incident faces with a ~N suffix.
        let name = match name.rsplit_once('~') {
            Some((base, number)) if number.parse::<usize>().is_ok() => base,
            _ => name,
        };
        name.split('|')
    }
}

#[derive(Deserialize)]
struct Cylinder {
    origin: Point,
    axis: Point,
    radius: f64,
}

#[derive(Deserialize)]
struct Face {
    name: Option<String>,
    index: usize,
    exact: bool,
    point: Option<Point>,
    plane_normal: Option<Point>,
    cylinder: Option<Cylinder>,
    sheet_face: Option<String>,
}

struct Topology {
    edges: Vec<Edge>,
    faces: Vec<Face>,
}

impl Topology {
    fn neighbours(&self, edge: &Edge) -> impl Iterator<Item = &Face> {
        edge.incident().filter_map(|name| {
            self.faces
                .iter()
                .find(|face| face.name.as_deref() == Some(name))
        })
    }
}

#[derive(Deserialize)]
struct Circle {
    center: Point,
    axis: Point,
    diameter: f64,
}

struct Entrance<'a> {
    edge: &'a Edge,
    plane: &'a Face,
    side: &'a Face,
    circle: Circle,
    outward: Point,
}

pub struct Mouth {
    pub body: String,
    pub center: Point,
    pub diameter: f64,
    pub pilot_depth: f64,
    pub can_deepen: bool,
    outward: Point,
    edge_name: String,
    plane_name: String,
    side_name: String,
}

impl Mouth {
    pub fn outward(&self) -> Point {
        self.outward
    }

    pub fn same_opening(&self, other: &Self) -> bool {
        self.body == other.body
            && self.plane_name == other.plane_name
            && norm(difference(self.center, other.center)) <= TOLERANCE_MM
    }

    pub fn coplanar_distance(&self, other: &Self) -> Option<f64> {
        let delta = difference(self.center, other.center);
        (self.body == other.body
            && dot(self.outward, other.outward) > 0.99999
            && dot(delta, self.outward).abs() <= TOLERANCE_MM)
            .then(|| norm(delta))
    }

    fn inward(&self) -> Point {
        self.outward.map(|component| -component)
    }
}

pub struct Planner<'a, H> {
    host: CallBudget<'a, H>,
    bodies: BTreeMap<String, Topology>,
    previous_seats: BTreeSet<u64>,
    selection: Vec<Pick>,
}

use crate::locale::{Language, local_format};

struct CallBudget<'a, H> {
    host: &'a mut H,
    used: u16,
    lang: Language,
}

impl<H: Host> Host for CallBudget<'_, H> {
    fn run(&mut self, command: &str, args: Value) -> Result<Value, String> {
        // SolidCraft 0.41 caps one invocation at 256 calls.
        if self.used == 256 {
            let lang = self.lang;
            return Err(local_format!(
                lang,
                [
                    "La selezione richiede troppe misure. Applica le sedi in gruppi più piccoli.",
                    "This selection requires too many measurements. Apply the seats in smaller groups.",
                    "Diese Auswahl benötigt zu viele Messungen. Erstelle die Aufnahmen in kleineren Gruppen.",
                    "La selección requiere demasiadas mediciones. Aplica los alojamientos en grupos más pequeños.",
                    "La sélection nécessite trop de mesures. Applique les logements par petits groupes.",
                    "A seleção exige medições demais. Aplique os alojamentos em grupos menores."
                ]
            ));
        }
        self.used += 1;
        self.host.run(command, args)
    }
}

impl<'a, H: Host> Planner<'a, H> {
    pub fn new(host: &'a mut H, picks: Option<Vec<Pick>>, lang: Language) -> Result<Self, String> {
        let mut host = CallBudget {
            host,
            used: 0,
            lang,
        };
        let design = host.run(
            "document.inspect",
            json!({"timeline":"summary","bodies":false}),
        )?;
        let timeline = list(&design, "timeline", lang)?;
        if !design["active_sketch"].is_null()
            || timeline
                .iter()
                .any(|feature| feature["rolled_back"] == true)
        {
            return Err(local_format!(
                lang,
                [
                    "Chiudi lo schizzo e porta il cursore della cronologia alla fine del modello.",
                    "Close the sketch and move the timeline marker to the end of the model."
                ]
            ));
        }
        let previous_seats = timeline
            .iter()
            .filter(|row| {
                row["name"]
                    .as_str()
                    .is_some_and(|name| name.starts_with(SEAT_NAME_PREFIX))
            })
            .filter_map(|row| row["id"].as_u64())
            .collect();
        let selection = match picks {
            Some(picks) => picks,
            None => decode_list(&design, "selection", lang)?,
        };
        if selection.is_empty() {
            return Err(local_format!(
                lang,
                [
                    "Seleziona il bordo circolare d’ingresso di ciascun foro nella vista 3D.",
                    "Select the circular rim of each hole in the 3D view.",
                    "Wähle die kreisförmige Kante jeder Bohrung in der 3D-Ansicht.",
                    "Selecciona el borde circular de cada agujero en la vista 3D.",
                    "Sélectionne le bord circulaire de chaque trou dans la vue 3D.",
                    "Selecione a borda circular de cada furo na vista 3D."
                ]
            ));
        }
        if selection.len() > MAX_SELECTION {
            return Err(local_format!(
                lang,
                [
                    "Seleziona al massimo {MAX_SELECTION} bordi o facce alla volta.",
                    "Select up to {MAX_SELECTION} edges or faces at a time."
                ]
            ));
        }
        Ok(Self {
            host,
            bodies: BTreeMap::new(),
            previous_seats,
            selection,
        })
    }

    pub fn take_selection(&mut self) -> Vec<Pick> {
        std::mem::take(&mut self.selection)
    }

    pub fn clear_selection(&mut self) -> Result<(), String> {
        self.host.run("select.clear", json!({}))?;
        Ok(())
    }

    pub fn mouth(&mut self, pick: Pick) -> Result<Mouth, String> {
        let lang = self.host.lang;
        let (body, index, edge_point) = match pick {
            Pick::Edge { body, index, point } => (body, index, Some(point)),
            Pick::Face { body, index } => (body, index, None),
            Pick::Other => {
                return Err(local_format!(
                    lang,
                    [
                        "Seleziona un bordo circolare d’ingresso o la parete cilindrica di un foro.",
                        "Select a circular rim or the cylindrical wall of a hole."
                    ]
                ));
            }
        };
        self.load(&body)?;
        let topology = &self.bodies[&body];
        let edges = if edge_point.is_none() {
            let name = topology
                .faces
                .iter()
                .find(|face| face.index == index && face.cylinder.is_some())
                .and_then(|face| face.name.as_deref())
                .ok_or_else(|| {
                    local_format!(
                        lang,
                        [
                            "La faccia selezionata non è la parete cilindrica di un foro.",
                            "The selected face is not the cylindrical wall of a hole."
                        ]
                    )
                })?;
            topology
                .edges
                .iter()
                .filter(|edge| edge.incident().any(|face| face == name))
                .collect::<Vec<_>>()
        } else {
            topology
                .edges
                .iter()
                .filter(|edge| edge.index == index)
                .collect()
        };
        let mut mouths = Vec::<Mouth>::new();
        for edge in edges {
            let Some(entrance) = entrance(&mut self.host, &body, topology, edge, lang)? else {
                continue;
            };
            if edge_point.is_some_and(|pick| {
                let delta = difference(pick, entrance.circle.center);
                let radius = entrance.circle.diameter / 2.0;
                // A viewport pick can be on the tessellated edge, inside the exact circle.
                let tolerance = 0.05_f64.max(radius * 0.02);
                dot(delta, entrance.outward).abs() > tolerance
                    || (radial_distance(delta, entrance.outward) - radius).abs() > tolerance
            }) {
                return Err(local_format!(
                    lang,
                    [
                        "Il bordo selezionato è cambiato. Seleziona di nuovo l’imbocco del foro.",
                        "The selected edge has changed. Select the hole rim again."
                    ]
                ));
            }
            let side_name = face_name(entrance.side, lang)?;
            if feature_id(side_name).is_some_and(|id| self.previous_seats.contains(&id)) {
                return Err(local_format!(
                    lang,
                    [
                        "La sede su {body} è già stata creata con SolidInserts. Modificala nella cronologia.",
                        "SolidInserts already created the seat on {body}. Edit it in the timeline.",
                        "SolidInserts hat die Aufnahme an {body} bereits erstellt. Bearbeite sie in der Zeitleiste.",
                        "SolidInserts ya creó el alojamiento en {body}. Edítalo en la línea de tiempo.",
                        "SolidInserts a déjà créé le logement sur {body}. Modifie-le dans la chronologie.",
                        "O SolidInserts já criou o alojamento em {body}. Edite-o na linha do tempo."
                    ]
                ));
            }
            let (pilot_depth, can_deepen) =
                bore_end(&mut self.host, &body, topology, &entrance, lang)?;
            let mouth = Mouth {
                body: body.clone(),
                center: entrance.circle.center,
                diameter: entrance.circle.diameter,
                pilot_depth,
                can_deepen,
                outward: entrance.outward,
                edge_name: edge.name.clone().ok_or_else(|| {
                    local_format!(
                        lang,
                        [
                            "Il bordo del foro è privo di un riferimento stabile.",
                            "The hole edge has no stable reference."
                        ]
                    )
                })?,
                plane_name: face_name(entrance.plane, lang)?.into(),
                side_name: side_name.into(),
            };
            if !mouths.iter().any(|other| other.same_opening(&mouth)) {
                mouths.push(mouth);
            }
        }
        match mouths.len() {
            0 => Err(local_format!(
                lang,
                [
                    "Selezione su {body}: scegli un imbocco circolare su una faccia piana, con parete cilindrica e senza smusso.",
                    "On {body}, select a circular rim on a flat face, with a cylindrical wall and no chamfer.",
                    "Wähle an {body} eine kreisförmige Bohrungskante auf einer ebenen Fläche mit zylindrischer Wand und ohne Fase.",
                    "En {body}, selecciona un borde circular sobre una cara plana, con pared cilíndrica y sin chaflán.",
                    "Sur {body}, sélectionne un bord circulaire sur une face plane, avec une paroi cylindrique et sans chanfrein.",
                    "Em {body}, selecione uma borda circular em uma face plana, com parede cilíndrica e sem chanfro."
                ]
            )),
            1 => Ok(mouths.remove(0)),
            _ => Err(local_format!(
                lang,
                [
                    "Il foro su {body} ha due imbocchi. Seleziona il bordo circolare dal lato in cui va l’inserto.",
                    "The hole on {body} has two openings. Select the rim on the side that takes the insert."
                ]
            )),
        }
    }

    pub fn entrance_clearance(&mut self, mouth: &Mouth) -> Result<f64, String> {
        let lang = self.host.lang;
        let topology = &self.bodies[&mouth.body];
        let mut distance = f64::INFINITY;
        for edge in &topology.edges {
            if !edge.incident().any(|name| name == mouth.plane_name)
                || edge.incident().any(|name| name == mouth.side_name)
            {
                continue;
            }
            let measure = self.host.run(
                "inspect.measure",
                json!({"items":[
                    {"type":"vertex","body":mouth.body,"index":0,"point":mouth.center},
                    edge_pick(&mouth.body, edge)
                ]}),
            )?;
            distance = distance.min(number(&measure, "distance_mm", lang)?);
        }
        distance.is_finite().then_some(distance).ok_or_else(|| {
            local_format!(
                lang,
                [
                    "Impossibile misurare i bordi attorno all’imbocco.",
                    "Could not measure the edges around the opening."
                ]
            )
        })
    }

    pub fn cut(
        &mut self,
        mouth: &Mouth,
        diameter: f64,
        depth: f64,
        name: &str,
    ) -> Result<u64, String> {
        let lang = self.host.lang;
        self.load(&mouth.body)?;
        let topology = &self.bodies[&mouth.body];
        let plane = topology
            .faces
            .iter()
            .find(|face| face.exact && face.name.as_deref() == Some(&mouth.plane_name))
            .ok_or_else(|| {
                local_format!(
                    lang,
                    [
                        "La faccia d’ingresso non è più disponibile dopo una modifica precedente.",
                        "The entrance face is no longer available after an earlier edit."
                    ]
                )
            })?;
        let plane_point = plane.point.ok_or_else(|| {
            local_format!(
                lang,
                [
                    "Impossibile leggere la posizione della faccia d’ingresso.",
                    "Could not read the position of the entrance face."
                ]
            )
        })?;
        let edge = topology
            .edges
            .iter()
            .find(|edge| {
                edge.name.as_deref() == Some(&mouth.edge_name)
                    && dot(difference(edge.mid, mouth.center), mouth.outward).abs() <= TOLERANCE_MM
                    && (norm(difference(edge.mid, mouth.center)) - mouth.diameter / 2.0).abs()
                        <= TOLERANCE_MM
            })
            .ok_or_else(|| {
                local_format!(
                    lang,
                    [
                        "L’imbocco è cambiato dopo una modifica precedente.",
                        "The hole opening changed after an earlier edit."
                    ]
                )
            })?;

        let before = volume(&mut self.host, &mouth.body, lang)?;
        let sketch = self.host.run(
            "sketch.create",
            json!({
                "plane":{"face":plane_point,"body":mouth.body,"index":plane.index},
                "project_edges":false,"name":local_format!(lang, ["Profilo {name}", "Profile {name}", "Profil {name}", "Perfil {name}", "Profil {name}", "Perfil {name}"])
            }),
        )?;
        let sketch_id = id(&sketch, "sketch", lang)?;
        if sketch["attachment"] != "exact_face" {
            return Err(local_format!(
                lang,
                [
                    "SolidCraft non ha collegato il profilo alla faccia d’ingresso.",
                    "SolidCraft could not attach the profile to the entrance face."
                ]
            ));
        }
        self.host.run(
            "sketch.project",
            json!({
                "refs":[{"edge":edge.mid,"body":mouth.body}],
                "projection_link":true,"construction":true
            }),
        )?;
        let profile = self
            .host
            .run("sketch.inspect", json!({"sketch":sketch_id}))?;
        let link = list(&profile, "links", lang)?
            .iter()
            .find(|link| link["edge_name"].as_str() == Some(&mouth.edge_name))
            .ok_or_else(|| {
                local_format!(
                    lang,
                    [
                        "SolidCraft non ha collegato il profilo al bordo del foro selezionato.",
                        "SolidCraft could not link the profile to the selected hole edge."
                    ]
                )
            })?;
        let centre = list(&profile, "curves", lang)?
            .iter()
            .find(|curve| curve["link"] == link["id"] && curve["center"].is_string())
            .map(|curve| &curve["center"])
            .ok_or_else(|| {
                local_format!(
                    lang,
                    [
                        "La proiezione del bordo non ha un centro circolare.",
                        "The projected edge has no circle center."
                    ]
                )
            })?;
        let centre_point = list(&profile, "points", lang)?
            .iter()
            .find(|p| &p["id"] == centre)
            .and_then(|p| point(&p["world"]))
            .ok_or_else(|| {
                local_format!(
                    lang,
                    [
                        "Impossibile leggere il centro del profilo.",
                        "Could not read the profile center."
                    ]
                )
            })?;
        if norm(difference(centre_point, mouth.center)) > TOLERANCE_MM {
            return Err(local_format!(
                lang,
                [
                    "Il centro proiettato si è spostato rispetto al foro selezionato.",
                    "The projected center shifted away from the selected hole."
                ]
            ));
        }
        let plane = &profile["plane"];
        let x = point(&plane["x_dir"]).ok_or_else(|| {
            local_format!(
                lang,
                [
                    "Impossibile leggere il piano del profilo.",
                    "Could not read the profile plane."
                ]
            )
        })?;
        let y = point(&plane["y_dir"]).ok_or_else(|| {
            local_format!(
                lang,
                [
                    "Impossibile leggere il piano del profilo.",
                    "Could not read the profile plane."
                ]
            )
        })?;
        let direction = if dot(cross(x, y), mouth.inward()) > 0.0 {
            "positive"
        } else {
            "negative"
        };

        let dimension = format!("{diameter} mm");
        let circle = self.host.run(
            "sketch.circle.center",
            json!({"center":centre,"diameter":dimension}),
        )?;
        let curve = circle
            .pointer("/created/curves/0")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                local_format!(
                    lang,
                    [
                        "Il cerchio della sede non è stato creato.",
                        "Could not create the seat circle."
                    ]
                )
            })?;
        let constraint = self.host.run(
            "sketch.dimension",
            json!({"entities":[curve],"type":"diameter","value":dimension}),
        )?;
        if constraint.pointer("/sketch/dof").and_then(Value::as_u64) != Some(0) {
            return Err(local_format!(
                lang,
                [
                    "Il profilo della sede non è completamente vincolato.",
                    "The seat profile is not fully constrained."
                ]
            ));
        }
        self.host.run("sketch.finish", json!({}))?;
        let cut = self.host.run("solid.extrude", json!({
            "sketch":sketch_id,"profiles":[{"curves":[curve]}],"operation":"cut","direction":direction,
            "distance":format!("{depth} mm"),"targets":[mouth.body],"name":format!("{SEAT_NAME_PREFIX}{name}")
        }))?;
        if cut["changed_bodies"] != json!([mouth.body])
            || cut["created_bodies"] != json!([])
            || cut["removed_bodies"] != json!([])
        {
            return Err(local_format!(
                lang,
                [
                    "Il taglio su {} non ha modificato unicamente il corpo selezionato.",
                    "The cut on {} did not modify only the selected body."
                ],
                mouth.body
            ));
        }
        let feature = id(&cut, "feature", lang)?;
        let after = volume(&mut self.host, &mouth.body, lang)?;
        let expected = PI / 4.0
            * (diameter * diameter * depth
                - mouth.diameter * mouth.diameter * depth.min(mouth.pilot_depth));
        let tolerance = 0.005_f64.max(expected * 0.0005);
        if (before - after - expected).abs() > tolerance {
            return Err(local_format!(
                lang,
                [
                    "La sede su {} interseca un bordo, una cavità o un altro foro alla profondità richiesta.",
                    "The seat on {} intersects an edge, a cavity or another hole at the requested depth.",
                    "Die Aufnahme an {} schneidet bei der gewünschten Tiefe eine Kante, einen Hohlraum oder eine andere Bohrung.",
                    "El alojamiento en {} intersecta un borde, una cavidad u otro agujero a la profundidad solicitada.",
                    "À la profondeur demandée, le logement sur {} coupe un bord, une cavité ou un autre trou.",
                    "Na profundidade solicitada, o alojamento em {} intercepta uma borda, uma cavidade ou outro furo."
                ],
                mouth.body
            ));
        }
        self.bodies.remove(&mouth.body);
        self.load(&mouth.body)?;
        let floor_name = format!("F{feature}:bottom");
        let has_floor = self.bodies[&mouth.body].faces.iter().any(|face| {
            face.name.as_deref() == Some(&floor_name)
                && face
                    .plane_normal
                    .is_some_and(|normal| dot(normal, mouth.outward) > 0.99999)
                && face.point.is_some_and(|p| {
                    (dot(difference(p, mouth.center), mouth.inward()) - depth).abs() <= TOLERANCE_MM
                })
        });
        if !has_floor {
            return Err(local_format!(
                lang,
                [
                    "La sede su {} non ha un fondo alla profondità richiesta. Controlla lo spessore o usa un inserto più corto.",
                    "The seat on {} has no floor at the requested depth. Check the thickness or use a shorter insert.",
                    "Die Aufnahme an {} hat in der gewünschten Tiefe keinen Boden. Prüfe die Wandstärke oder verwende einen kürzeren Einsatz.",
                    "El alojamiento en {} no tiene fondo a la profundidad solicitada. Comprueba el espesor o usa un inserto más corto.",
                    "Le logement sur {} n’a pas de fond à la profondeur demandée. Vérifie l’épaisseur ou utilise un insert plus court.",
                    "O alojamento em {} não tem fundo na profundidade solicitada. Verifique a espessura ou use um inserto mais curto."
                ],
                mouth.body
            ));
        }
        Ok(feature)
    }

    fn load(&mut self, body: &str) -> Result<(), String> {
        let lang = self.host.lang;
        if !self.bodies.contains_key(body) {
            let faces = self.host.run("model.faces", json!({"body":body}))?;
            let edges = self.host.run("model.edges", json!({"body":body}))?;
            self.bodies.insert(
                body.into(),
                Topology {
                    faces: decode_list(&faces, "faces", lang)?,
                    edges: decode_list(&edges, "edges", lang)?,
                },
            );
        }
        Ok(())
    }
}

fn entrance<'a>(
    host: &mut impl Host,
    body: &str,
    topology: &'a Topology,
    edge: &'a Edge,
    lang: Language,
) -> Result<Option<Entrance<'a>>, String> {
    let neighbours: Vec<_> = topology.neighbours(edge).collect();
    let (Some(plane), Some(side)) = (
        neighbours
            .iter()
            .copied()
            .find(|face| face.exact && face.sheet_face.is_none() && face.plane_normal.is_some()),
        neighbours
            .iter()
            .copied()
            .find(|face| face.exact && face.cylinder.is_some()),
    ) else {
        return Ok(None);
    };
    let (Some(outward), Some(plane_point), Some(side_point), Some(cylinder)) =
        (plane.plane_normal, plane.point, side.point, &side.cylinder)
    else {
        return Ok(None);
    };
    if dot(outward, cylinder.axis).abs() < 0.99999 {
        return Ok(None);
    }
    let Some(circle) = measure_circle(host, body, edge, lang)? else {
        return Ok(None);
    };
    let matches_side = (circle.diameter / 2.0 - cylinder.radius).abs() <= TOLERANCE_MM
        && radial_distance(difference(circle.center, cylinder.origin), cylinder.axis)
            <= TOLERANCE_MM;
    let is_inner_rim = radial_distance(difference(plane_point, circle.center), outward)
        > cylinder.radius + TOLERANCE_MM;
    let goes_inward = dot(difference(side_point, circle.center), outward) < -1e-5;
    Ok(
        (matches_side && is_inner_rim && goes_inward).then_some(Entrance {
            edge,
            plane,
            side,
            circle,
            outward,
        }),
    )
}

fn bore_end(
    host: &mut impl Host,
    body: &str,
    topology: &Topology,
    entrance: &Entrance<'_>,
    lang: Language,
) -> Result<(f64, bool), String> {
    let side_name = face_name(entrance.side, lang)?;
    let radius = entrance.circle.diameter / 2.0;
    let mut end: Option<(f64, bool)> = None;
    for edge in &topology.edges {
        let depth = dot(
            difference(entrance.circle.center, edge.mid),
            entrance.outward,
        );
        if edge.index == entrance.edge.index
            || depth <= TOLERANCE_MM
            || !edge.incident().any(|name| name == side_name)
            || end.is_some_and(|(previous, _)| depth >= previous - TOLERANCE_MM)
        {
            continue;
        }
        let Some(circle) = measure_circle(host, body, edge, lang)? else {
            continue;
        };
        if (circle.diameter / 2.0 - radius).abs() > TOLERANCE_MM
            || dot(circle.axis, entrance.outward).abs() < 0.99999
            || radial_distance(
                difference(circle.center, entrance.circle.center),
                entrance.outward,
            ) > TOLERANCE_MM
        {
            continue;
        }
        let bottom = topology.neighbours(edge).find(|face| {
            face.plane_normal
                .is_some_and(|normal| dot(normal, entrance.outward) > 0.99999)
                && face.point.is_some_and(|p| {
                    radial_distance(difference(p, circle.center), entrance.outward) < radius
                })
        });
        let flat_bottom = if let Some(bottom) = bottom {
            let measurement = host.run(
                "inspect.measure",
                json!({"items":[{"type":"face","body":body,"index":bottom.index}]}),
            )?;
            let area = number(&measurement["items"][0], "area_mm2", lang)?;
            (area - PI * radius * radius).abs() < 0.001
        } else {
            false
        };
        end = Some((
            dot(
                difference(entrance.circle.center, circle.center),
                entrance.outward,
            ),
            flat_bottom,
        ));
    }
    end.ok_or_else(|| {
        local_format!(
            lang,
            [
                "Impossibile misurare la profondità cilindrica del foro su {body}.",
                "Could not measure the cylindrical depth of the hole on {body}."
            ]
        )
    })
}

fn edge_pick(body: &str, edge: &Edge) -> Value {
    json!({"type":"edge","body":body,"index":edge.index,"point":edge.mid})
}

fn measure_circle(
    host: &mut impl Host,
    body: &str,
    edge: &Edge,
    lang: Language,
) -> Result<Option<Circle>, String> {
    let measure = host.run("inspect.measure", json!({"items":[edge_pick(body, edge)]}))?;
    let item = &measure["items"][0];
    if item["round"] != "circle" {
        return Ok(None);
    }
    serde_json::from_value(item.clone())
        .map(Some)
        .map_err(|error| {
            local_format!(
                lang,
                [
                    "SolidCraft non ha fornito la geometria del bordo: {error}",
                    "SolidCraft did not return the edge geometry: {error}"
                ]
            )
        })
}

fn volume(host: &mut impl Host, body: &str, lang: Language) -> Result<f64, String> {
    let measure = host.run("inspect.measure", json!({"bodies":[body]}))?;
    let item = &measure["bodies"][0];
    if item["name"].as_str() != Some(body) || item["closed"] != true {
        return Err(local_format!(
            lang,
            [
                "{body} deve essere un corpo solido chiuso.",
                "{body} must be a closed solid body."
            ]
        ));
    }
    number(item, "volume_mm3", lang)
}

fn list<'a>(value: &'a Value, key: &str, lang: Language) -> Result<&'a Vec<Value>, String> {
    value[key].as_array().ok_or_else(|| {
        local_format!(
            lang,
            [
                "La risposta di SolidCraft non contiene {key}.",
                "The SolidCraft response does not contain {key}."
            ]
        )
    })
}

fn decode_list<T: serde::de::DeserializeOwned>(
    value: &Value,
    key: &str,
    lang: Language,
) -> Result<Vec<T>, String> {
    serde_json::from_value(value[key].clone()).map_err(|error| {
        local_format!(
            lang,
            [
                "Elenco {key} non valido: {error}",
                "Invalid {key} list: {error}"
            ]
        )
    })
}

fn number(value: &Value, key: &str, lang: Language) -> Result<f64, String> {
    value[key].as_f64().ok_or_else(|| {
        local_format!(
            lang,
            [
                "SolidCraft non ha fornito la misura {key}.",
                "SolidCraft did not return the {key} measurement."
            ]
        )
    })
}

fn id(value: &Value, key: &str, lang: Language) -> Result<u64, String> {
    value[key].as_u64().ok_or_else(|| {
        local_format!(
            lang,
            [
                "SolidCraft non ha fornito l’identificatore {key}.",
                "SolidCraft did not return the {key} identifier."
            ]
        )
    })
}

fn face_name(face: &Face, lang: Language) -> Result<&str, String> {
    face.name.as_deref().ok_or_else(|| {
        local_format!(
            lang,
            [
                "La faccia del foro è priva di un riferimento stabile.",
                "The hole face has no stable reference."
            ]
        )
    })
}

fn feature_id(face: &str) -> Option<u64> {
    face.strip_prefix('F')?.split_once(':')?.0.parse().ok()
}
