use std::rc::Rc;

// use std::thread;
use log::info;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;

use web_sys::window;
use yew::{
    function_component, html, use_state, Callback, Html, InputEvent, MouseEvent, TargetCast,
};
use yew_router::hooks::use_navigator;

use crate::router::Route;
// use yew::prelude::*;

#[derive(Deserialize)]
struct LoginResponse {
    success: bool,
    data: Option<Value>,
    message: Option<String>,
}

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let navigator = use_navigator().unwrap();
    let navigator = Rc::new(navigator);
    // let onclick = Callback::from(move|_| navigator.push(&Route::Login));

    // State hooks for email and password
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    let response = use_state(|| None::<String>); //::<String> // ::<Value>

    let on_email_input = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            email.set(input);
        })
    };

    let on_password_input = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            password.set(input);
        })
    };

    let onclick = {
        let email = email.clone();
        let password = password.clone();
        let response = response.clone();
        Callback::from(move |_: MouseEvent| {
            let email = email.clone();
            let password = password.clone();
            let response = response.clone();
            let navigator = Rc::clone(&navigator);
            spawn_local(async move {
                // Here, instead of blocking, you would typically use an async HTTP client.
                // For demonstration, we'll use blocking reqwest inside a synchronous block.
                let res = send_login_request(&email, &password).await;
                // response.set(Some(res.unwrap_or_else(|_| "Error".into())));

                // if let Some(success) = res.unwrap().get("success") {
                //     println!("Success: {}", success.as_str().unwrap());
                //     info!("Login API response");
                //     web_sys::console::log_1(&JsValue::from_str(&serde_json::to_string(success).unwrap()));
                // }
                match res {
                    Ok(login_response) => {
                        if login_response.success {
                            if let Some(token) = login_response.data.unwrap().get("token") {
                                if let Some(storage) = window().unwrap().local_storage().unwrap() {
                                    storage.set_item("yew_shop_auth_token", serde_json::to_string(token).unwrap().as_str().trim_matches('"')).unwrap();
                                }
                                // response.set(Some("Login successful".to_string()));
                                // Redirect to home
                                navigator.push(&Route::Home);
                            }
                        } else {
                            // Set the response state with the message field
                            if let Some(message) = login_response.message {
                                response.set(Some(message.to_string().replace("\"", "")));
                            } else {
                                response.set(Some("An unknown error occurred.".to_string()));
                            }
                        }
                    }
                    Err(_) => {
                        // Handle the error, e.g., set response state to a generic error message
                        response.set(Some("Failed to connect to the server.".to_string()));
                    }
                }
            });
            // navigator.push(&Route::Home)
            // thread::spawn(move || {
            //     let res = send_login_request(&email, &password);
            //     response.set(Some(res.unwrap_or_else(|_| "Error".into())));
            // });
        })
        // Callback::from(move|_: MouseEvent| navigator.push(&Route::Home))
    };

    html! {
      <div class="w-full mx-auto max-w-sm bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-700 dark:border-gray-700">
        <div class="text-center my-2">
              { if let Some(res) = &*response {
                  html! { <p class="text-red-400">{ res }</p> }
              } else {
                  html! { <p>{ "" }</p> }
              }}
          </div>
          <div class="p-6">
              <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{"Login"}</h5>
              <p class="mb-3 font-normal text-gray-700 dark:text-gray-400">
                  {"Enter your email below to login to your account."}
              </p>
          </div>
          <div class="p-6 grid gap-4">
              <div class="grid gap-2">
                  <label for="email" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Email"}</label>
                  <input type="email" id="email" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    placeholder="my@email.com"
                    required=true
                    value={(*email).clone()}
                    oninput={on_email_input}
                  />
              </div>
              <div class="grid gap-2">
                  <label for="password" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"Password"}</label>
                  <input type="password" id="password" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    required=true
                    value={(*password).clone()}
                    oninput={on_password_input}
                  />
              </div>
          </div>
          <div class="p-6">
              <button type="button" {onclick} class="w-full text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 mr-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800">
                  {"Login"}
              </button>
          </div>
      </div>
    }
}

async fn send_login_request(email: &str, password: &str) -> Result<LoginResponse, reqwest::Error> {
    let url = "http://localhost:8888/api/v1/login";  
    let client = Client::new();
    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(format!(
            r#"{{"email": "{}", "password": "{}"}}"#,
            email, password
        ))
        .send()
        .await? // Await the future here
        .json::<LoginResponse>()
        .await?; // Await the text extraction as well
    Ok(res)
}
