use crate::{smart_vm_core::VMInstruction, state_store::ChainState, blockchain_crypto::BlockchainCrypto};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Contract {
    pub address: String,
    pub owner: String,
    pub bytecode: Vec<VMInstruction>,
    pub deploy_height: u64,
    pub checksum: String,
}

pub struct ContractDeployer {
    chain_state: ChainState,
}

impl ContractDeployer {
    pub fn new(chain_state: ChainState) -> Self {
        Self { chain_state }
    }

    pub fn deploy(
        &mut self,
        owner: String,
        bytecode: Vec<VMInstruction>,
        deploy_height: u64,
    ) -> Result<String, String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        let raw_addr = format!("{}{}{}", owner, deploy_height, timestamp);
        let address = format!("0x{}", &BlockchainCrypto::sha256_hash(raw_addr.as_bytes())[0..40]);
        
        let checksum = BlockchainCrypto::sha256_hash(
            format!("{:?}", bytecode).as_bytes()
        );

        let contract = Contract {
            address: address.clone(),
            owner,
            bytecode,
            deploy_height,
            checksum,
        };

        let code_bytes = bincode::serialize(&contract).map_err(|e| e.to_string())?;
        self.chain_state.store_contract(address.clone(), code_bytes);
        
        Ok(address)
    }

    pub fn get_contract(&self, address: &str) -> Option<Contract> {
        let bytes = self.chain_state.get_contract(address)?;
        bincode::deserialize(bytes).ok()
    }

    pub fn is_owner(&self, address: &str, caller: &str) -> bool {
        self.get_contract(address)
            .map(|c| c.owner == caller)
            .unwrap_or(false)
    }
}
