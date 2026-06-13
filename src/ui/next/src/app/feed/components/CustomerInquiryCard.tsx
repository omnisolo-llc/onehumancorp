import React from 'react';

interface CustomerInquiryCardProps {
  id: string;
  customerName: string;
  messageSnippet: string;
  time: string;
  onDraftReply: (id: string) => void;
}

export const CustomerInquiryCard: React.FC<CustomerInquiryCardProps> = ({
  id,
  customerName,
  messageSnippet,
  time,
  onDraftReply,
}) => {
  return (
    <div className="bg-white/80 dark:bg-gray-800/80 backdrop-blur-md rounded-2xl p-4 shadow-sm border border-gray-100 dark:border-gray-700 w-full" data-testid={`customer-inquiry-card-${id}`}>
      <div className="flex justify-between items-center mb-2">
        <div className="flex items-center gap-2">
          <div className="w-8 h-8 rounded-full bg-blue-100 dark:bg-blue-900 flex items-center justify-center text-blue-600 dark:text-blue-300 font-semibold text-sm">
            {customerName.charAt(0)}
          </div>
          <span className="font-medium text-gray-900 dark:text-white text-sm">{customerName}</span>
        </div>
        <span className="text-xs text-gray-500 dark:text-gray-400">{time}</span>
      </div>
      <p className="text-sm text-gray-600 dark:text-gray-300 mb-4 ml-10">"{messageSnippet}"</p>
      <div className="ml-10">
        <button
          onClick={() => onDraftReply(id)}
          className="bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded-xl min-h-[44px] transition-colors text-sm w-full sm:w-auto"
          data-testid={`draft-reply-btn-${id}`}
        >
          Draft Reply
        </button>
      </div>
    </div>
  );
};
