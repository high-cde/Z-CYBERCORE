use crate::neural_proxy::TrafficInterceptor;

pub struct NeuralAgent {
    pub name: String,
    pub status: String,
}

impl NeuralAgent {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            status: String::from("STANDBY"),
        }
    }

    /// Applica una policy locale a dati forniti dal chiamante.
    /// Non effettua scansioni, connessioni o operazioni su target esterni.
    pub fn inspect_local_payload(&mut self, packet_data: &str) -> bool {
        self.status = String::from("INSPECTING_LOCAL_INPUT");
        let allowed = TrafficInterceptor::default().analyze_packet(packet_data);
        self.status = String::from("STANDBY");
        allowed
    }
}
