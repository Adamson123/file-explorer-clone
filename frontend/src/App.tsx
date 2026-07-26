import { useEffect, useRef, useState } from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "./assets/vite.svg";
import heroImg from "./assets/hero.png";
import "./index.css";
import "./App.css";
import invoke_command from "./lib/invoke_command";
import useStartTask from "./hooks/task/useStartTask";
import window_commands from "./lib/window_commands";
//import start_task from "./lib/start_task";

function App() {
    const [count, setCount] = useState(0);
    const minimize = useRef(true);
    const { events_handler, start_task } = useStartTask("monitor_dir");
    const [pathInput, setPathInput] = useState("");

    useEffect(() => {
        (async () => {
            try {
                console.log("ss");

                const res = await invoke_command<{ name: string }>("log", {
                    name: "Data from frontend invoke",
                });
                console.log({ res, name: res?.name });

                await start_task("hello");
                console.log("After starting task");

                events_handler.add_message_listener((e) => {
                    console.log(e);
                }, "1");

                events_handler.add_exit_listener((e) => {
                    console.log(e);
                }, "1");

                events_handler.add_error_listener((e) => {
                    console.error("Error: ", e);
                }, "1");

                events_handler.send_msg({
                    msg: "from frontend listener",
                    path: "C:\\Users\\Admin\\Downloads\\The Hobbit An Unexpected Journey (2012) [1080p]",
                });
            } catch (error: any) {
                console.error(error.message);
            }
        })();

        // return () => {
        //     events_handler.cancel();
        // };
    }, []);

    const minimize_window = async () => {
        //  invoke_command("minimize_window");

        const res = await invoke_command<{ name: string }>("log_struct", {
            name: "Data from frontend",
        });
        console.log({ res, name: res?.name });
    };

    const move_window = async () => {
        // if (!mouseDown.current) {
        //     return;
        // }
        const res2 = await invoke_command("move_window");
        console.log({ res2 });
    };

    const toggle_decor = () => {
        invoke_command("hide_decoration", { minimize: minimize.current });
        minimize.current = !minimize.current;
    };

    const get_dir_contents = async () => {
        const req = await invoke_command("get_dir_contents", {
            path: "C:\\Users\\Admin\\dev\\file-explorer-clone\\frontend",
        });
        console.log(req);
    };

    const create_window = async () => {
        // const req = await invoke_command("create_window", {
        //     window_name: "Note.txt",
        //     url: "http://localhost:5173/", //"https://www.youtube.com/watch?v=LffX3pZ2BiA&t=68s", ,
        // });
        // //  `file:///${pathInput}`, //"file:///C:/Users/Admin/Downloads/rust_memory_layouts.txt",
        // console.log(req);
        const req = await window_commands.create_window({
            window_name: "New Window from me",
            url: "http://localhost:5173/",
            width: 600,
            height: 500,
            decoration: false,
            icon_path: "",
        });

        console.log(req);
    };

    const monitor_dir = async () => {
        const req = await invoke_command("monitor_dir");
        console.log(req);
    };

    return (
        <>
            <section id="center" className="bg-white">
                <div
                    // onMouseDown={() => (mouseDown.current = true)}
                    // onMouseUp={() => (mouseDown.current = false)}
                    // onMouseLeave={() => (mouseDown.current = false)}
                    // onMouseMove={move_window}
                    //onMouseDown={move_window}
                    move-window="true"
                    style={{
                        background: "red",
                        width: "100%",
                        height: "70px",
                    }}
                    className="border-5"
                ></div>
                <div className="hero">
                    <img
                        src={heroImg}
                        className="base"
                        width="170"
                        height="179"
                        alt=""
                    />
                    <img
                        src={reactLogo}
                        className="framework"
                        alt="React logo"
                    />
                    <img src={viteLogo} className="vite" alt="Vite logo" />
                </div>
                <div>
                    <h1>Get started</h1>
                    <p>
                        Edit <code>src/App.tsx</code> and save to test ooo{" "}
                        <code>HMR</code>
                    </p>
                </div>
                <div className="flex items-center">
                    <input
                        onChange={(e) => setPathInput(e.target.value)}
                        type="text"
                        className="w-100  bg-red-500/40 p-2 border-2"
                    />
                    <button
                        type="button"
                        className="p-2.5 bg-red-300 hover:bg-red-500 active:scale-95"
                        onClick={async () => {
                            await events_handler.send_msg({
                                msg: "from frontend again!!!",
                                path: pathInput, //.replaceAll("\\", "\\\\"),
                            });
                            //console.log({ msg_res: res });
                        }}
                    >
                        Send dir
                    </button>
                </div>

                <button
                    type="button"
                    className="counter"
                    onClick={() => {
                        events_handler.pause();
                    }}
                >
                    Pause
                </button>

                <button
                    type="button"
                    className="counter"
                    onClick={() => {
                        events_handler.resume();
                    }}
                >
                    Resume
                </button>

                <button
                    type="button"
                    className="counter"
                    onClick={() => {
                        events_handler.cancel();
                    }}
                >
                    Cancel
                </button>

                <button
                    type="button"
                    className="counter"
                    onClick={async () => {
                        await events_handler.force_cancel();
                    }}
                >
                    Force cancel
                </button>

                <button
                    type="button"
                    className="counter"
                    onClick={() => {
                        start_task({
                            path: "C:\\Users\\Admin\\Downloads\\youtube-analysis\\requests",
                        }).then(() => {
                            events_handler.remove_message_listener("1");
                            events_handler.add_message_listener((e) => {
                                console.log(e);
                            }, "2");
                        });
                    }}
                >
                    Start
                </button>

                <button
                    type="button"
                    className="counter"
                    onClick={minimize_window}
                >
                    Count is {count}
                </button>

                <button
                    type="button"
                    className="counter"
                    onClick={toggle_decor}
                >
                    toggle_decor
                </button>

                <button
                    type="button"
                    className="counter"
                    onClick={get_dir_contents}
                >
                    get_dir_contents
                </button>

                <button
                    type="button"
                    className="counter"
                    onClick={create_window}
                >
                    create_window
                </button>

                <button type="button" className="counter" onClick={monitor_dir}>
                    monitor_dir
                </button>
            </section>

            <div className="ticks"></div>

            <section id="next-steps">
                <div id="docs">
                    <svg
                        className="icon"
                        role="presentation"
                        aria-hidden="true"
                    >
                        <use href="/icons.svg#documentation-icon"></use>
                    </svg>
                    <h2>Documentation</h2>
                    <p>Your questions, answered</p>
                    <ul>
                        <li>
                            <a href="https://vite.dev/" target="_blank">
                                <img className="logo" src={viteLogo} alt="" />
                                Explore Vite
                            </a>
                        </li>
                        <li>
                            <a href="https://react.dev/" target="_blank">
                                <img
                                    className="button-icon"
                                    src={reactLogo}
                                    alt=""
                                />
                                Learn more
                            </a>
                        </li>
                    </ul>
                </div>
                <div id="social">
                    <svg
                        className="icon"
                        role="presentation"
                        aria-hidden="true"
                    >
                        <use href="/icons.svg#social-icon"></use>
                    </svg>
                    <h2>Connect with us</h2>
                    <p>Join the Vite community</p>
                    <ul>
                        <li>
                            <a
                                href="https://github.com/vitejs/vite"
                                target="_blank"
                            >
                                <svg
                                    className="button-icon"
                                    role="presentation"
                                    aria-hidden="true"
                                >
                                    <use href="/icons.svg#github-icon"></use>
                                </svg>
                                GitHub
                            </a>
                        </li>
                        <li>
                            <a href="https://chat.vite.dev/" target="_blank">
                                <svg
                                    className="button-icon"
                                    role="presentation"
                                    aria-hidden="true"
                                >
                                    <use href="/icons.svg#discord-icon"></use>
                                </svg>
                                Discord
                            </a>
                        </li>
                        <li>
                            <a href="https://x.com/vite_js" target="_blank">
                                <svg
                                    className="button-icon"
                                    role="presentation"
                                    aria-hidden="true"
                                >
                                    <use href="/icons.svg#x-icon"></use>
                                </svg>
                                X.com
                            </a>
                        </li>
                        <li>
                            <a
                                href="https://bsky.app/profile/vite.dev"
                                target="_blank"
                            >
                                <svg
                                    className="button-icon"
                                    role="presentation"
                                    aria-hidden="true"
                                >
                                    <use href="/icons.svg#bluesky-icon"></use>
                                </svg>
                                Bluesky
                            </a>
                        </li>
                    </ul>
                </div>
            </section>

            <div className="ticks"></div>
            <section id="spacer"></section>
        </>
    );
}

export default App;
