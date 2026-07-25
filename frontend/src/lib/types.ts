export type CommandMsg = {
    cmd: string;
    args: any;
    id: string;
};

export type TaskMsg = {
    task_name: string;
    task_id: string;
    event_name: string;
    args: any;
    id: string;
    action: "start" | "task_msg" | "force_kill" | "kill_all";
};

export type IPCMsg = {
    msg_type: "command" | "task";
    msg: CommandMsg | TaskMsg;
};
