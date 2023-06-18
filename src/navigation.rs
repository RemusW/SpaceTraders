use spacedust::{apis::{systems_api::*, fleet_api::*}, models::*};


pub async fn find_shipyward_waypoint() {
    let system_symbol_req = get_system_waypoints(&conf, "X1-HQ18", Some(1), Some(20)).await;
    let mut shipyard_waypoint: String = String::new();
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
}

pub async fn buy_ship() {
    if shipyard_waypoint.len() == 14 {
        let shipyard_req = get_shipyard(&conf, &shipyard_waypoint[0..7], &shipyard_waypoint).await;
        print_req(&shipyard_req); 
        let purchase_ship_request = PurchaseShipRequest::new(ShipType::MiningDrone, shipyard_waypoint);
        let purchase_req = purchase_ship(&conf, Some(purchase_ship_request)).await;
        print_req(&purchase_req);
    }
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