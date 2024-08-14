use reqwest::Client;
use serde_json::Value;
use web_sys::window;
use serde::Deserialize;
use yew_router::hooks::use_navigator;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_effect_with, Callback, Html};

use crate::{components::LoginForm, router::Route};

#[function_component(Login)]
pub fn login() -> Html {
    let navigator = use_navigator().unwrap();
    
    use_effect_with((), move |_| {
        spawn_local(async move {
            if let Some(token) = get_token_from_local_storage() {
                match verify_user_token(&token).await {
                    Ok(response) => {
                        if !response.success {
                            navigator.push(&Route::Login);
                        }
                    }
                    Err(_) => {
                        navigator.push(&Route::Login);
                    }
                }
            } else {
                navigator.push(&Route::Login);
            }
        });
  
        || ()
    });
    
    html! {
        <div class="flex flex-col container mx-auto items-center">
            <LoginForm />
        </div>
    }
}

async fn verify_user_token(token: &str) -> Result<AuthenticatedUserResponse, reqwest::Error> {
    let url = "http://localhost:8888/api/v1/user";
    let client = Client::new();
    let res = client
        .get(url)
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await? // Await the future here
        .json::<AuthenticatedUserResponse>()
        .await?; // Await the text extraction as well
    Ok(res)
}

fn get_token_from_local_storage() -> Option<String> {
    let storage = window()?.local_storage().ok()??;
    storage.get_item("yew_shop_auth_token").ok().flatten()
}

#[derive(Deserialize)]
struct AuthenticatedUserResponse {
    success: bool,
    data: Value,
}