"use client";
import { useEffect, useState } from "react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { AppShell } from "../components/AppShell";
import { UnifiedAgentFeed } from "./UnifiedAgentFeed";

export default function Dashboard() {
  const router = useRouter();
  const [userName, setUserName] = useState("Owner");
  const [isOffline, setIsOffline] = useState(false);
  const [offlineQueueCount, setOfflineQueueCount] = useState(0);

  const tenantId = () => {
    if (typeof window === "undefined") return "default";
    return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
  };

  useEffect(() => {
    let mounted = true;
    const name = localStorage.getItem("user_name");
    if (name && mounted) setUserName(name);

    function updateOfflineStatus() {
      if (!mounted) return;
      setIsOffline(!navigator.onLine);
      try {
        const queue = JSON.parse(localStorage.getItem('offline_payment_queue') || '[]');
        setOfflineQueueCount(queue.length);
      } catch (e) {
        setOfflineQueueCount(0);
      }
    }

    updateOfflineStatus();
    window.addEventListener("online", updateOfflineStatus);
    window.addEventListener("offline", updateOfflineStatus);
    window.addEventListener("storage", updateOfflineStatus);

    return () => {
      mounted = false;
      window.removeEventListener("online", updateOfflineStatus);
      window.removeEventListener("offline", updateOfflineStatus);
      window.removeEventListener("storage", updateOfflineStatus);
    };
  }, []);

  const statusItems = [
    { label: "Growth", value: "Active", tone: "good" as const },
  ];

  return (
    <AppShell
      title="Dashboard"
      subtitle="Unified Agent Feed"
      statusItems={statusItems}
      actions={[
        { label: "New Product", href: "/products/new", primary: true },
      ]}
    >
      <div className="mb-4 flex flex-wrap gap-2">
        <div id="network-status-indicator" className={isOffline ? "app-badge warn block" : "hidden"} style={{ display: isOffline ? 'block' : 'none' }}>
          Offline - changes saved locally
        </div>
        <div id="queue-dashboard" className={offlineQueueCount > 0 ? "app-badge warn block" : "hidden"}>
          {offlineQueueCount} Payments Pending Sync
        </div>
      </div>

      <main id="dashboard-screen" className="flex flex-col items-center justify-center w-full" style={{ gap: 16 }}>
        <UnifiedAgentFeed />
      </main>
    </AppShell>
  );
}