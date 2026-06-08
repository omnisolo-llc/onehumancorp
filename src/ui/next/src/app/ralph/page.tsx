'use client';

import React, { useState, useEffect } from 'react';

type Feature = {
  name: string;
  status: string;
};

type RalphProgress = {
  task_description: string;
  features: Feature[];
  current_feature_index: number;
  notes: string[];
  is_complete: boolean;
};

export default function RalphLoopDashboard() {
  const [task, setTask] = useState('');
  const [progress, setProgress] = useState<RalphProgress | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchProgress = async () => {
    try {
      const res = await fetch('/api/agents/ralph/progress');
      if (res.ok) {
        const data = await res.json();
        setProgress(data);
      } else {
        if (res.status === 404) {
          setProgress(null);
        } else {
          setError('Failed to fetch progress');
        }
      }
    } catch (err: any) {
      setError(err.message);
    }
  };

  useEffect(() => {
    fetchProgress();
    const interval = setInterval(fetchProgress, 3000);
    return () => clearInterval(interval);
  }, []);

  const handleStartTask = async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch('/api/agents/ralph/start', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ task }),
      });
      if (!res.ok) {
        throw new Error('Failed to start Ralph Loop');
      }
      setTask('');
      await fetchProgress();
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 p-8 font-outfit">
      <div className="max-w-4xl mx-auto">
        <header className="mb-8">
          <h1 className="text-4xl font-bold text-gray-900 mb-2">Ralph Loop Agent CLI</h1>
          <p className="text-xl text-gray-600">
            Orchestrate long-running, asynchronous agent tasks securely.
          </p>
        </header>

        <div className="bg-white p-6 rounded-2xl shadow-sm border border-gray-200 mb-8">
          <h2 className="text-2xl font-bold mb-4">Start New Task</h2>
          <textarea
            value={task}
            onChange={(e) => setTask(e.target.value)}
            className="w-full p-4 rounded-xl border border-gray-300 focus:ring-2 focus:ring-blue-500 outline-none resize-none h-32 mb-4"
            placeholder="Describe the large task you want the agent to accomplish..."
            disabled={progress !== null && !progress.is_complete}
          />
          <button
            onClick={handleStartTask}
            disabled={loading || task.trim() === '' || (progress !== null && !progress.is_complete)}
            className="px-6 py-3 bg-blue-600 text-white font-medium rounded-xl hover:bg-blue-700 disabled:bg-gray-400"
          >
            {loading ? 'Starting...' : 'Start Ralph Agent'}
          </button>
        </div>

        {error && (
          <div className="p-4 mb-8 bg-red-100 text-red-700 rounded-xl">
            {error}
          </div>
        )}

        {progress && (
          <div className="bg-white p-6 rounded-2xl shadow-sm border border-gray-200">
            <h2 className="text-2xl font-bold mb-4">Current Task Progress</h2>
            <div className="mb-4">
              <strong>Objective:</strong> {progress.task_description}
            </div>

            <div className="mb-6">
              <h3 className="text-xl font-semibold mb-2">Features Breakdown</h3>
              <ul className="space-y-2">
                {progress.features.map((feature, idx) => (
                  <li key={idx} className={`p-3 rounded-lg border flex justify-between items-center ${
                    feature.status === 'completed' ? 'bg-green-50 border-green-200' :
                    progress.current_feature_index === idx ? 'bg-blue-50 border-blue-200' : 'bg-gray-50 border-gray-200'
                  }`}>
                    <span className="font-medium">{feature.name}</span>
                    <span className={`text-sm font-bold uppercase ${
                      feature.status === 'completed' ? 'text-green-600' :
                      progress.current_feature_index === idx ? 'text-blue-600' : 'text-gray-500'
                    }`}>
                      {feature.status === 'completed' ? 'Done' : progress.current_feature_index === idx ? 'In Progress' : 'Pending'}
                    </span>
                  </li>
                ))}
              </ul>
            </div>

            <div>
              <h3 className="text-xl font-semibold mb-2">Scratchpad Notes</h3>
              <div className="bg-gray-50 p-4 rounded-xl font-mono text-sm max-h-64 overflow-y-auto border border-gray-200">
                {progress.notes.length > 0 ? progress.notes.map((note, idx) => (
                  <div key={idx} className="mb-2 pb-2 border-b border-gray-200 last:border-0 last:mb-0 last:pb-0 text-gray-700">
                    &gt; {note}
                  </div>
                )) : (
                  <div className="text-gray-400 italic">No notes yet...</div>
                )}
              </div>
            </div>

            {progress.is_complete && (
              <div className="mt-6 p-4 bg-green-100 text-green-800 rounded-xl font-medium border border-green-200">
                🎉 Ralph Loop task successfully completed!
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
