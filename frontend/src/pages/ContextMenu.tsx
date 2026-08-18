import { useEffect, useMemo, useState } from "react";
import window_commands from "../lib/window_commands";

const menu_option_names = {
    COPY: "copy",
    CUT: "cut",
    DELETE: "delete",
    NEW: "new",
    OPEN_FOLDER: "open_folder",
    OPEN_FILE: "open_file",
    OPEN_IN_NEW_WINDOW: "open_in_new_window",
    PIN: "pin",
    PASTE: "paste",
    PROPERTIES: "properties",
    REFRESH: "refresh",
    RENAME: "rename",
};

const LINE = "line";

const current_dir_menu_options = [
    menu_option_names.PASTE,
    LINE,
    menu_option_names.REFRESH,
    LINE,
    menu_option_names.NEW,
    LINE,
    menu_option_names.PROPERTIES,
];

const dir_menu_options = [
    menu_option_names.OPEN_FOLDER,
    menu_option_names.OPEN_IN_NEW_WINDOW,
    menu_option_names.PIN,
    LINE,
    menu_option_names.CUT,
    menu_option_names.COPY,
    LINE,
    menu_option_names.DELETE,
    menu_option_names.RENAME,
    LINE,
    menu_option_names.PROPERTIES,
];

const file_menu_options = [
    menu_option_names.OPEN_FILE,
    LINE,
    menu_option_names.CUT,
    menu_option_names.COPY,
    LINE,
    menu_option_names.DELETE,
    menu_option_names.RENAME,
    LINE,
    menu_option_names.PROPERTIES,
];

const menu_options: { [key: string]: string[] } = {
    current_dir_menu_options,
    dir_menu_options,
    file_menu_options,
};

type MenuOption = {
    name: string;
    label: string;
    func: () => void;
};

// window.addEventListener("DOMContentLoaded", async () => {
//     await window_commands.send_msg_to_window_by_selector("main", {
//         type: "is_loaded",
//         value: true,
//     });
//     console.log("Sent DOM loaded");
// });

const ContextMenu = () => {
    const [current_menu, set_current_menu] = useState<(MenuOption | string)[]>(
        [],
    );

    const menu: MenuOption[] = useMemo(() => {
        return [
            {
                name: menu_option_names.CUT,
                label: "Cut",
                func: () => {},
            },
            {
                name: menu_option_names.COPY,
                label: "Copy",
                func: () => {},
            },

            {
                name: menu_option_names.DELETE,
                label: "Delete",
                func: () => {},
            },

            { name: menu_option_names.NEW, label: "New", func: () => {} },

            {
                name: menu_option_names.OPEN_FOLDER,
                label: "Open",
                func: () => {},
            },
            {
                name: menu_option_names.OPEN_FILE,
                label: "Open",
                func: () => {},
            },
            {
                name: menu_option_names.OPEN_IN_NEW_WINDOW,
                label: "Open in new window",
                func: () => {},
            },

            { name: menu_option_names.PIN, label: "Pin", func: () => {} },
            { name: menu_option_names.PASTE, label: "Paste", func: () => {} },
            {
                name: menu_option_names.PROPERTIES,
                label: "Properties",
                func: () => {},
            },

            {
                name: menu_option_names.REFRESH,
                label: "Refresh",
                func: () => {},
            },
            { name: menu_option_names.RENAME, label: "Rename", func: () => {} },
        ];
    }, []);

    useEffect(() => {
        //  alert("Run!!!");
        const window_ipc_com: any = (e: CustomEvent) => {
            let data = e.detail.data;
            console.log(data, "data");
            //  alert("ran");
            switch (data.type) {
                case "menu":
                    console.log(data, "menu");
                    const options = menu_options[data.value];
                    if (options?.length) {
                        set_current_menu(
                            options
                                .map((op) => {
                                    if (op === LINE) {
                                        return "";
                                    }
                                    return menu.find((o) => op === o.name);
                                })
                                .filter((v) => v !== undefined),
                        );
                    }
                    break;
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
                case "selection":
                    {
                        // use selected element data (should contain position data)
                    }
                    break;
                case "position":
                    window_commands.set_position(data.value);
                    break;
                case "close":
                    window_commands.close_window();
                    break;
                case "is_parent_blur":
                    // if (data.value === false && !document.hasFocus()) {
                    //     window_commands.set_visibility(false);
                    // }
                    break;
            }
        };
        const window_blur: any = async () => {
            await window_commands.set_visibility(false); //.then(() => {});
            await window_commands.send_msg_to_window_by_selector("main", {
                type: "is_blur",
                value: true,
            });
        };

        const tao_window_event_handler: any = async (e: CustomEvent) => {
            console.log(e.detail, "tao event");
            switch (e.detail.type) {
                case "focused":
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
                    const new_height = entry.target.clientHeight + 2;

                    window_commands.set_size({
                        width: 250,
                        height: new_height,
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
    }, [current_menu]);

    useEffect(() => {
        window_commands.send_msg_to_window_by_selector("main", {
            type: "is_loaded",
            value: true,
        });
    }, []);

    return (
        <main
            onContextMenu={(e) => e.preventDefault()}
            className="w-screen h-screen bg-transparent select-none overflow-hidden"
        >
            {/* <div className="w-full h-full bg-transparent overflow-hidden"> */}
            <div className="flex flex-col menu py-1 bg-primary rounded-md shadow-xl border border-gray-400/10">
                {current_menu.length ? (
                    current_menu.map((item, i) =>
                        typeof item === "string" ? (
                            <div key={i} className="p-1">
                                <hr className="bg-gray-400/10 h-px border-0" />
                            </div>
                        ) : (
                            <div
                                onClick={item.func}
                                className="py-1.5 px-2  cursor-pointer hover:bg-gray-400/10 transition-colors duration-100 text-xs"
                                key={i}
                            >
                                {item.label}
                            </div>
                        ),
                    )
                ) : (
                    <p className="text-center">No options</p>
                )}
            </div>
        </main>
    );
};

export default ContextMenu;
