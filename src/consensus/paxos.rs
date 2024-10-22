#[derive(Debug)]
enum PaxosMessage {
    Prepare(u64),     // Phase 1: Prepare message with proposal number
    Promise(u64),     // Phase 1: Promise message from acceptors
    Propose(u64, u64) // Phase 2: Propose a value and proposal number
}

struct PaxosNode {
    id: u64,
    accepted_value: Option<u64>,
    promised_number: u64,
}

impl PaxosNode {
    fn new(id: u64) -> Self {
        PaxosNode {
            id,
            accepted_value: None,
            promised_number: 0,
        }
    }

    fn handle_prepare(&mut self, proposal_number: u64) -> PaxosMessage {
        if proposal_number > self.promised_number {
            self.promised_number = proposal_number;
            println!("Node {} promises proposal {}", self.id, proposal_number);
            PaxosMessage::Promise(self.promised_number)
        } else {
            println!("Node {} rejects proposal {}", self.id, proposal_number);
            PaxosMessage::Promise(self.promised_number)
        }
    }

    fn handle_propose(&mut self, proposal_number: u64, value: u64) {
        if proposal_number >= self.promised_number {
            self.accepted_value = Some(value);
            println!("Node {} accepts value {} for proposal {}", self.id, value, proposal_number);
        }
    }
}
