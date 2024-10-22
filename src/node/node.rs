use std::collections::HashMap;
use tokio::time::{sleep, Duration};
use ring::{rand, signature};
use ring::signature::{Ed25519KeyPair, Signature, KeyPair};
use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-GCM encryption
use aes_gcm::aead::{Aead, NewAead};
use ring::signature::KeyPair;


#[derive(Debug, PartialEq)]
enum Role {
    Leader,
    Follower,
    Candidate,
}

#[derive(Clone, Debug)]
struct Transaction {
    id: u64,
    data: String,
}

#[derive(Clone, Debug)]
struct Snapshot {
    index: u64,
    state: HashMap<String, String>, // Simulate world state as a key-value store
}

struct Node {
    id: u64,
    role: Role,
    term: u64,
    voted_for: Option<u64>, // Node ID voted for in current term
    log: Vec<Transaction>,  // Log of transactions
    peers: Vec<u64>,        // List of peer node IDs
}

#[derive(Debug)]
enum ReconfigAction {
    AddNode(u64),    // Add a new node
    RemoveNode(u64), // Remove an existing node
}

fn compress_logs(log_entries: &Vec<Transaction>) -> Vec<u8> {
    let log_data = serde_json::to_string(log_entries).unwrap();
    println!("Compressing log data of size: {}", log_data.len());

    // Simulate compression (e.g., using zlib or a similar library)
    log_data.as_bytes().to_vec() // For simplicity, just return the byte array here
}


impl Node {

    fn new(id: u64, peers: Vec<u64>) -> Self {
        Node {
            id,
            role: Role::Follower,
            term: 0,
            voted_for: None,
            log: vec![],
            peers,
        }
    }
    //Dynamic Leader Election with Reconfiguration
    fn start_election(&mut self) {
        println!("Node {} starting election for term {}", self.id, self.term);
        self.role = Role::Candidate;
        self.term += 1;
        self.voted_for = Some(self.id);

        let votes = self.request_votes();
        if votes >= self.quorum() {
            self.role = Role::Leader;
            println!("Node {} became the leader for term {}", self.id, self.term);
        } else {
            self.role = Role::Follower;
            println!("Node {} returned to follower state", self.id);
        }
    }

    fn quorum(&self) -> u64 {
        (self.peers.len() as u64 + 1) / 2 + 1
    }

    fn request_votes(&self) -> u64 {
        // Simulate sending vote requests to all peers
        println!("Node {} requesting votes from peers", self.id);
        let mut votes = 1; // Vote for self
        for peer in &self.peers {
            if self.simulate_peer_vote(*peer) {
                votes += 1;
            }
        }
        votes
    }

    fn simulate_peer_vote(&self, peer_id: u64) -> bool {
        // For simplicity, let's assume the peer votes for this node
        println!("Peer node {} voted for node {}", peer_id, self.id);
        true
    }

    fn quorum(&self) -> u64 {
        (self.peers.len() as u64 + 1) / 2 + 1
    }

    //Sending Heartbeats
    fn send_heartbeat(&mut self, network_stable: bool) {
        let interval = if network_stable {
            Duration::from_secs(10) // Reduce heartbeat frequency when network is stable
        } else {
            Duration::from_secs(2)  // Increase heartbeat frequency when network is unstable
        };

        println!(
            "Node {} sending heartbeat every {:?} seconds (stable: {})",
            self.id, interval.as_secs(), network_stable
        );

        // Simulate sending heartbeat
        for peer in &self.peers {
            self.simulate_heartbeat(*peer);
        }
    }

    fn simulate_heartbeat(&self, peer_id: u64) {
        println!("Node {} sent heartbeat to Node {}", self.id, peer_id);
    }

    fn receive_heartbeat(&mut self) {
        // Reset the heartbeat timer when a follower receives a heartbeat from the leader
        self.leader_last_heartbeat = Some(Instant::now());
        println!("Node {} received heartbeat, resetting timer", self.id);
    }
//Log Replication
fn replicate_log(&mut self, transaction: Transaction) {
    if self.role == Role::Leader {
        println!("Node {} is replicating transaction: {:?}", self.id, transaction);
        
        // Append the transaction to the leader's log
        self.log.push(transaction.clone());

        // Send the log entry to all followers
        for peer in &self.peers {
            self.send_log_entry(*peer, transaction.clone());
        }

        // Check if quorum is reached for replication
        if self.has_quorum_for_log() {
            self.commit_log();
        }
    }
}

//Log Consistency Mechanism
fn reconcile_logs(&mut self) {
    // Simulate checking and reconciling logs with followers
    println!("Node {} (Leader) is reconciling logs with followers", self.id);
    for peer in &self.peers {
        self.send_log_diff(*peer);
    }
}

fn send_log_diff(&self, peer_id: u64) {
    // Simulate sending missing log entries to followers
    println!("Node {} sending missing logs to Node {}", self.id, peer_id);
}

fn send_log_entry(&self, peer_id: u64, transaction: Transaction) {
    // Simulate sending a log entry to the follower node
    println!("Node {} sending log entry to Node {}", self.id, peer_id);
    self.simulate_follower_acknowledgment(peer_id);
}

fn simulate_follower_acknowledgment(&self, peer_id: u64) {
    // Simulate follower node acknowledging receipt of log entry
    println!("Node {} acknowledged log entry from Node {}", peer_id, self.id);
}

fn has_quorum_for_log(&self) -> bool {
    // Simulate checking for quorum (for simplicity, always returns true in this case)
    true
}

fn commit_log(&self) {
    // Commit the log once a quorum of followers have replicated the entry
    println!("Node {} has committed the log entry", self.id);
}

    //Commit Transactions
    fn commit_log(&self) {
        println!("Node {} has committed the transaction to the ledger.", self.id);
        
        // Notify followers to commit the transaction
        for peer in &self.peers {
            self.send_commit_to_follower(*peer);
        }
    }

    fn send_commit_to_follower(&self, peer_id: u64) {
        // Simulate sending commit message to follower
        println!("Node {} sending commit message to Node {}", self.id, peer_id);
        self.simulate_follower_commit(peer_id);
    }

    fn simulate_follower_commit(&self, peer_id: u64) {
        // Simulate follower committing the transaction to its own ledger
        println!("Node {} has committed the transaction to its ledger.", peer_id);
    }

    //Partition Detection
    fn check_partition(&self) -> bool {
        // Simulate partition detection by checking connectivity to peers
        println!("Node {} checking for partition...", self.id);
        let reachable_peers = self.get_reachable_peers();
        
        // If fewer than a quorum of nodes are reachable, we're in a partition
        if reachable_peers < self.quorum() {
            println!("Node {} detected partition: only {} reachable peers", self.id, reachable_peers);
            return true;
        }
        false
    }

    //Step 2: Preventing Split-Brain

    fn get_reachable_peers(&self) -> u64 {
        // Simulate the number of reachable peers (randomized for testing purposes)
        let reachable = rand::random::<u64>() % (self.peers.len() as u64 + 1); // Random reachable peers
        reachable
    }

    async fn handle_partition(&mut self) {
        if self.check_partition() {
            println!("Node {} in partition, cannot elect leader.", self.id);
            // Prevent leader election in partitioned group
        } else {
            // Normal leader election
            self.start_election().await;
        }
    }

    //Partition Recovery and Log Reconciliation
    fn recover_from_partition(&mut self) {
        println!("Node {} recovering from partition, reconciling logs...", self.id);
        self.reconcile_logs();
    }


    //Log Matching
    fn match_log_with_follower(&self, follower_log: &[Transaction]) -> usize {
        // Compare logs starting from the latest index
        let mut match_index = 0;
        
        let leader_log_len = self.log.len();
        let follower_log_len = follower_log.len();

        // Start from the end of the logs and compare backwards
        for i in (0..usize::min(leader_log_len, follower_log_len)).rev() {
            if self.log[i] == follower_log[i] {
                match_index = i + 1; // Found matching index
                break;
            }
        }
        
        println!("Matched up to index {} with follower", match_index);
        match_index
    }

    //Detecting Log Conflicts
    fn detect_conflict(&self, follower_log: &[Transaction]) -> bool {
        let match_index = self.match_log_with_follower(follower_log);
        
        // Check if follower's log has conflicting entries beyond the match index
        if follower_log.len() > match_index {
            println!("Conflict detected in follower's log beyond index {}", match_index);
            return true;
        }
        false
    }

    //Conflict Resolution - Truncating or Appending
    fn resolve_conflict(&self, follower_log: &mut Vec<Transaction>) {
        if self.detect_conflict(follower_log) {
            let match_index = self.match_log_with_follower(follower_log);
            
            // Truncate the follower's log to the match index
            follower_log.truncate(match_index);
            println!("Truncated follower's log to index {}", match_index);
            
            // Append missing entries from the leader's log
            for i in match_index..self.log.len() {
                follower_log.push(self.log[i].clone());
                println!("Appended entry at index {} to follower's log", i);
            }
        } else {
            println!("No conflict detected, logs are consistent.");
        }
    }

//Commit Synchronization
    fn synchronize_commit(&mut self, follower_logs: &mut [Vec<Transaction>]) {
        // Simulate log synchronization with all followers
        for follower_log in follower_logs.iter_mut() {
            self.resolve_conflict(follower_log);
        }
        
        // After resolving conflicts, commit the logs
        self.commit_logs();
    }

    fn commit_logs(&self) {
        // Commit all entries in the leader's log
        println!("Node {} committed all logs up to index {}", self.id, self.log.len());
        
        // Notify followers to commit
        for peer in &self.peers {
            self.send_commit_to_follower(*peer);
        }
    }

    //Snapshot Creation
    fn create_snapshot(&self) -> Snapshot {
        println!("Node {} creating snapshot at index {}", self.id, self.log.len());
        let mut state = HashMap::new();

        // Simulate capturing the current state of the system (key-value store)
        state.insert("balance_user1".to_string(), "500".to_string());
        state.insert("balance_user2".to_string(), "300".to_string());

        Snapshot {
            index: self.log.len() as u64, // Snapshot at the last log index
            state,
        }
    }

    //Sending Snapshots to Followers
    fn send_snapshot_to_follower(&self, follower_id: u64, snapshot: &Snapshot) {
        println!("Node {} sending snapshot to Node {} at index {}", self.id, follower_id, snapshot.index);

        // Simulate follower receiving and applying the snapshot
        self.apply_snapshot(snapshot);
    }

    fn apply_snapshot(&self, snapshot: &Snapshot) {
        // Simulate applying snapshot to follower
        println!("Node {} applied snapshot at index {}, state: {:?}", self.id, snapshot.index, snapshot.state);
    }

    //Log Truncation After Snapshot
    fn truncate_log(&mut self, snapshot: &Snapshot) {
        // Truncate the log entries up to the snapshot point
        self.log = self.log.split_off(snapshot.index as usize);
        println!("Node {} truncated log to index {}", self.id, snapshot.index);
    }

    //Recovery Using Snapshots
    fn recover_from_snapshot(&self, snapshot: &Snapshot) {
        println!("Node {} recovering from snapshot at index {}", self.id, snapshot.index);
        self.apply_snapshot(snapshot);
    }

    //Adding New Nodes
    fn add_new_node(&mut self, new_node_id: u64) {
        println!("Node {} adding new node {}", self.id, new_node_id);
        self.peers.push(new_node_id);

        // Simulate sending snapshot and log entries to the new node
        let snapshot = self.create_snapshot();
        self.send_snapshot_to_follower(new_node_id, &snapshot);

        // Adjust quorum and leader election logic
        self.update_quorum();
    }

    fn update_quorum(&self) {
        println!("Updating quorum size based on new peer count...");
        // The quorum is a majority of the total number of nodes (including leader)
        let quorum_size = (self.peers.len() as u64 + 1) / 2 + 1;
        println!("New quorum size: {}", quorum_size);
    }

    //Removing Nodes
    fn remove_node(&mut self, node_id: u64) {
        println!("Node {} removing node {}", self.id, node_id);
        self.peers.retain(|&id| id != node_id); // Remove node from peer list

        // Adjust quorum and leader election logic
        self.update_quorum();
    }

    //Handling Reconfiguration via Consensus
    fn propose_reconfiguration(&mut self, action: ReconfigAction) {
        // Propose reconfiguration to the cluster
        println!("Node {} proposing reconfiguration: {:?}", self.id, action);

        // Replicate the reconfiguration action through consensus
        self.replicate_log(Transaction {
            id: self.log.len() as u64 + 1,
            data: format!("{:?}", action),
        });

        // Apply the reconfiguration after it's committed
        self.apply_reconfiguration(action);
    }

    fn apply_reconfiguration(&mut self, action: ReconfigAction) {
        match action {
            ReconfigAction::AddNode(new_node_id) => self.add_new_node(new_node_id),
            ReconfigAction::RemoveNode(node_id) => self.remove_node(node_id),
        }
    }

    //Batching Log Entries 

    fn batch_replicate_logs(&self, log_entries: Vec<Transaction>) {
        if self.role == Role::Leader {
            println!("Node {} is batching and replicating {} log entries", self.id, log_entries.len());
            
            // Batch log entries and send them to all followers
            for peer in &self.peers {
                self.send_log_batch(*peer, log_entries.clone());
            }
        }
    }

    fn send_log_batch(&self, peer_id: u64, log_entries: Vec<Transaction>) {
        // Simulate sending a batch of log entries to a follower
        println!("Node {} sending batch of {} log entries to Node {}", self.id, log_entries.len(), peer_id);
        // Simulate the follower acknowledging the batch
        self.simulate_follower_acknowledgment(peer_id, log_entries.len());
    }

    fn simulate_follower_acknowledgment(&self, peer_id: u64, entries: usize) {
        // Simulate the follower acknowledging the batch of log entries
        println!("Node {} acknowledged {} log entries from Node {}", peer_id, entries, self.id);
    }


    //Log Compression for Replication
    fn replicate_compressed_logs(&self, log_entries: Vec<Transaction>) {
        if self.role == Role::Leader {
            println!("Node {} compressing and replicating logs", self.id);

            // Compress log entries before sending them to followers
            let compressed_data = compress_logs(&log_entries);

            for peer in &self.peers {
                self.send_compressed_log(*peer, compressed_data.clone());
            }
        }
    }

    fn send_compressed_log(&self, peer_id: u64, compressed_data: Vec<u8>) {
        println!(
            "Node {} sending compressed log of size {} bytes to Node {}",
            self.id, compressed_data.len(), peer_id
        );
    }


    //Concurrent Replication
    async fn replicate_logs_concurrently(&self, log_entries: Vec<Transaction>) {
        if self.role == Role::Leader {
            println!("Node {} is replicating logs concurrently", self.id);

            let mut tasks = vec![];

            for peer in &self.peers {
                let peer_id = *peer;
                let log_clone = log_entries.clone();

                // Spawn a task to replicate logs to each peer concurrently
                let task = task::spawn(async move {
                    println!("Replicating logs to Node {}...", peer_id);
                    Node::simulate_follower_acknowledgment_concurrent(peer_id, log_clone.len()).await;
                });

                tasks.push(task);
            }

            // Wait for all tasks to complete
            for task in tasks {
                task.await.unwrap();
            }

            println!("All logs replicated concurrently");
        }
    }

    async fn simulate_follower_acknowledgment_concurrent(peer_id: u64, entries: usize) {
        // Simulate asynchronous acknowledgment
        println!("Node {} acknowledged {} log entries", peer_id, entries);
    }


    //Efficient Log Application
    fn apply_logs_in_batch(&mut self, log_entries: &[Transaction]) {
        println!("Node {} applying {} log entries in batch", self.id, log_entries.len());

        for entry in log_entries {
            // Simulate applying each transaction to the state machine
            self.apply_transaction(entry);
        }
    }

    fn apply_transaction(&mut self, entry: &Transaction) {
        println!("Node {} applying transaction: {:?}", self.id, entry);
        // Simulate state machine update (e.g., update account balance)
    }

    //Snapshot Installation Optimization
    fn install_snapshot(&mut self, snapshot: &Snapshot) {
        println!("Node {} installing snapshot at index {}", self.id, snapshot.index);
        
        // Simulate state reset to match the snapshot state
        self.state = snapshot.state.clone();

        // Truncate the log to the snapshot point
        self.log.clear();
        println!("Node {} has installed snapshot and cleared log", self.id);
    }


    //Pipelined Log Acknowledgment
    async fn acknowledge_logs_pipelined(&self, log_entries: &[Transaction]) {
        println!("Node {} acknowledging {} log entries in pipeline", self.id, log_entries.len());

        // Simulate asynchronous acknowledgment of the log entries
        tokio::time::sleep(Duration::from_millis(100)).await;

        println!("Node {} acknowledged log entries in batch", self.id);
    }

    //Incremental Snapshot Application
    fn apply_incremental_snapshot(&mut self, incremental_snapshot: &Snapshot) {
        println!("Node {} applying incremental snapshot at index {}", self.id, incremental_snapshot.index);

        // Simulate merging the incremental snapshot into the current state
        for (key, value) in &incremental_snapshot.state {
            self.state.insert(key.clone(), value.clone());
        }

        println!("Node {} applied incremental snapshot", self.id);
    }

    //Leader Failure Detection and Election
    async fn detect_leader_failure(&mut self) {
        let timeout_duration = Duration::from_secs(5);

        // Simulate checking for heartbeats with a timeout
        let timeout_result = tokio::time::timeout(timeout_duration, self.wait_for_heartbeat()).await;

        if timeout_result.is_err() {
            println!("Node {} detected leader failure, starting a new election", self.id);
            self.start_election().await;
        }
    }

    async fn wait_for_heartbeat(&self) {
        // Simulate waiting for a heartbeat from the leader
        tokio::time::sleep(Duration::from_secs(4)).await;
        println!("Node {} received heartbeat", self.id);
    }

    //Handling Log Divergence and Consistency after Failures

    fn reconcile_logs_after_failure(&mut self) {
        println!("Node {} reconciling logs after failure...", self.id);

        // Simulate the leader matching its log with followers
        for peer in &self.peers {
            let match_index = self.match_log_with_follower();
            self.reconcile_log_differences(*peer, match_index);
        }
    }

    fn match_log_with_follower(&self) -> usize {
        // Simulate log matching (leader-follower comparison)
        let match_index = self.log.len(); // Simplified example
        println!("Node {} log matches with follower up to index {}", self.id, match_index);
        match_index
    }

    fn reconcile_log_differences(&self, peer_id: u64, match_index: usize) {
        // Simulate resolving log differences by appending missing entries
        println!("Node {} reconciling log differences with Node {} after index {}", self.id, peer_id, match_index);
    }

    //State Persistence for Crash Recovery

    fn persist_state(&self) {
        // Persist the node's current term and log to disk
        let state_data = serde_json::to_string(&self).unwrap();
        fs::write(format!("node_{}_state.json", self.id), state_data).expect("Unable to persist state");
        println!("Node {} persisted its state to disk", self.id);
    }

    fn recover_state(&mut self) {
        // Recover the node's state from disk
        let state_data = fs::read_to_string(format!("node_{}_state.json", self.id)).expect("Unable to recover state");
        let recovered_node: Node = serde_json::from_str(&state_data).unwrap();
        *self = recovered_node;
        println!("Node {} recovered its state from disk", self.id);
    }

    //Snapshot Recovery and Fault Tolerance
    fn recover_from_snapshot_failure(&mut self, snapshot: &Snapshot) {
        println!("Node {} recovering from snapshot failure", self.id);

        // Re-apply snapshot after failure
        self.apply_snapshot(snapshot);
        println!("Node {} successfully recovered from snapshot failure", self.id);
    }

    //Digital Signatures for Message Authentication

    fn sign_message(&self, message: &str) -> Signature {
        // Generate key pair (for simplicity, generated for each message; in practice, use persistent keys)
        let rng = rand::SystemRandom::new();
        let key_pair = Ed25519KeyPair::from_seed_and_public_key(
            &rng.generate().unwrap().as_ref()[..32], 
            &[0; 32],
        ).unwrap();

        // Sign the message
        let sig = key_pair.sign(message.as_bytes());
        println!("Node {} signed message: {}", self.id, message);
        sig
    }

    fn verify_message(&self, message: &str, signature: Signature, public_key: &[u8]) -> bool {
        // Verify the message using the sender's public key
        let peer_public_key = signature::UnparsedPublicKey::new(&signature::ED25519, public_key);
        peer_public_key.verify(message.as_bytes(), signature.as_ref()).is_ok()
    }


    //Encrypting Logs and State
    fn encrypt_log_entry(&self, entry: &Transaction, key: &Key) -> Vec<u8> {
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(b"unique nonce"); // Use a unique nonce for each message
        let plaintext = serde_json::to_string(entry).unwrap().as_bytes();

        // Encrypt the log entry
        let ciphertext = cipher.encrypt(nonce, plaintext).expect("encryption failure!");
        println!("Node {} encrypted log entry", self.id);
        ciphertext
    }

    fn decrypt_log_entry(&self, ciphertext: &[u8], key: &Key) -> Transaction {
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(b"unique nonce"); // Same nonce as used for encryption

        // Decrypt the log entry
        let plaintext = cipher.decrypt(nonce, ciphertext).expect("decryption failure!");
        let entry: Transaction = serde_json::from_slice(&plaintext).unwrap();
        println!("Node {} decrypted log entry", self.id);
        entry
    }

    //Securing Leader Elections
    fn request_vote_secure(&self, node_id: u64, challenge: &str) -> bool {
        // Simulate a challenge-response for verifying the voter's identity
        println!("Node {} requesting secure vote from Node {}", self.id, node_id);

        // In a real system, this would involve cryptographic verification of the challenge response
        let valid = self.verify_challenge_response(challenge);
        if valid {
            println!("Node {} verified identity of Node {}", self.id, node_id);
        }
        valid
    }

    fn verify_challenge_response(&self, challenge: &str) -> bool {
        // Simulate verification of a challenge response
        challenge == "valid_challenge" // In practice, this would be cryptographically signed
    }

    //Node Identity Verification (Certificates)
    fn generate_certificate(&self) -> Vec<u8> {
        // Generate and return a public key as the "certificate" (simplified for this example)
        let key_pair = Ed25519KeyPair::generate_pkcs8(&rand::SystemRandom::new()).unwrap();
        key_pair.public_key().as_ref().to_vec()
    }

    fn verify_certificate(&self, certificate: &[u8]) -> bool {
        // Simulate certificate verification (in a real-world scenario, this would involve CA verification)
        println!("Node {} verifying certificate", self.id);
        certificate.len() == 32 // Check if it's a valid public key
    }

}
