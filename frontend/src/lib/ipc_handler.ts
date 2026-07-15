import { RustError } from "./error";

type IPCHandlerExecutors = {
    resolve: (value: unknown) => void;
    reject: (reason?: any) => void;
    id: string;
};

class IPCHandler {
    static executors: Map<string, IPCHandlerExecutors> = new Map();

    static addPromise(
        res: IPCHandlerExecutors["resolve"],
        rej: IPCHandlerExecutors["reject"],
    ) {
        const id = crypto.randomUUID();
        IPCHandler.executors.set(id, { resolve: res, reject: rej, id });
        return id;
    }

    static parseResponse(data: any) {
        try {
            return JSON.parse(data);
        } catch (error) {
            return data;
        }
    }

    static listen() {
        document.addEventListener("ipc-response", (event: Event) => {
            const customEvent = event as CustomEvent;
            const { id, data, error } = customEvent.detail;
            const executor = IPCHandler.executors.get(id);

            if (data) {
                executor?.resolve(IPCHandler.parseResponse(data));
                //   console.log("Resolved: " + data, "Id: " + id);
            }
            if (error) {
                executor?.reject(new RustError(error));
                //    console.log("Rejected: " + String(error), "Id: " + id);
            }
        });
    }
}

IPCHandler.listen();

export default IPCHandler;
