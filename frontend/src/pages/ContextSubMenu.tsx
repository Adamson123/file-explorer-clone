import { useMemo, useState } from "react";
import MenuPopUp, { type MenuOption } from "../components/MenuPopUp";
import window_commands from "../lib/window_commands";

const menu_option_names = {
    FILE: "file",
    FOLDER: "folder",
    SHORTCUT: "shortcut",
    RAR: "rar",
};

const LINE = "line";

const new_content_options = [
    menu_option_names.FOLDER,
    menu_option_names.FILE,
    LINE,
    menu_option_names.RAR,
    LINE,
    menu_option_names.SHORTCUT,
];

const menu_options: { [key: string]: string[] } = {
    new_content_options,
};

// window.addEventListener("load", () => {
//     window_commands.send_msg_to_window_by_selector("context_menu", {
//         type: "visibility",
//         value: true,
//     });
// });

const ContextSubMenu = () => {
    const [current_menu, set_current_menu] = useState<(MenuOption | string)[]>(
        [],
    );

    const menu: MenuOption[] = useMemo(() => {
        return [
            {
                name: menu_option_names.FOLDER,
                label: "Folder",
                func: () => {},
            },
            {
                name: menu_option_names.FILE,
                label: "File",
                func: () => {},
            },

            {
                name: menu_option_names.RAR,
                label: "RAR",
                func: () => {},
            },
            {
                name: menu_option_names.SHORTCUT,
                label: "Shortcut",
                func: () => {},
            },
        ];
    }, []);

    //    useEffect(() => {
    //     const on_load = async () => {
    //         await window_commands.send_msg_to_window_by_selector("submenu", {
    //             type: "close",
    //             value: true,
    //         });
    //     };

    //     window.addEventListener("load", on_load);

    //     return () => {
    //         window.removeEventListener("load", on_load);
    //     };
    // }, []);

    return (
        <MenuPopUp
            do_not_hide_on_blur
            menu_options={current_menu}
            parent_selector="context_menu"
            tao_window_event={(e) => {
                switch (e.detail.type) {
                    case "focused":
                        // if (e.detail.value) {
                        //     window_commands.send_msg_to_window_by_selector(
                        //         "context_menu",
                        //         {
                        //             type: "visibility",
                        //             value: true,
                        //         },
                        //     );
                        // }

                        //   console.log("Tao data", e.detail);

                        break;
                }
            }}
            onMouseLeave={() => window_commands.set_visibility(false)}
            window_ipc_com={(e) => {
                let data = e.detail.data;
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
                }
            }}
        />
    );
};

export default ContextSubMenu;
