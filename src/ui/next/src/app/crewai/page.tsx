'use client';

import React, { useState } from 'react';
import Head from 'next/head';
import Link from 'next/link';

export default function CrewAIPage() {
  const [task, setTask] = useState('');
  const [report, setReport] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const handleExecute = async () => {
    if (!task.trim()) {
      setError('Please enter a task description.');
      return;
    }
    setError('');
    setLoading(true);
    setReport('');

    try {
      const res = await fetch('/api/agents/crewai', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ task_description: task }),
      });

      if (!res.ok) {
        throw new Error('Failed to execute CrewAI workflow');
      }

      const data = await res.json();
      setReport(data.report);
    } catch (err: any) {
      setError(err.message || 'An unexpected error occurred');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col items-center pt-12 px-4 font-sans text-gray-900">
      <Head>
        <title>CrewAI - Role-Based Agent Flow</title>
      </Head>

      <div className="w-full max-w-2xl bg-white rounded-xl shadow-sm border border-gray-200 p-8">
        <div className="mb-8">
          <Link href="/" className="text-sm font-semibold text-blue-600 hover:text-blue-800 mb-4 inline-block">&larr; Back to Dashboard</Link>
          <h1 className="text-3xl font-extrabold tracking-tight text-gray-900 mb-2">CrewAI Agent Harness</h1>
          <p className="text-gray-600">
            Execute tasks using the CrewAI Role-Based flow deterministic backbone. The LLM only handles intelligence where it matters.
          </p>
        </div>

        <div className="space-y-6">
          <div>
            <label className="block text-sm font-semibold text-gray-700 mb-2">
              Task Description
            </label>
            <textarea
              className="w-full h-32 px-4 py-3 bg-gray-50 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors"
              placeholder="E.g., Research competitor pricing strategies and draft a summary report."
              value={task}
              onChange={(e) => setTask(e.target.value)}
              disabled={loading}
              data-testid="crewai-task-input"
            />
            {error && <p className="mt-2 text-sm text-red-600 font-medium" data-testid="crewai-error">{error}</p>}
          </div>

          <button
            onClick={handleExecute}
            disabled={loading}
            className={`w-full py-3 px-4 rounded-lg font-bold text-white transition-all ${
              loading ? 'bg-blue-400 cursor-not-allowed' : 'bg-blue-600 hover:bg-blue-700 active:transform active:scale-[0.99] shadow-md hover:shadow-lg'
            }`}
            data-testid="crewai-execute-btn"
          >
            {loading ? (
              <span className="flex items-center justify-center">
                <svg className="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                  <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                  <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                Executing Flow...
              </span>
            ) : 'Execute CrewAI Flow'}
          </button>
        </div>

        {report && (
          <div className="mt-8 border-t border-gray-200 pt-6 animate-in fade-in slide-in-from-bottom-4 duration-500">
            <h2 className="text-lg font-bold text-gray-900 mb-4">Execution Report</h2>
            <div className="bg-gray-900 rounded-lg p-4 overflow-x-auto shadow-inner">
              <pre className="text-sm font-mono text-green-400 whitespace-pre-wrap" data-testid="crewai-report-output">
                {report}
              </pre>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
