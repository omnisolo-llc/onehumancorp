"use client";

import { useEffect, useState } from "react";
import { useParams, useRouter } from "next/navigation";

interface ProposalLineItem {
  id: string;
  description: string;
  unit_price_cents: number;
  quantity: number;
}

interface Proposal {
  id: string;
  status: string;
  total_amount_cents: number;
  required_deposit_cents: number;
  line_items: ProposalLineItem[];
}

export default function ProposalReviewPage() {
  const params = useParams();
  const router = useRouter();
  const [proposal, setProposal] = useState<Proposal | null>(null);
  const [loading, setLoading] = useState(true);
  const [approving, setApproving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!params.id) return;
    fetchProposal(params.id as string);
  }, [params.id]);

  const fetchProposal = async (id: string) => {
    try {
      const res = await fetch(`/api/proposals/${id}`);
      if (!res.ok) throw new Error("Failed to fetch proposal");
      const data = await res.json();
      setProposal(data);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleApprove = async () => {
    if (!proposal) return;
    setApproving(true);
    try {
      const res = await fetch(`/api/proposals/${proposal.id}/approve`, {
        method: "PATCH",
      });
      if (!res.ok) throw new Error("Failed to approve proposal");
      // Simulating a successful deposit payment flow redirection or showing success
      alert("Proposal approved! Deposit requested.");
      fetchProposal(proposal.id);
    } catch (err: any) {
      alert(err.message);
    } finally {
      setApproving(false);
    }
  };

  if (loading) return <div className="p-4">Loading proposal...</div>;
  if (error) return <div className="p-4 text-red-500">Error: {error}</div>;
  if (!proposal) return <div className="p-4">Proposal not found.</div>;

  return (
    <div className="flex flex-col min-h-screen bg-gray-50/50 pb-24">
      {/* Top Split View: Context summary */}
      <div className="p-4 bg-white/70 backdrop-blur-md border-b border-black/5 shadow-sm sticky top-0 z-10">
        <h1 className="text-xl font-bold tracking-tight">Review Proposal</h1>
        <p className="text-sm text-gray-500 mt-1">Generated from client intake request.</p>
        <div className="mt-2 text-xs uppercase tracking-wider font-semibold text-indigo-600 bg-indigo-50 inline-block px-2 py-1 rounded">
          {proposal.status}
        </div>
      </div>

      {/* Bottom Split View: Line Items & Adjustments */}
      <div className="p-4 flex-1 overflow-y-auto space-y-3">
        <h2 className="text-lg font-semibold text-gray-800 mb-2">Line Items</h2>
        {proposal.line_items.map((item) => (
          <div
            key={item.id}
            className="bg-white/80 backdrop-blur-xl border border-gray-100 rounded-xl p-4 shadow-sm flex items-center justify-between min-h-[44px]"
          >
            <div>
              <p className="font-medium text-gray-900">{item.description}</p>
              <p className="text-sm text-gray-500">Qty: {item.quantity}</p>
            </div>
            <div className="text-right">
              <p className="font-semibold text-gray-900">
                ${(item.unit_price_cents / 100).toFixed(2)}
              </p>
            </div>
          </div>
        ))}

        <div className="mt-8 border-t border-gray-200 pt-4 flex justify-between items-center px-1">
          <p className="text-gray-600 font-medium">Total</p>
          <p className="text-xl font-bold">${(proposal.total_amount_cents / 100).toFixed(2)}</p>
        </div>
        <div className="flex justify-between items-center px-1 mt-1">
          <p className="text-gray-500 text-sm">Required Deposit</p>
          <p className="text-sm font-semibold text-indigo-600">${(proposal.required_deposit_cents / 100).toFixed(2)}</p>
        </div>
      </div>

      {/* Prominent sticky approval action */}
      {proposal.status !== "ACCEPTED" && (
        <div className="fixed bottom-0 left-0 right-0 p-4 bg-white/90 backdrop-blur-lg border-t border-gray-200 safe-area-pb">
          <button
            onClick={handleApprove}
            disabled={approving}
            className="w-full min-h-[50px] bg-indigo-600 active:bg-indigo-700 text-white font-semibold rounded-xl shadow-lg disabled:opacity-50 transition-colors"
          >
            {approving ? "Approving..." : "Approve & Send"}
          </button>
        </div>
      )}
    </div>
  );
}
