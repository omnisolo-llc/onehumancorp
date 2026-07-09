"use client";

import React, { useState, useEffect } from 'react';
import { useSyncGateway } from '../hooks/useSyncGateway';
import { WithTooltip } from './TooltipRegistry';

export function RealTimeNotifications() {
  const [toastMessage, setToastMessage] = useState<string | null>(null);

  const { lastMessage } = useSyncGateway({
    topics: ['tenant_events'],
    enabled: true,
  });

  useEffect(() => {
    if (lastMessage) {
      if (lastMessage.type === 'notification' && lastMessage.message) {
        setToastMessage(lastMessage.message);
        setTimeout(() => setToastMessage(null), 5000);
      } else if (lastMessage.message) {
        setToastMessage(lastMessage.message);
        setTimeout(() => setToastMessage(null), 5000);
      } else if (lastMessage.data && lastMessage.data.message) {
        setToastMessage(lastMessage.data.message);
        setTimeout(() => setToastMessage(null), 5000);
      } else if (typeof lastMessage === 'string') {
        setToastMessage(lastMessage);
        setTimeout(() => setToastMessage(null), 5000);
      }
    }
  }, [lastMessage]);

  if (!toastMessage) return null;

  return (
    <div
      className="fixed bottom-4 left-1/2 transform -translate-x-1/2 bg-white/65 dark:bg-[#16161a]/70 border border-white/40 dark:border-white/10 text-gray-800 dark:text-gray-100 px-6 py-4 rounded-xl shadow-lg z-[9999] flex items-start gap-3 max-w-md w-full backdrop-blur-[30px] backdrop-saturate-[2.1] animate-in slide-in-from-bottom-5"
      role="alert"
      aria-live="polite"
    >
      <span className="text-xl">🔔</span>
      <div className="flex-1">
        <h3 className="font-semibold text-sm">New Notification</h3>
        <p className="text-sm mt-1 leading-relaxed">{toastMessage}</p>
      </div>
      <WithTooltip id="notification-close-tooltip" defaultText="Dismiss this notification.">
        <button
          onClick={() => setToastMessage(null)}
          className="text-gray-500 hover:text-gray-700 dark:hover:text-gray-300 transition-colors p-1"
          aria-label="Close notification"
        >
          ✕
        </button>
      </WithTooltip>
    </div>
  );
}
