import invoke_command from "./invoke_command";

/*
pub struct WebViewWindowConfig {
    pub window_name: String,
    pub url: String,
    pub width: i32,
    pub height: i32,
    pub decoration: bool,
    pub transparent: bool,
    pub icon_path: String,
    pub shadow: bool,
}

I don't know why but if you set transparent to true and you want it to take effect, you have to set decoration and shadow to false.
*/
const webview_window_config = {
    window_name: "",
    url: "",

    decoration: true,
    icon_path: "",
    transparent: false,
    shadow: true,
    resizable: true,
    visibility: true,

    size: null as null | {
        width: number;
        height: number;
    },
    position: null as null | {
        x: number;
        y: number;
    },
    parent_window_key: "",
    selector: "",
    kind: "App" as "App" | "Tool" | "Popup",
};

const create_window = async (
    args: Partial<typeof webview_window_config> & {
        window_name: string;
        url: string;
    },
): Promise<void> => {
    const default_args = {
        ...webview_window_config,
        ...args,
    };
    return await invoke_command("create_window", default_args);
};

const minimize_window = async (): Promise<void> => {
    return await invoke_command("minimize_window");
};

const maximize_window = async (): Promise<void> => {
    return await invoke_command("maximize_window");
};

const restore_window = async (): Promise<void> => {
    return await invoke_command("restore_window");
};

const move_window = async (): Promise<void> => {
    return await invoke_command("move_window");
};

const set_decoration = async (args: { decoration: boolean }): Promise<void> => {
    return await invoke_command("set_decoration", args);
};

const close_window = async (): Promise<void> => {
    return await invoke_command("close_window");
};

const send_msg_to_window_by_selector = async (
    target_selector: string,
    msg: any,
) => {
    return await invoke_command("send_msg_to_window_by_selector", {
        target_selector,
        msg,
    });
};

const set_visibility = async (visibility: boolean) => {
    return await invoke_command("set_visibility", { visibility });
};

const set_focus = async () => {
    return await invoke_command("set_focus");
};

const set_position = async (position: { x: number; y: number }) => {
    return await invoke_command("set_position", { position });
};

const set_size = async (size: { width: number; height: number }) => {
    return await invoke_command("set_size", { size });
};

const resize_window = async (args: {
    direction:
        | "Top"
        | "Bottom"
        | "Left"
        | "Right"
        | "TopLeft"
        | "TopRight"
        | "BottomLeft"
        | "BottomRight";
}): Promise<void> => {
    return await invoke_command("resize_window", args);
};

const window_commands = {
    create_window,
    minimize_window,
    move_window,
    set_decoration,
    resize_window,
    close_window,
    maximize_window,
    restore_window,
    send_msg_to_window_by_selector,
    set_visibility,
    set_focus,
    set_position,
    set_size,
};

export default window_commands;
