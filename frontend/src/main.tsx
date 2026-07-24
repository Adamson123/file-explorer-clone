import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App.tsx";

window.addEventListener("load", () => {
    console.log("Window fully reloaded");
    (window as any).ipc.postMessage(
        JSON.stringify({
            msg_type: "task",
            action: "kill_all",
        }),
    );
});

createRoot(document.getElementById("root")!).render(
    <StrictMode>
        <App />
    </StrictMode>,
);
