import React from 'react';

export type Position = 'top' | 'bottom' | 'left' | 'right';

export function calculateBubbleStyle(targetRect: DOMRect | null, position: Position = 'bottom', margin: number = 16) {
  let bubbleStyle: React.CSSProperties = {};
  let arrowClass = "";

  if (targetRect) {
    switch (position) {
      case 'bottom':
        bubbleStyle = {
          top: targetRect.bottom + margin,
          left: targetRect.left + (targetRect.width / 2),
          transform: 'translateX(-50%)'
        };
        arrowClass = "bottom-full left-1/2 -translate-x-1/2 border-b-white/90 border-x-transparent border-t-0 border-8";
        break;
      case 'top':
        bubbleStyle = {
          top: targetRect.top - margin,
          left: targetRect.left + (targetRect.width / 2),
          transform: 'translate(-50%, -100%)'
        };
        arrowClass = "top-full left-1/2 -translate-x-1/2 border-t-white/90 border-x-transparent border-b-0 border-8";
        break;
      case 'right':
         bubbleStyle = {
          top: targetRect.top + (targetRect.height / 2),
          left: targetRect.right + margin,
          transform: 'translateY(-50%)'
        };
        arrowClass = "right-full top-1/2 -translate-y-1/2 border-r-white/90 border-y-transparent border-l-0 border-8";
        break;
      case 'left':
         bubbleStyle = {
          top: targetRect.top + (targetRect.height / 2),
          left: targetRect.left - margin,
          transform: 'translate(-100%, -50%)'
        };
        arrowClass = "left-full top-1/2 -translate-y-1/2 border-l-white/90 border-y-transparent border-r-0 border-8";
        break;
    }
  }

  return { bubbleStyle, arrowClass };
}
