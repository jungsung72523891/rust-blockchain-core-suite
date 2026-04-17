use std::net::SocketAddr;
use std::collections::HashSet;
use std::time::{Instant, Duration};

pub struct PeerDiscovery {
    bootstrap_nodes: Vec<SocketAddr>,
    discovered_peers: HashSet<SocketAddr>,
    last_discovery: Instant,
    discovery_interval: Duration,
}

impl PeerDiscovery {
    pub fn new(bootstrap_nodes: Vec<SocketAddr>, interval_secs: u64) -> Self {
        Self {
            bootstrap_nodes,
            discovered_peers: HashSet::new(),
            last_discovery: Instant::now() - Duration::from_secs(interval_secs),
            discovery_interval: Duration::from_secs(interval_secs),
        }
    }

    pub fn can_discover(&self) -> bool {
        self.last_discovery.elapsed() >= self.discovery_interval
    }

    pub fn run_discovery(&mut self) -> Vec<SocketAddr> {
        self.last_discovery = Instant::now();
        for node in &self.bootstrap_nodes {
            self.discovered_peers.insert(*node);
        }
        self.discovered_peers.iter().cloned().collect()
    }

    pub fn add_peer(&mut self, addr: SocketAddr) {
        self.discovered_peers.insert(addr);
    }

    pub fn remove_peer(&mut self, addr: &SocketAddr) {
        self.discovered_peers.remove(addr);
    }

    pub fn get_all_peers(&self) -> Vec<SocketAddr> {
        self.discovered_peers.iter().cloned().collect()
    }

    pub fn peer_count(&self) -> usize {
        self.discovered_peers.len()
    }
}
