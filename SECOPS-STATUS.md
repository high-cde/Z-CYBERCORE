# Z-CYBERCORE · Security Operations Status

## Ruolo

`Z-CYBERCORE` è il nucleo di security operations dell’ecosistema ZDOS. La superficie pubblica presenta il posizionamento; la War Room autenticata gestisce programmi, scope, asset, finding, evidenze e job autorizzati.

## Stato verificabile

| Capability | Stato | Confine |
|---|---|---|
| War Room autenticata | `DEPLOYED` | accesso separato su `warroom.zdos-sec.it` |
| Asset/scope/finding/evidenze | `CONTROLLED-WRITE` | ruoli e audit richiesti |
| Safe probes | `AUTHORIZED` | DNS, TLS, header, HTTP baseline |
| OSINT/Shodan | `CONFIGURABLE` | fonti pubbliche; Shodan server-side su asset autorizzati |
| Runner | `GATED` | approvazione esplicita e stop condition |

## Safety boundary

Non descrivere questo progetto come strumento per attacchi autonomi. Exploit, brute force, credential attack, evasione, persistence, DoS, fuzzing aggressivo e target fuori scope sono esclusi dalla baseline operativa.

## Collegamenti

- War Room: https://warroom.zdos-sec.it
- Security landing: https://zdos-sec.it
- ZDOS Hub: https://zdos-hub.it
- ZDOS Lab: https://github.com/high-cde/ZDOS-lab-v1

## Evoluzione prevista

Aggiungere solo capability che abbiano policy, test, audit, scope e rollback documentati. La potenza della piattaforma deve derivare da correlazione, qualità delle evidenze e automazione controllata.
