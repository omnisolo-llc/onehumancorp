import React from 'react';

export function AgentAvatar() {
  return (
    <div className="w-8 h-8 rounded-full bg-[#0066FF] flex items-center justify-center mr-2 shrink-0 shadow-lg">
      <svg className="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
      </svg>
    </div>
  );
}
