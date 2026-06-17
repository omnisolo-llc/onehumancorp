import React from 'react';

interface ProposalApprovalCardProps {
  customerName: string;
  totalAmountCents: number;
  draftedEmailText: string;
  onApprove: () => void;
  onEdit: () => void;
}

export const ProposalApprovalCard: React.FC<ProposalApprovalCardProps> = ({
  customerName,
  totalAmountCents,
  draftedEmailText,
  onApprove,
  onEdit
}) => {
  return (
    <div className="bg-white/70 backdrop-blur-xl rounded-2xl p-5 shadow-sm border border-white/20 max-w-[375px] mx-auto">
      <div className="flex items-center space-x-2 mb-4">
        <div className="w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center">
          <span className="text-blue-600 font-semibold text-sm">🤖</span>
        </div>
        <div>
          <p className="text-xs text-gray-500 font-medium uppercase tracking-wide">Sales & Revenue Agent</p>
          <p className="text-sm font-semibold text-gray-900">Proposal Draft Ready</p>
        </div>
      </div>

      <div className="bg-white/80 rounded-xl p-4 mb-4 border border-gray-100">
        <p className="text-sm text-gray-600 mb-1">Quote for</p>
        <p className="font-medium text-gray-900 mb-3">{customerName}</p>

        <p className="text-sm text-gray-600 mb-1">Total</p>
        <p className="text-xl font-bold text-gray-900 mb-4">${(totalAmountCents / 100).toFixed(2)}</p>

        <div className="bg-gray-50 rounded-lg p-3">
          <p className="text-xs text-gray-500 uppercase tracking-wide mb-2">Drafted Message</p>
          <p className="text-sm text-gray-700 italic">"{draftedEmailText}"</p>
        </div>
      </div>

      <div className="flex flex-col space-y-2">
        <button
          onClick={onApprove}
          className="w-full bg-black text-white rounded-xl py-3.5 font-medium text-[15px] hover:bg-gray-800 active:scale-[0.98] transition-all min-h-[44px]"
        >
          Approve & Send
        </button>
        <button
          onClick={onEdit}
          className="w-full bg-white text-gray-700 border border-gray-200 rounded-xl py-3.5 font-medium text-[15px] hover:bg-gray-50 active:scale-[0.98] transition-all min-h-[44px]"
        >
          Edit Proposal
        </button>
      </div>
    </div>
  );
};
