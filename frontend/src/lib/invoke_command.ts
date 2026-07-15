import IPCHandler from "./ipc_handler";

type CMD =
    | "minimize_window"
    | "move_window"
    | "hide_decoration"
    | (string & {});

const invoke_command = async <T = any>(
    cmd: CMD,
    args: any = "",
): Promise<T | undefined> => {
    // try {
    const response = await new Promise((res, rej) => {
        const id = IPCHandler.addPromise(res, rej);
        (window as any).ipc.postMessage(
            JSON.stringify({
                cmd,
                args,
                id,
            }),
        );
    });
    return response as T;
};

export default invoke_command;
