use puddle_farm_api_client_openapi_client::apis::{
    configuration::Configuration,
    default_api::{player_id_get, top_get},
};
use tokio;
#[tokio::main]
async fn main() {
    println!(
        "Top players:{:?}",
        top_get(&Configuration::new()).await.unwrap()
    );

    // println!(
    //     "{:?}",
    //     player_id_get(&Configuration::new(), 240608152606560723)
    //         .await
    //         .unwrap()
    // );
}
