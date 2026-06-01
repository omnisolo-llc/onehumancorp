import React from 'react';

interface StepProgressProps {
  currentStep: number;
  totalSteps: number;
}

export const StepProgress: React.FC<StepProgressProps> = ({ currentStep, totalSteps }) => {
  return (
    <div className="w-full mb-8">
      <div className="flex justify-between mb-2">
        <span className="text-xs font-bold text-[#0066FF] uppercase tracking-wider">Step {currentStep} of {totalSteps}</span>
        <span className="text-xs font-bold text-gray-400 uppercase tracking-wider">{Math.round((currentStep / totalSteps) * 100)}% Complete</span>
      </div>
      <div className="h-1.5 w-full bg-gray-200 dark:bg-white/10 rounded-full overflow-hidden">
        <div
          className="h-full bg-[#0066FF] transition-all duration-500 ease-out rounded-full shadow-[0_0_8px_rgba(0,102,255,0.5)]"
          style={{ width: `${(currentStep / totalSteps) * 100}%` }}
        />
      </div>
    </div>
  );
};
