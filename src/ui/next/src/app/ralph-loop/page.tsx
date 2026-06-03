'use client';

import React, { useState, useEffect } from 'react';
import {
  Rocket,
  CheckCircle2,
  Circle,
  Loader2,
  Terminal,
  Bug,
  BookOpen,
  Play,
  ArrowRight,
  GitBranch
} from 'lucide-react';

interface Feature {
  name: string;
  status: 'pending' | 'in_progress' | 'completed';
}

interface RalphProgress {
  task_description: string;
  features: Feature[];
  current_feature_index: number;
  notes: string[];
  architectural_decisions: string[];
  unresolved_bugs: string[];
  session_id: string;
  is_complete: boolean;
}

export default function RalphLoopPage() {
  const [task, setTask] = useState('');
  const [missionId, setMissionId] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [progress, setProgress] = useState<RalphProgress | null>(null);
  const [polling, setPolling] = useState(false);

  const startMission = async () => {
    setLoading(true);
    try {
      const res = await fetch('/api/ralph-loop', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ task }),
      });
      const data = await res.json();
      if (data.taskId) {
        setMissionId(data.taskId);
        setPolling(true);
      }
    } catch (err) {
      console.error("Failed to start mission", err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    let timer: NodeJS.Timeout;
    if (polling && missionId) {
      const fetchProgress = async () => {
        try {
          const res = await fetch(`/api/ralph-loop?taskId=${missionId}`);
          const data = await res.json();
          setProgress(data);
          if (data.is_complete) setPolling(false);
        } catch (err) {
          console.error("Polling failed", err);
        }
      };

      fetchProgress();
      timer = setInterval(fetchProgress, 5000);
    }
    return () => clearInterval(timer);
  }, [polling, missionId]);

  return (
    <div className="min-h-screen bg-[#F8FAFC] p-8 font-sans">
      <div className="max-w-6xl mx-auto">
        {/* Header */}
        <header className="mb-12 flex items-center justify-between">
          <div>
            <div className="flex items-center gap-3 mb-2">
              <div className="bg-blue-600 p-2 rounded-xl shadow-lg shadow-blue-200">
                <Rocket className="text-white w-6 h-6" />
              </div>
              <h1 className="text-3xl font-extrabold text-slate-900 tracking-tight">Ralph Mission Control</h1>
            </div>
            <p className="text-slate-500 text-lg">Asynchronous multi-session agent orchestration for complex business logic.</p>
          </div>

          {missionId && (
            <div className="bg-white px-4 py-2 rounded-full border border-slate-200 shadow-sm flex items-center gap-2">
              <span className="w-2 h-2 bg-green-500 rounded-full animate-pulse" />
              <span className="text-sm font-mono text-slate-600">ID: {missionId}</span>
            </div>
          )}
        </header>

        {!missionId ? (
          /* Initial State: Setup Mission */
          <div className="bg-white rounded-3xl shadow-xl border border-slate-200 p-10 max-w-2xl mx-auto backdrop-blur-xl bg-white/80">
            <h2 className="text-2xl font-bold mb-6 text-slate-800">Launch New Mission</h2>
            <div className="space-y-6">
              <div>
                <label className="block text-sm font-semibold text-slate-700 mb-2 uppercase tracking-wider">Mission Objective</label>
                <textarea
                  className="w-full bg-slate-50 border-slate-200 rounded-2xl p-5 text-slate-800 focus:ring-4 focus:ring-blue-100 transition-all focus:border-blue-500 outline-none min-h-[200px] text-lg leading-relaxed"
                  placeholder="e.g. Build a comprehensive inventory management system with real-time stock alerts, multi-warehouse support, and automated reordering logic."
                  value={task}
                  onChange={(e) => setTask(e.target.value)}
                />
              </div>
              <button
                onClick={startMission}
                disabled={loading || !task}
                className="w-full py-4 bg-blue-600 hover:bg-blue-700 disabled:opacity-50 text-white rounded-2xl font-bold text-xl flex items-center justify-center gap-3 transition-all shadow-xl shadow-blue-200 group"
              >
                {loading ? <Loader2 className="animate-spin" /> : <Play className="group-hover:translate-x-1 transition-transform" />}
                Initiate Ralph Loop
              </button>
            </div>
          </div>
        ) : (
          /* Active Mission State */
          <div className="grid grid-cols-12 gap-8">

            {/* Left Column: Progress & Features */}
            <div className="col-span-12 lg:col-span-4 space-y-8">
              <div className="bg-white rounded-3xl p-8 border border-slate-200 shadow-sm">
                <h3 className="text-xl font-bold mb-6 flex items-center gap-2">
                  <CheckCircle2 className="text-blue-600" />
                  Mission Roadmap
                </h3>
                <div className="space-y-6">
                  {progress?.features.map((f, i) => (
                    <div key={i} className="flex gap-4 relative">
                      {i !== (progress.features.length - 1) && (
                        <div className={`absolute left-3 top-8 w-0.5 h-10 ${i < progress.current_feature_index ? 'bg-blue-500' : 'bg-slate-200'}`} />
                      )}
                      <div className={`z-10 w-6 h-6 rounded-full flex items-center justify-center mt-1 transition-colors ${
                        f.status === 'completed' ? 'bg-blue-600' :
                        f.status === 'in_progress' ? 'bg-amber-500' : 'bg-slate-200'
                      }`}>
                        {f.status === 'completed' ? <CheckCircle2 className="w-4 h-4 text-white" /> :
                         f.status === 'in_progress' ? <Loader2 className="w-4 h-4 text-white animate-spin" /> :
                         <Circle className="w-4 h-4 text-slate-400" />}
                      </div>
                      <div>
                        <p className={`font-bold transition-colors ${
                          f.status === 'pending' ? 'text-slate-400' : 'text-slate-800'
                        }`}>{f.name}</p>
                        <p className="text-xs uppercase font-bold tracking-widest text-slate-400 mt-1">{f.status}</p>
                      </div>
                    </div>
                  ))}
                </div>
              </div>

              {/* State Summary Stats */}
              <div className="grid grid-cols-2 gap-4">
                <div className="bg-white p-6 rounded-3xl border border-slate-200 shadow-sm flex flex-col items-center justify-center text-center">
                  <BookOpen className="text-indigo-600 mb-2" />
                  <span className="text-2xl font-black text-slate-900">{progress?.architectural_decisions.length || 0}</span>
                  <span className="text-xs font-bold text-slate-400 uppercase tracking-tighter">Decisions</span>
                </div>
                <div className="bg-white p-6 rounded-3xl border border-slate-200 shadow-sm flex flex-col items-center justify-center text-center">
                  <Bug className="text-rose-600 mb-2" />
                  <span className="text-2xl font-black text-slate-900">{progress?.unresolved_bugs.length || 0}</span>
                  <span className="text-xs font-bold text-slate-400 uppercase tracking-tighter">Bugs Found</span>
                </div>
              </div>
            </div>

            {/* Right Column: Mission Console */}
            <div className="col-span-12 lg:col-span-8 space-y-8">

              {/* Mission Objective Header */}
              <div className="bg-slate-900 rounded-3xl p-8 text-white shadow-2xl relative overflow-hidden">
                <div className="absolute top-0 right-0 p-8 opacity-10">
                  <GitBranch className="w-32 h-32" />
                </div>
                <div className="relative z-10">
                  <h4 className="text-blue-400 text-sm font-black uppercase tracking-[0.2em] mb-3">Primary Directive</h4>
                  <p className="text-2xl font-medium leading-relaxed italic">"{progress?.task_description}"</p>
                </div>
              </div>

              {/* Live Mission Logs */}
              <div className="bg-white rounded-3xl border border-slate-200 shadow-sm overflow-hidden flex flex-col">
                <div className="p-6 border-b border-slate-100 flex items-center justify-between bg-slate-50/50">
                  <div className="flex items-center gap-3">
                    <Terminal className="text-slate-400 w-5 h-5" />
                    <h3 className="font-bold text-slate-800">Mission Terminal Logs</h3>
                  </div>
                  <div className="flex gap-1">
                    <div className="w-2 h-2 rounded-full bg-slate-300" />
                    <div className="w-2 h-2 rounded-full bg-slate-300" />
                    <div className="w-2 h-2 rounded-full bg-slate-300" />
                  </div>
                </div>
                <div className="p-8 space-y-4 max-h-[500px] overflow-y-auto font-mono text-sm leading-relaxed bg-[#0F172A] text-slate-300">
                  {progress?.notes.map((n, i) => (
                    <div key={i} className="flex gap-4 group">
                      <span className="text-slate-600 select-none">{i+1}</span>
                      <p className="group-hover:text-blue-400 transition-colors">
                        <span className="text-green-500 mr-2">✓</span>
                        {n}
                      </p>
                    </div>
                  ))}
                  {polling && (
                    <div className="flex gap-4 animate-pulse">
                      <span className="text-slate-600">{ (progress?.notes.length || 0) + 1 }</span>
                      <div className="flex items-center gap-2 text-blue-400">
                        <Loader2 className="w-4 h-4 animate-spin" />
                        <span>Ralph is implementing next feature...</span>
                      </div>
                    </div>
                  )}
                </div>
              </div>

              {/* Architectural Tracker */}
              <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
                 <div className="bg-indigo-50/50 rounded-3xl p-6 border border-indigo-100">
                    <h4 className="font-black text-indigo-900 text-sm uppercase tracking-widest mb-4 flex items-center gap-2">
                      <ArrowRight className="w-4 h-4" />
                      Key Decisions
                    </h4>
                    <ul className="space-y-3">
                      {progress?.architectural_decisions.map((d, i) => (
                        <li key={i} className="text-indigo-800 text-sm leading-relaxed flex gap-2">
                          <span className="font-bold text-indigo-300">•</span>
                          {d}
                        </li>
                      ))}
                    </ul>
                 </div>
                 <div className="bg-rose-50/50 rounded-3xl p-6 border border-rose-100">
                    <h4 className="font-black text-rose-900 text-sm uppercase tracking-widest mb-4 flex items-center gap-2">
                      <Bug className="w-4 h-4" />
                      Known Issues
                    </h4>
                    <ul className="space-y-3">
                      {progress?.unresolved_bugs.map((b, i) => (
                        <li key={i} className="text-rose-800 text-sm leading-relaxed flex gap-2">
                          <span className="font-bold text-rose-300">!</span>
                          {b}
                        </li>
                      ))}
                    </ul>
                 </div>
              </div>

            </div>
          </div>
        )}
      </div>
    </div>
  );
}
