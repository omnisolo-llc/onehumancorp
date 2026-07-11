"use client";

import React, { useState, useEffect, ReactNode } from 'react';
import { WithTooltip } from './TooltipRegistry';

export type Step = {
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
      targetElement.scrollIntoView?.({ behavior: 'smooth', block: 'center' });

      // We need a slight delay to let scrolling settle before measuring
      const timeoutId = setTimeout(() => {
        setTargetRect(targetElement.getBoundingClientRect());
      }, 300);

      // Also attach resize/scroll listeners for recalculation with debounce
      let resizeTimeoutId: NodeJS.Timeout;
      const handleScroll = () => {
          clearTimeout(resizeTimeoutId);
          resizeTimeoutId = setTimeout(() => {
              setTargetRect(targetElement.getBoundingClientRect());
          }, 50);
      };
      window.addEventListener('scroll', handleScroll, true);
      window.addEventListener('resize', handleScroll);

      return () => {
        clearTimeout(timeoutId);
        clearTimeout(resizeTimeoutId);
        window.removeEventListener('scroll', handleScroll, true);
        window.removeEventListener('resize', handleScroll);
      };
    } else {
      console.warn(`Walkthrough: Target element with id "${currentStep.targetId}" not found.`);
      setTargetRect(null);
    }
  }, [isOpen, currentStepIndex, steps]);

  if (!isOpen || steps.length === 0) return null;

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

  if (!targetRect) return null; // Enforce requirement: no generic popups/modals without target

  // Calculate bubble position based on targetRect
  let bubbleStyle: React.CSSProperties = {};
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
        arrowClass = "bottom-full left-1/2 -translate-x-1/2 border-b-white/90 border-x-transparent border-t-0 border-8";
        break;
      case 'top':
        bubbleStyle = {
          top: targetRect.top - margin,
          left: targetRect.left + (targetRect.width / 2),
          transform: 'translate(-50%, -100%)'
        };
        arrowClass = "top-full left-1/2 -translate-x-1/2 border-t-white/90 border-x-transparent border-b-0 border-8";
        break;
      case 'right':
         bubbleStyle = {
          top: targetRect.top + (targetRect.height / 2),
          left: targetRect.right + margin,
          transform: 'translateY(-50%)'
        };
        arrowClass = "right-full top-1/2 -translate-y-1/2 border-r-white/90 border-y-transparent border-l-0 border-8";
        break;
      case 'left':
         bubbleStyle = {
          top: targetRect.top + (targetRect.height / 2),
          left: targetRect.left - margin,
          transform: 'translate(-100%, -50%)'
        };
        arrowClass = "left-full top-1/2 -translate-y-1/2 border-l-white/90 border-y-transparent border-r-0 border-8";
        break;
    }
  }

  return (
    <>
      {/* Target Highlight Overlay (using box-shadow to punch a hole) */}
      {targetRect && (
        <div
          id="walkthrough-overlay" className="ohc-walkthrough-overlay fixed pointer-events-none transition-all duration-300 ease-in-out ring-4 ring-blue-500/50 rounded-2xl shadow-[0_0_0_9999px_rgba(0,0,0,0.6)] backdrop-blur-[2px]"
          style={{
            zIndex: 9999,
            top: targetRect.top - 4,
            left: targetRect.left - 4,
            width: targetRect.width + 8,
            height: targetRect.height + 8,
          }}
        />
      )}

      {/* Speech Bubble */}
      <div
        role="dialog"
        aria-label={`${currentStep.title} walkthrough step`}
        id="walkthrough-bubble"
        className="ohc-walkthrough-bubble fixed z-[10000] backdrop-blur-[30px] saturate-[210%] bg-white/65 dark:bg-[#16161a]/70 border border-white/40 dark:border-white/10 shadow-[0_8px_32px_rgba(0,0,0,0.15)] p-6 w-[300px] max-w-[calc(100vw-32px)] font-inter animate-pop-in"
        style={bubbleStyle}
      >
        {targetRect && (
           <div className={`absolute w-0 h-0 border-solid ${arrowClass.replace('white/90', 'white/80')}`}></div>
        )}

        <div className="flex justify-between items-start mb-3">
          <h4 className="font-bold font-outfit text-gray-900 dark:text-gray-100 text-lg leading-tight pr-4">{currentStep.title}</h4>
          <button onClick={handleSkip} id="wt-close" className="wt-close text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-100 bg-black/5 hover:bg-black/10 dark:bg-white/5 dark:hover:bg-white/10 backdrop-blur-[30px] saturate-[210%] rounded-full p-1.5 transition-all flex-shrink-0">
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
          </button>
        </div>

        <p className="text-sm text-gray-700 dark:text-gray-300 mb-5 leading-relaxed">{currentStep.content}</p>

        <div className="flex justify-between items-center pt-3 border-t border-gray-200/50 dark:border-gray-700/50">
          <span className="text-xs font-semibold tracking-wide text-gray-500 dark:text-gray-400 uppercase">
            Step {currentStepIndex + 1} of {steps.length}
          </span>
          <button
            id="wt-next"
            onClick={handleNext}
            className="bg-blue-600/90 hover:bg-blue-700 text-white px-5 py-2.5 rounded-xl text-sm font-bold shadow-lg backdrop-blur-[30px] saturate-[210%] active:scale-95 transition-all"
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

/**
 * Helper component to mark targets in the UI for the Interactive Walkthrough system.
 * It wraps its children in a relative container with the specified ID, which is then
 * targeted by the walkthrough overlay and speech bubble logic.
 */
export function WalkthroughTarget({ id, children, className = "", tooltipId }: { id: string, children?: ReactNode, className?: string, tooltipId?: string }) {
  const inner = (
    <div id={id} className={`relative ${className}`}>
      {children || <div className="hidden" aria-hidden="true" />}
    </div>
  );
  if (tooltipId) {
    return <WithTooltip id={tooltipId}>{inner}</WithTooltip>;
  }
  return inner;
}
