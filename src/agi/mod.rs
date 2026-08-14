use crate::osint::ReconScanner;
use crate::offensive::ExploitEngine;

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
        println!("[AGI - {}] 🧠 Bersaglio acquisito: {}", self.name, target);
        
        let scanner = ReconScanner::new(target);
        let is_vulnerable = scanner.execute_stealth_scan();
        
        if is_vulnerable {
            // Cambio di stato: da Analisi a Ingaggio!
            self.status = String::from("ENGAGING");
            println!("[AGI - {}] 🚨 Autorizzazione all'attacco concessa.", self.name);
            
            let weapon = ExploitEngine::new("Z-Lang Zero-Day Injector");
            weapon.deploy(target);
            
            println!("[AGI - {}] 🏴 Operazione completata. Ritorno nell'ombra.", self.name);
        } else {
            println!("[AGI - {}] ✅ Analisi pulita. Attesa nuovi ordini.", self.name);
        }
        
        self.status = String::from("STANDBY");
    }
}
