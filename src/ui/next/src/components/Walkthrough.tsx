'use client';
import { useState, useEffect } from 'react';

export default function Walkthrough({ steps }: { steps: { target: string, content: string }[] }) {
  const [currentStep, setCurrentStep] = useState(0);
  const [targetRect, setTargetRect] = useState<DOMRect | null>(null);

  useEffect(() => {
    if (currentStep >= steps.length) return;
    const updateRect = () => {
      const el = document.getElementById(steps[currentStep].target);
      if (el) {
        setTargetRect(el.getBoundingClientRect());
      }
    };
    updateRect();
    window.addEventListener('resize', updateRect);
    return () => window.removeEventListener('resize', updateRect);
  }, [currentStep, steps]);

  if (currentStep >= steps.length) return null;

  return (
    <div className="fixed inset-0 z-[100] pointer-events-none">
       {/* Highlight overlay logic */}
       <div className="absolute inset-0 bg-black/40 pointer-events-auto transition-opacity" />

       {targetRect && (
         <div
           className="absolute bg-transparent shadow-[0_0_0_9999px_rgba(0,0,0,0.4)] pointer-events-none transition-all duration-300 rounded-lg border-2 border-blue-500"
           style={{
             top: targetRect.top - 4,
             left: targetRect.left - 4,
             width: targetRect.width + 8,
             height: targetRect.height + 8
           }}
         />
       )}

       {/* Dialog Bubble */}
       <div
         className="absolute pointer-events-auto bg-white/90 backdrop-blur-xl saturate-200 p-5 rounded-xl shadow-2xl max-w-sm border border-gray-100 transition-all duration-300"
         style={targetRect ? {
            top: targetRect.bottom + 16 > window.innerHeight - 200 ? targetRect.top - 150 : targetRect.bottom + 16,
            left: Math.max(16, Math.min(targetRect.left, window.innerWidth - 350))
         } : { bottom: 40, right: 40 }}
       >
         <p className="text-sm font-medium text-gray-800 mb-4">{steps[currentStep].content}</p>
         <div className="flex justify-between items-center">
           <span className="text-xs text-gray-500 font-semibold tracking-wide uppercase">Step {currentStep + 1} of {steps.length}</span>
           <button
             className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-sm font-bold touch-manipulation transition-colors shadow-sm"
             onClick={() => setCurrentStep(c => c + 1)}
           >
             {currentStep === steps.length - 1 ? 'Finish' : 'Next'}
           </button>
         </div>
       </div>
    </div>
  );
}
