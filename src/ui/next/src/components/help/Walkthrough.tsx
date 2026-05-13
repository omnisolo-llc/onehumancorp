'use client';

import React, { useState, useEffect } from 'react';

interface Step {
  targetId: string;
  title: string;
  content: string;
}

interface WalkthroughProps {
  flowId: string;
  steps: Step[];
  onComplete?: () => void;
}

export function Walkthrough({ flowId, steps, onComplete }: WalkthroughProps) {
  const [currentStepIndex, setCurrentStepIndex] = useState(0);
  const [isActive, setIsActive] = useState(false);
  const [targetRect, setTargetRect] = useState<DOMRect | null>(null);

  useEffect(() => {
    // Check if user has already seen this walkthrough
    const hasSeen = localStorage.getItem(`walkthrough_${flowId}`);
    if (!hasSeen) {
      setIsActive(true);
    }
  }, [flowId]);

  useEffect(() => {
    if (!isActive) return;

    const step = steps[currentStepIndex];
    if (!step) return;

    const updatePosition = () => {
      const el = document.getElementById(step.targetId);
      if (el) {
        setTargetRect(el.getBoundingClientRect());
      } else {
        setTargetRect(null);
      }
    };

    updatePosition();
    window.addEventListener('resize', updatePosition);
    window.addEventListener('scroll', updatePosition);

    // MutationObserver to catch element appearing later
    const observer = new MutationObserver(updatePosition);
    observer.observe(document.body, { childList: true, subtree: true });

    return () => {
      window.removeEventListener('resize', updatePosition);
      window.removeEventListener('scroll', updatePosition);
      observer.disconnect();
    };
  }, [isActive, currentStepIndex, steps]);

  if (!isActive || !targetRect || !steps[currentStepIndex]) return null;

  const step = steps[currentStepIndex];

  const handleNext = () => {
    if (currentStepIndex < steps.length - 1) {
      setCurrentStepIndex(curr => curr + 1);
    } else {
      finish();
    }
  };

  const finish = () => {
    setIsActive(false);
    localStorage.setItem(`walkthrough_${flowId}`, 'true');
    if (onComplete) onComplete();
  };

  return (
    <>
      <div className="fixed inset-0 z-40 bg-black/20 backdrop-blur-[2px] pointer-events-none" />
      <div
        className="fixed z-40 pointer-events-none border-4 border-blue-500 rounded-lg transition-all duration-300 shadow-[0_0_0_9999px_rgba(0,0,0,0.4)]"
        style={{
          top: targetRect.top - 4,
          left: targetRect.left - 4,
          width: targetRect.width + 8,
          height: targetRect.height + 8,
        }}
      />
      <div
        className="fixed z-50 bg-white/90 backdrop-blur-[20px] saturate-[200%] p-4 rounded-xl shadow-2xl border border-white/20 w-72 pointer-events-auto transition-all duration-300"
        style={{
          top: targetRect.bottom + 12,
          left: Math.max(12, Math.min(targetRect.left, window.innerWidth - 300)),
          fontFamily: 'Inter, sans-serif'
        }}
      >
        <h4 className="font-bold text-slate-900 mb-1" style={{ fontFamily: 'Outfit, sans-serif' }}>{step.title}</h4>
        <p className="text-sm text-slate-600 mb-4">{step.content}</p>
        <div className="flex items-center justify-between">
          <span className="text-xs text-slate-400">Step {currentStepIndex + 1} of {steps.length}</span>
          <div className="flex gap-2">
            <button onClick={finish} className="px-3 py-1.5 text-xs font-medium text-slate-500 hover:text-slate-700">Skip</button>
            <button onClick={handleNext} className="px-3 py-1.5 text-xs font-medium bg-blue-600 text-white rounded-lg hover:bg-blue-700 shadow-md">
              {currentStepIndex < steps.length - 1 ? 'Next' : 'Done'}
            </button>
          </div>
        </div>
      </div>
    </>
  );
}
