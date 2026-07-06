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
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch('/api/v1/growth/promoter/proposals')
      .then(res => res.json())
      .then(data => {
        if (Array.isArray(data)) {
          // Show pending proposals or whatever status we care about
          setProposals(data.filter((p: SocialPostProposal) => p.status === 'pending' || p.status === 'generated'));
        }
      })
      .catch(err => console.error(err))
      .finally(() => setLoading(false));
  }, []);

  const updateProposalStatus = async (id: string, status: string) => {
    try {
      const res = await fetch(`/api/v1/growth/promoter/proposals/${id}`, {
        method: 'PATCH',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ status })
      });
      if (res.ok) {
        setProposals(prev => prev.filter(p => p.id !== id));
        setSelectedProposal(null);
      }
    } catch (err) {
      console.error(err);
    }
  };

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
                <button onClick={() => updateProposalStatus(selectedProposal.id, 'approved')} className="flex-1 bg-blue-500 text-white py-2 rounded">Approve & Publish</button>
                <button className="flex-1 bg-gray-200 text-gray-800 py-2 rounded">Edit</button>
                <button onClick={() => updateProposalStatus(selectedProposal.id, 'discarded')} className="flex-1 bg-red-100 text-red-600 py-2 rounded">Discard</button>
            </div>
        </div>
      </div>
    );
  }

  return (
    <div className="p-4 max-w-sm mx-auto">
      <h1 className="text-2xl font-bold mb-4">The Promoter</h1>
      <div className="space-y-4">
        {loading ? (
          <p className="text-gray-500 text-sm">Loading proposals...</p>
        ) : proposals.length === 0 ? (
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
