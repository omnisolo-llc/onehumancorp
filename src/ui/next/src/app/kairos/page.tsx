"use client";

<<<<<<< HEAD
import { Suspense, useEffect, useState } from "react";
import { useSearchParams } from "next/navigation";
import { AppShell } from "../components/AppShell";
import { useWalkthrough } from "../../components/help";

type KairosTask = {
  id: string;
  name?: string;
  title?: string;
  status?: string;
  priority?: string;
};

type MeshNode = {
  id: string;
  type?: string;
  status?: string;
  load?: string | number;
};

function badgeTone(status?: string) {
  const normalized = (status || "").toLowerCase();
  if (["online", "completed", "healthy", "ready"].includes(normalized)) return "good";
  if (["queued", "in progress", "pending"].includes(normalized)) return "warn";
  if (["offline", "failed", "degraded"].includes(normalized)) return "bad";
  return "";
}

=======
import { useState, useEffect } from "react";
import Link from "next/link";
import { useSearchParams } from 'next/navigation';
import { Suspense } from 'react';
import { WithTooltip } from "../../components/TooltipRegistry";
import { useWalkthrough } from "../../components/help";

>>>>>>> e123d49a (feat: [architecture] Unified Multimodal Autonomous Customer Support Engine Research Report (#23362))
export default function KairosDashboard() {
  return (
    <Suspense fallback={<div className="p-8">Loading Kairos UI...</div>}>
      <KairosContent />
    </Suspense>
  );
}

function KairosContent() {
  const searchParams = useSearchParams();
  const { startWalkthrough } = useWalkthrough();
<<<<<<< HEAD
  const [tasks, setTasks] = useState<KairosTask[]>([]);
  const [meshNodes, setMeshNodes] = useState<MeshNode[]>([]);
  const [memoryStats, setMemoryStats] = useState<Record<string, string | number>>({});
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  useEffect(() => {
    if (searchParams.get("walkthrough") === "true") {
      setTimeout(() => {
        startWalkthrough([
          { targetId: "kairos-brain", message: "Shared tasks appear here when the orchestration backend returns active work." },
          { targetId: "kairos-nerves", message: "Mesh nodes appear here when live mesh status is available." },
          { targetId: "kairos-memory", message: "AutoDream memory statistics appear here when the backend exposes them." },
=======
  const [activeTasks, setActiveTasks] = useState([
    { id: "task-1", name: "Inventory Reorder Strategy", status: "In Progress", priority: "High" },
    { id: "task-2", name: "Customer Sentiment Analysis", status: "Queued", priority: "Medium" },
    { id: "task-3", name: "Social Media Campaign Draft", status: "Completed", priority: "Low" },
  ]);

  const [meshNodes, setMeshNodes] = useState([
    { id: "node-1", type: "Brain", status: "Online", load: "12%" },
    { id: "node-2", type: "Nerve", status: "Online", load: "45%" },
    { id: "node-3", type: "Memory", status: "Online", load: "8%" },
  ]);

  useEffect(() => {
    if (searchParams.get('walkthrough') === 'true') {
      setTimeout(() => {
        startWalkthrough([
          { targetId: "kairos-brain", message: "The Shared Task List is the 'Brain' of your business, where KAIROS manages and prioritizes all agent activities." },
          { targetId: "kairos-nerves", message: "The Teammate Mesh acts as the 'Nerves', providing lightning-fast communication between your AI workforce." },
          { targetId: "kairos-memory", message: "AutoDream is the 'Memory', storing every interaction so your agents never forget a detail about your business." }
>>>>>>> e123d49a (feat: [architecture] Unified Multimodal Autonomous Customer Support Engine Research Report (#23362))
        ]);
      }, 1000);
    }
  }, [searchParams, startWalkthrough]);

<<<<<<< HEAD
  useEffect(() => {
    async function loadKairos() {
      setLoading(true);
      setError("");
      try {
        const [tasksRes, meshRes, memoryRes] = await Promise.allSettled([
          fetch("/api/kairos/tasks"),
          fetch("/api/kairos/mesh"),
          fetch("/api/kairos/memory"),
        ]);

        if (tasksRes.status === "fulfilled" && tasksRes.value.ok) {
          const data = await tasksRes.value.json();
          setTasks(Array.isArray(data?.tasks) ? data.tasks : Array.isArray(data) ? data : []);
        }

        if (meshRes.status === "fulfilled" && meshRes.value.ok) {
          const data = await meshRes.value.json();
          setMeshNodes(Array.isArray(data?.nodes) ? data.nodes : Array.isArray(data) ? data : []);
        }

        if (memoryRes.status === "fulfilled" && memoryRes.value.ok) {
          const data = await memoryRes.value.json();
          setMemoryStats(data && typeof data === "object" ? data : {});
        }
      } catch (e: any) {
        setError(e?.message || "Failed to load Kairos data");
      } finally {
        setLoading(false);
      }
    }

    loadKairos();
  }, []);

  return (
    <AppShell
      title="Kairos"
      subtitle="Light-theme orchestration console using the same application side menu."
      statusItems={[
        { label: "Tasks", value: String(tasks.length), tone: tasks.length > 0 ? "good" : "neutral" },
        { label: "Mesh", value: String(meshNodes.length), tone: meshNodes.length > 0 ? "good" : "neutral" },
        { label: "Memory", value: Object.keys(memoryStats).length > 0 ? "Available" : "No data", tone: Object.keys(memoryStats).length > 0 ? "good" : "neutral" },
      ]}
    >
      {error && <div className="mb-4 app-badge bad">{error}</div>}

      <div className="app-grid two">
        <section id="kairos-brain" className="app-panel">
          <div className="app-panel-header">
            <div>
              <div className="app-panel-title">Shared Task List</div>
              <div className="app-list-subtitle">Live orchestration tasks only. No placeholder tasks are shown.</div>
            </div>
            <span className="app-badge">Brain</span>
          </div>
          {tasks.length === 0 ? (
            <div className="app-empty">{loading ? "Loading orchestration tasks..." : "No Kairos task rows returned by the backend."}</div>
          ) : (
            <div className="app-table-wrap">
              <table className="app-table">
                <thead>
                  <tr>
                    <th>Task</th>
                    <th>Status</th>
                    <th>Priority</th>
                  </tr>
                </thead>
                <tbody>
                  {tasks.map((task) => (
                    <tr key={task.id}>
                      <td className="font-semibold">{task.name || task.title || task.id}</td>
                      <td><span className={`app-badge ${badgeTone(task.status)}`}>{task.status || "Unknown"}</span></td>
                      <td>{task.priority || "Unspecified"}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </section>

        <section id="kairos-memory" className="app-panel">
          <div className="app-panel-header">
            <div>
              <div className="app-panel-title">AutoDream Memory</div>
              <div className="app-list-subtitle">Backend memory telemetry.</div>
            </div>
            <span className="app-badge">Memory</span>
          </div>
          {Object.keys(memoryStats).length === 0 ? (
            <div className="app-empty">{loading ? "Loading memory telemetry..." : "No AutoDream memory telemetry returned by the backend."}</div>
          ) : (
            <div className="app-panel-body">
              <div className="grid grid-cols-1 gap-3">
                {Object.entries(memoryStats).map(([key, value]) => (
                  <div key={key} className="app-card">
                    <div className="app-metric-label">{key.replaceAll("_", " ")}</div>
                    <div className="app-metric-value">{String(value)}</div>
                  </div>
                ))}
              </div>
            </div>
          )}
        </section>
      </div>

      <section id="kairos-nerves" className="app-panel mt-4">
        <div className="app-panel-header">
          <div>
            <div className="app-panel-title">Teammate Mesh</div>
            <div className="app-list-subtitle">Live mesh nodes only. No dark-theme node mockups.</div>
          </div>
          <span className="app-badge">Nerves</span>
        </div>
        {meshNodes.length === 0 ? (
          <div className="app-empty">{loading ? "Loading mesh status..." : "No mesh node rows returned by the backend."}</div>
        ) : (
          <div className="app-table-wrap">
            <table className="app-table">
              <thead>
                <tr>
                  <th>Node</th>
                  <th>Type</th>
                  <th>Status</th>
                  <th>Load</th>
                </tr>
              </thead>
              <tbody>
                {meshNodes.map((node) => (
                  <tr key={node.id}>
                    <td className="font-semibold">{node.id}</td>
                    <td>{node.type || "Unknown"}</td>
                    <td><span className={`app-badge ${badgeTone(node.status)}`}>{node.status || "Unknown"}</span></td>
                    <td>{node.load ?? "Unknown"}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </section>
    </AppShell>
=======
  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#16161A' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(22, 22, 26, 0.7)', backdropFilter: 'blur(20px) saturate(200%)', borderBottom: '1px solid rgba(255, 255, 255, 0.1)', position: 'sticky', top: 0, zIndex: 50 }}>
         <div className="flex items-center gap-4">
             <Link href="/dashboard" className="text-blue-600 hover:text-blue-800 transition-colors">
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
             </Link>
             <h1 className="text-2xl font-bold font-outfit" style={{ color: '#F5F5F7', letterSpacing: '-0.02em' }}>KAIROS Orchestration</h1>
         </div>
         <div className="flex items-center gap-2 px-3 py-1 bg-blue-50 rounded-full border border-blue-100">
            <div className="w-2 h-2 rounded-full animate-pulse" style={{ backgroundColor: '#0066FF' }}></div>
            <span className="text-xs font-medium text-blue-600">System Synchronized</span>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-7xl mx-auto w-full grid grid-cols-1 lg:grid-cols-3 gap-8">

        {/* 1. Shared Task List (The Brain) */}
        <section id="kairos-brain" className="lg:col-span-2 space-y-6">
            <div className="ohc-hybrid-panel shadow-lg flex flex-col gap-4">
                <div className="flex items-center justify-between">
                    <h2 className="text-xl font-bold font-outfit text-[#F5F5F7]">Shared Task List</h2>
                    <span className="text-xs font-bold px-2 py-1 bg-indigo-100 text-indigo-700 rounded-md">THE BRAIN</span>
                </div>
                <p className="text-sm text-gray-400 leading-relaxed">
                    KAIROS prioritizes and assigns business tasks across your autonomous team.
                </p>
                <div className="space-y-3">
                    {activeTasks.map(task => (
                        <div key={task.id} className="flex items-center justify-between p-4 bg-white/5 rounded-xl border border-gray-800 shadow-sm">
                            <div className="flex items-center gap-3">
                                <div className={`w-2 h-2 rounded-full ${task.status === 'Completed' ? 'bg-green-500' : task.status === 'In Progress' ? 'bg-blue-500' : 'bg-gray-400'}`}></div>
                                <span className="text-sm font-semibold text-gray-200">{task.name}</span>
                            </div>
                            <div className="flex items-center gap-4">
                                <span className="text-xs font-medium text-gray-400">{task.status}</span>
                                <span className={`text-[10px] font-bold uppercase tracking-wider px-1.5 py-0.5 rounded border ${task.priority === 'High' ? 'border-red-900/50 text-red-400 bg-red-900/20' : 'border-gray-700 text-gray-400'}`}>
                                    {task.priority}
                                </span>
                            </div>
                        </div>
                    ))}
                </div>
            </div>

            {/* 2. Teammate Mesh (The Nerves) */}
            <div id="kairos-nerves" className="ohc-hybrid-panel shadow-lg flex flex-col gap-4">
                <div className="flex items-center justify-between">
                    <h2 className="text-xl font-bold font-outfit text-[#F5F5F7]">Teammate Mesh</h2>
                    <span className="text-xs font-bold px-2 py-1 bg-green-100 text-green-700 rounded-md">THE NERVES</span>
                </div>
                <p className="text-sm text-gray-400 leading-relaxed">
                    Real-time communication and coordination layer for all active agents.
                </p>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                    {meshNodes.map(node => (
                        <div key={node.id} className="p-4 bg-white/5 rounded-xl border border-gray-800 shadow-sm flex flex-col gap-2">
                            <div className="flex justify-between items-center">
                                <span className="text-xs font-bold text-gray-400 uppercase tracking-widest">{node.type}</span>
                                <span className="w-2 h-2 rounded-full bg-green-500"></span>
                            </div>
                            <div className="text-lg font-bold text-gray-200">{node.status}</div>
                            <div className="flex items-center justify-between mt-2">
                                <span className="text-xs text-gray-400">Load</span>
                                <span className="text-xs font-bold text-gray-300">{node.load}</span>
                            </div>
                            <div className="w-full h-1 bg-gray-800 rounded-full overflow-hidden">
                                <div className="h-full bg-blue-500 transition-all duration-1000" style={{ width: node.load }}></div>
                            </div>
                        </div>
                    ))}
                </div>
            </div>
        </section>

        {/* 3. AutoDream (The Memory) */}
        <section id="kairos-memory" className="lg:col-span-1 space-y-6">
            <div className="ohc-hybrid-panel shadow-lg h-full flex flex-col gap-6">
                <div className="flex items-center justify-between">
                    <h2 className="text-xl font-bold font-outfit text-[#F5F5F7]">AutoDream Memory</h2>
                    <span className="text-xs font-bold px-2 py-1 bg-purple-100 text-purple-700 rounded-md">THE MEMORY</span>
                </div>

                <div className="flex-1 flex flex-col items-center justify-center text-center p-8">
                    <div className="relative w-32 h-32 mb-6">
                        <div className="absolute inset-0 bg-purple-200 rounded-full opacity-20 animate-ping"></div>
                        <div className="relative w-32 h-32 bg-gradient-to-br from-purple-500 to-indigo-600 rounded-full shadow-xl flex items-center justify-center text-white">
                            <svg className="w-16 h-16" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" /></svg>
                        </div>
                    </div>
                    <h3 className="text-lg font-bold text-[#F5F5F7] mb-2">Infinite Context</h3>
                    <p className="text-sm text-gray-400 leading-relaxed">
                        AutoDream stores every interaction, ensuring your team learns and grows with your business.
                    </p>
                </div>

                <div className="space-y-4">
                    <div className="p-4 bg-purple-900/20 rounded-xl border border-purple-800">
                        <div className="text-xs font-bold text-purple-600 uppercase tracking-widest mb-1">Knowledge Density</div>
                        <div className="text-2xl font-bold text-purple-200">842.5 MB</div>
                    </div>
                    <div className="p-4 bg-indigo-900/20 rounded-xl border border-indigo-800">
                        <div className="text-xs font-bold text-indigo-600 uppercase tracking-widest mb-1">Semantic Clusters</div>
                        <div className="text-2xl font-bold text-indigo-200">12 Active</div>
                    </div>
                </div>
            </div>
        </section>

      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .ohc-hybrid-panel {
            backdrop-filter: blur(20px) saturate(200%);
            background: rgba(255, 255, 255, 0.03);
            font-family: 'Outfit', 'Inter', sans-serif;
            border: 1px solid rgba(255, 255, 255, 0.08);
            border-radius: 16px;
            padding: 24px;
        }
      `}} />
    </div>
>>>>>>> e123d49a (feat: [architecture] Unified Multimodal Autonomous Customer Support Engine Research Report (#23362))
  );
}
