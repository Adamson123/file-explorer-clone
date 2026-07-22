import { useCallback, useRef } from "react";
import IPCHandler from "../lib/ipc_handler";

const parseResponse = (data: any) => {
    try {
        // let d = JSON.parse(data);
        // console.log({ dddd: d });
        // return d;
        return JSON.parse(data);
    } catch (error) {
        //   console.log("Not parsable");
        return data;
    }
};

function useStartTask(task_name: string) {
    let called_ref = useRef(false);
    let event_name_ref = useRef("");

    const start_task = async (args: any) => {
        if (called_ref.current) return;

        called_ref.current = true;
        const e_name = await new Promise((res, rej) => {
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

        event_name_ref.current = e_name as string;

        return e_name;
    };

    const listener = {
        listen: (callback: (d: any) => void) => {
            document.addEventListener(
                event_name_ref.current as string,
                (event: any) => {
                    callback(parseResponse(event.detail));
                },
            );
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

        unlisten: () => {},
    };

    return [listener, start_task] as const;
}

// type Listener = {
//     listen: (callback: (d: any) => void) => void;
//     send_msg: (args: any) => void;
//     unlisten: () => void;
// };

// function useStartTask(
//     task_name: string,
// ): [Listener, (args: any) => Promise<unknown>] {
//     const calledRef = useRef(false);
//     const eventNameRef = useRef("");
//     const listenerRef = useRef<((event: any) => void) | null>(null);

//     const start_task = useCallback(
//         async (args: any) => {
//             if (calledRef.current) return;
//             console.log("Ran");
//             const e_name = await new Promise((res, rej) => {
//                 const id = IPCHandler.addPromise(res, rej);
//                 (window as any).ipc.postMessage(
//                     JSON.stringify({
//                         task_name,
//                         args: { sender: "listener", data: args },
//                         id,
//                         msg_type: "task",
//                         action: "start",
//                     }),
//                 );
//             });
//             calledRef.current = true;
//             eventNameRef.current = e_name as string;
//             return e_name;
//         },
//         [task_name],
//     );

//     const listener: Listener = {
//         listen: useCallback((callback: (d: any) => void) => {
//             const handler = (event: any) => callback(event.detail);
//             listenerRef.current = handler;
//             document.addEventListener(eventNameRef.current, handler);
//         }, []),
//         send_msg: useCallback((args: any) => {
//             (window as any).ipc.postMessage(
//                 JSON.stringify({
//                     event_name: eventNameRef.current,
//                     args: { sender: "listener", data: args },
//                     msg_type: "task",
//                     action: "task_msg",
//                 }),
//             );
//         }, []),
//         unlisten: useCallback(() => {
//             if (listenerRef.current) {
//                 document.removeEventListener(
//                     eventNameRef.current,
//                     listenerRef.current,
//                 );
//                 listenerRef.current = null;
//             }
//         }, []),
//     };

//     return [listener, start_task];
// }

export default useStartTask;
