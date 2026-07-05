'use client';

import React, { useState, useEffect } from 'react';

type Agent = {
 id: string;
 name: string;
 description: string;
 author: string;
 version: string;
 endpoint: string;
};

export default function AgentMarketplacePage() {
 const [agents, setAgents] = useState<Agent[]>([]);
 const [query, setQuery] = useState('');
 const [loading, setLoading] = useState(false);
 const [error, setError] = useState<string | null>(null);
 const [installedAgents, setInstalledAgents] = useState<string[]>([]);
 const [toastMessage, setToastMessage] = useState<string | null>(null);

 const fetchAgents = async (searchQuery: string) => {
 setLoading(true);
 setError(null);
 try {
 const res = await fetch(`/api/agents/marketplace?q=${encodeURIComponent(searchQuery)}`);
 if (!res.ok) {
 throw new Error('Failed to fetch agents');
 }
 const data: Agent[] = await res.json();
 setAgents(data);
 } catch (err: any) {
 setError(err.message || 'An error occurred while fetching agents');
 } finally {
 setLoading(false);
 }
 };

 useEffect(() => {
 fetchAgents(query);
 }, [query]);

 return (
 <div className="min-h-screen bg-gray-50 p-8 font-outfit">
 <div className="max-w-6xl mx-auto">
 <header className="mb-8">
 <h1 className="text-4xl font-bold text-gray-900 mb-2">Agent Marketplace</h1>
 <p className="text-xl text-gray-600">
 Discover and install pre-built AI agents for your business. (AutoGPT Unique Harness Innovations)
 </p>
 </header>

 <div className="mb-8 relative">
 <input
 type="text"
 placeholder="Search for agents..."
 value={query}
 onChange={(e) => setQuery(e.target.value)}
 className="w-full p-4 pl-12 text-lg rounded-xl shadow-sm focus:ring-2 focus:ring-[#0066FF] focus:border-[#0066FF] outline-none transition-all bg-white backdrop-blur-[30px] saturate-[210%] border border-white/40"
 />
 <div className="absolute left-4 top-1/2 transform -translate-y-1/2 text-gray-400">
 🔍
 </div>
 </div>

 {error && (
 <div className="p-4 mb-8 bg-red-100 text-red-700 border border-red-200 rounded-xl">
 {error}
 </div>
 )}

 {loading ? (
 <div className="flex justify-center items-center h-64">
 <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-[#0066FF]"></div>
 </div>
 ) : (
 <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
 {agents.map((agent) => (
 <div
 key={agent.id}
 className="p-6 rounded-2xl shadow-sm hover:shadow-md transition-all flex flex-col bg-white backdrop-blur-[30px] saturate-[210%] border border-white/40"
>
 <div className="mb-4 flex-grow">
 <h3 className="text-2xl font-bold text-gray-900 mb-2">{agent.name}</h3>
 <p className="text-sm text-gray-500 mb-4">
 By <span className="font-medium text-gray-700">{agent.author}</span> • v{agent.version}
 </p>
 <p className="text-gray-600 leading-relaxed">{agent.description}</p>
 </div>
 <div className="mt-auto">
 <button
 onClick={() => {
   const isInstalled = installedAgents.includes(agent.id);
   if (!isInstalled) {
     setToastMessage(`Successfully installed ${agent.name}!`);
     setTimeout(() => setToastMessage(null), 3000);
   }
   setInstalledAgents((current) => current.includes(agent.id) ? current : [...current, agent.id]);
 }}
 aria-pressed={installedAgents.includes(agent.id)}
 className="w-full py-3 px-4 bg-[#0071E3] hover:bg-blue-700 text-white font-medium rounded-xl transition-colors focus:ring-4 focus:ring-blue-200"
>
 {installedAgents.includes(agent.id) ? 'Installed' : 'Install Agent'}
 </button>
 </div>
 </div>
 ))}
 {agents.length === 0 && (
 <div className="col-span-full flex flex-col items-center justify-center p-12 rounded-2xl border border-dashed text-gray-500 bg-white backdrop-blur-[30px] saturate-[210%] border-white/40">
 <span className="text-4xl mb-4">🤖</span>
 <p className="text-xl">No agents found matching "{query}"</p>
 </div>
 )}
 </div>
 )}

 {toastMessage && (
 <div className="fixed bottom-8 left-1/2 transform -translate-x-1/2 bg-white/80 backdrop-blur-[30px] saturate-[210%] shadow-lg rounded-full px-6 py-3 text-gray-900 border border-white/50 animate-in fade-in slide-in-from-bottom-4 flex items-center gap-2">
 <span className="text-[#34C759]">✓</span>
 <span className="font-medium">{toastMessage}</span>
 </div>
 )}

 </div>
 </div>
 );
}
