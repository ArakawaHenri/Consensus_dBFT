use std::collections::HashMap;
use sha2::{Sha256, Digest};

// 定义区块结构体
struct Block {
    index: u32,
    timestamp: u128,
    data: String,
    previous_hash: String,
    hash: String,
    validator: String,
}

impl Clone for Block {
    fn clone(&self) -> Block {
        Block {
            index: self.index,
            timestamp: self.timestamp,
            data: self.data.clone(),
            previous_hash: self.previous_hash.clone(),
            hash: self.hash.clone(),
            validator: self.validator.clone(),
        }
    }
}

impl Block {
    fn calculate_hash(&mut self) {
        let mut hasher = Sha256::new();

        hasher.update(self.index.to_string().as_bytes());
        hasher.update(self.timestamp.to_string().as_bytes());
        hasher.update(self.data.as_bytes());
        hasher.update(self.previous_hash.as_bytes());
        hasher.update(self.validator.as_bytes());

        let hash = hasher.finalize();
        let hash_str = format!("{:x}", hash);

        self.hash = hash_str;
    }
}

// 定义共识状态
struct ConsensusState {
    validators: Vec<String>,
    validator_votes: HashMap<String, bool>,
    current_block: Block,
    current_block_votes: HashMap<String, bool>,
}

// 初始化共识状态
fn init_consensus_state(validators: Vec<String>, current_block: Block) -> ConsensusState {
    let mut validator_votes = HashMap::new();
    let mut current_block_votes = HashMap::new();

    for validator in &validators {
        validator_votes.insert(validator.to_string(), false);
        current_block_votes.insert(validator.to_string(), false);
    }

    ConsensusState {
        validators,
        validator_votes,
        current_block,
        current_block_votes,
    }
}

// 定义验证器投票函数
fn validator_vote(consensus_state: &mut ConsensusState, validator: &str, vote: bool) {
    consensus_state
        .validator_votes
        .insert(validator.to_string(), vote);
}

// 定义当前区块投票函数
fn current_block_vote(consensus_state: &mut ConsensusState, validator: &str, vote: bool) {
    consensus_state
        .current_block_votes
        .insert(validator.to_string(), vote);
}

// 定义共识函数
fn consensus(consensus_state: &mut ConsensusState) -> bool {
    // 检查验证器是否达成共识
    let mut validator_votes = 0;
    let mut validator_consensus = false;

    for vote in consensus_state.validator_votes.values() {
        if *vote {
            validator_votes += 1;
        }
    }

    if validator_votes > consensus_state.validators.len() / 2 {
        validator_consensus = true;
    }

    // 检查当前区块是否达成共识
    let mut current_block_votes = 0;
    let mut current_block_consensus = false;

    for vote in consensus_state.current_block_votes.values() {
        if *vote {
            current_block_votes += 1;
        }
    }

    if current_block_votes > consensus_state.validators.len() / 2 {
        current_block_consensus = true;
    }

    // 如果验证器和当前区块都达成共识，则提交区块
    if validator_consensus && current_block_consensus {
        println!(
            "Block submitted by validator {}",
            consensus_state.current_block.validator
        );
        true
    } else {
        false
    }
}

fn main() {
    // 初始化验证器列表和当前区块
    let validators = vec!["validator1".to_string(), "validator2".to_string(), "validator3".to_string()];
    let mut current_block = Block {
        index: 1,
        timestamp: 123456789,
        data: "This is the first block".to_string(),
        previous_hash: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
        hash: String::new(),
        validator: "validator1".to_string(),
    };

    // 为当前区块计算哈希值
    current_block.calculate_hash();

    // 初始化共识状态
    let mut consensus_state = init_consensus_state(validators.clone(), current_block.clone());

    // 使所有验证器投赞成票
    for validator in &validators {
        validator_vote(&mut consensus_state, validator, true);
    }

    // 使当前区块的验证器投赞成票
    current_block_vote(&mut consensus_state, "validator1", true);

    // 进行共识
    consensus(&mut consensus_state);
}
