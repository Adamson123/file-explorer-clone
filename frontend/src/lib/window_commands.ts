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
    width: 600,
    height: 500,
    decoration: true,
    icon_path: "",
    transparent: false,
    shadow: true,
    resizable: true,
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

const move_window = async (): Promise<void> => {
    return await invoke_command("move_window");
};

const set_decoration = async (args: boolean): Promise<void> => {
    return await invoke_command("set_decoration", args);
};

const window_commands = {
    create_window,
    minimize_window,
    move_window,
    set_decoration,
};

export default window_commands;
