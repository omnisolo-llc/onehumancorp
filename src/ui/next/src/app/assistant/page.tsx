'use client';
import { errorMessage } from '@/lib/errors';
import { isRecord, recordOrEmpty } from '@/lib/records';
import type { Step } from '@/components/Walkthrough';
type ResourceData = Record<string, unknown>;

import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { AppShell } from '../components/AppShell';
import styles from './assistant.module.css';
import { InteractiveWalkthrough, WalkthroughTarget } from "../../components/Walkthrough";

import type { AssistantTask, AssistantTaskStatus } from './taskTypes';
import { assistantTask, useAssistantExecution } from './useAssistantExecution';
type Section =
  | 'tasks'
  | 'compose'
  | 'conversation'
  | 'results'
  | 'automations'
  | 'memory'
  | 'skills'
  | 'connectors'
  | 'remote'
  | 'data'
  | 'cloud'
  | 'billing'
  | 'permissions'
  | 'models'
  | 'system'
  | 'parity';
type ResultTab = 'Artifacts' | 'All Files' | 'Changes' | 'Preview';

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
  ['remote', 'Remote Control'],
  ['data', 'Data'],
  ['cloud', 'Cloud Runtime'],
  ['billing', 'Billing'],
  ['permissions', 'Permissions'],
  ['models', 'Models'],
  ['system', 'System'],
  ['parity', 'Parity Audit'],
];

const resourceConfig: Partial<Record<Section, { title: string; endpoint: string; rootKeys: string[] }>> = {
  automations: { title: 'Automations', endpoint: '/api/v1/assistant/automations', rootKeys: ['automations'] },
  memory: { title: 'Memory', endpoint: '/api/v1/assistant/memory', rootKeys: ['memories'] },
  skills: { title: 'Skills', endpoint: '/api/v1/assistant/skills', rootKeys: ['skills'] },
  connectors: { title: 'Connectors', endpoint: '/api/v1/assistant/connectors', rootKeys: ['connectors'] },
  remote: { title: 'Remote Control', endpoint: '/api/v1/assistant/remote', rootKeys: ['connections'] },
  data: { title: 'Data', endpoint: '/api/v1/assistant/data', rootKeys: ['sharedFiles', 'archivedTasks', 'unshareQueue'] },
  cloud: { title: 'Cloud Runtime', endpoint: '/api/v1/assistant/cloud', rootKeys: ['sessions'] },
  billing: { title: 'Billing', endpoint: '/api/v1/assistant/billing', rootKeys: [] },
  permissions: { title: 'Permissions', endpoint: '/api/v1/assistant/permissions', rootKeys: ['authorizedFolders', 'rules'] },
  models: { title: 'Models', endpoint: '/api/v1/assistant/models', rootKeys: ['models', 'runtime'] },
  system: { title: 'System', endpoint: '/api/v1/assistant/settings', rootKeys: ['settings'] },
  parity: { title: 'Parity Audit', endpoint: '/api/v1/assistant/parity', rootKeys: ['summary', 'categories'] },
};

const resultTabs: ResultTab[] = ['Artifacts', 'All Files', 'Changes', 'Preview'];
const fallbackCapabilities: Required<AssistantCapabilities> = {
  outputFormats: ['Text'],
  workModes: ['Ask'],
  modelProviders: ['Auto'],
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
  const capabilities = fallbackCapabilities;
  const [activeTaskId, setActiveTaskId] = useState('');
  const [section, setSection] = useState<Section>('tasks');
  const [resultTab, setResultTab] = useState<ResultTab>('Artifacts');
  const [taskSearch, setTaskSearch] = useState('');
  const [taskStatusFilter, setTaskStatusFilter] = useState<'all' | AssistantTaskStatus>('all');
  const [taskDateFilter, setTaskDateFilter] = useState<'all' | 'today' | 'this_week' | 'older'>('all');
  const [prompt, setPrompt] = useState('');
  const [workspace, setWorkspace] = useState('Personal OS');
  const workDirectory = '';
  const [outputFormat, setOutputFormat] = useState('Text');
  const [mode, setMode] = useState('Ask');
  const [model, setModel] = useState('Auto');
  const [constraints, setConstraints] = useState('');
  const [error, setError] = useState('');
  const [actionNotice, setActionNotice] = useState('');
  const [agentName, setAgentName] = useState('Agent');
  const [resourceData, setResourceData] = useState<Partial<Record<Section, ResourceData>>>({});
  const [resourceLoading, setResourceLoading] = useState('');
  const [resourceError, setResourceError] = useState('');
  const [nextCursor, setNextCursor] = useState<string | null>(null);
  const [legacyHistory, setLegacyHistory] = useState(false);
  const reads = useRef(0);
  const taskReads = useRef(new Map<string,number>());
  const focusedReceipt = useRef<AssistantTask|null>(null);
  const retire = useCallback(() => {
    reads.current += 1; taskReads.current.clear(); focusedReceipt.current = null; setTasks([]); setActiveTaskId(''); setPrompt(''); setConstraints('');
    setWorkspace('Personal OS'); setTaskSearch(''); setTaskStatusFilter('all'); setTaskDateFilter('all'); setModel('Auto'); setMode('Ask'); setOutputFormat('Text');
    setError(''); setActionNotice(''); setResourceData({}); setResourceError(''); setResourceLoading(''); setAgentName('Agent'); setWalkthroughSteps([]); setIsWalkthroughOpen(false); setNextCursor(null); setLegacyHistory(false);
  }, []);
  const received = useCallback((task: AssistantTask) => {
    reads.current += 1; taskReads.current.set(task.id, (taskReads.current.get(task.id) ?? 0) + 1); focusedReceipt.current = task;
    setTasks(current => [task, ...current.filter(item => item.id !== task.id)]);
    setActiveTaskId(task.id); setResultTab('Artifacts'); setSection('results');
  }, []);
  const restore = useCallback((draft: Record<string,unknown>) => {
    if (typeof draft.prompt === 'string') setPrompt(draft.prompt);
    if (typeof draft.workspace === 'string') setWorkspace(draft.workspace);
    if (typeof draft.constraints === 'string') setConstraints(draft.constraints);
  }, []);
  const execution = useAssistantExecution(retire, received, restore);
  const { read, action, ready, revision } = execution;
  const starting = execution.busy;
  const loadTaskPage = useCallback(async (legacy = false, before: string | null = null, preserveFocused = false) => {
    const sequence = ++reads.current;
    try {
      const data = recordOrEmpty(await read(legacy ? '/api/v1/assistant/legacy-tasks' : `/api/v1/assistant/tasks${before ? `?before=${before}` : ''}`));
      if (sequence !== reads.current) return;
      if (!Array.isArray(data.tasks)) throw new Error('Invalid task list');
      const loaded = data.tasks.map(assistantTask);
      if (loaded.some(task => !task)) throw new Error('Invalid task receipt');
      const focused = preserveFocused ? focusedReceipt.current : null;
      const page = loaded as AssistantTask[];
      setTasks(focused && !page.some(task => task.id === focused.id) ? [focused,...page] : page); setActiveTaskId(focused?.id || page[0]?.id || '');
      setNextCursor(typeof data.nextCursor === 'string' ? data.nextCursor : null); setLegacyHistory(legacy); setError('');
    } catch (cause) { if (sequence === reads.current) setError(errorMessage(cause, 'Assistant tasks unavailable')); }
  }, [read]);
  useEffect(() => { if (ready) void loadTaskPage(false, null, true); return () => { reads.current += 1; }; }, [ready, revision, loadTaskPage]);


  const [isWalkthroughOpen, setIsWalkthroughOpen] = useState(false);
  const [walkthroughSteps, setWalkthroughSteps] = useState<Step[]>([]);
  const [activeTourSteps, setActiveTourSteps] = useState<Step[]>([]);
  const [tourNotice, setTourNotice] = useState('Loading the configured tour…');
  const startTour = () => {
    const available = walkthroughSteps.filter(step => document.getElementById(step.targetId || step.target_id || ''));
    if (!available.length) { setTourNotice('No tour steps are available in this section. Open the relevant section and try again.'); return; }
    setTourNotice(''); setActiveTourSteps(available); setIsWalkthroughOpen(true);
  };

  useEffect(() => {
    if (!ready) return;
    let mounted = true;
    if (typeof window !== 'undefined') {
      const searchParams = new URLSearchParams(window.location.search);
      const panel = searchParams.get('panel') || searchParams.get('section');
      if (panel && sections.some(([s]) => s === panel)) {
        setSection(panel as Section);
      }
    }

    read("/api/v1/walkthrough/assistant")
      .then((data: unknown) => {
        if (!mounted) return;
        if (!Array.isArray(data)) throw new Error('Invalid tour response');
        const steps: Step[] = data.filter((step: unknown): step is Step => {
          if (!step || typeof step !== 'object') return false;
          const item = step as Record<string, unknown>;
          const target = item.targetId || item.target_id;
          return typeof target === 'string' && !!target.trim()
            && typeof item.title === 'string' && !!item.title.trim()
            && typeof item.content === 'string' && !!item.content.trim();
        });
        if (steps.length !== data.length) throw new Error('Invalid tour step');
        setWalkthroughSteps(steps);
        setTourNotice(steps.length ? '' : 'No tour is configured for this page.');
      })
      .catch(() => {
        if (!mounted) return;
        setWalkthroughSteps([]);
        setTourNotice('The configured tour could not be loaded.');
      });

    async function loadSettings() {
      try {
        const data = recordOrEmpty(await read('/api/v1/assistant/settings'));
        const settings = recordOrEmpty(data.settings);
        if (mounted && typeof settings.agentName === 'string') setAgentName(settings.agentName);
      } catch { /* Optional display settings cannot replace verified task state. */ }
    }

    void loadSettings();
    return () => {
      mounted = false;
    };
  }, [ready, revision, read]);

  useEffect(() => {
    const config = resourceConfig[section];
    if (!config || !ready) return;

    let mounted = true;
    async function loadResource() {
      setResourceLoading(section);
      setResourceError('');
      try {
        const data = recordOrEmpty(await read(config.endpoint));
        if (mounted) {
          setResourceData((current) => ({ ...current, [section]: data }));
        }
      } catch (loadError: unknown) {
        if (mounted) setResourceError(errorMessage(loadError, `${config.title} unavailable`));
      } finally {
        if (mounted) setResourceLoading('');
      }
    }

    loadResource();
    return () => {
      mounted = false;
    };
  }, [section, ready, revision, read]);

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
      const matchesStatus = taskStatusFilter === 'all' || (taskStatusFilter === 'archived' ? task.archived : task.status === taskStatusFilter);
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
    if (!prompt.trim() || starting || execution.held || !execution.configured) return;
    setError(''); setActionNotice('');
    await execution.start({prompt,workspace,mode,model,provider:'Auto',workDirectory,outputFormat,constraints,permissionProfile:'Guarded'});
  }
  const refreshTask = useCallback(async (task: AssistantTask) => {
    if (task.legacy) return;
    const sequence = reads.current;
    const taskSequence = (taskReads.current.get(task.id) ?? 0) + 1; taskReads.current.set(task.id, taskSequence);
    try {
      const data = recordOrEmpty(await read(`/api/v1/assistant/tasks/${task.id}`));
      const updated = assistantTask(data.task);
      if (sequence !== reads.current || taskReads.current.get(task.id) !== taskSequence) return;
      if (!updated || updated.id !== task.id) throw new Error('Invalid task receipt');
      setTasks(current => current.map(item => item.id === updated.id ? updated : item)); setError('');
    } catch (cause) { if (sequence === reads.current && taskReads.current.get(task.id) === taskSequence) setError(errorMessage(cause, 'Task refresh unavailable')); }
  }, [read]);
  useEffect(() => {
    if (!activeTask || !['queued','running'].includes(activeTask.status)) return;
    const timer = window.setInterval(() => { void refreshTask(activeTask); }, 2000);
    return () => window.clearInterval(timer);
  }, [activeTask, refreshTask]);

  async function runResultAction(kind: 'share' | 'preview') {
    if (!activeTask?.artifacts?.length) return;
    const artifact = activeTask.artifacts[0];
    const generation = reads.current;
    try {
      await action(kind === 'share' ? '/api/v1/assistant/share' : '/api/v1/assistant/previews', kind === 'share' ? 'POST' : 'PATCH', kind === 'share' ? {taskId:activeTask.id,artifactId:artifact.id,target:'Share Link'} : {action:'open_external',artifactId:artifact.id});
      if (generation === reads.current) setActionNotice(kind === 'share' ? 'Share link created' : 'Preview opened');
    } catch (cause) { if (generation === reads.current) setError(errorMessage(cause,'Action unconfirmed')); }
  }
  async function runResourceAction(targetSection: Section, body: Record<string, unknown>) {
    const config = resourceConfig[targetSection];
    if (!config) return;
    const generation = reads.current;
    setResourceError(''); setActionNotice('');
    try {
      const payload = recordOrEmpty(body.payload || body);
      await action(config.endpoint, body.method === 'POST' ? 'POST' : 'PATCH', payload);
      const data = recordOrEmpty(await read(config.endpoint));
      if (generation !== reads.current) return;
      setResourceData(current => ({...current,[targetSection]:data}));
      setActionNotice(targetSection === 'system' ? 'Settings saved' : 'Change confirmed');
    } catch (cause) { if (generation === reads.current) setResourceError(errorMessage(cause,'Change unconfirmed')); }
  }

  return (
    <AppShell
      title={`${agentName} Assistant`}
      subtitle="Receipt-backed text tasks with saved responses and truthful execution status."
      actions={[{ label: 'Expert Center', href: '/agents' }]}
    >
      <InteractiveWalkthrough
        steps={activeTourSteps}
        isOpen={isWalkthroughOpen}
        onClose={() => setIsWalkthroughOpen(false)}
      />
      <div className="mb-4 flex flex-wrap gap-2 px-6 pt-4">
         <button
           type="button"
           onClick={startTour}
           disabled={!walkthroughSteps.length}
           aria-describedby={tourNotice ? 'assistant-tour-status' : undefined}
           className="app-button min-h-[44px]"
         >
           Start Tour
         </button>
         {tourNotice && <p id="assistant-tour-status" role="status">{tourNotice}</p>}
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
          <p role="status">{execution.notice}</p>
          {error && <p role="alert" className={styles.error}>{error}</p>}
          {execution.held && <div className={styles.resultActions}>
            <button type="button" disabled={starting} onClick={() => execution.recover()} className={styles.smallButton}>Check acceptance</button>
            <button type="button" disabled={starting || !execution.configured} onClick={() => execution.retry()} className={styles.smallButton}>Retry same request</button>
          </div>}
          {section === 'tasks' && <div className={styles.resultActions}>
            <button type="button" disabled={!ready} onClick={() => loadTaskPage()} className={styles.smallButton}>Refresh text tasks</button>
            <button type="button" disabled={!ready} onClick={() => loadTaskPage(true)} className={styles.smallButton}>Legacy history</button>
            {!legacyHistory && nextCursor && <button type="button" onClick={() => loadTaskPage(false, nextCursor)} className={styles.smallButton}>Next task page</button>}
          </div>}
          {legacyHistory && section === 'tasks' && <p>Legacy records have no admitted execution receipt. They remain available for reference.</p>}

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
                <WalkthroughTarget id="omnisolo-help-input-area">
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
                    <input aria-label="Work directory" value={workDirectory} disabled className={styles.input} placeholder="File access is unsupported" />
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
                <p>Text responses only. Coding, files, live research, and long-running delegation are unsupported.</p>
                <button type="button" onClick={startTask} disabled={starting || !prompt.trim() || !ready || !execution.configured || execution.held} className={styles.startButton}>
                  {starting ? 'Starting...' : 'Start Task'}
                </button>
              </div>
            </section>
          )}

          {section === 'conversation' && <ConversationPage task={activeTask} />}
          {section === 'results' && activeTask?.execution && <div className={styles.resultActions}>
            <button type="button" disabled={starting} onClick={() => refreshTask(activeTask)} className={styles.smallButton}>Refresh Task</button>
            {['queued','running'].includes(activeTask.status) && <button type="button" disabled={starting} onClick={() => execution.mutate(activeTask, 'stop')} className={styles.smallButton}>Stop Task</button>}
            {activeTask.status === 'cancelled' && !activeTask.archived && <button type="button" disabled={starting || execution.held || !execution.configured} onClick={() => execution.mutate(activeTask, 'resume')} className={styles.smallButton}>Start new attempt</button>}
            <button type="button" disabled={starting} onClick={() => execution.mutate(activeTask, activeTask.archived ? 'unarchive' : 'archive')} className={styles.smallButton}>{activeTask.archived ? 'Unarchive Task' : 'Archive Task'}</button>
          </div>}
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
          <option value="queued">Queued</option>
          <option value="cancelled">Cancelled</option>
          <option value="outcome_unknown">Outcome unknown</option>
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
        <button type="button" onClick={onReset} disabled={taskSearch === '' && taskStatusFilter === 'all' && taskDateFilter === 'all'} className={styles.inlineButton}>Reset task filters</button>
      </div>
      <div className={styles.taskList}>
        {visibleTasks.length === 0 && <p className={styles.emptyText}>No matching tasks.</p>}
        {visibleTasks.map((task) => (
          <button key={task.id} type="button" onClick={() => onSelect(task.id)} aria-pressed={activeTaskId === task.id} disabled={activeTaskId === task.id} className={cx(styles.taskCard, activeTaskId === task.id && styles.taskCardActive)}>
            <div className={styles.metaRow}>
              <span className={styles.overline}>{task.workspace}</span>
              <span className={cx(styles.statusBadge, statusClass(task.status))}>{task.status}{task.archived ? ' (archived)' : ''}</span>
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
        <span className={cx(styles.statusBadge, statusClass(task.status))}>{task.status}{task.archived ? ' (archived)' : ''}</span>
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
                      await fetch(`/api/v1/assistant/tasks/${task.id}`, { method: 'PATCH', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ action: 'approve_action' }) });
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
      <div className={styles.sectionHeader}>
        <div>
          <h2 className={styles.sectionTitle}>Results</h2>
          <p className={styles.eyebrow}>Results Panel</p>
        </div>
      </div>
      {task && <h3 className={styles.taskTitle}>{task.title}</h3>}
      {task?.execution && <>
        <p>Execution receipt: {task.execution.id}</p>
        <p>Status: {task.status}{task.archived ? ' (archived)' : ''}</p>
        {task.status === 'outcome_unknown' && <p role="alert">The provider outcome is unknown. This attempt cannot be resumed or retried as new work.</p>}
        {task.output && <article aria-label="Text response" className={styles.resultItem} style={{whiteSpace:'pre-wrap'}}>{task.output}</article>}
      </>}
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
  data: ResourceData;
  loading: boolean;
  error: string;
  onAction: (section: Section, body: Record<string, unknown>) => void;
}) {
  const [folder, setFolder] = useState('/workspace/assistant');
  const [agentNameInput, setAgentNameInput] = useState('');
  const [customConnector, setCustomConnector] = useState('');
  const [customSkill, setCustomSkill] = useState('');
  const blocks = resourceBlocks(data, config.rootKeys);

  return (
    <section
      className={styles.panel}
      aria-label={
        section === 'parity'
          ? 'Parity audit panel'
          : section === 'cloud'
          ? 'Cloud runtime panel'
          : undefined
      }
    >
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
              <div className={`${styles.cardTitle} cardTitle`}>Observation Masking</div>
              <p id="assistant-masking-unavailable" className={styles.eyebrow}>This text-only Assistant has no tool observations to mask. Tenant masking controls are unavailable. Built-in agent masking is configured separately.</p>
              <div style={{ marginTop: '1rem', display: 'flex', alignItems: 'center', gap: '1rem' }}>
                <label style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', cursor: 'pointer' }}>
                  <input
                    type="checkbox"
                    checked={false}
                    disabled
                    aria-describedby="assistant-masking-unavailable"
                    aria-label="Observation Masking Toggle"
                  />
                  <span className={styles.eyebrow} style={{ margin: 0 }}>Enable Masking</span>
                </label>
                <button
                  type="button"
                  className={styles.smallButton}
                  disabled
                  aria-describedby="assistant-masking-unavailable"
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
                  <div key={String(item.id || item.name || item.title || `${block.title}-${index}`)} className={styles.featureCard}>
                    <div className={`${styles.cardTitle} cardTitle`}>{recordTitle(item)}</div>
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
  item: Record<string, unknown>;
  onAction: (section: Section, body: Record<string, unknown>) => void;
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

function resourceBlocks(data: ResourceData, rootKeys: string[]): { title: string; items: Record<string, unknown>[] }[] {
  if (!data) return [];
  if (rootKeys.length === 0) return [{ title: 'Details', items: [data] }];
  return rootKeys.map((key) => {
    const value = data[key] === undefined && data.total !== undefined && key === 'summary' ? data : data[key];
    let items: Record<string, unknown>[] = [];
    if (Array.isArray(value)) {
      items = value.map((entry: unknown, index) => {
        const record = recordOrEmpty(entry);
        return isRecord(entry) ? { ...record, id: record.id || record.name || `${key}-${index}` }
          : { id: `${key}-${index}`, value: String(entry) };
      });
    } else if (isRecord(value)) {
      const flatItem: Record<string, unknown> = { id: key, name: key };
      for (const [field, entry] of Object.entries(value)) {
        flatItem[field] = typeof entry === 'number' || typeof entry === 'boolean' ? String(entry) : entry;
      }
      items = [flatItem];
    } else if (value !== undefined) {
      items = [{ id: key, name: key, value: String(value) }];
    }
    return { title: labelFor(key), items };
  });
}

function recordTitle(item: Record<string, unknown>) {
  return String(item?.name || item?.title || item?.filename || item?.provider || item?.id || 'Record');
}

function recordEntries(item: Record<string, unknown>) {
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
