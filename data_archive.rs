use std::fs::{File, create_dir_all};
use std::path::Path;
use flate2::write::GzEncoder;
use flate2::Compression;
use crate::basic_blockchain::Block;

pub struct DataArchive {
    archive_dir: String,
    batch_size: u64,
}

impl DataArchive {
    pub fn new(archive_dir: String, batch_size: u64) -> Self {
        if !Path::new(&archive_dir).exists() {
            let _ = create_dir_all(&archive_dir);
        }
        Self { archive_dir, batch_size }
    }

    pub fn archive_blocks(&self, blocks: &[Block], batch_id: u64) -> std::io::Result<String> {
        let filename = format!("{}/block_batch_{}.gz", self.archive_dir, batch_id);
        let file = File::create(&filename)?;
        let encoder = GzEncoder::new(file, Compression::default());
        
        serde_json::to_writer(encoder, blocks)?;
        Ok(filename)
    }

    pub fn load_archive(&self, batch_id: u64) -> std::io::Result<Vec<Block>> {
        let filename = format!("{}/block_batch_{}.gz", self.archive_dir, batch_id);
        let file = File::open(filename)?;
        let decoder = flate2::read::GzDecoder::new(file);
        
        let blocks = serde_json::from_reader(decoder)?;
        Ok(blocks)
    }

    pub fn should_archive(height: u64, batch_size: u64) -> bool {
        height % batch_size == 0 && height > 0
    }

    pub fn get_batch_id(height: u64, batch_size: u64) -> u64 {
        height / batch_size
    }
}
