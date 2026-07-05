"use client";

import React, { createContext, useContext, useState, ReactNode, useRef, useEffect } from 'react';
import { motion, AnimatePresence } from "framer-motion";

type TooltipContextType = {
  activeTooltip: string | null;
  setActiveTooltip: (id: string | null) => void;
  tooltipRect: DOMRect | null;
  setTooltipRect: (rect: DOMRect | null) => void;
  tooltipText: string;
  setTooltipText: (text: string) => void;
  getTooltip: (id: string) => string | undefined;
};

const TooltipContext = createContext<TooltipContextType | undefined>(undefined);

declare global {
  interface Window {
    OHC_TOOLTIPS?: Record<string, string>;
  }
}

export function TooltipProvider({ children }: { children: ReactNode }) {
  const [activeTooltip, setActiveTooltip] = useState<string | null>(null);
  const [tooltipRect, setTooltipRect] = useState<DOMRect | null>(null);
  const [tooltipText, setTooltipText] = useState<string>("");
  const [tooltips, setTooltips] = useState<Record<string, string>>({});

  useEffect(() => {
    const abortController = new AbortController();
    const fetchTooltips = async () => {
      try {
        const response = await fetch("/api/tooltips", { signal: abortController.signal });
        if (!response.ok) {
          throw new Error(`Failed to load tooltips, status: ${response.status}`);
        }
        const data = await response.json();
        if (data && typeof data === 'object' && !Array.isArray(data)) {
          const safeTooltips = Object.fromEntries(
            Object.entries(data).filter((entry): entry is [string, string] => typeof entry[1] === 'string')
          );
          setTooltips(prev => ({ ...prev, ...safeTooltips }));
          window.OHC_TOOLTIPS = { ...(window.OHC_TOOLTIPS || {}), ...safeTooltips };
        }
      } catch (e: any) {
        if (e.name !== 'AbortError') {
          console.error('Failed to load tooltips', e);
        }
      }
    };
    fetchTooltips();
    return () => { abortController.abort(); };
  }, []);

  const [windowWidth, setWindowWidth] = useState(1000);

  useEffect(() => {
    setWindowWidth(window.innerWidth);
    let timeoutId: NodeJS.Timeout;
    const handleResize = () => {
      clearTimeout(timeoutId);
      timeoutId = setTimeout(() => setWindowWidth(window.innerWidth), 150);
    };
    window.addEventListener('resize', handleResize);
    return () => {
      clearTimeout(timeoutId);
      window.removeEventListener('resize', handleResize);
    };
  }, []);

  useEffect(() => {
    let scrollTimeoutId: NodeJS.Timeout;
    const handleScroll = () => {
      clearTimeout(scrollTimeoutId);
      scrollTimeoutId = setTimeout(() => setActiveTooltip(null), 150);
    };
    window.addEventListener('scroll', handleScroll, true);
    return () => {
      window.removeEventListener('scroll', handleScroll, true);
    };
  }, []);

  return (
    <TooltipContext.Provider value={{ activeTooltip, setActiveTooltip, tooltipRect, setTooltipRect, tooltipText, setTooltipText, getTooltip: (id: string) => tooltips[id] }}>
      {children}

      <AnimatePresence>
      {activeTooltip && tooltipRect && (
        <motion.div
          initial={{ opacity: 0, y: 5, x: "-50%" }}
          animate={{ opacity: 1, y: 0, x: "-50%" }}
          exit={{ opacity: 0 }}
          transition={{ duration: 0.2 }}
          className="fixed z-[100] backdrop-blur-[30px] backdrop-saturate-[2.1] bg-white/65 dark:bg-[#16161a]/70 border border-white/40 dark:border-white/10 !rounded-[8px] text-gray-900 dark:text-gray-100 text-sm font-inter p-3 shadow-[0_12px_40px_rgba(0,0,0,0.2)] pointer-events-none w-auto max-w-[calc(100vw-32px)] md:max-w-xs mx-4 text-center leading-relaxed"
          style={{
            top: tooltipRect.top - 10,
            left: Math.max(144, Math.min(windowWidth - 144, tooltipRect.left + tooltipRect.width / 2)),
            marginTop: '-100%'
          }}
        >
          {tooltipText}
          <div className="absolute top-full left-1/2 transform -translate-x-1/2 border-solid border-t-white/90 dark:border-t-black/80 border-t-8 border-x-transparent border-x-8 border-b-0 backdrop-blur-[30px] backdrop-saturate-[2.1] bg-white/65 dark:bg-[#16161a]/70"></div>
        </motion.div>
      )}
      </AnimatePresence>


    </TooltipContext.Provider>
  );
}

export function useTooltip() {
  const context = useContext(TooltipContext);
  if (!context) {
    throw new Error('useTooltip must be used within a TooltipProvider');
  }
  return context;
}

export function WithTooltip({ children, id, defaultText }: { children: ReactNode, id: string, defaultText?: string }) {
  const { setActiveTooltip, setTooltipRect, setTooltipText, getTooltip } = useTooltip();
  const wrapperRef = useRef<HTMLDivElement>(null);

  const handleMouseEnter = React.useCallback(() => {
    if (wrapperRef.current) {
      setTooltipRect(wrapperRef.current.getBoundingClientRect());
      setTooltipText(getTooltip(id) || defaultText || id);
      setActiveTooltip(id);
    }
  }, [id, defaultText, getTooltip, setActiveTooltip, setTooltipRect]);

  const handleMouseLeave = React.useCallback(() => {
    setActiveTooltip(null);
  }, [setActiveTooltip]);

  // Mobile support: Long press
  const timerRef = useRef<NodeJS.Timeout | null>(null);
  const handleTouchStart = () => {
    if (timerRef.current) clearTimeout(timerRef.current);
    timerRef.current = setTimeout(() => {
      handleMouseEnter();
    }, 500); // 500ms for long press
  };

  const handleTouchMove = () => {
    // If the user scrolls, cancel the long press tooltip
    if (timerRef.current) clearTimeout(timerRef.current);
    setActiveTooltip(null);
  };

  const handleTouchEnd = () => {
    if (timerRef.current) clearTimeout(timerRef.current);
    const hideTimer = setTimeout(() => {
        setActiveTooltip(null);
    }, 2000); // Hide after 2 seconds on mobile
    timerRef.current = hideTimer;
  };

  useEffect(() => {
    return () => {
      if (timerRef.current) clearTimeout(timerRef.current);
    };
  }, []);

  return (
    <div
      ref={wrapperRef}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onTouchStart={handleTouchStart}
      onTouchMove={handleTouchMove}
      onTouchEnd={handleTouchEnd}
      onTouchCancel={handleTouchEnd}
      onContextMenu={(e) => e.preventDefault()}
      id={id}
      className="inline-block relative cursor-help"
    >
      {children}
    </div>
  );
}
