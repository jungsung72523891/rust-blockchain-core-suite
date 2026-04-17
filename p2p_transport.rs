use std::net::{SocketAddr, TcpListener, TcpStream};
use std::io::{Read, Write};
use std::collections::HashSet;

pub struct P2PTransport {
    pub local_addr: SocketAddr,
    pub peers: HashSet<SocketAddr>,
    listener: TcpListener,
}

impl P2PTransport {
    pub fn new(addr: SocketAddr) -> std::io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;
        
        Ok(Self {
            local_addr: addr,
            peers: HashSet::new(),
            listener,
        })
    }

    pub fn connect_peer(&mut self, peer_addr: SocketAddr) -> std::io::Result<()> {
        if self.peers.contains(&peer_addr) || peer_addr == self.local_addr {
            return Ok(());
        }
        
        let _stream = TcpStream::connect(peer_addr)?;
        self.peers.insert(peer_addr);
        Ok(())
    }

    pub fn broadcast(&self, data: &[u8]) -> std::io::Result<()> {
        for peer in &self.peers {
            if let Ok(mut stream) = TcpStream::connect(peer) {
                let _ = stream.write_all(data);
                let _ = stream.flush();
            }
        }
        Ok(())
    }

    pub fn accept_connections(&mut self) -> std::io::Result<Vec<TcpStream>> {
        let mut streams = Vec::new();
        loop {
            match self.listener.accept() {
                Ok((stream, addr)) => {
                    self.peers.insert(addr);
                    streams.push(stream);
                }
                Err(_) => break,
            }
        }
        Ok(streams)
    }

    pub fn read_message(stream: &mut TcpStream) -> std::io::Result<Vec<u8>> {
        let mut buffer = [0; 4096];
        let n = stream.read(&mut buffer)?;
        Ok(buffer[..n].to_vec())
    }
}
