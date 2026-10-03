"use client";

import React, { useState, useEffect } from "react";
import { AppShell } from "../../../app/components/AppShell";

interface Task {
  id: string;
  title: string;
  status: string;
}

async function readTaskReceipt(response: Response): Promise<Record<string, unknown>> {
  if (response.status !== 200) throw new Error("Task request was not completed");
  const receipt: unknown = await response.json();
  if (!receipt || typeof receipt !== "object" || Array.isArray(receipt)
    || ("success" in receipt && receipt.success !== true) || "error" in receipt) {
    throw new Error("Invalid task acknowledgement");
  }
  return receipt as Record<string, unknown>;
}

async function readTaskMutationReceipt(response: Response): Promise<void> {
  const receipt = await readTaskReceipt(response);
  if (receipt.success !== true) throw new Error("Task mutation was not acknowledged");
}

export default function TasksPage() {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState("");
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
    const controller = new AbortController();
    fetch("/api/v1/staff/tasks", { signal: controller.signal })
      .then((res) => {
        if (!res.ok) throw new Error("Tasks unavailable");
        return res.json();
      })
      .then((data) => {
        if (!Array.isArray(data.tasks)) throw new Error("Invalid task list");
        if (!controller.signal.aborted) setTasks(data.tasks);
      })
      .catch(() => {
        if (!controller.signal.aborted) setError("Could not load tasks. Please reload to try again.");
      })
      .finally(() => { if (!controller.signal.aborted) setLoading(false); });
    return () => controller.abort();
  }, []);

  const handleOpenNew = () => {
    setNewTitle("");
    setNewError("");
    setIsNewModalOpen(true);
  };

  const handleSaveNew = async () => {
    if (saving) return;
    if (!newTitle.trim()) {
      setNewError("Title is required");
      return;
    }
    const title = newTitle.trim();
    setSaving(true);
    setNewError("");
    try {
      const response = await fetch("/api/v1/staff/tasks", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ title, staff_id: "", description: "", priority: "normal" }),
      });
      const data = await readTaskReceipt(response);
      if (typeof data.id !== "string" || !data.id.trim()) throw new Error("Invalid task ID");
      const id = data.id;
      setTasks((prev) => [{ id, title, status: "pending" }, ...prev]);
      setIsNewModalOpen(false);
      setNewTitle("");
    } catch {
      setNewError("Could not save task. Please try again.");
    } finally { setSaving(false); }
  };

  const handleOpenEdit = () => {
    if (!selectedTask) return;
    setEditTitle(selectedTask.title);
    setEditError("");
    setIsEditModalOpen(true);
  };

  const handleSaveEdit = async () => {
    if (saving) return;
    if (!editTitle.trim()) {
      setEditError("Title is required");
      return;
    }
    if (!selectedTask) return;
    const id = selectedTask.id;
    const title = editTitle.trim();
    setSaving(true);
    setEditError("");
    try {
      const response = await fetch(`/api/v1/staff/tasks/${encodeURIComponent(id)}`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ title }),
      });
      await readTaskMutationReceipt(response);
      setTasks((prev) => prev.map((task) => task.id === id ? { ...task, title } : task));
      setSelectedTask((prev) => prev?.id === id ? { ...prev, title } : prev);
      setIsEditModalOpen(false);
      setEditTitle("");
    } catch {
      setEditError("Could not save task. Please try again.");
    } finally { setSaving(false); }
  };

  const handleDelete = async () => {
    if (!selectedTask || saving) return;
    const id = selectedTask.id;
    setSaving(true);
    setError("");
    try {
      const response = await fetch(`/api/v1/staff/tasks/${encodeURIComponent(id)}`, { method: "DELETE" });
      await readTaskMutationReceipt(response);
      setTasks((prev) => prev.filter((task) => task.id !== id));
      setSelectedTask(null);
    } catch {
      setError("Could not delete task. Please try again.");
    } finally { setSaving(false); }
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
            disabled={loading || saving}
            className="app-button primary px-5 py-2.5 rounded-xl font-semibold shadow-sm"
          >
            New Task
          </button>
        </div>

        {error && <p role="alert" className="text-red-600 mb-4">{error}</p>}
        {loading && <p role="status">Loading tasks...</p>}
        {!loading && !error && tasks.length === 0 && <p>No tasks yet.</p>}

        {selectedTask && (
          <div className="mb-6 p-4 bg-indigo-50/70 border border-indigo-200 rounded-xl flex items-center justify-between">
            <div>
              <span className="text-xs uppercase font-bold text-indigo-700">Selected Task</span>
              <p className="font-semibold text-gray-900">{selectedTask.title}</p>
            </div>
            <div className="flex items-center gap-2">
              <button
                onClick={handleOpenEdit}
                disabled={saving}
                className="px-4 py-2 bg-white border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 font-medium"
              >
                Edit
              </button>
              <button
                onClick={handleDelete}
                disabled={saving}
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
                  <p role="alert" className="mt-2 text-sm text-red-600 font-medium">{newError}</p>
                )}
              </div>
              <div className="flex justify-end gap-2">
                <button
                  type="button"
                  onClick={() => setIsNewModalOpen(false)}
                  disabled={saving}
                  className="px-4 py-2 text-gray-600 rounded-lg hover:bg-gray-100 font-medium"
                >
                  Cancel
                </button>
                <button
                  type="button"
                  onClick={handleSaveNew}
                  disabled={saving}
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
                  <p role="alert" className="mt-2 text-sm text-red-600 font-medium">{editError}</p>
                )}
              </div>
              <div className="flex justify-end gap-2">
                <button
                  type="button"
                  onClick={() => setIsEditModalOpen(false)}
                  disabled={saving}
                  className="px-4 py-2 text-gray-600 rounded-lg hover:bg-gray-100 font-medium"
                >
                  Cancel
                </button>
                <button
                  type="button"
                  onClick={handleSaveEdit}
                  disabled={saving}
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
