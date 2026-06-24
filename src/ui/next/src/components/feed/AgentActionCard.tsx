import React from 'react';

export type AgentActionCardProps = {
  id: string;
  department: string;
  title: string;
  content: React.ReactNode;
  primaryAction: {
    label: string;
    onClick: () => void;
    testId?: string;
  };
  secondaryAction?: {
    label: string;
    onClick: () => void;
    testId?: string;
  };
  tertiaryAction?: {
    label: string;
    onClick: () => void;
    testId?: string;
  };
  timestamp?: string;
  isProcessing?: boolean;
};

export const AgentActionCard: React.FC<AgentActionCardProps> = ({
  id,
  department,
  title,
  content,
  primaryAction,
  secondaryAction,
  tertiaryAction,
  timestamp,
  isProcessing = false
}) => {
  const getDepartmentColor = () => {
    switch (department.toLowerCase()) {
      case 'dispute resolution':
      case 'dispute':
        return 'text-[#FF9500] dark:text-[#FF9F0A] bg-[#FF9500] dark:bg-[#FF9F0A]';
      case 'marketing':
      case 'instagram dm':
      case 'customer message':
      case 'smart estimate':
      case 'deposit follow-up':
      case 'new booking request':
        return 'text-[#0066FF] dark:text-[#0071E3] bg-[#0066FF] dark:bg-[#0071E3]';
      default:
        return 'text-[#0066FF] dark:text-[#0071E3] bg-[#0066FF] dark:bg-[#0071E3]';
    }
  };

  const colors = getDepartmentColor().split(' ');
  const textColorClass = colors[0] + ' ' + colors[1];
  const bgColorClass = colors[2] + ' ' + colors[3];

  return (
    <div
      className={`ohc-feed-card glassmorphism p-5 relative overflow-hidden transition-all duration-300 rounded-[16px] backdrop-blur-[30px] backdrop-saturate-[210%] w-full max-w-[375px] mx-auto ${
        isProcessing ? 'opacity-50 scale-[0.98]' : 'animate-fade-in'
      }`}
      data-testid="agent-feed-card"
      style={{ boxSizing: 'border-box' }}
    >
      <div className="flex justify-between items-start mb-3">
        <span className={`text-[11px] font-bold uppercase tracking-wider ${textColorClass} flex items-center gap-1.5`}>
          <span className={`w-2 h-2 rounded-full ${bgColorClass} opacity-80`}></span>
          {department}
        </span>
        {timestamp && (
          <span className="text-[11px] text-gray-400 font-medium">
            {timestamp}
          </span>
        )}
      </div>

      <h3 className="font-bold text-gray-900 dark:text-white text-[15px] mb-2 leading-snug">
        {title}
      </h3>

      <div className="text-[13px] text-gray-600 dark:text-gray-300 leading-relaxed mb-4">
        {content}
      </div>

      <div className="flex flex-col sm:flex-row gap-3 w-full">
        <button
          onClick={primaryAction.onClick}
          disabled={isProcessing}
          className={`flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[16px] text-white font-medium shadow-md flex items-center justify-center transition-all duration-200 ${
             department.toLowerCase() === 'dispute resolution' || department.toLowerCase() === 'dispute' ? 'bg-[#FF9500] hover:bg-[#E68A00]' : 'bg-[#0066FF] hover:bg-[#0052CC]'
          }`}
          data-testid={primaryAction.testId || 'feed-approve-btn'}
        >
          {isProcessing ? 'Processing...' : primaryAction.label}
        </button>

        {secondaryAction && (
          <button
            onClick={secondaryAction.onClick}
            disabled={isProcessing}
            className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[16px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
            data-testid={secondaryAction.testId || 'feed-edit-btn'}
          >
            {secondaryAction.label}
          </button>
        )}

        {tertiaryAction && (
          <button
            onClick={tertiaryAction.onClick}
            disabled={isProcessing}
            className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[16px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
            data-testid={tertiaryAction.testId || 'feed-dismiss-btn'}
          >
            {tertiaryAction.label}
          </button>
        )}
      </div>
    </div>
  );
};
