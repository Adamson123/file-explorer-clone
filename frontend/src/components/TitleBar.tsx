import { ArrowLeft, ArrowRight, Minus, Square, X } from "lucide-react";
import window_commands from "../lib/window_commands";
import invoke_command from "../lib/invoke_command";

//If there's an issue with clicking a child inside a parent with move-window="true", we can add move-window="false" to the child element to solve it. This is useful for buttons inside a title bar, for example.
const TitleBar = () => {
    return (
        <section
            move-window="true"
            //onMouseDown={() => {}}
            className="bg-primary py-3 px-4 font-semibold flex items-center justify-between"
        >
            {/* Right */}
            <div className="flex gap-12 items-center">
                <div className="flex gap-4">
                    <ArrowLeft className="text-gray-700/60 size-5" />
                    <ArrowRight className="text-gray-400 size-5" />
                </div>
                <h1>Local Disc (C:)</h1>
            </div>
            {/* Left */}
            <div className="flex gap-6 items-center">
                <Minus
                    move-window="false"
                    onClick={() => window_commands.minimize_window()}
                    className="text-gray-400 size-4"
                />
                <Square
                    move-window="false"
                    onClick={() => {
                        invoke_command("log_window_key");
                    }}
                    className="text-gray-400 size-3"
                />

                <X
                    move-window="false"
                    onClick={() => {
                        window_commands.close_window();
                    }}
                    className="text-gray-400 size-4"
                />
            </div>
        </section>
    );
};

export default TitleBar;
