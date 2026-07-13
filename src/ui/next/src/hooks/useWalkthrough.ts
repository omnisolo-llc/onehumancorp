import { useState, useEffect, useCallback } from 'react';

type UseWalkthroughOptions = {
  id: string;
  autoStartDelay?: number; // Delay in ms before auto-starting
};

export function useWalkthrough({ id, autoStartDelay = 1000 }: UseWalkthroughOptions) {
  const [isOpen, setIsOpen] = useState(false);
  const [isCompleted, setIsCompleted] = useState(false);

  const storageKey = `ohc_walkthrough_${id}_completed`;

  useEffect(() => {
    // Check if the walkthrough was already completed
    const checkCompletion = () => {
      try {
        const completed = localStorage.getItem(storageKey) === 'true';
        setIsCompleted(completed);

        // If not completed, auto-start after a delay
        if (!completed) {
          const timer = setTimeout(() => {
            setIsOpen(true);
          }, autoStartDelay);
          return () => clearTimeout(timer);
        }
      } catch (e) {
        // Fallback for cases where localStorage is not accessible
        console.warn('Walkthrough: Could not access localStorage', e);
      }
    };

    // Only run on client side
    if (typeof window !== 'undefined') {
      return checkCompletion();
    }
  }, [id, storageKey, autoStartDelay]);

  const start = useCallback(() => {
    setIsOpen(true);
  }, []);

  const close = useCallback(() => {
    setIsOpen(false);
  }, []);

  const complete = useCallback(() => {
    try {
      localStorage.setItem(storageKey, 'true');
    } catch (e) {
      console.warn('Walkthrough: Could not save completion state to localStorage', e);
    }
    setIsCompleted(true);
    setIsOpen(false);
  }, [storageKey]);

  return {
    isOpen,
    start,
    close,
    complete,
    isCompleted
  };
}
