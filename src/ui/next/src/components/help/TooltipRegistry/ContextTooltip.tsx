import React, { useState } from 'react';
import tooltipData from '../../../../../../docs/help_center/tooltip_registry.json';

interface TooltipProps {
    category: keyof typeof tooltipData;
    itemKey: string;
    children: React.ReactNode;
}

export const ContextTooltip: React.FC<TooltipProps> = ({ category, itemKey, children }) => {
    const [isVisible, setIsVisible] = useState(false);

    // Safely retrieve tooltip text
    const categoryData = tooltipData[category] as Record<string, string> | undefined;
    const tooltipText = categoryData ? categoryData[itemKey] : null;

    if (!tooltipText) {
        return <>{children}</>;
    }

    return (
        <div
            style={{ position: 'relative', display: 'inline-block' }}
            onMouseEnter={() => setIsVisible(true)}
            onMouseLeave={() => setIsVisible(false)}
            onFocus={() => setIsVisible(true)}
            onBlur={() => setIsVisible(false)}
            onTouchStart={() => setIsVisible(true)}
            tabIndex={0}
        >
            {children}
            {isVisible && (
                <div style={{
                    position: 'absolute',
                    bottom: '100%',
                    left: '50%',
                    transform: 'translateX(-50%)',
                    marginBottom: '8px',
                    padding: '8px 12px',
                    backgroundColor: '#111',
                    color: 'white',
                    fontSize: '13px',
                    fontFamily: 'Inter, sans-serif',
                    borderRadius: '6px',
                    whiteSpace: 'nowrap',
                    zIndex: 10000,
                    boxShadow: '0 4px 6px rgba(0,0,0,0.1)',
                    pointerEvents: 'none'
                }}>
                    {tooltipText}
                    <div style={{
                        position: 'absolute',
                        top: '100%',
                        left: '50%',
                        transform: 'translateX(-50%)',
                        borderWidth: '5px',
                        borderStyle: 'solid',
                        borderColor: '#111 transparent transparent transparent'
                    }} />
                </div>
            )}
        </div>
    );
};
