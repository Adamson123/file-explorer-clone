import window_commands, { type WindowIPCCom } from "./window_commands";

export type Exchange = "request" | "response";

export type WindowAPIHandlerRequest = CustomEvent<{
    data: {
        type: string;
        route: string;
        value: any;
        request_id: string;
        exchange: Exchange;
    };
    from: WindowIPCCom["detail"]["from"];
}> & { res: (e: any) => void };

export type WindowAPIHandlerResponse = CustomEvent<{
    data: {
        type: string;
        value: any;
        request_id: string;
        exchange: Exchange;
    };
    from: WindowIPCCom["detail"]["from"];
}>;

export type WindowAPIHandlerExecutors = {
    resolve: (value: WindowAPIHandlerResponse) => void;
    reject: (reason?: any) => void;
    request_id: string;
};

export const send_request_to_window = async (
    target_selector: string,
    route: string,
    msg: any,
    timeout: number = 10000, //10s
): Promise<WindowAPIHandlerResponse> => {
    let timeout_id: any;
    return (await new Promise((res, rej) => {
        const request_id = WindowAPIHandler.addPromise(res, rej);
        window_commands.send_msg_to_window_by_selector(target_selector, {
            type: WindowAPIHandler.type,
            route: route,
            value: msg,
            request_id,
            exchange: "request",
        } as WindowAPIHandlerRequest["detail"]["data"]);

        timeout_id = setTimeout(() => {
            rej("Timeout");
        }, timeout);
    }).finally(() => {
        clearTimeout(timeout_id);
    })) as Promise<WindowAPIHandlerResponse>;
};

export const send_response_to_window = async (
    target_selector: string,
    request_id: string,
    msg: any,
) => {
    window_commands.send_msg_to_window_by_selector(target_selector, {
        type: WindowAPIHandler.type,
        value: msg,
        request_id,
        exchange: "response",
    } as WindowAPIHandlerResponse["detail"]["data"]);
};

export default class WindowAPIHandler {
    static type = "/api";
    static executors: Map<string, WindowAPIHandlerExecutors> = new Map();
    static requests_handlers: {
        route: string;
        fn: (e: WindowAPIHandlerRequest) => void;
    }[] = [];

    static addPromise(
        res: WindowAPIHandlerExecutors["resolve"],
        rej: WindowAPIHandlerExecutors["reject"],
    ) {
        const request_id = crypto.randomUUID();
        WindowAPIHandler.executors.set(request_id, {
            resolve: res,
            reject: rej,
            request_id,
        });

        return request_id;
    }

    static listen() {
        document.addEventListener("window_ipc_com", (event: Event) => {
            const customEvent = event as CustomEvent;

            let _data = customEvent.detail.data;
            if (!(_data?.type === WindowAPIHandler.type)) return;

            let data = _data as
                | WindowAPIHandlerRequest["detail"]["data"]
                | WindowAPIHandlerResponse["detail"]["data"];

            const exchange = data.exchange;

            //Handle incoming response
            if (exchange === "response") {
                const request_id = data.request_id;
                const executor = WindowAPIHandler.executors.get(request_id);
                executor?.resolve(customEvent as any);
            }

            //Handle incoming requests
            let route = (data as any)?.route;
            if (
                exchange === "request" &&
                route &&
                WindowAPIHandler.requests_handlers.length
            ) {
                for (let rq of WindowAPIHandler.requests_handlers) {
                    let event = customEvent as WindowAPIHandlerRequest;
                    event.res = (e: any) => {
                        send_response_to_window(
                            event.detail.from.selector,
                            event.detail.data.request_id,
                            e,
                        );
                    };
                    if (rq.route === route) rq.fn(event);
                }
            }
        });
    }

    static listen_for_request(
        route: string,
        callback: (e: WindowAPIHandlerRequest) => void,
    ) {
        WindowAPIHandler.requests_handlers.push({
            route,
            fn: callback,
        });
    }
}

WindowAPIHandler.listen();
