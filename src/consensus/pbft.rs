use bls_signatures::{PrivateKey, PublicKey, Signature};


#[derive(Debug)]
enum PBFTMessage {
    PrePrepare(u64, u64), // Proposal by the leader
    Prepare(u64),         // Prepare phase message
    Commit(u64),          // Commit phase message
}

struct PBFTNode {
    id: u64,
    prepared: bool,
    committed: bool,
}
//pub fn initialize_pbft_consensus() {
   // println!("Initializing PBFT consensus mechanism...");
    // Implement PBFT node setup, message passing, consensus...
    // More PBFT consensus logic...
//}

#[derive(Debug)]
struct PBFTBatch {
    requests: Vec<u64>, // Batch of client requests (simplified as request IDs)
}


impl PBFTNode {
    fn new(id: u64) -> Self {
        PBFTNode {
            id,
            prepared: false,
            committed: false,
        }
    }

    fn handle_pre_prepare(&mut self, view: u64, value: u64) -> PBFTMessage {
        println!("Node {} received PrePrepare message: view {}, value {}", self.id, view, value);
        PBFTMessage::Prepare(value)
    }

    fn handle_prepare(&mut self, value: u64) -> PBFTMessage {
        if !self.prepared {
            self.prepared = true;
            println!("Node {} prepared for value {}", self.id, value);
        }
        PBFTMessage::Commit(value)
    }

    fn handle_commit(&mut self, value: u64) {
        if self.prepared && !self.committed {
            self.committed = true;
            println!("Node {} committed to value {}", self.id, value);
        }
    }

    //Message Batching in PBFT
    fn handle_pre_prepare_batch(&mut self, batch: PBFTBatch) -> PBFTMessage {
        println!("Node {} received PrePrepare batch: {:?}", self.id, batch.requests);
        PBFTMessage::Prepare(batch.requests.len() as u64)
    }

    fn handle_prepare_batch(&mut self, batch_size: u64) -> PBFTMessage {
        if !self.prepared {
            self.prepared = true;
            println!("Node {} prepared for batch of size {}", self.id, batch_size);
        }
        PBFTMessage::Commit(batch_size)
    }

    fn handle_commit_batch(&mut self, batch_size: u64) {
        if self.prepared && !self.committed {
            self.committed = true;
            println!("Node {} committed to batch of size {}", self.id, batch_size);
        }
    }

    //Pipelining Phases
    async fn pipeline_consensus(&mut self, batches: Vec<PBFTBatch>) {
        // Start PrePrepare for all batches concurrently
        let mut prepare_futures = vec![];
        for batch in batches {
            let pre_prepare_msg = self.handle_pre_prepare_batch(batch.clone());
            let future = self.handle_prepare_pipelined(pre_prepare_msg).await;
            prepare_futures.push(future);
        }

        // Await all Prepare phases in parallel
        for future in prepare_futures {
            future.await.unwrap();
        }

        println!("Node {} completed pipelined consensus for all batches", self.id);
    }

    async fn handle_prepare_pipelined(&mut self, pre_prepare_msg: PBFTMessage) -> tokio::task::JoinHandle<()> {
        // Simulate processing the Prepare phase in parallel
        tokio::spawn(async move {
            if let PBFTMessage::Prepare(batch_size) = pre_prepare_msg {
                println!("Node {} preparing for batch of size {} in pipelined mode", self.id, batch_size);
                tokio::time::sleep(Duration::from_millis(200)).await;
                println!("Node {} completed Prepare phase for batch of size {}", self.id, batch_size);
            }
        })
    }

    //Signature Aggregation

    fn aggregate_signatures(&self, signatures: Vec<Signature>) -> Signature {
        // Aggregate multiple signatures into a single signature
        let aggregated_sig = Signature::aggregate(&signatures).unwrap();
        println!("Node {} aggregated {} signatures", self.id, signatures.len());
        aggregated_sig
    }

    fn verify_aggregated_signature(&self, agg_signature: &Signature, public_keys: Vec<PublicKey>, message: &[u8]) -> bool {
        // Verify the aggregated signature
        let is_valid = agg_signature.verify(message, &public_keys);
        println!("Node {} verified aggregated signature: {}", self.id, is_valid);
        is_valid
    }

    //Adaptive Timeout Mechanism
    fn adaptive_timeout(&self, network_stable: bool) -> Duration {
        let timeout = if network_stable {
            Duration::from_millis(200) // Shorter timeout for stable networks
        } else {
            Duration::from_secs(1) // Longer timeout for unstable networks
        };
        println!("Node {} adjusted timeout to {:?}", self.id, timeout);
        timeout
    }
}
