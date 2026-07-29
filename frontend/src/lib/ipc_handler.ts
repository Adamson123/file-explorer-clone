import { parseJson } from "../utils";
import { RustError } from "./error";

type IPCHandlerExecutors = {
    resolve: (value: unknown) => void;
    reject: (reason?: any) => void;
    request_id: string;
};

class IPCHandler {
    static executors: Map<string, IPCHandlerExecutors> = new Map();

    static addPromise(
        res: IPCHandlerExecutors["resolve"],
        rej: IPCHandlerExecutors["reject"],
    ) {
        const request_id = crypto.randomUUID();
        IPCHandler.executors.set(request_id, {
            resolve: res,
            reject: rej,
            request_id,
        });
        return request_id;
    }

    static listen() {
        document.addEventListener("ipc-response", (event: Event) => {
            const customEvent = event as CustomEvent;
            // console.log(customEvent.detail);
            const { request_id, data, error } = customEvent.detail;
            const executor = IPCHandler.executors.get(request_id);

            if (data) {
                executor?.resolve(parseJson(data));
            }
            if (error) {
                executor?.reject(new RustError(error));
            }

            if (!data && !error) executor?.resolve(parseJson(data));
        });
    }
}

IPCHandler.listen();

export default IPCHandler;
