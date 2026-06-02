"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { WithTooltip } from "@/components/WithTooltip";

export default function DeliveryDispatchPage() {
  const router = useRouter();
  const [routeStarted, setRouteStarted] = useState(false);
  const [tasks, setTasks] = useState([
    { id: "task_1", customer: "John Doe", address: "123 Main St", status: "PENDING" },
    { id: "task_2", customer: "Jane Smith", address: "456 Oak Ave", status: "PENDING" }
  ]);

  const startRoute = () => {
    setRouteStarted(true);
    const updatedTasks = tasks.map(t => ({ ...t, status: "IN_TRANSIT" }));
    setTasks(updatedTasks);
  };

  const markDelivered = (id: string) => {
    const updatedTasks = tasks.map(t => t.id === id ? { ...t, status: "DELIVERED" } : t);
    setTasks(updatedTasks);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Today's Route</h1>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-lg mx-auto w-full flex flex-col gap-6">
        <div className="flex justify-between items-center">
          <p className="text-gray-700">Delivery Itinerary</p>
          {!routeStarted && (
             <WithTooltip id="start-route-tooltip" defaultText="Start your local delivery route now.">
               <button onClick={startRoute} className="px-4 py-2 bg-indigo-600 text-white rounded-lg shadow-sm font-medium hover:bg-indigo-700">Start Route</button>
             </WithTooltip>
          )}
        </div>

        <div className="flex flex-col gap-4">
          {tasks.map(task => (
            <div key={task.id} className="p-4 shadow-sm flex flex-col gap-2" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
              <div className="flex justify-between items-start">
                <div>
                  <h3 className="font-bold text-gray-900">{task.customer}</h3>
                  <p className="text-sm text-gray-600">{task.address}</p>
                </div>
                <span className={`text-xs px-2 py-1 rounded-full font-semibold ${task.status === 'DELIVERED' ? 'bg-green-100 text-green-800' : task.status === 'IN_TRANSIT' ? 'bg-blue-100 text-blue-800' : 'bg-gray-100 text-gray-800'}`}>
                  {task.status}
                </span>
              </div>

              {task.status === "IN_TRANSIT" && (
                <WithTooltip id={`mark-delivered-${task.id}-tooltip`} defaultText="Mark this order as delivered.">
                  <button onClick={() => markDelivered(task.id)} className="w-full mt-2 px-4 py-2 bg-green-600 text-white rounded-lg font-medium hover:bg-green-700 transition-colors">
                    Mark Delivered
                  </button>
                </WithTooltip>
              )}
            </div>
          ))}
        </div>
      </main>
    </div>
  );
}
