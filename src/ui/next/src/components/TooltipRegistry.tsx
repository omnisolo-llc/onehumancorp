"use client";

import React, { createContext, useContext, useState, ReactNode, useRef, useEffect } from 'react';

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

export function TooltipProvider({ children }: { children: ReactNode }) {
  const [activeTooltip, setActiveTooltip] = useState<string | null>(null);
  const [tooltipRect, setTooltipRect] = useState<DOMRect | null>(null);
  const [tooltipText, setTooltipText] = useState<string>("");
  const [tooltips, setTooltips] = useState<Record<string, string>>({});

  useEffect(() => {
    let mounted = true;
    fetch("/api/tooltips")
      .then(r => {
        if (!r.ok) throw new Error("Failed to load tooltips");
        return r.json();
      })
      .then(data => {
        if (mounted && data && typeof data === 'object' && !Array.isArray(data)) {
          const safeTooltips = Object.fromEntries(
            Object.entries(data).filter((entry): entry is [string, string] => typeof entry[1] === 'string')
          );
          setTooltips(safeTooltips);
        }
      })
      .catch(() => {});
    return () => { mounted = false; };
  }, []);

  return (
    <TooltipContext.Provider value={{ activeTooltip, setActiveTooltip, tooltipRect, setTooltipRect, tooltipText, setTooltipText, getTooltip: (id: string) => tooltips[id] }}>
      {children}
      {activeTooltip && tooltipRect && (
        <div
          className="fixed z-[100] bg-gray-900/80 text-white text-sm font-inter p-3 rounded-lg shadow-[0_8px_32px_rgba(0,0,0,0.12)] pointer-events-none w-64 text-center leading-relaxed backdrop-blur-[20px] saturate-200 border border-white/40 animate-fade-in-up"
          style={{
            top: tooltipRect.top - 10,
            left: tooltipRect.left + tooltipRect.width / 2,
            transform: 'translate(-50%, -100%)'
          }}
        >
          {tooltipText}
          <div className="absolute top-full left-1/2 transform -translate-x-1/2 border-solid border-t-gray-900/80 border-t-8 border-x-transparent border-x-8 border-b-0"></div>
        </div>
      )}
      <style dangerouslySetInnerHTML={{__html: `
        @keyframes fade-in-up {
          0% { opacity: 0; transform: translate(-50%, -90%); }
          100% { opacity: 1; transform: translate(-50%, -100%); }
        }
        .animate-fade-in-up { animation: fade-in-up 0.2s ease-out forwards; }
      `}} />
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

  const handleMouseEnter = () => {
    if (wrapperRef.current) {
      setTooltipRect(wrapperRef.current.getBoundingClientRect());
      setTooltipText(getTooltip(id) || defaultText || id);
      setActiveTooltip(id);
    }
  };

  const handleMouseLeave = () => {
    setActiveTooltip(null);
  };

  // Mobile support: Long press
  const timerRef = useRef<NodeJS.Timeout | null>(null);
  const handleTouchStart = () => {
    if (timerRef.current) clearTimeout(timerRef.current);
    timerRef.current = setTimeout(() => {
      handleMouseEnter();
    }, 500); // 500ms for long press
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
      onTouchEnd={handleTouchEnd}
      onTouchCancel={handleTouchEnd}
      onContextMenu={(e) => e.preventDefault()}
      className="inline-block relative cursor-help"
    >
      {children}
    </div>
  );
}
