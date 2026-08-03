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
import invoke_command from "../lib/invoke_command";
import type { ReactSetStateAction } from "../type";
import type { PathData } from "../App";
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
            //onMouseDown={() => {}}
            className="bg-primary py-3 px-4 font-semibold flex items-center justify-between"
        >
            {/* Right */}
            <div className="flex gap-12 items-center">
                <div className="flex gap-4">
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
            {/* TODO: Wrap each of them with buttons */}
            <div className="flex gap-6 items-center">
                {/* <button
                    move-window="false"
                    onClick={() => invoke_command("log_window_key")}
                >
                    Log
                </button> */}
                <Minus
                    move-window="false"
                    onClick={() => window_commands.minimize_window()}
                    className="text-gray-400 size-4"
                />
                {!is_maximized ? (
                    <Square
                        move-window="false"
                        onClick={async () => {
                            await window_commands.maximize_window();
                            set_is_maximized(true);
                        }}
                        className="text-gray-400 size-3"
                    />
                ) : (
                    <SquaresUnite
                        move-window="false"
                        onClick={async () => {
                            await window_commands.restore_window();
                            set_is_maximized(false);
                        }}
                        className="text-gray-400 size-3"
                    />
                )}
                <X
                    move-window="false"
                    onClick={() => {
                        window_commands.close_window();
                    }}
                    className="text-gray-400 size-4"
                />
            </div>
        </section>
    );
};

export default TitleBar;
