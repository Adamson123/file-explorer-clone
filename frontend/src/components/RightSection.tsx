import { LayoutListIcon, LucideLayoutGrid, RefreshCcw } from "lucide-react";
import { useEffect, useState } from "react";
import invoke_command from "../lib/invoke_command";
import useStartTask from "../hooks/task/useStartTask";

type DirContents = {
    name: string;
    size: number;
    is_dir: boolean;
    path: string;
};

const RightSection = () => {
    const { events_handler, start_task } = useStartTask<{ path: string }>(
        "monitor_dir",
    );
    const [dir_contents, set_dir_contents] = useState<DirContents[]>([]);

    useEffect(() => {
        (async () => {
            // const dir_contents = await invoke_command("get_dir_contents", {
            //     path: "C:\\",
            // });
            // console.log(dir_contents);
            const res = await start_task({
                path: "C:\\Users\\Admin\\dev\\pc-usage\\core_logic\\src",
            });
            console.log(res);

            events_handler.add_error_listener((e) => {
                console.log("Error: ", e);
            }, "1");

            events_handler.add_message_listener((message) => {
                console.log("Message from task:", message);
            }, "1");
        })();

        setTimeout(() => {
            events_handler.send_msg({
                path: "C:\\Users\\Admin\\dev\\pc-usage\\core_logic",
            });
        }, 10000);
    }, []);
    return (
        <section className="bg-primary grow h-full pl-5 pr-2">
            {/* Head */}
            <div className="flex items-center gap-5 border-b border-gray-700/50">
                {/* Layout */}
                <div className="flex gap-2">
                    <LucideLayoutGrid className="text-gray-400 size-5" />
                    <LayoutListIcon className="text-gray-700 size-5" />
                </div>
                {/* Input */}
                <div
                    className="grow flex items-center  px-3
                 rounded-sm  outline-gray-700/50 focus-within:outline-gray-500/50"
                >
                    <img src="./assets/folder.svg" className="size-4.5" />
                    <input
                        type="text"
                        placeholder="Search"
                        className="bg-transparent text-sm text-gray-400 px-2 py-2 w-full outline-none"
                    />
                    <RefreshCcw className="text-gray-400 size-4.5" />
                </div>
                {/* Content
                <div className="mt-4">
                    <p className="text-gray-400 text-sm">No items to display</p>
                </div> */}
            </div>
            {/* flex-wrap flex gap-4 justify-baseline mt-7 */}
            <div className="grid grid-cols-[repeat(auto-fill,_minmax(95px,_1fr))] gap-4  mt-7 -translate-x-4">
                {[
                    "Applogs",
                    "Profile",
                    "Settings",
                    "Temp",
                    "Windows",
                    "Pictures",
                    "System32",
                    "System",
                    "Program Files",
                    "Program Files (x86)",
                    "Users",
                    "Documents",
                    "Downloads",
                ].map((folderName) => (
                    <div
                        key={folderName}
                        className="flex flex-col items-center"
                    >
                        <img src="./assets/folder.svg" className="size-18" />
                        <p className="text-gray-400 text-xs text-center">
                            {folderName}
                        </p>
                    </div>
                ))}
            </div>
        </section>
    );
};

export default RightSection;
