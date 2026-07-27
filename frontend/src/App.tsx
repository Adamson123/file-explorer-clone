import LeftSection from "./components/LeftSection";
import RightSection from "./components/RightSection";
import TitleBar from "./components/TitleBar";
import window_commands from "./lib/window_commands";

const App = () => {
    return (
        <main className="bg-primary w-screen h-screen flex flex-col">
            {/* <div
                onMouseDown={() =>
                    window_commands.resize_window({ direction: "Right" })
                }
                className="fixed top-0 right-0 w-2 bg-amber-200 h-full cursor-ew-resize"
            />
            <div
                onMouseDown={() =>
                    window_commands.resize_window({ direction: "Bottom" })
                }
                className="fixed bottom-0 left-0 w-full bg-amber-200 h-2 cursor-ns-resize"
            />
            <div
                onMouseDown={() =>
                    window_commands.resize_window({ direction: "BottomRight" })
                }
                className="fixed bottom-0 right-0 w-2 bg-amber-200 h-2 cursor-nwse-resize"
            />
            <div
                onMouseDown={() =>
                    window_commands.resize_window({ direction: "Top" })
                }
                className="fixed top-0 left-0 w-full bg-amber-200 h-2 cursor-ns-resize"
            />
            <div
                onMouseDown={() =>
                    window_commands.resize_window({ direction: "Left" })
                }
                className="fixed top-0 left-0 w-2 bg-amber-200 h-full cursor-ew-resize"
            /> */}
            <TitleBar />
            {/* grid grid-cols-[20%_80%] grow */}
            <div className="w-full grow flex">
                <LeftSection />
                <RightSection />
            </div>
        </main>
    );
};

export default App;
