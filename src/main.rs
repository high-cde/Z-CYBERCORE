use z_cybercore::agi::NeuralAgent;

fn main() {
    println!("========================================");
    println!("      Z-CYBERCORE ENGINE FULL-SPECTRUM  ");
    println!("========================================");
    
    let mut zero_agent = NeuralAgent::new("Z-PRIME");
    
    // Test 1: L'AGI difende la nostra rete da un attacco in ingresso
    println!("\n--- FASE 1: Scudo Neurale (Difesa) ---");
    zero_agent.defend_network("INCOMING_DATA_TCP_PACKET_MALWARE_0x44A");
    
    // Test 2: L'AGI passa all'offensiva su un nodo vulnerabile
    println!("\n--- FASE 2: Contraccolpo (Offensiva) ---");
    zero_agent.analyze_target("ENEMY-SERVER-VULN-01");
    
    println!("\n========================================");
    println!("[Z-CYBERCORE] Sistema 100% Operativo. Demone in ascolto...");
}
