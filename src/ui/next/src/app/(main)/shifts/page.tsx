"use client";

import { useState } from "react";

export default function ManagerDashboard() {
  const [shifts, setShifts] = useState([
    { id: 1, startTime: "08:00 AM", endTime: "04:00 PM", status: "Active" }
  ]);
  const [tasks, setTasks] = useState([
    { id: 1, description: "Restock Inventory", status: "Pending" },
    { id: 2, description: "Clean Floor", status: "Completed" }
  ]);

  return (
    <div className="p-4 max-w-[375px] mx-auto min-h-screen bg-gray-50/50 backdrop-blur-md pb-24">
      <h1 className="text-2xl font-semibold tracking-tight text-gray-900 mb-6">Manager Dashboard</h1>

      <section className="mb-8">
        <h2 className="text-xl font-medium text-gray-800 mb-4">Current Shift</h2>
        {shifts.map((shift) => (
          <div key={shift.id} className="bg-white/80 backdrop-blur-md shadow-sm border border-gray-100/50 rounded-xl p-5">
             <div className="flex justify-between items-center">
                <span className="text-gray-600 text-sm">{shift.startTime} - {shift.endTime}</span>
                <span className="inline-flex items-center rounded-full bg-green-50 px-2.5 py-0.5 text-xs font-medium text-green-700">{shift.status}</span>
             </div>
             <div className="mt-4 text-sm text-gray-500">
                <p><strong>AI Summary:</strong> Shift is progressing well. High customer volume expected.</p>
             </div>
          </div>
        ))}
      </section>

      <section>
        <h2 className="text-xl font-medium text-gray-800 mb-4">Unresolved Tasks</h2>
        <div className="space-y-3">
          {tasks.map((task) => (
            <div key={task.id} className="flex items-center justify-between bg-white/80 backdrop-blur-md shadow-sm border border-gray-100/50 rounded-xl p-4">
              <span className={`text-sm ${task.status === 'Completed' ? 'line-through text-gray-400' : 'text-gray-700'}`}>{task.description}</span>
              <span className={`text-xs font-medium px-2.5 py-0.5 rounded-full ${task.status === 'Completed' ? 'bg-gray-100 text-gray-600' : 'bg-yellow-50 text-yellow-700'}`}>{task.status}</span>
            </div>
          ))}
        </div>
      </section>
    </div>
  );
}
