"use client";

import React, { useState } from "react";
import { useRouter } from "next/navigation";

export default function IncidentIntakePage() {
  const [description, setDescription] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);
  const router = useRouter();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!description.trim()) return;

    setIsSubmitting(true);
    try {
      let tenantId = localStorage.getItem("tenant_id");
      let userId = localStorage.getItem("user_id");

      const token = localStorage.getItem("token") || "";

      if (!tenantId) {
        tenantId = "default";
      }

      if (!userId) {
        userId = "default";
      }
      const res = await fetch("/api/v1/incidents", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "x-tenant-id": tenantId,
          "x-user-id": userId,
          "Authorization": `Bearer ${token}`
        },
        body: JSON.stringify({ description }),
      });

      if (res.ok) {
        // Redirect back to dashboard to see the feed
        router.push("/dashboard");
      } else {
        console.error("Failed to submit incident");
      }
    } catch (err) {
      console.error(err);
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#121212] flex items-center justify-center p-4">
      <div className="w-full max-w-md bg-white dark:bg-[#16161a] p-6 shadow-sm border border-gray-200 dark:border-white/10">
        <h1 className="text-2xl font-bold mb-4 text-[#1D1D1F] dark:text-[#F5F5F7]">Report Incident</h1>
        <p className="text-sm text-gray-500 mb-6">Describe what went wrong. The AI assistant will triage and propose a resolution.</p>
        <form onSubmit={handleSubmit} className="flex flex-col gap-4">
          <textarea
            className="w-full min-h-[120px] p-4 border border-gray-300 dark:border-gray-700 bg-transparent text-[#1D1D1F] dark:text-[#F5F5F7] focus:outline-none focus:ring-2 focus:ring-[#0066FF]"
            placeholder="e.g. Espresso machine down"
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            disabled={isSubmitting}
            data-testid="incident-description"
          />
          <button
            type="submit"
            disabled={isSubmitting || !description.trim()}
            className="w-full min-h-[44px] bg-[#0066FF] hover:bg-[#0052CC] text-white font-medium transition-colors disabled:opacity-50"
            data-testid="submit-incident"
          >
            {isSubmitting ? "Reporting..." : "Submit Incident"}
          </button>
        </form>
      </div>
    </div>
  );
}
