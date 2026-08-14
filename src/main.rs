use z_cybercore::agi::NeuralAgent;

fn main() {
    println!("========================================");
    println!("      Z-CYBERCORE ENGINE ARMATO         ");
    println!("========================================");
    
    let mut zero_agent = NeuralAgent::new("Z-PRIME");
    
    // Test 1: Bersaglio Sicuro
    println!("\n--- TEST 1: Nodo Sicuro ---");
    zero_agent.analyze_target("GHOSTNET-NODE-01");
    
    // Test 2: Bersaglio Vulnerabile (l'AGI attaccherà)
    println!("\n--- TEST 2: Server Vulnerabile ---");
    zero_agent.analyze_target("CORP-SERVER-VULN-99");
    
    println!("\n========================================");
    println!("[Z-CYBERCORE] Demone in ascolto...");
}
