use crate::neural_proxy::TrafficInterceptor;
use crate::offensive::ExploitEngine;
use crate::osint::ReconScanner;

pub struct NeuralAgent {
    pub name: String,
    pub status: String,
}

impl NeuralAgent {
    pub fn new(name: &str) -> Self {
        NeuralAgent {
            name: name.to_string(),
            status: String::from("STANDBY"),
        }
    }

    // Modalità Attacco (già implementata)
    pub fn analyze_target(&mut self, target: &str) {
        self.status = String::from("ENGAGING");
        println!("[AGI - {}] 🧠 Bersaglio acquisito: {}", self.name, target);

        let scanner = ReconScanner::new(target);
        if scanner.execute_stealth_scan() {
            println!(
                "[AGI - {}] 🚨 Autorizzazione all'attacco concessa.",
                self.name
            );
            let weapon = ExploitEngine::new("Z-Lang Zero-Day Injector");
            weapon.deploy(target);
        } else {
            println!(
                "[AGI - {}] ✅ Analisi pulita. Nessuna azione offensiva richiesta.",
                self.name
            );
        }
        self.status = String::from("STANDBY");
    }

    // NUOVA: Modalità Difesa (Usa il Proxy)
    pub fn defend_network(&mut self, packet_data: &str) {
        self.status = String::from("DEFENDING");
        println!(
            "[AGI - {}] 🛡️ Modalità Difesa Attiva. Scansione perimetrale...",
            self.name
        );

        let proxy = TrafficInterceptor::new();
        let is_safe = proxy.analyze_packet(packet_data);

        if !is_safe {
            println!(
                "[AGI - {}] ⚡ Minaccia neutralizzata al confine. Rete ZDOS protetta.",
                self.name
            );
        }

        self.status = String::from("STANDBY");
    }
}
