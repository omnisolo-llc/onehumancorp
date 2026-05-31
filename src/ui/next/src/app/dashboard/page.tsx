"use client";

import React, { useEffect, useState } from "react";

export default function Dashboard() {
  const [currentMilestone, setCurrentMilestone] = useState<any>(null);
  const [showMilestoneModal, setShowMilestoneModal] = useState<boolean>(false);

  useEffect(() => {
    async function checkMilestones() {
      if (localStorage.getItem("10th_order_milestone_shown") === "true") return;
      try {
        const res = await fetch("/api/v1/growth/milestones/check");
        const data = await res.json();
        if (data && data.milestones) {
          const orderMilestone = data.milestones.find((m: any) => m.id === "3" && m.reached);
          if (orderMilestone) {
            setCurrentMilestone(orderMilestone);
            setShowMilestoneModal(true);
            localStorage.setItem("10th_order_milestone_shown", "true");
          }
        }
      } catch (e) {
        console.error("Failed to check milestones", e);
      }
    }
    checkMilestones();
  }, []);

  return (
    <div className="p-8 font-inter animate-fade-in text-[#1D1D1F] dark:text-[#F5F5F7] min-h-screen">
      <header className="mb-8 flex justify-between items-end border-b border-gray-200 dark:border-white/10 pb-4">
        <div>
          <h1 className="text-3xl font-bold font-outfit tracking-tight">Dashboard</h1>
          <p className="text-gray-500 dark:text-[#A1A1A6]">Business Snapshot</p>
        </div>
      </header>
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="p-6 rounded-[16px] bg-white/60 dark:bg-black/30 backdrop-blur-xl shadow-sm border border-white/50 dark:border-white/10 flex flex-col items-center justify-center min-h-[120px]">
          <h3 className="text-sm font-semibold uppercase text-gray-500 tracking-wider">Revenue Today</h3>
          <p className="text-3xl font-bold mt-2">$0.00</p>
        </div>
        <div className="p-6 rounded-[16px] bg-white/60 dark:bg-black/30 backdrop-blur-xl shadow-sm border border-white/50 dark:border-white/10 flex flex-col items-center justify-center min-h-[120px]">
          <h3 className="text-sm font-semibold uppercase text-gray-500 tracking-wider">Pending Orders</h3>
          <p className="text-3xl font-bold mt-2">0</p>
        </div>
      </div>
      {showMilestoneModal && currentMilestone && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm p-4">
          <div className="mac-glass-container w-full max-w-sm flex flex-col items-center p-8 text-center animate-scale-up border border-white/40 shadow-2xl relative overflow-hidden">
             <div className="absolute inset-0 bg-gradient-to-br from-[#0066FF]/20 to-transparent pointer-events-none"></div>
             <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-white mb-2 tracking-tight">You hit a milestone!</h2>
             <p className="text-[#1D1D1F]/80 dark:text-[#F5F5F7]/80 text-sm mb-6">{currentMilestone.description}</p>
             <button
                onClick={() => setShowMilestoneModal(false)}
                className="w-full bg-[#1D1D1F] dark:bg-[#F5F5F7] text-white dark:text-[#1D1D1F] font-bold py-3 rounded-[8px] hover:scale-[1.02] transition-transform shadow-md"
             >
                Awesome
             </button>
          </div>
        </div>
      )}
    </div>
  );
}
