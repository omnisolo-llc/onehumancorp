'use client';

import React, { useState, useEffect } from 'react';

export default function AgentProtocolPage() {
  const [tasks, setTasks] = useState<any[]>([]);
  const [taskInput, setTaskInput] = useState('');
  const [selectedTaskId, setSelectedTaskId] = useState('');
  const [stepInput, setStepInput] = useState('');
  const [steps, setSteps] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchTasks = async () => {
    try {
      const res = await fetch('/api/agents/protocol?method=ap_list_tasks');
      if (!res.ok) throw new Error('Failed to fetch tasks');
      const data = await res.json();
      setTasks(data.tasks || []);
    } catch (e: any) {
      setError(e.message);
    }
  };

  const createTask = async () => {
    if (!taskInput) return;
    setLoading(true);
    try {
      const res = await fetch('/api/agents/protocol', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ method: 'ap_create_task', params: { input: taskInput } }),
      });
      if (!res.ok) throw new Error('Failed to create task');
      await fetchTasks();
      setTaskInput('');
    } catch (e: any) {
      setError(e.message);
    } finally {
      setLoading(false);
    }
  };

  const fetchSteps = async (taskId: string) => {
    try {
      const res = await fetch(`/api/agents/protocol?method=ap_list_steps&task_id=${taskId}`);
      if (!res.ok) throw new Error('Failed to fetch steps');
      const data = await res.json();
      setSteps(data.steps || []);
    } catch (e: any) {
      setError(e.message);
    }
  };

  const executeStep = async () => {
    if (!selectedTaskId) return;
    setLoading(true);
    try {
      const res = await fetch('/api/agents/protocol', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          method: 'ap_execute_step',
          params: { task_id: selectedTaskId, input: stepInput }
        }),
      });
      if (!res.ok) throw new Error('Failed to execute step');
      await fetchSteps(selectedTaskId);
      setStepInput('');
    } catch (e: any) {
      setError(e.message);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchTasks();
  }, []);

  useEffect(() => {
    if (selectedTaskId) {
      fetchSteps(selectedTaskId);
    } else {
      setSteps([]);
    }
  }, [selectedTaskId]);

  return (
    <div className="max-w-6xl mx-auto p-8">
      <h1 className="text-3xl font-bold mb-4">Agent Protocol UI</h1>
      <p className="text-gray-600 mb-8">
        Interact with the standardized Agent Protocol (AutoGPT Unique Harness Innovations).
      </p>

      {error && (
        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative mb-4">
          <span className="block sm:inline">{error}</span>
          <button className="absolute top-0 bottom-0 right-0 px-4 py-3" onClick={() => setError(null)}>
            <span className="text-2xl">&times;</span>
          </button>
        </div>
      )}

      <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
        <div className="glassmorphism bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 p-6 shadow-sm">
          <h2 className="text-xl font-bold mb-4">Tasks</h2>

          <div className="flex space-x-2 mb-6">
            <input
              type="text"
              placeholder="New Task Input..."
              value={taskInput}
              onChange={(e) => setTaskInput(e.target.value)}
              className="flex-1 border border-gray-300 rounded-md px-3 py-2 bg-white/65 backdrop-blur-[30px] saturate-[210%]"
            />
            <button
              onClick={createTask}
              disabled={loading}
              className="bg-[#0071E3] text-white px-4 py-2 rounded-md hover:bg-blue-700 disabled:opacity-50"
            >
              Create
            </button>
          </div>

          <ul className="space-y-2">
            {tasks.map((task) => (
              <li
                key={task.task_id}
                className={`p-3 border rounded-md cursor-pointer transition ${selectedTaskId === task.task_id ? 'border-[#0066FF] bg-blue-50' : 'border-gray-200 hover:bg-gray-50'}`}
                onClick={() => setSelectedTaskId(task.task_id)}
              >
                <div className="font-semibold">{task.input || 'Untitled Task'}</div>
                <div className="text-xs text-gray-500 truncate">{task.task_id}</div>
              </li>
            ))}
            {tasks.length === 0 && <div className="text-gray-500 text-sm italic">No tasks found.</div>}
          </ul>
        </div>

        <div className="glassmorphism bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 p-6 shadow-sm">
          <h2 className="text-xl font-bold mb-4">Steps</h2>
          {!selectedTaskId ? (
            <div className="text-gray-500 text-sm italic">Select a task to view its steps.</div>
          ) : (
            <>
              <div className="flex space-x-2 mb-6">
                <input
                  type="text"
                  placeholder="Optional Step Input..."
                  value={stepInput}
                  onChange={(e) => setStepInput(e.target.value)}
                  className="flex-1 border border-gray-300 rounded-md px-3 py-2 bg-white/65 backdrop-blur-[30px] saturate-[210%]"
                />
                <button
                  onClick={executeStep}
                  disabled={loading}
                  className="bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700 disabled:opacity-50"
                >
                  Execute Step
                </button>
              </div>

              <ul className="space-y-4">
                {steps.map((step, idx) => (
                  <li key={step.step_id} className="p-4 border rounded-md border-gray-200">
                    <div className="flex justify-between mb-2">
                      <span className="font-bold text-sm">Step {idx + 1}</span>
                      <span className={`text-xs px-2 py-1 rounded-full ${step.status === 'completed' ? 'bg-green-100 text-green-800' : 'bg-yellow-100 text-yellow-800'}`}>
                        {step.status}
                      </span>
                    </div>
                    {step.input && <div className="text-sm mb-1"><span className="font-medium">Input:</span> {step.input}</div>}
                    {step.output && <div className="text-sm text-gray-700 mt-2 bg-gray-50 p-2 rounded whitespace-pre-wrap">{step.output}</div>}
                  </li>
                ))}
                {steps.length === 0 && <div className="text-gray-500 text-sm italic">No steps executed yet.</div>}
              </ul>
            </>
          )}
        </div>
      </div>
    </div>
  );
}
