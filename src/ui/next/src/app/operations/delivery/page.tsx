"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function DriverDispatchApp() {
  const router = useRouter();
  const [tasks, setTasks] = useState([
    { id: '1', address: '123 Main St', status: 'PENDING' },
    { id: '2', address: '456 Market St', status: 'PENDING' },
  ]);

  const handleStartRoute = () => {
    alert('Route started. Navigation initiated.');
  };

  const markDelivered = (id) => {
    setTasks(tasks.map(t => t.id === id ? { ...t, status: 'DELIVERED' } : t));
    alert('Task marked as Delivered.');
  };

  return (
    <div className="min-h-screen bg-gray-50 font-inter">
      <header className="p-4 bg-white shadow-sm sticky top-0 flex justify-between items-center z-10">
        <h1 className="text-xl font-bold font-outfit text-gray-900">Today's Route</h1>
        <button
          onClick={handleStartRoute}
          id="start-route-btn"
          className="bg-indigo-600 text-white px-4 py-2 rounded-full font-medium"
        >
          Start
        </button>
      </header>

      <main className="p-4 flex flex-col gap-4">
        {tasks.map(task => (
          <div key={task.id} className="bg-white p-4 rounded-xl shadow-sm border border-gray-100 flex flex-col gap-3">
            <div className="flex justify-between items-start">
              <div className="flex flex-col">
                <span className="font-semibold text-gray-900">Stop #{task.id}</span>
                <span className="text-gray-600 text-sm">{task.address}</span>
              </div>
              <span className={`px-2 py-1 text-xs rounded-full font-medium ${task.status === 'DELIVERED' ? 'bg-green-100 text-green-700' : 'bg-yellow-100 text-yellow-700'}`}>
                {task.status}
              </span>
            </div>

            {task.status !== 'DELIVERED' && (
              <div className="flex gap-2 mt-2">
                <button
                  onClick={() => markDelivered(task.id)}
                  id={`mark-delivered-${task.id}`}
                  className="flex-1 bg-gray-900 text-white py-3 rounded-lg font-medium text-sm"
                >
                  Mark Delivered
                </button>
                <button className="flex-1 bg-gray-100 text-gray-800 py-3 rounded-lg font-medium text-sm">
                  Call
                </button>
              </div>
            )}
          </div>
        ))}
      </main>
    </div>
  );
}
