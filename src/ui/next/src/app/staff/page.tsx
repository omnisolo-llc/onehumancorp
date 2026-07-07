"use client";

import React, { useState, useEffect } from 'react';
import { AppShell } from '@/app/components/AppShell';

interface Task {
  id: string;
  description: string;
  status: string;
  priority: string;
}

interface Shift {
  id: string;
  role: string;
  startTime: string;
  endTime: string;
  status: string;
}

export default function StaffPage() {
  const [shifts, setShifts] = useState<Shift[]>([]);
  const [tasks, setTasks] = useState<Task[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');

  const fetchTasks = async () => {
    try {
      const res = await fetch('/api/staff/tasks');
      if (res.ok) {
        const data = await res.json();
        setTasks(data);
      } else {
        console.error('Failed to fetch tasks');
      }
    } catch (e) {
      console.error('Error fetching tasks', e);
    }
  };

  const fetchShifts = async () => {
    try {
      const res = await fetch('/api/staff/shifts'); // Assuming a shifts endpoint would exist
      if (res.ok) {
        const data = await res.json();
        setShifts(data);
      } else {
        console.error('Failed to fetch shifts');
      }
    } catch (e) {
      console.error('Error fetching shifts', e);
    }
  };

  useEffect(() => {
    const init = async () => {
      setLoading(true);
      await fetchTasks();
      await fetchShifts();
      setLoading(false);
    };
    init();
  }, []);

  const handleTaskToggle = async (task: Task) => {
    const newStatus = task.status === 'COMPLETED' ? 'PENDING' : 'COMPLETED';

    // Optimistic update
    setTasks(prev => prev.map(t => t.id === task.id ? { ...t, status: newStatus } : t));

    try {
      const res = await fetch(`/api/staff/tasks/${task.id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ status: newStatus }),
      });
      if (!res.ok) {
        // Revert on failure
        setTasks(prev => prev.map(t => t.id === task.id ? { ...t, status: task.status } : t));
        setError('Failed to update task');
      }
    } catch (e) {
      // Revert on error
      setTasks(prev => prev.map(t => t.id === task.id ? { ...t, status: task.status } : t));
      setError('Error updating task');
    }
  };

  if (loading) {
    return (
      <AppShell title="My Shifts & Tasks">
        <div className="flex items-center justify-center min-h-screen">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
        </div>
      </AppShell>
    );
  }

  return (
    <AppShell title="My Shifts & Tasks">
      <div className="max-w-[375px] mx-auto min-h-screen bg-gray-50/50 pb-20 relative">
        {/* macOS Translucent Glass Header */}
        <header className="sticky top-0 z-10 px-4 py-6 bg-white/65 backdrop-blur-[30px] saturate-[210%] border-b border-white/40 shadow-sm">
          <h1 className="text-2xl font-bold text-gray-900 tracking-tight">My Shifts & Tasks</h1>
        </header>

        <main className="p-4 space-y-6">
          {error && (
            <div className="p-3 mb-4 text-sm text-red-800 rounded-lg bg-red-50 border border-red-100">
              {error}
            </div>
          )}

          <section>
            <h2 className="text-lg font-semibold text-gray-800 mb-3 tracking-tight">Active Shift</h2>
            {shifts.length === 0 ? (
               <div className="text-center py-10 bg-white/50 rounded-2xl border border-dashed border-gray-200">
                 <p className="text-gray-500 text-sm font-medium">No shifts scheduled.</p>
               </div>
            ) : (
            <div className="space-y-3">
              {shifts.map((shift: any) => (
                <div key={shift.id} className="bg-white/80 backdrop-blur-md p-4 rounded-2xl shadow-sm border border-gray-100/50 transition-all duration-200 hover:shadow-md">
                  <div className="flex justify-between items-start mb-2">
                    <span className="font-semibold text-gray-900">{shift.role}</span>
                    <span className="text-[11px] font-medium px-2.5 py-1 bg-blue-50/80 text-blue-600 rounded-full border border-blue-100/50 uppercase tracking-wide">
                      {shift.status}
                    </span>
                  </div>
                  <div className="text-sm text-gray-500 font-medium mb-4">
                    {new Date(shift.startTime).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})} -
                    {new Date(shift.endTime).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})}
                  </div>
                  <div className="grid grid-cols-2 gap-2">
                    <button className="py-2.5 px-4 bg-gray-100/80 text-gray-700 rounded-xl text-sm font-semibold hover:bg-gray-200/80 active:bg-gray-300 transition-colors min-h-[44px]">
                      Request Swap
                    </button>
                    <button className="py-2.5 px-4 bg-red-50 text-red-600 rounded-xl text-sm font-semibold hover:bg-red-100 transition-colors min-h-[44px] border border-red-100/50">
                      Escalate Issue
                    </button>
                  </div>
                </div>
              ))}
            </div>
            )}
          </section>

          <section>
            <div className="flex justify-between items-end mb-3">
              <h2 className="text-lg font-semibold text-gray-800 tracking-tight">Action Items</h2>
              <span className="text-xs font-medium text-gray-500 bg-gray-100 px-2 py-1 rounded-md">{tasks.filter(t => t.status === 'PENDING').length} Remaining</span>
            </div>

            {tasks.length === 0 ? (
               <div className="text-center py-10 bg-white/50 rounded-2xl border border-dashed border-gray-200">
                 <p className="text-gray-500 text-sm font-medium">All caught up! No tasks assigned.</p>
               </div>
            ) : (
              <div className="space-y-3">
                {tasks.map((task: Task) => (
                  <div
                    key={task.id}
                    className={`p-4 rounded-2xl shadow-sm border transition-all duration-300 flex gap-4 ${
                      task.status === 'COMPLETED'
                        ? 'bg-gray-50 border-transparent opacity-60'
                        : task.priority === 'HIGH'
                          ? 'bg-orange-50/50 border-orange-200/50 shadow-orange-100/20'
                          : 'bg-white border-gray-100/80'
                    }`}
                  >
                    <div className="flex-shrink-0 pt-0.5">
                      <div className="relative flex items-center justify-center">
                        <input
                          type="checkbox"
                          checked={task.status === 'COMPLETED'}
                          onChange={() => handleTaskToggle(task)}
                          className={`peer h-6 w-6 cursor-pointer appearance-none rounded-full border-2 transition-all ${
                            task.status === 'COMPLETED'
                              ? 'border-green-500 bg-green-500'
                              : 'border-gray-300 hover:border-blue-500 bg-white'
                          }`}
                        />
                        <svg className={`pointer-events-none absolute h-3.5 w-3.5 text-white opacity-0 peer-checked:opacity-100 transition-opacity`} viewBox="0 0 14 14" fill="none" stroke="currentColor" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round">
                          <polyline points="2.5 7.5 5.5 10.5 11.5 3.5"></polyline>
                        </svg>
                      </div>
                    </div>
                    <div className="flex-1 min-w-0">
                      <p className={`text-[15px] font-medium leading-snug tracking-tight ${
                        task.status === 'COMPLETED' ? 'text-gray-400 line-through' : 'text-gray-800'
                      }`}>
                        {task.description}
                      </p>
                      {task.priority === 'HIGH' && task.status !== 'COMPLETED' && (
                        <p className="text-xs font-semibold text-orange-600 mt-1.5 uppercase tracking-wider">
                          Priority Action
                        </p>
                      )}
                    </div>
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
