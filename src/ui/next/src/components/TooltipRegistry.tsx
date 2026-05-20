"use client";

import React, { createContext, useContext, useState, ReactNode, useRef, useEffect } from 'react';

type TooltipContextType = {
  activeTooltip: string | null;
  setActiveTooltip: (id: string | null) => void;
  tooltipRect: DOMRect | null;
  setTooltipRect: (rect: DOMRect | null) => void;
  tooltipText: string;
  setTooltipText: (text: string) => void;
};

const TooltipContext = createContext<TooltipContextType | undefined>(undefined);

export function TooltipProvider({ children }: { children: ReactNode }) {
  const [activeTooltip, setActiveTooltip] = useState<string | null>(null);
  const [tooltipRect, setTooltipRect] = useState<DOMRect | null>(null);
  const [tooltipText, setTooltipText] = useState<string>("");

  return (
    <TooltipContext.Provider value={{ activeTooltip, setActiveTooltip, tooltipRect, setTooltipRect, tooltipText, setTooltipText }}>
      {children}
      {activeTooltip && tooltipRect && (
        <div
          className="fixed z-[100] bg-gray-900 text-white text-sm font-inter p-3 rounded-lg shadow-xl pointer-events-none w-64 text-center leading-relaxed backdrop-blur-md bg-opacity-95 border border-gray-700 animate-fade-in-up"
          style={{
            top: tooltipRect.top - 10,
            left: tooltipRect.left + tooltipRect.width / 2,
            transform: 'translate(-50%, -100%)'
          }}
        >
          {tooltipText}
          <div className="absolute top-full left-1/2 transform -translate-x-1/2 border-solid border-t-gray-900 border-t-8 border-x-transparent border-x-8 border-b-0"></div>
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

export function WithTooltip({ text, children, id }: { text: string, children: ReactNode, id: string }) {
  const { setActiveTooltip, setTooltipRect, setTooltipText } = useTooltip();
  const wrapperRef = useRef<HTMLDivElement>(null);

  const handleMouseEnter = () => {
    if (wrapperRef.current) {
      setTooltipRect(wrapperRef.current.getBoundingClientRect());
      setTooltipText(text);
      setActiveTooltip(id);
    }
  };

  const handleMouseLeave = () => {
    setActiveTooltip(null);
  };

  // Mobile support: Long press
  let timer: NodeJS.Timeout;
  const handleTouchStart = () => {
    timer = setTimeout(() => {
      handleMouseEnter();
    }, 500); // 500ms for long press
  };

  const handleTouchEnd = () => {
    clearTimeout(timer);
    setTimeout(() => {
        setActiveTooltip(null);
    }, 2000); // Hide after 2 seconds on mobile
  };

  return (
    <div
      ref={wrapperRef}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onTouchStart={handleTouchStart}
      onTouchEnd={handleTouchEnd}
      onTouchCancel={handleTouchEnd}
      className="inline-block relative cursor-help"
    >
      {children}
    </div>
  );
}