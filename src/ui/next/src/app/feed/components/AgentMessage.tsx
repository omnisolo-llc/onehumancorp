import React, { ReactNode } from 'react';

interface AgentMessageProps {
  children: ReactNode;
}

export const AgentMessage: React.FC<AgentMessageProps> = ({ children }) => {
  return (
    <div className="flex gap-3 mb-6 w-full">
      <div className="flex-shrink-0 w-8 h-8 rounded-full bg-indigo-600 flex items-center justify-center text-white shadow-sm mt-1">
        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
        </svg>
      </div>
      <div className="flex-1 w-full max-w-[calc(100%-2.75rem)]">
        {children}
      </div>
    </div>
  );
};
