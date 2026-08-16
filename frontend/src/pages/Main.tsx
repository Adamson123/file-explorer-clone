import { useEffect, useRef, useState } from "react";
import LeftSection from "../components/LeftSection";
import RightSection from "../components/RightSection";
import TitleBar from "../components/TitleBar";
import invoke_command from "../lib/invoke_command";
import window_commands from "../lib/window_commands";

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
    const is_context_menu_created_ref = useRef(false);
    const last_context_element_ref = useRef<HTMLElement>(null);
    const is_new_context_content_ref = useRef(false);
    //const is_context_menu_open_ref = useRef(false);

    useEffect(() => {
        set_current_path_data(nav_history[active_history_index]);
        set_active_shortcut(nav_history[active_history_index].shortcut);
    }, [active_history_index]);

    const get_dir_contents_cmd = async (path: string) => {
        try {
            set_is_dir_updated(false);
            const res = await invoke_command("get_dir_contents", {
                path,
            });

            set_is_dir_updated(true);
            //   console.log("Contents: ", res);
            if (res) set_dir_contents(res);
            else set_dir_contents([]);
        } catch (error) {
            set_is_dir_updated(true);
            console.log("Error reading dir:", { error });
            throw error;
        }
    };

    const hide_menu = async () => {
        return await window_commands.send_msg_to_window_by_selector(
            "context_menu",
            {
                type: "visibility",
                value: false,
            },
        );
        // .then(() => {
        //     is_context_menu_open_ref.current = false;
        // });
    };

    useEffect(() => {
        //For initial load, we can get the contents of the current path
        get_dir_contents_cmd(current_path_data.path);

        const tao_window_event_handler: any = async (e: CustomEvent) => {
            switch (e.detail.type) {
                case "focused":
                    if (e.detail.value === false) {
                        // console.log(
                        //     "Should hide , ",
                        //     is_context_menu_open_ref.current,
                        //     e.detail.value,
                        // );
                        // if (is_context_menu_open_ref.current) await hide_menu();
                        await window_commands.send_msg_to_window_by_selector(
                            "context_menu",
                            {
                                type: "is_parent_blur",
                                value: false,
                            },
                        );
                        //    is_context_menu_open_ref.current = false;
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

        const window_ipc_com: any = (e: CustomEvent) => {
            let data = e.detail.data;
            console.log(e.detail);
            switch (data.type) {
                case "is_blur":
                    if (data.value === true) {
                        if (
                            last_context_element_ref.current //&&
                            // !is_new_context_content_ref.current
                        ) {
                            last_context_element_ref.current.classList.remove(
                                "bg-gray-400/10",
                            );
                            console.log(
                                "context content cleared -> blur",
                                last_context_element_ref.current,
                            );
                            last_context_element_ref.current = null;
                        } else {
                            console.log("Content context is protected");
                        }
                    }

                    is_new_context_content_ref.current = false;
                    break;
            }
        };

        document.addEventListener("window_event", tao_window_event_handler);
        document.addEventListener("window_ipc_com", window_ipc_com);

        return () => {
            document.removeEventListener(
                "window_event",
                tao_window_event_handler,
            );
            document.removeEventListener("window_ipc_com", window_ipc_com);
        };
    }, []);

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

    const get_menu_position = (
        e: React.MouseEvent<HTMLDivElement, MouseEvent>,
    ) => {
        if (last_context_element_ref.current) {
            last_context_element_ref.current.classList.remove("bg-gray-400/10");
            last_context_element_ref.current = null;
            console.log("context content cleared -> 1");
        }

        const dir_content = (e.target as any).closest(
            ".dir_content",
        ) as HTMLDivElement | null;

        if (!dir_content) {
            return { clientX: e.clientX, clientY: e.clientY };
        }
        const bounding_rect = dir_content.getBoundingClientRect();
        const r = 0.45;
        const new_x = bounding_rect.x + (bounding_rect.width * r) / 2;
        const new_y = bounding_rect.y + (bounding_rect.height * r) / 2;

        last_context_element_ref.current = dir_content;
        last_context_element_ref.current.classList.add("bg-gray-400/10");
        is_new_context_content_ref.current = true;
        console.log("context content created");

        return { clientX: new_x, clientY: new_y };
    };

    return (
        <main
            // onMouseDown={() =>
            //     last_context_element_ref.current?.classList.remove(
            //         "bg-gray-400/10",
            //     )
            // }
            className="w-screen h-screen"
        >
            <div className="flex flex-col size-full rounded-xl overflow-hidden shadow-2xl">
                <TitleBar
                    active_history_index={active_history_index}
                    set_active_history_index={set_active_history_index}
                    nav_history={nav_history}
                    get_dir_contents_cmd={get_dir_contents_cmd}
                />
                <div
                    onContextMenu={async (e) => {
                        e.preventDefault();
                        // console.log(e.target.closest(".dir_content"));
                        const { clientX, clientY } = get_menu_position(e);
                        if (!is_context_menu_created_ref.current) {
                            const res = await window_commands.create_window({
                                url: "http://localhost:5173/context-menu",
                                window_name: "Context Menu",
                                decoration: false,
                                resizable: false,
                                parent_window_key:
                                    localStorage.getItem("window_key") || "",
                                size: {
                                    height: 300,
                                    width: 250,
                                },
                                position: {
                                    x: clientX,
                                    y: clientY,
                                },
                                selector: "context_menu",
                                shadow: false,
                                transparent: true,
                            });
                            is_context_menu_created_ref.current = true;
                            //      is_context_menu_open_ref.current = true;
                        } else {
                            const res =
                                await window_commands.send_msg_to_window_by_selector(
                                    "context_menu",
                                    {
                                        type: "visibility",
                                        value: true,
                                    },
                                );

                            const res2 =
                                await window_commands.send_msg_to_window_by_selector(
                                    "context_menu",
                                    {
                                        type: "position",
                                        value: {
                                            x: clientX,
                                            y: clientY,
                                        },
                                    },
                                );

                            //   is_context_menu_open_ref.current = true;
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
