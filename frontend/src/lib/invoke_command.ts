import IPCHandler from "./ipc_handler";

type CMD =
    | "minimize_window"
    | "move_window"
    | "set_decoration"
    | "create_window"
    | (string & {});

const invoke_command = async <T = any>(
    cmd: CMD,
    args: any = "",
): Promise<T | undefined> => {
    const response = await new Promise((res, rej) => {
        const id = IPCHandler.addPromise(res, rej);
        (window as any).ipc.postMessage(
            JSON.stringify({
                cmd,
                args,
                id,
                type: "command",
            }),
        );
    });
    return response as T;
};

export default invoke_command;
