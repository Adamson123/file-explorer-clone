import { useState } from "react";
import LeftSection from "./components/LeftSection";
import RightSection from "./components/RightSection";
import TitleBar from "./components/TitleBar";

const App = () => {
    const [current_dir, set_current_dir] = useState(
        "C:\\Users\\Admin\\dev\\pc-usage\\tauri_app",
    );

    return (
        <main className="w-screen h-screen">
            {/* <div className="absolute inset-0 w-full h-full object-cover blur-2xl pointer-events-none" /> */}
            {/*
            rounded-lg overflow-hidden
            */}
            <div className="flex flex-col size-full rounded-xl overflow-hidden">
                <TitleBar />
                {/* grid grid-cols-[20%_80%] grow */}
                <div className="w-full h-full flex">
                    <LeftSection
                        current_dir={current_dir}
                        set_current_dir={set_current_dir}
                    />
                    <RightSection
                        current_dir={current_dir}
                        set_current_dir={set_current_dir}
                    />
                </div>
            </div>
        </main>
    );
};

export default App;
