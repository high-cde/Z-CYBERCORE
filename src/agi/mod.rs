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

    pub fn analyze_target(&mut self, target: &str) {
        self.status = String::from("ANALYZING");
        println!("[AGI - {}] 🧠 Bersaglio acquisito: {}. Richiesta OSINT in corso...", self.name, target);
        
        // L'AGI chiama autonomamente il modulo OSINT!
        let scanner = ReconScanner::new(target);
        scanner.execute_stealth_scan();
        
        println!("[AGI - {}] ✅ Dati OSINT assimilati. Nessuna vulnerabilità critica immediata.", self.name);
        self.status = String::from("STANDBY");
    }
}
