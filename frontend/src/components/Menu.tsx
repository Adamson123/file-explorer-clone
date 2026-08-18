import { useEffect } from "react";
import window_commands from "../lib/window_commands";
import { ChevronRight } from "lucide-react";
import type { DirContents } from "../pages/Main";

type MenuOption = {
    name: string;
    label: string;
    type?: string;
    func: () => void;
};

export type SelectionData = {
    dir_content: DirContents | null;
    client_x: number;
    client_y: number;
    screen_x: number;
    screen_y: number;
    inner_width: number;
    inner_height: number;
};

const Menu = ({
    tao_window_event,
    window_ipc_com: user_window_ipc_com,
    parent_selector,
    menu_options,
}: {
    parent_selector: string;
    menu_options: (MenuOption | string)[];
    window_ipc_com: (e: CustomEvent) => void;
    tao_window_event: (e: CustomEvent) => void;
}) => {
    const handle_selection = async (selection_data: SelectionData) => {
        if (!selection_data) return;

        const cap_x = (x: number) => {
            const max_x = screen.width;
            const menu_width =
                document.querySelector(".menu")?.clientWidth || 0; //250;

            const menu_right = selection_data.screen_x + menu_width;
            return menu_right > max_x ? x - (menu_right - max_x) : x;
        };

        const cap_y = (y: number) => {
            const taskbar_height = 42;
            const max_y = screen.height - taskbar_height;
            const menu_height =
                document.querySelector(".menu")?.clientHeight || 0;

            const menu_bottom = selection_data.screen_y + menu_height;
            return menu_bottom > max_y ? y - (menu_bottom - max_y) : y;
        };

        const extra = 10;
        const position = {
            x: cap_x(
                Math.min(
                    selection_data.client_x,
                    selection_data.inner_width - extra,
                ),
            ),
            y: cap_y(
                Math.min(
                    selection_data.client_y,
                    selection_data.inner_height - extra,
                ),
            ),
        };

        await window_commands.set_position(position);
    };

    useEffect(() => {
        const menu_height = document.querySelector(".menu")?.clientHeight || 0;
        const menu_width = document.querySelector(".menu")?.clientWidth || 0;
        window_commands.set_size({
            width: menu_width,
            height: menu_height + 16,
        });

        const resize_observer = new ResizeObserver((entries) => {
            for (let entry of entries) {
                if (entry.target === document.querySelector(".menu")) {
                    const new_height = entry.target.clientHeight + 2;

                    window_commands.set_size({
                        width: 250,
                        height: new_height,
                    });
                }
            }
        });

        const window_blur: any = async () => {
            await window_commands.set_visibility(false);
            await window_commands.send_msg_to_window_by_selector(
                parent_selector,
                {
                    type: "is_blur",
                    value: true,
                },
            );
        };

        const window_ipc_com: any = (e: CustomEvent) => {
            let data = e.detail.data;
            switch (data.type) {
                case "selection":
                    handle_selection(data.value);
                    break;
            }
            user_window_ipc_com(e);
        };

        resize_observer.observe(document.querySelector(".menu") as Element);
        document.addEventListener("window_ipc_com", window_ipc_com);
        document.addEventListener("tao_window_event", tao_window_event as any);
        window.addEventListener("blur", window_blur);

        return () => {
            resize_observer.disconnect();
            document.removeEventListener("window_ipc_com", window_ipc_com);
            document.removeEventListener(
                "tao_window_event",
                tao_window_event as any,
            );
            window.removeEventListener("blur", window_blur);
        };
    }, [menu_options]);

    useEffect(() => {
        window_commands.send_msg_to_window_by_selector(parent_selector, {
            type: "is_loaded",
            value: true,
        });
    }, []);

    return (
        <main
            onContextMenu={(e) => e.preventDefault()}
            className="w-screen h-screen bg-transparent select-none overflow-hidden"
        >
            {/* <div className="w-full h-full bg-transparent overflow-hidden"> */}
            <div className="flex flex-col menu py-1 bg-primary rounded-md shadow-xl border border-gray-400/10">
                {menu_options.length ? (
                    menu_options.map((item, i) =>
                        typeof item === "string" ? (
                            <div key={i} className="p-1">
                                <hr className="bg-gray-400/10 h-px border-0" />
                            </div>
                        ) : item?.type ? (
                            <div
                                onMouseEnter={item.func}
                                className="py-1 px-2  cursor-pointer hover:bg-gray-400/10 transition-colors duration-100 text-xs flex justify-between items-center"
                                key={i}
                            >
                                {item.label}{" "}
                                <ChevronRight className="size-4.5 text-gray-400" />
                            </div>
                        ) : (
                            <div
                                onClick={item.func}
                                className="py-1.5 px-2  cursor-pointer hover:bg-gray-400/10 transition-colors duration-100 text-xs"
                                key={i}
                            >
                                {item.label}
                            </div>
                        ),
                    )
                ) : (
                    <p className="text-center">No options</p>
                )}
            </div>
        </main>
    );
};

export default Menu;
