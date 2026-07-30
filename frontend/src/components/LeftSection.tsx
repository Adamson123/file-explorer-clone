import { ChevronRight, Pin } from "lucide-react";
import type { ReactSetStateAction } from "../type";

const quick_access = [
    {
        name: "Desktop",
        icon: "./assets/desktop.svg",
        path: "C:\\Users\\Admin\\Desktop",
    },
    {
        name: "Documents",
        icon: "./assets/documents.svg",
        path: "C:\\Users\\Admin\\Documents",
    },
    {
        name: "Downloads",
        icon: "./assets/downloads.svg",
        path: "C:\\Users\\Admin\\Downloads",
    },
    {
        name: "Pictures",
        icon: "./assets/pictures.svg",
        path: "C:\\Users\\Admin\\Pictures",
    },
    {
        name: "Music",
        icon: "./assets/music.svg",
        path: "C:\\Users\\Admin\\Music",
    },
    {
        name: "Videos",
        icon: "./assets/video.svg",
        path: "C:\\Users\\Admin\\Videos",
    },
];

const pinned = [
    {
        name: "Movies",
        icon: "./assets/folder.svg",
        path: "",
    },
    {
        name: "Projects",
        icon: "./assets/folder.svg",
        path: "",
    },
];

const this_pc = [
    {
        name: "Local Disc (C:)",
        icon: "./assets/harddisk.svg",
        path: "C:\\",
    },
    {
        name: "Local Disc (D:)",
        icon: "./assets/harddisk.svg",
        path: "D:\\",
    },
];

const ShortcutCard = ({
    name,
    icon,
    path,
    set_current_dir,
    current_dir,
    OtherElements = undefined,
}: (typeof quick_access)[0] & {
    set_current_dir: ReactSetStateAction<string>;
    current_dir: string;
    OtherElements?: React.ReactNode;
}) => {
    return (
        <div
            onClick={() => set_current_dir(path)}
            className={`flex items-center gap-1.5 p-2.5 cursor-pointer hover:bg-gray-400/10 rounded-lg transition-colors duration-100 ${
                current_dir === path && "bg-gray-400/10"
            }`}
        >
            <img src={icon} className="size-4.5" />
            <p>{name}</p>
            {OtherElements}
        </div>
    );
};

const LeftSection = ({
    set_current_dir,
    current_dir,
}: {
    set_current_dir: ReactSetStateAction<string>;
    current_dir: string;
}) => {
    return (
        // bg-[#1b1b1b]
        <section
            style={{
                scrollbarWidth: "thin",
                scrollbarColor:
                    "var(--color-primary) color-mix(in oklab, var(--color-gray-400) 10%, transparent)",
            }}
            className="w-full max-w-75 pl-5.5 p-4.5 flex z-10 flex-col gap-8
          bg-transparent-black overflow-y-auto pb-25"
        >
            {/* <div className="bg-red-300 size-10 absolute top-10 left-6 z-[-10]"></div> */}
            {/* Quick Access */}
            <div className="gap-1 flex flex-col">
                <h2 className="text-gray-400 flex text-sm items-center gap-1.5">
                    <ChevronRight className="size-3.5" /> Quick Access
                </h2>
                <div className="pl-4 text-sm flex flex-col gap-1">
                    {quick_access.map((item, index) => (
                        // <div
                        //     key={index}
                        //     className="flex items-center gap-1.5 p-1"
                        // >
                        //     <img src={item.icon} className="size-4.5" />
                        //     <p>{item.name}</p>
                        // </div>
                        <ShortcutCard
                            {...item}
                            key={index}
                            current_dir={current_dir}
                            set_current_dir={set_current_dir}
                        />
                    ))}
                </div>
            </div>
            {/* Pinned */}
            <div className="gap-1 flex flex-col">
                <h2 className="text-gray-400 flex items-center gap-1.5 text-sm">
                    <ChevronRight className="size-3.5" /> Pinned
                </h2>
                <div className="pl-4 text-xs flex flex-col gap-1">
                    {pinned.map((item, index) => (
                        // <div
                        //     onClick={() => set_current_dir(item.path)}
                        //     key={index}
                        //     className="flex items-center gap-1.5 p-1"
                        // >
                        //     <img src={item.icon} className="size-4.5" />
                        //     <p>{item.name}</p>
                        //     <Pin className="size-3.5 text-gray-400 rotate-45 ml-auto" />
                        // </div>
                        <ShortcutCard
                            {...item}
                            key={index}
                            set_current_dir={set_current_dir}
                            current_dir={current_dir}
                            OtherElements={
                                <Pin className="size-3.5 text-gray-400 rotate-45 ml-auto" />
                            }
                        />
                    ))}
                </div>
            </div>
            {/* This PC */}
            <div className="gap-1 flex flex-col">
                <h2 className="text-gray-400 flex items-center gap-1.5 text-sm">
                    <ChevronRight className="size-3.5" /> This PC
                </h2>
                <div className="pl-4 text-xs flex flex-col gap-1">
                    {this_pc.map((item, index) => (
                        // <div
                        //     key={index}
                        //     className="flex items-center gap-1.5 p-1"
                        // >
                        //     <img src={item.icon} className="size-4.5" />
                        //     <p>{item.name}</p>
                        // </div>
                        <ShortcutCard
                            {...item}
                            key={index}
                            current_dir={current_dir}
                            set_current_dir={set_current_dir}
                        />
                    ))}
                </div>
            </div>
        </section>
    );
};

export default LeftSection;
