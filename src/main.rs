mod test;
use clightningrpc::LightningRPC;
use std::{env, path::PathBuf};

use crate::test::LnLambdaInputs;
fn main() {
  #[allow(deprecated)]
  let pub_sock : &PathBuf = &env::home_dir().unwrap().join(".lightning/testnet/lightning-rpc");
  println!("Using socket {}", pub_sock.display());
  let client = LightningRPC::new(&pub_sock);
  let params : LnLambdaInputs = LnLambdaInputs::get_settings(&client);
  let query_string = LnLambda{node_id : params.node_info.node_id, host : params.node_info.host, rune : params.rune.rune, server : "lnlambda.lnmetrics.info/".to_string()};
  get_query_link(query_string);
}

fn get_query_link(queries : LnLambda) -> String{
  let query_link = format!("http://localhost:8080/clnapp?nodeid={}&host={}&rune={}&server={}", queries.node_id, queries.host, queries.rune, queries.server);
  println!("{}", query_link);
  query_link
}

#[derive(Debug)]
pub struct LnLambda {
  node_id : String,
  host : String,
  rune : String,
  server : String,
}