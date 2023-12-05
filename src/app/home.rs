use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew_router::prelude::*;
use yew::prelude::*;
use crate::app::{invoke, Route};

#[derive(Serialize, Deserialize)]
struct SearchArgs<'a> {
    scmd: &'a str,
}

#[function_component(Home)]
pub fn home() -> Html {
    let search_input_ref = use_node_ref();
    let install_input_ref = use_node_ref();
    let bucket_input_ref = use_node_ref();
    let update_input_ref = use_node_ref();

    let name = use_state(|| String::new());
    let iname = use_state(|| String::new());
    let bname = use_state(|| String::new());
    let uname = use_state(|| String::new());

    let result_msg = use_state(|| String::new());
    {   // 处理搜索
        let result_msg = result_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if name.is_empty() {
                        return;
                    }

                    let args = to_value(&SearchArgs {
                        scmd: &(String::from("scoop ") + "search " + &*name)
                    }).unwrap();
                    result_msg.set(String::from("处理中……"));
                    if invoke("is_proxying", JsValue::NULL).await.as_bool().unwrap() {
                        invoke("pscmd", to_value(&SearchArgs {
                            scmd: r#"scoop config proxy 127.0.0.1:3128"#
                        }).unwrap()).await;
                    }
                    let new_msg = invoke("pscmd", args).await.as_string().unwrap();
                    invoke("pscmd", to_value(&SearchArgs {
                        scmd: r#"scoop config rm proxy"#
                    }).unwrap()).await;
                    result_msg.set(new_msg);
                });

                || {}
            },
            name2,
        )
    }
    {   // 处理安装
        let result_msg = result_msg.clone();
        let iname = iname.clone();
        let name2 = iname.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if iname.is_empty() {
                        return;
                    }

                    let args = to_value(&SearchArgs {
                        scmd: &(String::from("scoop ") + "install " + &*iname)
                    }).unwrap();
                    result_msg.set(String::from("处理中……"));
                    if invoke("is_proxying", JsValue::NULL).await.as_bool().unwrap() {
                        invoke("pscmd", to_value(&SearchArgs {
                            scmd: r#"scoop config proxy 127.0.0.1:3128"#
                        }).unwrap()).await;
                    }
                    let new_msg = invoke("pscmd", args).await.as_string().unwrap();
                    invoke("pscmd", to_value(&SearchArgs {
                        scmd: r#"scoop config rm proxy"#
                    }).unwrap()).await;
                    result_msg.set(new_msg);
                });

                || {}
            },
            name2,
        )
    }
    {   // 处理源
        let result_msg = result_msg.clone();
        let bname = bname.clone();
        let name2 = bname.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if bname.is_empty() {
                        return;
                    }

                    let args = to_value(&SearchArgs {
                        scmd: &(String::from(r#"scoop bucket add "#) + &*bname)
                    }).unwrap();
                    result_msg.set(String::from("处理中……"));
                    if invoke("is_proxying", JsValue::NULL).await.as_bool().unwrap() {
                        invoke("pscmd", to_value(&SearchArgs {
                            scmd: r#"scoop config proxy 127.0.0.1:3128"#
                        }).unwrap()).await;
                    }
                    let new_msg = invoke("pscmd", args).await.as_string().unwrap();
                    invoke("pscmd", to_value(&SearchArgs {
                        scmd: r#"scoop config rm proxy"#
                    }).unwrap()).await;
                    result_msg.set(new_msg);
                });

                || {}
            },
            name2,
        )
    }
    {   // 处理更新
        let result_msg = result_msg.clone();
        let uname = uname.clone();
        let name2 = uname.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if uname.is_empty() {
                        return;
                    }

                    let args = to_value(&SearchArgs {
                        scmd: &(String::from("scoop ") + "update " + &*uname)
                    }).unwrap();
                    result_msg.set(String::from("处理中……"));
                    if invoke("is_proxying", JsValue::NULL).await.as_bool().unwrap() {
                        invoke("pscmd", to_value(&SearchArgs {
                            scmd: r#"scoop config proxy 127.0.0.1:3128"#
                        }).unwrap()).await;
                    }
                    let new_msg = invoke("pscmd", args).await.as_string().unwrap();
                    invoke("pscmd", to_value(&SearchArgs {
                        scmd: r#"scoop config rm proxy"#
                    }).unwrap()).await;
                    result_msg.set(new_msg);
                });

                || {}
            },
            name2,
        )
    }

    let search = {
        let name = name.clone();
        let search_input_ref = search_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            name.set(
                search_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };
    let install = {
        let iname = iname.clone();
        let install_input_ref = install_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            iname.set(
                install_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };
    let bucket = {
        let bname = bname.clone();
        let bucket_input_ref = bucket_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            bname.set(
                bucket_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };
    let update = {
        let uname = uname.clone();
        let update_input_ref = update_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            uname.set(
                update_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Settings));

    html! {
        <main class="container">

            <form class="row" onsubmit={search}>
                <input id="search-input" ref={search_input_ref} placeholder="Enter a package name..." />
                <button type="submit">{"搜索"}</button>
            </form>
            <form class="row" onsubmit={install}>
                <input id="install-input" ref={install_input_ref} placeholder="Enter a package name..." />
                <button type="submit">{"安装"}</button>
            </form>
            <form class="row" onsubmit={update}>
                <input id="update-input" ref={update_input_ref} placeholder="Enter a package name..." />
                <button type="submit">{"更新"}</button>
            </form>
            <form class="row" onsubmit={bucket}>
                <input id="bucket-input" ref={bucket_input_ref} placeholder="Enter a bucket name..." />
                <button type="submit">{"添加"}</button>
            </form>

            <pre><code>{ &*result_msg }</code></pre>

            <button {onclick}>{ "设置" }</button>
        </main>
    }
}