use std::collections::{HashSet, HashMap};
use std::net::SocketAddr;
use std::time::{Instant, Duration};

pub struct PeerManager {
    peers: HashSet<SocketAddr>,
    banned_peers: HashSet<SocketAddr>,
    peer_heartbeat: HashMap<SocketAddr, Instant>,
    heartbeat_timeout: Duration,
}

impl PeerManager {
    pub fn new(heartbeat_secs: u64) -> Self {
        Self {
            peers: HashSet::new(),
            banned_peers: HashSet::new(),
            peer_heartbeat: HashMap::new(),
            heartbeat_timeout: Duration::from_secs(heartbeat_secs),
        }
    }

    pub fn add_peer(&mut self, addr: SocketAddr) -> bool {
        if self.banned_peers.contains(&addr) {
            return false;
        }
        self.peers.insert(addr);
        self.peer_heartbeat.insert(addr, Instant::now());
        true
    }

    pub fn remove_peer(&mut self, addr: &SocketAddr) {
        self.peers.remove(addr);
        self.peer_heartbeat.remove(addr);
    }

    pub fn ban_peer(&mut self, addr: SocketAddr) {
        self.peers.remove(&addr);
        self.banned_peers.insert(addr);
    }

    pub fn unban_peer(&mut self, addr: &SocketAddr) {
        self.banned_peers.remove(addr);
    }

    pub fn update_heartbeat(&mut self, addr: &SocketAddr) {
        if self.peers.contains(addr) {
            self.peer_heartbeat.insert(*addr, Instant::now());
        }
    }

    pub fn prune_inactive_peers(&mut self) -> Vec<SocketAddr> {
        let now = Instant::now();
        let inactive: Vec<SocketAddr> = self.peer_heartbeat
            .iter()
            .filter(|(_, &time)| now.duration_since(time) > self.heartbeat_timeout)
            .map(|(&addr, _)| addr)
            .collect();

        for addr in &inactive {
            self.remove_peer(addr);
        }
        inactive
    }

    pub fn get_all_peers(&self) -> Vec<SocketAddr> {
        self.peers.iter().cloned().collect()
    }
}
