import { ArrowLeft, ArrowRight, Minus, Square, X } from "lucide-react";

const TitleBar = () => {
    return (
        <section
            move-window="true"
            className="bg-primary py-3 px-4 font-semibold cursor-default flex items-center justify-between"
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
                <Minus className="text-gray-400 size-4" />
                <Square className="text-gray-400 size-3" />
                <X className="text-gray-400 size-4" />
            </div>
        </section>
    );
};

export default TitleBar;
