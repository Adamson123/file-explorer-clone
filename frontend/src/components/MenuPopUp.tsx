import { useEffect, useRef, type HTMLAttributes, type RefObject } from "react";
import window_commands from "../lib/window_commands";
import { ChevronRight } from "lucide-react";
import type { DirContents } from "../pages/Main";
//import type { ReactSetStateAction } from "../type";

export type MenuOption = {
    name: string;
    label: string;
    type?: string;
    func: (e: any) => void;
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

const MenuPopUp = (
    props: HTMLAttributes<HTMLDivElement> & {
        parent_selector: string;
        menu_options: (MenuOption | string)[];
        window_ipc_com: (e: CustomEvent) => void;
        window_blur?: (e: FocusEvent) => void;
        tao_window_event: (e: CustomEvent) => void;
        do_not_hide_on_blur?: boolean;
        do_not_hide_on_blur_ref?: RefObject<boolean>;
        //  set_should_hide_on_blur: ReactSetStateAction<boolean>;
    },
) => {
    const {
        tao_window_event,
        window_ipc_com: user_window_ipc_com,
        window_blur: user_window_blur = () => {},
        parent_selector,
        menu_options,
        do_not_hide_on_blur = false,
        do_not_hide_on_blur_ref = useRef(false),
        className,
        ...otherProps
    } = props;

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

        const window_blur: any = async (e: FocusEvent) => {
            if (!do_not_hide_on_blur && !do_not_hide_on_blur_ref.current) {
                await window_commands.set_visibility(false);
                await window_commands.send_msg_to_window_by_selector(
                    parent_selector,
                    {
                        type: "is_blur",
                        value: true,
                    },
                );
            }
            console.log("Shoudld hide? ", !do_not_hide_on_blur_ref.current);
            user_window_blur(e);
        };

        const window_ipc_com: any = (e: CustomEvent) => {
            let data = e.detail.data;
            switch (data.type) {
                case "visibility":
                    window_commands.set_visibility(data.value);
                    //TODO: Use !data.value
                    if (data.value === false)
                        window_commands.send_msg_to_window_by_selector("main", {
                            type: "is_blur",
                            value: true,
                        });
                    else {
                        window_commands.send_msg_to_window_by_selector("main", {
                            type: "is_blur",
                            value: false,
                        });
                    }
                    break;
                case "position":
                    window_commands.set_position(data.value);
                    break;
                case "close":
                    window_commands.close_window();
                    break;
                case "is_parent_blur":
                    break;
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
    }, [menu_options, do_not_hide_on_blur]);

    useEffect(() => {
        window_commands.send_msg_to_window_by_selector(parent_selector, {
            type: "is_loaded",
            value: true,
        });
    }, []);

    return (
        <main
            className={`w-screen h-screen bg-transparent select-none overflow-hidden ${className}`}
            {...otherProps}
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
                                //  onMouseLeave={item?.leave_func}
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

export default MenuPopUp;
