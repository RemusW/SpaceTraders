mod dashboard;
mod agent_manager;
// mod navigation;

use std::fmt::Debug;
use dashboard::SpaceConsole;
use serde::Serialize;
use spacedust::apis::configuration::Configuration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(SpaceConsole::new(cc))));

    let mut agent = agent_manager::Agent::new();
    let _ = agent.login_agent().await;
    agent.agent_info().await;
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
