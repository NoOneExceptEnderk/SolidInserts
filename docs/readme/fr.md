# SolidInserts

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="../../assets/branding/solidinserts-logo-inverse.svg">
  <img src="../../assets/branding/solidinserts-logo.svg" width="390" alt="SolidInserts">
</picture>

[English](../../README.md) · [Italiano](../../README.it.md) · [Español](es.md) · **Français** · [Deutsch](de.md) · [Português (Portugal)](pt.md) · [Русский](ru.md) · [简体中文](zh-CN.md) · [தமிழ்](ta.md)

Créez des logements pour inserts filetés à chaud dans des trous existants avec SolidCraft Desktop.

SolidInserts positionne chaque logement à partir de l’arête circulaire du trou. Choisissez une série CNC Kitchen et une taille de vis pour usiner plusieurs logements en une seule opération. Les esquisses et les enlèvements de matière restent modifiables dans l’historique de SolidCraft. Une seule annulation retire tous les usinages de l’opération.

## Essayer SolidInserts

1. Utilisez [SolidCraft Desktop 0.44.1](https://github.com/tangency-labs/solidcraft/releases/tag/v0.44.1). Téléchargez le ZIP de l’extension depuis les [versions publiées](https://github.com/NoOneExceptEnderk/SolidInserts/releases). Vous pouvez aussi compiler l’extension avec les commandes ci-dessous pour produire `dist/solidinserts-<version>.zip`.
2. Installez le ZIP depuis **Fichier → Plugins…**. Pour une installation manuelle, extrayez le dossier `solidinserts` complet, avec ses ressources, dans le dossier des extensions de votre système.

   | Système | Dossier des extensions |
   | --- | --- |
   | Linux | `$XDG_DATA_HOME/solidcraft/plugins/` (généralement `~/.local/share/solidcraft/plugins/`) |
   | Windows | `%LOCALAPPDATA%\SolidCraft\plugins\` |
   | macOS | `~/Library/Application Support/SolidCraft/plugins/` |

3. Ouvrez l’[éprouvette avec des trous de Ø3 et Ø4 mm](../../examples/pilot-holes.sc). Ouvrez **Model → Plugins → Insert Seats**. Dans **Hole rims**, sélectionnez les arêtes circulaires du côté où les inserts seront posés. Choisissez la série et le filetage, vérifiez l’aperçu et cliquez sur **Run**. L’[éprouvette terminée](../../examples/insert-seats.sc) montre le résultat.

## Prévisualiser les enlèvements de matière

La version officielle de SolidCraft 0.44.1 affiche l’aperçu 3D sans correctif. Il se met à jour lorsque vous sélectionnez des arêtes ou modifiez les réglages. **Cancel** ferme l’aperçu.

## Inserts pris en charge

SolidInserts est une extension indépendante, sans affiliation avec CNC Kitchen. Les dimensions des logements pour les inserts indiqués sont basées sur la fiche dimensionnelle publiée par CNC Kitchen.

| Série CNC Kitchen | Tailles de vis | Exemple M3 (diamètre × profondeur) |
| --- | --- | --- |
| Standard | M2, M2,5, M3, M4, M5, M6, M8, M10 | Ø4 × 6,7 mm |
| Short Version | M3, M4, M5 | Ø4 × 4 mm |
| VORON M3 × 5 × 4 | M3 | Ø4,4 × 5 mm |

Le mode automatique associe le diamètre du trou à un diamètre nominal de filetage à ±0,05 mm près. Pour un trou de passage avec jeu ou un avant-trou de taraudage, choisissez la taille de vis à la main. Par exemple, choisissez M3 pour un trou de passage de Ø3,2 mm destiné à une vis M3. La série s’applique à toute la sélection. Le mode automatique peut choisir un filetage différent pour chaque trou.

SolidInserts accepte les trous cylindriques dont l’entrée circulaire se trouve sur une face plane. Il vérifie la matière autour du logement et la profondeur disponible. Dans un trou débouchant, il conserve une partie du perçage d’origine. La profondeur du logement correspond à la longueur de l’insert plus 1 mm. La compensation démarre à zéro et agit uniquement sur le diamètre. Imprimez une éprouvette pour ajuster le montage. [Dimensions et conseils de conception CNC Kitchen](https://cnckitchen.com/s/CNCKitchen_Heat-Set-Insert-Dimensions-and-Design-Guidelines.pdf).

## Compilation et tests

Installez Rust avec Cargo, Bash, Python 3 et [wasm-tools](https://github.com/bytecodealliance/wasm-tools). Exécutez les commandes à la racine du dépôt. La compilation produit le ZIP et sa somme SHA-256.

```sh
rustup target add wasm32-unknown-unknown
cargo test --locked
./build.sh
python3 tests/solidcraft.py --cli /path/to/solidcraft-cli
```

Le test d’intégration utilise la CLI de SolidCraft 0.44.1. Dans les scripts CLI ou MCP, appelez `applica` via `plugin.run` avec `solo_quote: true` pour obtenir les cotes sans modifier le modèle. SolidInserts utilise l’[environnement d’exécution des extensions de SolidCraft](https://github.com/tangency-labs/solidcraft/blob/v0.44.1/docs/plugin-runtime.md).

## Licence et soutien

SolidInserts est une extension indépendante distribuée sous [licence MIT](../../LICENSE). [Crédits](../../NOTICE.md) · [Soutenir le développement sur Ko-fi](https://ko-fi.com/enderk).
