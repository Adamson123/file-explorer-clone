import { useMemo } from "react";
import type { ListenersRef } from "./useStartTask";
import type useTaskEvents from "./useTaskEvents";

function useEventsHandler(
    listeners_ref: ListenersRef,
    event_name_ref: React.RefObject<string>,
    task_id_ref: React.RefObject<string>,
    task_events: ReturnType<typeof useTaskEvents>,
    setIsStarted: React.Dispatch<React.SetStateAction<boolean>>,
) {
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

        return {
            add_message_listener(callback: (e: any) => void, id: string) {
                if (
                    listeners.message_listeners.map((c) => c?.id).includes(id)
                ) {
                    return;
                }

                listeners.message_listeners.push({ id, callback });
            },

            add_exit_listener(callback: (e: any) => void, id: string) {
                if (listeners.exit_listeners.map((c) => c?.id).includes(id)) {
                    return;
                }

                listeners.exit_listeners.push({ id, callback });
            },

            add_error_listener(callback: (e: any) => void, id: string) {
                if (listeners.error_listeners.map((c) => c?.id).includes(id)) {
                    return;
                }

                listeners.error_listeners.push({ id, callback });
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

                //  task_events.remove_on_error();
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

    return events_handler;
}

export default useEventsHandler;
