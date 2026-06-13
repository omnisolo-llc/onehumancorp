import React from 'react';

interface ApprovalCardProps {
  id: string;
  title: string;
  amount: number;
  customerName: string;
  onAccept: (id: string) => void;
  onEdit: (id: string) => void;
}

export const ApprovalCard: React.FC<ApprovalCardProps> = ({
  id,
  title,
  amount,
  customerName,
  onAccept,
  onEdit,
}) => {
  return (
    <div className="bg-white/80 dark:bg-gray-800/80 backdrop-blur-md rounded-2xl p-4 shadow-sm border border-gray-100 dark:border-gray-700 w-full" data-testid={`approval-card-${id}`}>
      <div className="flex justify-between items-start mb-3">
        <div>
          <h3 className="font-medium text-gray-900 dark:text-white text-sm">Quote Approval</h3>
          <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">For {customerName}</p>
        </div>
        <span className="font-bold text-gray-900 dark:text-white text-lg">${amount.toFixed(2)}</span>
      </div>
      <p className="text-sm text-gray-700 dark:text-gray-300 mb-4 bg-gray-50 dark:bg-gray-700/50 p-3 rounded-xl border border-gray-100 dark:border-gray-600">
        {title}
      </p>
      <div className="flex gap-2">
        <button
          onClick={() => onAccept(id)}
          className="flex-1 bg-green-600 hover:bg-green-700 text-white font-medium py-2 px-4 rounded-xl min-h-[44px] transition-colors text-sm"
          data-testid={`accept-btn-${id}`}
        >
          Accept
        </button>
        <button
          onClick={() => onEdit(id)}
          className="flex-1 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 text-gray-900 dark:text-white font-medium py-2 px-4 rounded-xl min-h-[44px] transition-colors text-sm"
          data-testid={`edit-btn-${id}`}
        >
          Edit
        </button>
      </div>
    </div>
  );
};
