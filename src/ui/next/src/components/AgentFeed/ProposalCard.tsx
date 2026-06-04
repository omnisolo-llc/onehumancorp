'use client';

import React from 'react';

export interface AgentProposal {
  id: string;
  department: string;
  description: string;
  actionRisk: 'LOW' | 'HIGH';
  status: 'PendingApproval' | 'Approved' | 'Rejected';
  payload?: any;
}

interface ProposalCardProps {
  proposal: AgentProposal;
  onApprove: (id: string) => void;
  onDecline: (id: string) => void;
}

export const ProposalCard: React.FC<ProposalCardProps> = ({ proposal, onApprove, onDecline }) => {
  const isHighRisk = proposal.actionRisk === 'HIGH';

  return (
    <div
      className="p-5 rounded-[16px] shadow-sm flex flex-col gap-4 transition-all hover:shadow-md"
      style={{
        background: 'rgba(255, 255, 255, 0.65)',
        backdropFilter: 'blur(30px) saturate(210%)',
        border: '1px solid rgba(255, 255, 255, 0.4)',
      }}
    >
      <div className="flex flex-col gap-1">
        <div className="flex items-center gap-2">
          <span
            className="text-[10px] font-bold uppercase tracking-wider px-2 py-1 rounded-md"
            style={{
              backgroundColor: 'rgba(0, 102, 255, 0.1)',
              color: '#0066FF',
            }}
          >
            {proposal.department.replace('_', ' ')}
          </span>
          {isHighRisk && (
            <span
              className="text-[10px] font-bold uppercase tracking-wider px-2 py-1 rounded-md"
              style={{
                backgroundColor: 'rgba(255, 59, 48, 0.1)',
                color: '#FF3B30',
              }}
            >
              Requires Review
            </span>
          )}
        </div>
        <h3 className="text-lg font-semibold font-outfit text-[#1D1D1F] leading-snug mt-1">
          {proposal.description}
        </h3>

        {proposal.payload?.context && (
          <div className="mt-2 flex flex-col gap-1 p-3 rounded-lg bg-black/5">
            {Object.entries(proposal.payload.context).map(([key, value]) => (
              <div key={key} className="flex justify-between items-center text-sm font-inter">
                <span className="text-gray-500 capitalize">{key.replace(/_/g, ' ')}:</span>
                <span className="font-semibold text-[#1D1D1F]">
                  {typeof value === 'number' && key.includes('revenue') ? `$${value.toFixed(2)}` : String(value)}
                </span>
              </div>
            ))}
          </div>
        )}
      </div>

      <div className="flex flex-col gap-3 w-full mt-2">
        <button
          onClick={() => onApprove(proposal.id)}
          className="w-full min-h-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-colors shadow-sm font-inter"
          aria-label="Approve proposal"
        >
          Approve
        </button>
        <div className="flex gap-3 w-full">
          <button
            onClick={() => {}}
            className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-200 text-[#1D1D1F] font-medium hover:bg-white/50 transition-colors font-inter"
            aria-label="Edit proposal"
          >
            Edit
          </button>
          <button
            onClick={() => onDecline(proposal.id)}
            className="flex-1 min-h-[44px] px-4 rounded-[8px] border border-gray-200 text-[#1D1D1F] font-medium hover:bg-white/50 transition-colors font-inter"
            aria-label="Decline proposal"
          >
            Decline
          </button>
        </div>
      </div>
    </div>
  );
};
