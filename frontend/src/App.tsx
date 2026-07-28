import LeftSection from "./components/LeftSection";
import RightSection from "./components/RightSection";
import TitleBar from "./components/TitleBar";

const App = () => {
    return (
        <main className="w-screen h-screen">
            {/* <div className="absolute inset-0 w-full h-full object-cover blur-2xl pointer-events-none" /> */}
            {/*
            rounded-lg overflow-hidden
            */}
            <div className="flex flex-col size-full">
                <TitleBar />
                {/* grid grid-cols-[20%_80%] grow */}
                <div className="w-full grow flex">
                    <LeftSection />
                    <RightSection />
                </div>
            </div>
        </main>
    );
};

export default App;
