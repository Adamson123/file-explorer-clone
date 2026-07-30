import {
    File,
    LayoutListIcon,
    LucideLayoutGrid,
    RefreshCcw,
} from "lucide-react";
import { useEffect, useState } from "react";
import useStartTask from "../hooks/task/useStartTask";
import invoke_command from "../lib/invoke_command";
import type { ReactSetStateAction } from "../type";

type DirContents = {
    name: string;
    size: number;
    is_dir: boolean;
    path: string;
};

const RightSection = ({
    current_dir,
    set_current_dir,
}: {
    current_dir: string;
    set_current_dir: ReactSetStateAction<string>;
}) => {
    const { events_handler, start_task, is_started } = useStartTask<{
        path: string;
    }>("monitor_dir");
    const [dir_contents, set_dir_contents] = useState<DirContents[]>([]);
    const [is_dir_updated, set_is_dir_updated] = useState(false);

    useEffect(() => {
        (async () => {
            // const res = await start_task({
            //     //  path: "C:\\Users\\Admin\\dev\\pc-usage",
            //     path: current_dir,
            // });
            // console.log(res);

            events_handler.add_error_listener((e) => {
                console.log("Error: ", e);
            }, "1");
        })();

        // setTimeout(() => {
        //     set_is_dir_updated(false);
        //     events_handler.send_msg({
        //         path: "C:\\Users\\Admin\\dev\\pc-usage\\core_logic",
        //     });
        // }, 10000);
    }, []);

    useEffect(() => {
        set_is_dir_updated(false);
        invoke_command("get_dir_contents", { path: current_dir })
            .then((res) => {
                set_is_dir_updated(true);
                console.log("Contents: ", res);

                if (res) set_dir_contents(res);
                else set_dir_contents([]);
            })
            .catch((e) => {
                set_is_dir_updated(true);
                console.log("Error reading dir:", { e });
            });

        events_handler.add_message_listener((message) => {
            //  console.log("Message from task:", message);
            // console.log(message);
            // console.log("Path: ", message.contents);

            if (message.path !== current_dir) return;
            set_is_dir_updated(true);
            set_dir_contents(message.contents);
        }, "1");

        events_handler.send_msg({ path: current_dir });
    }, [current_dir]);
    return (
        <section className="bg-primary h-full w-full flex flex-col">
            {/* Head */}
            <div className=" pl-5 ">
                <div className="flex items-center  gap-5 border-b border-gray-700/50">
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
                            value={current_dir}
                            type="text"
                            placeholder="Search"
                            className="bg-transparent text-[13px] text-gray-400 px-2 py-2 w-full outline-none"
                        />
                        <RefreshCcw className="text-gray-400 size-4.5" />
                    </div>
                    {/* Content
                <div className="mt-4">
                    <p className="text-gray-400 text-sm">No items to display</p>
                </div> */}
                </div>
            </div>
            {/* flex-wrap flex gap-4 justify-baseline mt-7 */}
            {/* auto-fill */}
            {dir_contents.length ? (
                <div
                    style={{
                        //var(--color-transparent-black)
                        //rgba(8,8,8,0.50)
                        scrollbarWidth: "thin",
                        scrollbarColor:
                            "var(--color-primary) color-mix(in oklab, var(--color-gray-400) 10%, transparent)",
                    }}
                    className="grid grid-cols-[repeat(auto-fill,minmax(95px,1fr))] content-start items-start  gap-y-3 mt-3  overflow-y-auto grow  pb-25 px-2"
                >
                    {dir_contents.map((content) => (
                        <div
                            onDoubleClick={() =>
                                content.is_dir && set_current_dir(content.path)
                            }
                            key={content.path}
                            tabIndex={1}
                            className="flex flex-col items-center cursor-pointer hover:bg-gray-400/10 rounded-lg transition-colors duration-100 py-3 focus:bg-gray-400/10"
                        >
                            {content.is_dir ? (
                                <img
                                    src="./assets/folder.svg"
                                    className="size-18"
                                />
                            ) : (
                                <File className="size-17 stroke-white fill-white mb-1" />
                            )}
                            <p
                                title={content.name}
                                className="text-gray-400 text-xs text-center line-clamp-2  break-all px-1"
                            >
                                {content.name}
                            </p>
                        </div>
                    ))}
                </div>
            ) : (
                <div className="mt-4">
                    {/*
                    is_started && is_dir_updated
                    */}
                    <p className="text-gray-400 text-sm text-center">
                        {is_dir_updated ? "No items to display" : "Loading..."}
                    </p>
                </div>
            )}
        </section>
    );
};

export default RightSection;
