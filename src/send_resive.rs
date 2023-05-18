#![allow(unused)]
use json::{self, parse, JsonValue};
use serde_json::Value;
use std::collections::HashMap;
use std::io::Write;
pub enum Type {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

pub fn input(prompt: &str) -> String {
    use std::io::{stdin, stdout};
    let mut s = String::new();
    print!("{}", prompt);
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Something happened.");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }

    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    s
}

pub async fn request<K: serde::ser::Serialize, V: serde::Serialize>(
    token: &String,
    endpoint: &str,
    data: &HashMap<K, V>,
    req_type: Type,
) -> JsonValue {
    let client = reqwest::Client::new();
    match req_type {
        Type::GET => {
            if data.is_empty() {
                let data: HashMap<String, String> = HashMap::new();
                let res = client
                    .get(format!("https://api.spacetraders.io/v2/{}", endpoint))
                    .form(&data)
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await
                    .unwrap();
                let json_res = res.json::<serde_json::Value>();
                
                parse(
                    &serde_json::to_string_pretty(&json_res.await.unwrap_or(Value::Null))
                        .unwrap_or(JsonValue::Null.to_string()),
                )
                .unwrap()
            } else {
                let res = client
                    .get(format!("https://api.spacetraders.io/v2/{}", endpoint))
                    .form(&data)
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await
                    .unwrap();
                let json_res = res.json::<serde_json::Value>();
                
                parse(
                    &serde_json::to_string_pretty(&json_res.await.unwrap_or(Value::Null))
                        .unwrap_or(JsonValue::Null.to_string()),
                )
                .unwrap()
            }

        }

        Type::POST => {
            if data.is_empty() {
                let data: HashMap<String, String> = HashMap::new();
                let res = client
                    .post(format!("https://api.spacetraders.io/v2/{}", endpoint))
                    .form(&data)
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await
                    .unwrap();
                let json_res = res.json::<serde_json::Value>();
                
                parse(
                    &serde_json::to_string_pretty(&json_res.await.unwrap_or(Value::Null))
                        .unwrap_or(JsonValue::Null.to_string()),
                )
                .unwrap()
            } else {
                let res = client
                    .post(format!("https://api.spacetraders.io/v2/{}", endpoint))
                    .form(&data)
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await
                    .unwrap();
                let json_res = res.json::<serde_json::Value>();
                
                parse(
                    &serde_json::to_string_pretty(&json_res.await.unwrap_or(Value::Null))
                        .unwrap_or(JsonValue::Null.to_string()),
                )
                .unwrap()
            }

        }

        Type::PUT => {
            if data.is_empty() {
                let data: HashMap<String, String> = HashMap::new();
                let res = client
                    .put(format!("https://api.spacetraders.io/v2/{}", endpoint))
                    .form(&data)
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await
                    .unwrap();
                let json_res = res.json::<serde_json::Value>();
                
                parse(
                    &serde_json::to_string_pretty(&json_res.await.unwrap_or(Value::Null))
                        .unwrap_or(JsonValue::Null.to_string()),
                )
                .unwrap()
            } else {
                let res = client
                    .put(format!("https://api.spacetraders.io/v2/{}", endpoint))
                    .form(&data)
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await
                    .unwrap();
                let json_res = res.json::<serde_json::Value>();
                
                parse(
                    &serde_json::to_string_pretty(&json_res.await.unwrap_or(Value::Null))
                        .unwrap_or(JsonValue::Null.to_string()),
                )
                .unwrap()
            }

        }

        Type::DELETE => {
            if data.is_empty() {
                let data: HashMap<String, String> = HashMap::new();
                let res = client
                    .delete(format!("https://api.spacetraders.io/v2/{}", endpoint))
                    .form(&data)
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await
                    .unwrap();
                let json_res = res.json::<serde_json::Value>();
                
                parse(
                    &serde_json::to_string_pretty(&json_res.await.unwrap_or(Value::Null))
                        .unwrap_or(JsonValue::Null.to_string()),
                )
                .unwrap()
            } else {
                let res = client
                    .delete(format!("https://api.spacetraders.io/v2/{}", endpoint))
                    .form(&data)
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await
                    .unwrap();
                let json_res = res.json::<serde_json::Value>();
                
                parse(
                    &serde_json::to_string_pretty(&json_res.await.unwrap_or(Value::Null))
                        .unwrap_or(JsonValue::Null.to_string()),
                )
                .unwrap()
            }

        }

        Type::PATCH => {
            if data.is_empty() {
                let data: HashMap<String, String> = HashMap::new();
                let res = client
                    .patch(format!("https://api.spacetraders.io/v2/{}", endpoint))
                    .form(&data)
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await
                    .unwrap();
                let json_res = res.json::<serde_json::Value>();
                
                parse(
                    &serde_json::to_string_pretty(&json_res.await.unwrap_or(Value::Null))
                        .unwrap_or(JsonValue::Null.to_string()),
                )
                .unwrap()
            } else {
                let res = client
                    .patch(format!("https://api.spacetraders.io/v2/{}", endpoint))
                    .form(&data)
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await
                    .unwrap();
                let json_res = res.json::<serde_json::Value>();
                
                parse(
                    &serde_json::to_string_pretty(&json_res.await.unwrap_or(Value::Null))
                        .unwrap_or(JsonValue::Null.to_string()),
                )
                .unwrap()
            }

        }

    }

}

pub async fn get_credits(token: &String) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    let data = request(token, "my/agent", &data, Type::GET).await;
    data["data"]["credits"].clone()
}

pub async fn new_acc(faction: String, username: String) -> JsonValue {
    let mut data: HashMap<String, String> = HashMap::new();
    let client = reqwest::Client::new();

    data.insert("faction".to_string(), faction);
    data.insert("symbol".to_string(), username);
    data.insert("email".to_string(), "a@gmail.com".to_string());    
    let res = client
        .post(format!("https://api.spacetraders.io/v2/{}", "register"))
        .form(&data)
        .send()
        .await
        .unwrap();
    let json_res = res.json::<serde_json::Value>();
    
    parse(
        &serde_json::to_string_pretty(&json_res.await.unwrap_or(Value::Null))
            .unwrap_or(JsonValue::Null.to_string()),
    )
    .unwrap()
}

// ------------------- SYSTEMS ------------------- //
pub async fn list_systems(token: &String, limit: u64, page: u64) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(
        token,
        &format!("systems?page={}&?limit={}", page, limit),
        &data,
        Type::GET,
    )
    .await
}

pub async fn get_system(token: &String, system: String) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(token, &format!("systems/{}", system), &data, Type::GET).await
}

pub async fn list_waypoints(
    token: &String,
    page: u64,
    limit: u64,
    system_symbol: &String,
) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(
        token,
        &format!(
            "systems/{}/waypoints?page={}&limit={}",
            system_symbol, page, limit
        ),
        &data,
        Type::GET,
    )
    .await
}

pub async fn get_username(token: &String) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(token, "my/agent", &data, Type::GET).await
}

pub async fn get_waypoint(
    token: &String,
    system_symbol: String,
    waypoint_symbol: String,
) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(
        token,
        &format!("systems/{}/waypoints/{}", system_symbol, waypoint_symbol),
        &data,
        Type::GET,
    )
    .await
}

pub async fn get_market(token: &String, system: String, waypoint: String) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(
        token,
        &format!("systems/{}/waypoints/{}/market", system, waypoint),
        &data,
        Type::GET,
    )
    .await
}

pub async fn get_shipyard(token: &String, system: String, waypoint: String) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(
        token,
        &format!("systems/{}/waypoints/{}/shipyard", system, waypoint),
        &data,
        Type::GET,
    )
    .await
}

pub async fn get_jumpgate(token: &String, system: String, waypoint: String) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(
        token,
        &format!("systems/{}/waypoints/{}/jump-gate", system, waypoint),
        &data,
        Type::GET,
    )
    .await
}

// ------------------- CONTRACTS ------------------- //
pub async fn list_contracts(token: &String, page: u64, limit: u64) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(
        token,
        &format!("my/contracts?page={}&limit={}", page, limit),
        &data,
        Type::GET,
    )
    .await
}

pub async fn get_contract(token: &String, contract_id: String) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(
        token,
        &format!("my/contracts/{}", contract_id),
        &data,
        Type::GET,
    )
    .await
}

pub async fn accept_contract(token: &String, contract_id: String) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(
        token,
        &format!("my/contracts/{}/accept", contract_id),
        &data,
        Type::POST,
    )
    .await
}

pub async fn deliver_contract(token: &String, contract_id: String) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(
        token,
        &format!("my/contracts/{}/deliver", contract_id),
        &data,
        Type::DELETE,
    )
    .await
}

pub async fn fulfill_contract(token: &String, contract_id: String) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(
        token,
        &format!("my/contracts/{}/fulfill", contract_id),
        &data,
        Type::POST,
    )
    .await
}

// ------------------- FLEET ------------------- //
pub async fn list_ships(token: &String, page: u64, limit: u64) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(
        token,
        &format!("my/ships?page={}&limit={}", page, limit),
        &data,
        Type::GET,
    )
    .await
}

pub async fn purchase_ship(token: &String, ship_type: String, waypoint: String) -> JsonValue {
    let mut data: HashMap<String, String> = HashMap::new();

    data.insert("shipType".to_string(), ship_type);
    data.insert("waypointSymbol".to_string(), waypoint);
    
    request(token, "my/ships/", &data, Type::POST).await
}

pub async fn get_ship(token: &String, ship_symbol: String) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(
        token,
        &format!("my/ships/{}", ship_symbol),
        &data,
        Type::GET,
    )
    .await
}

pub async fn get_ship_cargo(token: &String, ship_symbol: String) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(
        token,
        &format!("my/ships/{}/cargo", ship_symbol),
        &data,
        Type::GET,
    )
    .await
}

pub async fn orbit_ship(token: &String, ship_symbol: String) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(
        token,
        &format!("my/ships/{}/orbit", ship_symbol),
        &data,
        Type::POST,
    )
    .await
}

pub async fn ship_refine(token: &String, ship_symbol: String, produce: String) -> JsonValue {
    let mut data: HashMap<String, String> = HashMap::new();

    data.insert("produce".to_string(), produce);
    
    request(
        token,
        &format!("my/ships/{}/refine", ship_symbol),
        &data,
        Type::POST,
    )
    .await
}

pub async fn create_chart(token: &String, ship_symbol: String) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(
        token,
        &format!("my/ships/{}/chart", ship_symbol),
        &data,
        Type::POST,
    )
    .await
}

pub async fn get_ship_cooldown(token: &String, ship_symbol: String) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(
        token,
        &format!("my/ships/{}/cooldown", ship_symbol),
        &data,
        Type::GET,
    )
    .await
}

pub async fn doc_ship(token: &String, ship_symbol: String) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(
        token,
        &format!("my/ships/{}/dock", ship_symbol),
        &data,
        Type::POST,
    )
    .await
}

pub async fn create_survey(token: &String, ship_symbol: String) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(
        token,
        &format!("my/ships/{}/survey", ship_symbol),
        &data,
        Type::POST,
    )
    .await
}

/// Skiped due to complexity (remamer to return)
pub async fn extract_resources(token: &String, ship_symbol: String) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(
        token,
        &format!("my/ships/{}/extract", ship_symbol),
        &data,
        Type::POST,
    )
    .await
}

pub async fn jettison_cargo(
    token: &String,
    ship_symbol: String,
    cargo_symbol: String,
    amount: usize,
) -> JsonValue {
    let mut data: HashMap<String, String> = HashMap::new();

    data.insert("symbol".to_string(), cargo_symbol);
    data.insert("units".to_string(), amount.to_string());
    
    request(
        token,
        &format!("my/ships/{}/jettison", ship_symbol),
        &data,
        Type::POST,
    )
    .await
}

pub async fn jump_ship(token: &String, ship_symbol: String, system_symbol: String) -> JsonValue {
    let mut data: HashMap<String, String> = HashMap::new();

    data.insert("systemSymbol".to_string(), system_symbol);
    
    request(
        token,
        &format!("my/ships/{}/jump", ship_symbol),
        &data,
        Type::POST,
    )
    .await
}

pub async fn navigate_ship(
    token: &String,
    ship_symbol: String,
    waypoint_symbol: String,
) -> JsonValue {
    let mut data: HashMap<String, String> = HashMap::new();

    data.insert("waypointSymbol".to_string(), waypoint_symbol);
    
    request(
        token,
        &format!("my/ships/{}/navigate", ship_symbol),
        &data,
        Type::POST,
    )
    .await
}

pub async fn patch_ship_nav(token: &String, ship_symbol: String, flight_mode: String) -> JsonValue {
    let mut data: HashMap<String, String> = HashMap::new();

    data.insert("flightMode".to_string(), flight_mode);
    
    request(
        token,
        &format!("my/ships/{}/nav", ship_symbol),
        &data,
        Type::PATCH,
    )
    .await
}

pub async fn get_ship_nav(token: &String, ship_symbol: String) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(
        token,
        &format!("my/ships/{}/nav", ship_symbol),
        &data,
        Type::GET,
    )
    .await
}

pub async fn warp_ship(token: &String, ship_symbol: String, waypoint_symbol: String) -> JsonValue {
    let mut data: HashMap<String, String> = HashMap::new();

    data.insert("waypointSymbol".to_string(), waypoint_symbol);
    
    request(
        token,
        &format!("my/ships/{}/warp", ship_symbol),
        &data,
        Type::POST,
    )
    .await
}

pub async fn sell_cargo(
    token: &String,
    ship_symbol:  String,
    cargo_symbol: String,
    amount: usize,
) -> JsonValue {
    let mut data: HashMap<String, String> = HashMap::new();

    data.insert("symbol".to_string(), cargo_symbol);
    data.insert("units".to_string(), amount.to_string());
    
    request(
        token,
        &format!("my/ships/{}/sell", ship_symbol),
        &data,
        Type::POST,
    )
    .await
}

pub async fn scan_systems(token: &String, ship_symbol: String) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(
        token,
        &format!("my/ships/{}/scan/systems", ship_symbol),
        &data,
        Type::POST,
    )
    .await
}

pub async fn scan_waypoints(token: &String, ship_symbol: String) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(
        token,
        &format!("my/ships/{}/scan/waypoints", ship_symbol),
        &data,
        Type::POST,
    )
    .await
}

pub async fn scan_ships(token: &String, ship_symbol: String) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(
        token,
        &format!("my/ships/{}/scan/ships", ship_symbol),
        &data,
        Type::POST,
    )
    .await
}

pub async fn refuel(token: &String, ship_symbol: String) -> JsonValue {
    let data: HashMap<String, String> = HashMap::new();
    
    request(
        token,
        &format!("my/ships/{}/refuel", ship_symbol),
        &data,
        Type::POST,
    )
    .await
}

pub async fn buy_cargo(
    token: &String,
    ship_symbol: String,
    cargo_symbol: String,
    amount: usize,
) -> JsonValue {
    let mut data: HashMap<String, String> = HashMap::new();

    data.insert("symbol".to_string(), cargo_symbol);
    data.insert("units".to_string(), amount.to_string());
    
    request(
        token,
        &format!("my/ships/{}/purchase", ship_symbol),
        &data,
        Type::POST,
    )
    .await
}

pub async fn transfer_cargo(
    token: &String,
    ship_symbol: String,
    cargo_symbol: String,
    amount: usize,
    reseiving_ship: String,
) -> JsonValue {
    let mut data: HashMap<String, String> = HashMap::new();

    data.insert("tradeSymbol".to_string(), cargo_symbol);
    data.insert("units".to_string(), amount.to_string());
    data.insert("shipSymbol".to_string(), reseiving_ship);
    
    request(
        token,
        &format!("my/ships/{}/transfer", ship_symbol),
        &data,
        Type::POST,
    )
    .await
}