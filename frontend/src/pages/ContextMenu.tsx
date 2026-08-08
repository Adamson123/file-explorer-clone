import { useEffect } from "react";

const ContextMenu = () => {
    useEffect(() => {
        let window_ipc_com = (e: any) => {
            console.log(e.detail);
        };

        document.addEventListener("window_ipc_com", window_ipc_com);

        return () => {
            document.removeEventListener("window_ipc_com", window_ipc_com);
        };
    }, []);

    return (
        <main
            move-window="true"
            className="w-screen h-screen bg-red-500"
        ></main>
    );
};

export default ContextMenu;
