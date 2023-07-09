use serde_json::{Value, json};
use clightningrpc::LightningRPC;


impl LnLambdaInputs {
    pub fn get_settings(ln_rpc : &LightningRPC)  -> LnLambdaInputs {
        let getinfo_output : NodeInfo = NodeInfo::getinfo(&ln_rpc);
        let getrune_output : Rune = Rune::getrune(&ln_rpc);
        LnLambdaInputs{node_info : getinfo_output, rune : getrune_output}
    }
}

impl NodeInfo {
    pub fn getinfo(ln_rpc : &LightningRPC) -> NodeInfo {
        let getinfo_output = ln_rpc.getinfo().unwrap();
        let json_data : Value = serde_json::from_str(&serde_json::to_string(&getinfo_output.binding[0]).unwrap()).expect("Failed to parse json");
        let host = format!("{}:{}", json_data["address"], json_data["port"]).replace("\"", "");
        NodeInfo { node_id : getinfo_output.id, host: host }
    }
}
impl Rune {
    pub fn getrune(ln_rpc : &LightningRPC) -> Rune{
        let getrune_output = ln_rpc.call::<serde_json::Value, serde_json::Value>("commando-rune",  json!({})).unwrap();
        let json_data : Value = serde_json::from_str(&serde_json::to_string(&getrune_output).unwrap()).expect("Failed to parse json");
        let rune = json_data["rune"].to_string().replace("\"", "");
        Rune{rune}
    }
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