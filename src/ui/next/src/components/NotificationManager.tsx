"use client";

import React, { useEffect, useState } from 'react';

interface AppNotification {
  id: string;
  message: string;
  type?: 'info' | 'success' | 'warning' | 'error';
}

export function NotificationManager() {
  const [notifications, setNotifications] = useState<AppNotification[]>([]);

  useEffect(() => {
    const handleEventReceived = (event: Event) => {
      const customEvent = event as CustomEvent;
      let payloadData = customEvent.detail;

      if (typeof payloadData === 'string') {
        try {
          payloadData = JSON.parse(payloadData);
        } catch (e) {
          // ignore
        }
      }

      // Check if it's a tenant event with a message
      const message = payloadData?.message || payloadData?.event || (typeof payloadData === 'string' ? payloadData : "New event received");

      const newNotification: AppNotification = {
        id: Date.now().toString() + Math.random().toString(),
        message,
        type: payloadData?.type || 'info',
      };

      setNotifications((prev) => [...prev, newNotification]);

      // Remove after 5 seconds
      setTimeout(() => {
        setNotifications((prev) => prev.filter((n) => n.id !== newNotification.id));
      }, 5000);
    };

    window.addEventListener('ohc_event_received', handleEventReceived);

    return () => {
      window.removeEventListener('ohc_event_received', handleEventReceived);
    };
  }, []);

  if (notifications.length === 0) return null;

  return (
    <div className="fixed top-4 right-4 z-50 flex flex-col gap-2">
      {notifications.map((notification) => (
        <div
          key={notification.id}
          className="bg-[rgba(255,255,255,0.03)] backdrop-blur-[20px] backdrop-saturate-[200%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[8px] p-4 shadow-lg text-sm text-[#1D1D1F] dark:text-[#F5F5F7] animate-in slide-in-from-top-2 fade-in duration-300 pointer-events-auto"
          data-testid="notification-toast"
        >
          {notification.message}
        </div>
      ))}
    </div>
  );
}
