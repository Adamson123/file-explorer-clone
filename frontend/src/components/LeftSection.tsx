import { ChevronRight, Pin } from "lucide-react";

const quick_access = [
    {
        name: "Desktop",
        icon: "./assets/desktop.svg",
    },
    {
        name: "Documents",
        icon: "./assets/documents.svg",
    },
    {
        name: "Downloads",
        icon: "./assets/downloads.svg",
    },
    {
        name: "Pictures",
        icon: "./assets/pictures.svg",
    },
    {
        name: "Music",
        icon: "./assets/music.svg",
    },
    {
        name: "Videos",
        icon: "./assets/video.svg",
    },
];

const pinned = [
    {
        name: "Movies",
        icon: "./assets/folder.svg",
    },
    {
        name: "Projects",
        icon: "./assets/folder.svg",
    },
];

const this_pc = [
    {
        name: "Local Disc (C:)",
        icon: "./assets/harddisk.svg",
    },
    {
        name: "Local Disc (D:)",
        icon: "./assets/harddisk.svg",
    },
];

const LeftSection = () => {
    return (
        // bg-[#1b1b1b]
        <section className="w-full max-w-75 pl-5.5 p-4.5 flex z-10 flex-col gap-8  bg-transparent-black">
            {/* <div className="bg-red-300 size-10 absolute top-10 left-6 z-[-10]"></div> */}
            {/* Quick Access */}
            <div className="gap-1 flex flex-col">
                <h2 className="text-gray-400 flex text-sm items-center gap-1.5">
                    <ChevronRight className="size-3.5" /> Quick Access
                </h2>
                <div className="pl-4 text-sm flex flex-col gap-3">
                    {quick_access.map((item, index) => (
                        <div
                            key={index}
                            className="flex items-center gap-1.5 p-1"
                        >
                            <img src={item.icon} className="size-4.5" />
                            <p>{item.name}</p>
                        </div>
                    ))}
                </div>
            </div>
            {/* Pinned */}
            <div className="gap-1 flex flex-col">
                <h2 className="text-gray-400 flex items-center gap-1.5 text-sm">
                    <ChevronRight className="size-3.5" /> Pinned
                </h2>
                <div className="pl-4 text-xs flex flex-col gap-3">
                    {pinned.map((item, index) => (
                        <div
                            key={index}
                            className="flex items-center gap-1.5 p-1"
                        >
                            <img src={item.icon} className="size-4.5" />
                            <p>{item.name}</p>
                            <Pin className="size-3.5 text-gray-400 rotate-45 ml-auto" />
                        </div>
                    ))}
                </div>
            </div>
            {/* This PC */}
            <div className="gap-1 flex flex-col">
                <h2 className="text-gray-400 flex items-center gap-1.5 text-sm">
                    <ChevronRight className="size-3.5" /> This PC
                </h2>
                <div className="pl-4 text-xs flex flex-col gap-3">
                    {this_pc.map((item, index) => (
                        <div
                            key={index}
                            className="flex items-center gap-1.5 p-1"
                        >
                            <img src={item.icon} className="size-4.5" />
                            <p>{item.name}</p>
                        </div>
                    ))}
                </div>
            </div>
        </section>
    );
};

export default LeftSection;
