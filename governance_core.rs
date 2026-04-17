use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub proposer: String,
    pub start_time: u128,
    pub end_time: u128,
    pub for_votes: u64,
    pub against_votes: u64,
    pub executed: bool,
}

#[derive(Debug, Clone)]
pub struct Governance {
    proposals: HashMap<String, Proposal>,
    votes: HashMap<String, HashMap<String, bool>>,
    voting_period: u128,
}

impl Governance {
    pub fn new(voting_hours: u64) -> Self {
        Self {
            proposals: HashMap::new(),
            votes: HashMap::new(),
            voting_period: voting_hours as u128 * 3600000,
        }
    }

    pub fn create_proposal(&mut self, id: String, title: String, description: String, proposer: String) {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        let proposal = Proposal {
            id: id.clone(),
            title,
            description,
            proposer,
            start_time: now,
            end_time: now + self.voting_period,
            for_votes: 0,
            against_votes: 0,
            executed: false,
        };
        self.proposals.insert(id, proposal);
    }

    pub fn vote(&mut self, proposal_id: &str, voter: String, support: bool) -> bool {
        let proposal = match self.proposals.get_mut(proposal_id) {
            Some(p) => p,
            None => return false,
        };

        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        if now > proposal.end_time {
            return false;
        }

        let user_votes = self.votes.entry(proposal_id.to_string()).or_insert_with(HashMap::new);
        if user_votes.contains_key(&voter) {
            return false;
        }

        user_votes.insert(voter, support);
        if support {
            proposal.for_votes += 1;
        } else {
            proposal.against_votes += 1;
        }
        true
    }

    pub fn execute_proposal(&mut self, proposal_id: &str) -> bool {
        let proposal = self.proposals.get_mut(proposal_id)?;
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        
        if now < proposal.end_time || proposal.executed || proposal.for_votes <= proposal.against_votes {
            return false;
        }
        
        proposal.executed = true;
        true
    }
}
