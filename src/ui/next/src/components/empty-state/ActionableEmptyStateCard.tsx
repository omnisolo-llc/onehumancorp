import React from 'react';

export interface AgentAction {
  label: string;
  onClick: () => void;
  primary?: boolean;
}

export interface ActionableEmptyStateCardProps {
  moduleContext: string;
  message?: string;
  actions: AgentAction[];
}

export const ActionableEmptyStateCard: React.FC<ActionableEmptyStateCardProps> = ({
  moduleContext,
  message,
  actions,
}) => {
  const defaultMessage = `You don't have any active ${moduleContext} yet. I can help you get started. What would you like to do?`;
  const displayMessage = message || defaultMessage;

  return (
    <div className="w-full max-w-[343px] mx-auto mt-8 p-6 rounded-[16px] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] shadow-sm flex flex-col items-center text-center">
      <div className="w-12 h-12 rounded-full bg-blue-100 dark:bg-blue-900/30 flex items-center justify-center mb-4">
        <span className="text-2xl" aria-hidden="true">✨</span>
      </div>

      <h3 className="text-lg font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">
        Let's get started
      </h3>

      <p className="text-sm text-gray-600 dark:text-gray-300 mb-6 leading-relaxed">
        {displayMessage}
      </p>

      <div className="flex flex-col w-full gap-3">
        {actions.map((action, index) => (
          <button
            key={index}
            onClick={action.onClick}
            className={`
              w-full min-h-[44px] px-4 py-3 rounded-[8px] font-medium text-sm transition-colors
              ${action.primary
                ? 'bg-[#0066FF] text-white hover:bg-blue-600'
                : 'bg-white/50 dark:bg-black/20 text-[#1D1D1F] dark:text-[#F5F5F7] hover:bg-white/80 dark:hover:bg-black/40 border border-gray-200 dark:border-gray-700'
              }
            `}
          >
            {action.label}
          </button>
        ))}
      </div>
    </div>
  );
};
