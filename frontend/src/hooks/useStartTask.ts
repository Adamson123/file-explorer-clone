import { useCallback, useMemo, useRef } from "react";
import IPCHandler from "../lib/ipc_handler";
import { parseJson } from "../utils";

function useStartTask(task_name: string) {
    let called_ref = useRef(false);
    let event_name_ref = useRef("");
    const message_listeners_ref = useRef<
        { callback: (e: any) => void; id: string }[]
    >([]);
    const exit_listeners_ref = useRef<
        { callback: (e: any) => void; id: string }[]
    >([]);

    const start_task = useCallback(async (args: any) => {
        if (called_ref.current) return;
        called_ref.current = true;

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
        event_name_ref.current = event_name as string;

        document.addEventListener(event_name as string, (e: any) => {
            if (message_listeners_ref.current.length) {
                message_listeners_ref.current.forEach((f) =>
                    f.callback(parseJson(e.detail)),
                );
            }
        });

        let event_name_exit = event_name + "_exit";
        document.addEventListener(event_name_exit, (e: any) => {
            if (exit_listeners_ref.current.length) {
                exit_listeners_ref.current.forEach((f) =>
                    f.callback(parseJson(e.detail)),
                );
            }
        });

        return event_name;
    }, []);

    const listener = useMemo(() => {
        return {
            // listeners: [] as (() => void)[],
            on_message(callback: (e: any) => void, id: string) {
                if (
                    message_listeners_ref.current.map((c) => c?.id).includes(id)
                ) {
                    return () => {};
                }

                message_listeners_ref.current.push({ id, callback });
                return () => {
                    message_listeners_ref.current =
                        message_listeners_ref.current.filter(
                            (f) => f.id !== id,
                        );
                    console.log("Removed");
                };
            },

            on_exit(
                callback: (e: any) => void,
                id: string = crypto.randomUUID(),
            ) {
                if (exit_listeners_ref.current.map((c) => c?.id).includes(id)) {
                    return () => {};
                }

                exit_listeners_ref.current.push({ id, callback });
                return () => {
                    exit_listeners_ref.current =
                        exit_listeners_ref.current.filter((f) => f.id !== id);
                    console.log("Removed");
                };
            },

            send_msg: (args: any) => {
                (window as any).ipc.postMessage(
                    JSON.stringify({
                        event_name: event_name_ref.current,
                        args: { sender: "listener", data: args },
                        msg_type: "task",
                        action: "task_msg",
                    }),
                );
            },

            pause: () => {
                (window as any).ipc.postMessage(
                    JSON.stringify({
                        event_name: event_name_ref.current,
                        args: { sender: "manager", data: { state: "pause" } },
                        msg_type: "task",
                        action: "task_msg",
                    }),
                );
            },

            resume: () => {
                (window as any).ipc.postMessage(
                    JSON.stringify({
                        event_name: event_name_ref.current,
                        args: { sender: "manager", data: { state: "run" } },
                        msg_type: "task",
                        action: "task_msg",
                    }),
                );
            },

            cancel() {
                (window as any).ipc.postMessage(
                    JSON.stringify({
                        event_name: event_name_ref.current,
                        args: { sender: "manager", data: { state: "cancel" } },
                        msg_type: "task",
                        action: "task_msg",
                    }),
                );

                called_ref.current = false;
            },

            remove_message_listener(id: string) {
                message_listeners_ref.current =
                    message_listeners_ref.current.filter((f) => f.id !== id);
            },

            remove_exit_listener(id: string) {
                exit_listeners_ref.current = exit_listeners_ref.current.filter(
                    (f) => f.id !== id,
                );
            },
        };
    }, []);

    return [listener, start_task] as const;
}

export default useStartTask;
