use z_cybercore::agi::NeuralAgent;

fn main() {
    println!("========================================");
    println!("      Z-CYBERCORE ENGINE INIZIALIZZATO  ");
    println!("========================================");
    
    // Inizializziamo il primo agente neurale
    let mut zero_agent = NeuralAgent::new("Z-PRIME");
    
    println!("Stato iniziale agente: {}", zero_agent.status);
    
    // Assegniamo un target fittizio per il test di rete
    zero_agent.analyze_target("GHOSTNET-NODE-01");
    
    println!("Stato finale agente: {}", zero_agent.status);
    println!("========================================");
    println!("[Z-CYBERCORE] Demone in ascolto...");
}
