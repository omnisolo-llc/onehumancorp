"use client";

import { useEffect, useState } from "react";
import { useParams } from "next/navigation";

interface Proposal {
  id: string;
  status: string;
  total_amount_cents: number;
  deposit_percentage: number;
  deposit_amount_cents: number;
  payment_link_url: string;
  line_items: any[];
}

export default function ProposalViewerPage() {
  const params = useParams();
  const id = params.id as string;
  const [proposal, setProposal] = useState<Proposal | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch(`/api/proposals/${id}`)
      .then((res) => res.json())
      .then((data) => {
        if (data.success) {
          setProposal(data.proposal);
        }
        setLoading(false);
      })
      .catch(() => setLoading(false));
  }, [id]);

  if (loading) return <div className="p-8 text-white">Loading...</div>;
  if (!proposal) return <div className="p-8 text-white">Proposal not found.</div>;

  return (
    <div className="min-h-screen bg-gray-900 flex flex-col items-center p-4">
      <div className="w-full max-w-md mt-10 flex flex-col gap-6" style={{
          backdropFilter: "blur(20px) saturate(200%)",
          backgroundColor: "rgba(255, 255, 255, 0.05)",
          border: "1px solid rgba(255, 255, 255, 0.1)",
          borderRadius: "16px",
          padding: "24px",
      }}>
        <h1 className="text-2xl text-white font-semibold">Your Proposal</h1>

        <div className="flex justify-between items-center bg-gray-800 p-4 rounded-lg">
           <span className="text-gray-400">Status</span>
           <span className="text-white font-medium">{proposal.status}</span>
        </div>

        <div className="flex flex-col gap-2">
          <h3 className="text-gray-400 uppercase text-xs tracking-wider">Scope of Work</h3>
          {proposal.line_items.map((item, idx) => (
             <div key={idx} className="flex justify-between text-white text-sm">
                <span>{item.description} (x{item.quantity})</span>
                <span>${(item.total_price_cents / 100).toFixed(2)}</span>
             </div>
          ))}
        </div>

        <div className="h-px bg-gray-700 w-full my-2"></div>

        <div className="flex justify-between items-center">
           <span className="text-gray-300">Total Estimated Cost</span>
           <span className="text-xl text-white font-bold">${(proposal.total_amount_cents / 100).toFixed(2)}</span>
        </div>

        {proposal.status !== 'Accepted' && proposal.payment_link_url && (
            <a
              href={proposal.payment_link_url}
              className="mt-6 p-4 rounded-lg bg-blue-600 text-white font-semibold text-center hover:bg-blue-700 w-full sticky bottom-4 shadow-lg"
            >
              Pay ${(proposal.deposit_amount_cents / 100).toFixed(2)} Deposit to Lock
            </a>
        )}
      </div>
    </div>
  );
}
