export default function ChatSystemPage() {
    return (
        <div className="flex flex-col h-screen max-w-[375px] mx-auto bg-gray-50/50 backdrop-blur-md">
            <header className="p-4 border-b border-gray-200/50 bg-white/50 sticky top-0 z-10">
                <h1 className="text-xl font-semibold text-gray-900">Work Triage</h1>
            </header>
            <main className="flex-1 overflow-y-auto p-4 space-y-4">
                <div className="flex gap-3">
                    <div className="w-10 h-10 rounded-full bg-gray-200 flex-shrink-0" />
                    <div className="flex flex-col gap-1 flex-1">
                        <div className="bg-white/80 p-3 rounded-2xl rounded-tl-sm shadow-sm">
                            <p className="text-sm text-gray-800">Do you do vegan cakes?</p>
                        </div>
                        <span className="text-xs text-gray-500 ml-1">Instagram DM • Maya</span>
                    </div>
                </div>

                <div className="flex gap-3 flex-row-reverse">
                    <div className="w-10 h-10 rounded-full bg-blue-100 flex-shrink-0 flex items-center justify-center">
                        <span className="text-blue-600 text-sm font-medium">AI</span>
                    </div>
                    <div className="flex flex-col gap-1 flex-1 items-end">
                        <div className="bg-blue-50/80 p-3 rounded-2xl rounded-tr-sm shadow-sm border border-blue-100/50 relative group">
                            <div className="absolute -top-2 -right-2 bg-blue-500 text-white text-[10px] px-2 py-0.5 rounded-full font-medium shadow-sm">Draft</div>
                            <p className="text-sm text-gray-800">Yes, we absolutely do vegan cakes! 🌱 All of our custom designs can be made 100% plant-based. What kind of design were you thinking of?</p>
                        </div>
                        <button className="text-xs bg-blue-500 hover:bg-blue-600 text-white px-3 py-1.5 rounded-full font-medium shadow-sm transition-colors mt-1 active:scale-95 touch-manipulation min-w-[44px] min-h-[44px]">
                            Send Draft
                        </button>
                    </div>
                </div>
            </main>
            <footer className="p-4 bg-white/80 border-t border-gray-200/50 sticky bottom-0 z-10 backdrop-blur-md">
                <div className="flex gap-2">
                    <input type="text" placeholder="Type a message..." className="flex-1 bg-gray-100/50 border-none rounded-full px-4 py-2 text-sm focus:ring-2 focus:ring-blue-500/50 outline-none min-h-[44px]" />
                    <button className="bg-gray-900 text-white rounded-full px-4 font-medium text-sm hover:bg-gray-800 transition-colors touch-manipulation min-w-[44px] min-h-[44px]">
                        Send
                    </button>
                </div>
            </footer>
        </div>
    );
}
