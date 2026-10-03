'use client';

import { errorMessage } from '@/lib/errors';

import { useState,useEffect,useRef,useCallback } from 'react';
import { useTenantAnalysis } from '../agents/useTenantAnalysis';
import { RecordedTextAnalysis } from './RecordedTextAnalysis';

type ProtocolTask = { task_id: string; input?: string };
type ProtocolStep = { step_id: string; status: string; input?: string; output?: string };
type ProtocolCheckpoint = { checkpoint_id: string; created_at: string };

export default function AgentProtocolPage() {
  const [scope, setScope] = useState(0);
  const retire = useCallback(() => setScope(value => value + 1), []);
  const execution = useTenantAnalysis(retire);
  return <main className="max-w-6xl mx-auto p-8 font-sans">
    <h1 className="text-3xl font-bold mb-4">Agent Protocol UI</h1>
    <RecordedTextAnalysis key={`analysis-${scope}`} execution={execution} />
    <WorkspaceRuntime key={`runtime-${scope}`} />
  </main>;
}

function WorkspaceRuntime() {
  const [tasks, setTasks] = useState<ProtocolTask[]>([]);
  const [taskInput, setTaskInput] = useState('');
  const [selectedTaskId, setSelectedTaskId] = useState('');
  const [stepInput, setStepInput] = useState('');
  const [steps, setSteps] = useState<ProtocolStep[]>([]);
  const [checkpoints, setCheckpoints] = useState<ProtocolCheckpoint[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [unknownCreation, setUnknownCreation] = useState<string | null>(null);
  const [reviewStatus, setReviewStatus] = useState('');
  const creating = useRef(false);
  const [taskRead, setTaskRead] = useState('unverified');

  const fetchTasks = async () => {
    setTaskRead('loading');
    try {
      const res = await fetch('/api/v1/agents/protocol?method=ap_list_tasks');
      if (!res.ok) throw new Error('Failed to fetch tasks');
      const data = await res.json();
      if (!data || !Array.isArray(data.tasks) || data.error != null || data.success === false) throw new Error('The task list could not be verified');
      setTasks(data.tasks); setTaskRead('ready'); return true;
    } catch (e: unknown) {
      setTaskRead('unavailable'); setError(errorMessage(e)); return false;
    }
  };

  const createTask = async () => {
    if (!taskInput.trim() || loading || creating.current || unknownCreation !== null) return;
    const submittedInput = taskInput;
    creating.current = true; setLoading(true); setError(null);
    let unconfirmed = false;
    try {
      unconfirmed = true;
      const res = await fetch('/api/v1/agents/protocol', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ method: 'ap_create_task', params: { input: submittedInput } }),
      });
      if (res.status === 401 || res.status === 403) {
        unconfirmed = false;
        throw new Error('Task creation was rejected by authentication or permission checks. Your input is retained.');
      }
      if (!res.ok) throw new Error('Failed to create task');
      const data = await res.json();
      if (res.status !== 200 || !data || typeof data !== 'object' || Array.isArray(data) || data.error != null || ('success' in data && data.success !== true) || typeof data.task_id !== 'string' || !data.task_id.trim()) throw new Error('Task creation was not acknowledged. Your input is retained.');
      unconfirmed = false;
      setTasks((prev) => {
        if (prev.some((t) => t.task_id === data.task_id)) return prev;
        return [data, ...prev];
      });
      setSelectedTaskId(data.task_id);
      await fetchTasks();
      setTaskInput(current => current === submittedInput ? '' : current);
    } catch (e: unknown) {
      if (unconfirmed) setUnknownCreation(submittedInput);
      setError(errorMessage(e));
    } finally {
      creating.current = false; setLoading(false);
    }
  };

  const reviewExistingTasks = async () => {
    setReviewStatus('');
    if (await fetchTasks()) setReviewStatus('The current task list is loaded. It does not prove whether the unconfirmed request was saved.');
  };

  const fetchSteps = async (taskId: string) => {
    try {
      const res = await fetch(`/api/v1/agents/protocol?method=ap_list_steps&task_id=${taskId}`);
      if (!res.ok) throw new Error('Failed to fetch steps');
      const data = await res.json();
      setSteps(data.steps || []);
    } catch (e: unknown) {
      setError(errorMessage(e));
    }
  };


  const fetchCheckpoints = async (taskId: string) => {
    try {
      const res = await fetch(`/api/v1/agents/protocol?method=ap_list_checkpoints&task_id=${taskId}`);
      if (!res.ok) throw new Error('Failed to fetch checkpoints');
      const data = await res.json();
      setCheckpoints(data.checkpoints || []);
    } catch (e: unknown) {
      console.error(e);
      setCheckpoints([]);
    }
  };

  const restoreCheckpoint = async (taskId: string, checkpointId: string) => {
    setLoading(true);
    try {
      const res = await fetch('/api/v1/agents/protocol', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          method: 'ap_restore_checkpoint',
          params: { task_id: taskId, checkpoint_id: checkpointId }
        }),
      });
      if (!res.ok) throw new Error('Failed to restore checkpoint');
      await fetchSteps(taskId);
      await fetchCheckpoints(taskId);
    } catch (e: unknown) {
      setError(errorMessage(e));
    } finally {
      setLoading(false);
    }
  };

  const executeStep = async () => {
    if (!selectedTaskId) return;
    setLoading(true);
    try {
      const res = await fetch('/api/v1/agents/protocol', {
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
    } catch (e: unknown) {
      setError(errorMessage(e));
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    if (selectedTaskId) {
      fetchSteps(selectedTaskId);
      fetchCheckpoints(selectedTaskId);
    } else {
      setSteps([]);
      setCheckpoints([]);
    }
  }, [selectedTaskId]);

  return (
    <div className="max-w-6xl mx-auto p-8 font-sans">
      <h2 className="text-2xl font-bold mb-4">Workspace runtime</h2>
      <p className="text-gray-600 mb-8">
        These existing Agent Protocol controls require a separately configured and authorized workspace runtime. Text analysis does not grant runtime or workspace access.
      </p>

      <button type="button" disabled={taskRead === 'loading'} onClick={() => void fetchTasks()}>Load workspace runtime tasks</button>
      {taskRead === 'unavailable' && <p role="alert">Workspace runtime task history could not be verified.</p>}
      {error && (
        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative mb-4">
          <span className="block sm:inline">{error}</span>
          <button className="absolute top-0 bottom-0 right-0 px-4 py-3" onClick={() => setError(null)}>
            <span className="text-2xl">&times;</span>
          </button>
        </div>
      )}

      {unknownCreation !== null && <section aria-label="Unconfirmed task creation" className="mb-6 rounded-lg border border-amber-300 p-4"><p>Task creation is unconfirmed. Another Create is held in this view to avoid duplicating the request.</p><p className="whitespace-pre-wrap">Submitted input: {unknownCreation}</p><button type="button" onClick={() => void reviewExistingTasks()}>Review existing tasks</button>{reviewStatus && <p role="status">{reviewStatus}</p>}<p>Review existing tasks before starting a new request. Reloading this page does not establish what happened to the original request.</p></section>}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
        <div className="glassmorphism glass-card bg-white/50 backdrop-blur-[30px] saturate-[210%] border border-white/40 p-6 shadow-sm rounded-2xl">
          <h2 className="text-xl font-bold mb-4">Tasks</h2>

          <div className="flex space-x-2 mb-6">
            <input
              type="text"
              placeholder="New Task Input..."
              value={taskInput}
              onChange={(e) => setTaskInput(e.target.value)}
              className="flex-1 border border-gray-300 rounded-lg px-3 py-2 bg-white/65 backdrop-blur-[30px] saturate-[210%] focus:ring-[#0066FF] focus:border-[#0066FF] transition-colors shadow-sm"
            />
            <button
              onClick={createTask}
              disabled={loading || !taskInput.trim() || unknownCreation !== null}
              className="bg-[#0071E3] text-white px-4 py-2 rounded-lg hover:bg-blue-700 disabled:opacity-50 shadow-sm transition-colors font-medium"
            >
              Create
            </button>
          </div>

          <ul className="space-y-2">
            {tasks.map((task) => (
              <li
                key={task.task_id}
                className={`p-4 border rounded-xl cursor-pointer transition shadow-sm bg-white/80 backdrop-blur-[30px] saturate-[210%] ${selectedTaskId === task.task_id ? 'border-[#0066FF] ring-1 ring-[#0066FF] bg-blue-50/50' : 'border-gray-200 hover:bg-gray-50/80'}`}
                onClick={() => setSelectedTaskId(task.task_id)}
              >
                <div className="font-semibold">{task.input || 'Untitled Task'}</div>
                <div className="text-xs text-gray-500 truncate">{task.task_id}</div>
              </li>
            ))}
            {taskRead === 'ready' && tasks.length === 0 && <div className="text-gray-500 text-sm italic">No tasks found.</div>}
          </ul>
        </div>

        <div className="glassmorphism bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 p-6 shadow-sm rounded-2xl">
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
                  className="flex-1 border border-gray-300 rounded-lg px-3 py-2 bg-white/65 backdrop-blur-[30px] saturate-[210%] focus:ring-[#0066FF] focus:border-[#0066FF] transition-colors shadow-sm"
                />
                <button
                  onClick={executeStep}
                  disabled={loading}
                  className="bg-green-600 text-white px-4 py-2 rounded-lg hover:bg-green-700 disabled:opacity-50 shadow-sm transition-colors font-medium"
                >
                  Execute Step
                </button>
              </div>

              <ul className="space-y-4">
                {steps.map((step, idx) => (
                  <li key={step.step_id} className="p-4 border rounded-xl border-gray-200 shadow-sm bg-white/80 backdrop-blur-[30px] saturate-[210%]">
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

              <div className="mt-8">
                <h3 className="text-lg font-bold mb-4">State Checkpoints</h3>
                <ul className="space-y-4">
                  {checkpoints.map((cp) => (
                    <li key={cp.checkpoint_id} className="p-4 border rounded-xl border-gray-200 shadow-sm bg-white/80 backdrop-blur-[30px] saturate-[210%]">
                      <div className="flex justify-between items-center mb-2">
                        <div>
                          <span className="font-bold text-sm">Checkpoint: </span>
                          <span className="text-xs text-gray-500 font-mono">{cp.checkpoint_id}</span>
                        </div>
                        <button
                          onClick={() => restoreCheckpoint(selectedTaskId, cp.checkpoint_id)}
                          disabled={loading}
                          className="bg-red-50 text-red-600 hover:bg-red-100 px-3 py-1 rounded text-sm font-medium transition shadow-sm disabled:opacity-50"
                        >
                          Restore Checkpoint
                        </button>
                      </div>
                      <div className="text-xs text-gray-400">Created: {cp.created_at}</div>
                    </li>
                  ))}
                  {checkpoints.length === 0 && <div className="text-gray-500 text-sm italic">No checkpoints saved.</div>}
                </ul>
              </div>
            </>
          )}
        </div>
      </div>
    </div>
  );
}
