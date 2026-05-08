# Rust-Phone_Number

Description
-----------

Le projet sert à simuler un annuaire téléphonique.

Utilisation
-----------

1. Prérequis : Rust (rustup + toolchain stable).

2. Construire le projet :

```bash
cargo build --release
```

3. Exécuter (exemple avec un fichier de données) :

```bash
cargo run -- data/01_simple.json
```

4. Lancer les tests :

```bash
cargo test
```

Visualisation
-------------

Les fichiers PlantUML dans `graph/` montrent des représentations des tries construits à partir des exemples. Pour générer les images, utilisez PlantUML (ou un plugin IDE).



*Mention : L’IA générative a servi de support pour l’aide au code et au debugage.*
