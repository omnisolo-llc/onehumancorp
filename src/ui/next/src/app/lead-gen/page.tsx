"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { AppShell } from "../components/AppShell";

export default function LeadGenPage() {
  const router = useRouter();
  const [budget, setBudget] = useState("");
  const [zipCode, setZipCode] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError("");

    try {
      const res = await fetch("/api/v1/growth/campaign/start-lead-gen", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          budget: parseInt(budget, 10),
          service_radius: zipCode,
        }),
      });

      if (!res.ok) {
        throw new Error("Failed to start campaign");
      }

      router.push("/dashboard?lead_gen_started=1");
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <AppShell
      title="Hyperlocal Lead Generation"
      subtitle="Find new customers near you automatically"
    >
      <div className="max-w-md mx-auto mt-8">
        <div className="glassmorphism p-6 rounded-2xl border border-white/20">
          <h2 className="text-xl font-bold font-outfit mb-4">Start Finding Jobs</h2>

          {error && <div className="text-red-500 mb-4 text-sm">{error}</div>}

          <form onSubmit={handleSubmit} className="space-y-4">
            <div>
              <label htmlFor="budget" className="block text-sm font-medium text-gray-700 mb-1">
                Weekly Budget ($)
              </label>
              <input
                id="budget"
                type="number"
                inputMode="numeric"
                pattern="[0-9]*"
                required
                min="10"
                placeholder="e.g. 50"
                value={budget}
                onChange={(e) => setBudget(e.target.value)}
                className="w-full px-4 py-2 rounded-lg border border-gray-200 focus:ring-2 focus:ring-blue-500 outline-none"
              />
            </div>

            <div>
              <label htmlFor="zipCode" className="block text-sm font-medium text-gray-700 mb-1">
                Target Zip Code / Radius
              </label>
              <input
                id="zipCode"
                type="text"
                required
                placeholder="e.g. 90210"
                value={zipCode}
                onChange={(e) => setZipCode(e.target.value)}
                className="w-full px-4 py-2 rounded-lg border border-gray-200 focus:ring-2 focus:ring-blue-500 outline-none"
              />
            </div>

            <button
              type="submit"
              disabled={loading}
              className="w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-3 rounded-xl transition-colors disabled:opacity-50"
            >
              {loading ? "Starting..." : "Start Finding Jobs"}
            </button>
          </form>
        </div>
      </div>
    </AppShell>
  );
}
