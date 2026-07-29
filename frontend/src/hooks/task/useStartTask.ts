import { useCallback, useRef, useState } from "react";
import useTaskEvents from "./useTaskEvents";
import useEventsHandler from "./useEventsHandler";
import type { TaskIPCMsg } from "../../lib/types";
import { send_ipc_msg_with_promise } from "../../lib/ipc_send_helper";

type ListenerCallback = { callback: (e: any) => void; id: string };

export type ListenersRef = React.RefObject<{
    message_listeners: ListenerCallback[];
    exit_listeners: ListenerCallback[];
    error_listeners: ListenerCallback[];
}>;

function useStartTask<T = any>(task_name: string) {
    // let called_ref = useRef(false);

    const task_id_ref = useRef(crypto.randomUUID());
    const event_name_ref = useRef(`${task_name}_${task_id_ref.current}`);

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
    const task_events = useTaskEvents(listeners_ref, event_name_ref);
    const events_handler = useEventsHandler<T>(
        listeners_ref,
        event_name_ref,
        task_id_ref,
        task_name,
        setIsStarted,
    );

    const start_task = useCallback(async (args: any) => {
        task_events.add_on_message();
        task_events.add_on_exit();
        task_events.add_on_error();

        const start_msg = await send_ipc_msg_with_promise<TaskIPCMsg>({
            msg_type: "task",
            body: {
                action: "Start",
                task_name,
                task_id: task_id_ref.current,
                event_name: event_name_ref.current,
                args: { sender: "listener", data: args },
            },
        });

        setIsStarted(true);
        return start_msg;
    }, []);

    return { events_handler, start_task, isStarted };
}

export default useStartTask;
