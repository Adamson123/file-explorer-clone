import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import IPCHandler from "../lib/ipc_handler";
import { parseJson } from "../utils";

type ListenerCallback = { callback: (e: any) => void; id: string };

function useStartTask(task_name: string) {
    // let called_ref = useRef(false);

    const task_id_ref = useRef(crypto.randomUUID());
    let event_name_ref = useRef(`${task_name}_${task_id_ref.current}`);

    const listeners_ref = useRef<{
        message_listeners: ListenerCallback[];
        exit_listeners: ListenerCallback[];
        error_listeners: ListenerCallback[];
    }>({
        message_listeners: [],
        exit_listeners: [],
        error_listeners: [],
    });

    const [isStarted, setIsStarted] = useState(false);

    // useEffect(() => {
    //     const reset_call_state = () => {
    //         called_ref.current = false;
    //     };

    //     window.addEventListener("load", reset_call_state);

    //     return () => {
    //         window.removeEventListener("load", reset_call_state);
    //     };
    // }, []);

    const task_events = useMemo(() => {
        const listeners = listeners_ref.current;

        const on_message_func = (e: any) => {
            if (listeners.message_listeners.length) {
                listeners.message_listeners.forEach((f) =>
                    f.callback(parseJson(e.detail)),
                );
            }
        };

        const on_exit_func = (e: any) => {
            if (listeners.exit_listeners.length) {
                listeners.exit_listeners.forEach((f) =>
                    f.callback(parseJson(e.detail)),
                );
            }
            task_events.remove_on_exit();
        };

        return {
            add_on_message() {
                document.addEventListener(
                    event_name_ref.current,
                    on_message_func,
                );
            },

            add_on_exit() {
                let event_name_exit = event_name_ref.current + "_exit";
                document.addEventListener(event_name_exit, on_exit_func);
            },

            remove_on_message() {
                document.removeEventListener(
                    event_name_ref.current,
                    on_message_func,
                );
            },

            remove_on_exit() {
                let event_name_exit = event_name_ref.current + "_exit";
                document.removeEventListener(event_name_exit, on_exit_func);
            },
        };
    }, []);

    const start_task = useCallback(async (args: any) => {
        // if (called_ref.current) return;
        // called_ref.current = true;

        task_events.add_on_message();
        task_events.add_on_exit();

        //Send task_name, task_id to rust backend -> rust backend combines task_name + task_id to form event_name, this is the only time we will send task_name
        const start_msg = await new Promise((res, rej) => {
            const id = IPCHandler.addPromise(res, rej);
            (window as any).ipc.postMessage(
                JSON.stringify({
                    task_name,
                    task_id: task_id_ref.current,
                    args: { sender: "listener", data: args },
                    id,
                    msg_type: "task",
                    action: "start",
                }),
            );
        });

        setIsStarted(true);
        return start_msg;
    }, []);

    const events_handler = useMemo(() => {
        const listeners = listeners_ref.current;

        const msg_default_obj = {
            event_name: event_name_ref.current,
            task_id: task_id_ref.current,
            args: {} as any,
            msg_type: "task",
            action: "task_msg",
        };

        const get_msg_obj = (
            fields: Partial<typeof msg_default_obj> & {
                [key: string]: unknown;
            },
        ) => ({
            ...msg_default_obj,
            ...fields,
        });

        get_msg_obj({});

        return {
            add_message_listener(callback: (e: any) => void, id: string) {
                if (
                    listeners.message_listeners.map((c) => c?.id).includes(id)
                ) {
                    return;
                }

                listeners.message_listeners.push({ id, callback });
            },

            add_exit_listener(
                callback: (e: any) => void,
                id: string = crypto.randomUUID(),
            ) {
                if (listeners.exit_listeners.map((c) => c?.id).includes(id)) {
                    return;
                }

                listeners.exit_listeners.push({ id, callback });
            },

            send_msg: (args: any) => {
                (window as any).ipc.postMessage(
                    JSON.stringify(
                        get_msg_obj({
                            args: { sender: "listener", data: args },
                        }),
                    ),
                );
            },

            pause: () => {
                (window as any).ipc.postMessage(
                    JSON.stringify(
                        get_msg_obj({
                            args: {
                                sender: "manager",
                                data: { state: "pause" },
                            },
                        }),
                    ),
                );
            },

            resume: () => {
                (window as any).ipc.postMessage(
                    JSON.stringify(
                        get_msg_obj({
                            args: { sender: "manager", data: { state: "run" } },
                        }),
                    ),
                );
            },

            cancel() {
                (window as any).ipc.postMessage(
                    JSON.stringify(
                        get_msg_obj({
                            args: {
                                sender: "manager",
                                data: { state: "cancel" },
                            },
                        }),
                    ),
                );

                task_events.remove_on_message();
                //Removing exit listener immediately task is stopped, will make exit listener miss task exit message
                //So we will remove exit listener when exit message arrives
                //task_events.remove_on_exit();

                // called_ref.current = false;
                setIsStarted(false);
            },

            remove_message_listener(id: string) {
                let message_listeners = listeners_ref.current.message_listeners;
                message_listeners = message_listeners.filter(
                    (f) => f.id !== id,
                );
            },

            remove_exit_listener(id: string) {
                let exit_listeners = listeners_ref.current.exit_listeners;
                exit_listeners = exit_listeners.filter((f) => f.id !== id);
            },

            remove_all_message_listeners() {
                listeners_ref.current.message_listeners = [];
            },

            remove_all_exit_listeners() {
                listeners_ref.current.exit_listeners = [];
            },
        };
    }, []);

    return { events_handler, start_task, isStarted };
}

export default useStartTask;
