import { useCallback, useRef, useState } from "react";
import IPCHandler from "../../lib/ipc_handler";
import useTaskEvents from "./useTaskEvents";
import useEventsHandler from "./useEventsHandler";

type ListenerCallback = { callback: (e: any) => void; id: string };

export type ListenersRef = React.RefObject<{
    message_listeners: ListenerCallback[];
    exit_listeners: ListenerCallback[];
    error_listeners: ListenerCallback[];
}>;

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
    const task_events = useTaskEvents(listeners_ref, event_name_ref);
    const events_handler = useEventsHandler(
        listeners_ref,
        event_name_ref,
        task_id_ref,
        task_events,
        setIsStarted,
    );

    // useEffect(() => {
    //     const reset_call_state = () => {
    //         called_ref.current = false;
    //     };

    //     window.addEventListener("load", reset_call_state);

    //     return () => {
    //         window.removeEventListener("load", reset_call_state);
    //     };
    // }, []);

    const start_task = useCallback(async (args: any) => {
        // if (called_ref.current) return;
        // called_ref.current = true;

        task_events.add_on_message();
        task_events.add_on_exit();
        task_events.add_on_error();

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

    return { events_handler, start_task, isStarted };
}

export default useStartTask;
