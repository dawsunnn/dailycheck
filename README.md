# dailycheck

CLI TUI pour gérer une liste de tâches quotidienne, directement dans le terminal.

Les tâches sont sauvegardées dans `~/.dailycheck/YYYY-MM-DD.txt`.

## Build

```sh
cargo build --release
```

## Utilisation

```sh
./dailycheck.exe
```

## Raccourcis

| Touche | Action |
|--------|--------|
| `j` / `↓` | Descendre |
| `k` / `↑` | Monter |
| `Space` | Changer le statut (`[ ]` → `[-]` → `[x]`) |
| `a` | Ajouter une tâche |
| `e` | Modifier la tâche sélectionnée |
| `d` | Supprimer la tâche sélectionnée |
| `h` | Historique (naviguer entre les jours) |
| `l` / `Enter` | Sélectionner une date dans l'historique |
| `q` / `Ctrl+C` | Quitter |
