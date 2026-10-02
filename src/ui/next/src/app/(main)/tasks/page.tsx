"use client";

import React, { useState, useEffect } from "react";
import { AppShell } from "../../../app/components/AppShell";

interface Task {
  id: string;
  title: string;
  status: string;
}

export default function TasksPage() {
  const [tasks, setTasks] = useState<Task[]>([
    { id: "1", title: "Restock front shelf", status: "Pending" },
    { id: "2", title: "Wipe down counters", status: "Pending" },
  ]);
  const [selectedTask, setSelectedTask] = useState<Task | null>(null);

  // New task modal state
  const [isNewModalOpen, setIsNewModalOpen] = useState(false);
  const [newTitle, setNewTitle] = useState("");
  const [newError, setNewError] = useState("");

  // Edit task modal state
  const [isEditModalOpen, setIsEditModalOpen] = useState(false);
  const [editTitle, setEditTitle] = useState("");
  const [editError, setEditError] = useState("");

  useEffect(() => {
    fetch("/api/v1/tasks")
      .then((res) => (res.ok ? res.json() : null))
      .then((data) => {
        if (Array.isArray(data) && data.length > 0) {
          setTasks(data);
        }
      })
      .catch(() => {
        // keep fallback tasks
      });
  }, []);

  const handleOpenNew = () => {
    setNewTitle("");
    setNewError("");
    setIsNewModalOpen(true);
  };

  const handleSaveNew = () => {
    if (!newTitle.trim()) {
      setNewError("Title is required");
      return;
    }
    const newTask: Task = {
      id: `task-${Date.now()}`,
      title: newTitle.trim(),
      status: "Pending",
    };
    setTasks((prev) => [newTask, ...prev]);
    setIsNewModalOpen(false);
    setNewTitle("");
    setNewError("");
  };

  const handleOpenEdit = () => {
    if (!selectedTask) return;
    setEditTitle(selectedTask.title);
    setEditError("");
    setIsEditModalOpen(true);
  };

  const handleSaveEdit = () => {
    if (!editTitle.trim()) {
      setEditError("Title is required");
      return;
    }
    if (!selectedTask) return;
    setTasks((prev) =>
      prev.map((t) => (t.id === selectedTask.id ? { ...t, title: editTitle.trim() } : t))
    );
    setSelectedTask((prev) => (prev ? { ...prev, title: editTitle.trim() } : null));
    setIsEditModalOpen(false);
    setEditTitle("");
    setEditError("");
  };

  const handleDelete = () => {
    if (!selectedTask) return;
    setTasks((prev) => prev.filter((t) => t.id !== selectedTask.id));
    setSelectedTask(null);
  };

  return (
    <AppShell
      title="Tasks"
      subtitle="Manage, delegate, and inspect workflow jobs."
    >
      <div className="max-w-4xl mx-auto p-6 font-sans">
        <div className="flex justify-between items-center mb-6">
          <h1 className="text-3xl font-bold font-outfit text-gray-900">Tasks</h1>
          <button
            onClick={handleOpenNew}
            className="app-button primary px-5 py-2.5 rounded-xl font-semibold shadow-sm"
          >
            New Task
          </button>
        </div>

        {selectedTask && (
          <div className="mb-6 p-4 bg-indigo-50/70 border border-indigo-200 rounded-xl flex items-center justify-between">
            <div>
              <span className="text-xs uppercase font-bold text-indigo-700">Selected Task</span>
              <p className="font-semibold text-gray-900">{selectedTask.title}</p>
            </div>
            <div className="flex items-center gap-2">
              <button
                onClick={handleOpenEdit}
                className="px-4 py-2 bg-white border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 font-medium"
              >
                Edit
              </button>
              <button
                onClick={handleDelete}
                className="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 font-medium shadow-sm"
              >
                Delete
              </button>
            </div>
          </div>
        )}

        <ul id="task-list" className="space-y-3">
          {tasks.map((task) => (
            <li
              key={task.id}
              onClick={() => setSelectedTask(task)}
              className={`p-4 rounded-xl border transition-all cursor-pointer flex justify-between items-center ${
                selectedTask?.id === task.id
                  ? "bg-indigo-50/40 border-indigo-400 shadow-sm"
                  : "bg-white border-gray-200 hover:border-gray-300 hover:shadow-xs"
              }`}
            >
              <div className="flex items-center gap-3">
                <span className="text-gray-400">📋</span>
                <span className="font-medium text-gray-900">{task.title}</span>
              </div>
              <span className="text-xs font-semibold px-2.5 py-1 rounded-full bg-gray-100 text-gray-700">
                {task.status}
              </span>
            </li>
          ))}
        </ul>

        {/* New Task Modal */}
        {isNewModalOpen && (
          <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm">
            <div className="bg-white rounded-2xl p-6 max-w-md w-full shadow-2xl border border-gray-100">
              <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">New Task</h2>
              <div className="mb-4">
                <label htmlFor="new-task-title" className="block text-sm font-semibold text-gray-700 mb-1">
                  Title
                </label>
                <input
                  id="new-task-title"
                  aria-label="Title"
                  type="text"
                  value={newTitle}
                  onChange={(e) => {
                    setNewTitle(e.target.value);
                    if (newError) setNewError("");
                  }}
                  className="w-full border rounded-lg p-2.5 text-gray-900 focus:outline-none focus:ring-2 focus:ring-indigo-500"
                  placeholder="Enter task title"
                />
                {newError && (
                  <p className="mt-2 text-sm text-red-600 font-medium">Title is required</p>
                )}
              </div>
              <div className="flex justify-end gap-2">
                <button
                  type="button"
                  onClick={() => setIsNewModalOpen(false)}
                  className="px-4 py-2 text-gray-600 rounded-lg hover:bg-gray-100 font-medium"
                >
                  Cancel
                </button>
                <button
                  type="button"
                  onClick={handleSaveNew}
                  className="px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 font-medium shadow-sm"
                >
                  Save
                </button>
              </div>
            </div>
          </div>
        )}

        {/* Edit Task Modal */}
        {isEditModalOpen && (
          <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm">
            <div className="bg-white rounded-2xl p-6 max-w-md w-full shadow-2xl border border-gray-100">
              <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">Edit Task</h2>
              <div className="mb-4">
                <label htmlFor="edit-task-title" className="block text-sm font-semibold text-gray-700 mb-1">
                  Title
                </label>
                <input
                  id="edit-task-title"
                  aria-label="Title"
                  type="text"
                  value={editTitle}
                  onChange={(e) => {
                    setEditTitle(e.target.value);
                    if (editError) setEditError("");
                  }}
                  className="w-full border rounded-lg p-2.5 text-gray-900 focus:outline-none focus:ring-2 focus:ring-indigo-500"
                  placeholder="Enter task title"
                />
                {editError && (
                  <p className="mt-2 text-sm text-red-600 font-medium">Title is required</p>
                )}
              </div>
              <div className="flex justify-end gap-2">
                <button
                  type="button"
                  onClick={() => setIsEditModalOpen(false)}
                  className="px-4 py-2 text-gray-600 rounded-lg hover:bg-gray-100 font-medium"
                >
                  Cancel
                </button>
                <button
                  type="button"
                  onClick={handleSaveEdit}
                  className="px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 font-medium shadow-sm"
                >
                  Save Changes
                </button>
              </div>
            </div>
          </div>
        )}
      </div>
    </AppShell>
  );
}
