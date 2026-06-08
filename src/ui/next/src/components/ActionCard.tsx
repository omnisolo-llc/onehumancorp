import React from 'react';

export type ActionCardProps = {
  id: string;
  department: string;
  description: string;
  status: 'pending' | 'approved' | 'rejected';
  featureType?: string;
  suggestedPrice?: number;
  scope?: string;
  onApprove: (id: string) => void;
  onEdit: (description: string) => void;
  onDiscard: (id: string) => void;
};

export const ActionCard: React.FC<ActionCardProps> = ({
  id,
  department,
  description,
  status,
  featureType,
  suggestedPrice,
  scope,
  onApprove,
  onEdit,
  onDiscard
}) => {
  return (
    <div className="bg-white/60 backdrop-blur-md border border-gray-200 rounded-xl p-4 shadow-sm relative overflow-hidden" data-testid="action-card">
      <div className={`absolute top-0 left-0 w-full h-1 ${status === 'approved' ? 'bg-green-500' : status === 'rejected' ? 'bg-red-500' : 'bg-gradient-to-r from-blue-400 to-indigo-500'}`}></div>
      <div className="flex items-center gap-2 mb-2">
        {status === 'pending' && (
          <span className="text-xs font-bold px-2 py-0.5 bg-orange-100 text-orange-700 rounded-full uppercase tracking-wide">Needs Approval</span>
        )}
        {status === 'approved' && (
          <span className="text-xs font-bold px-2 py-0.5 bg-green-100 text-green-700 rounded-full uppercase tracking-wide">Approved</span>
        )}
        {status === 'rejected' && (
          <span className="text-xs font-bold px-2 py-0.5 bg-red-100 text-red-700 rounded-full uppercase tracking-wide">Rejected</span>
        )}
      </div>

      {featureType === 'quote_draft' ? (
        <div data-testid="draft-quote-card">
          <p className="text-sm font-semibold text-gray-900 mb-1">Draft Quote: {department} for Customer</p>
          <p className="text-xs text-gray-600 mb-2">Scope of Work: {scope || description}</p>
          <p className="text-sm font-bold text-gray-900 mb-4">Calculated Total: ${suggestedPrice || 0}</p>
        </div>
      ) : (
        <>
          <p className="text-sm font-semibold text-gray-900 mb-1">{department}</p>
          <p className="text-xs text-gray-600 mb-4">{description}</p>
        </>
      )}

      {status === 'pending' && (
        <div className="flex gap-2">
          <button
            onClick={() => onApprove(id)}
            className="flex-1 bg-blue-600 hover:bg-blue-700 text-white text-xs font-semibold py-2 px-3 rounded-lg transition-colors"
            data-testid="approve-action-btn"
          >
            {featureType === 'quote_draft' ? 'Approve & Send' : 'Approve & Execute'}
          </button>
          <button
            type="button"
            onClick={() => onEdit(description)}
            className="bg-gray-100 hover:bg-gray-200 text-gray-700 text-xs font-medium py-2 px-3 rounded-lg transition-colors"
          >
            Edit Details
          </button>
          {featureType === 'quote_draft' && (
             <button
               type="button"
               onClick={() => onDiscard(id)}
               className="bg-red-50 hover:bg-red-100 text-red-700 text-xs font-medium py-2 px-3 rounded-lg transition-colors"
             >
               Discard
             </button>
          )}
        </div>
      )}
    </div>
  );
};
