import { useEffect, useMemo, useRef, useState } from "react";
import MenuPopUp, {
    type MenuOption,
    type SelectionData,
} from "../components/MenuPopUp";
import window_commands from "../lib/window_commands";
import { Outlet } from "react-router-dom";

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

const ContextMenu = () => {
    if (location.href === "http://localhost:5173/context-menu/submenu") {
        return <Outlet />;
    }

    const [current_menu, set_current_menu] = useState<(MenuOption | string)[]>(
        [],
    );
    // const [selection_data, set_selection_data] = useState<SelectionData | null>(
    //     null,
    // );
    const selection_data_ref = useRef<SelectionData | null>(null);
    // const [do_not_hide_on_blur, set_do_not_hide_on_blur] = useState(false);
    const do_not_hide_on_blur_ref = useRef(false);
    const is_menu_created_ref = useRef(false);

    const get_selection_data = (
        e: React.MouseEvent<HTMLDivElement, MouseEvent>,
    ): SelectionData | null => {
        const element = e.currentTarget;
        const rect = element.getBoundingClientRect();

        if (selection_data_ref.current) {
            const position: SelectionData = {
                ...selection_data_ref.current,

                client_x: rect.x + (rect.width - 10),
                client_y: rect.y,

                // client_x: 0,
                // client_y: 0,
                // inner_height: 0,
                // inner_width: 0,
                // screen_x: 0,
                // screen_y: 0,

                // client_x: window.screenX + rect.x + rect.width,
                // client_y: window.screenY + rect.y,
            };

            return position;
        }
        return null;
    };

    const update_menu = async () => {
        await window_commands.send_msg_to_window_by_selector("submenu", {
            type: "menu",
            value: "new_content_options",
        });
    };

    const set_menu_visibility = async (visibility: boolean) => {
        return await window_commands.send_msg_to_window_by_selector("submenu", {
            type: "visibility",
            value: visibility,
        });
    };

    const send_selection_data = async (selection_data: SelectionData) => {
        await window_commands.send_msg_to_window_by_selector("submenu", {
            type: "selection",
            value: selection_data,
        });
    };

    const create_submenu = async () => {
        do_not_hide_on_blur_ref.current = true;
        if (!is_menu_created_ref.current) {
            await window_commands.create_window({
                url: "http://localhost:5173/context-menu/submenu",
                window_name: "Context SubMenu",
                decoration: false,
                resizable: false,
                parent_window_key: localStorage.getItem("window_key") || "",
                size: {
                    height: 300,
                    width: 250,
                },
                selector: "submenu",
                shadow: false,
                transparent: true,
                visibility: false,
                kind: "Tool",
            });
            is_menu_created_ref.current = true;
        }
    };

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

            {
                name: menu_option_names.NEW,
                label: "New",
                type: "subfield",
                leave_func: async () => {
                    await set_menu_visibility(false);
                },
                func: async (
                    e: React.MouseEvent<HTMLDivElement, MouseEvent>,
                ) => {
                    try {
                        console.log("FIRED!!!");

                        const selection_data = get_selection_data(e);
                        if (!selection_data) {
                            console.log("No Selection Data!!!");
                            return;
                        }
                        do_not_hide_on_blur_ref.current = true;

                        await update_menu();
                        await send_selection_data(selection_data);
                        await set_menu_visibility(true);
                    } catch (error) {
                        console.log("Error creating context submenu: ", error);
                    }
                },
            },

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
        const on_load = async () => {
            if (is_menu_created_ref.current) return;
            // window_commands
            //     .send_msg_to_window_by_selector("submenu", {
            //         type: "close",
            //         value: true,
            //     })
            //     .then(() =>
            create_submenu().then(() => {
                console.log("Submenu created....");
            });
            // )
            // .catch((e) => {
            //     console.log("Error closing submenu ", e);
            // });
        };

        on_load();
        // const on_focus = () => {
        //     console.log("Focused");
        // };

        // window.addEventListener("load", on_load);
        // window.addEventListener("focus", on_focus);

        // return () => {
        //     window.removeEventListener("load", on_load);
        //     window.removeEventListener("focus", on_focus);
        // };
    }, []);
    return (
        <MenuPopUp
            //onContextMenu={(e) => e.preventDefault()}
            //  do_not_hide_on_blur={do_not_hide_on_blur_ref.current}
            do_not_hide_on_blur_ref={do_not_hide_on_blur_ref}
            //  do_not_hide_on_blur
            parent_selector="main"
            menu_options={current_menu}
            window_ipc_com={(e) => {
                let data = e.detail.data;
                switch (data.type) {
                    case "menu":
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
                    case "selection":
                        // set_selection_data(data.value);
                        selection_data_ref.current = data.value;
                        break;
                }
            }}
            tao_window_event={(e) => {
                switch (e.detail.type) {
                    case "focused":
                        // if (
                        //     e.detail.value === false
                        //     //&& !do_not_hide_on_blur_ref.current
                        // ) {
                        //     window_commands.set_visibility(false);
                        // } else {
                        //     //   window_commands.set_focus();
                        // }
                        // do_not_hide_on_blur_ref.current = false;

                        //   console.log("Tao data", e.detail);
                        // set_do_not_hide_on_blur(true);

                        break;
                }
            }}
            window_blur={async () => {
                if (!do_not_hide_on_blur_ref.current)
                    set_menu_visibility(false);
                else {
                    //Make blur listen again
                    window_commands.set_focus();
                    do_not_hide_on_blur_ref.current = false;
                }
                console.log("HIDE on blur!!!");
            }}
        />
    );
};

export default ContextMenu;
