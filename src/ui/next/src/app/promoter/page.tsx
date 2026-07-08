"use client";

import React, { useState, useEffect } from "react";

interface SocialPostProposal {
  id: string;
  tenant_id: string;
  product_id: string;
  content: string;
  image_url: string;
  seo_alt_text: string;
  seo_meta_description: string;
  status: string;
  created_at_unix: number;
}

export default function PromoterPage() {
  const [proposals, setProposals] = useState<SocialPostProposal[]>([]);
  const [selectedProposal, setSelectedProposal] = useState<SocialPostProposal | null>(null);

  useEffect(() => {
    async function fetchProposals() {
      try {
        const token = localStorage.getItem("token") || "";
        const res = await fetch("/api/v1/agents/approvals/pending", {
          headers: {
            "Authorization": `Bearer ${token}`
          }
        });
        if (res.ok) {
          const data = await res.json();
          // Filter out the approvals that are specifically for the promoter
          const promoterProposals = data.pending_approvals.filter((a: any) =>
            a.department === "Marketing" && a.action_type === "DraftForReview"
          ).map((a: any) => ({
            id: a.id,
            tenant_id: a.tenant_id,
            product_id: a.payload?.product_id || "unknown",
            content: a.payload?.draft_content || a.description || "",
            image_url: a.payload?.image_url || "",
            seo_alt_text: a.payload?.seo_alt_text || "",
            seo_meta_description: a.payload?.seo_meta_description || "",
            status: a.status,
            created_at_unix: new Date(a.created_at).getTime() / 1000
          }));
          setProposals(promoterProposals);
        }
      } catch (err) {
        console.error("Failed to fetch proposals", err);
      }
    }
    fetchProposals();
  }, []);


  async function handleApprove() {
    if (!selectedProposal) return;
    try {
      const token = localStorage.getItem("token") || "";
      const res = await fetch(`/api/v1/agents/approvals/${selectedProposal.id}/decide`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "Authorization": `Bearer ${token}`
        },
        body: JSON.stringify({ approved: true, edited_payload: null })
      });
      if (res.ok) {
        setProposals(prev => prev.filter(p => p.id !== selectedProposal.id));
        setSelectedProposal(null);
      }
    } catch (err) {
      console.error("Approval failed", err);
    }
  }

  async function handleDiscard() {
    if (!selectedProposal) return;
    try {
      const token = localStorage.getItem("token") || "";
      const res = await fetch(`/api/v1/agents/approvals/${selectedProposal.id}/decide`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "Authorization": `Bearer ${token}`
        },
        body: JSON.stringify({ approved: false, edited_payload: null })
      });
      if (res.ok) {
        setProposals(prev => prev.filter(p => p.id !== selectedProposal.id));
        setSelectedProposal(null);
      }
    } catch (err) {
      console.error("Discard failed", err);
    }
  }

  if (selectedProposal) {

    return (
      <div className="p-4 max-w-sm mx-auto">
        <button onClick={() => setSelectedProposal(null)} className="mb-4 text-blue-500">&larr; Back</button>
        <h2 className="text-xl font-bold mb-4">Promoter Proposal</h2>
        <div className="bg-white rounded shadow p-4">
            {selectedProposal.image_url && <img src={selectedProposal.image_url} alt={selectedProposal.seo_alt_text} className="w-full h-48 object-cover rounded mb-4" />}
            <p className="text-sm font-semibold text-gray-700">Caption:</p>
            <p className="text-gray-900 mb-4">{selectedProposal.content}</p>
            <details className="mb-4">
                <summary className="text-sm text-gray-500 cursor-pointer">SEO Details</summary>
                <div className="mt-2 text-sm text-gray-700">
                    <p><strong>Alt Text:</strong> {selectedProposal.seo_alt_text}</p>
                    <p><strong>Meta Description:</strong> {selectedProposal.seo_meta_description}</p>
                </div>
            </details>
            <div className="flex gap-2">
                <button className="flex-1 bg-blue-500 text-white py-2 rounded" onClick={handleApprove}>Approve & Publish</button>
                <button className="flex-1 bg-gray-200 text-gray-800 py-2 rounded">Edit</button>
                <button className="flex-1 bg-red-100 text-red-600 py-2 rounded" onClick={handleDiscard}>Discard</button>
            </div>
        </div>
      </div>
    );
  }

  return (
    <div className="p-4 max-w-sm mx-auto">
      <h1 className="text-2xl font-bold mb-4">The Promoter</h1>
      <div className="space-y-4">
        {proposals.length === 0 ? (
          <p className="text-gray-500 text-sm">No new proposals generated.</p>
        ) : (
          proposals.map((p) => (
            <div key={p.id} className="bg-white rounded shadow p-4 cursor-pointer" onClick={() => setSelectedProposal(p)}>
              <p className="font-semibold text-gray-800">New Product Detected!</p>
              <p className="text-sm text-gray-600 mt-1">The Promoter has generated a social post and SEO tags.</p>
            </div>
          ))
        )}
      </div>
    </div>
  );
}
