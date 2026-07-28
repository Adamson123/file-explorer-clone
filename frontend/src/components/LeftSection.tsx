import { ChevronRight, Pin } from "lucide-react";

const LeftSection = () => {
    return (
        // bg-[#1b1b1b]
        <section className="w-full max-w-75 pl-5 p-4.5 flex z-10 flex-col gap-8  bg-[#131212ef]">
            {/* <div className="bg-red-300 size-10 absolute top-10 left-6 z-[-10]"></div> */}
            {/* Quick Access */}
            <div className="gap-1 flex flex-col">
                <h2 className="text-gray-400 flex text-sm items-center gap-1.5">
                    <ChevronRight className="size-3.5" /> Quick Access
                </h2>
                <div className="pl-4 text-sm flex flex-col gap-3">
                    <div className="flex items-center gap-1.5 p-1">
                        {/* <Monitor className="size-3.5 stroke-3 text-gray-400" />{" "} */}
                        <img src="./assets/desktop.svg" className="size-5" />
                        <p>Desktop</p>
                    </div>
                    <div className="flex items-center gap-1.5 p-1">
                        {/* <File className="size-3.5 stroke-3 text-gray-400" />{" "} */}
                        <img
                            src="./assets/documents.svg"
                            className="size-4.5"
                        />
                        <p>Documents</p>
                    </div>
                    <div className="flex items-center gap-1.5 p-1">
                        {/* <Download className="size-3.5 stroke-3 text-gray-400" />{" "} */}
                        <img
                            src="./assets/downloads.svg"
                            className="size-4.5"
                        />
                        <p>Downloads</p>
                    </div>
                    <div className="flex items-center gap-1.5 p-1">
                        {/* <Image className="size-3.5 stroke-3 text-gray-400" />{" "} */}
                        <img src="./assets/pictures.svg" className="size-4.5" />
                        <p>Pictures</p>
                    </div>
                    <div className="flex items-center gap-1.5 p-1">
                        {/* <Music className="size-3.5 stroke-3 text-gray-400" />{" "} */}
                        <img src="./assets/music.svg" className="size-4.5" />
                        <p>Music</p>
                    </div>
                    <div className="flex items-center gap-1.5 p-1">
                        {/* <Video className="size-3.5 stroke-3 text-gray-400" />{" "} */}
                        <img src="./assets/video.svg" className="size-4.5" />
                        <p>Videos</p>
                    </div>
                </div>
            </div>
            {/* Pinned */}
            <div className="gap-1 flex flex-col">
                <h2 className="text-gray-400 flex items-center gap-1.5 text-sm">
                    <ChevronRight className="size-3.5" /> Pinned
                </h2>
                <div className="pl-4 text-xs flex flex-col gap-3">
                    <div className="flex items-center gap-1.5 p-1">
                        {/* <Folder className="size-3.5 stroke-3 text-gray-400" />{" "} */}
                        <img src="./assets/folder.svg" className="size-4.5" />
                        <p>Movies</p>
                        <Pin className="size-3.5 stroke-gray-400 fill-gray-400 ml-auto rotate-50" />
                    </div>
                    <div className="flex items-center gap-1.5 p-1">
                        {/* <Folder className="size-3.5 stroke-3 text-gray-400" />{" "} */}
                        <img src="./assets/folder.svg" className="size-4.5" />
                        <p>Projects</p>
                        <Pin className="size-3.5 stroke-gray-400 fill-gray-400 ml-auto rotate-50" />
                    </div>
                </div>
            </div>
            {/* This PC */}
            <div className="gap-1 flex flex-col">
                <h2 className="text-gray-400 flex items-center gap-1.5 text-sm">
                    <ChevronRight className="size-3.5" /> This PC
                </h2>
                <div className="pl-4 text-xs flex flex-col gap-3">
                    <div className="flex items-center gap-1.5 p-1">
                        {/* <Disc3 className="size-3.5 stroke-3 text-gray-400" />{" "} */}
                        <img src="./assets/harddisk.svg" className="size-4.5" />
                        <p>Local Disc (C:)</p>
                    </div>
                    <div className="flex items-center gap-1.5 p-1">
                        {/* <Disc3 className="size-3.5 stroke-3 text-gray-400" />{" "} */}
                        <img src="./assets/harddisk.svg" className="size-4.5" />
                        <p>Local Disc (D:)</p>
                    </div>
                </div>
            </div>
        </section>
    );
};

export default LeftSection;
