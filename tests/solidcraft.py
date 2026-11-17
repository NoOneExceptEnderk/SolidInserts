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
        run_tests(SolidCraft(args.cli, root, root / "solidinserts"))


if __name__ == "__main__":
    main()
