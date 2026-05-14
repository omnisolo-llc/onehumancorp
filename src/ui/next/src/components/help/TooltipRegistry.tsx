"use client";
import React, { useState, useEffect } from 'react';
import { createPortal } from 'react-dom';

export default function TooltipRegistry() {
    const [tooltips, setTooltips] = useState<{id: string, element_selector: string, text: string}[]>([]);
    const [activeTooltip, setActiveTooltip] = useState<{id: string, text: string, top: number, left: number} | null>(null);

    useEffect(() => {
        fetch('/api/help/tooltips')
            .then(res => res.json())
            .then(data => setTooltips(data))
            .catch(() => {});
    }, []);

    useEffect(() => {
        if (tooltips.length === 0 || typeof document === 'undefined') return;

        let pressTimer: NodeJS.Timeout;

        const handleInteraction = (e: Event) => {
            const target = e.target as HTMLElement;
            if (!target) return;

            for (const t of tooltips) {
                if (target.matches(t.element_selector) || target.closest(t.element_selector)) {
                    const el = target.matches(t.element_selector) ? target : target.closest(t.element_selector)!;
                    const rect = el.getBoundingClientRect();
                    setActiveTooltip({
                        id: t.id,
                        text: t.text,
                        top: rect.bottom + window.scrollY + 5,
                        left: rect.left + window.scrollX
                    });
                    return;
                }
            }
        };

        const handleHide = () => {
            setActiveTooltip(null);
            clearTimeout(pressTimer);
        };

        const handleTouchStart = (e: TouchEvent) => {
            pressTimer = setTimeout(() => handleInteraction(e), 500);
        };

        document.addEventListener('mouseover', handleInteraction);
        document.addEventListener('mouseout', handleHide);
        document.addEventListener('touchstart', handleTouchStart);
        document.addEventListener('touchend', handleHide);

        return () => {
            document.removeEventListener('mouseover', handleInteraction);
            document.removeEventListener('mouseout', handleHide);
            document.removeEventListener('touchstart', handleTouchStart);
            document.removeEventListener('touchend', handleHide);
            clearTimeout(pressTimer);
        };
    }, [tooltips]);

    if (!activeTooltip || typeof document === 'undefined') return null;

    return createPortal(
        <div
            className="ohc-custom-tooltip"
            style={{
                position: 'absolute',
                top: activeTooltip.top,
                left: activeTooltip.left,
                background: 'rgba(0,0,0,0.8)',
                color: '#fff',
                padding: '8px',
                borderRadius: '4px',
                zIndex: 99999,
                pointerEvents: 'none'
            }}
        >
            {activeTooltip.text}
        </div>,
        document.body
    );
}
