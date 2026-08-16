import {
    ArrowLeft,
    ArrowRight,
    Minus,
    Square,
    SquaresUnite,
    X,
} from "lucide-react";
import window_commands from "../lib/window_commands";
import { useState } from "react";
import type { ReactSetStateAction } from "../type";
import invoke_command from "../lib/invoke_command";
import type { PathData } from "../pages/Main";
//import invoke_command from "../lib/invoke_command";

//If there's an issue with clicking a child inside a parent with move-window="true", we can add move-window="false" to the child element to solve it. This is useful for buttons inside a title bar, for example.
const TitleBar = ({
    active_history_index,
    nav_history,
    set_active_history_index,
    get_dir_contents_cmd,
}: {
    active_history_index: number;
    set_active_history_index: ReactSetStateAction<number>;
    nav_history: PathData[];
    get_dir_contents_cmd: (path: string) => Promise<void>;
}) => {
    const [is_maximized, set_is_maximized] = useState(false);

    const is_first = active_history_index === 0;
    const is_last = active_history_index === nav_history.length - 1;

    let folder_name =
        nav_history[active_history_index].shortcut.split("\\").at(-1) ||
        nav_history[active_history_index].shortcut;
    if (folder_name === "C:\\") folder_name = "Local Disk (C:)";

    return (
        <section
            move-window="true"
            className="bg-primary font-semibold flex items-center justify-between"
        >
            {/* Right */}
            <div className="flex gap-12 items-center">
                <div className="flex">
                    <button
                        move-window="false"
                        disabled={is_first}
                        onClick={async () => {
                            if (!is_first) {
                                await get_dir_contents_cmd(
                                    nav_history[active_history_index - 1].path,
                                );
                                set_active_history_index(
                                    active_history_index - 1,
                                );
                            }
                        }}
                        className="size-12 items-center flex justify-center cursor-pointer hover:bg-gray-400/10"
                    >
                        <ArrowLeft
                            className={`size-5 ${is_first ? "text-gray-700/60" : "text-gray-400"}`}
                        />
                    </button>
                    <button
                        move-window="false"
                        disabled={is_last}
                        onClick={async () => {
                            if (!is_last) {
                                await get_dir_contents_cmd(
                                    nav_history[active_history_index + 1].path,
                                );
                                set_active_history_index(
                                    active_history_index + 1,
                                );
                            }
                        }}
                        className="size-12 items-center flex justify-center cursor-pointer hover:bg-gray-400/10"
                    >
                        <ArrowRight
                            className={`size-5 ${is_last ? "text-gray-700/60" : "text-gray-400"}`}
                        />
                    </button>
                </div>
                <h1>
                    {/* Local Disc (C:) */}
                    {folder_name}
                </h1>
            </div>
            {/* Left */}

            <div className="flex items-center">
                {/* <button
                    className="cursor-pointer"
                    move-window="false"
                    // onClick={async () => {
                    //     const res = await invoke_command("create_file", {
                    //         path:
                    //             nav_history[active_history_index].path +
                    //             "\\log_text_click.txt",
                    //     });
                    //     console.log(res);
                    // }}
                    onClick={async (e) => {
                        const res = await window_commands.create_window({
                            url: "http://localhost:5173/context-menu",
                            window_name: "Context Menu",
                            decoration: false,
                            height: 300,
                            width: 200,
                            resizable: false,
                            parent_window_key:
                                localStorage.getItem("window_key") || "",
                            position: {
                                x: e.clientX,
                                y: e.clientY,
                            },
                            selector: "context_menu",
                        });
                        console.log(res);
                    }}
                >
                    Log
                </button>

                <button
                    className="cursor-pointer"
                    move-window="false"
                    // onClick={async () => {
                    //     const res = await invoke_command("create_file", {
                    //         path:
                    //             nav_history[active_history_index].path +
                    //             "\\log_text_click.txt",
                    //     });
                    //     console.log(res);
                    // }}
                    onClick={async () => {
                        const res =
                            await window_commands.send_msg_to_window_by_selector(
                                "context_menu",
                                ["Hah_____Hah"],
                            );
                        console.log(res);
                    }}
                >
                    Send Msg
                </button> */}

                <button
                    move-window="false"
                    onClick={() => window_commands.minimize_window()}
                    className="size-12 items-center flex justify-center cursor-pointer hover:bg-gray-400/10"
                >
                    <Minus className="text-gray-400 size-4 " />
                </button>
                {!is_maximized ? (
                    <button
                        move-window="false"
                        onClick={async () => {
                            await window_commands.maximize_window();
                            set_is_maximized(true);
                        }}
                        className="size-12 items-center flex justify-center cursor-pointer hover:bg-gray-400/10"
                    >
                        <Square className="text-gray-400 size-3" />
                    </button>
                ) : (
                    <button
                        move-window="false"
                        onClick={async () => {
                            await window_commands.restore_window();
                            set_is_maximized(false);
                        }}
                        className="size-12 items-center flex justify-center cursor-pointer hover:bg-gray-400/10"
                    >
                        <SquaresUnite className="text-gray-400 size-3" />
                    </button>
                )}
                <button
                    move-window="false"
                    onClick={() => {
                        window_commands.close_window();
                    }}
                    className="size-12 items-center flex justify-center cursor-pointer hover:bg-gray-400/10"
                >
                    <X
                        move-window="false"
                        onClick={() => {
                            window_commands.close_window();
                        }}
                        className="text-gray-400 size-4"
                    />
                </button>
            </div>
        </section>
    );
};

export default TitleBar;
