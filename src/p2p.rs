use std::net;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use jsonrpc::{self, Client, Error, Request, Response};
use jsonrpc::simple_tcp::TcpTransport;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{json, value, Value};
// use serde_json::raw::RawValue;
use serde_json::value::{RawValue, to_raw_value};

use crate::database::{BaseDB, BlockchainDB, TransactionDB, UnTransactionDB};
use crate::modules::blockchain::Blockchain;
use crate::modules::node::{add_node, get_nodes};
use crate::modules::transactions::Transaction;

// represent the current node as a RPC Server ready to receive connections
struct RPCServer {
    // server address: `tcp://127.0.0.1:8000
    server: String,
}

// represent the current node ready to send connections
struct RPCClient {
    // server address
    node: String,
    // server client object
    client: Client,
}

struct BroadCast {}

impl BroadCast {}


impl RPCServer {
    fn new(server: String) -> RPCServer {
        // return an initialized RPC Server
        RPCServer {
            server
        }
    }

    /// Check for connectivity
    fn ping(&self) -> bool {
        // indicate connectivity
        true
    }

    /// Get blockchain from local database
    fn get_blockchain(&self) -> Vec<Blockchain> {
        // return all block from the local blockchain database
        BlockchainDB::new().find_all()
    }

    /// Add a new block to the local database
    fn new_block<T: Serialize + DeserializeOwned>(&self, block: T) -> () {

        // insert a new block to the blockchain database
        BlockchainDB::new().insert(block).unwrap();

        // clear database that stores transactions to be mined
        UnTransactionDB::new().clear().unwrap();

        println!("Received New Block")
    }

    /// Add a node to the local database
    fn add_node(&self, address: String) {
        // add node to local database cloning `address` as mutable string
        add_node(&mut address.clone())
    }

    /// Get all transactions from the local database
    fn get_transactions(&self) -> Vec<Transaction> {
        // return all transactions from local database
        TransactionDB::new().find_all()
    }

    /// Add an unmined transaction to the local database
    fn new_untransaction<T: Serialize + DeserializeOwned>(&self, untxns: T) -> () {
        // TODO: What if it fails to insert the transaction?
        UnTransactionDB::new().insert(untxns).unwrap()
    }

    /// Write a new mined transaction to the local database
    fn block_transaction<T: Serialize + DeserializeOwned>
    (&self, txns: T) -> () {
        println!("Received new block transaction!");

        // TODO: What if it fails to write a transcation?
        TransactionDB::new().write(txns).unwrap()
    }
}


impl RPCClient {
    fn new(node: String) -> RPCClient {

        // If any, strip scheme from address
        let stripped_node: String = node
            .strip_prefix("tcp://")
            .unwrap_or(&node)
            .to_string();

        // split the node address `127.0.0.1:8000` to `127.0.0.1` and `8080`
        let address_split: Vec<&str> = stripped_node.split(":")
            .collect();

        // parse String to IP address
        let addr: IpAddr = address_split[0]
            .parse()
            .unwrap();

        // parse String to port
        let port: u16 = address_split[1]
            .parse::<u16>()
            .unwrap();

        // Defining the transport protocol to use tcp 1.0 is used because minimal dependencies
        // ...is the goal and it's ok to use synchronous communication
        let transport: TcpTransport = TcpTransport::new(SocketAddr::new(addr, port));

        // construct a RPC client
        RPCClient {
            node,
            // construct client with transport tcp transport protocol
            client: Client::with_transport(transport),
        }
    }

    fn ping(&self, args: Vec<String>) -> Result<bool, Error> {
        // serialize arguments to raw json
        let params = [to_raw_value(&args)?];

        // build request with parameters
        let request: Request = self.client
            .build_request("ping", &params);

        // send request
        let response = self.client
            .send_request(request)?;

        // deserialize response or an error
        response.result::<bool>()
    }

    fn get_blockchain(&self, args: Vec<String>) -> Result<Vec<Blockchain>, Error> {
        // serialize arguments to raw json
        let params: [Box<RawValue>; 1] =
            [to_raw_value(&args)?];

        // build request with parameters
        let request: Request = self.client
            .build_request("get_blockchain", &params);

        // send request
        let response: Response = self.client.send_request(request)?;
        // deserialize response or return an error
        response.result::<Vec<Blockchain>>()
    }

    fn new_block(&self, block: Blockchain) -> Result<(), Error> {
        // serialize arguments to raw json
        let params: [Box<RawValue>; 1] =
            [to_raw_value(&block)?];

        // construct request with parameters
        let request: Request = self.client
            .build_request("new_block", &params);

        // send request || doesn't require a response
        self.client.send_request(request)?;

        // print debug message
        println!("Sent new block");
        Ok(())
    }

    fn add_node(&self, address: String) -> Result<(), Error> {
        // serialize arguments to raw json
        let params: [Box<RawValue>; 1] =
            [to_raw_value(&address)?];

        // construct request with parameters
        let request: Request = self.client
            .build_request("add_node", &params);

        // send request || doesn't require a respnse
        self.client.send_request(request)?;

        println!("Adding node {address} to network");

        Ok(())
    }

    fn get_transactions(&self, args: Vec<String>) -> Result<Vec<Transaction>, Error> {
        // serialize arguments to raw json
        let params: [Box<RawValue>; 1] =
            [to_raw_value(&args)?];

        // construct request with parameters
        let request: Request = self.client
            .build_request("get_transactions", &params);

        let response: Response = self.client
            .send_request(request)?;

        // deserialize response or throw error
        response.result::<Vec<Transaction>>()
    }

    fn new_untransaction<T: Serialize + DeserializeOwned>
    (&self, args: T) -> Result<(), Error> {
        // serialize arguments to raw json
        let params: [Box<RawValue>; 1] =
            [to_raw_value(&args)?];

        // construct request with parameters
        let request: Request = self.client.
            build_request("new_untransaction", &params);

        // send request || doesn't require response
        self.client.send_request(request);

        Ok(())
    }

    fn block_transaction<T: Serialize + DeserializeOwned>
    (&self, txn: T) -> Result<(), Error>
    {
        // serialize arguments to json
        let params: [Box<RawValue>; 1] =
            [to_raw_value(&txn)?];

        // construct request with parameters
        let request: Request = self.client
            .build_request("block_transacation", &params);

        self.client.send_request(request);

        Ok(())
    }
}


/// Returns an iterable RPCClient(s)
fn get_clients() -> Vec<RPCClient> {
    // placeholder to store queried clients
    let mut clients: Vec<RPCClient> = Vec::new();

    // query local database for available nodes
    let nodes: Vec<String> = get_nodes();

    // iterate through all nodes
    for node in nodes {
        // construct RPC client from the node
        clients.push(RPCClient::new(node));
    }

    // return clients
    clients
}