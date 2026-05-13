'use client';

import React, { useState, useEffect } from 'react';

interface TooltipProps {
  id: string;
  text: string;
  children: React.ReactNode;
  position?: 'top' | 'bottom' | 'left' | 'right';
}

export function Tooltip({ id, text, children, position = 'top' }: TooltipProps) {
  const [isVisible, setIsVisible] = useState(false);
  const [isMobile, setIsMobile] = useState(false);

  useEffect(() => {
    setIsMobile(window.innerWidth < 768);
    const handleResize = () => setIsMobile(window.innerWidth < 768);
    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, []);

  // Strict 2 sentence validation
  const sentences = text.split(/[.!?]+/).filter(Boolean);
  const safeText = sentences.length > 2
    ? sentences.slice(0, 2).join('. ') + '.'
    : text;

  const handleMouseEnter = () => !isMobile && setIsVisible(true);
  const handleMouseLeave = () => !isMobile && setIsVisible(false);

  let touchTimer: NodeJS.Timeout;
  const handleTouchStart = () => {
    touchTimer = setTimeout(() => setIsVisible(true), 500); // Long press
  };
  const handleTouchEnd = () => {
    clearTimeout(touchTimer);
    setTimeout(() => setIsVisible(false), 2000); // Hide after 2 seconds on mobile
  };

  const posClasses = {
    top: 'bottom-full mb-2 left-1/2 -translate-x-1/2',
    bottom: 'top-full mt-2 left-1/2 -translate-x-1/2',
    left: 'right-full mr-2 top-1/2 -translate-y-1/2',
    right: 'left-full ml-2 top-1/2 -translate-y-1/2'
  };

  return (
    <div
      className="relative inline-flex items-center justify-center"
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      onTouchStart={handleTouchStart}
      onTouchEnd={handleTouchEnd}
      onTouchCancel={handleTouchEnd}
      aria-describedby={`tooltip-${id}`}
    >
      {children}
      {isVisible && (
        <div
          id={`tooltip-${id}`}
          role="tooltip"
          className={`absolute z-50 px-3 py-2 text-sm text-white bg-slate-900/90 rounded-lg shadow-xl backdrop-blur-md saturate-200 w-max max-w-xs transition-opacity duration-200 pointer-events-none ${posClasses[position]}`}
          style={{ fontFamily: 'Inter, sans-serif' }}
        >
          {safeText}
        </div>
      )}
    </div>
  );
}
