"use client";

import { useState, useEffect } from "react";
import { Play, Pause, CheckCircle2, ChevronRight, Phone } from "lucide-react";

export default function VoiceDispatchApprovalCard() {
  const [isPlaying, setIsPlaying] = useState(false);
  const [audioProgress, setAudioProgress] = useState(0);
  const [isApproved, setIsApproved] = useState(false);

  useEffect(() => {
    let interval: NodeJS.Timeout;
    if (isPlaying) {
      interval = setInterval(() => {
        setAudioProgress((prev) => {
          if (prev >= 100) {
            setIsPlaying(false);
            return 0;
          }
          return prev + 1;
        });
      }, 100);
    }
    return () => clearInterval(interval);
  }, [isPlaying]);

  return (
    <div className="min-h-screen bg-[#f5f5f7] dark:bg-black p-4 flex flex-col items-center pt-[10vh]">
      {/* Mobile Lock Screen Notification Simulation */}
      <div className="w-[375px] mb-8 bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 rounded-2xl p-4 shadow-xl text-black dark:text-[#F5F5F7]">
        <div className="flex items-center gap-3 mb-2">
          <div className="w-8 h-8 rounded-full bg-[#0066FF] flex items-center justify-center">
            <Phone className="w-4 h-4 text-white" />
          </div>
          <span className="font-semibold text-sm">OHC Voice Assistant</span>
        </div>
        <p className="text-sm">
          Carlos, new pipe repair request from Sarah for tomorrow 2PM. $50 deposit collected.
        </p>
        <div className="mt-3 flex items-center text-[#0066FF] text-sm font-semibold">
          Tap to approve <ChevronRight className="w-4 h-4" />
        </div>
      </div>

      {/* Main Approval Screen */}
      <div className="w-[375px] bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 rounded-2xl shadow-xl overflow-hidden text-black dark:text-[#F5F5F7] flex flex-col h-[600px]">
        {/* Header */}
        <div className="p-6 border-b border-white/40 dark:border-white/10 text-center">
          <h1 className="text-xl font-bold tracking-tight">Booking Proposal</h1>
          <p className="text-sm opacity-70 mt-1">Pending your approval</p>
        </div>

        <div className="p-6 flex-1 flex flex-col gap-6">
          {/* Audio Summary */}
          <div className="bg-white/50 dark:bg-black/50 rounded-xl p-4 border border-white/40 dark:border-white/10">
            <p className="text-xs font-semibold mb-3 uppercase tracking-wider opacity-70">AI Call Summary</p>
            <div className="flex items-center gap-4">
              <button
                onClick={() => setIsPlaying(!isPlaying)}
                className="w-10 h-10 rounded-full bg-[#0066FF] text-white flex items-center justify-center flex-shrink-0 transition-transform active:scale-95"
              >
                {isPlaying ? <Pause className="w-4 h-4" /> : <Play className="w-4 h-4" />}
              </button>
              <div className="flex-1">
                <div className="h-1.5 w-full bg-gray-200 dark:bg-gray-800 rounded-full overflow-hidden">
                  <div
                    className="h-full bg-[#0066FF] transition-all duration-100 ease-linear"
                    style={{ width: `${audioProgress}%` }}
                  />
                </div>
                <div className="flex justify-between mt-2 text-xs opacity-60">
                  <span>0:00</span>
                  <span>0:10</span>
                </div>
              </div>
            </div>
            <p className="text-sm mt-3 opacity-90 leading-relaxed italic">
              "Customer Sarah called needing a pipe repair. I quoted $150 and scheduled for Tomorrow at 2:00 PM. A $50 deposit link was sent via SMS."
            </p>
          </div>

          {/* Proposed Details */}
          <div className="space-y-4">
             <div className="flex justify-between items-center py-2 border-b border-white/20 dark:border-white/10">
                <span className="text-sm opacity-70">Service</span>
                <span className="font-medium text-sm">Pipe Repair</span>
             </div>
             <div className="flex justify-between items-center py-2 border-b border-white/20 dark:border-white/10">
                <span className="text-sm opacity-70">Customer</span>
                <span className="font-medium text-sm">Sarah Jenkins</span>
             </div>
             <div className="flex justify-between items-center py-2 border-b border-white/20 dark:border-white/10">
                <span className="text-sm opacity-70">Date & Time</span>
                <span className="font-medium text-sm">Tomorrow, 2:00 PM</span>
             </div>
             <div className="flex justify-between items-center py-2">
                <span className="text-sm opacity-70">Deposit Status</span>
                <span className="text-sm font-medium text-[#34C759] flex items-center gap-1">
                  <CheckCircle2 className="w-4 h-4" /> Paid ($50)
                </span>
             </div>
          </div>
        </div>

        {/* Action Button */}
        <div className="p-6">
          <button
            onClick={() => setIsApproved(true)}
            disabled={isApproved}
            className={`w-full h-12 rounded-lg font-semibold text-[15px] flex items-center justify-center gap-2 transition-all active:scale-[0.98] ${
              isApproved
                ? "bg-[#34C759] text-white cursor-not-allowed"
                : "bg-[#0066FF] text-white hover:bg-blue-600"
            }`}
          >
            {isApproved ? (
              <>
                <CheckCircle2 className="w-5 h-5" /> Confirmed & Scheduled
              </>
            ) : (
              "Approve Route & Send Confirmation"
            )}
          </button>
        </div>
      </div>
    </div>
  );
}
