import IPCHandler from "./ipc_handler";

const start_task = async (task_name: string, args: any) => {
    const event_name = await new Promise((res, rej) => {
        const id = IPCHandler.addPromise(res, rej);
        (window as any).ipc.postMessage(
            JSON.stringify({
                task_name,
                args: { sender: "listener", data: args },
                id,
                msg_type: "task",
                action: "start",
            }),
        );
    });
    console.log({ event_name });

    const listener = {
        listen: (callback: (d: any) => void) => {
            document.addEventListener(event_name as string, (event: any) => {
                callback(event.detail);
            });
        },

        send_msg: (args: any) => {
            (window as any).ipc.postMessage(
                JSON.stringify({
                    event_name,
                    args: { sender: "listener", data: args },
                    msg_type: "task",
                    action: "task_msg",
                }),
            );
        },

        unlisten: () => {},
    };

    return listener;
};

export default start_task;
