"use client";

import { useState } from "react";

export default function NewProposalPage() {
  const [topic, setTopic] = useState("");
  const [loading, setLoading] = useState(false);
  const [proposal, setProposal] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const generateProposal = async () => {
    if (!topic.trim()) return;
    setLoading(true);
    setProposal(null);
    setError(null);
    try {
      const res = await fetch("/api/v1/proposals/draft", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ topic }),
      });
      if (!res.ok) throw new Error("Proposal request failed");
      const data = (await res.json()) as { proposal?: unknown };
      if (typeof data.proposal !== "string" || !data.proposal.trim()) {
        throw new Error("Proposal response was invalid");
      }
      setProposal(data.proposal);
    } catch {
      setError("Failed to draft proposal");
    } finally {
      setLoading(false);
    }
  };

  return (
    <main className="mx-auto max-w-4xl p-8 font-sans">
      <h1 className="text-3xl font-bold mb-4">AI Proposal Generator</h1>
      <p className="text-gray-600 mb-8">
        Using GPT Researcher Planner + Execution mechanics to automatically draft a comprehensive multi-section proposal.
      </p>

      <div className="mb-6">
        <label
          htmlFor="proposal-topic"
          className="mb-2 block text-sm font-medium text-gray-700"
        >
          Project Brief / Topic
        </label>
        <textarea
          id="proposal-topic"
          className="w-full rounded-md border border-gray-300 p-3 shadow-sm"
          rows={4}
          value={topic}
          onChange={(e) => setTopic(e.target.value)}
          placeholder="e.g. Website redesign for a local bakery..."
        ></textarea>
      </div>

      <button
        type="button"
        onClick={generateProposal}
        disabled={loading}
        className="bg-indigo-600 text-white px-6 py-2 rounded shadow hover:bg-indigo-700 disabled:opacity-50"
      >
        {loading ? "Drafting..." : "Generate Proposal"}
      </button>

      {error ? (
        <p className="mt-6 text-sm font-medium text-red-700" role="alert">
          {error}
        </p>
      ) : null}

      {proposal && (
        <section className="mt-12 rounded border border-gray-200 bg-white/65 p-6 shadow-sm backdrop-blur-[30px] backdrop-saturate-[2.1]">
          <h2 className="text-xl font-semibold mb-4">Generated Draft</h2>
          <div className="whitespace-pre-wrap font-serif leading-relaxed text-gray-800">
            {proposal}
          </div>
        </section>
      )}
    </main>
  );
}
