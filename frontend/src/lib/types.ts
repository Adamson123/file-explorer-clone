export type CommandIPCMsg = {
    cmd: string;
    args: any;
    request_id: string;
};

export type TaskIPCMsg = {
    task_name: string;
    task_id: string;
    event_name: string;
    args: any;
    request_id: string;
    action: "Start" | "TaskMsg" | "ForceKill" | "KillAll";
};

export type IPCMsg<T extends CommandIPCMsg | TaskIPCMsg> = {
    msg_type: "command" | "task";
    body: T;
};
