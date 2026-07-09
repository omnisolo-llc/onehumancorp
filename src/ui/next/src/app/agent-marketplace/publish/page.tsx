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
    <div className="min-h-screen bg-[#f4f6f8] p-8 font-outfit flex flex-col items-center">
      <div className="max-w-3xl w-full">
        <header className="mb-8">
          <h1 className="text-4xl font-bold text-[#18212f] mb-2">Publish New Agent</h1>
          <p className="text-xl text-gray-600">
            Add your custom pre-built agent to the Agent Marketplace. (AutoGPT Harness Mechanic)
          </p>
        </header>

        {error && (
          <div className="p-4 mb-8 bg-red-100 text-red-700 border border-red-200 rounded-[12px] animate-in fade-in slide-in-from-top-4">
            {error}
          </div>
        )}

        <form onSubmit={handleSubmit} className="glassmorphism p-8  border border-white/40 shadow-sm backdrop-blur-[30px] saturate-[210%] bg-white/65">
          <div className="mb-6">
            <label htmlFor="name" className="block text-[#18212f] font-semibold mb-2">Agent Name</label>
            <input
              id="name"
              type="text"
              required
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full px-4 py-3 rounded-[12px] bg-white/80 border border-gray-200 outline-none focus:ring-2 focus:ring-[#007aff] transition-shadow shadow-sm"
              placeholder="e.g. Content Writer"
            />
          </div>

          <div className="mb-6">
            <label htmlFor="description" className="block text-[#18212f] font-semibold mb-2">Description</label>
            <input
              id="description"
              type="text"
              required
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              className="w-full px-4 py-3 rounded-[12px] bg-white/80 border border-gray-200 outline-none focus:ring-2 focus:ring-[#007aff] transition-shadow shadow-sm"
              placeholder="e.g. Writes engaging blog posts."
            />
          </div>

          <div className="mb-6">
            <label htmlFor="role" className="block text-[#18212f] font-semibold mb-2">Role</label>
            <input
              id="role"
              type="text"
              required
              value={role}
              onChange={(e) => setRole(e.target.value)}
              className="w-full px-4 py-3 rounded-[12px] bg-white/80 border border-gray-200 outline-none focus:ring-2 focus:ring-[#007aff] transition-shadow shadow-sm"
              placeholder="e.g. Writer"
            />
          </div>

          <div className="mb-8">
            <label htmlFor="systemPrompt" className="block text-[#18212f] font-semibold mb-2">System Prompt</label>
            <textarea
              id="systemPrompt"
              required
              rows={4}
              value={systemPrompt}
              onChange={(e) => setSystemPrompt(e.target.value)}
              className="w-full px-4 py-3 rounded-[12px] bg-white/80 border border-gray-200 outline-none focus:ring-2 focus:ring-[#007aff] transition-shadow shadow-sm resize-y"
              placeholder="You are a helpful assistant..."
            />
          </div>

          <button
            type="submit"
            disabled={loading}
            className="w-full py-3 px-6 bg-[#007aff] hover:bg-[#005bb5] text-white font-semibold rounded-[12px] disabled:opacity-50 transition-colors shadow-sm focus:ring-4 focus:ring-blue-200"
          >
            {loading ? 'Publishing...' : 'Publish to Marketplace'}
          </button>
        </form>
      </div>
    </div>
  );
}
