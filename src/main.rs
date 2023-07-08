mod test;
fn main() {
  let answer : test::LnLambdaInputs = test::LnLambdaInputs::get_settings();
  let query_string = LnLambda{node_id : answer.node_info.node_id, host : answer.node_info.host, rune : answer.rune.rune, server : "lnlambda.lnmetrics.info/".to_string()};
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