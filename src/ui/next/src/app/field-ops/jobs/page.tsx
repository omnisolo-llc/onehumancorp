"use client";

import React, { useState, useEffect } from "react";
import { SyncManager } from "../../../lib/sync/SyncManager";
import { useQuery } from "@powersync/react";
import { PowerSyncProvider } from "../../../lib/powersync/PowerSyncProvider";

type Appointment = {
  id: string;
  customer_id: string;
  customer_name: string;
  job_template_id: string;
  job_name: string;
  status: string;
  scheduled_start_time: string;
  scheduled_end_time: string;
  location_address: string;
  notes: string;
  actual_start_time?: string;
  actual_end_time?: string;
};

function FieldOpsJobsPageContent() {
  const [isOffline, setIsOffline] = useState(false);
  const [jobs, setJobs] = useState<Appointment[]>([]);
  const { data: offlineJobs } = useQuery<Appointment>('SELECT * FROM appointments ORDER BY scheduled_start_time ASC');
  const [agentSuggestion, setAgentSuggestion] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [delayAction, setDelayAction] = useState<{
    jobId: string;
    subsequentCount: number;
  } | null>(null);
  const [delayingJobId, setDelayingJobId] = useState<string | null>(null);
  const [proposedRoute, setProposedRoute] = useState<Appointment[] | null>(
    null,
  );

  const [voiceQuoteJobId, setVoiceQuoteJobId] = useState<string | null>(null);
  const [voiceTranscript, setVoiceTranscript] = useState("");
  const [draftingQuote, setDraftingQuote] = useState(false);
  const [draftQuoteResult, setDraftQuoteResult] = useState<any | null>(null);


  useEffect(() => {
    if (isOffline && offlineJobs && offlineJobs.length > 0) {
       setJobs(offlineJobs);
    }
  }, [isOffline, offlineJobs]);

  useEffect(() => {
    setIsOffline(!navigator.onLine);
    const handleOnline = () => setIsOffline(false);
    const handleOffline = () => setIsOffline(true);
    window.addEventListener("online", handleOnline);
    window.addEventListener("offline", handleOffline);

    // Fetch initial schedule
    if (navigator.onLine) {
      fetch("/api/v1/field-ops/appointments")
        .then((res) => res.json())
        .then(async (data) => {
          if (data.appointments) {
            setJobs(data.appointments);
            // Sync to local DB
            try {
               const { getPowerSyncDB } = await import('../../../lib/powersync/db');
               const db = await getPowerSyncDB();
               await db.execute('DELETE FROM appointments');
               for (const appt of data.appointments) {
                  await db.execute('INSERT INTO appointments (id, tenant_id, customer_id, customer_name, job_template_id, job_name, status, scheduled_start_time, scheduled_end_time, location_address, notes, actual_start_time, actual_end_time) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)', [
                      appt.id, 'default', appt.customer_id, appt.customer_name, appt.job_template_id, appt.job_name, appt.status, appt.scheduled_start_time, appt.scheduled_end_time, appt.location_address, appt.notes || '', appt.actual_start_time || null, appt.actual_end_time || null
                  ]);
               }
            } catch (e) {
               console.error("Failed to sync to local DB", e);
            }
          }
          setLoading(false);
        })
        .catch((err) => {
          console.error("Failed to load appointments", err);
          setLoading(false);
        });
    } else {
        setLoading(false);
    }

    return () => {
      window.removeEventListener("online", handleOnline);
      window.removeEventListener("offline", handleOffline);
    };
  }, []);

  const handleStatusChange = async (jobId: string, newStatus: string) => {
    const now = new Date().toISOString();
    setJobs((currentJobs) =>
      currentJobs.map((j) => {
        if (j.id === jobId) {
          const updated = { ...j, status: newStatus };
          if (newStatus === "In-Progress") updated.actual_start_time = now;
          if (newStatus === "Completed") updated.actual_end_time = now;
          return updated;
        }
        return j;
      }),
    );

    const jobToUpdate = jobs.find(j => j.id === jobId);
    if (!jobToUpdate) return;

    const updatedJob = { ...jobToUpdate, status: newStatus };
    if (newStatus === "In-Progress") updatedJob.actual_start_time = now;
    if (newStatus === "Completed") updatedJob.actual_end_time = now;

    if (isOffline) {
      const eventId = crypto.randomUUID ? crypto.randomUUID() : Date.now().toString();
      await SyncManager.getInstance().enqueue({
        id: eventId,
        type: 'sync_event',
        payload: {
          id: eventId,
          entity_type: 'appointment',
          entity_id: updatedJob.id,
          action_type: 'UpdateStatus',
          base_version: 1, // simplified assumption for offline UI
          payload: {
            status: updatedJob.status,
            notes: updatedJob.notes,
            scheduled_start_time: updatedJob.scheduled_start_time,
            scheduled_end_time: updatedJob.scheduled_end_time,
          }
        },
        timestamp: Date.now()
      });
      return;
    }

    // Call optimize route endpoint after state change to simulate Operations Agent logic
    const updatedJobs = jobs.map((j) => {
      if (j.id === jobId) {
        return updatedJob;
      }
      return j;
    });

    if (newStatus === "Completed") {
      fetch("/api/v1/field-ops/optimize-route", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          tenantId: "default",
          appointments: updatedJobs,
          currentLocationLat: 0,
          currentLocationLng: 0,
        }),
      })
        .then((res) => res.json())
        .then((data) => {
          if (data.success) {
            setJobs(data.optimizedRoute);
            if (data.agentSuggestion) {
              setAgentSuggestion(data.agentSuggestion);
            }
          }
        })
        .catch((err) => console.error("Optimization failed", err));
    }
  };

  const handleNotesChange = (jobId: string, notes: string) => {
    setJobs(jobs.map((j) => (j.id === jobId ? { ...j, notes } : j)));
  };

  const handleRunningLate = (jobId: string) => {
    setDelayingJobId(jobId);

    fetch("/api/v1/field-ops/running-late", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ appointments: jobs, delayJobId: jobId }),
    })
      .then((res) => res.json())
      .then((data) => {
        if (data.success) {
          setDelayAction({
            jobId,
            subsequentCount: data.subsequentCount,
          });
          if (data.agentSuggestion) {
            setAgentSuggestion(data.agentSuggestion);
          }
          setProposedRoute(data.optimizedRoute);
        }
        setDelayingJobId(null);
      })
      .catch((err) => {
        console.error("Running late failed", err);
        setDelayingJobId(null);
      });
  };

  const handleApproveDelay = () => {
    if (proposedRoute) {
      setJobs(proposedRoute);

      Promise.all(
        proposedRoute.map((job) =>
          fetch("/api/v1/field-ops/appointments", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
              id: job.id,
              status: job.status,
              notes: job.notes,
              scheduled_start_time: job.scheduled_start_time,
              scheduled_end_time: job.scheduled_end_time,
            }),
          }),
        ),
      ).catch((err) =>
        console.error("Failed to persist updated schedule", err),
      );

      setProposedRoute(null);
    }
    setDelayAction(null);
    setAgentSuggestion(null);
  };

  const handleComplete = (jobId: string) => {
    const job = jobs.find((j) => j.id === jobId);
    if (!job) return;

    handleStatusChange(jobId, "Completed");

    // Queue invoice generating / tap-to-pay intent
    SyncManager.getInstance().enqueue({
      id: crypto.randomUUID ? crypto.randomUUID() : Date.now().toString(),
      type: "generate_invoice",
      payload: {
        job_id: jobId,
        customer_id: job.customer_id
      },
      timestamp: Date.now()
    });

    if (job.notes) {
      SyncManager.getInstance().enqueue({
        id: `mutation-${Date.now()}`,
        type: "draft_quote",
        notes: `Follow up quote requested by field op for job ${jobId}. Notes: ${job.notes}`,
      });
    }
  };

  const handleDraftVoiceQuote = async () => {
    if (!voiceQuoteJobId || !voiceTranscript.trim()) return;

    setDraftingQuote(true);
    setDraftQuoteResult(null);
    try {
      const job = jobs.find(j => j.id === voiceQuoteJobId);
      const res = await fetch("/api/quotes/draft_agent", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          inquiry: voiceTranscript,
          customer_id: job?.customer_id || "unknown",
          tenant_id: "default"
        })
      });

      if (!res.ok) throw new Error("Failed to draft quote");
      const data = await res.json();
      setDraftQuoteResult(data);
    } catch (err) {
      console.error(err);
      alert("Failed to draft quote.");
    } finally {
      setDraftingQuote(false);
    }
  };

  if (loading) {
    return (
      <div className="p-4 bg-gray-50 min-h-screen flex items-center justify-center">
        Loading schedule...
      </div>
    );
  }

  return (
    <div className="p-4 bg-gray-50 min-h-screen">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold font-outfit text-gray-900">
          Today's Route
        </h1>
        {isOffline && (
          <div className="flex items-center text-orange-600 bg-orange-50 px-3 py-1 rounded-full text-sm font-semibold">
            <span className="mr-2">☁️</span> Offline Mode
          </div>
        )}
      </div>

      {delayAction && (
        <div className="mb-6 p-4 bg-orange-50 border border-orange-200 rounded-xl shadow-sm relative">
          <button
            onClick={() => setDelayAction(null)}
            className="absolute top-2 right-2 text-gray-400 hover:text-gray-600 p-1"
          >
            ✕
          </button>
          <div className="flex gap-3">
            <div className="w-8 h-8 bg-orange-100 rounded-full flex items-center justify-center text-xl shrink-0">
              🤖
            </div>
            <div>
              <p className="text-sm font-medium text-gray-900 mb-2">
                Drafting delay notifications for the next{" "}
                {delayAction.subsequentCount} clients. Approve?
              </p>
              <div className="flex gap-2">
                <button
                  onClick={handleApproveDelay}
                  className="px-3 py-1.5 bg-orange-600 text-white text-xs font-semibold rounded-lg"
                >
                  Approve & Send
                </button>
                <button
                  onClick={() => setDelayAction(null)}
                  className="px-3 py-1.5 bg-white border border-gray-300 text-gray-700 text-xs font-semibold rounded-lg"
                >
                  Cancel
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

      {agentSuggestion && (
        <div className="mb-6 p-4 bg-blue-50 border border-blue-200 rounded-xl shadow-sm relative">
          <button
            onClick={() => setAgentSuggestion(null)}
            className="absolute top-2 right-2 text-gray-400 hover:text-gray-600 p-1"
          >
            ✕
          </button>
          <div className="flex gap-3">
            <div className="w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center text-xl shrink-0">
              🤖
            </div>
            <div>
              <p className="text-sm font-medium text-gray-900 mb-2">
                {agentSuggestion}
              </p>
              <div className="flex gap-2">
                <button
                  onClick={() => setAgentSuggestion(null)}
                  className="px-3 py-1.5 bg-[#0071E3] text-white text-xs font-semibold rounded-lg"
                >
                  Yes, text them
                </button>
                <button
                  onClick={() => setAgentSuggestion(null)}
                  className="px-3 py-1.5 bg-white border border-gray-300 text-gray-700 text-xs font-semibold rounded-lg"
                >
                  No, stick to schedule
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

      <div className="space-y-4">
        {jobs.map((job) => (
          <div
            key={job.id}
            className="bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] shadow-sm border border-white/40 overflow-hidden rounded-[16px]"
          >
            <div className="p-5 border-b border-gray-100 bg-gray-50/50">
              <div className="flex justify-between items-start mb-2">
                <h3 className="font-bold text-lg text-gray-900">
                  {job.customer_name}
                </h3>
                <span
                  className={`px-2 py-1 text-xs font-semibold rounded-full ${
                    job.status === "Completed"
                      ? "bg-green-100 text-green-700"
                      : job.status === "In-Progress"
                        ? "bg-yellow-100 text-yellow-700"
                        : job.status === "En-Route"
                          ? "bg-purple-100 text-purple-700"
                          : "bg-blue-100 text-blue-700"
                  }`}
                >
                  {job.status.toUpperCase()}
                </span>
              </div>
              <p className="text-gray-600 text-sm mb-1 flex items-center gap-2">
                📍 {job.location_address}
              </p>
              <p className="text-gray-600 text-sm flex items-center gap-2">
                🔧 {job.job_name}
              </p>
              <p className="text-gray-500 text-xs mt-2 font-medium">
                ⏱{" "}
                {new Date(job.scheduled_start_time).toLocaleTimeString([], {
                  hour: "2-digit",
                  minute: "2-digit",
                })}{" "}
                -{" "}
                {new Date(job.scheduled_end_time).toLocaleTimeString([], {
                  hour: "2-digit",
                  minute: "2-digit",
                })}
              </p>
            </div>

            {job.status !== "Completed" && (
              <div className="p-5">
                <div className="flex justify-between items-center mb-2">
                  <label className="block text-sm font-medium text-gray-700">
                    Service Notes & Potential Follow-ups
                  </label>
                  <button
                    onClick={() => setVoiceQuoteJobId(job.id)}
                    className="w-10 h-10 rounded-full bg-[#0066FF]/10 text-[#0066FF] hover:bg-[#0066FF]/20 flex items-center justify-center transition-colors"
                    title="Voice-to-Quote"
                    data-testid={`voice-quote-btn-${job.id}`}
                  >
                    🎤
                  </button>
                </div>
                <textarea
                  className="w-full border border-gray-300 rounded-lg p-3 text-sm focus:ring-2 focus:ring-[#0066FF] focus:border-transparent outline-none transition-shadow min-h-[80px]"
                  placeholder="E.g., Needs a replacement quote."
                  value={job.notes}
                  onChange={(e) => handleNotesChange(job.id, e.target.value)}
                />

                <div className="mt-4 flex flex-col gap-2 sm:flex-row sm:gap-3 flex-wrap">
                  {job.status === "Requested" || job.status === "Scheduled" ? (
                    <div className="flex w-full gap-2 flex-col sm:flex-row flex-wrap">
                      <button
                        className="flex-1 bg-purple-100 hover:bg-purple-200 text-purple-700 font-semibold py-3 rounded-xl transition-colors active:scale-[0.98] min-h-[44px] min-w-[44px]"
                        onClick={() => handleStatusChange(job.id, "En-Route")}
                      >
                        Heading to Job
                      </button>
                      <button
                        className="flex-1 bg-red-50 hover:bg-red-100 text-red-600 font-semibold py-3 rounded-xl transition-colors active:scale-[0.98] min-h-[44px] min-w-[44px]"
                        onClick={() => handleRunningLate(job.id)}
                        disabled={delayingJobId === job.id}
                      >
                        {delayingJobId === job.id
                          ? "Calculating..."
                          : "Running Late"}
                      </button>
                    </div>
                  ) : job.status === "En-Route" ? (
                    <div className="flex w-full gap-2 flex-col sm:flex-row flex-wrap">
                      <button
                        className="flex-1 bg-yellow-100 hover:bg-yellow-200 text-yellow-700 font-semibold py-3 rounded-xl transition-colors active:scale-[0.98] min-h-[44px] min-w-[44px]"
                        onClick={() =>
                          handleStatusChange(job.id, "In-Progress")
                        }
                      >
                        Start Work
                      </button>
                      <button
                        className="flex-1 bg-red-50 hover:bg-red-100 text-red-600 font-semibold py-3 rounded-xl transition-colors active:scale-[0.98] min-h-[44px] min-w-[44px]"
                        onClick={() => handleRunningLate(job.id)}
                        disabled={delayingJobId === job.id}
                      >
                        {delayingJobId === job.id
                          ? "Calculating..."
                          : "Running Late"}
                      </button>
                    </div>
                  ) : (
                    <div className="flex w-full gap-2 flex-col sm:flex-row flex-wrap">
                      <button
                        className="flex-1 bg-[#0071E3] hover:bg-blue-700 text-white font-semibold py-3 rounded-xl transition-colors active:scale-[0.98] min-h-[44px] min-w-[44px]"
                        onClick={() => handleComplete(job.id)}
                      >
                        Complete & Pay
                      </button>
                      <button
                        className="flex-1 bg-red-50 hover:bg-red-100 text-red-600 font-semibold py-3 rounded-xl transition-colors active:scale-[0.98] min-h-[44px] min-w-[44px]"
                        onClick={() => handleRunningLate(job.id)}
                        disabled={delayingJobId === job.id}
                      >
                        {delayingJobId === job.id
                          ? "Calculating..."
                          : "Running Late"}
                      </button>
                    </div>
                  )}
                </div>
              </div>
            )}
            {job.status === "Completed" && job.notes && (
              <div className="p-5 bg-blue-50/50">
                <p className="text-sm font-medium text-gray-800 mb-1">
                  Saved Notes:
                </p>
                <p className="text-sm text-gray-600 italic">"{job.notes}"</p>
                <p className="text-xs text-[#0071E3] font-semibold mt-2">
                  ✨ Sales Agent will draft an estimate based on these notes
                  once online.
                </p>
              </div>
            )}
          </div>
        ))}
      </div>

      {/* Voice-to-Quote Modal */}
      {voiceQuoteJobId && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/40 backdrop-blur-sm">
          <div className="bg-white/80 dark:bg-[#16161A]/80 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 rounded-[24px] shadow-2xl p-6 w-full max-w-md flex flex-col">
            <div className="flex justify-between items-center mb-4">
              <h3 className="text-lg font-bold font-outfit text-gray-900 dark:text-white flex items-center gap-2">
                <span>🎤</span> Voice-to-Quote
              </h3>
              <button
                onClick={() => {
                  setVoiceQuoteJobId(null);
                  setDraftQuoteResult(null);
                  setVoiceTranscript("");
                }}
                className="text-gray-500 hover:text-gray-800 dark:hover:text-white"
              >
                ✕
              </button>
            </div>

            {!draftQuoteResult ? (
              <>
                <p className="text-sm text-gray-600 dark:text-gray-300 mb-3">
                  Speak your notes and the Sales Assistant will draft a professional quote. (Simulated)
                </p>
                <textarea
                  data-testid="voice-transcript-input"
                  className="w-full border border-gray-300 dark:border-gray-700 rounded-lg p-3 text-sm focus:ring-2 focus:ring-[#0066FF] outline-none min-h-[100px] mb-4 bg-white/50 dark:bg-black/20 text-gray-900 dark:text-white"
                  placeholder="e.g. Needs 2 hours labor for pipe repair, $50 in parts..."
                  value={voiceTranscript}
                  onChange={e => setVoiceTranscript(e.target.value)}
                />
                <button
                  data-testid="generate-quote-btn"
                  onClick={handleDraftVoiceQuote}
                  disabled={draftingQuote || !voiceTranscript.trim()}
                  className="w-full bg-[#0066FF] text-white font-semibold py-3 disabled:opacity-50 min-h-[44px]"
                >
                  {draftingQuote ? "Drafting..." : "Generate Draft Quote"}
                </button>
              </>
            ) : (
              <div data-testid="draft-quote-result" className="flex flex-col gap-3">
                <div className="bg-green-100 dark:bg-green-900/30 text-green-800 dark:text-green-200 p-3 rounded-xl text-sm font-medium text-center">
                  ✨ Draft Quote Ready
                </div>

                {draftQuoteResult.quote && (
                  <div className="bg-white/50 dark:bg-black/20 p-4 rounded-xl border border-gray-200 dark:border-gray-700">
                     <p className="text-xs text-gray-500 dark:text-gray-400 mb-2">Calculated Total</p>
                     <p className="text-xl font-bold text-gray-900 dark:text-white mb-4">
                       ${((draftQuoteResult.quote.total_amount_cents || 0) / 100).toFixed(2)}
                     </p>
                     <div className="space-y-2">
                       {draftQuoteResult.line_items?.map((item: any, i: number) => (
                         <div key={i} className="flex justify-between text-sm">
                           <span className="text-gray-700 dark:text-gray-300">{item.description} (x{item.quantity})</span>
                           <span className="text-gray-900 dark:text-white font-medium">${((item.unit_price_cents || 0) / 100).toFixed(2)}</span>
                         </div>
                       ))}
                     </div>
                  </div>
                )}

                <div className="flex gap-2 mt-2">
                   <button
                     onClick={() => {
                        setVoiceQuoteJobId(null);
                        setDraftQuoteResult(null);
                        setVoiceTranscript("");
                     }}
                     className="flex-1 bg-[#0066FF] text-white font-semibold py-3 min-h-[44px]"
                   >
                     Approve & Send
                   </button>
                </div>
              </div>
            )}
          </div>
        </div>
      )}

    </div>
  );
}

export default function FieldOpsJobsPage() {
  return (
    <PowerSyncProvider>
      <FieldOpsJobsPageContent />
    </PowerSyncProvider>
  );
}
