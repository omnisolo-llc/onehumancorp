'use client';

import React, { useState, useEffect } from 'react';
import Link from 'next/link';
import AgentAutomations from './components/AgentAutomations';

export default function AgentsPage() {
 const [activeTab, setActiveTab] = useState<'departments' | 'workflows' | 'feed' | 'approvals' | 'automations'>('departments');
 const [feed, setFeed] = useState<any[]>([]);
 const [feedLoading, setFeedLoading] = useState(false);
 const [showAdvanced, setShowAdvanced] = useState(false);
 const [hasPro, setHasPro] = useState(false);
 const [showSoftPaywall, setShowSoftPaywall] = useState(false);
 const [approvals, setApprovals] = useState<any[]>([]);
 const [workflows, setWorkflows] = useState<any[]>([]);
 const [workflowName, setWorkflowName] = useState('Branch review');
 const [workflowTask, setWorkflowTask] = useState('Review the current branch for correctness, security, deployment, and test coverage issues.');
 const [workflowError, setWorkflowError] = useState('');
 const [workflowLoading, setWorkflowLoading] = useState(false);
 const [loading, setLoading] = useState(false);
 const [actionLoading, setActionLoading] = useState<string | null>(null);

 const fetchFeed = async () => {
 setFeedLoading(true);
 try {
 const res = await fetch('/api/agents/approvals/activity');
 if (res.ok) {
 const data = await res.json();
 setFeed(data.pending_approvals || []);
 }
 } catch (e) {
 console.error('Failed to fetch feed:', e);
 }
 setFeedLoading(false);
 };

 const fetchApprovals = async () => {
 setLoading(true);
 try {
 const res = await fetch('/api/agents/approvals');
 if (res.ok) {
 const data = await res.json();
 setApprovals(data.pending_approvals || []);
 }
 } catch (e) {
 console.error('Failed to fetch approvals:', e);
 }
 setLoading(false);
 };

 const fetchWorkflows = async () => {
 try {
 const res = await fetch('/api/agents/workflows');
 if (res.ok) {
 const data = await res.json();
 setWorkflows(data.workflows || []);
 }
 } catch (e) {
 console.error('Failed to fetch workflows:', e);
 }
 };

 useEffect(() => {
 fetchApprovals();
 fetchFeed();
 fetchWorkflows();
 }, []);

 const createWorkflow = async (event: React.FormEvent<HTMLFormElement>) => {
 event.preventDefault();
 setWorkflowError('');
 setWorkflowLoading(true);

 try {
 const res = await fetch('/api/agents/workflows', {
 method: 'POST',
 headers: { 'Content-Type': 'application/json' },
 body: JSON.stringify({
 name: workflowName,
 workflow: 'ohc_review_branch',
 task: workflowTask,
 }),
 });

 const data = await res.json();
 if (!res.ok) {
 setWorkflowError(data.error || 'Workflow could not be created');
 return;
 }

 setWorkflows((current) => [data.workflow, ...current.filter((item) => item.id !== data.workflow.id)]);
 setWorkflowName('Branch review');
 setWorkflowTask('Review the current branch for correctness, security, deployment, and test coverage issues.');
 setActiveTab('workflows');
 } catch (e) {
 setWorkflowError('Workflow service is unavailable');
 } finally {
 setWorkflowLoading(false);
 }
 };

 const handleDecision = async (id: string, approved: boolean) => {
 setActionLoading(id);
 try {
 const res = await fetch(`/api/agents/approvals/${id}`, {
 method: 'POST',
 headers: {
 'Content-Type': 'application/json',
 },
 body: JSON.stringify({ approved }),
 });
 if (res.ok) {
 // Remove the processed approval from the list
 setApprovals(prev => prev.filter(req => req.id !== id));
 fetchFeed(); // Refresh the activity feed
 } else {
 console.error('Failed to process approval');
 }
 } catch (e) {
 console.error('Error making decision:', e);
 }
 setActionLoading(null);
 };

 const departments = [
 { id: 'operations', name: 'The Manager', role: 'Operations', icon: '⚙️', description: 'Handles inventory, orders, and fulfillment.' },
 { id: 'customer_success', name: 'The Ambassador', role: 'Customer Success', icon: '🤝', description: 'Responds to customer inquiries and builds loyalty.' },
 { id: 'marketing', name: 'The Promoter', role: 'Marketing', icon: '📣', description: 'Creates social posts and promotional campaigns.' },
 { id: 'sales', name: 'The Closer', role: 'Sales', icon: '💼', description: 'Generates quotes and follows up on leads.' },
 { id: 'finance', name: 'The Accountant', role: 'Finance', icon: '💰', description: 'Tracks expenses and generates invoices.' },
 { id: 'legal', name: 'The Counsel', role: 'Legal', icon: '⚖️', description: 'Drafts contracts and handles compliance.' },
 { id: 'business_advisory', name: 'The Strategist', role: 'Advisory', icon: '📈', description: 'Provides insights and growth strategies.' },
 { id: 'discovery', name: 'The Scout', role: 'Discovery', icon: '🔍', description: 'Optimizes structured data for LLM crawlers.' },
 ];

 return (
 <div className="min-h-screen bg-gray-50 flex justify-center font-inter">
 {/* 375px Mobile Container constraint as per design doc */}
 <div className="w-full max-w-[375px] min-h-screen shadow-xl relative overflow-x-hidden flex flex-col bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40">
 {/* Header */}
 <header className="px-5 pt-8 pb-4 sticky top-0 z-20 border-b bg-white/65 backdrop-blur-[30px] saturate-[210%] border-white/40">
 <div className="flex justify-between items-center mb-4">
 <Link href="/dashboard" className="text-gray-400 hover:text-gray-600">
 <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
 </Link>
 <div className="flex items-center gap-2">
 <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider">Pro Mode</span>
 <button
 onClick={() => {
 if (!hasPro && !showAdvanced) {
 setShowSoftPaywall(true);
 return;
 }
 setShowAdvanced(!showAdvanced);
 }}
 className={`w-10 h-6 rounded-full transition-colors relative ${showAdvanced ? 'bg-indigo-600' : 'bg-gray-200'}`}
>
 <span className={`absolute top-1 left-1 bg-white/65 w-4 h-4 rounded-full transition-transform ${showAdvanced ? 'translate-x-4' : 'translate-x-0'}`}></span>
 </button>
 </div>
 </div>
 <h1 className="text-3xl font-extrabold font-outfit text-gray-900">AI Departments</h1>
 <p className="text-sm text-gray-500 mt-1">Your autonomous business team.</p>
 </header>

 {/* Tabs */}
 <div className="flex border-b border-gray-100">
 <button
 onClick={() => setActiveTab('departments')}
 className={`flex-1 py-3 text-sm font-semibold transition-colors ${activeTab === 'departments' ? 'text-indigo-600 border-b-2 border-indigo-600' : 'text-gray-500 hover:text-gray-700'}`}
>
 My Team
 </button>
 <button
 onClick={() => setActiveTab('workflows')}
 className={`flex-1 py-3 text-sm font-semibold transition-colors ${activeTab === 'workflows' ? 'text-indigo-600 border-b-2 border-indigo-600' : 'text-gray-500 hover:text-gray-700'}`}
>
 Workflows
 </button>
 <button
                onClick={() => setActiveTab('feed')}
                className={`pb-4 px-2 text-sm font-bold uppercase tracking-wide transition-colors ${activeTab === 'feed' ? 'text-indigo-600 border-b-2 border-indigo-600' : 'text-gray-500 hover:text-gray-900'}`}
              >
                Activity Feed
              </button>
              <button
                onClick={() => setActiveTab('automations')}
                className={`pb-4 px-2 text-sm font-bold uppercase tracking-wide transition-colors ${activeTab === 'automations' ? 'text-indigo-600 border-b-2 border-indigo-600' : 'text-gray-500 hover:text-gray-900'}`}
              >
                Automations
              </button>
              <button
                onClick={() => setActiveTab('automations')}
                className={`pb-4 px-2 text-sm font-bold uppercase tracking-wide transition-colors ${activeTab === 'automations' ? 'text-indigo-600 border-b-2 border-indigo-600' : 'text-gray-500 hover:text-gray-900'}`}
              >
                Automations
              </button>
 <button
 onClick={() => setActiveTab('approvals')}
 className={`flex-1 py-3 text-sm font-semibold transition-colors flex items-center justify-center gap-2 ${activeTab === 'approvals' ? 'text-indigo-600 border-b-2 border-indigo-600' : 'text-gray-500 hover:text-gray-700'}`}
>
 Needs Approval
 {approvals.length> 0 && (
 <span className="bg-red-500 text-white text-[10px] font-bold px-2 py-0.5 rounded-full">
 {approvals.length}
 </span>
 )}
 </button>
 </div>

 {/* Main Content */}
 <main className="flex-1 p-5 overflow-y-auto pb-24 bg-gray-50">
 {showSoftPaywall && (
 <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4">
 <div className="w-full max-w-sm rounded-[16px] bg-white p-6 shadow-2xl">
 <h2 className="text-2xl font-bold font-outfit text-gray-900">Upgrade to Pro</h2>
 <p className="mt-2 text-sm text-gray-600">Unlock Pro Mode for advanced automation controls.</p>
 <a href="/pricing" className="mt-4 block rounded-[8px] bg-indigo-600 px-4 py-3 text-center text-sm font-bold text-white">
 Upgrade to Pro
 </a>
 <button
 type="button"
 onClick={() => {
 setHasPro(true);
 setShowAdvanced(true);
 setShowSoftPaywall(false);
 }}
 className="mt-3 w-full rounded-[8px] border border-indigo-200 bg-indigo-50 px-4 py-3 text-sm font-bold text-indigo-700"
 >
 Share on X to get 7 Days Free
 </button>
 <button type="button" onClick={() => setShowSoftPaywall(false)} className="mt-3 w-full text-sm font-semibold text-gray-500">
 Close
 </button>
 </div>
 </div>
 )}
 {activeTab === 'departments' ? (
 <div className="space-y-4">
 {departments.map((dept) => (
 <button
 type="button"
 key={dept.id}
 className="w-full text-left shadow-sm p-4 rounded-[16px] flex items-start gap-4 cursor-pointer hover:shadow-md transition-shadow bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40"
>
 <div className="w-12 h-12 rounded-xl bg-indigo-50 flex items-center justify-center text-2xl shrink-0">
 {dept.icon}
 </div>
 <div>
 <h3 className="font-bold text-gray-900 font-outfit text-lg">{dept.name}</h3>
 <p className="text-xs font-semibold text-indigo-600 uppercase tracking-wide mb-1">{dept.role}</p>
 <p className="text-sm text-gray-600 leading-relaxed">{dept.description}</p>
 <p className="mt-2 text-xs font-semibold text-green-700">Active and running</p>

 {showAdvanced && (
 <div className="mt-3 pt-3 border-t border-gray-100 flex gap-4 text-xs text-gray-500">
 <span className="flex items-center gap-1">
 <span className="w-2 h-2 rounded-full bg-green-500"></span> Active
 </span>
 <span>Auto-approve: $0</span>
 </div>
 )}
 </div>
 </button>
 ))}
 <button type="button" className="w-full rounded-[16px] border border-indigo-200 bg-indigo-50 p-4 text-left font-bold text-indigo-700">
 Hire Agent
 </button>
 <div className="rounded-[16px] border border-gray-100 bg-white/65 p-4 text-sm text-gray-700">
 <div className="font-bold">Marketing Pro</div>
 <div className="font-bold">Create Workflow</div>
 </div>
 </div>

 ) : activeTab === 'workflows' ? (
 <div className="space-y-4 pb-8">
 <form onSubmit={createWorkflow} className="rounded-[16px] shadow-sm p-5 font-inter bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40">
 <div className="flex justify-between items-start gap-3 mb-4">
 <div>
 <h2 className="font-bold text-gray-900 font-outfit text-xl">Create Workflow</h2>
 <p className="text-sm text-gray-500 mt-1">Send a multi-agent workflow to the backend agent CLI.</p>
 </div>
 <span className="bg-indigo-50 text-indigo-700 text-xs font-bold px-2.5 py-1 rounded-md whitespace-nowrap">RunWorkflow</span>
 </div>

 <label className="block text-xs font-bold text-gray-600 uppercase tracking-wide mb-2" htmlFor="workflow-name">
 Name
 </label>
 <input
 id="workflow-name"
 value={workflowName}
 onChange={(event) => setWorkflowName(event.target.value)}
 className="w-full min-h-[44px] rounded-xl border border-gray-200 px-3 text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-indigo-500"
 placeholder="Branch review"
 />

 <label className="block text-xs font-bold text-gray-600 uppercase tracking-wide mt-4 mb-2" htmlFor="workflow-task">
 Task
 </label>
 <textarea
 id="workflow-task"
 value={workflowTask}
 onChange={(event) => setWorkflowTask(event.target.value)}
 className="w-full min-h-[112px] rounded-xl border border-gray-200 p-3 text-sm text-gray-900 leading-relaxed focus:outline-none focus:ring-2 focus:ring-indigo-500 resize-none"
 placeholder="Describe what the workflow should review"
 />

 {workflowError && (
 <p className="mt-3 text-sm font-medium text-red-600">{workflowError}</p>
 )}

 <button
 type="submit"
 disabled={workflowLoading}
 className={`mt-4 w-full min-h-[44px] rounded-xl font-semibold text-sm transition-colors shadow-sm ${workflowLoading ? 'bg-indigo-400 text-white cursor-not-allowed' : 'bg-indigo-600 hover:bg-indigo-700 text-white'}`}
>
 {workflowLoading ? 'Creating...' : 'Create & Run Workflow'}
 </button>
 </form>

 {workflows.length === 0 ? (
 <div className="rounded-[16px] border border-dashed p-5 text-center bg-white/65 backdrop-blur-[30px] saturate-[210%] border-white/40">
 <p className="text-gray-500 text-sm">No workflows yet.</p>
 </div>
 ) : (
 workflows.map((workflow) => (
 <div key={workflow.id} className="rounded-[16px] shadow-sm p-5 font-inter bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40">
 <div className="flex justify-between items-start gap-3">
 <div>
 <h3 className="font-bold text-gray-900 font-outfit text-lg">{workflow.name}</h3>
 <p className="text-xs font-semibold text-indigo-600 uppercase tracking-wide mt-1">{workflow.workflow}</p>
 </div>
 <span className={`text-xs font-bold px-2.5 py-1 rounded-md uppercase tracking-wide ${workflow.status === 'failed' ? 'bg-red-100 text-red-700' : workflow.status === 'completed' ? 'bg-green-100 text-green-700' : 'bg-blue-100 text-blue-700'}`}>
 {workflow.status}
 </span>
 </div>
 <p className="text-sm text-gray-700 mt-3 leading-relaxed">{workflow.task}</p>
 <div className="mt-4 rounded-xl bg-gray-50 border border-gray-100 p-3">
 <p className="text-xs font-bold text-gray-500 uppercase tracking-wide mb-1">Backend CLI</p>
 <p className="text-xs text-gray-700 break-words">{workflow.command || 'Preparing command...'}</p>
 </div>
 {workflow.error && (
 <p className="mt-3 text-sm font-medium text-red-600">{workflow.error}</p>
 )}
 </div>
 ))
 )}
 </div>

 ) : activeTab === 'automations' ? (
            <div className="pb-8">
              <AgentAutomations />
            </div>
          ) : activeTab === 'feed' ? (
 <div className="h-full flex flex-col">
 {feedLoading ? (
 <div className="flex flex-col items-center justify-center flex-1 text-center py-12">
 <div className="w-8 h-8 border-4 border-indigo-200 border-t-indigo-600 rounded-full animate-spin mb-4"></div>
 <p className="text-gray-500 text-sm font-medium">Fetching feed...</p>
 </div>
 ) : feed.length === 0 ? (
 <div className="flex flex-col items-center justify-center flex-1 text-center py-12">
 <p className="text-gray-500 text-sm">No activity yet.</p>
 </div>
 ) : (
 <div className="space-y-4 pb-8">
 {feed.map((item) => (
 <div key={item.id} className="rounded-[16px] shadow-sm p-5 font-inter bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40">
 <div className="flex justify-between items-start mb-3">
 <span className="bg-blue-100 text-blue-800 text-xs font-bold px-2.5 py-1 rounded-md uppercase tracking-wide">
 {item.department}
 </span>
 <span className="text-xs text-gray-400 font-medium">{item.status === "Approved" ? "Approved" : item.status === "Rejected" ? "Rejected" : "Draft"}</span>
 </div>
 <p className="text-gray-800 text-sm font-medium leading-relaxed">
 {item.description}
 </p>
 </div>
 ))}
 </div>
 )}
 </div>

 ) : (
 <div className="h-full flex flex-col">
 {loading ? (
 <div className="flex flex-col items-center justify-center flex-1 text-center py-12">
 <div className="w-8 h-8 border-4 border-indigo-200 border-t-indigo-600 rounded-full animate-spin mb-4"></div>
 <p className="text-gray-500 text-sm font-medium">Fetching approvals...</p>
 </div>
 ) : approvals.length === 0 ? (
 <div className="flex flex-col items-center justify-center flex-1 text-center py-12">
 <div className="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mb-4">
 <span className="text-3xl">🎉</span>
 </div>
 <h3 className="font-bold text-gray-900 font-outfit text-xl mb-2">All Caught Up!</h3>
 <p className="text-gray-500 text-sm">Your AI team has no pending tasks.</p>
 </div>
 ) : (
 <div className="space-y-4 pb-8">
 {approvals.map((req) => (
 <div key={req.id} className="rounded-[16px] shadow-sm p-5 font-inter bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40">
 <div className="flex justify-between items-start mb-3">
 <span className="bg-orange-100 text-orange-800 text-xs font-bold px-2.5 py-1 rounded-md uppercase tracking-wide">
 {req.department}
 </span>
 <span className="text-xs text-gray-400 font-medium">Draft For Review</span>
 </div>
 <p className="text-gray-800 text-sm font-medium mb-5 leading-relaxed">
 {req.description}
 </p>

 <div className="flex gap-3 mt-auto">
 <button
 onClick={() => handleDecision(req.id, false)}
 disabled={actionLoading === req.id}
 className={`flex-1 min-h-[44px] min-w-[44px] py-3 rounded-xl font-semibold text-sm transition-colors ${actionLoading === req.id ? 'bg-gray-200 text-gray-400 cursor-not-allowed' : 'bg-gray-100 hover:bg-gray-200 text-gray-700'}`}
>
 {actionLoading === req.id ? '...' : 'Edit Draft'}
 </button>
 <button
 onClick={() => handleDecision(req.id, true)}
 disabled={actionLoading === req.id}
 className={`flex-1 min-h-[44px] min-w-[44px] py-3 rounded-xl font-semibold text-sm transition-colors shadow-sm ${actionLoading === req.id ? 'bg-indigo-400 text-white cursor-not-allowed' : 'bg-indigo-600 hover:bg-indigo-700 text-white'}`}
>
 {actionLoading === req.id ? '...' : 'Approve & Send'}
 </button>
 </div>
 </div>
 ))}
 </div>
 )}
 </div>
 )}
 </main>
 </div>
 <style dangerouslySetInnerHTML={{__html: `
 @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
 .font-inter { font-family: 'Inter', sans-serif; }
 .font-outfit { font-family: 'Outfit', sans-serif; }
 `}} />
 </div>
 );
}
