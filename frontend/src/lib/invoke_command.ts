import { send_ipc_msg_with_promise } from "./ipc_send_helper";
import type { CommandIPCMsg } from "./types";

type CMD =
    | "minimize_window"
    | "move_window"
    | "set_decoration"
    | "create_window"
    | (string & {});

const invoke_command = async <T = any>(
    cmd: CMD,
    args: { [key: string]: any } | undefined = undefined,
): Promise<T | undefined> => {
    const response = await send_ipc_msg_with_promise<CommandIPCMsg>({
        msg_type: "command",
        body: {
            cmd,
            args,
            //  request_id: "",
        },
    });

    // const response = await new Promise((res, rej) => {
    //     const id = IPCHandler.addPromise(res, rej);
    //     (window as any).ipc.postMessage(
    //         JSON.stringify({
    //             msg_type: "command",
    //      body: {
    //             cmd,
    //          args,
    //           request_id: id,
    //            },
    //         } as IPCMsg<CommandIPCMsg>),
    //     );
    // });

    return response as T;
};

export default invoke_command;
