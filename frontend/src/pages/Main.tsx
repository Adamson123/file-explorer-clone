import { useEffect, useRef, useState } from "react";
import LeftSection from "../components/LeftSection";
import RightSection from "../components/RightSection";
import TitleBar from "../components/TitleBar";
import invoke_command from "../lib/invoke_command";
import window_commands from "../lib/window_commands";
import type { SelectionData } from "./ContextMenu";

export type PathData = {
    shortcut: string;
    path: string;
};

export type DirContents = {
    name: string;
    size: number;
    is_dir: boolean;
    path: string;
};

const Main = () => {
    const [current_path_data, set_current_path_data] = useState<PathData>({
        shortcut: "C:\\",
        path: "C:\\Users\\Admin\\dev\\pc-usage\\tauri_app",
    });
    const [active_shortcut, set_active_shortcut] = useState("C:\\");
    const [nav_history, set_nav_history] = useState<PathData[]>([
        current_path_data,
    ]);
    const [active_history_index, set_active_history_index] = useState(0);
    const [dir_contents, set_dir_contents] = useState<DirContents[]>([]);
    const [is_dir_updated, set_is_dir_updated] = useState(false);
    const [is_maximized, set_is_maximized] = useState(false);

    const is_context_menu_created_ref = useRef(false);
    const last_context_element_ref = useRef<HTMLElement>(null);
    //   const is_new_context_content_ref = useRef(false);

    const update_nav_history = async (
        path: string,
        active_short: string | null = active_shortcut,
        is_shortcut = false,
    ) => {
        if (!active_short) active_short = active_shortcut;

        const new_nav_data: PathData = {
            path,
            shortcut: active_short,
        };

        try {
            //If the path is the same as the last path in the history, we don't need to update the history
            if (
                path === nav_history[nav_history.length - 1].path &&
                active_short === nav_history[nav_history.length - 1].shortcut
            ) {
                await get_dir_contents_cmd(path);
                return;
            }

            //If we are at the end of the history, we can just add the new path to the history
            if (active_history_index === nav_history.length - 1) {
                await get_dir_contents_cmd(path);
                set_nav_history((nvh) => [...nvh, new_nav_data]);
                set_active_history_index(nav_history.length);
                console.log("New!!!");
            } else {
                const existing = nav_history.findIndex((n) => n.path === path);

                //If the path is not in the history and it's not a shortcut, we can add it to the history
                if (existing === -1 && !is_shortcut) {
                    console.log(
                        "Not Existing!!!",
                        active_history_index + 1,
                        nav_history.length,
                    );
                    await get_dir_contents_cmd(path);

                    set_nav_history((nvh) => {
                        const n = nvh.filter(
                            (n, i) =>
                                !(
                                    i > active_history_index &&
                                    n.shortcut === active_shortcut
                                ),
                        );
                        n.push(new_nav_data);
                        set_active_history_index(n.length - 1);
                        return n;
                    });
                } else {
                    console.log("Existing!!! 1");

                    await get_dir_contents_cmd(path);

                    set_nav_history((nvh) => [...nvh, new_nav_data]);
                    set_active_history_index(nav_history.length);
                }
            }
        } catch (error) {}
    };

    const clear_context_content = () => {
        if (last_context_element_ref.current) {
            last_context_element_ref.current.classList.remove("bg-gray-400/10");
            last_context_element_ref.current = null;
        }
    };

    const get_selection_data = (
        e: React.MouseEvent<HTMLDivElement, MouseEvent>,
    ): SelectionData => {
        clear_context_content();

        const dir_element = (e.target as any).closest(
            ".dir_content",
        ) as HTMLDivElement | null;

        // //..
        // const extra = 0;
        // const menu_width = 250;
        // const max_x = screen.width + extra;
        // //const max_y = window.innerHeight + 20;

        // const cap_x = (x: number) => {
        //     const menu_right = e.screenX + menu_width;
        //     return menu_right > max_x ? x - (menu_right - max_x) : x;
        // };

        // const position = {
        //     clientX: cap_x(Math.min(e.clientX, window.innerWidth - 10)),
        //     clientY: e.clientY,
        // };

        //..

        const position: SelectionData = {
            client_x: e.clientX,
            client_y: e.clientY,
            inner_width: window.innerWidth,
            inner_height: window.innerHeight,
            screen_x: e.screenX,
            screen_y: e.screenY,
            dir_content: null,
        };

        if (!dir_element) {
            return position;
        }

        // const bounding_rect = dir_element.getBoundingClientRect();
        // const r = 0.45;
        // const new_x = bounding_rect.x + (bounding_rect.width * r) / 2;
        // const new_y = bounding_rect.y + (bounding_rect.height * r) / 2;

        last_context_element_ref.current = dir_element;
        last_context_element_ref.current.classList.add("bg-gray-400/10");

        //TODO: Get dir content and add
        return position;
    };

    const update_menu = async (
        dir_content: HTMLElement | null = last_context_element_ref.current,
    ) => {
        if (!dir_content) {
            await window_commands.send_msg_to_window_by_selector(
                "context_menu",
                {
                    type: "menu",
                    value: "current_dir_menu_options",
                },
            );
        } else {
            //Handle others
            const is_dir = dir_content.getAttribute("data-is-dir") === "true";
            if (is_dir) {
                await window_commands.send_msg_to_window_by_selector(
                    "context_menu",
                    {
                        type: "menu",
                        value: "dir_menu_options",
                    },
                );
            } else {
                await window_commands.send_msg_to_window_by_selector(
                    "context_menu",
                    {
                        type: "menu",
                        value: "file_menu_options",
                    },
                );
            }
        }
    };

    const set_menu_visibility = async (visibility: boolean) => {
        return await window_commands.send_msg_to_window_by_selector(
            "context_menu",
            {
                type: "visibility",
                value: visibility,
            },
        );
    };

    const update_menu_position = async (clientX: number, clientY: number) => {
        await window_commands.send_msg_to_window_by_selector("context_menu", {
            type: "position",
            value: {
                x: clientX,
                y: clientY,
            },
        });
    };

    const send_selection_data = async (selection_data: SelectionData) => {
        await window_commands.send_msg_to_window_by_selector("context_menu", {
            type: "selection",
            value: selection_data,
        });
    };

    const get_dir_contents_cmd = async (path: string) => {
        try {
            set_is_dir_updated(false);
            const res = await invoke_command("get_dir_contents", {
                path,
            });

            set_is_dir_updated(true);

            if (res) set_dir_contents(res);
            else set_dir_contents([]);
        } catch (error) {
            set_is_dir_updated(true);
            console.log("Error reading dir:", { error });
            throw error;
        }
    };

    useEffect(() => {
        set_current_path_data(nav_history[active_history_index]);
        set_active_shortcut(nav_history[active_history_index].shortcut);
    }, [active_history_index]);

    useEffect(() => {
        //For initial load, we can get the contents of the current path
        get_dir_contents_cmd(current_path_data.path);

        const tao_window_event: any = async (e: CustomEvent) => {
            switch (e.detail.type) {
                case "focused":
                    if (e.detail.value === false) {
                        await window_commands.send_msg_to_window_by_selector(
                            "context_menu",
                            {
                                type: "is_parent_blur",
                                value: false,
                            },
                        );
                    } else {
                        await window_commands.send_msg_to_window_by_selector(
                            "context_menu",
                            {
                                type: "is_parent_blur",
                                value: true,
                            },
                        );
                    }
                    break;
            }
        };

        const window_ipc_com: any = async (e: CustomEvent) => {
            let data = e.detail.data;

            switch (data.type) {
                case "is_blur":
                    if (data.value === true) {
                        clear_context_content();
                    }
                    break;
                // case "is_loaded":
                //     await update_menu()
                //     await  set_menu_visibility(true);
                //     await update_menu_position();

                //     break;
            }
        };

        //Send kill request to menu
        const on_load = async () => {
            await window_commands.send_msg_to_window_by_selector(
                "context_menu",
                {
                    type: "close",
                    value: true,
                },
            );
        };

        document.addEventListener("tao_window_event", tao_window_event);
        document.addEventListener("window_ipc_com", window_ipc_com);
        window.addEventListener("load", on_load);

        return () => {
            document.removeEventListener("tao_window_event", tao_window_event);
            document.removeEventListener("window_ipc_com", window_ipc_com);
            window.removeEventListener("load", on_load);
        };
    }, []);

    return (
        <main className="w-screen h-screen">
            <div
                className={`flex flex-col size-full overflow-hidden shadow-2xl ${!is_maximized && "rounded-xl"}`}
            >
                <TitleBar
                    active_history_index={active_history_index}
                    set_active_history_index={set_active_history_index}
                    nav_history={nav_history}
                    get_dir_contents_cmd={get_dir_contents_cmd}
                    set_is_maximized={set_is_maximized}
                    is_maximized={is_maximized}
                />
                <div
                    onContextMenu={async (e) => {
                        e.preventDefault();

                        try {
                            const selection_data = get_selection_data(e);

                            if (!is_context_menu_created_ref.current) {
                                await window_commands.create_window({
                                    url: "http://localhost:5173/context-menu",
                                    window_name: "Context Menu",
                                    decoration: false,
                                    resizable: false,
                                    parent_window_key:
                                        localStorage.getItem("window_key") ||
                                        "",
                                    size: {
                                        height: 300,
                                        width: 250,
                                    },
                                    // position: {
                                    //     x: selection_data.screen_x,
                                    //     y: selection_data.screen_y,
                                    // },
                                    selector: "context_menu",
                                    shadow: false,
                                    transparent: true,
                                    visibility: false,
                                    kind: "Tool",
                                });
                                is_context_menu_created_ref.current = true;
                                const menu_onload: any = async (
                                    e: CustomEvent,
                                ) => {
                                    // let data = e.detail.data;
                                    if ((e.detail.data.type = "is_loaded")) {
                                        await update_menu();
                                        // await update_menu_position(
                                        //     selection_data.client_x,
                                        //     selection_data.client_y,
                                        // );
                                        await send_selection_data(
                                            selection_data,
                                        );
                                        await set_menu_visibility(true);
                                        document.removeEventListener(
                                            "window_ipc_com",
                                            menu_onload,
                                        );
                                    }
                                };
                                document.addEventListener(
                                    "window_ipc_com",
                                    menu_onload,
                                );
                            } else {
                                await update_menu();
                                // await update_menu_position(
                                //     selection_data.client_x,
                                //     selection_data.client_y,
                                // );
                                await send_selection_data(selection_data);
                                set_menu_visibility(true);
                            }
                        } catch (error) {
                            console.log("Error creating context menu:", error);
                        }
                    }}
                    className="w-full h-full flex"
                >
                    <LeftSection
                        //  current_path_data={current_path_data}
                        set_current_path_data={set_current_path_data}
                        active_shortcut={active_shortcut}
                        set_active_shortcut={set_active_shortcut}
                        update_nav_history={update_nav_history}
                    />
                    <RightSection
                        current_path_data={current_path_data}
                        set_current_path_data={set_current_path_data}
                        update_nav_history={update_nav_history}
                        dir_contents={dir_contents}
                        is_dir_updated={is_dir_updated}
                        set_is_dir_updated={set_is_dir_updated}
                        set_dir_contents={set_dir_contents}
                    />
                </div>
            </div>
        </main>
    );
};

export default Main;
