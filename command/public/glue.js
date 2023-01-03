const invoke = window.__TAURI__.invoke

export async function invokeGen(features, payload, sendTo) {
  return await invoke(
      "gen", {features : features, payload : payload, sendTo : sendTo});
}

export async function isOn(
    id) { return await document.getElementById(id).checked;}

export async function setPayload() {
  document.getElementById("payload").value =
      await window.__TAURI__.dialog.open();
}

export async function
getPayload() { return document.getElementById("payload").value;}

export async function getIp() {
  return document.getElementById("ip").value + ":" +
         document.getElementById("port").value;
}
