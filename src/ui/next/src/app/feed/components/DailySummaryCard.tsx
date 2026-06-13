import React from 'react';

interface DailySummaryCardProps {
  id: string;
  date: string;
  summaryText: string;
  onViewDetails: (id: string) => void;
}

export const DailySummaryCard: React.FC<DailySummaryCardProps> = ({
  id,
  date,
  summaryText,
  onViewDetails,
}) => {
  return (
    <div className="bg-gradient-to-br from-indigo-50 to-white dark:from-indigo-900/20 dark:to-gray-800/80 backdrop-blur-md rounded-2xl p-5 shadow-sm border border-indigo-100 dark:border-indigo-800/30 w-full" data-testid={`daily-summary-card-${id}`}>
      <div className="flex items-center gap-2 mb-3">
        <div className="w-8 h-8 rounded-full bg-indigo-100 dark:bg-indigo-900/50 flex items-center justify-center text-indigo-600 dark:text-indigo-400">
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" /></svg>
        </div>
        <h3 className="font-medium text-indigo-900 dark:text-indigo-100 text-sm">Daily Summary • {date}</h3>
      </div>
      <p className="text-sm text-gray-700 dark:text-gray-300 mb-4 leading-relaxed">
        {summaryText}
      </p>
      <button
        onClick={() => onViewDetails(id)}
        className="w-full bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700 text-indigo-600 dark:text-indigo-400 font-medium py-2 px-4 rounded-xl min-h-[44px] transition-colors text-sm border border-indigo-200 dark:border-indigo-800"
        data-testid={`view-details-btn-${id}`}
      >
        View Details
      </button>
    </div>
  );
};
