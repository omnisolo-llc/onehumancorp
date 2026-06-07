'use client';

import Link from 'next/link';
import { useEffect, useMemo, useState } from 'react';

type PermissionProfile = 'Guarded' | 'Full Access';
type AssistantTaskStatus = 'running' | 'completed' | 'blocked' | 'failed' | 'archived';

type AssistantArtifact = {
  id: string;
  type: string;
  filename: string;
  preview: string;
};

type AssistantChange = {
  id: string;
  path: string;
  summary: string;
  approvalStatus: string;
};

type AssistantMessage = {
  id: string;
  role: string;
  content: string;
};

type AssistantTask = {
  id: string;
  title: string;
  workspace: string;
  status: AssistantTaskStatus;
  currentStep: string;
  mode: string;
  model: string;
  provider: string;
  permissionProfile: PermissionProfile;
  riskSummary: string[];
  artifacts: AssistantArtifact[];
  changes: AssistantChange[];
  messages: AssistantMessage[];
};

type Panel = 'remote' | 'automations' | 'memory' | 'skills' | 'connectors';
type ResultTab = 'Artifacts' | 'All Files' | 'Changes' | 'Preview';

const resultTabs: ResultTab[] = ['Artifacts', 'All Files', 'Changes', 'Preview'];

const fallbackTasks: AssistantTask[] = [
  {
    id: 'fallback-task',
    title: 'Create a personal briefing',
    workspace: 'Personal OS',
    status: 'running',
    currentStep: 'Ready to plan',
    mode: 'Ask',
    model: 'Auto',
    provider: 'Auto',
    permissionProfile: 'Guarded',
    riskSummary: ['Guarded mode is active'],
    artifacts: [],
    changes: [],
    messages: [
      {
        id: 'fallback-message',
        role: 'assistant',
        content: 'Jarvis is ready.',
      },
    ],
  },
];

function statusClass(status: AssistantTaskStatus) {
  if (status === 'running') return 'bg-emerald-50 text-emerald-700 border-emerald-200';
  if (status === 'blocked') return 'bg-amber-50 text-amber-800 border-amber-200';
  if (status === 'failed') return 'bg-red-50 text-red-700 border-red-200';
  return 'bg-zinc-100 text-zinc-700 border-zinc-200';
}

function PanelButton({
  active,
  children,
  onClick,
}: {
  active: boolean;
  children: React.ReactNode;
  onClick: () => void;
}) {
  return (
    <button
      type="button"
      aria-pressed={active}
      onClick={onClick}
      className={`h-10 rounded-md border px-3 text-sm font-semibold ${
        active ? 'border-zinc-950 bg-zinc-950 text-white' : 'border-zinc-200 bg-white text-zinc-700 hover:border-teal-300'
      }`}
    >
      {children}
    </button>
  );
}

export default function AssistantPage() {
  const [tasks, setTasks] = useState<AssistantTask[]>([]);
  const [activeTaskId, setActiveTaskId] = useState('');
  const [resultTab, setResultTab] = useState<ResultTab>('Artifacts');
  const [panel, setPanel] = useState<Panel>('remote');
  const [prompt, setPrompt] = useState('Build a weekly research brief with charts');
  const [workspace, setWorkspace] = useState('Personal OS');
  const [workDirectory, setWorkDirectory] = useState('/workspace/assistant');
  const [outputFormat, setOutputFormat] = useState('Document');
  const [mode, setMode] = useState('Plan');
  const [model, setModel] = useState('Auto');
  const [contextReferences, setContextReferences] = useState('@notes @files');
  const [attachments, setAttachments] = useState('');
  const [constraints, setConstraints] = useState('Ask before sharing or overwriting files');
  const [starting, setStarting] = useState(false);
  const [error, setError] = useState('');

  useEffect(() => {
    let mounted = true;

    async function loadTasks() {
      try {
        const response = await fetch('/api/assistant/tasks');
        if (!response.ok) throw new Error('Assistant tasks unavailable');
        const data = await response.json();
        const loadedTasks: AssistantTask[] = data.tasks?.length ? data.tasks : fallbackTasks;
        if (!mounted) return;
        setTasks(loadedTasks);
        setActiveTaskId(loadedTasks[0]?.id || '');
      } catch (loadError: any) {
        if (!mounted) return;
        setError(loadError.message || 'Assistant tasks unavailable');
        setTasks(fallbackTasks);
        setActiveTaskId(fallbackTasks[0].id);
      }
    }

    loadTasks();
    return () => {
      mounted = false;
    };
  }, []);

  const activeTask = useMemo(
    () => tasks.find((task) => task.id === activeTaskId) || tasks[0] || fallbackTasks[0],
    [activeTaskId, tasks],
  );

  const workspaces = useMemo(() => Array.from(new Set(tasks.map((task) => task.workspace))), [tasks]);

  async function startTask() {
    if (!prompt.trim()) return;
    setStarting(true);
    setError('');
    try {
      const response = await fetch('/api/assistant/tasks', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          prompt,
          workspace,
          mode,
          model,
          provider: 'Auto',
          workDirectory,
          outputFormat,
          constraints,
          contextReferences,
          attachments: attachments.split(',').map((item) => item.trim()).filter(Boolean),
          skills: ['Web Research', 'Document Writer', 'Chart Builder'],
          connectors: ['Google Drive', 'Slack'],
          permissionProfile: 'Guarded',
        }),
      });
      const data = await response.json();
      if (!response.ok) throw new Error(data.error || 'Task could not be started');
      setTasks((current) => [data.task, ...current.filter((task) => task.id !== data.task.id)]);
      setActiveTaskId(data.task.id);
      setResultTab('Artifacts');
    } catch (startError: any) {
      setError(startError.message || 'Task could not be started');
    } finally {
      setStarting(false);
    }
  }

  return (
    <div className="min-h-screen bg-zinc-100 text-zinc-950">
      <header className="border-b border-zinc-200 bg-white">
        <div className="mx-auto flex max-w-7xl flex-col gap-4 px-4 py-4 sm:px-6 lg:px-8">
          <div className="flex flex-col gap-3 lg:flex-row lg:items-center lg:justify-between">
            <div>
              <h1 className="text-3xl font-bold tracking-normal">Jarvis Assistant</h1>
              <p className="mt-1 max-w-3xl text-sm text-zinc-600">
                Natural-language workstation for tasks, files, artifacts, remote control, automations, memory, skills, and connectors.
              </p>
            </div>
            <div className="flex flex-wrap gap-2">
              <Link href="/agents" className="inline-flex h-10 items-center rounded-md border border-teal-700 bg-teal-700 px-3 text-sm font-bold text-white">
                Expert Center
              </Link>
              <button type="button" className="h-10 rounded-md border border-zinc-200 bg-white px-3 text-sm font-semibold text-zinc-700">
                New Task
              </button>
            </div>
          </div>
          <nav className="flex flex-wrap gap-2" aria-label="Assistant utilities">
            {([
              ['remote', 'Remote Control'],
              ['automations', 'Automations'],
              ['memory', 'Memory'],
              ['skills', 'Skills'],
              ['connectors', 'Connectors'],
            ] as [Panel, string][]).map(([id, label]) => (
              <PanelButton key={id} active={panel === id} onClick={() => setPanel(id)}>
                {label}
              </PanelButton>
            ))}
          </nav>
        </div>
      </header>

      <main className="mx-auto grid max-w-7xl gap-4 px-4 py-4 sm:px-6 lg:grid-cols-[280px_minmax(0,1fr)_360px] lg:px-8">
        <aside className="rounded-lg border border-zinc-200 bg-white p-4">
          <div className="flex items-center justify-between">
            <h2 className="text-lg font-bold">Workspaces</h2>
            <span className="rounded-md bg-zinc-100 px-2 py-1 text-xs font-semibold text-zinc-600">{tasks.length} tasks</span>
          </div>
          <div className="mt-3 flex flex-wrap gap-2">
            {workspaces.map((name) => (
              <span key={name} className="rounded-md border border-zinc-200 bg-zinc-50 px-2 py-1 text-xs font-semibold text-zinc-700">
                {name}
              </span>
            ))}
          </div>
          <div className="mt-4 space-y-2">
            {tasks.map((task) => (
              <button
                key={task.id}
                type="button"
                onClick={() => setActiveTaskId(task.id)}
                className={`w-full rounded-lg border p-3 text-left ${
                  activeTask.id === task.id ? 'border-teal-700 bg-teal-50' : 'border-zinc-200 bg-white hover:border-teal-300'
                }`}
              >
                <div className="flex items-center justify-between gap-2">
                  <span className="text-xs font-bold uppercase text-zinc-500">{task.workspace}</span>
                  <span className={`rounded-md border px-2 py-1 text-xs font-bold ${statusClass(task.status)}`}>{task.status}</span>
                </div>
                <div className="mt-2 text-sm font-bold leading-5">{task.title}</div>
                <div className="mt-1 text-xs text-zinc-500">{task.currentStep}</div>
              </button>
            ))}
          </div>
        </aside>

        <section className="space-y-4">
          <section className="rounded-lg border border-zinc-200 bg-white p-4">
            <div className="flex flex-col gap-3 md:flex-row md:items-start md:justify-between">
              <div>
                <h2 className="text-xl font-bold">{activeTask.title}</h2>
                <p className="mt-1 text-sm text-zinc-600">{activeTask.currentStep}</p>
              </div>
              <div className="flex flex-wrap gap-2">
                <span className={`rounded-md border px-2 py-1 text-xs font-bold ${statusClass(activeTask.status)}`}>{activeTask.status}</span>
                <span className="rounded-md border border-zinc-200 bg-zinc-50 px-2 py-1 text-xs font-bold text-zinc-700">
                  {activeTask.permissionProfile}
                </span>
              </div>
            </div>
            <div className="mt-4 space-y-3">
              {activeTask.messages.map((message) => (
                <div
                  key={message.id}
                  className={`rounded-lg border p-3 ${
                    message.role === 'user' ? 'border-teal-200 bg-teal-50' : 'border-zinc-200 bg-zinc-50'
                  }`}
                >
                  <div className="text-xs font-bold uppercase text-zinc-500">{message.role}</div>
                  <p className="mt-1 text-sm leading-6 text-zinc-800">{message.content}</p>
                </div>
              ))}
            </div>
          </section>

          <section className="rounded-lg border border-zinc-200 bg-white p-4">
            <h2 className="text-lg font-bold">Task Composer</h2>
            <div className="mt-4 grid gap-3">
              <label className="text-sm font-bold text-zinc-700">
                Task prompt
                <textarea
                  aria-label="Task prompt"
                  value={prompt}
                  onChange={(event) => setPrompt(event.target.value)}
                  className="mt-1 min-h-[96px] w-full resize-none rounded-md border border-zinc-300 p-3 text-sm leading-6 outline-none focus:border-teal-600 focus:ring-2 focus:ring-teal-100"
                />
              </label>
              <div className="grid gap-3 md:grid-cols-3">
                <label className="text-sm font-bold text-zinc-700">
                  Workspace
                  <input
                    aria-label="Workspace"
                    value={workspace}
                    onChange={(event) => setWorkspace(event.target.value)}
                    className="mt-1 h-10 w-full rounded-md border border-zinc-300 px-3 text-sm"
                  />
                </label>
                <label className="text-sm font-bold text-zinc-700">
                  Work directory
                  <input
                    aria-label="Work directory"
                    value={workDirectory}
                    onChange={(event) => setWorkDirectory(event.target.value)}
                    className="mt-1 h-10 w-full rounded-md border border-zinc-300 px-3 text-sm"
                  />
                </label>
                <label className="text-sm font-bold text-zinc-700">
                  Output format
                  <select
                    aria-label="Output format"
                    value={outputFormat}
                    onChange={(event) => setOutputFormat(event.target.value)}
                    className="mt-1 h-10 w-full rounded-md border border-zinc-300 bg-white px-3 text-sm"
                  >
                    <option>Document</option>
                    <option>Spreadsheet</option>
                    <option>Presentation</option>
                    <option>PDF</option>
                  </select>
                </label>
              </div>
              <div className="grid gap-3 md:grid-cols-4">
                <label className="text-sm font-bold text-zinc-700">
                  Mode
                  <select value={mode} onChange={(event) => setMode(event.target.value)} className="mt-1 h-10 w-full rounded-md border border-zinc-300 bg-white px-3 text-sm">
                    <option>Ask</option>
                    <option>Craft</option>
                    <option>Plan</option>
                  </select>
                </label>
                <label className="text-sm font-bold text-zinc-700">
                  Model
                  <select value={model} onChange={(event) => setModel(event.target.value)} className="mt-1 h-10 w-full rounded-md border border-zinc-300 bg-white px-3 text-sm">
                    <option>Auto</option>
                    <option>MiniMax-M3</option>
                    <option>OpenAI GPT-4.1</option>
                    <option>Claude Sonnet</option>
                    <option>Local Ollama</option>
                  </select>
                </label>
                <label className="text-sm font-bold text-zinc-700">
                  Context
                  <input value={contextReferences} onChange={(event) => setContextReferences(event.target.value)} className="mt-1 h-10 w-full rounded-md border border-zinc-300 px-3 text-sm" />
                </label>
                <label className="text-sm font-bold text-zinc-700">
                  Attachments
                  <input value={attachments} onChange={(event) => setAttachments(event.target.value)} className="mt-1 h-10 w-full rounded-md border border-zinc-300 px-3 text-sm" />
                </label>
              </div>
              <label className="text-sm font-bold text-zinc-700">
                Constraints
                <input value={constraints} onChange={(event) => setConstraints(event.target.value)} className="mt-1 h-10 w-full rounded-md border border-zinc-300 px-3 text-sm" />
              </label>
              {error && <p className="text-sm font-semibold text-red-700">{error}</p>}
              <div className="flex flex-wrap items-center justify-between gap-3">
                <div className="flex flex-wrap gap-2">
                  {['@ files', 'Screenshot', 'Guarded', 'Parallel tasks'].map((item) => (
                    <span key={item} className="rounded-md border border-zinc-200 bg-zinc-50 px-2 py-1 text-xs font-bold text-zinc-700">{item}</span>
                  ))}
                </div>
                <button
                  type="button"
                  onClick={startTask}
                  disabled={starting || !prompt.trim()}
                  className="h-11 rounded-md bg-zinc-950 px-5 text-sm font-bold text-white hover:bg-zinc-800 disabled:bg-zinc-400"
                >
                  {starting ? 'Starting...' : 'Start Task'}
                </button>
              </div>
            </div>
          </section>

          <FeaturePanel panel={panel} />
        </section>

        <aside className="rounded-lg border border-zinc-200 bg-white p-4">
          <h2 className="text-lg font-bold">Results</h2>
          <div className="mt-3 grid grid-cols-4 gap-2">
            {resultTabs.map((tab) => (
              <button
                key={tab}
                type="button"
                onClick={() => setResultTab(tab)}
                aria-pressed={resultTab === tab}
                className={`h-9 rounded-md border text-xs font-bold ${
                  resultTab === tab ? 'border-teal-700 bg-teal-700 text-white' : 'border-zinc-200 bg-white text-zinc-700'
                }`}
              >
                {tab}
              </button>
            ))}
          </div>
          <ResultContent task={activeTask} tab={resultTab} />
          <div className="mt-4 flex flex-wrap gap-2">
            {['Share', 'Download', 'Copy', 'Archive'].map((action) => (
              <button key={action} type="button" className="rounded-md border border-zinc-200 bg-zinc-50 px-3 py-2 text-xs font-bold text-zinc-700">
                {action}
              </button>
            ))}
          </div>
        </aside>
      </main>
    </div>
  );
}

function ResultContent({ task, tab }: { task: AssistantTask; tab: ResultTab }) {
  if (tab === 'Artifacts') {
    return (
      <div className="mt-4 space-y-2">
        {task.artifacts.length === 0 && <p className="text-sm text-zinc-500">No artifacts yet.</p>}
        {task.artifacts.map((artifact) => (
          <div key={artifact.id} className="rounded-md border border-zinc-200 bg-zinc-50 p-3">
            <div className="text-sm font-bold">{artifact.filename}</div>
            <div className="text-xs uppercase text-zinc-500">{artifact.type}</div>
          </div>
        ))}
      </div>
    );
  }

  if (tab === 'All Files') {
    return (
      <div className="mt-4 space-y-2">
        {task.changes.map((change) => (
          <div key={change.id} className="rounded-md border border-zinc-200 bg-zinc-50 p-3 text-sm text-zinc-700">{change.path}</div>
        ))}
        {task.artifacts.map((artifact) => (
          <div key={artifact.id} className="rounded-md border border-zinc-200 bg-zinc-50 p-3 text-sm text-zinc-700">{artifact.filename}</div>
        ))}
      </div>
    );
  }

  if (tab === 'Changes') {
    return (
      <div className="mt-4 space-y-2">
        {task.changes.length === 0 && <p className="text-sm text-zinc-500">No file changes yet.</p>}
        {task.changes.map((change) => (
          <div key={change.id} className="rounded-md border border-zinc-200 bg-zinc-50 p-3">
            <div className="text-sm font-bold">{change.summary}</div>
            <div className="mt-1 text-xs font-semibold text-amber-700">{change.approvalStatus}</div>
          </div>
        ))}
      </div>
    );
  }

  return (
    <div className="mt-4 space-y-2">
      {task.artifacts.length === 0 && <p className="text-sm text-zinc-500">Preview appears after the first artifact.</p>}
      {task.artifacts.map((artifact) => (
        <div key={artifact.id} className="rounded-md border border-zinc-200 bg-zinc-50 p-3 text-sm leading-6 text-zinc-700">
          {artifact.preview}
        </div>
      ))}
    </div>
  );
}

function FeaturePanel({ panel }: { panel: Panel }) {
  if (panel === 'remote') {
    return (
      <section className="rounded-lg border border-zinc-200 bg-white p-4">
        <h2 className="text-lg font-bold">Remote Control</h2>
        <div className="mt-3 grid gap-3 md:grid-cols-3">
          {['Slack', 'Telegram', 'Discord'].map((platform) => (
            <div key={platform} className="rounded-md border border-zinc-200 bg-zinc-50 p-3">
              <div className="font-bold">{platform}</div>
              <div className="mt-1 text-sm text-zinc-600">Connected task intake</div>
            </div>
          ))}
        </div>
      </section>
    );
  }

  if (panel === 'automations') {
    return (
      <section className="rounded-lg border border-zinc-200 bg-white p-4">
        <h2 className="text-lg font-bold">Automations</h2>
        <div className="mt-3 grid gap-3 md:grid-cols-2">
          <div className="rounded-md border border-zinc-200 bg-zinc-50 p-3">
            <div className="font-bold">Weekly research brief</div>
            <div className="mt-1 text-sm text-zinc-600">Every Monday 09:00</div>
          </div>
          <div className="rounded-md border border-zinc-200 bg-zinc-50 p-3">
            <div className="font-bold">Execution history</div>
            <div className="mt-1 text-sm text-zinc-600">Runs, approvals, outputs, and notifications</div>
          </div>
        </div>
      </section>
    );
  }

  if (panel === 'memory') {
    return (
      <section className="rounded-lg border border-zinc-200 bg-white p-4">
        <h2 className="text-lg font-bold">Memory</h2>
        <div className="mt-3 space-y-2">
          <div className="rounded-md border border-zinc-200 bg-zinc-50 p-3 text-sm">Prefer concise technical summaries with citations.</div>
          <div className="rounded-md border border-zinc-200 bg-zinc-50 p-3 text-sm">Ask before sending messages or modifying original files.</div>
        </div>
        <button type="button" className="mt-3 rounded-md border border-zinc-200 bg-white px-3 py-2 text-sm font-bold text-zinc-700">
          Import Memory
        </button>
      </section>
    );
  }

  if (panel === 'skills') {
    return (
      <section className="rounded-lg border border-zinc-200 bg-white p-4">
        <h2 className="text-lg font-bold">Skill Marketplace</h2>
        <div className="mt-3 grid gap-3 md:grid-cols-3">
          {['Web Research', 'Document Writer', 'Chart Builder'].map((skill) => (
            <div key={skill} className="rounded-md border border-zinc-200 bg-zinc-50 p-3">
              <div className="font-bold">{skill}</div>
              <div className="mt-1 text-sm text-zinc-600">Installed</div>
            </div>
          ))}
        </div>
      </section>
    );
  }

  return (
    <section aria-label="Connector panel" className="rounded-lg border border-zinc-200 bg-white p-4">
      <h2 className="text-lg font-bold">Connectors</h2>
      <div className="mt-3 grid gap-3 md:grid-cols-3">
        {['Google Drive', 'Slack', 'MCP Endpoint'].map((connector) => (
          <div key={connector} className="rounded-md border border-zinc-200 bg-zinc-50 p-3">
            <div className="font-bold">{connector}</div>
            <div className="mt-1 text-sm text-zinc-600">Available</div>
          </div>
        ))}
      </div>
    </section>
  );
}
