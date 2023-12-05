use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew_router::prelude::*;
use yew::prelude::*;
use crate::app::{invoke, Route};

#[derive(Serialize, Deserialize)]
struct ProxyArgs<'a> {
    ip: &'a str,
    port: &'a str,
    ciphers: &'a str,
    password: &'a str,
}

#[function_component(Settings)]
pub fn settings() -> Html {
    let ipaddr_input_ref = use_node_ref();
    let port_input_ref = use_node_ref();
    let ciphers_ref = use_node_ref();
    let password_input_ref = use_node_ref();

    let ipaddr = use_state(|| String::new());
    let port = use_state(|| String::new());
    let ciphers = use_state(|| String::new());
    let password = use_state(|| String::new());

    let setproxy = {
        let ipaddr = ipaddr.clone();
        let ipaddr_input_ref = ipaddr_input_ref.clone();
        let port = port.clone();
        let port_input_ref = port_input_ref.clone();
        let ciphers = ciphers.clone();
        let ciphers_ref = ciphers_ref.clone();
        let password = password.clone();
        let password_input_ref = password_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            ipaddr.set(
                ipaddr_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
            port.set(
                port_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
            ciphers.set(
                ciphers_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
            password.set(
                password_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
            let ipaddr_clone = ipaddr.clone();
            let port_clone = port.clone();
            let ciphers_clone = ciphers.clone();
            let password_clone = password.clone();
            spawn_local(async move {
                if ipaddr_clone.is_empty() {
                    return;
                }
                if port_clone.is_empty() {
                    return;
                }
                if ciphers_clone.is_empty() {
                    return;
                }
                if password_clone.is_empty() {
                    return;
                }

                let args = to_value(&ProxyArgs {
                    ip: &*ipaddr_clone,
                    port: &*port_clone,
                    ciphers: &*ciphers_clone,
                    password: &*password_clone
                }).unwrap();
                invoke("downproxy", JsValue::NULL).await;
                invoke("setproxy", args).await;
                invoke("upproxy", JsValue::NULL).await;
            });
        })
    };

    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div class="container">
            <h1>{ "设置" }</h1>

            <form class="container" onsubmit={setproxy}>
                <input id="ipaddr-input" ref={ipaddr_input_ref} placeholder="Enter a IP address..." />
                <input id="port-input" ref={port_input_ref} placeholder="Enter a IP port..." />
                <select id="ciphers" ref={ciphers_ref}>
                    <option value="aes-128-gcm">{"aes-128-gcm"}</option>
                    <option value="aes-256-gcm">{"aes-256-gcm"}</option>
                    <option value="chacha20-ietf-poly1305">{"chacha20-ietf-poly1305"}</option>
                    <option value="2022-blake3-aes-128-gcm">{"2022-blake3-aes-128-gcm"}</option>
                    <option value="2022-blake3-aes-256-gcm">{"2022-blake3-aes-256-gcm"}</option>
                    <option value="2022-blake3-chacha20-poly1305">{"2022-blake3-chacha20-poly1305"}</option>
                    <option value="2022-blake3-chacha8-poly1305">{"2022-blake3-chacha8-poly1305"}</option>
                </select>
                <input id="password-input" ref={password_input_ref} placeholder="Enter a password..." />
                <button type="submit">{"保存"}</button>
            </form>

            <p><b>{ &*ipaddr }{":"}{ &*port }{"\t"}{ &*ciphers }{"\t"}{ &*password }</b></p>

            <button {onclick}>{ "<-    返回" }</button>
        </div>
    }
}