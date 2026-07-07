'use client';
import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';

interface Task {
  id: string;
  title: string;
  priority: string;
  status: string;
  created_at: string;
}

export function StaffDashboard() {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [loading, setLoading] = useState(true);

  const fetchTasks = async () => {
    try {
      const res = await fetch('/api/proxy/staff/tasks', {
        headers: { 'x-spiffe-id': 'spiffe://ohc/org/demo/agent/ui' }
      });
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
    fetchTasks();
  }, []);

  const completeTask = async (id: string) => {
    try {
      await fetch('/api/proxy/staff/tasks/sync', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'x-spiffe-id': 'spiffe://ohc/org/demo/agent/ui'
        },
        body: JSON.stringify({
          mutations: [{ id, action: 'complete' }]
        })
      });
      setTasks(tasks.filter(t => t.id !== id));
    } catch (e) {
      console.error(e);
    }
  };

  const flagLowSupply = async () => {
    try {
      await fetch('/api/proxy/staff/tasks/sync', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'x-spiffe-id': 'spiffe://ohc/org/demo/agent/ui'
        },
        body: JSON.stringify({
          mutations: [{ id: 'new', action: 'create_alert', title: 'Low Cups' }]
        })
      });
      fetchTasks();
    } catch (e) {
      console.error(e);
    }
  };

  if (loading) {
    return <div className="text-center py-10 text-gray-500">Loading tasks...</div>;
  }

  return (
    <div className="space-y-6">
      <div className="bg-white/80 backdrop-blur-lg border border-gray-200/50 rounded-2xl p-4 shadow-sm">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-lg font-medium text-gray-800">Current Tasks</h2>
          <span className="bg-blue-100 text-blue-800 text-xs px-2 py-1 rounded-full font-medium">
            {tasks.length} Active
          </span>
        </div>

        <div className="space-y-3">
          <AnimatePresence>
            {tasks.length === 0 ? (
              <motion.div initial={{ opacity: 0 }} animate={{ opacity: 1 }} className="text-gray-400 text-sm py-4 text-center">
                No active tasks. You're all caught up!
              </motion.div>
            ) : (
              tasks.map(task => (
                <motion.div
                  key={task.id}
                  initial={{ opacity: 0, y: 10 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, scale: 0.95 }}
                  className="bg-white border border-gray-100 p-4 rounded-xl shadow-sm flex flex-col gap-3"
                >
                  <div className="flex justify-between items-start">
                    <span className="font-medium text-gray-900">{task.title}</span>
                    {task.priority === 'urgent' && (
                      <span className="text-xs text-red-600 bg-red-50 px-2 py-0.5 rounded font-semibold border border-red-100">URGENT</span>
                    )}
                    {task.priority === 'high' && (
                      <span className="text-xs text-orange-600 bg-orange-50 px-2 py-0.5 rounded font-semibold border border-orange-100">HIGH</span>
                    )}
                  </div>
                  <div className="flex justify-end gap-2 mt-2">
                    <button
                      onClick={() => completeTask(task.id)}
                      className="px-4 py-2 bg-black text-white text-sm font-medium rounded-lg hover:bg-gray-800 active:scale-95 transition-all"
                    >
                      Mark Complete
                    </button>
                  </div>
                </motion.div>
              ))
            )}
          </AnimatePresence>
        </div>
      </div>

      <div className="bg-white/80 backdrop-blur-lg border border-gray-200/50 rounded-2xl p-4 shadow-sm flex flex-col gap-3">
        <h2 className="text-lg font-medium text-gray-800">Alerts & Actions</h2>
        <button
          onClick={flagLowSupply}
          className="w-full py-3 bg-red-50 text-red-700 font-semibold rounded-xl border border-red-100 hover:bg-red-100 active:scale-95 transition-all"
        >
          Flag Low Supply
        </button>
      </div>
    </div>
  );
}
