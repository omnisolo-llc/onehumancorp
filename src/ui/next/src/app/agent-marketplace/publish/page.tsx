'use client';

import React, { useState } from 'react';

export default function PublishAgentPage() {

  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [role, setRole] = useState('');
  const [systemPrompt, setSystemPrompt] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    // The current registry persists descriptor links, not role/prompt definitions.
    // Keep a programmatic or keyboard submit from sending an unsupported draft.
    e.preventDefault();
  };

  return (
    <div className="min-h-screen bg-[#f4f6f8] p-8 font-outfit flex flex-col items-center">
      <div className="max-w-3xl w-full">
        <header className="mb-8">
          <h1 className="text-4xl font-bold text-[#18212f] mb-2">Publish New Agent</h1>
          <p className="text-xl text-gray-600">
            Prepare agent details in this unsaved form.
          </p>
        </header>

        <p id="marketplace-publication-status" role="status" className="mb-6 rounded-lg border border-amber-200 bg-amber-50 p-4 text-amber-900">
          Full-agent publication is unavailable. Roles and system prompts cannot be saved by this catalogue. Your entries are kept only in this open form.
        </p>

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
            disabled aria-describedby="marketplace-publication-status"
            className="w-full py-3 px-6 bg-[#007aff] hover:bg-[#005bb5] text-white font-semibold rounded-[12px] disabled:opacity-50 transition-colors shadow-sm focus:ring-4 focus:ring-blue-200"
          >
            Publish to Marketplace
          </button>
        </form>
      </div>
    </div>
  );
}
