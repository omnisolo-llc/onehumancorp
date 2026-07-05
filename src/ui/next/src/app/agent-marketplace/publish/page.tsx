'use client';

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function PublishAgentPage() {
  const router = useRouter();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [role, setRole] = useState('');
  const [systemPrompt, setSystemPrompt] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);
    try {
      const res = await fetch('/api/agents/marketplace', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name,
          description,
          role,
          system_prompt: systemPrompt,
        }),
      });

      if (!res.ok) {
        throw new Error('Failed to publish agent');
      }

      const data = await res.json();
      if (data.error) {
        throw new Error(data.error);
      }

      router.push('/agent-marketplace');
    } catch (err: any) {
      setError(err.message || 'An error occurred while publishing the agent');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 p-8 font-outfit">
      <div className="max-w-3xl mx-auto">
        <header className="mb-8">
          <h1 className="text-4xl font-bold text-gray-900 mb-2">Publish New Agent</h1>
          <p className="text-xl text-gray-600">
            Add your custom pre-built agent to the Agent Marketplace. (AutoGPT Harness Mechanic)
          </p>
        </header>

        {error && (
          <div className="p-4 mb-8 bg-red-100 text-red-700 border border-red-200 rounded-xl">
            {error}
          </div>
        )}

        <form onSubmit={handleSubmit} className="bg-white backdrop-blur-[30px] saturate-[210%] border border-white/40 p-8 rounded-2xl shadow-sm">
          <div className="mb-6">
            <label htmlFor="name" className="block text-gray-700 font-bold mb-2">Agent Name</label>
            <input
              id="name"
              type="text"
              required
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full p-4 text-lg rounded-xl shadow-sm border outline-none"
              placeholder="e.g. Content Writer"
            />
          </div>

          <div className="mb-6">
            <label htmlFor="description" className="block text-gray-700 font-bold mb-2">Description</label>
            <input
              id="description"
              type="text"
              required
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              className="w-full p-4 text-lg rounded-xl shadow-sm border outline-none"
              placeholder="e.g. Writes engaging blog posts."
            />
          </div>

          <div className="mb-6">
            <label htmlFor="role" className="block text-gray-700 font-bold mb-2">Role</label>
            <input
              id="role"
              type="text"
              required
              value={role}
              onChange={(e) => setRole(e.target.value)}
              className="w-full p-4 text-lg rounded-xl shadow-sm border outline-none"
              placeholder="e.g. Writer"
            />
          </div>

          <div className="mb-8">
            <label htmlFor="systemPrompt" className="block text-gray-700 font-bold mb-2">System Prompt</label>
            <textarea
              id="systemPrompt"
              required
              rows={4}
              value={systemPrompt}
              onChange={(e) => setSystemPrompt(e.target.value)}
              className="w-full p-4 text-lg rounded-xl shadow-sm border outline-none"
              placeholder="You are a helpful assistant..."
            />
          </div>

          <button
            type="submit"
            disabled={loading}
            className="w-full py-4 px-6 bg-[#0071E3] hover:bg-blue-700 text-white font-bold rounded-xl disabled:bg-blue-300"
          >
            {loading ? 'Publishing...' : 'Publish to Marketplace'}
          </button>
        </form>
      </div>
    </div>
  );
}
