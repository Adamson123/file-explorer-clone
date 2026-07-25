export type CommandIPCMsg = {
    cmd: string;
    args: any;
    id: string;
};

export type TaskIPCMsg = {
    task_name: string;
    task_id: string;
    event_name: string;
    args: any;
    id: string;
    action: "start" | "task_msg" | "force_kill" | "kill_all";
};

export type IPCMsg<T extends CommandIPCMsg | TaskIPCMsg> = {
    msg_type: "command" | "task";
    body: T;
};
