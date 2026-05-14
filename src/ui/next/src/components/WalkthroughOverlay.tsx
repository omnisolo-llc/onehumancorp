'use client';
import React, { useEffect, useState } from 'react';
import { createPortal } from 'react-dom';

interface WalkthroughOverlayProps {
  isActive: boolean;
  targetSelector: string;
  title: string;
  content: string;
  onDismiss: () => void;
  onNext: () => void;
}

export const WalkthroughOverlay: React.FC<WalkthroughOverlayProps> = ({ isActive, targetSelector, title, content, onDismiss, onNext }) => {
  const [targetRect, setTargetRect] = useState<DOMRect | null>(null);

  useEffect(() => {
    if (!isActive) return;

    const findTarget = () => {
      const el = document.querySelector(targetSelector);
      if (el) {
        setTargetRect(el.getBoundingClientRect());
      }
    };

    findTarget();
    window.addEventListener('resize', findTarget);

    // Fallback polling for dynamically rendered elements
    const intervalId = setInterval(findTarget, 1000);

    return () => {
      window.removeEventListener('resize', findTarget);
      clearInterval(intervalId);
    };
  }, [isActive, targetSelector]);

  if (!isActive || !targetRect) return null;

  return createPortal(
    <div style={{
      position: 'fixed',
      top: 0, left: 0, right: 0, bottom: 0,
      zIndex: 9998,
      pointerEvents: 'none'
    }}>
      {/* Dimmed Background with cutout */}
      <div style={{
        position: 'absolute',
        top: targetRect.top - 4,
        left: targetRect.left - 4,
        width: targetRect.width + 8,
        height: targetRect.height + 8,
        boxShadow: '0 0 0 9999px rgba(0, 0, 0, 0.5)',
        borderRadius: '4px',
        pointerEvents: 'auto',
        transition: 'all 0.3s ease-in-out'
      }} />

      {/* Speech Bubble */}
      <div style={{
        position: 'absolute',
        top: targetRect.bottom + 16,
        left: targetRect.left,
        backgroundColor: '#fff',
        padding: '20px',
        borderRadius: '8px',
        boxShadow: '0 4px 20px rgba(0,0,0,0.15)',
        width: '300px',
        pointerEvents: 'auto',
        fontFamily: 'Inter, sans-serif'
      }}>
        <h3 style={{ margin: '0 0 8px 0', fontSize: '16px', color: '#111' }}>{title}</h3>
        <p style={{ margin: '0 0 16px 0', fontSize: '14px', color: '#444', lineHeight: 1.5 }}>{content}</p>
        <div style={{ display: 'flex', justifyContent: 'space-between' }}>
          <button onClick={onDismiss} style={{ background: 'none', border: 'none', color: '#666', cursor: 'pointer', padding: '4px 8px' }}>Skip Tour</button>
          <button onClick={onNext} style={{ background: '#0070f3', border: 'none', color: '#fff', borderRadius: '4px', cursor: 'pointer', padding: '6px 12px', fontWeight: 'bold' }}>Next Step →</button>
        </div>
      </div>
    </div>,
    document.body
  );
};
