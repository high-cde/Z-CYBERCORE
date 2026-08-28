# Architettura

Z-CYBERCORE separa la superficie informativa pubblica dalle attività operative che richiedono autorizzazione, controllo di accesso e audit. Le componenti Rust costituiscono il nucleo applicativo; la landing HTML comunica il posizionamento senza certificare stati operativi.

| Livello | Responsabilità | Vincolo |
|---|---|---|
| Superficie pubblica | Informazioni sul progetto | Nessuna capacità operativa implicita. |
| Nucleo applicativo | Modelli e logica dei componenti Rust | Test e revisione obbligatori. |
| Operazioni autorizzate | Scope, evidenze e job sicuri | Autorizzazione, audit e stop condition. |
| Integrazioni | Fonti esterne configurabili | Dati minimizzati e policy specifiche. |

Ogni cambiamento che attraversa un confine deve dichiarare input, autorizzazione, evidenza prodotta e rollback.
