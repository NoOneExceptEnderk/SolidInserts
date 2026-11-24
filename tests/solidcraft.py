"""Run the built component with the SolidCraft CLI, without desktop or user files."""

import argparse
import json
import math
from pathlib import Path
import subprocess
import tempfile
from zipfile import ZipFile


def step(command, **params):
    return {"command": command, "params": params}


def close(actual, expected, tolerance=1e-4):
    assert math.isclose(actual, expected, abs_tol=tolerance), (actual, expected)


class SolidCraft:
    def __init__(self, cli, root, package):
        self.cli, self.root, self.package = cli, root, package
        self.approval = self.run([step("plugin.inspect", path=str(package))])[0]["approval"]

    def run(self, steps, design=None, save=None, expect_error=None):
        script = self.root / "steps.json"
        result = self.root / "results.json"
        script.write_text(json.dumps(steps), encoding="utf-8")
        args = [self.cli, "run", str(script), "--results", "--param-check", "strict", "--json-out", str(result)]
        if design:
            args += ["--in", str(design)]
        if save:
            args += ["--save", str(save)]
        run = subprocess.run(args, capture_output=True, text=True, timeout=60)
        if expect_error is not None:
            assert run.returncode != 0 and expect_error in run.stderr, run.stderr
            return None
        assert run.returncode == 0, run.stderr
        return json.loads(result.read_text(encoding="utf-8"))

    def pick(self, edges, *names):
        return step("select.set", items=[
            {"type": "edge", "body": "Body1", "index": edge["index"]}
            for name in names
            for edge in [next(edge for edge in edges if edge["name"] == name)]
        ])

    def plugin(self, entry="applica", **params):
        return step("plugin.run", path=str(self.package), approval=self.approval, entry=entry, params=params)


def measure(result):
    return result["bodies"][0]["volume_mm3"]


def assert_cylinder(faces, x, y, radius):
    for face in faces["faces"]:
        cylinder = face.get("cylinder")
        if cylinder and all(math.isclose(a, b, abs_tol=1e-5) for a, b in zip(
            [*cylinder["origin"][:2], cylinder["radius"]], [x, y, radius]
        )):
            return
    raise AssertionError(f"Missing cylinder r={radius} at {x}, {y}")


def run_tests(cad):
    inspect = step("document.inspect", timeline="summary", bodies=False)
    measure_body = step("inspect.measure", bodies=["Body1"])
    faces = step("model.faces", body="Body1")
    edges = step("model.edges", body="Body1")
    pilot = cad.root / "pilot.sc"
    finished = cad.root / "seats.sc"

    *_, rim_data, original = cad.run([
        step("parameters.change", name="hole_x", expression="8 mm"),
        step("solid.box", length=42, width=30, height=18, name="Plate"),
        step("solid.hole", position=["hole_x", "8 mm", "18 mm"], diameter=3, depth=7, name="M3 blind"),
        step("solid.hole", position=[23, 9, 18], diameter=4, name="M4 through"),
        step("solid.cylinder", base=[31, 20, 18], radius=1.5, height=3, operation="join", name="Boss"),
        edges, measure_body,
    ], save=pilot)
    rims = rim_data["edges"]
    m3 = "F1:+z|F2:bore:side:0"
    m4 = "F1:+z|F3:bore:side:0"
    select_both = cad.pick(rims, m3, m4)
    cad.run(
        [select_both, cad.plugin(operazione="Anteprima quote")],
        design=pilot,
        expect_error="Foro Ø4 → M4 × 0,70 · sede Ø5,7 × 9,1 mm",
    )
    _, before_quote, quoted, after_quote = cad.run([
        select_both, inspect, cad.plugin(solo_quote=True, filettatura="Automatic (nominal Ø)"), inspect,
    ], design=pilot)
    assert after_quote["selection"] == before_quote["selection"]
    assert after_quote["timeline"] == before_quote["timeline"] and after_quote["undo"] == before_quote["undo"] == 0
    proposed = quoted["result"]["sedi"]
    assert [row["inserto"] for row in proposed] == ["M3 × 5.7", "M4 × 8.1"]
    assert [row["filettatura"] for row in proposed] == ["M3 × 0,50", "M4 × 0,70"]
    assert [(row["diametro_sede_mm"], row["profondita_sede_mm"]) for row in proposed] == [(4.0, 6.7), (5.7, 9.1)]

    rim = next(edge for edge in rims if edge["name"] == m3)
    pick = {"type": "edge", "body": "Body1", "index": rim["index"], "point": rim["mid"]}
    _, quote, untouched, cut, after_click, _, undone = cad.run([
        cad.pick(rims, m4),
        cad.plugin(hole_pick=pick, solo_quote=True), inspect,
        cad.plugin(hole_pick=pick), inspect, step("edit.undo"), measure_body,
    ], design=pilot)
    assert quote["result"]["sedi"][0]["normale"] == [0, 0, 1]
    assert quote["result"]["sedi"][0]["filettatura"] == proposed[0]["filettatura"]
    assert untouched["selection"][0]["index"] != pick["index"] and untouched["undo"] == 0
    assert cut["result"]["applicate"] == 1 and after_click["undo"] == 1
    close(measure(undone), measure(original))

    for name, side, z in [(m4, 1, 18), ("F1:-z|F3:bore:side:0", -1, 0)]:
        rim = next(edge for edge in rims if edge["name"] == name)
        pick = {"type": "edge", "body": "Body1", "index": rim["index"], "point": rim["mid"]}
        result, = cad.run([cad.plugin(hole_pick=pick)], design=pilot)
        seat, = result["result"]["sedi"]
        close(seat["centro_mm"][2], z)
        assert seat["normale"] == [0, 0, side]
    print("PASS explicit hole pick leaves selection and history alone until apply, and accepts either entrance")

    def explicit_pick(name):
        edge = next(edge for edge in rims if edge["name"] == name)
        return {"type": "edge", "body": "Body1", "index": edge["index"], "point": edge["start"]}

    picks = [explicit_pick(name) for name in (m3, m4)]
    _, doc_before, quotes, doc_after, applied, after_cut, cylinders = cad.run([
        cad.pick(rims, m4), inspect,
        cad.plugin(hole_picks=picks, solo_quote=True), inspect,
        cad.plugin(hole_picks=picks), measure_body, faces,
    ], design=pilot, save=finished)
    assert quotes["result"]["sedi"] == proposed
    assert doc_before["selection"] == doc_after["selection"]
    assert doc_before["timeline"] == doc_after["timeline"] and doc_after["undo"] == 0
    cad.run(
        [select_both, cad.plugin(hole_picks=[])], design=pilot, expect_error="Select the circular rim"
    )
    cad.run(
        [cad.plugin(hole_picks=[picks[0], explicit_pick("F1:+z|F4:side")])],
        design=pilot,
        expect_error="select a circular rim",
    )
    for locale, message in [
        ("it", "Seleziona il bordo circolare"),
        ("fr", "Sélectionne le bord circulaire"),
        ("ja", "Select the circular rim"),
    ]:
        cad.run([cad.plugin(hole_picks=[], locale=locale)], design=pilot, expect_error=message)
    localized, = cad.run([
        cad.plugin(hole_picks=picks, locale="fr", solo_quote=True, filettatura="Automatica (Ø nominale)")
    ], design=pilot)
    assert localized["result"]["sedi"] == proposed
    print("PASS locale changes diagnostics, with English fallback, and preserves seat dimensions")
    assert applied["result"]["applicate"] == 2
    removed = math.pi / 4 * ((4.0**2 - 3.0**2) * 6.7 + (5.7**2 - 4.0**2) * 9.1)
    close(measure(original) - measure(after_cut), removed, 0.005)
    for x, y, radius in [(8, 8, 2), (8, 8, 1.5), (23, 9, 2.85), (23, 9, 2)]:
        assert_cylinder(cylinders, x, y, radius)
    print("PASS automatic M3 + M4, blind and through, volumes and concentricity")

    _, _, _, undone, doc, _, redone = cad.run(
        [select_both, cad.plugin(), step("edit.undo"), measure_body, inspect, step("edit.redo"), measure_body],
        design=pilot,
    )
    close(measure(undone), measure(original))
    assert len(doc["timeline"]) == 4 and doc["undo"] == 0
    close(measure(redone), measure(after_cut))
    reopened, _, shifted, shifted_doc = cad.run([
        measure_body, step("parameters.change", name="hole_x", expression="10 mm"), faces, inspect,
    ], design=finished)
    close(measure(reopened), measure(after_cut))
    assert_cylinder(shifted, 10, 8, 2)
    assert all("error" not in feature for feature in shifted_doc["timeline"])
    print("PASS one-step undo/redo, reopen, and seat follows the original hole parameter")

    _, alternate = cad.run([cad.pick(rims, m3), cad.plugin(solo_quote=True, serie="VORON M3 × 5 × 4", compensazione_mm=0.15)], design=pilot)
    voron = alternate["result"]["sedi"][0]
    assert voron["inserto"] == "VORON M3 × 5 × 4"
    assert (voron["diametro_sede_mm"], voron["profondita_sede_mm"]) == (4.55, 5.0)
    for bad_pick, settings, expected in [
        ("F1:+z|F4:side", {}, "select a circular rim"),
        (m4, {"serie": "Voron"}, "is not available"),
    ]:
        cad.run([cad.pick(rims, bad_pick), cad.plugin(**settings)], design=pilot, expect_error=expected)
    print("PASS Voron dimensions, compensation, convex boss and unavailable sizes")

    thin = cad.root / "thin.sc"
    *_, shallow_edges = cad.run([
        step("solid.box", length=38, width=22, height=6, name="Thin plate"),
        step("solid.hole", position=[8, 8, 6], diameter=3, name="M3 through"),
        edges,
    ], save=thin)
    select_shallow = cad.pick(shallow_edges["edges"], "F1:+z|F2:bore:side:0")
    _, blocked = cad.run([
        select_shallow, cad.plugin(solo_quote=True),
    ], design=thin)
    assert "6.00 mm" in blocked["result"]["impedimenti"][0]
    cad.run([select_shallow, cad.plugin()], design=thin, expect_error="6.00 mm")
    print("PASS insufficient depth reported before a through cut")


def test_repeated_insert(cad):
    pilot = cad.root / "m25-pilots.sc"
    partial = cad.root / "m25-partial.sc"
    edges = step("model.edges", body="Body1")
    faces = step("model.faces", body="Body1")
    inspect = step("document.inspect", timeline="summary", bodies=False)
    measure_body = step("inspect.measure", bodies=["Body1"])
    centers = [(8, 8), (23, 8), (38, 8)]
    *_, rims, original = cad.run([
        step("solid.box", length=46, width=20, height=12, name="Plate"),
        *(step("solid.hole", position=[x, y, 12], diameter=2.5, name=f"Pilot {i}")
          for i, (x, y) in enumerate(centers, start=1)),
        edges, measure_body,
    ], save=pilot)
    names = [f"F1:+z|F{i}:bore:side:0" for i in range(2, 5)]
    plugin = cad.plugin(serie="Standard", filettatura="M2,5 × 0,45", locale="en")

    _, applied, batch_volume, _, undone = cad.run([
        cad.pick(rims["edges"], *names), plugin, measure_body, step("edit.undo"), measure_body,
    ], design=pilot)
    assert applied["result"]["applicate"] == len(centers)
    removed_per_seat = math.pi / 4 * (4.0**2 - 2.5**2) * 5.0
    close(measure(original) - measure(batch_volume), len(centers) * removed_per_seat, 0.005)
    close(measure(undone), measure(original))

    _, _, updated_rims = cad.run([
        cad.pick(rims["edges"], names[0]), plugin, edges,
    ], design=pilot, save=partial)
    _, applied, updated_rims = cad.run([
        cad.pick(updated_rims["edges"], names[1]), plugin, edges,
    ], design=partial, save=partial)
    assert applied["result"]["applicate"] == 1
    _, applied, document, sequential_volume, cylinders = cad.run([
        cad.pick(updated_rims["edges"], names[2]), plugin, inspect, measure_body, faces,
    ], design=partial)
    assert applied["result"]["applicate"] == 1
    feature_names = [feature["name"] for feature in document["timeline"]]
    assert len(feature_names) == len(set(feature_names)) == 4 + 2 * len(centers)
    close(measure(sequential_volume), measure(batch_volume))
    for x, y in centers:
        assert_cylinder(cylinders, x, y, 2.0)
    print("PASS identical M2.5 seats in one batch and in separate runs after reopening")


def test_dense_batch(cad):
    pilot = cad.root / "dense-pilots.sc"
    edges = step("model.edges", body="Body1")
    measure_body = step("inspect.measure", bodies=["Body1"])
    centers = [(8 + x * 12, 8 + y * 12) for y in range(4) for x in range(6)]
    *_, rims, before = cad.run([
        step("solid.box", length=76, width=52, height=12),
        *(step("solid.hole", position=[x, y, 12], diameter=2.5) for x, y in centers),
        edges, measure_body,
    ], save=pilot)
    names = ["|".join(sorted(("F1:+z", f"F{i}:bore:side:0"))) for i in range(2, 26)]
    _, applied, after = cad.run([
        cad.pick(rims["edges"], *names), cad.plugin(), measure_body,
    ], design=pilot)
    assert applied["result"]["applicate"] == 24
    assert len({row["feature"] for row in applied["result"]["sedi"]}) == 24
    close(measure(before) - measure(after), 24 * math.pi / 4 * (4.0**2 - 2.5**2) * 5, 0.005)
    print("PASS 24 M2.5 seats on one densely drilled face")


def test_opposite_seats(cad):
    for thickness in (8, 12):
        pilot = cad.root / f"opposite-{thickness}.sc"
        *_, rims = cad.run([
            step("solid.box", length=18, width=16, height=thickness),
            step("solid.hole", position=[8, 8, thickness], diameter=2.5),
            step("model.edges", body="Body1"),
        ], save=pilot)
        pick = cad.pick(rims["edges"], "F1:+z|F2:bore:side:0", "F1:-z|F2:bore:side:0")
        _, quote = cad.run([pick, cad.plugin(solo_quote=True)], design=pilot)
        if thickness == 8:
            assert "seats for holes 1 and 2 overlap" in quote["result"]["impedimenti"][0]
            cad.run([pick, cad.plugin()], design=pilot, expect_error="seats for holes 1 and 2 overlap")
        else:
            assert not quote["result"].get("impedimenti")
            _, applied = cad.run([pick, cad.plugin()], design=pilot)
            assert applied["result"]["applicate"] == 2
    print("PASS opposing seats are blocked only when they would overlap")


def test_captive_nut_pockets(cad):
    pilot = cad.root / "nut-pilots.sc"
    saved = cad.root / "nut-pockets.sc"
    faces = step("model.faces", body="Body1")
    measure_body = step("inspect.measure", bodies=["Body1"])
    inspect = step("document.inspect", timeline="summary")
    *_, rims, planes, before = cad.run([
        step("solid.box", length=38, width=24, height=16),
        step("solid.hole", position=[9, 12, 16], diameter=3.3),
        step("solid.hole", position=[27, 12, 16], diameter=3.3),
        step("model.edges", body="Body1"), faces, measure_body,
    ], save=pilot)
    entries = [
        next(edge for edge in rims["edges"] if edge["name"] == f"F1:+z|F{i}:bore:side:0")
        for i in (2, 3)
    ]

    def pick_face(name):
        face = next(face for face in planes["faces"] if face["name"] == name)
        return {"type": "face", "body": "Body1", "index": face["index"], "point": face["point"]}

    def pockets(**overrides):
        params = {
            "hole_picks": [
                {"type": "edge", "body": "Body1", "index": edge["index"], "point": edge["mid"]}
                for edge in entries
            ],
            "print_face": [pick_face("F1:-z")],
            "nut_size": "M3", "inset_mm": 6, "side_clearance_mm": 0.2, "axial_clearance_mm": 0.2,
            "layer_height_mm": 0.2, "first_layer_mm": 0,
        }
        return cad.plugin("captive-nut-pockets", **(params | overrides))

    before_doc, quote, after_doc = cad.run([inspect, pockets(solo_quote=True), inspect], design=pilot)
    assert before_doc == after_doc
    report = quote["result"]
    assert (report["across_flats_mm"], report["pocket_height_mm"]) == (5.9, 2.6)
    assert [(row["floor_z_mm"], row["roof_z_mm"]) for row in report["pockets"]] == [(7.4, 10), (7.4, 10)]
    assert report["pauses"] == [{
        "before_layer": 51, "printed_z_mm": 10, "layer_z_mm": 10.2,
        "nut_size": "M3", "quantity": 2, "holes": [1, 2],
    }]
    flipped, adaptive, shifted, coarse = [
        row["result"] for row in cad.run([
            pockets(solo_quote=True, print_face=[pick_face("F1:+z")]),
            pockets(solo_quote=True, layer_height_mm=0),
            pockets(solo_quote=True, first_layer_mm=0.24),
            pockets(solo_quote=True, layer_height_mm=1.2, axial_clearance_mm=0),
        ], design=pilot)
    ]
    assert flipped["pauses"][0]["layer_z_mm"] == 8.8
    assert adaptive["pauses"] == [] and adaptive["holes_without_pause"] == []
    assert "10.0" in adaptive["summary"]
    assert shifted["pauses"][0]["printed_z_mm"] == 10.04
    assert shifted["pauses"][0]["layer_z_mm"] == 10.24
    assert coarse["pauses"] == [] and coarse["holes_without_pause"] == [1, 2]
    for opts, message in [
        ({"inset_mm": 14}, "Leave material beyond"),
        ({"print_face": [pick_face("F1:+x")]}, "perpendicular"),
    ]:
        cad.run([pockets(**opts)], design=pilot, expect_error=message)

    applied, after, topology, doc = cad.run([pockets(), measure_body, faces, inspect], design=pilot, save=saved)
    expected = 2 * (math.sqrt(3) / 2 * 5.9**2 - math.pi / 4 * 3.3**2) * 2.6
    close(measure(before) - measure(after), expected, 0.005)
    assert doc["undo"] == 1
    for row in applied["result"]["pockets"]:
        prefix = f'F{row["feature"]}:'
        pocket_faces = [face for face in topology["faces"] if face["name"].startswith(prefix)]
        assert sum(":side:" in face["name"] for face in pocket_faces) == 6
        levels = sorted(face["point"][2] for face in pocket_faces if ":bottom" in face["name"])
        assert len(levels) == 2
        close(levels[0], 7.4)
        close(levels[1], 10)
    assert all(any(face["name"] == name for face in topology["faces"]) for name in ("F1:+z", "F1:-z"))

    reopened, = cad.run([measure_body], design=saved)
    close(measure(reopened), measure(after))
    exported, = cad.run([
        step("file.export_3mf", path=str(cad.root / "nut-pockets.3mf"), lay_on_print_faces=True),
    ], design=saved)
    assert len(exported["print_faces"]) == 1
    _, _, undone, _, redone = cad.run([
        pockets(), step("edit.undo"), measure_body, step("edit.redo"), measure_body,
    ], design=pilot)
    close(measure(undone), measure(before))
    close(measure(redone), measure(after))
    print("PASS enclosed hex nut geometry, grouped pause estimates, print orientation, reopen and undo/redo")


def test_captive_nut_auto(cad):
    pilot = cad.root / "nominal-and-clearance-holes.sc"
    diameters = [3.0, 4.25, 3.30, 2.75, 2.80, 3.25]
    *_, rims, planes = cad.run([
        step("solid.box", length=105, width=24, height=16),
        *(step("solid.hole", position=[10 + 16 * i, 12, 16], diameter=diameter)
          for i, diameter in enumerate(diameters)),
        step("model.edges", body="Body1"),
        step("model.faces", body="Body1"),
    ], save=pilot)
    picks = []
    for i in range(2, 2 + len(diameters)):
        rim = next(edge for edge in rims["edges"] if edge["name"] == f"F1:+z|F{i}:bore:side:0")
        picks.append({"type": "edge", "body": "Body1", "index": rim["index"], "point": rim["mid"]})
    bed = next(face for face in planes["faces"] if face["name"] == "F1:-z")

    def pockets(indices, **overrides):
        return cad.plugin("captive-nut-pockets", **({
            "hole_picks": [picks[i] for i in indices],
            "print_face": [{"type": "face", "body": "Body1", "index": bed["index"], "point": bed["point"]}],
            "inset_mm": 6, "side_clearance_mm": 0.2, "axial_clearance_mm": 0.2,
            "layer_height_mm": 0.2, "first_layer_mm": 0, "solo_quote": True,
        } | overrides))

    (mixed, edge, undersized, chosen) = [record["result"] for record in cad.run([
        pockets([0, 1], solo_quote=False),
        pockets([5], nut_size="Automatic (nominal Ø)"),
        pockets([4]),
        pockets([2], nut_size="M3"),
    ], design=pilot)]
    assert [row["nut_size"] for row in mixed["pockets"]] == ["M3", "M4"]
    assert [row["across_flats_mm"] for row in mixed["pockets"]] == [5.9, 7.4]
    assert [row["pocket_height_mm"] for row in mixed["pockets"]] == [2.6, 3.4]
    assert all("feature" in row for row in mixed["pockets"])
    assert [(event["nut_size"], event["layer_z_mm"]) for event in mixed["pauses"]] == [("M3", 10.2), ("M4", 10.2)]
    assert edge["nut_size"] == chosen["nut_size"] == undersized["nut_size"] == "M3"
    assert edge["holes_with_undersize_bore"] == []
    assert undersized["holes_with_undersize_bore"] == [1]
    cad.run([pockets([2])], design=pilot, expect_error="No nominal size within")
    cad.run([pockets([3])], design=pilot, expect_error="M2.5 and M3 are equally close")
    print("PASS automatic M3/M4, ±0.25 mm threshold, ties, bore warning and manual override")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--cli", default="solidcraft-cli")
    version = json.loads(Path(__file__).resolve().parents[1].joinpath("plugin.json").read_text(encoding="utf-8"))["version"]
    parser.add_argument("--zip", type=Path, default=Path(f"dist/solidinserts-{version}.zip"))
    args = parser.parse_args()
    with tempfile.TemporaryDirectory(prefix="solidinserts-test-") as temporary:
        root = Path(temporary)
        with ZipFile(args.zip) as archive:
            images = {f"assets/solidinserts/{name}.svg" for name in ("standard", "short", "voron")}
            assert set(archive.namelist()) == {
                "solidinserts/plugin.json", "solidinserts/plugin.wasm",
                "solidinserts/LICENSE", "solidinserts/LICENSE-SOLIDCRAFT-WIT",
                *(f"solidinserts/{image}" for image in images),
            }
            project = Path(__file__).resolve().parents[1]
            for image in images:
                assert archive.read(f"solidinserts/{image}") == (project / image).read_bytes()
            assert archive.read("solidinserts/LICENSE") == (project / "LICENSE").read_bytes()
            assert archive.read("solidinserts/LICENSE-SOLIDCRAFT-WIT") == (project / "wit/LICENSE-MIT").read_bytes()
            archive.extractall(root)
        cad = SolidCraft(args.cli, root, root / "solidinserts")
        test_repeated_insert(cad)
        test_opposite_seats(cad)
        test_dense_batch(cad)
        run_tests(cad)
        test_captive_nut_pockets(cad)
        test_captive_nut_auto(cad)


if __name__ == "__main__":
    main()
