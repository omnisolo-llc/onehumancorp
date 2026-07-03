"use client";

import React, { useState, useEffect } from 'react';
import { AppShell } from '@/components/AppShell';

export default function StaffPage() {
  const [shifts, setShifts] = useState([]);
  const [tasks, setTasks] = useState([]);

  useEffect(() => {
    // In a real implementation, we would fetch the staff member's shifts and tasks
    setShifts([
      { id: '1', role: 'Baker', startTime: new Date().toISOString(), endTime: new Date(Date.now() + 8*3600000).toISOString(), status: 'Scheduled' }
    ]);
    setTasks([
      { id: '1', description: 'Fulfill 5 cake orders', status: 'Pending' }
    ]);
  }, []);

  return (
    <AppShell>
      <div className="max-w-[375px] mx-auto min-h-screen bg-gray-50 pb-20">
        <header className="px-4 py-6 bg-white border-b border-gray-200">
          <h1 className="text-2xl font-bold text-gray-900">My Shifts & Tasks</h1>
        </header>

        <main className="p-4 space-y-6">
          <section>
            <h2 className="text-lg font-semibold text-gray-800 mb-3">Upcoming Shifts</h2>
            <div className="space-y-3">
              {shifts.map((shift: any) => (
                <div key={shift.id} className="bg-white p-4 rounded-xl shadow-sm border border-gray-100">
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
            <h2 className="text-lg font-semibold text-gray-800 mb-3">My Tasks</h2>
            <div className="space-y-3">
              {tasks.map((task: any) => (
                <div key={task.id} className="bg-white p-4 rounded-xl shadow-sm border border-gray-100 flex items-center justify-between">
                  <span className="text-gray-800">{task.description}</span>
                  <input type="checkbox" className="h-6 w-6 rounded border-gray-300 text-blue-600 focus:ring-blue-500" />
                </div>
              ))}
            </div>
          </section>
        </main>
      </div>
    </AppShell>
  );
}
