# Z-CYBERCORE ⚔️🧠
**Il nucleo di Security Operations per l’ecosistema ZDOS**

Z-CYBERCORE coordina analisi difensive autorizzate, asset intelligence, evidenze e workflow di bug bounty attraverso una superficie pubblica e una War Room autenticata. L’architettura privilegia policy esplicite, provenance e audit rispetto all’automazione indiscriminata.

### 🛡️ Moduli Principali

* **Control Plane:** programmi, scope, asset, finding, evidenze e approvazioni.
* **OSINT:** raccolta da fonti pubbliche con origine, timestamp e confidence.
* **Asset Intelligence:** enrichment Shodan server-side limitato ad asset autorizzati.
* **Safe Probes:** DNS metadata, TLS metadata, security headers e HTTP baseline.
* **ZLang Orchestrator:** piani dichiarativi allowlisted, senza shell arbitraria.
* **Evidence Chain:** risultati hashati e audit trail per ricostruire ogni operazione.

### 🔒 Safety Boundary

La baseline non include exploit, brute force, credential attack, evasione, persistence, DoS, fuzzing aggressivo o target fuori scope. Ogni job richiede autenticazione, scope attivo, limiti, approvazione umana e condizioni di stop.

### 🌐 Superfici

* **War Room:** https://warroom.zdos-sec.it
* **Security landing:** https://zdos-sec.it
* **ZDOS Hub:** https://zdos-hub.it
* **Stato tecnico:** [SECOPS-STATUS.md](SECOPS-STATUS.md)

*Forgiato per il controllo verificabile.*
