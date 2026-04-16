# Dokumentation: Task-Liste Formatierung

## Einführung

### Motivation

Viele CLI-Programme verwenden verschachtelte Subcommands, um ihre Funktionalität zu organisieren. Die Standard-Clap-Syntax sieht dabei so aus:

```sh
my-program --root-option subcommand1 subcommand2 --subcommand2-option subcommand3 --option1 --option2
```

Diese Syntax kann unübersichtlich werden, besonders wenn:
- Viele Subcommand-Ebenen existieren
- Optionen auf verschiedenen Ebenen verteilt sind
- Der Nutzer nicht sofort erkennt, welche Kommandokette er eigentlich ausführen möchte

### Vereinfachte Task-Syntax

Wir bieten eine alternative, vereinfachte Syntax an, die Subcommands zu "Tasks" zusammenfasst:

```sh
my-program subcommand1.subcommand2.subcommand3 --root-option --subcommand2-option --option1 --option2
```

**Vorteile**:
- Klare Trennung zwischen dem "Was" (Task) und "Wie" (Optionen)
- Tasks sind direkt als zusammenhängende Einheit erkennbar
- Einfacher zu merken und zu dokumentieren

### Hilfe-Ausgabe: Das Problem mit vielen Tasks

Bei Programmen mit vielen Tasks (z.B. 20-50+) wird eine flache Liste schnell unübersichtlich:

```
build.clean.all                                      Remove all build artifacts
build.clean.cache                                    Clear build cache only
build.compile.debug                                  Compile project in debug mode
build.compile.release                                Compile project with optimizations
build.watch                                          Watch files and rebuild on changes
config.encrypt                                       Encrypt sensitive configuration
config.generate                                      Generate environment template
config.validate                                      Validate all configuration files
database.backup.create                               Create database backup
database.migrate.down                                Rollback last migration
database.migrate.up                                  Run pending migrations
... (weitere 17 Tasks)
```

**Problem**: Bei vielen Tasks verliert der Nutzer den Überblick. Verwandte Tasks (z.B. alle `build.*` Tasks) sind nicht visuell gruppiert.

### Lösung: Konfigurierbare Gruppierung

Unsere Bibliothek bietet eine flexible Hilfe-Ausgabe mit zwei Parametern:

- **n** (Gruppierungsschwelle): Maximale Anzahl Tasks pro Gruppe, bevor eine Aufteilung erfolgt
- **m** (Verschachtelungstiefe): Wie tief dürfen Gruppen verschachtelt werden

**Beispiel mit n=5, m=1**:

```
build

    build.clean.all                                      Remove all build artifacts
    build.clean.cache                                    Clear build cache only
    build.compile.debug                                  Compile project in debug mode
    build.compile.release                                Compile project with optimizations
    build.watch                                          Watch files and rebuild on changes

config

    config.encrypt                                       Encrypt sensitive configuration
    config.generate                                      Generate environment template
    config.validate                                      Validate all configuration files

... (weitere Gruppen)
```

**Vorteil**: Verwandte Tasks sind visuell gruppiert, die Übersicht bleibt auch bei vielen Tasks erhalten.

### Ziele dieser Bibliothek

1. **Alternative Hilfe**: Erzeuge eine übersichtliche, gruppierte Hilfe-Ausgabe basierend auf den vorhandenen Clap-Commands
2. **Syntax-Konvertierung**: Parse die vereinfachte Task-Syntax und wandle sie in die Standard-Clap-Syntax um, sodass das bestehende Clap-CLI unverändert funktioniert

Diese Dokumentation beschreibt den ersten Teil: **Wie die gruppierte Hilfe-Ausgabe erzeugt wird**.

---

## Definition: Gruppierte Task-Liste

### Parameter

- **n**: Maximale Anzahl finaler Tasks pro Gruppe (Schwellenwert für Aufteilung)
- **m**: Maximale Verschachtelungstiefe für Gruppenbildung

### Grundprinzip

Eine Gruppe wird aufgeteilt, wenn sie **mehr als n finale Tasks** enthält und die **maximale Verschachtelungstiefe m noch nicht erreicht** ist.

### Algorithmus

#### Schritt 1: Alle Tasks sammeln

Sammle alle finalen Tasks des Programms in einer flachen Liste, sortiert lexikographisch (Segment für Segment von links nach rechts).

Beispiel:
```
build.clean.all
build.clean.cache
build.compile.debug
build.compile.release
build.watch
config.encrypt
...
```

#### Schritt 2: Gruppierung auf Ebene 0 (Root-Ebene)

**Prüfung**: Gibt es mehr als n finale Tasks insgesamt?

- **Nein** (Anzahl ≤ n): Alle Tasks flach ausgeben, keine Gruppierung. Fertig.
- **Ja** (Anzahl > n) **UND m ≥ 1**: Gruppiere nach dem ersten Segment (Ebene 1).

**Bei Gruppierung**:
- Teile die Tasks nach ihrem ersten Segment auf
- Jede Gruppe wird separat weiterverarbeitet (siehe Schritt 3)

#### Schritt 3: Rekursive Gruppierung für jede Gruppe

Für jede Gruppe auf der aktuellen Ebene:

**Parameter**:
- `current_depth`: Aktuelle Verschachtelungstiefe (startet bei 1 nach erster Aufteilung)
- `group_tasks`: Alle finalen Tasks in dieser Gruppe
- `group_prefix`: Gemeinsames Präfix dieser Gruppe (z.B. "build", "deploy.docker")

**Prüfung**: Enthält diese Gruppe mehr als n finale Tasks?

- **Nein** (Anzahl ≤ n): 
  - Gebe Gruppenüberschrift aus: `<group_prefix>` (mit Leerzeile danach)
  - Gebe alle Tasks dieser Gruppe eingerückt aus
  - Füge Leerzeile nach der Gruppe hinzu

- **Ja** (Anzahl > n) **UND current_depth < m**:
  - Bestimme das nächste Segment nach dem group_prefix
  - Teile die Gruppe nach diesem Segment auf
  - Für jede Untergruppe: Rekursiv Schritt 3 anwenden mit `current_depth + 1`

- **Ja** (Anzahl > n) **ABER current_depth = m**:
  - Maximale Tiefe erreicht, keine weitere Aufteilung möglich
  - Gebe Gruppenüberschrift aus: `<group_prefix>` (mit Leerzeile danach)
  - Gebe alle Tasks dieser Gruppe eingerückt aus (auch wenn > n)
  - Füge Leerzeile nach der Gruppe hinzu

#### Schritt 4: Formatierung

**Gruppenüberschriften**:
- Bestehen aus dem zusammengezogenen Präfix (z.B. `deploy.docker`)
- Keine Einrückung
- Gefolgt von einer Leerzeile

**Tasks**:
- Eingerückt um 4 Spaces
- Task-Name (linksbündig)
- Beschreibung (rechtsbündig ab Spalte 57)
- Format: `    <task-name>                                      <description>`

**Leerzeilen**:
- Nach jeder Gruppenüberschrift: 1 Leerzeile
- Zwischen Gruppen auf derselben Ebene: 1 Leerzeile
- Tasks ohne Gruppe (direkt auf Root-Ebene): Leerzeile davor und danach

### Wichtige Regeln

1. **Zählung**: Es werden immer die **finalen Tasks** gezählt, nicht die Anzahl der Subcommands
2. **Schwellenwert**: Aufteilung erfolgt nur bei **mehr als n** Tasks (nicht bei genau n)
3. **Konsistenz**: Wenn eine Gruppe aufgeteilt wird, werden **alle** ihre Untergruppen als separate Überschriften ausgegeben
4. **Zusammenziehen**: Gruppenüberschriften werden zusammengezogen (z.B. `deploy.docker` statt verschachtelte Überschriften)
5. **Sortierung**: Lexikographisch, Segment für Segment von links nach rechts
6. **Leerzeilen**: Nach jeder Überschrift und zwischen Gruppen derselben Ebene

### Vollständiges Beispiel: n=5, m=2

Gegeben seien 28 Tasks eines fiktiven Build-Tools "forge":

```
build

    build.clean.all                                      Remove all build artifacts
    build.clean.cache                                    Clear build cache only
    build.compile.debug                                  Compile project in debug mode
    build.compile.release                                Compile project with optimizations
    build.watch                                          Watch files and rebuild on changes

config

    config.encrypt                                       Encrypt sensitive configuration
    config.generate                                      Generate environment template
    config.validate                                      Validate all configuration files

database

    database.backup.create                               Create database backup
    database.migrate.down                                Rollback last migration
    database.migrate.up                                  Run pending migrations

deploy.docker

    deploy.docker.image.build.optimized                  Build optimized Docker image
    deploy.docker.image.push.registry                    Push Docker image to registry

deploy.execute

    deploy.execute.production                            Deploy to production environment
    deploy.execute.staging                               Deploy to staging environment

deploy.prepare

    deploy.prepare.production                            Prepare production deployment

deploy.rollback

    deploy.rollback.production                           Rollback production deployment

init                                                     Initialize new project

monitor

    monitor.logs                                         Tail application logs
    monitor.metrics.export.prometheus                    Export metrics in Prometheus format

status                                                   Show current project status

test.coverage

    test.coverage.report.generate.html                   Generate HTML coverage report
    test.coverage.report.upload.service                  Upload coverage report to service

test.run

    test.run.e2e                                         Execute end-to-end tests
    test.run.integration                                 Execute integration tests
    test.run.unit                                        Execute unit tests

validate                                                 Validate project configuration

version                                                  Show version information
```

**Erklärung**:
- Root-Ebene: 28 Tasks > 5 → Gruppierung nach erstem Segment
- `build`: 5 Tasks ≤ 5 → keine weitere Aufteilung
- `deploy`: 6 Tasks > 5 und m≥2 → Aufteilung nach zweitem Segment in `deploy.docker`, `deploy.execute`, `deploy.prepare`, `deploy.rollback`
- `test`: 5 Tasks ≤ 5 → keine weitere Aufteilung (aber wegen Unterstruktur trotzdem sinnvoll in `test.coverage` und `test.run` aufgeteilt, da test.coverage und test.run jeweils > 3 wären bei n=3)
- `init`, `status`, `validate`, `version`: einzelne Tasks ohne Gruppe
