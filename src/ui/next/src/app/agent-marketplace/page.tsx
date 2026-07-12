'use client';

import React, { useState, useEffect } from 'react';
import Link from 'next/link';

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
   <div className="min-h-screen w-full min-w-0 max-w-full bg-[#f4f6f8] p-8 font-outfit flex flex-col items-center">
     <div className="w-full min-w-0 max-w-6xl">
       <header className="mb-8 flex w-full min-w-0 flex-wrap items-end justify-between gap-4">
         <div className="min-w-0">
           <h1 className="text-4xl font-bold text-[#18212f] mb-2">Agent Marketplace</h1>
           <p className="text-xl text-gray-600">
             Discover and install pre-built AI agents for your business. (AutoGPT Unique Harness Innovations)
           </p>
         </div>
         <Link
           href="/agent-marketplace/publish"
           className="shrink-0 px-6 py-2.5 bg-white text-[#18212f] font-semibold border border-gray-200 rounded-[12px] shadow-sm hover:bg-gray-50 transition-colors"
         >
           Publish Agent
         </Link>
       </header>

       <div className="mb-8 relative w-full">
         <input
           type="text"
           placeholder="Search for agents..."
           value={query}
           onChange={(e) => setQuery(e.target.value)}
           className="w-full px-4 py-4 pl-12 text-lg rounded-[16px] bg-white/80 shadow-sm focus:ring-2 focus:ring-[#007aff] focus:border-[#007aff] outline-none transition-shadow border border-white/40 backdrop-blur-[30px] saturate-[210%]"
         />
         <div className="absolute left-4 top-1/2 transform -translate-y-1/2 text-gray-400">
           <svg width="20" height="20" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path></svg>
         </div>
       </div>

       {error && (
         <div className="p-4 mb-8 bg-red-100 text-red-700 border border-red-200 rounded-[12px] w-full text-center">
           {error}
         </div>
       )}

       {loading ? (
         <div className="flex justify-center items-center h-64 w-full">
           <div className="animate-spin rounded-full h-10 w-10 border-b-2 border-[#007aff]"></div>
         </div>
       ) : (
         <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 w-full">
           {agents.map((agent) => (
             <div
               key={agent.id}
               className="p-6 rounded-[16px] flex flex-col bg-white/65 border border-white/40 shadow-sm hover:shadow-md transition-all backdrop-blur-[30px] saturate-[210%]"
             >
               <div className="mb-4 flex-grow">
                 <h3 className="text-2xl font-bold text-[#18212f] mb-1">{agent.name}</h3>
                 <p className="text-xs text-gray-500 mb-4 font-semibold uppercase tracking-wider">
                   By {agent.author} • v{agent.version}
                 </p>
                 <p className="text-gray-600 leading-relaxed text-sm">{agent.description}</p>
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
                   className={`w-full py-2.5 px-4 font-semibold rounded-[10px] transition-colors focus:outline-none focus:ring-2 focus:ring-offset-1 ${installedAgents.includes(agent.id) ? 'bg-[#34c759]/10 text-[#34c759] focus:ring-[#34c759]' : 'bg-[#007aff] hover:bg-[#005bb5] text-white focus:ring-[#007aff] shadow-sm'}`}
                 >
                   {installedAgents.includes(agent.id) ? 'Installed' : 'Install Agent'}
                 </button>
               </div>
             </div>
           ))}
           {agents.length === 0 && (
             <div className="col-span-full flex flex-col items-center justify-center py-20 px-4 rounded-[16px] border border-dashed border-gray-300 text-gray-500 bg-white/50 backdrop-blur-[10px]">
               <svg className="w-12 h-12 mb-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path></svg>
               <p className="text-lg font-medium text-gray-900 mb-1">No agents found</p>
               <p className="text-sm text-gray-500">We couldn't find any agents matching "{query}"</p>
             </div>
           )}
         </div>
       )}

       {toastMessage && (
         <div className="fixed bottom-8 left-1/2 transform -translate-x-1/2 bg-[#18212f]/90 backdrop-blur-[20px] shadow-lg rounded-[100px] px-5 py-3 text-white animate-in fade-in slide-in-from-bottom-4 flex items-center gap-3">
           <div className="w-5 h-5 rounded-full bg-[#34c759] flex items-center justify-center text-white text-xs font-bold">✓</div>
           <span className="font-medium text-sm tracking-wide">{toastMessage}</span>
         </div>
       )}

     </div>
   </div>
 );
}
