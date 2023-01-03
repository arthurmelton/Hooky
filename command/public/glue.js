const invoke = window.__TAURI__.invoke

export async function invokeGen(features, payload, sendTo) {
    document.getElementById("GenCover").style.display = "block";
    document.getElementById("setup").style.display = "none";
    await invoke("gen", {
        features: features,
        payload: payload,
        sendTo: sendTo
    });
    while (!(await is_done())) {
        await new Promise(r => setTimeout(r, 100));
    }
    document.getElementById("GenCover").style.display = "none";
    document.getElementById("dashboard").style.display = "block";
}

async function is_done() {
    return await invoke("is_done", {});
}

export async function isOn(
    id) {
    return await document.getElementById(id).checked;
}

export async function setPayload() {
    document.getElementById("payload").value =
        await window.__TAURI__.dialog.open();
}

export async function
getPayload() {
    return document.getElementById("payload").value;
}

export async function getIp() {
    return document.getElementById("ip").value + ":" +
        document.getElementById("port").value;
}
