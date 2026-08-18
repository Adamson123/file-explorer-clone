import { File, LayoutListIcon, LucideLayoutGrid } from "lucide-react";
import { useEffect } from "react";
import useStartTask from "../hooks/task/useStartTask";
import type { ReactSetStateAction } from "../type";
import type { DirContents, PathData } from "../pages/Main";

const RightSection = ({
    current_path_data,
    set_current_path_data,
    update_nav_history,
    set_is_dir_updated,
    set_dir_contents,
    dir_contents,
    is_dir_updated,
}: {
    current_path_data: PathData;
    set_current_path_data: ReactSetStateAction<PathData>;
    update_nav_history: (path: string, active_shortcut?: string) => void;
    set_is_dir_updated: ReactSetStateAction<boolean>;
    set_dir_contents: ReactSetStateAction<DirContents[]>;
    dir_contents: DirContents[];
    is_dir_updated: boolean;
}) => {
    const { events_handler, start_task } = useStartTask<{
        path: string;
    }>("monitor_dir");

    useEffect(() => {
        (async () => {
            // const res = await start_task({
            //     path: current_dir,
            // });
            // console.log(res);

            events_handler.add_error_listener((e) => {
                console.log("Error: ", e);
            }, "1");
        })();
    }, []);

    useEffect(() => {
        events_handler.add_message_listener((message) => {
            //  console.log(message, current_dir);
            if (message.path !== current_path_data.path) return;

            set_is_dir_updated(true);
            set_dir_contents(message.contents);
        }, "1");

        // events_handler.send_msg({ path: current_path_data.path });
        return () => {
            //To add new message listener with updated current_dir
            events_handler.remove_message_listener("1");
        };
    }, [current_path_data]);
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
                        className="grow flex items-center  pl-3
                 rounded-sm  outline-gray-700/50 focus-within:outline-gray-500/50 pr-4.5"
                    >
                        <img src="./assets/folder.svg" className="size-4.5" />
                        <input
                            onChange={() => {}}
                            value={current_path_data.path}
                            type="text"
                            placeholder="Search"
                            className="bg-transparent text-[13px] text-gray-400 px-2 py-2 w-full outline-none"
                        />
                        <img src="/assets/refresh.svg" className=" size-3.5" />
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
                    className="grid grid-cols-[repeat(auto-fill,minmax(95px,1fr))] content-start items-start mt-3  overflow-y-auto grow pb-25 px-2"
                >
                    {dir_contents.map((content) => (
                        <div
                            onDoubleClick={
                                () =>
                                    content.is_dir &&
                                    update_nav_history(content.path) //set_current_dir(content.path)
                            }
                            key={content.path}
                            data-is-dir={content.is_dir}
                            className="dir_content flex flex-col items-center cursor-pointer hover:bg-gray-400/10 transition-colors rounded-sm duration-100 py-2 h-full"
                        >
                            {content.is_dir ? (
                                <img
                                    src="./assets/folder.svg"
                                    className="size-18"
                                />
                            ) : (
                                <File className="size-17 stroke-white fill-white mb-1" />
                            )}
                            {/* line-clamp-2 truncate max-w-[95%] text-wrap */}
                            <p
                                title={content.name}
                                className="text-gray-400 text-xs text-center line-clamp-2 max-w-[95%] break-all"
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
