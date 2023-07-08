use std::{process::{Command, Output}};
use serde_json::{Value};

impl LnLambdaInputs {
    pub fn get_settings()  -> LnLambdaInputs {
        let getinfo_output : NodeInfo = NodeInfo::getinfo();
        let getrune_output : Rune = Rune::getrune();
        LnLambdaInputs{node_info : getinfo_output, rune : getrune_output}
    }
}

impl NodeInfo {
    pub fn getinfo() -> NodeInfo {
        let getinfo_output = Command::new("lightning-cli")
            .arg("getinfo")
            .arg("--testnet")
            .output()
            .expect("Failed to execute command");
        let temp : NodeInfo;
        if getinfo_output.status.success() {
                temp = decoder(&getinfo_output);
        } else {
            panic!("{}", String::from_utf8_lossy(&getinfo_output.stderr));    
        }
        temp
    }
}
impl Rune {
    pub fn getrune() -> Rune {
        let getrune_output = Command::new("lightning-cli")
            .arg("commando-rune")
            .arg("--testnet")
            .output()
            .expect("Failed to execute command");
    
        let mut rune : String = String::new();
    
        if getrune_output.status.success() {
            let stdout: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&getrune_output.stdout);
            let json_data: Value = serde_json::from_str(&stdout).expect("Failed to parse JSON");
            rune = json_data["rune"].to_string().replace("\"", "");
        } else {
                let stderr = String::from_utf8_lossy(&getrune_output.stderr);
                println!("Command failed");
                println!("Error:\n{}", stderr);
            }
        Rune {rune}
    }
}

    
fn decoder(output : &Output) -> NodeInfo {
    let stdout: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
    let json_data: Value = serde_json::from_str(&stdout).expect("Failed to parse JSON");
    let mut port : String = String::new();
    let mut address : String = String::new();
    let node_id = json_data["id"].to_string().replace("\"", "");
    if let Some(array) = &json_data["binding"].as_array() {
        if let Some(first_element) = array.get(0) {
            address = first_element["address"].to_string().replace("\"", "");
            port = first_element["port"].to_string();
        }
    }
    let host = format!("{}:{}", address, port);
    NodeInfo { node_id, host: host.to_string() }
}

pub struct NodeInfo {
    pub node_id : String,
    pub host : String
}

pub struct  Rune {
    pub rune : String
}

pub struct LnLambdaInputs {
    pub node_info : NodeInfo,
    pub rune : Rune
}