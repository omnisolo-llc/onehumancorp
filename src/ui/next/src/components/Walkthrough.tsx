"use client";

import React, { useState, useEffect, ReactNode } from 'react';

type Step = {
  targetId: string;
  title: string;
  content: string;
  position?: 'top' | 'bottom' | 'left' | 'right';
};

type WalkthroughProps = {
  steps: Step[];
  isOpen: boolean;
  onClose: () => void;
  onComplete?: () => void;
};

export function InteractiveWalkthrough({ steps, isOpen, onClose, onComplete }: WalkthroughProps) {
  const [currentStepIndex, setCurrentStepIndex] = useState(0);
  const [targetRect, setTargetRect] = useState<DOMRect | null>(null);

  useEffect(() => {
    if (!isOpen || steps.length === 0) return;

    const currentStep = steps[currentStepIndex];
    const targetElement = document.getElementById(currentStep.targetId);

    if (targetElement) {
      // Scroll into view gently if needed
      targetElement.scrollIntoView({ behavior: 'smooth', block: 'center' });

      // We need a slight delay to let scrolling settle before measuring
      const timeoutId = setTimeout(() => {
        setTargetRect(targetElement.getBoundingClientRect());
      }, 300);

      // Also attach resize/scroll listeners for recalculation (simplified for this example)
      const handleScroll = () => setTargetRect(targetElement.getBoundingClientRect());
      window.addEventListener('scroll', handleScroll, true);
      window.addEventListener('resize', handleScroll);

      return () => {
        clearTimeout(timeoutId);
        window.removeEventListener('scroll', handleScroll, true);
        window.removeEventListener('resize', handleScroll);
      };
    } else {
      console.warn(`Walkthrough: Target element with id "${currentStep.targetId}" not found.`);
      setTargetRect(null);
    }
  }, [isOpen, currentStepIndex, steps]);

  if (!isOpen || steps.length === 0) return null;
  if (process.env.NEXT_PUBLIC_E2E === 'true') return null;

  const currentStep = steps[currentStepIndex];
  const isLastStep = currentStepIndex === steps.length - 1;

  const handleNext = () => {
    if (isLastStep) {
      onComplete?.();
      onClose();
    } else {
      setCurrentStepIndex(i => i + 1);
    }
  };

  const handleSkip = () => {
    onClose();
  };

  // Calculate bubble position based on targetRect
  let bubbleStyle: React.CSSProperties = { top: '50%', left: '50%', transform: 'translate(-50%, -50%)' }; // fallback center
  let arrowClass = "";

  if (targetRect) {
    const margin = 16;
    const position = currentStep.position || 'bottom';

    switch (position) {
      case 'bottom':
        bubbleStyle = {
          top: targetRect.bottom + margin,
          left: targetRect.left + (targetRect.width / 2),
          transform: 'translateX(-50%)'
        };
        arrowClass = "bottom-full left-1/2 -translate-x-1/2 border-b-white border-x-transparent border-t-0 border-8";
        break;
      case 'top':
        bubbleStyle = {
          top: targetRect.top - margin,
          left: targetRect.left + (targetRect.width / 2),
          transform: 'translate(-50%, -100%)'
        };
        arrowClass = "top-full left-1/2 -translate-x-1/2 border-t-white border-x-transparent border-b-0 border-8";
        break;
      case 'right':
         bubbleStyle = {
          top: targetRect.top + (targetRect.height / 2),
          left: targetRect.right + margin,
          transform: 'translateY(-50%)'
        };
        arrowClass = "right-full top-1/2 -translate-y-1/2 border-r-white border-y-transparent border-l-0 border-8";
        break;
      case 'left':
         bubbleStyle = {
          top: targetRect.top + (targetRect.height / 2),
          left: targetRect.left - margin,
          transform: 'translate(-100%, -50%)'
        };
        arrowClass = "left-full top-1/2 -translate-y-1/2 border-l-white border-y-transparent border-r-0 border-8";
        break;
    }
  }

  return (
    <>
      {/* Target Highlight Overlay (using box-shadow to punch a hole) */}
      {targetRect && (
        <div
          className="fixed z-[90] pointer-events-none transition-all duration-300 ease-in-out border-2 border-blue-500 rounded-lg shadow-[0_0_0_9999px_rgba(0,0,0,0.5)]"
          style={{
            top: targetRect.top - 4,
            left: targetRect.left - 4,
            width: targetRect.width + 8,
            height: targetRect.height + 8,
          }}
        />
      )}

      {/* Speech Bubble */}
      <div
        className="fixed z-[1000] bg-white rounded-xl shadow-2xl p-5 w-[280px] font-inter animate-pop-in"
        style={bubbleStyle}
      >
        {targetRect && (
           <div className={`absolute w-0 h-0 border-solid ${arrowClass}`}></div>
        )}

        <div className="flex justify-between items-start mb-2">
          <h3 className="font-bold font-outfit text-gray-900 text-lg">{currentStep.title}</h3>
          <button onClick={handleSkip} className="text-gray-400 hover:text-gray-600">
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
          </button>
        </div>

        <p className="text-sm text-gray-600 mb-4 leading-relaxed">{currentStep.content}</p>

        <div className="flex justify-between items-center">
          <span className="text-xs font-medium text-gray-400">
            Step {currentStepIndex + 1} of {steps.length}
          </span>
          <button
            onClick={handleNext}
            className="bg-blue-600 text-white px-4 py-2 rounded-lg text-sm font-bold shadow-sm active:scale-95 transition-transform"
          >
            {isLastStep ? 'Finish' : 'Next'}
          </button>
        </div>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @keyframes pop-in {
          0% { opacity: 0; transform: scale(0.9) ${bubbleStyle.transform}; }
          100% { opacity: 1; transform: scale(1) ${bubbleStyle.transform}; }
        }
        .animate-pop-in { animation: pop-in 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275) forwards; }
      `}} />
    </>
  );
}

// Helper component to mark targets in the UI
export function WalkthroughTarget({ id, children, className = "" }: { id: string, children: ReactNode, className?: string }) {
  return (
    <div id={id} className={`relative ${className}`}>
      {children}
    </div>
  );
}