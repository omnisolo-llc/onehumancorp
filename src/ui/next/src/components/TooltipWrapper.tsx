'use client';
import React, { useState, useEffect, useRef } from 'react';
import { createPortal } from 'react-dom';

interface TooltipWrapperProps {
  registryKey: string;
  children: React.ReactNode;
}

export const TooltipWrapper: React.FC<TooltipWrapperProps> = ({ registryKey, children }) => {
  const [isVisible, setIsVisible] = useState(false);
  const [position, setPosition] = useState({ top: 0, left: 0 });
  const triggerRef = useRef<HTMLDivElement>(null);
  const timeoutRef = useRef<NodeJS.Timeout>();

  // Dummy registry fetch for now
  const tooltipText = "Helpful tooltip for " + registryKey;

  const showTooltip = () => {
    if (triggerRef.current) {
      const rect = triggerRef.current.getBoundingClientRect();
      setPosition({
        top: rect.bottom + window.scrollY + 10,
        left: rect.left + window.scrollX,
      });
      setIsVisible(true);
    }
  };

  const hideTooltip = () => {
    setIsVisible(false);
  };

  const handleMouseEnter = () => {
    timeoutRef.current = setTimeout(showTooltip, 300);
  };

  const handleMouseLeave = () => {
    if (timeoutRef.current) clearTimeout(timeoutRef.current);
    hideTooltip();
  };

  const handleTouchStart = () => {
    timeoutRef.current = setTimeout(showTooltip, 500);
  };

  const handleTouchEnd = () => {
    if (timeoutRef.current) clearTimeout(timeoutRef.current);
    hideTooltip();
  };

  useEffect(() => {
    return () => {
      if (timeoutRef.current) clearTimeout(timeoutRef.current);
    };
  }, []);

  return (
    <>
      <div
        ref={triggerRef}
        onMouseEnter={handleMouseEnter}
        onMouseLeave={handleMouseLeave}
        onTouchStart={handleTouchStart}
        onTouchEnd={handleTouchEnd}
        style={{ display: 'inline-block' }}
      >
        {children}
      </div>
      {isVisible &&
        createPortal(
          <div
            style={{
              position: 'absolute',
              top: position.top,
              left: position.left,
              backgroundColor: '#333',
              color: '#fff',
              padding: '8px 12px',
              borderRadius: '4px',
              fontSize: '14px',
              zIndex: 9999,
              maxWidth: '250px',
              pointerEvents: 'none',
              fontFamily: 'Inter, sans-serif'
            }}
            role="tooltip"
          >
            {tooltipText}
          </div>,
          document.body
        )}
    </>
  );
};
