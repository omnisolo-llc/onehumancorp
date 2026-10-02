'use client';

import { useCallback, useEffect, useRef, useState } from 'react';

type ClipboardState = 'idle' | 'pending' | 'copied' | 'error';

/** Acknowledge platform completion only while the displayed content is current. */
export function useClipboardFeedback(contentKey: string) {
  const [state, setState] = useState<ClipboardState>('idle');
  const generation = useRef(0);
  useEffect(() => {
    generation.current++;
    setState('idle');
    return () => { generation.current++; };
  }, [contentKey]);

  const copy = useCallback(async (content: string, options: { html?: boolean } = {}) => {
    const operation = ++generation.current;
    setState('pending');
    try {
      if (!content || !navigator.clipboard) throw new Error('Clipboard unavailable');
      const clipboard = navigator.clipboard;
      if (options.html && typeof ClipboardItem !== 'undefined' && typeof clipboard.write === 'function') {
        try {
          await clipboard.write([new ClipboardItem({
            'text/html': new Blob([content], { type: 'text/html' }),
          })]);
        } catch (error) {
          if (error instanceof DOMException && error.name === 'NotAllowedError') throw error;
          if (operation !== generation.current) return false;
          await clipboard.writeText(content);
        }
      } else {
        await clipboard.writeText(content);
      }
      if (operation !== generation.current) return false;
      setState('copied');
      return true;
    } catch {
      if (operation === generation.current) setState('error');
      return false;
    }
  }, []);

  const message = state === 'pending' ? 'Copying…'
    : state === 'copied' ? 'Copied to clipboard.'
      : state === 'error' ? 'Copy failed. Select the content and copy it manually.' : '';
  return { state, message, copy };
}
