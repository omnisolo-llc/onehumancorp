"use client";

import React, { useState } from "react";

export default function GustoPayrollPage() {
    const [isSyncing, setIsSyncing] = useState(false);

    const handleSync = async () => {
        setIsSyncing(true);
        try {
            await fetch("/api/payroll/sync", { method: "POST" });
        } catch (error) {
            console.error("Failed to sync payroll", error);
        } finally {
            setIsSyncing(false);
        }
    };

    return (
        <div className="p-8 max-w-4xl mx-auto rounded-2xl bg-white/65 backdrop-blur-[30px] shadow-sm border border-white/40">
            <h1 className="text-3xl font-bold mb-4 font-outfit text-[#1D1D1F]">Gusto Payroll Integration</h1>
            <p className="text-lg text-gray-700 font-inter mb-6">
                Manage your team's payroll and compliance seamlessly with Gusto.
            </p>

            <button
                id="sync-payroll-button"
                onClick={handleSync}
                disabled={isSyncing}
                className="bg-[#0071E3] hover:bg-[#0066FF] text-white font-medium py-3 px-6 rounded-lg shadow-sm transition-all duration-200"
            >
                {isSyncing ? "Syncing..." : "Sync OHC Hours to Gusto"}
            </button>
        </div>
    );
}
