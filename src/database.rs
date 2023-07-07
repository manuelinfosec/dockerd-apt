use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json;

// use crate::modules::node::Nodes;

const BASEDBPATH: &str = "data";
const NODEFILE: &str = "nodes.json";
const TXFILE: &str = "txn.json";
const UNTXFILE: &str = "untxn.json";
const ACCOUNTDB: &str = "accounts.json";
const BLOCKCHAINDB: &str = "blockchain.json";

pub trait BaseDB {
    // get current path to local database
    fn get_path(&self) -> String;

    // read the database || return an object that is deserializable
    fn read<T: DeserializeOwned>(&self) -> Vec<T> {
        let file_path: String = self.get_path();
        // create an empty string to save data read from file
        let mut raw: String = String::new();

        // check if the file exists or return the file for reading
        let mut file = match File::open(file_path) {
            Ok(file) => file,
            // handle file open error, by returning an empty vector
            Err(e) => {
                return Vec::new();
            }
        };

        // handle errors when reading the file
        if let Err(_) = file.read_to_string(&mut raw) {
            // return an empty vector
            return Vec::new();
        }

        // deserialize from string to Vec<Node>
        let data: Result<Vec<T>, serde_json::Error> = serde_json::from_str(&raw);

        // check for deserialization errors
        match data {
            Ok(data) => {   // return the deserialized data
                data
            }
            Err(err) => {   // return a new vector as form of error handling
                Vec::new()
            }
        }
    }

    // write an item to the database | accepting parameters that can be serialized or deserialized
    fn write<T: Serialize + DeserializeOwned>(&self, item: T) -> io::Result<()> {
        // get current path to local database
        let file_path: String = self.get_path();

        // read the entire database to vector buffer (there should be a better to do this)
        let mut data: Vec<T> = self.read();

        // push item to the buffer
        data.push(item);

        let json_data = serde_json::to_string(&data)?;      // serialize buffer vector to string
        let mut file = File::create(file_path)?;             // overwrite existing database
        file.write_all(json_data.as_bytes())?;                     // write serialized string to the database

        Ok(())
    }

    // erase the database
    fn clear(&self) -> io::Result<()> {
        let file_path = self.get_path();
        // truncate the database file
        File::create(file_path)?;
        Ok(())
    }

    // return all objects in the local database
    fn find_all<T: DeserializeOwned>(&self) -> Vec<T> {
        // read the local database
        self.read()
    }

    // write an item to the local database
    fn insert<T: Serialize + DeserializeOwned>(&self, item: T) -> io::Result<()> {
        self.write(item)
    }

    // insert a transaction hash if it doesn't exist in the local database
    // fn hash_insert<T>(&self, item: T) -> io::Result<()>
    //     where T: Serialize + DeserializeOwned
    // {
    //     // flag for checking if a hash already exists
    //     let mut exists = false;
    //
    //     // loop through all available hashes
    //     for obj in self.find_all() {
    //
    //         // compare for any matching hashes
    //         if item.hash == obj.hash {
    //             // update the flag
    //             exists = true;
    //             break;
    //         }
    //     }
    //
    //     // check if the flag has not been updated
    //     if !exists {
    //         // write the transaction to database
    //         self.write(item)?;
    //     }
    //
    //     // return success
    //     Ok(())
    // }
}

// Nodes in the network
pub struct NodeDB {
    file_path: String, // database location
}

// Local user accounts
pub struct AccountDB {
    file_path: String, // database location
}

// Blocks in the database
pub struct BlockchainDB {
    file_path: String,  // database location
}

// Transactions in the database
pub struct TransactionDB {
    file_path: String,  // database location
}

// Transactions that don't store in the database
pub struct UnTransactionDB {
    file_path: String,  // database location
}

// Native methods for the Nodes database
impl NodeDB {
    // create an instance of the Nodes database
    pub fn new() -> NodeDB {
        // perform initialize with database location
        NodeDB {
            file_path: String::from(format!("{BASEDBPATH}/{NODEFILE}")) // set a default empty string
        }
    }
}

// Native methods for the Blockchain database
impl BlockchainDB {
    // create an instance of the Blockchain database
    pub fn new() -> BlockchainDB {
        // perform initialization with the database location
        BlockchainDB {
            file_path: String::from(format!("{BASEDBPATH}/{BLOCKCHAINDB}"))
        }
    }

    #[allow(unused_variables)]
    fn find(&self, hash: String) -> HashMap<String, String> {
        HashMap::new()
    }

    fn insert(&self) -> io::Result<()> {
        // self.hash_insert("Test".to_string())
        Ok(())
    }
}

// Native methods for the accounts database
impl AccountDB {
    // create an instance of the Account database
    pub fn new() -> AccountDB {
        // perform initialization with the database location
        AccountDB {
            file_path: String::from(format!("{BASEDBPATH}/{ACCOUNTDB}"))
        }
    }
    fn find_one(&self) -> &str {
        ""
    }
}

impl TransactionDB {
    // create an instance of the Transaction database
    pub fn new() -> TransactionDB {
        // perform initialization with the database location
        TransactionDB {
            file_path: String::from(format!("{BASEDBPATH}/{TXFILE}"))
        }
    }
}


// Inherited methods from BaseDB trait
impl BaseDB for NodeDB {
    // get current path to local database
    fn get_path(&self) -> String {
        self.file_path.to_owned()
    }

    // check if transaction hash exists or insert
}


impl BaseDB for AccountDB {
    // get current path to local database
    fn get_path(&self) -> String {
        self.file_path.to_owned()
    }
}


impl BaseDB for BlockchainDB {
    // get current path to local database
    fn get_path(&self) -> String {
        self.file_path.to_owned()
    }
}


impl BaseDB for TransactionDB {
    // get current path to local database
    fn get_path(&self) -> String {
        self.file_path.to_owned()
    }
}
