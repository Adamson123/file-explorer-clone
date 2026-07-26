import { useMemo } from "react";
import { parseJson } from "../../utils";
import type { ListenersRef } from "./useStartTask";

function useTaskEvents(
    listeners_ref: ListenersRef,
    event_name_ref: React.RefObject<string>,
) {
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
            task_events.remove_on_message();
            task_events.remove_on_error();
        };

        const on_error_func = (e: any) => {
            if (listeners.error_listeners.length) {
                listeners.error_listeners.forEach((f) =>
                    f.callback(parseJson(e.detail)),
                );
            }
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

            add_on_error() {
                let event_name_error = event_name_ref.current + "_error";
                document.addEventListener(event_name_error, on_error_func);
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

            remove_on_error() {
                let event_name_error = event_name_ref.current + "_error";
                document.removeEventListener(event_name_error, on_error_func);
            },
        };
    }, []);

    return task_events;
}

export default useTaskEvents;
