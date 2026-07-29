import { send_ipc_msg_with_promise } from "./ipc_send_helper";
import type { CommandIPCMsg } from "./types";

const invoke_command = async <T = any>(
    cmd: string,
    args: { [key: string]: any } | undefined = undefined,
): Promise<T | undefined> => {
    const response = await send_ipc_msg_with_promise<CommandIPCMsg>({
        msg_type: "command",
        body: {
            cmd,
            args: args,
            //  request_id: "",
        },
    });

    return response as T;
};

export default invoke_command;
