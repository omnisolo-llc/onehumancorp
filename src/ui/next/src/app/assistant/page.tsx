'use client';

import { useEffect, useMemo, useState } from 'react';
import { AppShell } from '../components/AppShell';
import styles from './assistant.module.css';
import { InteractiveWalkthrough, WalkthroughTarget } from "../../components/Walkthrough";

type AssistantTaskStatus = 'running' | 'completed' | 'blocked' | 'failed' | 'planning' | 'pending' | 'archived';
type Section =
  | 'tasks'
  | 'compose'
  | 'conversation'
  | 'results'
  | 'automations'
  | 'memory'
  | 'skills'
  | 'connectors'
  | 'data'
  | 'cloud'
  | 'billing'
  | 'permissions'
  | 'models'
  | 'system'
  | 'parity';
type ResultTab = 'Artifacts' | 'All Files' | 'Changes' | 'Preview';

type AssistantArtifact = {
  id: string;
  type: string;
  filename: string;
  preview?: string;
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
  tool_metadata_json?: any;
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
  permissionProfile: string;
  riskSummary: string[];
  artifacts: AssistantArtifact[];
  changes: AssistantChange[];
  messages: AssistantMessage[];
  createdAt?: string;
  updatedAt?: string;
};

type AssistantCapabilities = {
  outputFormats?: string[];
  workModes?: string[];
  modelProviders?: string[];
};

const sections: [Section, string][] = [
  ['tasks', 'Task List'],
  ['compose', 'New Task'],
  ['conversation', 'Conversation'],
  ['results', 'Results'],
  ['automations', 'Automations'],
  ['memory', 'Memory'],
  ['skills', 'Skills'],
  ['connectors', 'Connectors'],
  ['data', 'Data'],
  ['cloud', 'Cloud'],
  ['billing', 'Billing'],
  ['permissions', 'Permissions'],
  ['models', 'Models'],
  ['system', 'System'],
  ['parity', 'Parity Audit'],
];

const resourceConfig: Partial<Record<Section, { title: string; endpoint: string; rootKeys: string[] }>> = {
  automations: { title: 'Automations', endpoint: '/api/assistant/automations', rootKeys: ['automations'] },
  memory: { title: 'Memory', endpoint: '/api/assistant/memory', rootKeys: ['memories'] },
  skills: { title: 'Skills', endpoint: '/api/assistant/skills', rootKeys: ['skills'] },
  connectors: { title: 'Connectors', endpoint: '/api/assistant/connectors', rootKeys: ['connectors'] },
  data: { title: 'Data', endpoint: '/api/assistant/data', rootKeys: ['sharedFiles', 'archivedTasks', 'unshareQueue'] },
  cloud: { title: 'Cloud', endpoint: '/api/assistant/cloud', rootKeys: ['sessions'] },
  billing: { title: 'Billing', endpoint: '/api/assistant/billing', rootKeys: [] },
  permissions: { title: 'Permissions', endpoint: '/api/assistant/permissions', rootKeys: ['authorizedFolders', 'rules'] },
  models: { title: 'Models', endpoint: '/api/assistant/models', rootKeys: ['models', 'runtime'] },
  system: { title: 'System', endpoint: '/api/assistant/settings', rootKeys: ['settings'] },
  parity: { title: 'Parity Audit', endpoint: '/api/assistant/parity', rootKeys: ['summary', 'categories'] },
};

const resultTabs: ResultTab[] = ['Artifacts', 'All Files', 'Changes', 'Preview'];
const fallbackCapabilities: Required<AssistantCapabilities> = {
  outputFormats: ['Document', 'Presentation', 'PDF', 'Code App'],
  workModes: ['Ask', 'Agent', 'Plan', 'Coding'],
  modelProviders: ['Auto', 'Agent'],
};

function cx(...classes: Array<string | false | undefined>) {
  return classes.filter(Boolean).join(' ');
}

function statusClass(status: AssistantTaskStatus) {
  if (status === 'running') return styles.statusRunning;
  if (status === 'blocked') return styles.statusBlocked;
  if (status === 'failed') return styles.statusFailed;
  if (status === 'planning') return styles.statusPlanning;
  if (status === 'pending') return styles.statusPending;
  return styles.statusNeutral;
}

function SectionButton({
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
      className={cx(styles.panelButton, active ? styles.panelButtonActive : styles.panelButtonIdle)}
    >
      {children}
    </button>
  );
}

export default function AssistantPage() {
  const [tasks, setTasks] = useState<AssistantTask[]>([]);
  const [capabilities, setCapabilities] = useState<Required<AssistantCapabilities>>(fallbackCapabilities);
  const [activeTaskId, setActiveTaskId] = useState('');
  const [section, setSection] = useState<Section>('tasks');
  const [resultTab, setResultTab] = useState<ResultTab>('Artifacts');
  const [taskSearch, setTaskSearch] = useState('');
  const [taskStatusFilter, setTaskStatusFilter] = useState<'all' | AssistantTaskStatus>('all');
  const [taskDateFilter, setTaskDateFilter] = useState<'all' | 'today' | 'this_week' | 'older'>('all');
  const [prompt, setPrompt] = useState('Build a weekly research brief with charts');
  const [workspace, setWorkspace] = useState('Personal OS');
  const [workDirectory, setWorkDirectory] = useState('/workspace/assistant');
  const [outputFormat, setOutputFormat] = useState('Document');
  const [mode, setMode] = useState('Plan');
  const [model, setModel] = useState('Auto');
  const [constraints, setConstraints] = useState('Ask before sharing or overwriting files');
  const [starting, setStarting] = useState(false);
  const [error, setError] = useState('');
  const [actionNotice, setActionNotice] = useState('');
  const [agentName, setAgentName] = useState('Agent');
  const [resourceData, setResourceData] = useState<Record<string, any>>({});
  const [resourceLoading, setResourceLoading] = useState('');
  const [resourceError, setResourceError] = useState('');

  const [isWalkthroughOpen, setIsWalkthroughOpen] = useState(false);
  const [walkthroughSteps, setWalkthroughSteps] = useState<any[]>([]);

  useEffect(() => {
    fetch("/api/walkthrough/assistant")
      .then((res) => (res.ok ? res.json() : []))
      .then((data) => {
        if (Array.isArray(data)) {
          setWalkthroughSteps(data);
        }
      })
      .catch((err) => console.error("Walkthrough fetch failed:", err));

    let mounted = true;

    async function loadTasks() {
      try {
        const response = await fetch('/api/assistant/tasks');
        const data = await response.json().catch(() => ({}));
        if (!response.ok) throw new Error(data.error || 'Assistant tasks unavailable');
        if (!mounted) return;

        const loadedTasks: AssistantTask[] = Array.isArray(data.tasks) ? data.tasks : [];
        setTasks(loadedTasks);
        setActiveTaskId(loadedTasks[0]?.id || '');
        setCapabilities({
          outputFormats: data.capabilities?.outputFormats?.length ? data.capabilities.outputFormats : fallbackCapabilities.outputFormats,
          workModes: data.capabilities?.workModes?.length ? data.capabilities.workModes : fallbackCapabilities.workModes,
          modelProviders: data.capabilities?.modelProviders?.length ? data.capabilities.modelProviders : fallbackCapabilities.modelProviders,
        });
      } catch (loadError: any) {
        if (!mounted) return;
        setError(loadError.message || 'Assistant tasks unavailable');
      }
    }

    async function loadSettings() {
      try {
        const response = await fetch('/api/assistant/settings');
        const data = await response.json().catch(() => ({}));
        if (mounted && response.ok && data.settings?.agentName) {
          setAgentName(data.settings.agentName);
        }
      } catch (settingsError) {
        console.error(settingsError);
      }
    }

    Promise.all([loadTasks(), loadSettings()]);
    return () => {
      mounted = false;
    };
  }, []);

  useEffect(() => {
    const config = resourceConfig[section];
    if (!config) return;

    let mounted = true;
    async function loadResource() {
      setResourceLoading(section);
      setResourceError('');
      try {
        const response = await fetch(config.endpoint);
        const data = await response.json().catch(() => ({}));
        if (!response.ok) throw new Error(data.error || `${config.title} unavailable`);
        if (mounted) {
          setResourceData((current) => ({ ...current, [section]: data }));
        }
      } catch (loadError: any) {
        if (mounted) setResourceError(loadError.message || `${config.title} unavailable`);
      } finally {
        if (mounted) setResourceLoading('');
      }
    }

    loadResource();
    return () => {
      mounted = false;
    };
  }, [section]);

  const activeTask = useMemo(
    () => tasks.find((task) => task.id === activeTaskId) || tasks[0],
    [activeTaskId, tasks],
  );

  const visibleTasks = useMemo(() => {
    const query = taskSearch.trim().toLowerCase();
    const startOfToday = new Date();
    startOfToday.setHours(0, 0, 0, 0);
    const startOfWeek = new Date(startOfToday);
    startOfWeek.setDate(startOfToday.getDate() - 6);

    return tasks.filter((task) => {
      const matchesQuery =
        !query ||
        task.title.toLowerCase().includes(query) ||
        task.workspace.toLowerCase().includes(query) ||
        task.currentStep.toLowerCase().includes(query);
      const matchesStatus = taskStatusFilter === 'all' || task.status === taskStatusFilter;
      const taskDate = task.updatedAt || task.createdAt;
      const parsedDate = taskDate ? new Date(taskDate) : null;
      const matchesDate =
        taskDateFilter === 'all' ||
        !parsedDate ||
        (taskDateFilter === 'today' && parsedDate >= startOfToday) ||
        (taskDateFilter === 'this_week' && parsedDate >= startOfWeek) ||
        (taskDateFilter === 'older' && parsedDate < startOfWeek);
      return matchesQuery && matchesStatus && matchesDate;
    });
  }, [taskDateFilter, taskSearch, taskStatusFilter, tasks]);

  async function startTask() {
    if (!prompt.trim()) return;
    setStarting(true);
    setError('');
    setActionNotice('');

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
          permissionProfile: 'Guarded',
        }),
      });
      const data = await response.json().catch(() => ({}));
      if (!response.ok) throw new Error(data.error || 'Task could not be started');
      setTasks((current) => [data.task, ...current.filter((task) => task.id !== data.task.id)]);
      setActiveTaskId(data.task.id);
      setResultTab('Artifacts');
      setSection('results');
    } catch (startError: any) {
      setError(startError.message || 'Task could not be started');
    } finally {
      setStarting(false);
    }
  }

  async function runResultAction(action: 'share' | 'preview') {
    if (!activeTask?.artifacts?.length) return;
    const artifact = activeTask.artifacts[0];
    const request =
      action === 'share'
        ? fetch('/api/assistant/share', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ taskId: activeTask.id, artifactId: artifact.id, target: 'Share Link' }),
          })
        : fetch('/api/assistant/previews', {
            method: 'PATCH',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ action: 'open_external', artifactId: artifact.id }),
          });

    const response = await request;
    const data = await response.json().catch(() => ({}));
    if (!response.ok) {
      setError(data.error || 'Action failed');
      return;
    }
    setActionNotice(action === 'share' ? 'Share link created' : 'Preview opened');
  }

  async function refreshResource(targetSection: Section) {
    const config = resourceConfig[targetSection];
    if (!config) return;
    const response = await fetch(config.endpoint);
    const data = await response.json().catch(() => ({}));
    if (!response.ok) throw new Error(data.error || `${config.title} unavailable`);
    setResourceData((current) => ({ ...current, [targetSection]: data }));
  }

  async function runResourceAction(targetSection: Section, body: Record<string, any>) {
    const config = resourceConfig[targetSection];
    if (!config) return;
    setResourceError('');
    setActionNotice('');
    const response = await fetch(config.endpoint, {
      method: body.method || 'PATCH',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body.payload || body),
    });
    const data = await response.json().catch(() => ({}));
    if (!response.ok) {
      setResourceError(data.error || 'Action failed');
      return;
    }
    setResourceData((current) => ({ ...current, [targetSection]: data }));
    setActionNotice('Action completed');
    await refreshResource(targetSection).catch(() => {});
  }

  return (
    <AppShell
      title={`${agentName} Assistant`}
      subtitle="Task-backed workspace for creating work, reviewing conversations, and inspecting artifacts."
      actions={[{ label: 'Expert Center', href: '/agents' }]}
    >
      <InteractiveWalkthrough
        steps={walkthroughSteps}
        isOpen={isWalkthroughOpen}
        onClose={() => setIsWalkthroughOpen(false)}
      />
      <div className="mb-4 flex flex-wrap gap-2 px-6 pt-4">
         <button
           type="button"
           onClick={() => setIsWalkthroughOpen(true)}
           className="app-button min-h-[44px]"
         >
           Start Tour
         </button>
      </div>

      <div className={styles.shell} data-testid="assistant-shell">
        <main className={styles.workstation} data-testid="assistant-workstation">
        <nav className={cx(styles.panel, styles.sectionMenu)} aria-label="Assistant section menu">
          <div>
            <h2 className={styles.sectionTitle}>Sections</h2>
            <p className={styles.eyebrow}>Real task views</p>
          </div>
          <div className={styles.sectionMenuList} data-testid="assistant-section-list">
            {sections.map(([id, label]) => (
              <SectionButton key={id} active={section === id} onClick={() => setSection(id)}>
                {label}
              </SectionButton>
            ))}
          </div>
        </nav>

        <section className={styles.centerColumn}>
          {section === 'tasks' && (
            <TaskListPage
              activeTaskId={activeTask?.id || ''}
              taskCount={tasks.length}
              visibleTasks={visibleTasks}
              shownCountLabel={`${visibleTasks.length} ${visibleTasks.length === 1 ? 'task' : 'tasks'} shown`}
              taskSearch={taskSearch}
              taskStatusFilter={taskStatusFilter}
              taskDateFilter={taskDateFilter}
              onSearch={setTaskSearch}
              onStatusFilter={setTaskStatusFilter}
              onDateFilter={setTaskDateFilter}
              onReset={() => {
                setTaskSearch('');
                setTaskStatusFilter('all');
                setTaskDateFilter('all');
              }}
              onSelect={(id) => {
                setActiveTaskId(id);
              }}
            />
          )}

          {section === 'compose' && (
            <section className={styles.panel}>
              <h2 className={styles.sectionTitle}>New Task</h2>
              <div className={styles.fieldGrid}>
                <WalkthroughTarget id="ohc-help-input-area">
                  <label className={styles.fieldLabel}>
                    Task prompt
                    <textarea aria-label="Task prompt" value={prompt} onChange={(event) => setPrompt(event.target.value)} className={styles.textarea} />
                  </label>
                </WalkthroughTarget>
                <div className={styles.formGridThree}>
                  <label className={styles.fieldLabel}>
                    Workspace
                    <input aria-label="Workspace" value={workspace} onChange={(event) => setWorkspace(event.target.value)} className={styles.input} />
                  </label>
                  <label className={styles.fieldLabel}>
                    Work directory
                    <input aria-label="Work directory" value={workDirectory} onChange={(event) => setWorkDirectory(event.target.value)} className={styles.input} />
                  </label>
                  <label className={styles.fieldLabel}>
                    Output format
                    <select aria-label="Output format" value={outputFormat} onChange={(event) => setOutputFormat(event.target.value)} className={styles.select}>
                      {capabilities.outputFormats.map((option) => <option key={option}>{option}</option>)}
                    </select>
                  </label>
                </div>
                <div className={styles.formGridThree}>
                  <label className={styles.fieldLabel}>
                    Mode
                    <select aria-label="Mode" value={mode} onChange={(event) => setMode(event.target.value)} className={styles.select}>
                      {capabilities.workModes.map((option) => <option key={option}>{option}</option>)}
                    </select>
                  </label>
                  <label className={styles.fieldLabel}>
                    Model
                    <select aria-label="Model" value={model} onChange={(event) => setModel(event.target.value)} className={styles.select}>
                      {capabilities.modelProviders.map((option) => <option key={option}>{option}</option>)}
                    </select>
                  </label>
                  <label className={styles.fieldLabel}>
                    Constraints
                    <input value={constraints} onChange={(event) => setConstraints(event.target.value)} className={styles.input} />
                  </label>
                </div>
                {error && <p className={styles.error}>{error}</p>}
                <button type="button" onClick={startTask} disabled={starting || !prompt.trim()} className={styles.startButton}>
                  {starting ? 'Starting...' : 'Start Task'}
                </button>
              </div>
            </section>
          )}

          {section === 'conversation' && <ConversationPage task={activeTask} />}
          {section === 'results' && (
            <ResultsPage
              task={activeTask}
              resultTab={resultTab}
              onTab={setResultTab}
              onShare={() => runResultAction('share')}
              onPreview={() => runResultAction('preview')}
            />
          )}
          {!!resourceConfig[section] && (
            <ResourcePage
              section={section}
              config={resourceConfig[section]!}
              data={resourceData[section]}
              loading={resourceLoading === section}
              error={resourceError}
              onAction={runResourceAction}
            />
          )}

          {actionNotice && <div className={styles.resultItem} role="status">{actionNotice}</div>}
        </section>
        </main>
      </div>
    </AppShell>
  );
}

function TaskListPage({
  activeTaskId,
  taskCount,
  visibleTasks,
  shownCountLabel,
  taskSearch,
  taskStatusFilter,
  taskDateFilter,
  onSearch,
  onStatusFilter,
  onDateFilter,
  onReset,
  onSelect,
}: {
  activeTaskId: string;
  taskCount: number;
  visibleTasks: AssistantTask[];
  shownCountLabel: string;
  taskSearch: string;
  taskStatusFilter: 'all' | AssistantTaskStatus;
  taskDateFilter: 'all' | 'today' | 'this_week' | 'older';
  onSearch: (value: string) => void;
  onStatusFilter: (value: 'all' | AssistantTaskStatus) => void;
  onDateFilter: (value: 'all' | 'today' | 'this_week' | 'older') => void;
  onReset: () => void;
  onSelect: (id: string) => void;
}) {
  return (
    <section className={styles.panel} aria-label="Task rail">
      <div className={styles.sectionHeader}>
        <div>
          <h2 className={styles.sectionTitle}>Task List</h2>
          <p className={styles.eyebrow}>Database tasks</p>
        </div>
        <span className={styles.countBadge}>{taskCount} {taskCount === 1 ? 'task' : 'tasks'}</span>
      </div>
      <div className={styles.taskTools}>
        <input aria-label="Search tasks" value={taskSearch} onChange={(event) => onSearch(event.target.value)} className={styles.input} placeholder="Search tasks" />
        <select aria-label="Task status filter" value={taskStatusFilter} onChange={(event) => onStatusFilter(event.target.value as 'all' | AssistantTaskStatus)} className={styles.select}>
          <option value="all">All statuses</option>
          <option value="running">Running</option>
          <option value="completed">Completed</option>
          <option value="blocked">Blocked</option>
          <option value="failed">Failed</option>
          <option value="planning">Planning</option>
          <option value="pending">Pending</option>
          <option value="archived">Archived</option>
        </select>
        <select aria-label="Task date filter" value={taskDateFilter} onChange={(event) => onDateFilter(event.target.value as 'all' | 'today' | 'this_week' | 'older')} className={styles.select}>
          <option value="all">All dates</option>
          <option value="today">Today</option>
          <option value="this_week">This week</option>
          <option value="older">Older</option>
        </select>
      </div>
      <div className={styles.filterMeta}>
        <span>{shownCountLabel}</span>
        <button type="button" onClick={onReset} className={styles.inlineButton}>Reset task filters</button>
      </div>
      <div className={styles.taskList}>
        {visibleTasks.length === 0 && <p className={styles.emptyText}>No matching tasks.</p>}
        {visibleTasks.map((task) => (
          <button key={task.id} type="button" onClick={() => onSelect(task.id)} className={cx(styles.taskCard, activeTaskId === task.id && styles.taskCardActive)}>
            <div className={styles.metaRow}>
              <span className={styles.overline}>{task.workspace}</span>
              <span className={cx(styles.statusBadge, statusClass(task.status))}>{task.status}</span>
            </div>
            <div className={styles.taskTitle}>{task.title}</div>
            <div className={styles.mutedText}>{task.currentStep}</div>
          </button>
        ))}
      </div>
    </section>
  );
}

function ConversationPage({ task }: { task?: AssistantTask }) {
  if (!task) {
    return (
      <section className={styles.panel}>
        <h2 className={styles.sectionTitle}>Conversation</h2>
        <p className={styles.emptyText}>Select or create a task to view its conversation.</p>
      </section>
    );
  }

  return (
    <section className={styles.panel}>
      <div className={styles.conversationHeader}>
        <div>
          <div className={styles.overline}>Conversation</div>
          <h2 className={styles.conversationTitle}>{task.title}</h2>
          <p className={styles.mutedText}>{task.currentStep}</p>
        </div>
        <span className={cx(styles.statusBadge, statusClass(task.status))}>{task.status}</span>
      </div>
      <div className={styles.messageList}>
        {task.messages.map((message) => (
          <div key={message.id} className={cx(styles.message, message.role === 'user' ? styles.userMessage : styles.assistantMessage)}>
            <div className={styles.overline}>{message.role}</div>
            <p className={styles.messageText}>{message.content}</p>
            {message.tool_metadata_json?.proposed_action && (
              <div className="mt-4 p-4 border border-blue-200 rounded-lg bg-blue-50/50">
                <h4 className="font-bold text-sm text-blue-900 mb-2">Proposed Action</h4>
                <pre className="text-[11px] bg-white border border-gray-100 p-3 rounded text-gray-700 overflow-x-auto whitespace-pre-wrap">
                  {JSON.stringify(message.tool_metadata_json.proposed_action, null, 2)}
                </pre>
                <div className="mt-4 flex gap-3">
                  <button onClick={async () => {
                      if (!task) return;
                      await fetch(`/api/assistant/tasks/${task.id}`, { method: 'PATCH', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ action: 'approve_action' }) });
                  }} className="flex-1 bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded transition-colors text-sm">
                    Approve & Execute
                  </button>
                  <button className="px-4 py-2 text-gray-600 hover:bg-gray-100 font-medium rounded transition-colors text-sm border border-gray-200">
                    Reject
                  </button>
                </div>
              </div>
            )}
          </div>
        ))}
      </div>
    </section>
  );
}

function ResultsPage({
  task,
  resultTab,
  onTab,
  onShare,
  onPreview,
}: {
  task?: AssistantTask;
  resultTab: ResultTab;
  onTab: (tab: ResultTab) => void;
  onShare: () => void;
  onPreview: () => void;
}) {
  return (
    <section className={styles.panel}>
      <h2 className={styles.sectionTitle}>Results</h2>
      <div className={styles.tabGrid}>
        {resultTabs.map((tab) => (
          <button key={tab} type="button" onClick={() => onTab(tab)} aria-pressed={resultTab === tab} className={cx(styles.tabButton, resultTab === tab && styles.tabButtonActive)}>
            {tab}
          </button>
        ))}
      </div>
      {task ? <ResultContent task={task} tab={resultTab} /> : <div className={styles.resultList}><p className={styles.emptyText}>Select or create a task to inspect results.</p></div>}
      {!!task?.artifacts?.length && (
        <div className={styles.resultActions}>
          <button type="button" onClick={onShare} className={styles.smallButton}>Share Link</button>
          <button type="button" onClick={onPreview} className={styles.smallButton}>Open Preview</button>
        </div>
      )}
    </section>
  );
}

function ResourcePage({
  section,
  config,
  data,
  loading,
  error,
  onAction,
}: {
  section: Section;
  config: { title: string; endpoint: string; rootKeys: string[] };
  data: any;
  loading: boolean;
  error: string;
  onAction: (section: Section, body: Record<string, any>) => void;
}) {
  const [folder, setFolder] = useState('/workspace/assistant');
  const [agentNameInput, setAgentNameInput] = useState('');
  const [customConnector, setCustomConnector] = useState('');
  const [customSkill, setCustomSkill] = useState('');
  const [observationMasking, setObservationMasking] = useState(
    data?.settings?.observationMasking ?? true
  );

  useEffect(() => {
    if (data?.settings?.observationMasking !== undefined) {
      setObservationMasking(data.settings.observationMasking);
    }
  }, [data?.settings?.observationMasking]);

  const blocks = resourceBlocks(data, config.rootKeys);

  return (
    <section className={styles.panel}>
      <div className={styles.sectionHeader}>
        <div>
          <h2 className={styles.sectionTitle}>{config.title}</h2>
          <p className={styles.eyebrow}>{config.endpoint}</p>
        </div>
      </div>
      {loading && <p className={styles.emptyText}>Loading...</p>}
      {error && <p className={styles.error}>{error}</p>}
      {!loading && !data && <p className={styles.emptyText}>No data loaded yet.</p>}

      {section === 'skills' && (
        <div className={styles.inlineForm}>
          <input aria-label="Skill name" value={customSkill} onChange={(event) => setCustomSkill(event.target.value)} className={styles.input} placeholder="Skill name" />
          <button
            type="button"
            className={styles.smallButton}
            disabled={!customSkill.trim()}
            onClick={() => {
              onAction(section, { action: 'install', name: customSkill.trim(), category: 'Custom' });
              setCustomSkill('');
            }}
          >
            Install Skill
          </button>
        </div>
      )}

      {section === 'connectors' && (
        <div className={styles.inlineForm}>
          <input aria-label="Connector name" value={customConnector} onChange={(event) => setCustomConnector(event.target.value)} className={styles.input} placeholder="Connector name" />
          <button
            type="button"
            className={styles.smallButton}
            disabled={!customConnector.trim()}
            onClick={() => {
              onAction(section, { action: 'connect', name: customConnector.trim(), kind: 'custom' });
              setCustomConnector('');
            }}
          >
            Connect
          </button>
        </div>
      )}

      {section === 'permissions' && (
        <div className={styles.inlineForm}>
          <input aria-label="Authorized folder" value={folder} onChange={(event) => setFolder(event.target.value)} className={styles.input} />
          <button type="button" className={styles.smallButton} onClick={() => onAction(section, { action: 'grant', folder })}>Grant Folder</button>
          <button type="button" className={styles.smallButton} onClick={() => onAction(section, { action: 'revoke', folder })}>Revoke Folder</button>
        </div>
      )}

      {section === 'system' && (
        <div className={styles.inlineForm}>
          <input aria-label="Assistant name" value={agentNameInput} onChange={(event) => setAgentNameInput(event.target.value)} className={styles.input} placeholder="Assistant name" />
          <button
            type="button"
            className={styles.smallButton}
            disabled={!agentNameInput.trim()}
            onClick={() => onAction(section, { agentName: agentNameInput.trim() })}
          >
            Save Name
          </button>
        </div>
      )}

      {section === 'system' && (
        <div className={styles.resourceBlock}>
          <div className={styles.featureGridTwo}>
            <div className={styles.featureCard}>
              <div className={styles.cardTitle}>Observation Masking</div>
              <p className={styles.eyebrow}>Hides the raw output of old tools from the prompt, but keeps the tool_calls themselves visible so the model remembers what it did.</p>
              <div style={{ marginTop: '1rem', display: 'flex', alignItems: 'center', gap: '1rem' }}>
                <label style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', cursor: 'pointer' }}>
                  <input
                    type="checkbox"
                    checked={observationMasking}
                    onChange={(e) => setObservationMasking(e.target.checked)}
                    aria-label="Observation Masking Toggle"
                  />
                  <span className={styles.eyebrow} style={{ margin: 0 }}>Enable Masking</span>
                </label>
                <button
                  type="button"
                  className={styles.smallButton}
                  onClick={() => onAction(section, { observationMasking })}
                >
                  Save UI Settings
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

      <div className={styles.resourceStack}>
        {blocks.map((block) => (
          <div key={block.title} className={styles.resourceBlock}>
            <h3 className={styles.resourceTitle}>{block.title}</h3>
            {block.items.length === 0 ? (
              <p className={styles.emptyText}>No records.</p>
            ) : (
              <div className={styles.featureGridTwo}>
                {block.items.map((item, index) => (
                  <div key={item.id || item.name || item.title || `${block.title}-${index}`} className={styles.featureCard}>
                    <div className={styles.cardTitle}>{recordTitle(item)}</div>
                    <dl className={styles.recordFields}>
                      {recordEntries(item).map(([key, value]) => (
                        <div key={key}>
                          <dt>{labelFor(key)}</dt>
                          <dd>{formatValue(value)}</dd>
                        </div>
                      ))}
                    </dl>
                    <ResourceActions section={section} item={item} onAction={onAction} />
                  </div>
                ))}
              </div>
            )}
          </div>
        ))}
      </div>
    </section>
  );
}

function ResourceActions({
  section,
  item,
  onAction,
}: {
  section: Section;
  item: any;
  onAction: (section: Section, body: Record<string, any>) => void;
}) {
  if (section === 'automations' && item.id) {
    return (
      <div className={styles.actionRow}>
        <button type="button" className={styles.smallButton} onClick={() => onAction(section, { action: item.status === 'active' ? 'pause' : 'resume', id: item.id })}>
          {item.status === 'active' ? 'Pause' : 'Resume'}
        </button>
        <button type="button" className={styles.smallButton} onClick={() => onAction(section, { action: 'run_now', id: item.id })}>Run Now</button>
        <button type="button" className={styles.smallButton} onClick={() => onAction(section, { action: 'delete', id: item.id })}>Delete</button>
      </div>
    );
  }

  if (section === 'memory' && item.id) {
    return (
      <div className={styles.actionRow}>
        <button type="button" className={styles.smallButton} onClick={() => onAction(section, { action: 'forget', id: item.id })}>Forget</button>
      </div>
    );
  }

  if (section === 'skills' && item.name) {
    const action = item.status === 'installed' ? 'disable' : 'install';
    return (
      <div className={styles.actionRow}>
        <button type="button" className={styles.smallButton} onClick={() => onAction(section, { action, name: item.name, category: item.category })}>
          {action === 'disable' ? 'Disable' : 'Install'}
        </button>
        {item.status !== 'installed' && (
          <button type="button" className={styles.smallButton} onClick={() => onAction(section, { action: 'uninstall', name: item.name })}>Remove</button>
        )}
      </div>
    );
  }

  if (section === 'connectors' && item.name) {
    const action = item.status === 'connected' ? 'disconnect' : 'connect';
    return (
      <div className={styles.actionRow}>
        <button type="button" className={styles.smallButton} onClick={() => onAction(section, { action, name: item.name, kind: item.kind })}>
          {action === 'disconnect' ? 'Disconnect' : 'Connect'}
        </button>
      </div>
    );
  }

  if (section === 'data' && item.access === 'shared' && item.id) {
    return (
      <div className={styles.actionRow}>
        <button type="button" className={styles.smallButton} onClick={() => onAction(section, { action: 'unshare', id: item.id })}>Queue Unshare</button>
      </div>
    );
  }

  if (section === 'models' && item.provider && item.enabled !== false) {
    return (
      <div className={styles.actionRow}>
        <button type="button" className={styles.smallButton} onClick={() => onAction(section, { action: 'disable', provider: item.provider })}>Disable</button>
      </div>
    );
  }

  return null;
}

function resourceBlocks(data: any, rootKeys: string[]) {
  if (!data) return [];
  if (rootKeys.length === 0) {
    return [{ title: 'Details', items: [data] }];
  }
  return rootKeys.map((key) => {
    let value = data[key];



    // If the component is not seeing 'summary', it means `data` doesn't have it.

    if (value === undefined && data.total !== undefined && key === 'summary') {
      value = data;
    }

    let items = [];
    if (Array.isArray(value)) {
       items = value.map(v => typeof v === 'object' && v !== null ? { ...v, id: v.id || v.name || key } : v);
    } else if (value && typeof value === 'object') {
       // if it's an object, flatten it safely to include its fields explicitly
       const flatItem = { id: key, name: key };
       for (const [k, v] of Object.entries(value)) {
          flatItem[k] = typeof v === 'number' || typeof v === 'boolean' ? String(v) : v;
       }
       items = [flatItem];
    } else if (value !== undefined) {
       items = [{ id: key, name: key, value: String(value) }];
    }

    return {
      title: labelFor(key),
      items,
    };
  });
}

function recordTitle(item: any) {
  return String(item?.name || item?.title || item?.filename || item?.provider || item?.id || 'Record');
}

function recordEntries(item: any) {
  if (!item) return [];
  return Object.entries(item)
    .filter(([key, value]) => !['id', 'name', 'title'].includes(key) && value !== undefined && value !== null && (typeof value !== 'object' || Array.isArray(value)))
    .slice(0, 8);
}

function labelFor(value: string) {
  return value
    .replace(/([a-z])([A-Z])/g, '$1 $2')
    .replace(/_/g, ' ')
    .replace(/\b\w/g, (char) => char.toUpperCase());
}

function formatValue(value: unknown) {
  if (typeof value === 'boolean') return value ? 'Yes' : 'No';
  return String(value);
}

function ResultContent({ task, tab }: { task: AssistantTask; tab: ResultTab }) {
  if (tab === 'Artifacts') {
    return (
      <div className={styles.resultList}>
        {(!task.artifacts || task.artifacts.length === 0) && <p className={styles.emptyText}>No artifacts yet.</p>}
        {(task.artifacts || []).map((artifact) => (
          <div key={artifact.id} className={styles.resultItem}>
            <div className={styles.resultTitle}>{artifact.filename}</div>
            <div className={styles.overline}>{artifact.type}</div>
          </div>
        ))}
      </div>
    );
  }

  if (tab === 'All Files') {
    const files = [...(task.changes || []).map((change) => change.path), ...(task.artifacts || []).map((artifact) => artifact.filename)];
    return (
      <div className={styles.resultList}>
        {files.length === 0 && <p className={styles.emptyText}>No files yet.</p>}
        {files.map((file) => <div key={file} className={styles.resultItem}>{file}</div>)}
      </div>
    );
  }

  if (tab === 'Changes') {
    return (
      <div className={styles.resultList}>
        {(!task.changes || task.changes.length === 0) && <p className={styles.emptyText}>No file changes yet.</p>}
        {(task.changes || []).map((change) => (
          <div key={change.id} className={styles.resultItem}>
            <div className={styles.resultTitle}>{change.summary}</div>
            <div className={cx(styles.statusBadge, styles.warningButton)}>{change.approvalStatus}</div>
          </div>
        ))}
      </div>
    );
  }

  return (
    <div className={styles.resultList}>
      {(!task.artifacts || task.artifacts.length === 0) && <p className={styles.emptyText}>Preview appears after the first artifact.</p>}
      {(task.artifacts || []).map((artifact) => (
        <div key={artifact.id} className={styles.resultItem}>
          {artifact.preview || artifact.filename}
        </div>
      ))}
    </div>
  );
}
