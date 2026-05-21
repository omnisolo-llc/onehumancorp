"use client";

import React, { useState } from 'react';
import { ApprovalRequest } from '../page';

type Props = {
  departmentId: string;
  departmentName: string;
  approvals: ApprovalRequest[];
  onBack: () => void;
  onApprove: (id: string, newDescription?: string) => void;
  onReject: (id: string) => void;
};

export default function ApprovalInbox({ departmentName, approvals, onBack, onApprove, onReject }: Props) {
  const [editingId, setEditingId] = useState<string | null>(null);
  const [editValue, setEditValue] = useState('');
  const [localApprovals, setLocalApprovals] = useState<ApprovalRequest[]>(approvals);

  // Sync props when they change
  React.useEffect(() => {
    setLocalApprovals(approvals);
  }, [approvals]);

  const handleEditClick = (req: ApprovalRequest) => {
    setEditingId(req.id);
    setEditValue(req.description);
  };

  const handleCancelEdit = () => {
    setEditingId(null);
    setEditValue('');
  };

  const handleSaveEdit = (id: string) => {
    setLocalApprovals(prev => prev.map(a => a.id === id ? { ...a, description: editValue } : a));
    setEditingId(null);
    setEditValue('');
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] min-h-[812px] bg-gradient-to-br from-gray-50 to-gray-100 shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-6 px-6 bg-white/60 backdrop-blur-[30px] border-b border-white/40 sticky top-0 z-10 flex items-center gap-4">
          <button
            onClick={onBack}
            className="w-10 h-10 flex items-center justify-center rounded-full bg-white shadow-sm border border-gray-100 text-gray-500 hover:text-gray-900 transition-colors"
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
            </svg>
          </button>
          <div>
            <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">{departmentName}</h1>
            <p className="text-gray-500 text-xs font-medium uppercase tracking-wider mt-1">Approval Inbox</p>
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto px-4 py-6 pb-24 space-y-4 hide-scrollbar">
          {approvals.length === 0 ? (
            <div className="flex flex-col items-center justify-center h-64 text-center px-8">
               <div className="w-16 h-16 bg-green-50 text-green-500 rounded-full flex items-center justify-center mb-4">
                 <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                   <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                 </svg>
               </div>
               <h3 className="font-outfit font-bold text-gray-900 text-lg mb-2">All Caught Up!</h3>
               <p className="text-sm text-gray-500">There are no pending actions requiring your review.</p>
            </div>
          ) : (
            approvals.map(req => (
              <div key={req.id} className="bg-white/70 backdrop-blur-[20px] saturate-[210%] rounded-2xl p-5 shadow-[0_4px_20px_-4px_rgba(0,0,0,0.05)] border border-white/50 relative overflow-hidden">
                {/* Persona Badge */}
                <div className="flex items-center gap-3 mb-4 bg-gray-50/50 p-2 rounded-xl border border-gray-100/50 backdrop-blur-md">
                   <div className="w-8 h-8 rounded-full bg-gradient-to-tr from-blue-100 to-blue-50 flex items-center justify-center border border-blue-200/50 shadow-inner flex-shrink-0">
                      <span className="text-sm font-bold text-blue-600 font-outfit">{departmentName.charAt(4)}</span>
                   </div>
                   <div>
                     <p className="text-xs font-semibold text-gray-900 font-outfit">{departmentName}</p>
                     <p className="text-[10px] text-gray-500 uppercase tracking-wider">Action Proposed</p>
                   </div>
                   <div className="ml-auto">
                     <span className={`px-2 py-1 rounded-md text-[10px] font-bold uppercase tracking-wider ${
                       req.action_risk.toLowerCase() === 'high'
                         ? 'bg-orange-100 text-orange-700'
                         : 'bg-blue-100 text-blue-700'
                     }`}>
                       {req.action_risk} Risk
                     </span>
                   </div>
                </div>

                {editingId === req.id ? (
                  <div className="mb-6 flex flex-col gap-2">
                    <textarea
                      className="w-full text-sm leading-relaxed font-medium p-3 rounded-lg border border-blue-300 focus:outline-none focus:ring-2 focus:ring-blue-500/50 resize-none bg-white/90"
                      rows={3}
                      value={editValue}
                      onChange={(e) => setEditValue(e.target.value)}
                      autoFocus
                    />
                    <div className="flex justify-end gap-2">
                      <button onClick={handleCancelEdit} className="text-xs text-gray-500 hover:text-gray-700 px-3 py-1 font-semibold">Cancel</button>
                      <button onClick={() => handleSaveEdit(req.id)} className="text-xs bg-blue-100 text-blue-700 hover:bg-blue-200 px-3 py-1 rounded font-semibold transition-colors">Save</button>
                    </div>
                  </div>
                ) : (
                  <p className="text-gray-800 text-sm leading-relaxed mb-6 font-medium">
                    {localApprovals.find(a => a.id === req.id)?.description || req.description}
                  </p>
                )}

                {req.feature_type === 'legal_compliance' && (
                  <div className="mb-6 p-4 rounded-xl bg-orange-50 border border-orange-100 flex flex-col gap-3">
                    <div className="flex items-center gap-2 text-orange-800 font-semibold text-sm">
                      <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" /></svg>
                      Compliance Warning
                    </div>
                    <div className="text-xs text-orange-700">Projected revenue exceeds €10,000 threshold. VAT registration and updated Privacy Policy required.</div>
                    <div className="bg-white p-3 rounded-lg border border-orange-100 text-xs text-gray-600">
                      Drafting updated Europe-compliant privacy policy...
                    </div>
                  </div>
                )}

                {req.feature_type === 'global_localization' && (
                  <div className="mb-6 p-4 rounded-xl bg-indigo-50 border border-indigo-100 flex flex-col gap-3">
                    <div className="flex items-center justify-between text-indigo-800 font-semibold text-sm">
                      <div className="flex items-center gap-2">
                        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 5h12M9 3v2m1.048 9.5A18.022 18.022 0 016.412 9m6.088 9h7M11 21l5-10 5 10M12.751 5C11.783 10.77 8.07 15.61 3 18.129" /></svg>
                        Localization Preview
                      </div>
                      <span className="text-[10px] bg-indigo-100 px-2 py-0.5 rounded">es-ES</span>
                    </div>
                    <div className="grid grid-cols-2 gap-2 text-xs">
                      <div className="bg-white p-2 rounded border border-indigo-50">
                        <span className="text-gray-400 block mb-1">Original (EN)</span>
                        <div>Vegan Cake<br/>$25.00</div>
                      </div>
                      <div className="bg-white p-2 rounded border border-indigo-100 ring-1 ring-indigo-500/20">
                        <span className="text-indigo-400 block mb-1">Preview (ES)</span>
                        <div>Pastel Vegano<br/>€23.50</div>
                      </div>
                    </div>
                  </div>
                )}

                {req.feature_type === 'ai_geo' && (
                  <div className="mb-6 p-4 rounded-xl bg-emerald-50 border border-emerald-100 flex flex-col gap-3">
                     <div className="flex items-center gap-2 text-emerald-800 font-semibold text-sm">
                      <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6" /></svg>
                      Generative Engine Optimization
                    </div>
                    <div className="text-xs text-emerald-700">Optimizing unstructured data for Claude, Gemini, and ChatGPT.</div>
                    <div className="flex gap-2 text-[10px] text-emerald-600 mt-1">
                      <span className="bg-emerald-100 px-2 py-1 rounded">Smart Formatting</span>
                      <span className="bg-emerald-100 px-2 py-1 rounded">Search Engine Data</span>
                      <span className="bg-emerald-100 px-2 py-1 rounded">Answer Formatting</span>
                    </div>
                  </div>
                )}

                <div className="flex gap-3">
                  {editingId !== req.id && (
                    <button
                      onClick={() => handleEditClick(req)}
                      className="flex-1 py-3 px-4 rounded-xl font-semibold text-sm bg-gray-100 text-gray-700 hover:bg-gray-200 active:scale-[0.98] transition-all"
                    >
                      Edit
                    </button>
                  )}
                  <button
                    onClick={() => onReject(req.id)}
                    className="flex-1 py-3 px-4 rounded-xl font-semibold text-sm bg-red-50 text-red-600 hover:bg-red-100 active:scale-[0.98] transition-all"
                  >
                    Reject
                  </button>
                  <button
                    onClick={() => {
                       const currentDesc = localApprovals.find(a => a.id === req.id)?.description;
                       onApprove(req.id, currentDesc !== req.description ? currentDesc : undefined);
                    }}
                    className="flex-1 py-3 px-4 rounded-xl font-bold text-sm bg-blue-600 text-white hover:bg-blue-700 shadow-md shadow-blue-500/20 active:scale-[0.98] transition-all"
                  >
                    Approve
                  </button>
                </div>
              </div>
            ))
          )}
        </div>
      </div>
    </div>
  );
}
