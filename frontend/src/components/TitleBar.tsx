import {
    ArrowLeft,
    ArrowRight,
    Minus,
    Square,
    SquareDashed,
    SquareSigma,
    SquaresIntersect,
    SquaresUnite,
    X,
} from "lucide-react";
import window_commands from "../lib/window_commands";
import { useState } from "react";
//import invoke_command from "../lib/invoke_command";

//If there's an issue with clicking a child inside a parent with move-window="true", we can add move-window="false" to the child element to solve it. This is useful for buttons inside a title bar, for example.
const TitleBar = () => {
    const [is_maximized, set_is_maximized] = useState(false);

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
            {/* TODO: Wrap each of them with buttons */}
            <div className="flex gap-6 items-center">
                <Minus
                    move-window="false"
                    onClick={() => window_commands.minimize_window()}
                    className="text-gray-400 size-4"
                />
                {!is_maximized ? (
                    <Square
                        move-window="false"
                        onClick={async () => {
                            await window_commands.maximize_window();
                            set_is_maximized(true);
                        }}
                        className="text-gray-400 size-3"
                    />
                ) : (
                    <SquaresUnite
                        move-window="false"
                        onClick={async () => {
                            await window_commands.restore_window();
                            set_is_maximized(false);
                        }}
                        className="text-gray-400 size-3"
                    />
                )}
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
