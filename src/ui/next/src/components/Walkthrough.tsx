"use client";

import React, { useState, useEffect, ReactNode } from 'react';

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

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        onClose();
      } else if (e.key === 'Enter' || e.key === 'ArrowRight') {
        if (currentStepIndex === steps.length - 1) {
          onComplete?.();
          onClose();
        } else {
          setCurrentStepIndex(i => i + 1);
        }
      } else if (e.key === 'ArrowLeft' && currentStepIndex > 0) {
        setCurrentStepIndex(i => i - 1);
      }
    };
    window.addEventListener('keydown', handleKeyDown);

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
        window.removeEventListener('keydown', handleKeyDown);
      };
    } else {
      console.warn(`Walkthrough: Target element with id "${currentStep.targetId}" not found.`);
      setTargetRect(null);
      return () => window.removeEventListener('keydown', handleKeyDown);
    }
  }, [isOpen, currentStepIndex, steps, onClose, onComplete]);

  if (!isOpen || steps.length === 0) return null;
  const isE2E = process.env.NEXT_PUBLIC_E2E === 'true';
  const forceWalkthrough = typeof window !== 'undefined' && (window.localStorage.getItem('TEST_WALKTHROUGH') === 'true' || window.location.search.includes('test_walkthrough=true'));
  if (isE2E && !forceWalkthrough) return null;

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

  const handleBack = () => {
    if (currentStepIndex > 0) {
      setCurrentStepIndex(i => i - 1);
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
        arrowClass = "bottom-full left-1/2 -translate-x-1/2 border-b-white/65 border-x-transparent border-t-0 border-8";
        break;
      case 'top':
        bubbleStyle = {
          top: targetRect.top - margin,
          left: targetRect.left + (targetRect.width / 2),
          transform: 'translate(-50%, -100%)'
        };
        arrowClass = "top-full left-1/2 -translate-x-1/2 border-t-white/65 border-x-transparent border-b-0 border-8";
        break;
      case 'right':
         bubbleStyle = {
          top: targetRect.top + (targetRect.height / 2),
          left: targetRect.right + margin,
          transform: 'translateY(-50%)'
        };
        arrowClass = "right-full top-1/2 -translate-y-1/2 border-r-white/65 border-y-transparent border-l-0 border-8";
        break;
      case 'left':
         bubbleStyle = {
          top: targetRect.top + (targetRect.height / 2),
          left: targetRect.left - margin,
          transform: 'translate(-100%, -50%)'
        };
        arrowClass = "left-full top-1/2 -translate-y-1/2 border-l-white/65 border-y-transparent border-r-0 border-8";
        break;
    }
  }

  return (
    <>
      {/* Target Highlight Overlay (using box-shadow to punch a hole) */}
      {targetRect && (
        <div
          className="fixed z-[90] pointer-events-none transition-all duration-300 ease-in-out border-2 border-[#0066FF] rounded-lg shadow-[0_0_0_9999px_rgba(0,0,0,0.5)]"
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
        role="dialog"
        aria-label={`${currentStep.title} walkthrough step`}
        className="fixed z-[1000] bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 rounded-2xl shadow-[0_12px_40px_rgba(0,0,0,0.12)] p-6 w-[300px] max-w-[calc(100vw-32px)] font-inter animate-pop-in"
        style={bubbleStyle}
      >
        {targetRect && (
           <div className={`absolute w-0 h-0 border-solid ${arrowClass}`}></div>
        )}

        <div className="flex justify-between items-start mb-3">
          <h3 className="font-bold font-outfit text-gray-900 text-lg leading-tight pr-4">{currentStep.title}</h3>
          <button onClick={handleSkip} aria-label="Skip walkthrough" className="text-gray-400 hover:text-gray-900 bg-gray-100 hover:bg-gray-200 rounded-full p-1 transition-all flex-shrink-0 focus:outline-none focus:ring-2 focus:ring-[#0066FF]">
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
          </button>
        </div>

        <p className="text-sm text-gray-700 mb-5 leading-relaxed">{currentStep.content}</p>

        {/* Stepper Progress Bar */}
        <div className="w-full bg-gray-200 rounded-full h-1.5 mb-4">
          <div className="bg-[#0066FF] h-1.5 rounded-full transition-all duration-300" style={{ width: `${((currentStepIndex + 1) / steps.length) * 100}%` }}></div>
        </div>

        <div className="flex justify-between items-center pt-2 border-t border-gray-100/80">
          <span className="text-xs font-semibold tracking-wide text-gray-400 uppercase">
            Step {currentStepIndex + 1} of {steps.length}
          </span>
          <div className="flex space-x-2">
            {currentStepIndex > 0 && (
              <button
                onClick={handleBack}
                className="bg-gray-100 hover:bg-gray-200 text-gray-700 px-3 py-2 rounded-xl text-sm font-bold active:scale-95 transition-all focus:outline-none focus:ring-2 focus:ring-[#0066FF]"
              >
                Back
              </button>
            )}
            <button
              onClick={handleNext}
              className="bg-[#0066FF]/95 hover:bg-[#0052cc] text-white px-5 py-2 rounded-xl text-sm font-bold shadow-[0_4px_12px_rgba(0,102,255,0.2)] active:scale-95 transition-all focus:outline-none focus:ring-2 focus:ring-[#0066FF] focus:ring-offset-2"
            >
              {isLastStep ? 'Finish' : 'Next'}
            </button>
          </div>
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
export function WalkthroughTarget({ id, children, className = "" }: { id: string, children: ReactNode, className?: string }) {
  return (
    <div id={id} className={`relative ${className}`}>
      {children}
    </div>
  );
}
