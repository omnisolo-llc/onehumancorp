"use client";

import React, { useState, useEffect } from "react";
import { format } from "date-fns";

const Send = ({ className }: { className?: string }) => <svg className={className} xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><line x1="22" y1="2" x2="11" y2="13"></line><polygon points="22 2 15 22 11 13 2 9 22 2"></polygon></svg>;
const FileText = ({ className }: { className?: string }) => <svg className={className} xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path><polyline points="14 2 14 8 20 8"></polyline><line x1="16" y1="13" x2="8" y2="13"></line><line x1="16" y1="17" x2="8" y2="17"></line><polyline points="10 9 9 9 8 9"></polyline></svg>;
const Bot = ({ className }: { className?: string }) => <svg className={className} xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><rect x="3" y="11" width="18" height="10" rx="2"></rect><circle cx="12" cy="5" r="2"></circle><path d="M12 7v4"></path><line x1="8" y1="16" x2="8" y2="16"></line><line x1="16" y1="16" x2="16" y2="16"></line></svg>;
const Clock = ({ className }: { className?: string }) => <svg className={className} xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><circle cx="12" cy="12" r="10"></circle><polyline points="12 6 12 12 16 14"></polyline></svg>;
const CheckCircle = ({ className }: { className?: string }) => <svg className={className} xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path><polyline points="22 4 12 14.01 9 11.01"></polyline></svg>;
const XCircle = ({ className }: { className?: string }) => <svg className={className} xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><circle cx="12" cy="12" r="10"></circle><line x1="15" y1="9" x2="9" y2="15"></line><line x1="9" y1="9" x2="15" y2="15"></line></svg>;
const MoreVertical = ({ className }: { className?: string }) => <svg className={className} xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><circle cx="12" cy="12" r="1"></circle><circle cx="12" cy="5" r="1"></circle><circle cx="12" cy="19" r="1"></circle></svg>;

interface WorkItem {
    id: string;
    tenant_id: string;
    type_: string;
    status: string;
    title: string;
    preview: string | null;
    draft_response: string | null;
    payload: any;
    created_at: string;
    updated_at: string;
}

export default function AgentFeed() {
    const [items, setItems] = useState<WorkItem[]>([]);
    const [loading, setLoading] = useState(true);

    const fetchItems = async () => {
        try {
            const res = await fetch("/api/v1/work-feed");
            if (res.ok) {
                const data = await res.json();
                setItems(data);
            }
        } catch (err) {
            console.error("Failed to fetch work items", err);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        fetchItems();
    }, []);

    const handleAction = async (id: string, action: string) => {
        const newStatus = action === 'approve' ? 'completed' : 'archived';
        try {
            await fetch(`/api/v1/work-feed/${id}`, {
                method: "PUT",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ status: newStatus }),
            });
            fetchItems(); // Refresh the list
        } catch (err) {
            console.error(`Failed to ${action} item`, err);
        }
    };

    if (loading) {
        return (
            <div className="flex h-screen items-center justify-center bg-gray-50/50">
                <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-indigo-600"></div>
            </div>
        );
    }

    return (
        <div className="min-h-screen bg-gray-50 pb-20">
            {/* Mobile-First Header */}
            <div className="bg-white border-b border-gray-200 px-4 py-4 sticky top-0 z-10">
                <div className="max-w-md mx-auto flex items-center justify-between">
                    <div>
                        <h1 className="text-xl font-semibold text-gray-900 flex items-center gap-2">
                            <Bot className="w-6 h-6 text-indigo-600" />
                            Agent Feed
                        </h1>
                        <p className="text-sm text-gray-500 mt-1">What needs your attention today</p>
                    </div>
                </div>
            </div>

            {/* Feed Content */}
            <div className="max-w-md mx-auto px-4 py-6 space-y-6">
                {items.length === 0 ? (
                    <div className="text-center py-12">
                        <CheckCircle className="w-12 h-12 text-green-500 mx-auto mb-4" />
                        <h3 className="text-lg font-medium text-gray-900">All caught up!</h3>
                        <p className="text-gray-500 mt-2">Your agents are monitoring for new work.</p>
                    </div>
                ) : (
                    items.map((item) => (
                        <div key={item.id} className="bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden transition-all hover:shadow-md">
                            {/* Card Header */}
                            <div className="p-4 border-b border-gray-100 flex items-start justify-between">
                                <div className="flex items-center gap-3">
                                    <div className="p-2 bg-indigo-50 rounded-lg">
                                        <FileText className="w-5 h-5 text-indigo-600" />
                                    </div>
                                    <div>
                                        <h3 className="font-medium text-gray-900">{item.title}</h3>
                                        <div className="flex items-center gap-2 text-xs text-gray-500 mt-1">
                                            <span className="uppercase tracking-wider font-semibold text-indigo-600 bg-indigo-50 px-2 py-0.5 rounded">
                                                {item.type_}
                                            </span>
                                            <span>•</span>
                                            <span className="flex items-center gap-1">
                                                <Clock className="w-3 h-3" />
                                                {format(new Date(item.created_at), 'h:mm a')}
                                            </span>
                                        </div>
                                    </div>
                                </div>
                                <button className="text-gray-400 hover:text-gray-600">
                                    <MoreVertical className="w-5 h-5" />
                                </button>
                            </div>

                            {/* Card Body - Context */}
                            {item.preview && (
                                <div className="p-4 bg-gray-50/50">
                                    <p className="text-sm text-gray-700 italic">"{item.preview}"</p>
                                </div>
                            )}

                            {/* Card Body - Agent Draft (Translucent Glass Style) */}
                            {item.draft_response && (
                                <div className="p-4 border-l-4 border-indigo-500 bg-gradient-to-br from-indigo-50/80 to-white backdrop-blur-sm relative">
                                    <div className="absolute top-2 right-2 flex items-center gap-1 text-xs font-semibold text-indigo-600 bg-white/80 px-2 py-1 rounded-full shadow-sm">
                                        <Bot className="w-3 h-3" /> Agent Draft
                                    </div>
                                    <p className="text-sm text-gray-900 mt-2">{item.draft_response}</p>
                                </div>
                            )}

                            {/* Actions (Touch friendly > 44px) */}
                            <div className="p-4 bg-white flex items-center gap-3 border-t border-gray-100">
                                <button
                                    onClick={() => handleAction(item.id, 'discard')}
                                    className="flex-1 min-h-[44px] flex items-center justify-center gap-2 px-4 py-2 border border-gray-300 text-gray-700 rounded-lg text-sm font-medium hover:bg-gray-50 transition-colors"
                                >
                                    <XCircle className="w-4 h-4" />
                                    Discard
                                </button>
                                <button
                                    onClick={() => handleAction(item.id, 'edit')}
                                    className="flex-1 min-h-[44px] flex items-center justify-center gap-2 px-4 py-2 bg-indigo-50 text-indigo-700 rounded-lg text-sm font-medium hover:bg-indigo-100 transition-colors"
                                >
                                    Edit
                                </button>
                                <button
                                    onClick={() => handleAction(item.id, 'approve')}
                                    className="flex-[1.5] min-h-[44px] flex items-center justify-center gap-2 px-4 py-2 bg-indigo-600 text-white rounded-lg text-sm font-medium shadow-sm hover:bg-indigo-700 hover:shadow transition-all"
                                >
                                    <Send className="w-4 h-4" />
                                    Approve
                                </button>
                            </div>
                        </div>
                    ))
                )}
            </div>
        </div>
    );
}
