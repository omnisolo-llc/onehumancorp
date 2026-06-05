import React from 'react';

export interface AgentProposal {
  id: string;
  agent_id: string;
  title: string;
  description: string;
  status: 'pending' | 'approved' | 'declined';
  created_at: string;
  payload?: any;
}

interface ProposalCardProps {
  proposal: AgentProposal;
  onApprove: (id: string) => void;
  onDecline: (id: string) => void;
}

export const ProposalCard: React.FC<ProposalCardProps> = ({ proposal, onApprove, onDecline }) => {
  const isPending = proposal.status === 'pending';

  return (
    <div className="relative group overflow-hidden rounded-2xl border border-white/10 bg-white/5 backdrop-blur-3xl p-5 transition-all hover:bg-white/10">
      <div className="flex items-start justify-between mb-3">
        <div className="flex items-center space-x-3">
          <div className="w-10 h-10 rounded-full bg-blue-500/20 flex items-center justify-center border border-blue-500/30">
            <span className="text-blue-400 text-xs font-bold uppercase tracking-wider">
              {proposal.agent_id.substring(0, 2)}
            </span>
          </div>
          <div>
            <h3 className="text-white font-medium text-sm leading-tight">{proposal.title}</h3>
            <p className="text-white/40 text-[10px] uppercase tracking-widest mt-1">
              {proposal.agent_id} • {new Date(proposal.created_at).toLocaleDateString()}
            </p>
          </div>
        </div>
        <div className="flex items-center">
          {proposal.status === 'approved' && (
            <span className="bg-green-500/20 text-green-400 text-[10px] px-2 py-0.5 rounded-full border border-green-500/30 font-medium">
              Approved
            </span>
          )}
          {proposal.status === 'declined' && (
            <span className="bg-red-500/20 text-red-400 text-[10px] px-2 py-0.5 rounded-full border border-red-500/30 font-medium">
              Declined
            </span>
          )}
        </div>
      </div>

      <p className="text-white/80 text-sm leading-relaxed mb-5">
        {proposal.description}
      </p>

      {isPending && (
        <div className="flex space-x-3">
          <button
            onClick={() => onApprove(proposal.id)}
            className="flex-1 h-11 rounded-xl bg-blue-600 text-white text-sm font-semibold transition-all hover:bg-blue-500 active:scale-[0.98] focus:outline-none focus:ring-2 focus:ring-blue-500/50"
          >
            Approve
          </button>
          <button
            onClick={() => onDecline(proposal.id)}
            className="flex-1 h-11 rounded-xl bg-white/5 text-white/70 text-sm font-semibold border border-white/10 transition-all hover:bg-white/10 active:scale-[0.98] focus:outline-none focus:ring-2 focus:ring-white/20"
          >
            Decline
          </button>
        </div>
      )}
    </div>
  );
};
