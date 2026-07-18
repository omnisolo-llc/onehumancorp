"use client";

import { Suspense, useEffect, useRef, useState } from "react";
import { useSearchParams } from "next/navigation";
import { AppShell } from "../components/AppShell";
import { InteractiveWalkthrough } from "../../components/Walkthrough";

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

const kairosWalkthroughSteps = [
  {
    targetId: "kairos-brain",
    title: "Quick Guide",
    content: "Shared tasks appear here when the orchestration backend returns active work.",
    position: "bottom" as const,
  },
  {
    targetId: "kairos-nerves",
    title: "Quick Guide",
    content: "Mesh nodes appear here when live mesh status is available.",
    position: "top" as const,
  },
  {
    targetId: "kairos-memory",
    title: "Quick Guide",
    content: "AutoDream memory statistics appear here when the backend exposes them.",
    position: "bottom" as const,
  },
];

function badgeTone(status?: string) {
  const normalized = (status || "").toLowerCase();
  if (["online", "completed", "healthy", "ready"].includes(normalized)) return "good";
  if (["queued", "in progress", "pending"].includes(normalized)) return "warn";
  if (["offline", "failed", "degraded"].includes(normalized)) return "bad";
  return "";
}

export default function KairosDashboard() {
  return (
    <Suspense fallback={<div className="p-8">Loading Kairos UI...</div>}>
      <KairosContent />
    </Suspense>
  );
}

function KairosContent() {
  const searchParams = useSearchParams();
  const walkthroughStarted = useRef(false);
  const [tasks, setTasks] = useState<KairosTask[]>([]);
  const [meshNodes, setMeshNodes] = useState<MeshNode[]>([]);
  const [memoryStats, setMemoryStats] = useState<Record<string, string | number>>({});
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");
  const [isWalkthroughOpen, setIsWalkthroughOpen] = useState(false);

  useEffect(() => {
    if (searchParams.get("walkthrough") === "true" && !walkthroughStarted.current) {
      const timeoutId = window.setTimeout(() => {
        walkthroughStarted.current = true;
        setIsWalkthroughOpen(true);
      }, 0);
      return () => window.clearTimeout(timeoutId);
    }
  }, [searchParams]);

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
      <InteractiveWalkthrough
        steps={kairosWalkthroughSteps}
        isOpen={isWalkthroughOpen}
        onClose={() => setIsWalkthroughOpen(false)}
      />

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
  );
}
