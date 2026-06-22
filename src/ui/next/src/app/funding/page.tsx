"use client";

import { useState, useEffect } from 'react';
import { AppShell } from '../components/AppShell';

export default function FundingEnginePage() {
    const [opportunities, setOpportunities] = useState<any[]>([]);
    const [loading, setLoading] = useState(true);
    const [selectedOpportunity, setSelectedOpportunity] = useState<any | null>(null);

    useEffect(() => {
        // Mock fetch for UI testing
        setTimeout(() => {
            setOpportunities([
                {
                    id: "test-id",
                    tenant_id: "test-tenant",
                    grant_name: "Downtown Revitalization Grant",
                    amount: 10000,
                    draft_proposal_text: "You have a 92% match based on your location and revenue. The Legal Agent has drafted the required 500-word essay detailing how you will use the funds for a new oven.",
                    status: "Drafted",
                    deadline: "2024-12-31"
                }
            ]);
            setLoading(false);
        }, 1000);
    }, []);

    const handleSubmit = async () => {
        if (!selectedOpportunity) return;

        // Mock API call to submit
        alert("Opportunity submitted!");
        setSelectedOpportunity(null);

        // Update local state to reflect submission
        setOpportunities(opportunities.map(opp =>
            opp.id === selectedOpportunity.id ? { ...opp, status: 'Submitted' } : opp
        ));
    };

    return (
        <AppShell title="Funding Engine" subtitle="Autonomous Grants & Capital">
            <main className="p-4 sm:p-6 lg:p-8 max-w-7xl mx-auto w-full">
                {/* Discover Notification / Dashboard Banner */}
                {opportunities.filter(o => o.status === 'Drafted').length > 0 && (
                    <div className="mb-8 p-4 bg-indigo-50 dark:bg-indigo-900/20 border border-indigo-100 dark:border-indigo-800 rounded-xl flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
                        <div className="flex items-start sm:items-center gap-4">
                            <div className="w-10 h-10 bg-indigo-500 rounded-full flex items-center justify-center text-white text-xl shadow-md shrink-0">
                                ✨
                            </div>
                            <div>
                                <h3 className="font-semibold text-gray-900 dark:text-white text-lg">New Funding Opportunity Found!</h3>
                                <p className="text-sm text-gray-600 dark:text-gray-300">The Finance Agent discovered {opportunities.filter(o => o.status === 'Drafted').length} grant(s) you qualify for.</p>
                            </div>
                        </div>
                    </div>
                )}

                {loading ? (
                    <div className="flex justify-center py-12">
                        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-indigo-600"></div>
                    </div>
                ) : (
                    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        {opportunities.map((opp, idx) => (
                            <div key={idx} className="bg-white/65 dark:bg-gray-800/70 backdrop-blur-[30px] saturate-[210%] p-6 rounded-[16px] border border-white/40 dark:border-white/10 shadow-sm relative overflow-hidden group">
                                <div className="absolute top-0 right-0 p-4">
                                    <span className={`px-2 py-1 text-xs font-semibold rounded-full ${
                                        opp.status === 'Drafted' ? 'bg-yellow-100 text-yellow-800' :
                                        opp.status === 'Submitted' ? 'bg-blue-100 text-blue-800' :
                                        opp.status === 'Won' ? 'bg-green-100 text-green-800' :
                                        'bg-gray-100 text-gray-800'
                                    }`}>
                                        {opp.status.toUpperCase()}
                                    </span>
                                </div>
                                <h3 className="text-lg font-bold font-outfit text-gray-900 dark:text-white mb-1 pr-16">{opp.grant_name}</h3>
                                <p className="text-sm text-gray-500 mb-4">Deadline: {new Date(opp.deadline).toLocaleDateString()}</p>

                                <p className="text-sm text-gray-600 dark:text-gray-300 mb-6 line-clamp-3">
                                    {opp.draft_proposal_text}
                                </p>

                                <div className="flex justify-between items-end mt-auto pt-4 border-t border-gray-100 dark:border-gray-700/50">
                                    <div>
                                        <p className="text-xs text-gray-500 font-semibold uppercase tracking-wider mb-1">Grant Amount</p>
                                        <p className="text-2xl font-bold font-outfit text-green-600 dark:text-green-400">
                                            ${opp.amount.toLocaleString()}
                                        </p>
                                    </div>
                                    <button
                                        className="px-4 py-2 bg-[#0066FF] hover:bg-[#0055DD] text-white text-sm font-medium rounded-[8px] transition-colors"
                                        onClick={() => setSelectedOpportunity(opp)}
                                    >
                                        Review Proposal
                                    </button>
                                </div>
                            </div>
                        ))}
                    </div>
                )}
            </main>

            {/* 1-Tap Submission Modal (Glassmorphism) */}
            {selectedOpportunity && (
                <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
                    <div className="absolute inset-0 bg-black/40 backdrop-blur-sm" onClick={() => setSelectedOpportunity(null)}></div>
                    <div className="bg-white/80 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] w-full max-w-2xl rounded-[16px] shadow-2xl border border-white/40 dark:border-white/10 flex flex-col relative z-10 max-h-[90vh]">
                        <div className="p-6 border-b border-gray-200/50 dark:border-gray-700/50 flex justify-between items-center shrink-0">
                            <div>
                                <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-[#F5F5F7]">Proposal Review</h2>
                                <p className="text-sm text-gray-500">{selectedOpportunity.grant_name}</p>
                            </div>
                            <button onClick={() => setSelectedOpportunity(null)} className="text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-800 w-8 h-8 rounded-full flex items-center justify-center transition-colors">
                                ✕
                            </button>
                        </div>

                        <div className="p-6 overflow-y-auto custom-scrollbar flex-1 space-y-6">
                            <div>
                                <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">AI Generated Essay</label>
                                <div className="bg-gray-50/50 dark:bg-black/20 p-4 rounded-[8px] border border-gray-100 dark:border-gray-800 text-sm text-gray-800 dark:text-gray-300 leading-relaxed whitespace-pre-wrap">
                                    {selectedOpportunity.draft_proposal_text}
                                </div>
                            </div>

                            <div className="bg-blue-50/50 dark:bg-blue-900/10 p-4 rounded-[8px] border border-blue-100 dark:border-blue-900/30">
                                <p className="text-sm text-blue-800 dark:text-blue-300 font-medium text-center">
                                    By tapping Submit, OHC will automatically file this application on your behalf using your verified business details.
                                </p>
                            </div>
                        </div>

                        <div className="p-6 border-t border-gray-200/50 dark:border-gray-700/50 shrink-0">
                            <div className="flex justify-end gap-3">
                                <button
                                    className="px-6 py-3 bg-gray-100 hover:bg-gray-200 dark:bg-gray-800 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-200 font-medium rounded-[8px] transition-colors"
                                    onClick={() => setSelectedOpportunity(null)}
                                >
                                    Cancel
                                </button>
                                {selectedOpportunity.status === 'Drafted' && (
                                    <button
                                        className="px-8 py-3 bg-[#0066FF] hover:bg-[#0055DD] text-white font-bold rounded-[8px] shadow-lg shadow-blue-500/30 transition-all"
                                        onClick={handleSubmit}
                                    >
                                        Submit Application
                                    </button>
                                )}
                            </div>
                        </div>
                    </div>
                </div>
            )}
        </AppShell>
    );
}
