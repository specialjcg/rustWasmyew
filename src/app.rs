
use serde::{Deserialize, Serialize};
use serde_json::{to_string};
use yew::prelude::*;

use card::Card;
use add_team::AddTeam;


mod card;
mod add_team;

#[derive(PartialEq, Properties, Serialize, Deserialize, Clone)]
pub struct Team {
    name: String,
    price: f64,
}


#[derive(PartialEq, Properties, Serialize, Deserialize, Clone)]
pub struct Teams {
    teams: Vec<Team>,
}

#[derive(PartialEq, Properties,Serialize, Deserialize)]
pub struct Props {
    teams: Teams,
}

#[function_component(App)]
pub fn app() -> Html {
    let mut teams: Teams = Teams{teams:vec![]};
    teams.teams.push(Team { name: "Gaspard Tame".to_string(), price: 0.0 });
    teams.teams.push(Team { name: "Larry Vière".to_string(), price: 0.0 });



    let json_string = to_string(&teams).unwrap();
    let products: Vec<Html> = teams.teams.iter().map(|product: &Team| {
        html! {

             <Card  team={product.name.clone()}/>
                }
    })
        .collect();

    html! {

        <main >


        // With Properties
        <AddTeam ..Props { teams: teams }/>


        <div class="grid grid-cols-5 gap-4">
           {products}</div>
        </main>
    }
}

