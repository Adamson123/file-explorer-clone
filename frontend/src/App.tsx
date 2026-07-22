import { useEffect, useRef, useState } from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "./assets/vite.svg";
import heroImg from "./assets/hero.png";
import "./index.css";
import "./App.css";
import invoke_command from "./lib/invoke_command";

function App() {
    const [count, setCount] = useState(0);
    const minimize = useRef(true);
    //  const mouseDown = useRef(false);

    useEffect(() => {
        (async () => {
            console.log("ss");

            const res = await invoke_command<{ name: string }>("log_struct", {
                name: "Data from frontend",
            });
            console.log({ res, name: res?.name });
        })();
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
        const req = await invoke_command("create_window", {
            window_name: "Note.txt",
            url: "http://localhost:5173/", //"https://www.youtube.com/watch?v=LffX3pZ2BiA&t=68s", //"file:///C:/Users/Admin/Downloads/rust_memory_layouts.txt",
        });
        console.log(req);
    };

    const monitor_dir = async () => {
        const req = await invoke_command("monitor_dir");
        console.log(req);
    };

    return (
        <>
            <section id="center">
                <div
                    // onMouseDown={() => (mouseDown.current = true)}
                    // onMouseUp={() => (mouseDown.current = false)}
                    // onMouseLeave={() => (mouseDown.current = false)}
                    // onMouseMove={move_window}
                    onMouseDown={move_window}
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
                        Edit <code>src/App.tsx</code> and save to test{" "}
                        <code>HMR</code>
                    </p>
                </div>
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
