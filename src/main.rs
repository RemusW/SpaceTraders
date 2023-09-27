mod dashboard;
mod agent_manager;
mod navigation;
use std::fmt::Debug;
use serde::Serialize;

// use dashboard::SpaceConsole;
use spacedust::{apis::{configuration::Configuration, fleet_api::{get_my_ships, extract_resources, orbit_ship, ExtractResourcesError}, Error, contracts_api::{accept_contract, self, get_contracts, get_contract}, factions_api}, models::{ExtractResourcesRequest, FactionSymbols, faction, waypoint_trait}};

use crate::navigation::dock_and_refuel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let native_options = eframe::NativeOptions::default();
    // let _ = eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(SpaceConsole::new(cc))));

    let mut agent = agent_manager::Agent::new();
    // agent.register_agent().await;
    // let _ = agent.login_agent().await;
    // agent.agent_info().await;
    loop {
        let mut input = String::new();

        println!("Please enter something:");

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        let cmd = match input {
            "login" => Commands::Login,
            "register" => Commands::Register,
            "mine" => Commands::NavToAsteroid,
            "get contract" => Commands::GetContract,
            "finish" => Commands::CompleteContract,
            _ => Commands::GetMyShips 
        };
        match cmd {
            Commands::Login => {
                let _ = agent.login_agent().await;
            }
            Commands::Register => {
                let _ = agent.register_agent().await;
            }
            Commands::NavToAsteroid => {
                nav(&agent.conf).await;
            }
            Commands::GetContract => {
                let faction_req = factions_api::get_faction(&agent.conf, "COSMIC").await;
                if let Ok(res) = faction_req {
                    res.data.headquarters
                }
                let contracts_req = get_contracts(&agent.conf, Some(1), Some(10)).await;
                print_req(&contracts_req);
                
                println!("Enter contract_id you wish to accept");
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let input = input.trim();
                let accept_contract_req = accept_contract(&agent.conf, input).await;
                print_req(&accept_contract_req);
            }
            Commands::GetMyShips => {
                let s = navigation::get_my_ships(&agent.conf).await;
                println!("{:#?}", s);
            },
            _ => println!("ererer")
        } 
    }
    // let my_ships_req = get_my_ships(&conf, Some(1), Some(10)).await;
    // nav_to_asteroid(&conf, "WANDA-4").await;
    // let ship_req = get_my_ship(&conf, "WANDA-4").await;
    // print_req(&ship_req);

    // dock_and_refuel(&conf, "WANDA-4").await;
    // orbit_ship(&conf, "WANDA-4", 0).await;
    // let extract_res_req = ExtractResourcesRequest::new();
    // let extract_res = extract_resources(&conf, "WANDA-4", Some(extract_res_req)).await;
    // print_req(&extract_res);
    Ok(())
}


enum Commands {
    Login,
    Register,
    GetMyShips,
    NavToAsteroid,
    GetContract,
    CompleteContract,
}

async fn nav(conf: &Configuration) {
    let ship = navigation::get_my_ships(&conf).await;
    match ship {
        Some(ship) => {
            println!("{:#?}", ship.nav);
            navigation::nav_to_asteroid(&conf, &ship).await;
            // dock_and_refuel(&conf, &ship).await;
            // orbit_ship(&conf, &ship.symbol).await;
            loop {
                let eee = ExtractResourcesRequest::new();
                let extraction_req = extract_resources(&conf, &ship.symbol, Some(eee)).await;                
                match extraction_req {
                    Ok(res) => {
                        println!("{:#?}", res);
                        tokio::time::sleep(std::time::Duration::
                            from_secs(res.data.cooldown.total_seconds as u64)).await;
                    }
                    Err(e) => {
                        eprintln!("{:#?}", e);
                    }
                }    
                if ship.cargo.capacity <= ship.cargo.units {
                    break;
                }
            }
            println!("Done extrcting")
        },
        None => println!("Panic"),
    }
    
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