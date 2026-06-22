"use client";

import React, { useState, useEffect } from "react";
import Link from "next/link";
import { AppShell } from "../components/AppShell";
import { MorningBriefingCard } from "./../dashboard/MorningBriefingCard";

type Booking = {
  id: string;
  customer_name: string;
  product_title: string;
  start_time: string;
  end_time: string;
  status: string;
  payment_status?: string;
  ai_summary?: string;
};

export default function TodayOperationsDashboard() {
  const [appointments, setAppointments] = useState<Booking[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedAppointment, setSelectedAppointment] = useState<Booking | null>(null);
  const [tenantId, setTenantId] = useState<string>("e2e-tenant");

  useEffect(() => {
    const tId = localStorage.getItem("tenant_id") || "e2e-tenant";
    setTenantId(tId);

    const fetchBookings = async () => {
      try {
        const res = await fetch(`/api/ui/bookings?tenant_id=${tId}`);
        if (!res.ok) {
          throw new Error("Failed to load today's appointments");
        }
        const data = await res.json();

        if (Array.isArray(data)) {
            setAppointments(data);
        }
      } catch (err) {
        console.error(err);
        setError("Failed to load your schedule.");
      } finally {
        setLoading(false);
      }
    };
    fetchBookings();
  }, []);

  const getBookingState = (startTimeStr: string, endTimeStr: string) => {
    const now = new Date();
    const start = new Date(startTimeStr);
    const end = endTimeStr ? new Date(endTimeStr) : new Date(start.getTime() + 60 * 60 * 1000);

    if (end < now) return "past";
    if (start <= now && end >= now) return "current";
    return "future";
  };

  const closeDialog = () => setSelectedAppointment(null);

  return (
    <AppShell>
      <div className="min-h-screen font-inter" style={{ backgroundColor: "#F5F5F7" }}>
        <header className="px-4 py-4 flex items-center justify-between border-b" style={{ background: "rgba(255, 255, 255, 0.65)", backdropFilter: "blur(30px) saturate(210%)", borderBottom: "1px solid rgba(255, 255, 255, 0.4)", position: "sticky", top: 0, zIndex: 40 }}>
          <div className="flex items-center gap-3">
            <Link href="/dashboard" aria-label="Back to Dashboard" className="text-gray-500 hover:text-gray-900 transition-colors">
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" />
              </svg>
            </Link>
            <h1 className="text-xl md:text-2xl font-bold font-outfit" style={{ color: "#1D1D1F", letterSpacing: "-0.02em" }}>
              Today's Operations
            </h1>
          </div>
        </header>

        <main className="p-4 md:p-6 max-w-4xl mx-auto space-y-6">
          <MorningBriefingCard tenant={tenantId} />

          <section className="bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] rounded-[16px] shadow-sm p-4 md:p-6 border border-white/40">
            <h2 className="text-lg font-semibold font-outfit mb-4 text-[#1D1D1F]">Your Schedule</h2>

            {loading ? (
              <div className="flex justify-center p-8">
                <div className="w-6 h-6 border-2 border-gray-200 border-t-indigo-600 rounded-full animate-spin"></div>
              </div>
            ) : error ? (
              <div className="text-sm text-red-600 p-4 border border-red-100 bg-red-50 rounded-lg text-center">
                {error}
              </div>
            ) : appointments.length === 0 ? (
              <div className="text-sm text-gray-500 p-8 border border-gray-100 rounded-lg text-center bg-white/50">
                Your schedule is clear for today.
              </div>
            ) : (
              <div className="space-y-3 relative before:absolute before:inset-0 before:ml-4 md:before:ml-6 before:-translate-x-px before:h-full before:w-0.5 before:bg-gradient-to-b before:from-transparent before:via-gray-300 before:to-transparent">
                {appointments.map((apt) => {
                  const state = getBookingState(apt.start_time, apt.end_time);
                  const startDate = new Date(apt.start_time);
                  const timeString = startDate.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });

                  return (
                    <div
                      key={apt.id}
                      className="relative flex items-stretch gap-4 group cursor-pointer"
                      onClick={() => setSelectedAppointment(apt)}
                      data-testid={`appointment-card-${apt.id}`}
                    >
                      <div className="flex flex-col items-center z-10 pt-4">
                        <div className={`w-3 h-3 rounded-full border-2 ${
                            state === "past" ? "bg-gray-300 border-white" :
                            state === "current" ? "bg-green-500 border-green-200 shadow-[0_0_0_4px_rgba(34,197,94,0.2)]" :
                            "bg-blue-500 border-white"
                        }`} />
                      </div>
                      <div className={`flex-1 p-4 rounded-[12px] border transition-all ${
                          state === "past" ? "bg-gray-50/50 border-gray-100 opacity-70" :
                          state === "current" ? "bg-green-50/30 border-green-200 shadow-sm" :
                          "bg-white border-gray-100 hover:shadow-md hover:-translate-y-0.5"
                      }`}>
                        <div className="flex justify-between items-start mb-1">
                          <span className={`text-sm font-semibold ${state === 'past' ? 'text-gray-500' : 'text-indigo-600'}`}>
                            {timeString}
                          </span>
                          {apt.payment_status === "deposit_required" && (
                            <span className="text-[10px] uppercase tracking-wider font-bold bg-orange-100 text-orange-700 px-2 py-0.5 rounded-full">
                              Deposit Required
                            </span>
                          )}
                          {apt.payment_status === "paid" && (
                            <span className="text-[10px] uppercase tracking-wider font-bold bg-green-100 text-green-700 px-2 py-0.5 rounded-full">
                              Paid
                            </span>
                          )}
                        </div>
                        <h3 className={`text-lg font-bold leading-tight ${state === 'past' ? 'text-gray-700' : 'text-[#1D1D1F]'}`}>
                          {apt.product_title || "Service"}
                        </h3>
                        <p className="text-sm text-gray-500 mt-0.5">
                          with {apt.customer_name || "Customer"}
                        </p>
                      </div>
                    </div>
                  );
                })}
              </div>
            )}
          </section>
        </main>
      </div>

      {/* Detail Modal */}
      {selectedAppointment && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4" aria-modal="true" role="dialog">
          <div className="fixed inset-0 bg-black/40 backdrop-blur-sm transition-opacity" onClick={closeDialog}></div>
          <div className="relative bg-white w-full max-w-md rounded-[24px] shadow-2xl p-6 overflow-hidden flex flex-col transform transition-all">
            <button
              onClick={closeDialog}
              className="absolute top-4 right-4 w-8 h-8 flex items-center justify-center rounded-full bg-gray-100 hover:bg-gray-200 text-gray-600 transition-colors"
            >
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12"/></svg>
            </button>

            <div className="flex items-center gap-4 mb-6 pt-2">
              <div className="w-14 h-14 bg-gradient-to-br from-indigo-100 to-purple-100 rounded-full flex items-center justify-center text-xl font-bold text-indigo-700">
                {(selectedAppointment.customer_name || "C").charAt(0).toUpperCase()}
              </div>
              <div>
                <h2 className="text-xl font-bold text-[#1D1D1F] leading-tight">
                  {selectedAppointment.customer_name || "Customer"}
                </h2>
                <p className="text-sm text-gray-500">
                  {new Date(selectedAppointment.start_time).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" })} • {selectedAppointment.product_title}
                </p>
              </div>
            </div>

            <div className="bg-indigo-50/50 rounded-[12px] p-4 mb-6 border border-indigo-100">
              <div className="flex items-center gap-2 mb-2">
                <span className="text-indigo-500">✨</span>
                <h3 className="text-sm font-semibold text-indigo-900 uppercase tracking-wider">AI Summary</h3>
              </div>
              <p className="text-sm text-indigo-800 leading-relaxed" data-testid="appointment-ai-summary">
                {selectedAppointment.ai_summary || "No recent context or special requests noted for this appointment."}
              </p>
            </div>

            <div className="flex flex-col gap-3">
              <button className="w-full bg-[#0066FF] hover:bg-[#0052CC] text-white font-semibold py-3.5 px-4 rounded-[12px] transition-colors shadow-sm flex items-center justify-center gap-2">
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"/></svg>
                Message Client
              </button>

              <div className="flex gap-3">
                <button className="flex-1 bg-white border border-gray-300 hover:bg-gray-50 text-gray-700 font-medium py-3 px-4 rounded-[12px] transition-colors flex justify-center items-center">
                  Reschedule
                </button>
                <button className="flex-1 bg-white border border-gray-300 hover:bg-gray-50 text-gray-700 font-medium py-3 px-4 rounded-[12px] transition-colors flex justify-center items-center">
                  {selectedAppointment.payment_status === "paid" ? "View Receipt" : "Request Payment"}
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </AppShell>
  );
}