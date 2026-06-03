"use client";

import { useEffect } from "react";

export function OfflineSyncHandler() {
  useEffect(() => {
    if ("serviceWorker" in navigator) {
      window.addEventListener("load", function () {
        navigator.serviceWorker.register("/sw.js").then(
          function (registration) {
            console.log(
              "ServiceWorker registration successful with scope: ",
              registration.scope
            );
          },
          function (err) {
            console.log("ServiceWorker registration failed: ", err);
          }
        );
      });
    }

    const handleOnline = async () => {
      console.log("Back online. Triggering background sync...");
      if ("serviceWorker" in navigator && "SyncManager" in window) {
        try {
          const registration = await navigator.serviceWorker.ready;
          await (registration as any).sync.register("ohc-offline-sync");
        } catch (err) {
          console.error("Failed to register background sync", err);
        }
      }
    };

    // Listen for messages from service worker
    const handleMessage = (event: MessageEvent) => {
      if (event.data && event.data.type === 'SYNC_COMPLETE') {
         window.dispatchEvent(new Event('offline-queue-updated'));
      }
    };
    navigator.serviceWorker?.addEventListener('message', handleMessage);

    // Listen to push notification event specifically for E2E tests
    const handlePushNotification = (event: any) => {
        const notif = document.createElement('div');
        notif.id = 'push-notification-banner';
        notif.innerText = event.detail?.title + ' ' + event.detail?.body;
        // Basic styling
        notif.style.position = 'fixed';
        notif.style.top = '10px';
        notif.style.left = '50%';
        notif.style.transform = 'translateX(-50%)';
        notif.style.backgroundColor = '#4caf50';
        notif.style.color = 'white';
        notif.style.padding = '16px';
        notif.style.borderRadius = '8px';
        notif.style.zIndex = '9999';
        document.body.appendChild(notif);

        setTimeout(() => {
            if (document.body.contains(notif)) {
                 document.body.removeChild(notif);
            }
        }, 5000);
    }

    window.addEventListener("online", handleOnline);
    window.addEventListener("push-notification", handlePushNotification);

    // Initial check in case it's online
    handleOnline();

    return () => {
      window.removeEventListener("online", handleOnline);
      window.removeEventListener("push-notification", handlePushNotification);
      navigator.serviceWorker?.removeEventListener('message', handleMessage);
    };
  }, []);

  return null;
}
