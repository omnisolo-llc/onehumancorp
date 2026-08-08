import React, { useState, useEffect } from "react";

export interface WorkflowTask {
  id: string;
  title: string;
  description: string;
  role: string;
  phase: string;
  dependencies: string[];
  verification_of?: string | null;
}

export interface DynamicWorkflowPlan {
  id: string;
  prompt: string;
  status: string;
  tasks: WorkflowTask[];
}

interface ActionPlanCardProps {
  planId: string;
  prompt: string;
  initialPlan: DynamicWorkflowPlan;
  onApprove: (id: string) => Promise<any>;
}

export const ActionPlanCard: React.FC<ActionPlanCardProps> = ({
  planId,
  prompt,
  initialPlan,
  onApprove,
}) => {
  const [status, setStatus] = useState<string>("Awaiting Approval"); // Awaiting Approval, Approving, Running, Completed, Failed
  const [taskStates, setTaskStates] = useState<Record<string, "pending" | "running" | "completed" | "failed">>(() => {
    const states: Record<string, "pending" | "running" | "completed" | "failed"> = {};
    (initialPlan.tasks || []).forEach((t) => {
      states[t.id] = "pending";
    });
    return states;
  });
  const [error, setError] = useState<string | null>(null);
  const [retryCount, setRetryCount] = useState<number>(0);

  // Group tasks by phase
  const phases = ["planning", "execution", "verification", "synthesis"];
  const groupedTasks = phases.reduce((acc, phase) => {
    acc[phase] = (initialPlan.tasks || []).filter((t) => t.phase === phase);
    return acc;
  }, {} as Record<string, WorkflowTask[]>);

  const executeWithRetry = async (attempt: number) => {
    setError(null);
    try {
      await onApprove(planId);
      setStatus("Running");
    } catch (e: any) {
      if (attempt < 2) {
        // Automatically retry with a slight delay
        setTimeout(() => {
          setRetryCount(attempt + 1);
          executeWithRetry(attempt + 1);
        }, 1000);
      } else {
        setError("Flaky network: Request timed out. Click retry to resume safely.");
        setStatus("Failed");
      }
    }
  };

  const handleApprove = () => {
    setStatus("Approving");
    executeWithRetry(0);
  };

  // Simulate progress transition when running
  useEffect(() => {
    if (status !== "Running") return;

    let isMounted = true;
    const planTasks = groupedTasks["planning"] || [];
    const execTasks = groupedTasks["execution"] || [];
    const verifyTasks = groupedTasks["verification"] || [];
    const synthTasks = groupedTasks["synthesis"] || [];

    const setPhaseState = (tasks: WorkflowTask[], state: "pending" | "running" | "completed" | "failed") => {
      if (!isMounted) return;
      setTaskStates((prev) => {
        const next = { ...prev };
        tasks.forEach((t) => {
          next[t.id] = state;
        });
        return next;
      });
    };

    // Sequence execution simulation
    const runSequence = async () => {
      // 1. Planning phase
      setPhaseState(planTasks, "running");
      await new Promise((r) => setTimeout(r, 1200));
      setPhaseState(planTasks, "completed");

      // 2. Execution phase (parallel shards!)
      setPhaseState(execTasks, "running");
      await new Promise((r) => setTimeout(r, 1500));
      setPhaseState(execTasks, "completed");

      // 3. Verification phase
      setPhaseState(verifyTasks, "running");
      await new Promise((r) => setTimeout(r, 1500));
      setPhaseState(verifyTasks, "completed");

      // 4. Synthesis phase
      setPhaseState(synthTasks, "running");
      await new Promise((r) => setTimeout(r, 1200));
      setPhaseState(synthTasks, "completed");

      if (isMounted) {
        setStatus("Completed");
      }
    };

    runSequence();

    return () => {
      isMounted = false;
    };
  }, [status]);

  return (
    <div
      className="glassmorphism p-5 relative overflow-hidden rounded-2xl border border-white/20 dark:border-white/10 bg-white/60 dark:bg-black/40 backdrop-blur-2xl shadow-lg flex flex-col gap-4 max-w-full"
      data-testid={`action-plan-card-${planId}`}
    >
      {/* Background radial highlight */}
      <div className="absolute -right-16 -top-16 w-36 h-36 rounded-full bg-blue-500/10 dark:bg-blue-400/20 blur-2xl pointer-events-none"></div>

      {/* Header */}
      <div className="flex justify-between items-center">
        <div className="flex items-center gap-2">
          <span className="text-xl">⚡</span>
          <div>
            <h3 className="text-sm font-bold text-gray-400 uppercase tracking-wider">
              Autonomous Action Engine
            </h3>
            <p className="text-xs text-gray-500 font-medium">Multi-Step Workflow Graph</p>
          </div>
        </div>
        <span
          className={`text-xs font-bold uppercase tracking-wider px-2.5 py-1 rounded-full shadow-sm border ${
            status === "Completed"
              ? "text-green-600 bg-green-50 border-green-200"
              : status === "Failed"
              ? "text-red-600 bg-red-50 border-red-200"
              : status === "Running"
              ? "text-indigo-600 bg-indigo-50 border-indigo-200 animate-pulse"
              : "text-blue-600 bg-blue-50 border-blue-200"
          }`}
          data-testid="workflow-status-badge"
        >
          {status}
        </span>
      </div>

      {/* User Natural Language Intent */}
      <div className="bg-gray-50 dark:bg-gray-900/50 p-3.5 rounded-xl border border-gray-100 dark:border-gray-800">
        <p className="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase mb-1">
          Owner Request
        </p>
        <p className="text-sm font-semibold text-gray-900 dark:text-gray-100 leading-snug">
          "{prompt}"
        </p>
      </div>

      {/* Workflow Task Execution Tree */}
      <div className="flex flex-col gap-3 mt-1" data-testid="task-phases-container">
        {phases.map((phase) => {
          const tasks = groupedTasks[phase] || [];
          if (tasks.length === 0) return null;

          return (
            <div key={phase} className="flex flex-col gap-1.5 pl-3 border-l-2 border-gray-200 dark:border-gray-800 relative">
              <span className="text-[10px] font-bold text-gray-400 uppercase tracking-wider mb-0.5">
                {phase} phase
              </span>
              <div className="flex flex-col gap-2">
                {tasks.map((task) => {
                  const state = taskStates[task.id] || "pending";
                  return (
                    <div
                      key={task.id}
                      className="bg-white/80 dark:bg-gray-800/40 p-3 rounded-xl border border-gray-100 dark:border-gray-700/50 flex justify-between items-center shadow-sm"
                      data-testid={`task-node-${task.id}`}
                    >
                      <div className="flex flex-col gap-0.5 max-w-[80%]">
                        <span className="text-xs font-bold text-gray-900 dark:text-gray-100">
                          {task.title}
                        </span>
                        <span className="text-[10px] text-gray-500 dark:text-gray-400 leading-relaxed font-medium">
                          {task.description}
                        </span>
                        <span className="text-[9px] font-mono text-blue-500 bg-blue-50 dark:bg-blue-900/20 px-1.5 py-0.5 rounded-md self-start mt-1">
                          @{task.role}
                        </span>
                      </div>

                      {/* Status indicator */}
                      <div className="flex items-center">
                        {state === "completed" && (
                          <span
                            className="w-5 h-5 flex items-center justify-center text-xs text-green-600 bg-green-100 rounded-full"
                            data-testid={`badge-completed-${task.id}`}
                          >
                            ✓
                          </span>
                        )}
                        {state === "running" && (
                          <div
                            className="w-4 h-4 border-2 border-indigo-600 border-t-transparent rounded-full animate-spin"
                            data-testid={`badge-running-${task.id}`}
                          ></div>
                        )}
                        {state === "pending" && (
                          <span
                            className="w-2.5 h-2.5 bg-gray-300 dark:bg-gray-700 rounded-full"
                            data-testid={`badge-pending-${task.id}`}
                          ></span>
                        )}
                        {state === "failed" && (
                          <span
                            className="w-5 h-5 flex items-center justify-center text-xs text-red-600 bg-red-100 rounded-full"
                            data-testid={`badge-failed-${task.id}`}
                          >
                            ✗
                          </span>
                        )}
                      </div>
                    </div>
                  );
                })}
              </div>
            </div>
          );
        })}
      </div>

      {/* Network Error Notification */}
      {error && (
        <div
          className="p-3 bg-red-50 dark:bg-red-950/20 border border-red-200 dark:border-red-900/40 rounded-xl flex flex-col gap-2"
          data-testid="flaky-network-error"
        >
          <div className="flex items-center gap-2 text-red-600 dark:text-red-400 text-xs font-semibold">
            <span>⚠</span>
            <span>{error}</span>
          </div>
          <button
            onClick={handleApprove}
            className="self-start text-[11px] font-bold text-white bg-red-600 hover:bg-red-700 px-3 py-1.5 rounded-lg shadow-sm transition-all"
            data-testid="error-retry-btn"
          >
            Retry Execution
          </button>
        </div>
      )}

      {/* Approve and Execute Button */}
      {status === "Awaiting Approval" && (
        <button
          onClick={handleApprove}
          className="w-full min-h-[44px] min-w-[44px] bg-[#0066FF] hover:bg-[#0052CC] text-white font-bold rounded-xl shadow-md transition-all flex items-center justify-center transform active:scale-95 text-sm"
          data-testid="approve-execute-btn"
        >
          Approve & Execute Plan
        </button>
      )}

      {status === "Approving" && (
        <button
          disabled
          className="w-full min-h-[44px] min-w-[44px] bg-blue-400 text-white font-bold rounded-xl shadow-md flex items-center justify-center text-sm"
          data-testid="approve-execute-btn-loading"
        >
          <div className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin mr-2"></div>
          Contacting Autonomous Agents...
        </button>
      )}
    </div>
  );
};
