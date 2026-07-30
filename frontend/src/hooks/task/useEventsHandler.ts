import { useMemo } from "react";
import type { ListenersRef } from "./useStartTask";
import type { IPCMsg, TaskIPCMsg } from "../../lib/types";
import { send_ipc_msg_with_promise } from "../../lib/ipc_send_helper";

function useEventsHandler<T>(
    listeners_ref: ListenersRef,
    event_name_ref: React.RefObject<string>,
    task_id_ref: React.RefObject<string>,
    task_name: string,
    set_is_started: React.Dispatch<React.SetStateAction<boolean>>,
) {
    const events_handler = useMemo(() => {
        const listeners = listeners_ref.current;

        const msg_default_obj: IPCMsg<TaskIPCMsg> = {
            msg_type: "task",
            body: {
                task_name,
                task_id: task_id_ref.current,
                event_name: event_name_ref.current,
                args: {} as any,
                request_id: "",
                action: "TaskMsg",
            },
        };

        const get_msg_obj = (
            fields: Partial<typeof msg_default_obj> & {
                [key: string]: unknown;
            },
        ) => ({
            msg_type: msg_default_obj.msg_type,
            body: { ...msg_default_obj.body, ...fields },
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

            send_msg: async (args: T) => {
                await send_ipc_msg_with_promise<TaskIPCMsg>(
                    get_msg_obj({
                        args: { sender: "listener", data: args },
                    }),
                );
            },

            pause: async () => {
                await send_ipc_msg_with_promise<TaskIPCMsg>(
                    get_msg_obj({
                        args: {
                            sender: "manager",
                            data: { state: "pause" },
                        },
                    }),
                );
            },

            resume: async () => {
                await send_ipc_msg_with_promise<TaskIPCMsg>(
                    get_msg_obj({
                        args: { sender: "manager", data: { state: "run" } },
                    }),
                );
            },

            cancel: async () => {
                await send_ipc_msg_with_promise<TaskIPCMsg>(
                    get_msg_obj({
                        args: {
                            sender: "manager",
                            data: { state: "cancel" },
                        },
                    }),
                );

                //  task_events.remove_on_error();
                //Removing exit listener immediately task is stopped, will make exit listener miss task exit message
                //So we will remove exit listener when exit message arrives
                //task_events.remove_on_exit();

                // called_ref.current = false;
                set_is_started(false);
            },
            //TODO: Force kill task in rust backend, currently cancel is just a message to rust backend to cancel the task, but if the task is stuck in a loop or waiting for something, it will not be cancelled, we need to force cancel the task in rust backend, so we need to send a message to rust backend to force cancel the task, and rust backend will abort the task
            force_cancel: async () => {
                await send_ipc_msg_with_promise<TaskIPCMsg>(
                    get_msg_obj({
                        action: "ForceKill",
                    }),
                );
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
