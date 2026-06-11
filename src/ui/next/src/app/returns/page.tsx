"use client";

import { useEffect, useState } from "react";
import { AppShell } from "../components/AppShell";

type ReturnRequest = {
  id: string;
  order_id: string;
  customer_name: string;
  item_name?: string;
  reason?: string;
  type?: "Refund" | "Exchange";
  amount: number;
  status: "pending" | "approved" | "rejected" | string;
  created_at?: string;
};

export default function ReturnsPage() {
  const [requests, setRequests] = useState<ReturnRequest[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [actionStatus, setActionStatus] = useState("");

  const fetchReturns = async () => {
    try {
      setLoading(true);
      const tenantId = typeof window !== "undefined" ? localStorage.getItem("tenant_id") || "default" : "default";
      const res = await fetch(`/api/v1/returns/requests?tenant_id=${tenantId}`);
      if (!res.ok) throw new Error("Failed to fetch returns");
      const data = await res.json();
      setRequests(data);
    } catch (e: any) {
      setError(e.message);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchReturns();
  }, []);

  const handleApprove = async (id: string) => {
    try {
      setActionStatus("Processing...");
      const res = await fetch(`/api/v1/returns/requests/${id}/approve`, {
        method: "POST",
      });
      if (!res.ok) throw new Error("Failed to approve");

      setActionStatus("Approved! Return label generated and refund processed.");
      setRequests((prev) => prev.filter((r) => r.id !== id));
      setTimeout(() => setActionStatus(""), 3000);
    } catch (e: any) {
      setActionStatus("Error approving return.");
      setTimeout(() => setActionStatus(""), 3000);
    }
  };

  return (
    <AppShell
      title="Returns & Exchanges"
      subtitle="Omnichannel return orchestration"
      statusItems={[
        { label: "Pending", value: String(requests.length), tone: requests.length > 0 ? "warn" : "good" },
      ]}
    >
      <div className="w-full max-w-[375px] mx-auto md:max-w-none">
        {actionStatus && <div className="mb-4 app-badge good" role="status">{actionStatus}</div>}
        {error && <div className="mb-4 app-badge bad" role="alert">{error}</div>}

        {loading ? (
          <div className="flex justify-center py-10">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
          </div>
        ) : requests.length === 0 ? (
          <div className="app-empty">
            <h3 className="text-lg font-bold text-gray-900 mb-2">All Clear!</h3>
            <p className="text-sm text-gray-600">No pending return or exchange requests.</p>
          </div>
        ) : (
          <div className="space-y-4">
            {requests.map((req) => (
              <div key={req.id} className="app-card flex flex-col gap-4 p-4 glassmorphism border border-blue-100 bg-blue-50/30 rounded-2xl shadow-sm">
                <div className="flex items-center gap-2 text-blue-800 font-semibold text-sm">
                   Return requested by {req.customer_name} for Order #{req.order_id}.
                </div>

                <div>
                   <p className="text-sm text-gray-700">Operations Agent has generated a return label and prepared a ${req.amount.toFixed(2)} refund.</p>
                </div>

                <div className="flex gap-3 mt-2">
                  <button
                    onClick={() => handleApprove(req.id)}
                    className="flex-1 py-3 px-4 rounded-xl font-bold text-sm bg-[#0066FF] text-white hover:bg-[#0052CC] shadow-md shadow-[#0066FF]/20 active:scale-[0.98] transition-all min-h-[44px]"
                  >
                    Approve
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </AppShell>
  );
}
