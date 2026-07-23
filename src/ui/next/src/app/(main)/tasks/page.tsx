"use client";

import { useState } from "react";

export default function StaffTaskView() {
  const [tasks, setTasks] = useState([
    { id: 1, description: "Restock front shelf", status: "Pending", priority: "High" },
    { id: 2, description: "Wipe down counters", status: "Pending", priority: "Medium" }
  ]);

  const toggleTask = (id: number) => {
    setTasks(tasks.map(t => t.id === id ? { ...t, status: t.status === "Pending" ? "Completed" : "Pending" } : t));
  };

  return (
    <div className="p-4 max-w-[375px] mx-auto min-h-screen bg-gray-50/50 backdrop-blur-md pb-24">
      <h1 className="text-2xl font-semibold tracking-tight text-gray-900 mb-6">My Tasks</h1>

      <div className="space-y-4">
        {tasks.map((task) => (
          <div
            key={task.id}
            className={`flex items-start gap-4 p-5 bg-white/80 backdrop-blur-md shadow-sm border border-gray-100/50 rounded-2xl transition-all duration-200 cursor-pointer ${task.status === 'Completed' ? 'opacity-60' : 'hover:shadow-md'}`}
            onClick={() => toggleTask(task.id)}
          >
            <div className="mt-0.5">
              <input
                type="checkbox"
                className="w-5 h-5 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                checked={task.status === "Completed"}
                readOnly
              />
            </div>
            <div className="flex-1">
              <p className={`text-base font-medium ${task.status === 'Completed' ? 'text-gray-400 line-through' : 'text-gray-900'}`}>{task.description}</p>
              <div className="flex items-center gap-2 mt-2">
                <span className={`text-xs font-medium px-2 py-0.5 rounded border ${task.priority === 'High' ? 'bg-red-50 text-red-700 border-red-100' : 'bg-orange-50 text-orange-700 border-orange-100'}`}>
                  {task.priority} Priority
                </span>
                <span className="text-xs text-gray-500">Auto-assigned</span>
              </div>
            </div>
          </div>
        ))}
      </div>

      <button className="mt-8 w-full bg-blue-600 text-white rounded-xl py-3.5 px-4 font-medium shadow-sm hover:bg-blue-700 active:scale-95 transition-all text-[15px]">
        Flag Issue to Manager
      </button>
    </div>
  );
}
