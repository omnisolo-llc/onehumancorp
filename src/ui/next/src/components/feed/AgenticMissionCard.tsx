import React from 'react';

interface MissionCardProps {
  title: string;
  description: string;
  type: 'onboarding' | 'negotiation' | 'inventory';
  onApprove: () => void;
}

export const AgenticMissionCard: React.FC<MissionCardProps> = ({ title, description, type, onApprove }) => {
  const colorMap = {
    onboarding: 'border-[#0066FF] text-[#0066FF]',
    negotiation: 'border-[#34C759] text-[#34C759]',
    inventory: 'border-[#FF9500] text-[#FF9500]',
  };

  return (
    <div className={`glass-vibrant p-5 border-l-4 ${colorMap[type]} shadow-lg animate-fade-in`}>
      <h3 className="font-bold text-lg mb-2">{title}</h3>
      <p className="text-sm text-gray-600 dark:text-gray-300 mb-4">{description}</p>
      <button
        onClick={onApprove}
        className="w-full bg-[#0066FF] text-white py-3 rounded-lg font-bold hover:bg-[#0052CC] transition-colors"
      >
        Approve & Execute
      </button>
    </div>
  );
};
