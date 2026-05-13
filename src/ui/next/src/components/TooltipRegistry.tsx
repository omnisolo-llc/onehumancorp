"use client";
import React, { createContext, useContext, useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';

type TooltipContextType = {
  showTooltip: (id: string, content: string, x: number, y: number) => void;
  hideTooltip: () => void;
};

const TooltipContext = createContext<TooltipContextType | undefined>(undefined);

export const TooltipProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [tooltip, setTooltip] = useState<{ id: string; content: string; x: number; y: number } | null>(null);

  const showTooltip = (id: string, content: string, x: number, y: number) => {
    setTooltip({ id, content, x, y });
  };

  const hideTooltip = () => setTooltip(null);

  return (
    <TooltipContext.Provider value={{ showTooltip, hideTooltip }}>
      {children}
      <AnimatePresence>
        {tooltip && (
          <motion.div
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: 10 }}
            style={{
              position: 'fixed',
              top: tooltip.y + 10,
              left: tooltip.x,
              background: 'rgba(255, 255, 255, 0.9)',
              backdropFilter: 'blur(10px)',
              padding: '8px 12px',
              borderRadius: '8px',
              boxShadow: '0 4px 6px rgba(0, 0, 0, 0.1)',
              zIndex: 1000,
              fontFamily: 'Outfit, Inter, sans-serif',
              fontSize: '14px',
              maxWidth: '250px',
              color: '#333'
            }}
          >
            {tooltip.content}
          </motion.div>
        )}
      </AnimatePresence>
    </TooltipContext.Provider>
  );
};

export const useTooltip = () => {
  const context = useContext(TooltipContext);
  if (!context) throw new Error('useTooltip must be used within a TooltipProvider');
  return context;
};

export const TooltipTarget: React.FC<{ id: string; content: string; children: React.ReactNode }> = ({ id, content, children }) => {
  const { showTooltip, hideTooltip } = useTooltip();

  return (
    <div
      onMouseEnter={(e) => showTooltip(id, content, e.clientX, e.clientY)}
      onMouseLeave={hideTooltip}
      onTouchStart={(e) => showTooltip(id, content, e.touches?.[0]?.clientX || 0, e.touches?.[0]?.clientY || 0)}
      onTouchEnd={hideTooltip}
      style={{ display: 'inline-block' }}
    >
      {children}
    </div>
  );
};
