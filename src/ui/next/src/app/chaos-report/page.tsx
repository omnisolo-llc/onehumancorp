"use client";

import React, { useState, useEffect } from "react";

export default function ChaosReport() {
    const [theme, setTheme] = useState("light");

    return (
        <div className={`min-h-screen p-8 ${theme === "dark" ? "bg-black text-white" : "bg-gray-50 text-gray-900"}`}>
            <h1 className="text-3xl font-bold mb-6 font-outfit">Chaos Engineering & Reliability Report</h1>
            <button onClick={() => setTheme(theme === "light" ? "dark" : "light")} className="mb-4 px-4 py-2 bg-blue-500 text-white rounded">Toggle Theme</button>
            <div className={`p-6 rounded-2xl shadow-xl transition-all duration-300 ${theme === "light" ? "bg-white/65 border border-white/40 backdrop-blur-[30px] saturate-[2.1]" : "bg-[#16161A]/70 border border-white/10 backdrop-blur-[30px] saturate-[2.1]"}`}>
                <h2 className="text-xl font-semibold mb-4">System Degradation Validation</h2>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
                    <div>
                        <h3 className="text-lg font-medium mb-2">Latency Histogram (API Calls)</h3>
                        <div className="w-full h-48 bg-gray-200/50 dark:bg-gray-800/50 rounded flex items-end p-2 gap-1 relative overflow-hidden border border-gray-300 dark:border-gray-700">
                             {/* Mock Histogram Bars */}
                             <div className="w-1/6 bg-blue-400/80 h-[20%] rounded-t"></div>
                             <div className="w-1/6 bg-blue-400/80 h-[40%] rounded-t"></div>
                             <div className="w-1/6 bg-blue-500/80 h-[80%] rounded-t"></div>
                             <div className="w-1/6 bg-blue-400/80 h-[60%] rounded-t"></div>
                             <div className="w-1/6 bg-orange-400/80 h-[30%] rounded-t"></div>
                             <div className="w-1/6 bg-red-500/80 h-[10%] rounded-t"></div>
                        </div>
                        <div className="flex justify-between text-xs mt-1 opacity-70">
                            <span>0ms</span>
                            <span>p50</span>
                            <span>p95</span>
                            <span>p99</span>
                            <span>>2s</span>
                        </div>
                    </div>
                    <div>
                        <h3 className="text-lg font-medium mb-2">Error Rate (Network Drops)</h3>
                        <div className="w-full h-48 bg-gray-200/50 dark:bg-gray-800/50 rounded relative border border-gray-300 dark:border-gray-700 overflow-hidden">
                             {/* Mock Line Graph */}
                             <svg viewBox="0 0 100 100" preserveAspectRatio="none" className="w-full h-full absolute inset-0">
                                <polyline points="0,90 20,85 40,88 60,60 80,65 100,50" fill="none" stroke="#ef4444" strokeWidth="2" />
                                <polyline points="0,90 20,85 40,88 60,60 80,65 100,50 100,100 0,100" fill="rgba(239, 68, 68, 0.2)" stroke="none" />
                             </svg>
                        </div>
                        <div className="flex justify-between text-xs mt-1 opacity-70">
                            <span>T-60m</span>
                            <span>T-30m</span>
                            <span>Now</span>
                        </div>
                    </div>
                </div>
                <div className="mt-8 grid grid-cols-1 sm:grid-cols-3 gap-4">
                     <div className="p-4 bg-white/40 dark:bg-white/5 rounded-lg border border-white/20 dark:border-white/5">
                        <div className="text-sm opacity-80">Cloud Success Rate</div>
                        <div className="text-2xl font-bold text-green-600 dark:text-green-400">99.9%</div>
                     </div>
                     <div className="p-4 bg-white/40 dark:bg-white/5 rounded-lg border border-white/20 dark:border-white/5">
                        <div className="text-sm opacity-80">Standalone Recoveries</div>
                        <div className="text-2xl font-bold text-blue-600 dark:text-blue-400">100%</div>
                     </div>
                     <div className="p-4 bg-white/40 dark:bg-white/5 rounded-lg border border-white/20 dark:border-white/5">
                        <div className="text-sm opacity-80">Sync Lag (ms)</div>
                        <div className="text-2xl font-bold text-orange-500 dark:text-orange-400">42ms</div>
                     </div>
                </div>
            </div>
        </div>
    );
}
