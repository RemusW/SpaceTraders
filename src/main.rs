mod mine_asteroid;
mod dashboard;

use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use serde::{Serialize};
use spacedust::apis::ResponseContent;
use spacedust::apis::agents_api::get_my_agent;
use spacedust::apis::configuration::Configuration;
use spacedust::apis::default_api::register;
use spacedust::apis::contracts_api::{get_contracts, accept_contract};
use spacedust::apis::fleet_api::{purchase_ship, get_my_ships, navigate_ship, dock_ship, get_my_ship, refuel_ship, extract_resources, orbit_ship};
use spacedust::apis::systems_api::{get_shipyard, get_system, get_system_waypoints};
use spacedust::models::waypoint_trait::Symbol;
use spacedust::models::{Contract, WaypointTrait, purchase_ship_request, PurchaseShipRequest, ShipType, navigate_ship_request, NavigateShipRequest, ExtractResourcesRequest};
use spacedust::models::register_request::{Faction, RegisterRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth_key = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZGVudGlmaWVyIjoiV0FOREEiLCJ2ZXJzaW9uIjoidjIiLCJyZXNldF9kYXRlIjoiMjAyMy0wNi0wMyIsImlhdCI6MTY4NTg0MDQzNCwic3ViIjoiYWdlbnQtdG9rZW4ifQ.cHKTNHuDIJw7-hiJm4NeW1dtzNHfoRDGJJWyWCnQrnI0cphHnfMWnPxL0T1nDFAo367DvQr73KepWm24oDktsKrt4sGHSlo9184yHFg85qMMKvraiuDiC_3rw04Kcvf8qR6Znf3dD8ritvFpp_1gnVUwN3z2z223_r8zDGOdo6KtlLHSYK-56hS0SRFhPcO7NdhnbmdfrVd6k1LgQdj6rJKqTZ1REA0-KjUipWR4f0saVvqQuUelYvcAuozCIAl4Wga742wGB4nJLGtxD534uNmlj_sU5S4O21nWk0OrA7gFF-IUL3Hu9tJnzYxyLH4gKkAl2OqhxwlNtG30982WoQ";

    // Create Configuration
    let mut conf = Configuration::new();
    conf.bearer_access_token = Some(auth_key.to_string());
    // Create Register Request
    // let reg_req = RegisterRequest::new(Faction::Cosmic, "WANDA".to_string());
    // Register Agent
    // let register_response = register(&conf, Some(reg_req)).await;
    // match register_response {
    //     Ok(res) => {
    //         println!("{:#?}", res);
    //         // Update Config with Agent Token
    //         conf.bearer_access_token = Some(res.data.token);
    //     }
    //     Err(err_res) => {
    //         panic!("{:#?}", err_res);
    //     }
    // }

    // Get Agent Details to Confirm Working
    match get_my_agent(&conf).await {
        Ok(res) => {
            println!("{:#?}", res);
            // Print Symbol
        }
        Err(err_res) => {
            panic!("{:#?}", err_res);
        }
    }

    let contacts_req = get_contracts(&conf, Some(1), Some(10)).await;
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
    
    let system_symbol_req = get_system_waypoints(&conf, "X1-HQ18", Some(1), Some(20)).await;
    let mut shipyard_waypoint: String = "".to_string();
    match system_symbol_req {
        Ok(res) => {
            // println!("{:#?}", res);
            for waypoint in res.data {
                for waypoint_trait in waypoint.traits {
                    if waypoint_trait.symbol == Symbol::Shipyard {
                        shipyard_waypoint = waypoint.symbol.to_string();
                    }
                }
            }
        }
        Err(err_res) => {
            panic!("{:#?}", err_res);
        }
    }
    // if shipyard_waypoint.len() == 14 {
    //     let shipyard_req = get_shipyard(&conf, &shipyard_waypoint[0..7], &shipyard_waypoint).await;
    //     print_req(&shipyard_req); 
    //     let purchase_ship_request = PurchaseShipRequest::new(ShipType::MiningDrone, shipyard_waypoint);
    //     let purchase_req = purchase_ship(&conf, Some(purchase_ship_request)).await;
    //     print_req(&purchase_req);
    //     // if let Ok(shipyard) = shipyard_req {
    //     //     if let Some(ships) = shipyard.data.ships {
    //     //         for ship in ships {
    //     //             if ship.frame.symbol == Symbol::Miner {
    //     //             }
    //     //         }
    //     //     }
    //     // }
    // }
    
    let my_ships_req = get_my_ships(&conf, Some(1), Some(10)).await;
    // if let Ok(my_ships) = &my_ships_req {
    //     for ship in my_ships.data {

    //     }
    // }
    // print_req(&my_ships_req);
    // nav_to_asteroid(&conf, "WANDA-4").await;
    let ship_req = get_my_ship(&conf, "WANDA-4").await;
    // print_req(&ship_req);

    dock_and_refuel(&conf, "WANDA-4").await;
    orbit_ship(&conf, "WANDA-4", 0).await;
    let extract_res_req = ExtractResourcesRequest::new();
    let extract_res = extract_resources(&conf, "WANDA-4", Some(extract_res_req)).await;
    print_req(&extract_res);
    Ok(())
}

async fn nav_to_asteroid(conf: &Configuration, ship_symbol: &str) {
    let system_symbol_req = get_system_waypoints(&conf, "X1-HQ18", Some(1), Some(20)).await;
    let mut mining_waypoint: String = "".to_string();
    match system_symbol_req {
        Ok(res) => {
            // println!("{:#?}", res);
            for waypoint in res.data {
                for waypoint_trait in waypoint.traits {
                    if waypoint_trait.symbol == Symbol::MineralDeposits {
                        mining_waypoint = waypoint.symbol.to_string();
                    }
                }
            }
        }
        Err(err_res) => {
            panic!("{:#?}", err_res);
        }
    }

    let nav_req = NavigateShipRequest::new(mining_waypoint.to_string());
    let nav_ship_req = navigate_ship(&conf, ship_symbol, Some(nav_req)).await;
    print_req(&nav_ship_req);
}

async fn dock_and_refuel(conf: &Configuration, ship_symbol: &str) {
    let dock_req = dock_ship(&conf, ship_symbol, 0.0).await;
    let refuel_req = refuel_ship(&conf, ship_symbol, 0).await;
    print_req(&refuel_req);
}

fn print_req<T: Debug+Serialize, E: Debug>(req: &Result<T, E>) {
    match req {
        Ok(res) => {
            // res = serde_json::to_string_pretty(req.as_ref()).expect("msntg");
            // let mut file = File::create("log.json").expect("Failed to create");
            // file.write_all(res).expect("msnothingg");
            println!("{:#?}", res);
            // Print Symbol
        }
        Err(err_res) => {
            panic!("{:#?}", err_res);
        }
    }
}

async fn accept_best_contract(conf: &Configuration, contracts: &Vec<Contract>) {
    let contract_id = &contracts[0].id;
    let content_length = &contracts[0].expiration;
    let accept_req = accept_contract(conf, contract_id, 0).await;
    match accept_req {
        Ok(_) => {

        }
        Err(err_res) => {
            panic!("{:#?}", err_res);
        }
    }
}