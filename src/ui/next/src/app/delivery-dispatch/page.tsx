"use client";

import { useState, useEffect } from "react";
import { AppShell } from "../components/AppShell";
import Link from "next/link";

export default function DeliveryDispatchPage() {
  const [tasks, setTasks] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchItinerary = async () => {
      try {
        const response = await fetch("/api/delivery/itinerary");
        if (!response.ok) {
           throw new Error("Failed to fetch itinerary");
        }
        const data = await response.json();
        setTasks(data.tasks || []);
      } catch (e: any) {
         console.error("Error loading delivery dispatch", e);
         setError(e.message || "Failed to load delivery manifest.");

      } finally {
        setLoading(false);
      }
    };
    fetchItinerary();
  }, []);

  const updateTaskStatus = async (id: string, newStatus: string) => {
      // Optimistic UI update
      setTasks(tasks.map((t: any) => t.id === id ? { ...t, status: newStatus } : t));

      try {
        await fetch("/api/delivery/update-status", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ taskId: id, status: newStatus })
        });
      } catch (e) {
        console.error("Failed to update status", e);
      }
  };

  return (
    <AppShell title="Delivery Dispatch">
      <main className="p-4 max-w-lg mx-auto w-full">
        <div className="flex items-center gap-3 mb-6">
          <Link href="/dashboard" className="text-indigo-600 hover:text-indigo-800">
            &larr; Back
          </Link>
          <h1 className="text-2xl font-bold font-outfit">Local Delivery Manifest</h1>
        </div>

        {loading ? (
          <div className="text-center py-10">Loading deliveries...</div>
        ) : (
          <div className="space-y-4">
            {error && <div className="p-3 bg-red-100 text-red-800 rounded-lg text-sm">{error} (Showing fallback data)</div>}

            {tasks.map((task: any) => (
              <div key={task.id} className="mac-glass-container p-4 rounded-[16px] border border-white/40 shadow-sm relative">
                <div className="flex justify-between items-start mb-2">
                  <h3 className="font-bold text-lg">{task.order_id}</h3>
                  <span className={`px-2 py-1 rounded-full text-xs font-semibold ${task.status === 'PENDING' ? 'bg-yellow-100 text-yellow-800' : task.status === 'IN_TRANSIT' ? 'bg-blue-100 text-blue-800' : 'bg-green-100 text-green-800'}`}>
                    {task.status.replace('_', ' ')}
                  </span>
                </div>
                <div className="text-sm text-gray-600 mb-4">
                  <p className="flex items-center gap-2 mb-1">
                    <span className="font-medium">ETA:</span> {new Date(task.estimated_arrival_unix * 1000).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                  </p>
                  <p className="flex items-center gap-2">
                    <span className="font-medium">Location:</span> {task.delivery_location_lat?.toFixed(4) ?? 0}, {task.delivery_location_lng?.toFixed(4) ?? 0}
                  </p>
                </div>

                {task.status !== 'DELIVERED' && (
                  <div className="flex gap-2">
                     <button className="flex-1 bg-indigo-50 text-indigo-700 hover:bg-indigo-100 border border-indigo-200 py-2 rounded-lg text-sm font-medium transition-colors">
                       Map View
                     </button>
                     {task.status === 'PENDING' && (
                       <button onClick={() => updateTaskStatus(task.id, 'IN_TRANSIT')} className="flex-1 bg-indigo-600 hover:bg-indigo-700 text-white py-2 rounded-lg text-sm font-medium transition-colors shadow-sm">
                         Start Delivery
                       </button>
                     )}
                     {task.status === 'IN_TRANSIT' && (
                       <button onClick={() => updateTaskStatus(task.id, 'DELIVERED')} className="flex-1 bg-[#34C759] hover:bg-green-600 text-white py-2 rounded-lg text-sm font-medium transition-colors shadow-sm">
                         Mark Delivered
                       </button>
                     )}
                  </div>
                )}
              </div>
            ))}
            {tasks.length === 0 && (
              <div className="text-center py-10 text-gray-500 bg-gray-50 rounded-[16px] border border-gray-200 border-dashed">
                <p className="text-lg mb-2">🎉</p>
                <p>No local deliveries scheduled for today.</p>
              </div>
            )}
          </div>
        )}
      </main>
    </AppShell>
  );
}
