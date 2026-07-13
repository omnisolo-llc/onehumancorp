"use client";

import React, { useState, useEffect } from 'react';
import { AppShell } from '@/app/components/AppShell';

export default function ManagerDashboard() {
  const [shifts, setShifts] = useState([]);
  const [escalations, setEscalations] = useState([]);
  const [tasks, setTasks] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchData() {
      try {
        const [shiftsRes, escalationsRes, tasksRes] = await Promise.all([
          fetch('/api/staff/shifts'),
          fetch('/api/staff/escalations'),
          fetch('/api/staff/tasks')
        ]);

        if (shiftsRes.ok) {
          const data = await shiftsRes.json();
          setShifts(data.shifts || []);
        }
        if (escalationsRes.ok) {
          const data = await escalationsRes.json();
          setEscalations(data.escalations || []);
        }
        if (tasksRes.ok) {
          const data = await tasksRes.json();
          setTasks(data.tasks || []);
        }
      } catch (err) {
        console.error("Error fetching manager data:", err);
      } finally {
        setLoading(false);
      }
    }
    fetchData();
  }, []);

  const simulateEvent = async () => {
    try {
      await fetch('/api/staff/simulate-event', { method: 'POST' });
      const res = await fetch('/api/staff/tasks');
      if (res.ok) {
        const data = await res.json();
        setTasks(data.tasks || []);
      }
    } catch (err) {
      console.error(err);
    }
  };

  const generateSummary = async () => {
    try {
      await fetch('/api/staff/generate-summary', { method: 'POST' });
      alert("Shift Summary Generated!");
    } catch (err) {
      console.error(err);
    }
  };

  return (
    <AppShell title="Manager Dashboard">
      <div className="max-w-[1440px] mx-auto min-h-screen p-6">
        <header className="mb-8">
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Manager View (Jun)</h1>
          <p className="text-gray-500 mt-2">Manage daily operations, shifts, and handle escalations.</p>
        </header>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
          <button onClick={simulateEvent} className="bg-blue-600 text-white p-4 rounded-[12px] shadow hover:bg-blue-700">
            Simulate Business Event (Inventory Low)
          </button>
          <button onClick={generateSummary} className="bg-emerald-600 text-white p-4 rounded-[12px] shadow hover:bg-emerald-700">
            Generate End of Shift Summary
          </button>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          <section className="bg-white/65 backdrop-blur-[30px] border border-white/40 dark:bg-[#16161a]/70 dark:border-white/10 p-6 rounded-[16px]">
            <h2 className="text-xl font-bold mb-4">Active Shifts</h2>
            {loading ? <p>Loading...</p> : (
              <div className="space-y-4">
                {shifts.map((shift: any) => (
                  <div key={shift.id} className="p-4 bg-gray-50 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700">
                    <p className="font-semibold">{shift.role} - Staff ID: {shift.staff_id}</p>
                    <p className="text-sm text-gray-600 dark:text-gray-400">Status: {shift.status}</p>
                  </div>
                ))}
                {shifts.length === 0 && <p className="text-gray-500">No active shifts.</p>}
              </div>
            )}
          </section>

          <section className="bg-white/65 backdrop-blur-[30px] border border-white/40 dark:bg-[#16161a]/70 dark:border-white/10 p-6 rounded-[16px]">
            <h2 className="text-xl font-bold mb-4">Attention Needed (Escalations)</h2>
             {loading ? <p>Loading...</p> : (
              <div className="space-y-4">
                {escalations.map((esc: any) => (
                  <div key={esc.id} className="p-4 bg-red-50 dark:bg-red-900/20 rounded-lg border border-red-200 dark:border-red-800">
                    <p className="font-semibold text-red-700 dark:text-red-400">{esc.summary}</p>
                    <p className="text-sm text-red-600 dark:text-red-500">Status: {esc.status}</p>
                  </div>
                ))}
                {escalations.length === 0 && <p className="text-gray-500">No active escalations.</p>}
              </div>
            )}
          </section>
        </div>

        <div className="mt-8">
           <section className="bg-white/65 backdrop-blur-[30px] border border-white/40 dark:bg-[#16161a]/70 dark:border-white/10 p-6 rounded-[16px]">
            <h2 className="text-xl font-bold mb-4">Staff Tasks</h2>
            {loading ? <p>Loading...</p> : (
              <div className="space-y-4">
                {tasks.map((task: any) => (
                  <div key={task.id} className="p-4 bg-gray-50 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 flex justify-between">
                    <div>
                      <p className="font-semibold">{task.title || task.description}</p>
                      <p className="text-sm text-gray-600 dark:text-gray-400">Status: {task.status}</p>
                    </div>
                  </div>
                ))}
                {tasks.length === 0 && <p className="text-gray-500">No active tasks.</p>}
              </div>
            )}
          </section>
        </div>
      </div>
    </AppShell>
  );
}
