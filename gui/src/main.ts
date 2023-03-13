import { invoke } from "@tauri-apps/api/tauri";

import { Elm } from "./Main.elm";

import "tailwindcss/tailwind.css";

const app = Elm.Main.init({
  node: document.getElementById("root")!,
});

app.ports.greet.subscribe((yourName) => {
  void invoke("greet", { name: yourName }).then((message) => {
    if (typeof message !== "string") {
      throw new Error("Expected string: greet port");
    }

    app.ports.messageReceiver.send(message);
  });
});
