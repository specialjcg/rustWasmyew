use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub team: String,
}
#[function_component(Card)]
pub fn card(Props {team}: &Props ) -> Html {
    html! {


<div href="#" class="flex flex-col items-center bg-white border rounded-lg shadow-md md:flex-row md:max-w-xl hover:bg-gray-100 dark:border-gray-700 dark:bg-gray-800 dark:hover:bg-gray-700">
    <img class="max-w-xs h-auto rounded-lg" src="https://picsum.photos/100/200" alt=""/>
    <div class="flex flex-col justify-between p-4 leading-normal">
        <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{team.clone()}</h5>
        <p class="mb-3 font-normal text-gray-700 dark:text-gray-400">{"Here are the biggest enterprise technology acquisitions of 2021 so far, in reverse chronological order."}</p>

        <button type="button" class="text-white bg-gradient-to-r from-green-400 to-blue-500 hover:from-pink-500 hover:to-yellow-500 shadow-lg shadow-purple-500/50 dark:shadow-lg dark:shadow-purple-800/80 font-medium rounded-lg text-sm px-5 py-2.5 text-center mr-2 mb-2">{"Hover me"}</button>
    </div>
</div>


    }
}