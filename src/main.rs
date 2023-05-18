mod send_resive;
use chrono::prelude::Utc;
use send_resive::*;
use std::{collections::HashMap, fs, env::current_dir};

#[tokio::main]
async fn main() {
    let mut varibles: HashMap<String, String> = HashMap::new();
    let mut token = fs::read_to_string(&format!("{}/token.txt", current_dir().unwrap().to_str().unwrap())).unwrap_or("".to_string());
    varibles.insert("token".to_string(), token.as_str().to_string());
    let mut username =
        json::stringify(get_username(&token).await["data"]["symbol"].clone()).replace('\"', "");
    let mut credits = json::stringify(get_credits(&token).await);
    
    if credits == "null" {
        let user_name = input("Username: ");
        let com_token = new_acc("COSMIC".to_string(), user_name).await;
        if com_token["data"]["token"].to_string() == "null" {
            println!("\x1b[3m\x1b[38;5;160mFailed to create account!\nError: {}\nMessage: {}\x1b[0m", com_token["error"]["code"], com_token["error"]["message"]);
            return;
        } else {
            let com_token = com_token["data"]["token"].to_string();
            match fs::write(&format!("{}/token.txt", current_dir().unwrap().to_str().unwrap()), &com_token) {
                Ok(_) => {
                    username =
                        json::stringify(get_username(&token).await["data"]["symbol"].clone())
                            .replace('\"', "");
                        credits = json::stringify(get_credits(&token).await.clone());
                        token = fs::read_to_string(&format!("{}/token.txt", current_dir().unwrap().to_str().unwrap())).unwrap_or("".to_string());
                        println!("Please type refersh!");
                },
                Err(e) => println!("Falied: {}", e),
            }
        }
    }

    let default_system = list_systems(&token, 20, 1).await["data"][0]["symbol"].to_string().replace('\"', "");
    let default_waypoint = list_waypoints(&token, 1, 20, &default_system).await["data"][0]["symbol"].to_string().replace('\"', "");
    
    varibles.insert("default_system".to_string(), default_system.clone());
    varibles.insert("default_waypoint".to_string(), default_waypoint.clone());

    loop {
        let command = input(format!("\x1b[38;5;28m@{}\x1b[0m ➜ {}> ", username, credits).as_str());
        let mut commands: Vec<_> = command.split_whitespace().collect();

        for (key, value) in varibles.iter() {
            // println!("looking for {key}");
            if commands.contains(&format!("${key}").as_str()) {
                let index = commands
                    .iter()
                    .position(|&r| r == &format!("${key}"))
                    .unwrap();
                commands[index] = value;
            }
        }

        match commands.first() {
            Some(command) => {
                match *command {
                    // -------------------- STANDARD -------------------- //
                    "help" => match commands.get(1) {
                        Some(command) => {
                            match *command {
                                    "credits"         => println!("credits\nTakes no arguments\nShows your credits"),
                                    "quit"            => println!("quit\nTakes no arguments\nQuits the program"),
                                    "settoken"        => println!("settoken [TOKEN=\"\"]\nTakes one argument, has a default of a blank string\nSets your token"),
                                    "listSystems"     => println!("listSystems [PAGE=1] [LIMIT=20]\nTakes two arguments\nLists systems"),
                                    "getSystem"       => println!("getSystem [SYSTEM=\"{}\"]\nTakes one argument\nGets info on a system", &varibles["default_system"]),
                                    "echo"            => println!("echo [STRING=\"\"]\nTakes one argument\nPrints a string"),
                                    "waypoints"       => println!("waypoints [SYSTEM={}] [PAGE=1] [LIMIT=20]\nTakes three arguments\nLists waypoints", &varibles["default_system"]),
                                    "getWaypoint"     => println!("getWaypoint [SYSTEM={}] [WAYPOINT={}]\nTakes two arguments\nGets info on a waypoint", &varibles["default_system"], &varibles["default_waypoint"]),
                                    "getMarket"       => println!("getMarket [SYSTEM={}] [WAYPOINT={}]\nTakes two arguments\nGets info on a market", &varibles["default_system"], &varibles["default_waypoint"]),
                                    "getSipyard"      => println!("getSipyard [SYSTEM={}] [WAYPOINT={}]\nTakes two arguments\nGets info on a shipyard", &varibles["default_system"], &varibles["default_waypoint"]),
                                    "getJumpgate"     => println!("getJumpgate [SYSTEM={}] [WAYPOINT={}]\nTakes two arguments\nGets info on a jumpgate", &varibles["default_system"], &varibles["default_waypoint"]),
                                    "listContracts"   => println!("listContracts"),
                                    "getContract"     => println!("getContract [PAGE=1] [LIMIT=20]\nLists all contracts"),
                                    "acceptContract"  => println!("acceptContract [CONTRACTID=\"\"]\nAccepts a contract"),
                                    "deliverContract" => println!("deliverContract[CONTRACTID=\"\"]\nDelivers a contract"),
                                    "fulfillContract" => println!("fulfillContract[CONTRACTID=\"\"]\nFullfills a contract"),
                                    "export"          => println!("export [FILENAME=\"varibles.json\"]\nExports varibles to a file"),
                                    "load"            => println!("load [FILENAME=\"varibles.json\"]\nImports varibles from a file"),
                                    _ => println!("spacetraders: {}: command not found", command),
                                }
                        }

                        None => {
                           
                        }
                    },
                    "set" => {
                        let name = commands.get(1).unwrap_or(&"").trim();
                        let value = commands.get(2..).unwrap_or(&[""]);
                        let mut added = "".to_string();

                        if value.len() == 1 {
                            added = value[0].to_string();
                        } else if value.len() == 0 {
                            continue;
                        } else {
                            for (i, v) in value.iter().enumerate() {
                                added.push_str(v);
                                
                                if i == value.len() - 1 {
                                    continue;
                                } else {
                                    added.push_str(" ");
                                }
                            }
                        }

                        varibles.insert(name.to_string(), added);
                    }
                    "credits" => println!("You have {} credits", get_credits(&token).await),
                    "echo" => {
                        let value = commands.get(1..).unwrap_or(&[""]);
                        let mut added = "".to_string();

                        if value.len() == 1 {
                            added = value[0].to_string();
                        } else if value.len() == 0 {
                            continue;
                        } else {
                            for (i, v) in value.iter().enumerate() {
                                added.push_str(v);
                                
                                if i == value.len() - 1 {
                                    continue;
                                } else {
                                    added.push_str(" ");
                                }
                            }
                        }

                        println!("{}", added)
                    }
                    "quit" => {
                        let file = Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string() + ".json";
                        let json = serde_json::to_string(&varibles).unwrap();

                        match fs::write(&file, json) {
                            Ok(_) => println!("Exported varibles to {}", &file),
                            Err(_) => println!("Failed to export varibles to {}", &file),
                        }
                        break;
                    }
                    "export" => {
                        let time = Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string() + ".json";
                        let time = &time.as_str();
                        let file = commands.get(1).unwrap_or(time);
                        let json = serde_json::to_string(&varibles).unwrap();

                        match fs::write(file, json) {
                            Ok(_) => println!("Exported varibles to {}", file),
                            Err(_) => println!("Failed to export varibles to {}", file),
                        }
                    }
                    "load" => {
                        let file = commands.get(1).unwrap_or(&"varibles.json");
                        match fs::read_to_string(file) {
                            Ok(json) => {
                                let temp_varibles: HashMap<String, String> =
                                    serde_json::from_str(&json).unwrap_or(HashMap::new());

                                varibles = temp_varibles;
                            }
                            Err(_) => {
                                println!("Failed to load varibles from {}", file);
                                continue;
                            }
                        };
                    }
                    "settoken" => {
                        let com_token = commands.get(1).unwrap_or(&"").to_string();

                        match fs::write(&format!("{}/token.txt", current_dir().unwrap().to_str().unwrap()), &com_token) {
                            Ok(_) => println!("Set token to {}", &com_token),
                            Err(_) => println!("Failed to set token to {}", &com_token),
                        }
                    }
                    "refresh" => {
                        username =
                            json::stringify(get_username(&token).await["data"]["symbol"].clone())
                                .replace('\"', "");
                        credits = json::stringify(get_credits(&token).await.clone());
                        token = fs::read_to_string(&format!("{}/token.txt", current_dir().unwrap().to_str().unwrap())).unwrap_or("".to_string());
                    }
                    "clear" => print!("{esc}[2J{esc}[1;1H", esc = 27 as char),
                    // -------------------- SYSTEMS -------------------- //
                    "listSystems" => {
                        // println!(
                        //     "{:?}, {:?}",
                        //     commands.get(1),
                        //     commands
                        //         .get(2)
                        //         .unwrap_or(&"")
                        //         .parse::<u64>()
                        //         .unwrap_or(31101)
                        // );
                        let limit = commands
                            .get(2)
                            .unwrap_or(&"20")
                            .parse::<u64>()
                            .unwrap_or(20);

                        let page = commands.get(1).unwrap_or(&"1").parse::<u64>().unwrap_or(1);

                        let data = list_systems(&token, limit, page).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "getSystem" => {
                        let system = commands.get(1).unwrap_or(&varibles["default_system"].as_str()).to_string();

                        let data = get_system(&token, system).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "waypoints" => {
                        let limit = commands
                            .get(3)
                            .unwrap_or(&"20")
                            .parse::<u64>()
                            .unwrap_or(20);

                        let page = commands.get(2).unwrap_or(&"1").parse::<u64>().unwrap_or(1);

                        let system = commands.get(1).unwrap_or(&varibles["default_system"].as_str()).to_string();

                        let data = list_waypoints(&token, page, limit, &system).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "getWaypoint" => {
                        let system = commands.get(1).unwrap_or(&varibles["default_system"].as_str()).to_string();

                        let waypoint = commands.get(2).unwrap_or(&varibles["default_waypoint"].as_str()).to_string();

                        let data = get_waypoint(&token, system, waypoint).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "getMarket" => {
                        let system = commands.get(1).unwrap_or(&varibles["default_system"].as_str()).to_string();

                        let waypoint = commands.get(2).unwrap_or(&varibles["default_waypoint"].as_str()).to_string();

                        let data = get_market(&token, system, waypoint).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "getSipyard" => {
                        let system = commands.get(1).unwrap_or(&varibles["default_system"].as_str()).to_string();

                        let waypoint = commands.get(2).unwrap_or(&varibles["default_waypoint"].as_str()).to_string();
                        
                        let data = get_shipyard(&token, system, waypoint).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "getJumpgate" => {
                        let system = commands.get(1).unwrap_or(&varibles["default_system"].as_str()).to_string();

                        let waypoint = commands.get(2).unwrap_or(&varibles["default_waypoint"].as_str()).to_string();

                        let data = get_jumpgate(&token, system, waypoint).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    // -------------------- CONTRACTS -------------------- //
                    "listContracts" => {
                        let limit = commands
                            .get(2)
                            .unwrap_or(&"20")
                            .parse::<u64>()
                            .unwrap_or(20);

                        let page = commands.get(1).unwrap_or(&"20").parse::<u64>().unwrap_or(1);

                        let data = list_contracts(&token, page, limit).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "getContract" => {
                        let contract_id = commands.get(1).unwrap_or(&"").to_string();

                        let data = get_contract(&token, contract_id).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "acceptContract" => {
                        let contract_id = commands.get(1).unwrap_or(&"").to_string();

                        let data = accept_contract(&token, contract_id).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "deliverContract" => {
                        let contract_id = commands.get(1).unwrap_or(&"").to_string();

                        let data = deliver_contract(&token, contract_id).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "fulfillContract" => {
                        let contract_id = commands.get(1).unwrap_or(&"").to_string();

                        let data = fulfill_contract(&token, contract_id).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    // ---------------------- FLEET ---------------------- //
                    "listShips" => {
                        let limit = commands
                            .get(2)
                            .unwrap_or(&"20")
                            .parse::<u64>()
                            .unwrap_or(20);

                        let page = commands.get(1).unwrap_or(&"1").parse::<u64>().unwrap_or(1);

                        let data = list_ships(&token, page, limit).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "listShips: {}\nCode: {}\x1b[0m",
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "buyShip" => {
                        let location = commands.get(1).unwrap_or(&varibles["default_waypoint"].as_str()).to_string();

                        let ship_type = match commands.get(2) {
                            Some(ship_type) => {
                                let ship_type = *ship_type;

                                match ship_type {
                                    "SHIP_PROBE" => "SHIP_PROBE".to_string(),
                                    "SHIP_MINING_DRONE" => "SHIP_MINING_DRONE".to_string(),
                                    "SHIP_INTERCEPTOR" => "SHIP_INTERCEPTOR".to_string(),
                                    "SHIP_LIGHT_HAULER" => "SHIP_LIGHT_HAULER".to_string(),
                                    "SHIP_COMMAND_FRIGATE" => "SHIP_COMMAND_FRIGATE".to_string(),
                                    "SHIP_EXPLORER" => "SHIP_EXPLORER".to_string(),
                                    "SHIP_HEAVY_FREIGHTER" => "SHIP_HEAVY_FREIGHTER".to_string(),
                                    "SHIP_LIGHT_SHUTTLE" => "SHIP_LIGHT_SHUTTLE".to_string(),
                                    "SHIP_ORE_HOUND" => "SHIP_ORE_HOUND".to_string(),
                                    "SHIP_REFINING_FREIGHTER" => {
                                        "SHIP_REFINING_FREIGHTER".to_string()
                                    }
                                    _ => {
                                        eprintln!(
                                            "\x1b[3m\x1b[38;5;160m{}: invalid ship type\x1b[0m",
                                            commands[0]
                                        );
                                        continue;
                                    }
                                }
                            }
                            None => "SHIP_PROBE".to_string(),
                        };

                        let data = purchase_ship(&token, location, ship_type).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160mbuyShip: {}\nCode: {}\x1b[0m",
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "getShip" => {
                        let ship_id = commands
                            .get(1)
                            .unwrap_or(&format!("{}-1", username).as_str())
                            .to_string();

                        let data = get_ship(&token, ship_id).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160mgetShip: {}\nCode: {}\x1b[0m",
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "getShipCargo" => {
                        let ship_id = commands
                            .get(1)
                            .unwrap_or(&format!("{}-1", username).as_str())
                            .to_string();

                        let data = get_ship_cargo(&token, ship_id).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160mgetShip: {}\nCode: {}\x1b[0m",
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "orbitShip" => {
                        let ship_id = commands
                            .get(1)
                            .unwrap_or(&format!("{}-1", username).as_str())
                            .to_string();

                        let data = orbit_ship(&token, ship_id).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160mgetShip: {}\nCode: {}\x1b[0m",
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "shipRefine" => {
                        let ship_id = commands
                            .get(1)
                            .unwrap_or(&format!("{}-1", username).as_str())
                            .to_string();

                        let proudce = match commands.get(2) {
                            Some(proudce) => match *proudce {
                                "IRON" => String::from("IRON"),
                                "COPPER" => String::from("COPPER"),
                                "SILVER" => String::from("SILVER"),
                                "GOLD" => String::from("GOLD"),
                                "ALUMINUM" => String::from("ALUMINUM"),
                                "PLATINUM" => String::from("PLATINUM"),
                                "URANITE" => String::from("URANITE"),
                                "MERITIUM" => String::from("MERITIUM"),
                                "FUEL" => String::from("FUEL"),
                                _ => {
                                    eprintln!(
                                        "\x1b[3m\x1b[38;5;160m{}: invalid proudce\x1b[0m",
                                        commands[0]
                                    );
                                    continue;
                                }
                            },
                            None => "FUEL".to_string(),
                        };

                        let data = ship_refine(&token, ship_id, proudce).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160mgetShip: {}\nCode: {}\x1b[0m",
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "createChart" => {
                        let ship_id = commands
                            .get(1)
                            .unwrap_or(&format!("{}-1", username).as_str())
                            .to_string();

                        let data = create_chart(&token, ship_id).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160mgetShip: {}\nCode: {}\x1b[0m",
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "getShipCooldown" => {
                        let ship_id = commands
                            .get(1)
                            .unwrap_or(&format!("{}-1", username).as_str())
                            .to_string();

                        let data = get_ship_cooldown(&token, ship_id).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160mgetShip: {}\nCode: {}\x1b[0m",
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "dockShip" => {
                        let ship_id = commands
                            .get(1)
                            .unwrap_or(&format!("{}-1", username).as_str())
                            .to_string();

                        let data = doc_ship(&token, ship_id).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "createSurvay" => {
                        let ship_id = commands
                            .get(1)
                            .unwrap_or(&format!("{}-1", username).as_str())
                            .to_string();

                        let data = create_survey(&token, ship_id).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160mcreateSurvay: {}\nCode: {}\x1b[0m",
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "ejectCrago" => {
                        let ship_symbol = commands
                            .get(1)
                            .unwrap_or(&format!("{}-1", username).as_str())
                            .to_string();

                        let cargo_symbol = commands
                            .get(2)
                            .unwrap_or(&"")
                            .to_string();

                        let amount = commands
                            .get(3)
                            .unwrap_or(&"")
                            .to_string()
                            .parse::<usize>()
                            .unwrap_or(0);

                        let data = jettison_cargo(&token, ship_symbol, cargo_symbol, amount).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "jumpShip" => {
                        let ship_id = commands
                            .get(1)
                            .unwrap_or(&format!("{}-1", username).as_str())
                            .to_string();

                        let system_symbol = commands
                            .get(2)
                            .unwrap_or(&"")
                            .to_string();

                        let data = jump_ship(&token, ship_id, system_symbol).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "navShip" => {
                        let ship_id = commands
                            .get(1)
                            .unwrap_or(&format!("{}-1", username).as_str())
                            .to_string();

                        let waypoint_symbol = commands
                            .get(2)
                            .unwrap_or(&"")
                            .to_string();

                        let data = navigate_ship(&token, ship_id, waypoint_symbol).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "patchShipNav" => {
                        let ship_id = commands
                            .get(1)
                            .unwrap_or(&format!("{}-1", username).as_str())
                            .to_string();

                        let flight_mode = commands
                            .get(2)
                            .unwrap_or(&"CRUISE")
                            .to_string();

                        match flight_mode.as_str() {
                            "DRIFT"  => (),
                            "STELTH" => (),
                            "CRUISE" => (),
                            "BURN"   => (),
                            _ => {
                                eprintln!(
                                    "\x1b[3m\x1b[38;5;160m{}: invalid flight mode\x1b[0m",
                                    commands[0]
                                );
                                continue;
                            }
                        }

                        let data = patch_ship_nav(&token, ship_id, flight_mode).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "getShipNav" => {
                        let ship_id = commands
                            .get(1)
                            .unwrap_or(&format!("{}-1", username).as_str())
                            .to_string();

                        let data = get_ship_nav(&token, ship_id).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "warpShip" => {
                        let ship_id = commands
                            .get(1)
                            .unwrap_or(&format!("{}-1", username).as_str())
                            .to_string();

                        let waypoint_symbol = commands
                            .get(2)
                            .unwrap_or(&"")
                            .to_string();

                        let data = warp_ship(&token, ship_id, waypoint_symbol).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "sellCargo" => {
                        let ship_id = commands
                            .get(1)
                            .unwrap_or(&format!("{}-1", username).as_str())
                            .to_string();

                        let cargo_symbol = commands
                            .get(2)
                            .unwrap_or(&"")
                            .to_string();

                        let amount = commands
                            .get(3)
                            .unwrap_or(&"0")
                            .to_string()
                            .parse::<usize>()
                            .unwrap_or(0);

                        let data = sell_cargo(&token, ship_id, cargo_symbol, amount).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "scanSystems" => {
                        let ship_id = commands
                            .get(1)
                            .unwrap_or(&format!("{}-1", username).as_str())
                            .to_string();

                        let data = scan_systems(&token, ship_id).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "scanWaypoints" => {
                        let ship_id = commands
                            .get(1)
                            .unwrap_or(&format!("{}-1", username).as_str())
                            .to_string();

                        let data = scan_waypoints(&token, ship_id).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "scanShips" => {
                        let ship_id = commands
                            .get(1)
                            .unwrap_or(&format!("{}-1", username).as_str())
                            .to_string();

                        let data = scan_ships(&token, ship_id).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "refuel" => {
                        let ship_id = commands
                            .get(1)
                            .unwrap_or(&format!("{}-1", username).as_str())
                            .to_string();

                        let data = refuel(&token, ship_id).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "buyCargo" => {
                        let ship_id = commands
                            .get(1)
                            .unwrap_or(&format!("{}-1", username).as_str())
                            .to_string();

                        let cargo_symbol = commands
                            .get(2)
                            .unwrap_or(&"")
                            .to_string();

                        let amount = commands
                            .get(3)
                            .unwrap_or(&"0")
                            .to_string()
                            .parse::<usize>()
                            .unwrap_or(0);

                        let data = buy_cargo(&token, ship_id, cargo_symbol, amount).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    "transferCargo" => {
                        let ship_id = commands
                            .get(1)
                            .unwrap_or(&format!("{}-1", username).as_str())
                            .to_string();

                        let cargo_symbol = commands
                            .get(2)
                            .unwrap_or(&"")
                            .to_string();

                        let amount = commands
                            .get(3)
                            .unwrap_or(&"0")
                            .to_string()
                            .parse::<usize>()
                            .unwrap_or(0);

                        let reseiving_ship = commands
                            .get(1)
                            .unwrap_or(&format!("{}-2", username).as_str())
                            .to_string();

                        let data = transfer_cargo(&token, ship_id, cargo_symbol, amount, reseiving_ship).await;

                        if json::stringify_pretty(data["data"].clone(), 4) == *"null" {
                            eprintln!(
                                "\x1b[3m\x1b[38;5;160m{}: {}\nCode: {}\x1b[0m",
                                commands[0],
                                data["error"]["message"].clone(),
                                data["error"]["code"].clone()
                            );
                        } else {
                            println!("{}", json::stringify_pretty(data["data"].clone(), 4));
                        }
                    }
                    _ => println!(
                        "\x1b[3m\x1b[38;5;160mspacetraders: {}: command not found\x1b[0m",
                        commands[0]
                    ),
                }
            }

            None => continue,
        }
    }
}
