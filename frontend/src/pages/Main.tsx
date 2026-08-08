import { useEffect, useState } from "react";
import LeftSection from "../components/LeftSection";
import RightSection from "../components/RightSection";
import TitleBar from "../components/TitleBar";
import invoke_command from "../lib/invoke_command";

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
    //C:\\Users\\Admin\\dev\\pc-usage\\tauri_app
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

    useEffect(() => {
        console.log(nav_history);
        console.log(active_history_index);
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

    useEffect(() => {
        //For initial load, we can get the contents of the current path
        get_dir_contents_cmd(current_path_data.path);
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
                    //if the path is in history and
                    // if (is_shortcut) {
                    //     await get_dir_contents_cmd(path);

                    //     set_nav_history((nvh) => [...nvh, new_nav_data]);
                    //     set_active_history_index(nav_history.length);
                    // } else {
                    //     console.log("Existing!!! 2");

                    //     await get_dir_contents_cmd(path);

                    //     set_active_history_index(active_history_index + 1);
                    // }

                    await get_dir_contents_cmd(path);

                    set_nav_history((nvh) => [...nvh, new_nav_data]);
                    set_active_history_index(nav_history.length);
                }
            }
        } catch (error) {}
    };

    return (
        <main className="w-screen h-screen">
            {/* <div className="absolute inset-0 w-full h-full object-cover blur-2xl pointer-events-none" /> */}
            {/*
            rounded-lg overflow-hidden
            */}
            <div className="flex flex-col size-full rounded-xl overflow-hidden">
                <TitleBar
                    active_history_index={active_history_index}
                    set_active_history_index={set_active_history_index}
                    nav_history={nav_history}
                    get_dir_contents_cmd={get_dir_contents_cmd}
                />
                {/* grid grid-cols-[20%_80%] grow */}
                <div className="w-full h-full flex">
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
