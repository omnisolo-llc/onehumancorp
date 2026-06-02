"use client";

import React, { useState } from "react";
import { WithTooltip } from "../../components/TooltipRegistry";

interface DeliveryJob {
  id: string;
  pickupLocation: string;
  dropoffLocation: string;
  distanceMiles: number;
  payout: number;
  status: "AVAILABLE" | "CLAIMED" | "PICKED_UP" | "DELIVERED";
}

export default function CourierPage() {
  const [jobs, setJobs] = useState<DeliveryJob[]>([
    {
      id: "job_1",
      pickupLocation: "Maya's Bakery",
      dropoffLocation: "123 Main St",
      distanceMiles: 2.1,
      payout: 7.0,
      status: "AVAILABLE",
    },
    {
      id: "job_2",
      pickupLocation: "Carlos Repairs",
      dropoffLocation: "456 Oak Ave",
      distanceMiles: 3.5,
      payout: 10.5,
      status: "AVAILABLE",
    },
  ]);

  const [activeJob, setActiveJob] = useState<DeliveryJob | null>(null);

  const claimJob = (id: string) => {
    setJobs(
      jobs.map((job) =>
        job.id === id ? { ...job, status: "CLAIMED" } : job
      )
    );
    const claimed = jobs.find((j) => j.id === id);
    if (claimed) {
      setActiveJob({ ...claimed, status: "CLAIMED" });
    }
  };

  const updateActiveJobStatus = (newStatus: "PICKED_UP" | "DELIVERED") => {
    if (activeJob) {
      setActiveJob({ ...activeJob, status: newStatus });
      setJobs(
        jobs.map((job) =>
          job.id === activeJob.id ? { ...job, status: newStatus } : job
        )
      );

      if (newStatus === "DELIVERED") {
        setTimeout(() => {
          setActiveJob(null);
        }, 3000);
      }
    }
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7]">
      <header className="px-6 py-4 flex items-center justify-between border-b bg-white/65 backdrop-blur-[30px] border-white/40 sticky top-0 z-50">
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">
          Courier Jobs
        </h1>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-lg mx-auto w-full flex flex-col gap-6">
        {activeJob ? (
          <div className="bg-white p-6 rounded-2xl shadow-sm border border-gray-100 flex flex-col gap-4">
            <h2 className="text-xl font-bold font-outfit text-indigo-600">
              Active Job
            </h2>
            <div className="flex justify-between items-center pb-2 border-b">
              <span className="text-gray-500 font-semibold text-sm uppercase tracking-wider">
                Payout
              </span>
              <span className="text-xl font-bold text-green-600">
                ${activeJob.payout.toFixed(2)}
              </span>
            </div>
            <div className="flex flex-col gap-2">
              <div className="flex items-start gap-3">
                <div className="w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center text-blue-600 shrink-0 font-bold">
                  A
                </div>
                <div>
                  <p className="text-sm font-semibold text-gray-800">Pickup</p>
                  <p className="text-sm text-gray-600">
                    {activeJob.pickupLocation}
                  </p>
                </div>
              </div>
              <div className="flex items-start gap-3">
                <div className="w-8 h-8 rounded-full bg-orange-100 flex items-center justify-center text-orange-600 shrink-0 font-bold">
                  B
                </div>
                <div>
                  <p className="text-sm font-semibold text-gray-800">Dropoff</p>
                  <p className="text-sm text-gray-600">
                    {activeJob.dropoffLocation}
                  </p>
                </div>
              </div>
            </div>

            <div className="mt-4 flex flex-col gap-3">
              {activeJob.status === "CLAIMED" && (
                <button
                  onClick={() => updateActiveJobStatus("PICKED_UP")}
                  className="w-full py-3 rounded-xl bg-blue-600 text-white font-semibold hover:bg-blue-700 transition"
                  id="mark-picked-up"
                >
                  Mark as Picked Up
                </button>
              )}
              {activeJob.status === "PICKED_UP" && (
                <button
                  onClick={() => updateActiveJobStatus("DELIVERED")}
                  className="w-full py-3 rounded-xl bg-green-600 text-white font-semibold hover:bg-green-700 transition"
                  id="mark-delivered"
                >
                  Complete Delivery (Photo Proof)
                </button>
              )}
              {activeJob.status === "DELIVERED" && (
                <div className="w-full py-3 rounded-xl bg-green-100 text-green-700 font-semibold text-center border border-green-200">
                  🎉 Job Completed! Earned ${activeJob.payout.toFixed(2)}
                </div>
              )}
            </div>
          </div>
        ) : (
          <>
            <h2 className="text-lg font-semibold text-gray-800" id="available-jobs-header">Available Jobs</h2>
            <div className="flex flex-col gap-4">
              {jobs
                .filter((j) => j.status === "AVAILABLE")
                .map((job) => (
                  <div
                    key={job.id}
                    className="bg-white p-5 rounded-2xl shadow-sm border border-gray-100 flex flex-col gap-3"
                  >
                    <div className="flex justify-between items-start">
                      <div>
                        <p className="font-semibold text-gray-900 line-clamp-1">
                          {job.pickupLocation} <span className="text-gray-400 font-normal mx-1">→</span> {job.dropoffLocation}
                        </p>
                        <p className="text-sm text-gray-500 mt-1">
                          {job.distanceMiles} miles total
                        </p>
                      </div>
                      <span className="bg-green-50 text-green-700 px-3 py-1 rounded-full text-sm font-bold border border-green-100">
                        ${job.payout.toFixed(2)}
                      </span>
                    </div>
                    <button
                      onClick={() => claimJob(job.id)}
                      className="w-full mt-2 py-2.5 rounded-xl bg-indigo-50 text-indigo-700 font-semibold border border-indigo-100 hover:bg-indigo-100 transition"
                      data-testid={`claim-${job.id}`}
                    >
                      Claim Job
                    </button>
                  </div>
                ))}
              {jobs.filter((j) => j.status === "AVAILABLE").length === 0 && (
                <div className="text-center p-8 bg-white rounded-2xl border border-gray-100 text-gray-500">
                  No jobs currently available in your area.
                </div>
              )}
            </div>
          </>
        )}
      </main>
      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}