"use client";

import { useState } from "react";

export default function NewProposalPage() {
  const [topic, setTopic] = useState("");
  const [loading, setLoading] = useState(false);
  const [proposal, setProposal] = useState<string | null>(null);

  const generateProposal = async () => {
    if (!topic) return;
    setLoading(true);
    setProposal(null);
    try {
      const res = await fetch("/api/proposals/draft", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ topic })
      });
      const data = await res.json();
      setProposal(data.proposal);
    } catch (e) {
      console.error(e);
      setProposal("Failed to draft proposal");
    }
    setLoading(false);
  };

  return (
    <div className="p-8 max-w-4xl mx-auto font-sans">
      <h1 className="text-3xl font-bold mb-4">AI Proposal Generator</h1>
      <p className="text-gray-600 mb-8">
        Using GPT Researcher Planner + Execution mechanics to automatically draft a comprehensive multi-section proposal.
      </p>

      <div className="mb-6">
        <label className="block text-sm font-medium text-gray-700 mb-2">Project Brief / Topic</label>
        <textarea
          className="w-full border border-gray-300 rounded-md p-3 shadow-sm"
          rows={4}
          value={topic}
          onChange={(e) => setTopic(e.target.value)}
          placeholder="e.g. Website redesign for a local bakery..."
        ></textarea>
      </div>

      <button
        onClick={generateProposal}
        disabled={loading}
        className="bg-indigo-600 text-white px-6 py-2 rounded shadow hover:bg-indigo-700 disabled:opacity-50"
      >
        {loading ? "Drafting..." : "Generate Proposal"}
      </button>

      {proposal && (
        <div className="mt-12 p-6 border border-gray-200 rounded bg-white backdrop-blur-[30px] backdrop-saturate-[2.1] shadow-sm-sm">
          <h2 className="text-xl font-semibold mb-4">Generated Draft</h2>
          <div className="whitespace-pre-wrap font-serif leading-relaxed text-gray-800">
            {proposal}
          </div>
        </div>
      )}
    </div>
  );
}
