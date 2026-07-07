
"use client";

import React, { useState, useEffect } from 'react';
import { AppShell } from '@/app/components/AppShell';

export default function StaffPage() {
  const [shifts, setShifts] = useState([]);
  const [tasks, setTasks] = useState([]);
  const [loading, setLoading] = useState(true);
  const [isEscalating, setIsEscalating] = useState(false);

  const fetchData = async () => {
    try {
      setLoading(true);
      // In a real implementation we might fetch shifts from /api/staff/shifts
      setShifts([
        { id: '1', role: 'Baker', startTime: new Date().toISOString(), endTime: new Date(Date.now() + 8*3600000).toISOString(), status: 'Scheduled' }
      ]);
      const res = await fetch('/api/staff/tasks');
      if (res.ok) {
        const data = await res.json();
        setTasks(data.tasks || []);
      }
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
  }, []);

  const completeTask = async (id: string, isCompleted: boolean) => {
    try {
      const status = isCompleted ? 'completed' : 'pending';
      const res = await fetch(`/api/staff/tasks/${id}/complete`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ status })
      });
      if (res.ok) {
        setTasks(tasks.map((t: any) => t.id === id ? { ...t, status } : t));
      }
    } catch (e) {
      console.error("Failed to update task", e);
    }
  };

  const handleEscalation = async () => {
    setIsEscalating(true);
    try {
      // Send an intent to dynamically create a task using the Operations Agent
      await fetch('/api/staff/tasks', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ description: "URGENT: Low Supply (Cups)", priority: 1 })
      });
      fetchData(); // Refresh tasks after escalation
    } catch (e) {
      console.error("Failed to escalate", e);
    } finally {
      setIsEscalating(false);
    }
  };

  return (
    <AppShell title="My Shifts & Tasks">
      <div className="max-w-[375px] mx-auto min-h-screen bg-gray-50 pb-20">
        <header className="px-4 py-6 bg-white border-b border-gray-200">
          <h1 className="text-2xl font-bold text-gray-900">My Shifts & Tasks</h1>
        </header>

        <main className="p-4 space-y-6">
          <section>
            <h2 className="text-lg font-semibold text-gray-800 mb-3">Upcoming Shifts</h2>
            <div className="space-y-3">
              {shifts.map((shift: any) => (
                <div key={shift.id} className="bg-white/65 backdrop-blur-[30px] p-4 rounded-xl shadow-sm border border-white/40">
                  <div className="flex justify-between items-start mb-2">
                    <span className="font-medium text-gray-900">{shift.role}</span>
                    <span className="text-xs px-2 py-1 bg-blue-50 text-blue-700 rounded-full">{shift.status}</span>
                  </div>
                  <div className="text-sm text-gray-600">
                    {new Date(shift.startTime).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})} -
                    {new Date(shift.endTime).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})}
                  </div>
                  <button className="mt-3 w-full py-2 px-4 bg-gray-100 text-gray-700 rounded-lg text-sm font-medium hover:bg-gray-200 min-h-[44px]">
                    Request Swap
                  </button>
                </div>
              ))}
            </div>
          </section>

          <section>
            <div className="flex justify-between items-center mb-3">
              <h2 className="text-lg font-semibold text-gray-800">My Tasks</h2>
              <button
                onClick={handleEscalation}
                disabled={isEscalating}
                className="text-xs font-semibold px-3 py-1 bg-red-100 text-red-700 rounded-full hover:bg-red-200"
              >
                {isEscalating ? 'Escalating...' : 'Flag Low Supply'}
              </button>
            </div>

            {loading ? (
               <div className="text-center py-4"><div className="animate-spin rounded-full h-6 w-6 border-b-2 border-gray-900 mx-auto"></div></div>
            ) : tasks.length === 0 ? (
               <div className="text-center py-4 text-gray-500 text-sm">No pending tasks.</div>
            ) : (
              <div className="space-y-3">
                {tasks.map((task: any) => (
                  <div key={task.id} className="bg-white/65 backdrop-blur-[30px] p-4 rounded-xl shadow-sm border border-white/40 flex items-center justify-between">
                    <span className={`text-gray-800 ${task.status === 'completed' ? 'line-through text-gray-400' : ''}`}>{task.description}</span>
                    <input
                      type="checkbox"
                      className="h-6 w-6 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                      checked={task.status === 'completed'}
                      onChange={(e) => completeTask(task.id, e.target.checked)}
                    />
                  </div>
                ))}
              </div>
            )}
          </section>
        </main>
      </div>
    </AppShell>
  );
}
