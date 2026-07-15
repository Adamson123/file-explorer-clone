//Quick controlled launch script🥱

import { spawn } from "node:child_process";
import { createInterface } from "node:readline";

const logger = (name, msg) => {
    console.log(`[${name}]`, String(msg));
};

const run_rust_backend = () => {
    const rust_backend_process = spawn("cargo", ["run"], {
        cwd: "C:\\Users\\Admin\\dev\\file-explorer-clone\\rust-backend",
        shell: true,
    });

    rust_backend_process.stdout.on("data", (d) => {
        logger("rust", d);
    });

    rust_backend_process.stderr.on("data", (d) => {
        logger("rust", d);
    });

    return rust_backend_process;
};

const rl = createInterface({
    input: process.stdin,
    output: process.stdout,
});

const run_react_frontend = () => {
    const react_frontend_process = spawn("bun", ["run", "dev"], {
        cwd: "C:\\Users\\Admin\\dev\\file-explorer-clone\\frontend",
        shell: true,
    });

    react_frontend_process.stdout.on("data", (d) => {
        logger("react", d);
    });

    react_frontend_process.stderr.on("data", (d) => {
        logger("react", d);
    });

    return react_frontend_process;
};

const end_process = (pid) => {
    spawn("taskkill", ["/pid", pid, "/f", "/t"], {
        shell: true,
        stdio: "ignore",
    });
};

const restart_process = (pid, callback) => {
    end_process(pid);
    return callback();
};

const start = () => {
    let rb = run_rust_backend();
    let rf = run_react_frontend();

    const askQuestion = () => {
        rl.question(
            `
            \nr - restart rust backend
            \nf - restart react frontend
            \nb - restart both
            \nq - end both
            \n:> `,
            (d) => {
                if (!d.trim()) {
                    return askQuestion();
                }

                if (d === "r") {
                    console.log("restarting rust backend...");
                    rb = restart_process(rb.pid, run_rust_backend);
                }

                if (d === "f") {
                    console.log("restarting react frontend...");
                    rf = restart_process(rf.pid, run_react_frontend);
                }

                if (d === "b") {
                    console.log("restarting both...");
                    rb = restart_process(rb.pid, run_rust_backend);
                    rf = restart_process(rf.pid, run_react_frontend);
                }

                if (d === "q") {
                    console.log("Ending both...");
                    end_process(rb.pid);
                    end_process(rf.pid);

                    setTimeout(() => process.exit(0), 2000);
                }

                return askQuestion();
            },
        );
    };

    askQuestion();
};

start();
