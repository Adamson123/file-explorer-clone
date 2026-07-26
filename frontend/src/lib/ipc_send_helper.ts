import IPCHandler from "./ipc_handler";
import { type IPCMsg, type CommandIPCMsg, type TaskIPCMsg } from "./types";

const send_ipc_msg = <T extends CommandIPCMsg | TaskIPCMsg>(
    ipc_msg: IPCMsg<T>,
) => {
    (window as any).ipc.postMessage(JSON.stringify(ipc_msg));
};

const send_ipc_msg_with_promise = <T extends CommandIPCMsg | TaskIPCMsg>(
    ipc_msg: Omit<IPCMsg<T>, "body"> & { body: Omit<T, "request_id"> },
): Promise<any> => {
    return new Promise((res, rej) => {
        const id = IPCHandler.addPromise(res, rej);
        send_ipc_msg<T>({
            msg_type: ipc_msg.msg_type,
            body: {
                ...ipc_msg.body,
                request_id: id,
            },
        } as IPCMsg<T>);
    });
};

export { send_ipc_msg, send_ipc_msg_with_promise };
