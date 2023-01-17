use log::info;
use serde_json::{json, to_string};
use reqwest;
use yew::prelude::*;
use yew::{function_component, html, Html};
use yew::platform::spawn_local;
use crate::app::{Props, Teams};

#[function_component(HelloServer)]
fn hello_server(Props {teams}: &Props ) -> Html {
    // let json_string = to_string(team).unwrap();
    // let json_string=serde_json::to_string_pretty(&team).unwrap();
    let json_string = json!(teams).to_string();

    // Request `/api/hello` once
        use_effect(move || {
                spawn_local(async move {
                    let client = reqwest::Client::new();
                    let response = client
                        .post("http://0.0.0.0:8081/saveteam")
                        // .body(json_string)
                        .body(json_string)
                        // confirm the request using send()
                        .send()
                        .await
                        // the rest is the same!
                        .unwrap()
                        .text()
                        .await;
                    info!("{:?}", response);
                });
            });
    html! {
                <div>{"Got server response: "}</div>
            }
}


fn my_async_fn() {}

#[function_component(AddTeam)]
pub fn add_team(Props {teams}: &Props ) -> Html {
    info!("hello from async");
    // let teams = teams.clone();
    let json_string = to_string(&teams).unwrap();


    html! {

         <form>
       <HelloServer teams={teams.clone()}/>

    <label for="default-search" class="mb-2 text-sm font-medium text-gray-900 sr-only dark:text-white">{"Add Team"}</label>
    <div class="relative">
        <div class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-plus-circle" viewBox="0 0 16 16"> <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zm0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16z"/> <path d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4z"/> </svg>
        </div>
        <input type="search" id="default-search" class="block w-full p-4 pl-10 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Search Mockups, Logos..."/>
         <button type="submit" class="text-white absolute right-2.5 bottom-2.5 bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-4 py-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800" onclick={ move |_|{my_async_fn();}} >{"Add Team"}</button>

        </div>
</form>
    }
}
