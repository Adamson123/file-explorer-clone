import invoke_command from "./invoke_command";

const WindowConfig = {
    windwo_name: "",
    url: "",
    width: 600,
    height: 500,
    hide_decoration: false,
    icon_path: "",
};

const create_window = async (
    args: Partial<typeof WindowConfig> & { windwo_name: string; url: string },
): Promise<void> => {
    const default_args = {
        ...WindowConfig,
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

const hide_decoration = async (args: boolean): Promise<void> => {
    return await invoke_command("hide_decoration", args);
};

const window_commands = {
    create_window,
    minimize_window,
    move_window,
    hide_decoration,
};

export default window_commands;
