import React, { createContext, useContext, useState, ReactNode } from 'react';

// Tooltip Registry allows contextual help across the app without polluting component code
interface TooltipData {
  elementId: string;
  text: string;
}

interface TooltipContextType {
  registerTooltip: (id: string, text: string) => void;
  getTooltip: (id: string) => string | undefined;
}

const TooltipContext = createContext<TooltipContextType | undefined>(undefined);

export const TooltipProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [tooltips, setTooltips] = useState<Record<string, string>>({});

  const registerTooltip = (id: string, text: string) => {
    setTooltips(prev => {
      if (prev[id] === text) return prev;
      return { ...prev, [id]: text };
    });
  };

  const getTooltip = (id: string) => tooltips[id];

  return (
    <TooltipContext.Provider value={{ registerTooltip, getTooltip }}>
      {children}
    </TooltipContext.Provider>
  );
};

export const useTooltipRegistry = () => {
  const context = useContext(TooltipContext);
  if (!context) throw new Error("useTooltipRegistry must be used within TooltipProvider");
  return context;
};

// The wrapper component to add to elements
export const ContextualTooltip: React.FC<{ id: string; defaultText: string; children: React.ReactElement }> = ({ id, defaultText, children }) => {
  const { registerTooltip, getTooltip } = useTooltipRegistry();
  const [isHovered, setIsHovered] = useState(false);

  // Register the default text on mount
  React.useEffect(() => {
    registerTooltip(id, defaultText);
  }, [id, defaultText, registerTooltip]);

  const text = getTooltip(id) || defaultText;

  return (
    <div
      style={{ position: 'relative', display: 'inline-block' }}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
      onTouchStart={() => setIsHovered(true)} // Basic mobile long-press emulation
      onTouchEnd={() => setIsHovered(false)}
    >
      {children}
      {isHovered && (
        <div style={{
          position: 'absolute',
          bottom: '100%',
          left: '50%',
          transform: 'translateX(-50%)',
          marginBottom: '8px',
          padding: '8px 12px',
          background: 'rgba(0, 0, 0, 0.85)',
          backdropFilter: 'blur(20px) saturate(200%)', // Glassmorphism standard
          color: 'white',
          fontSize: '14px',
          borderRadius: '6px',
          whiteSpace: 'nowrap',
          zIndex: 1000,
          pointerEvents: 'none'
        }}>
          {text}
          {/* Arrow */}
          <div style={{
            position: 'absolute',
            top: '100%',
            left: '50%',
            transform: 'translateX(-50%)',
            borderWidth: '5px',
            borderStyle: 'solid',
            borderColor: 'rgba(0, 0, 0, 0.85) transparent transparent transparent'
          }} />
        </div>
      )}
    </div>
  );
};
