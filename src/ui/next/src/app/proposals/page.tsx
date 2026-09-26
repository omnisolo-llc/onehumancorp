"use client";

import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import Head from "next/head";

type Proposal = {
  id: string;
  customer_id: string;
  status: string;
  total_amount_cents: number;
  project_scope: string | null;
  created_at: string;
};

export default function ProposalsDashboard() {
  const [proposals, setProposals] = useState<Proposal[]>([]);
  const [loading, setLoading] = useState(true);
  const router = useRouter();

  useEffect(() => {
    async function fetchProposals() {
      try {
        const res = await fetch("/api/v1/proposals");
        if (res.ok) {
          const data = await res.json();
          setProposals(data.proposals);
        }
      } catch (e) {
        console.error("Failed to fetch proposals", e);
      } finally {
        setLoading(false);
      }
    }
    fetchProposals();
  }, []);

  const handleApprove = async (id: string) => {
    try {
      const res = await fetch(`/api/v1/proposals/${id}/approve`, {
        method: "POST",
      });
      if (res.ok) {
        setProposals((prev) =>
          prev.map((p) => (p.id === id ? { ...p, status: "APPROVED" } : p))
        );
      }
    } catch (e) {
      console.error("Failed to approve proposal", e);
    }
  };

  return (
    <div className="flex flex-col p-4 max-w-[375px] mx-auto min-h-screen bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:bg-[rgba(22,22,26,0.7)] dark:border-[rgba(255,255,255,0.1)] transition-colors duration-250 ease-[cubic-bezier(0.4,0,0.2,1)]">
      <Head>
        <title>Inquiries & Proposals | OHC</title>
      </Head>
      <h1 className="text-xl font-bold text-gray-900 dark:text-gray-100 mb-6">
        Inquiries & Proposals
      </h1>

      {loading ? (
        <p className="text-gray-500 text-sm">Loading proposals...</p>
      ) : proposals.length === 0 ? (
        <p className="text-gray-500 text-sm">No draft proposals found.</p>
      ) : (
        <div className="flex flex-col gap-4">
          {proposals.map((proposal) => (
            <div
              key={proposal.id}
              className="flex flex-col p-4 bg-white dark:bg-gray-800 rounded-[16px] shadow-sm border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] gap-3"
            >
              <div className="flex justify-between items-start">
                <span className="text-sm font-semibold text-gray-900 dark:text-gray-100">
                  Client: {proposal.customer_id}
                </span>
                <span
                  className={`text-xs px-2 py-1 rounded-[8px] font-medium ${
                    proposal.status === "DRAFT" || proposal.status === "NEEDS_REVIEW"
                      ? "bg-yellow-100 text-yellow-800"
                      : "bg-green-100 text-green-800"
                  }`}
                >
                  {proposal.status}
                </span>
              </div>

              <div className="text-sm text-gray-600 dark:text-gray-400">
                <strong>Scope:</strong> {proposal.project_scope || "N/A"}
              </div>

              <div className="text-sm text-gray-600 dark:text-gray-400">
                <strong>Total:</strong> ${(proposal.total_amount_cents / 100).toFixed(2)}
              </div>

              {(proposal.status === "DRAFT" || proposal.status === "NEEDS_REVIEW") && (
                <div className="flex gap-2 mt-2">
                  <button
                    onClick={() => router.push(`/proposals/${proposal.id}`)}
                    className="flex-1 px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-[8px] hover:bg-gray-200 transition-colors duration-150 ease-[cubic-bezier(0.4,0,0.2,1)]"
                  >
                    Review
                  </button>
                  <button
                    onClick={() => handleApprove(proposal.id)}
                    className="flex-1 px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-[8px] hover:bg-blue-700 transition-colors duration-150 ease-[cubic-bezier(0.4,0,0.2,1)]"
                  >
                    Approve
                  </button>
                </div>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
