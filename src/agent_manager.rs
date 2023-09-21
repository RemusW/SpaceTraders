use std::error::Error;
use std::fs::File;
use std::io::{Write, Read, self};

use spacedust::apis::configuration::Configuration;
use spacedust::models::{Contract, Register201ResponseData};
use spacedust::models::register_request::RegisterRequest;
use spacedust::models::faction_symbols::FactionSymbols;
use spacedust::apis::default_api::register;
use spacedust::apis::agents_api::get_my_agent;
use spacedust::apis::contracts_api::{get_contracts, accept_contract};

pub struct Agent {
    pub conf: Configuration
}

impl Agent {
    pub fn new() -> Self {
        Self {
            conf: Configuration::new()
        }
    }

    pub async fn login_agent(&mut self) -> Result<(), io::Error> {
        let mut file = File::open("./agent_keys.json")?;
        // Read the contents of the file into a String
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        // Deserialize the string into the desired data structure
        let agent_data: Register201ResponseData = serde_json::from_str(&content)?;

        // Use the deserialized data as needed
        self.conf.bearer_access_token = Some(agent_data.token);

        Ok(())
    }

    pub async fn register_agent(&mut self) -> Result<(), io::Error> {
        // Create Register Request
        let reg_req = RegisterRequest::new(FactionSymbols::Cosmic, "WANDA".to_string());
        // Register Agent
        let register_response = register(&self.conf, Some(reg_req)).await;
        match register_response {
            Ok(res) => {
                // Update Config with Agent Token
                let agent_data = Some(res.data);
                let agent_data = serde_json::to_string(&agent_data)?;
                let mut file = File::create("./agent_keys.json")?;
                file.write_all(agent_data.as_bytes())?;
                self.login_agent().await;
            }
            Err(err_res) => {
                panic!("{:#?}", err_res);
            }
        }
        Ok(())
    }

    pub async fn agent_info(self) {
        // Get Agent Details to Confirm Working
        match get_my_agent(&self.conf).await {
            Ok(res) => {
                println!("{:#?}", res);
                // Print Symbol
            }
            Err(err_res) => {
                panic!("{:#?}", err_res);
            }
        }
    }

    async fn contracts(self) {
        let contacts_req = get_contracts(&self.conf, Some(1), Some(10)).await;
        match contacts_req {
            Ok(res) => {
                println!("{:#?}", res);
                // let contracts = res.data;
                // accept_best_contract(&conf, &contracts).await;
            }
            Err(err_res) => {
                panic!("{:#?}", err_res);
            }
        }
    }

    async fn accept_best_contract(self, contracts: &Vec<Contract>) {
        let contract_id = &contracts[0].id;
        let content_length = &contracts[0].expiration;
        let accept_req = accept_contract(&self.conf, contract_id).await;
        match accept_req {
            Ok(_) => {
    
            }
            Err(err_res) => {
                panic!("{:#?}", err_res);
            }
        }
    }
}
