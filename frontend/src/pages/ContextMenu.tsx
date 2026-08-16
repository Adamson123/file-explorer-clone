import { useEffect } from "react";
import window_commands from "../lib/window_commands";

const ContextMenu = () => {
    useEffect(() => {
        const window_ipc_com: any = (e: CustomEvent) => {
            let data = e.detail.data;
            console.log(e.detail, "from context_menu.tsx");

            switch (data.type) {
                case "visibility":
                    window_commands.set_visibility(data.value);
                    if (data.value === false)
                        window_commands.send_msg_to_window_by_selector("main", {
                            type: "is_blur",
                            value: true,
                        });
                    else {
                        window_commands.send_msg_to_window_by_selector("main", {
                            type: "is_blur",
                            value: false,
                        });
                    }
                    break;
                case "position":
                    // window_commands.send_msg_to_window_by_selector("main", {
                    //     type: "is_blur",
                    //     value: true,
                    // });
                    window_commands.set_position(data.value);
                    break;
                case "is_parent_blur":
                    // if (data.value === false && !document.hasFocus()) {
                    //     window_commands.set_visibility(false);
                    // }
                    break;
            }
        };
        const window_blur: any = async () => {
            //await Promise.all([
            await window_commands.set_visibility(false); //.then(() => {});
            await window_commands.send_msg_to_window_by_selector("main", {
                type: "is_blur",
                value: true,
            });
            //]);
        };

        const tao_window_event_handler: any = async (e: CustomEvent) => {
            console.log(e.detail, "tao event");
            switch (e.detail.type) {
                case "focused":
                    // if (e.detail.value === false) {
                    //     window_commands.send_msg_to_window_by_selector("main", {
                    //         type: "is_blur",
                    //         value: true,
                    //     });
                    //     window_commands.set_visibility(false);
                    // } else {
                    // }
                    break;
            }
        };

        document.addEventListener("window_ipc_com", window_ipc_com);
        document.addEventListener("window_event", tao_window_event_handler);
        window.addEventListener("blur", window_blur);

        const menu_height = document.querySelector(".menu")?.clientHeight || 0;
        window_commands.set_size({ width: 250, height: menu_height + 16 });

        const resize_observer = new ResizeObserver((entries) => {
            for (let entry of entries) {
                if (entry.target === document.querySelector(".menu")) {
                    const new_height = entry.contentRect.height;
                    window_commands.set_size({
                        width: 250,
                        height: new_height + 16,
                    });
                }
            }
        });
        resize_observer.observe(document.querySelector(".menu") as Element);

        return () => {
            document.removeEventListener("window_ipc_com", window_ipc_com);
            document.removeEventListener(
                "window_event",
                tao_window_event_handler,
            );
            window.removeEventListener("blur", window_blur);
            resize_observer.disconnect();
        };
    }, []);

    return (
        <main
            onContextMenu={(e) => e.preventDefault()}
            className="w-screen h-screen p-2 bg-transparent select-none"
        >
            <div className="w-full h-full bg-primary rounded-md shadow-2xl border border-gray-400/10 overflow-hidden">
                <div className="flex flex-col menu">
                    {["Open", "Delete", "Rename", "Properties"].map(
                        (item, index) => (
                            <div
                                className="p-2 border-b border-gray-400/10 cursor-pointer hover:bg-gray-400/10 transition-colors duration-100 text-xs"
                                key={index}
                            >
                                {item}
                            </div>
                        ),
                    )}
                </div>
            </div>
        </main>
    );
};

export default ContextMenu;
