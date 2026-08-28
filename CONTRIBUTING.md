# Contribuire a Z-CYBERCORE

## Principi

Le modifiche devono rafforzare osservabilità, controllo dello scope, qualità delle evidenze e sicurezza difensiva. Il progetto non accetta contributi che introducano attacchi autonomi, bypass di autenticazione, brute force, persistenza, evasione, denial of service o attività fuori da un'autorizzazione esplicita.

## Flusso di lavoro

Creare un branch focalizzato, descrivere l'obiettivo in un'issue o in una pull request e mantenere i cambiamenti piccoli e revisionabili. Aggiornare README, Wiki o changelog se cambia un'interfaccia, un flusso operativo o un confine di sicurezza.

## Verifica locale

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
```

## Revisione

Ogni pull request deve spiegare la verifica svolta, gli impatti sulla superficie operativa e le misure di rollback. Non includere segreti, dati di target o evidenze non sanificate.
