'use client';
import React, { useEffect, useMemo, useState } from 'react';
import Link from 'next/link';
import { AgentWorkflowBuilder } from './components/AgentWorkflowBuilder';
import {
  automations,
  connectors,
  expertTeams,
  experts,
  exploreTemplates,
  memories,
  remoteAssistants,
  skillMarket,
  type ExpertCatalogItem,
} from './catalog';
type Panel =
  | 'browse'
  | 'teams'
  | 'skills'
  | 'connectors'
  | 'automations'
  | 'memory'
  | 'results'
  | 'operations'
  | 'workflows'
  | 'feed'
  | 'approvals'
  | 'explore'
  | 'remote'
  | 'data';
type Mode = 'Ask' | 'Craft' | 'Plan';
type WorkflowRecord = {
  id: string;
  name: string;
  workflow: string;
  task: string;
  status: string;
  command?: string;
  output?: string;
  error?: string;
};
type ApprovalItem = {
  id: string;
  department: string;
  description: string;
  status: string;
};
const departments = [
  { id: 'operations', name: 'The Manager', role: 'Operations', description: 'Inventory, orders, fulfillment, and handoffs.', status: 'Active' },
  { id: 'customer_success', name: 'The Ambassador', role: 'Customer Success', description: 'Customer replies, loyalty, review recovery, and escalations.', status: 'Active' },
  { id: 'marketing', name: 'The Promoter', role: 'Marketing', description: 'Posts, promos, campaigns, and brand voice.', status: 'Active' },
  { id: 'sales', name: 'The Closer', role: 'Sales', description: 'Quotes, follow-ups, pipeline cleanup, and win-back actions.', status: 'Active' },
  { id: 'finance', name: 'The Accountant', role: 'Finance', description: 'Invoices, margins, budgets, and cash-flow checks.', status: 'Guarded' },
  { id: 'legal', name: 'The Counsel', role: 'Legal', description: 'Contracts, compliance, and approval-only risk review.', status: 'Approval only' },
];
const modelOptions = ['MiniMax-M3', 'Auto', 'OpenAI GPT-4.1', 'Claude Sonnet', 'Local Ollama'];
const workspaces = ['Current business', 'Marketing sprint', 'Finance review', 'Customer support'];
const resultTabs = ['Artifacts', 'All files', 'Diffs', 'Preview'];
function slugTestId(id: string) {
  return `expert-card-${id}`;
}
function itemInitials(name: string) {
  return name
    .split(' ')
    .map((part) => part[0])
    .join('')
    .slice(0, 2)
    .toUpperCase();
}
function SectionHeader({ title, detail }: { title: string; detail?: string }) {
  return (
    <div className="mb-4 flex items-end justify-between gap-3">
      <div>
        <h2 className="text-xl font-bold text-zinc-950">{title}</h2>
        {detail && <p className="mt-1 text-sm text-zinc-600">{detail}</p>}
      </div>
    </div>
  );
}
function StatusPill({ children }: { children: React.ReactNode }) {
  return (
    <span className="rounded-md border border-emerald-200 bg-emerald-50 px-2 py-1 text-xs font-semibold text-emerald-700">
      {children}
    </span>
  );
}
export default function AgentsPage() {
  const [panel, setPanel] = useState<Panel>('browse');
  const [selected, setSelected] = useState<ExpertCatalogItem>(experts[0]);
  const [mode, setMode] = useState<Mode>('Ask');
  const [model, setModel] = useState('MiniMax-M3');
  const [workspace, setWorkspace] = useState('Current business');
  const [taskPrompt, setTaskPrompt] = useState('Create a practical operating plan and assign next actions.');
  const [summonMessage, setSummonMessage] = useState('Growth Strategist is ready');
  const [runMessage, setRunMessage] = useState('');
  const [runError, setRunError] = useState('');
  const [running, setRunning] = useState(false);
  const [workflows, setWorkflows] = useState<WorkflowRecord[]>([]);
  const [feed, setFeed] = useState<ApprovalItem[]>([]);
  const [approvals, setApprovals] = useState<ApprovalItem[]>([]);
  const [query, setQuery] = useState('');
  const [enabledSkills, setEnabledSkills] = useState<string[]>(['Web Research', 'Campaign Builder']);
  const [enabledConnectors, setEnabledConnectors] = useState<string[]>(['Tencent Docs', 'Stripe']);
  const [selectedResultTab, setSelectedResultTab] = useState('Artifacts');
  const [hasPro, setHasPro] = useState(false);
  const [showPaywall, setShowPaywall] = useState(false);
  const [contextReferences, setContextReferences] = useState('');
  const [attachments, setAttachments] = useState('');
  const [customProvider, setCustomProvider] = useState('');
  const [workDirectory, setWorkDirectory] = useState('');
  const [outputFormat, setOutputFormat] = useState('Brief');
  const [taskConstraints, setTaskConstraints] = useState('');
  const allCatalog = useMemo(() => [...experts, ...expertTeams], []);
  const visibleExperts = useMemo(() => {
    const source = panel === 'teams' ? expertTeams : experts;
    const normalized = query.trim().toLowerCase();
    if (!normalized) return source;
    return source.filter((item) =>
      [item.name, item.role, item.category, item.summary, ...item.strengths].join(' ').toLowerCase().includes(normalized),
    );
  }, [panel, query]);
  const mostUsed = useMemo(
    () => [...allCatalog].sort((a, b) => b.usageCount - a.usageCount).slice(0, 3),
    [allCatalog],
  );
  useEffect(() => {
    async function fetchAll() {
      try {
        const [approvalsRes, feedRes, workflowsRes] = await Promise.all([
          fetch('/api/agents/approvals'),
          fetch('/api/agents/approvals/activity'),
          fetch('/api/agents/workflows'),
        ]);

        const [approvalsData, feedData, workflowsData] = await Promise.all([
          approvalsRes.ok ? approvalsRes.json() : Promise.resolve({ pending_approvals: [] }),
          feedRes.ok ? feedRes.json() : Promise.resolve({ pending_approvals: [] }),
          workflowsRes.ok ? workflowsRes.json() : Promise.resolve({ workflows: [] })
        ]);

        setApprovals(approvalsData.pending_approvals || []);
        setFeed(feedData.pending_approvals || []);
        setWorkflows(workflowsData.workflows || []);
      } catch (err) {
        console.error('Failed to fetch initial agent data concurrently:', err);
      }
    }
    fetchAll();
  }, []);
  useEffect(() => {
    if (typeof EventSource === 'undefined') return;
    const events = new EventSource('/api/agents/events');
    events.onmessage = (event) => {
      try {
        const item = JSON.parse(event.data);
        if (!item?.id || !item?.description) return;
        setFeed((current) => [item, ...current.filter((existing) => existing.id !== item.id)]);
        if (String(item.status || '').toLowerCase().includes('draft')) {
          setApprovals((current) => [item, ...current.filter((existing) => existing.id !== item.id)]);
        }
      } catch (err) {
        console.error('Failed to parse agent event:', err);
      }
    };
    events.onerror = () => events.close();
    return () => events.close();
  }, []);
  function summon(item: ExpertCatalogItem) {
    setSelected(item);
    setModel(item.model);
    setEnabledSkills(item.skills.slice(0, 3));
    setEnabledConnectors(item.connectors.slice(0, 2));
    setTaskPrompt(item.examples[0]);
    setSummonMessage(`${item.name} is ready`);
  }
  async function startTask() {
    setRunning(true);
    setRunError('');
    setRunMessage('');
    try {
      const res = await fetch('/api/agents/hire', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: selected.name,
          role: selected.role,
          providerType: 'builtin',
          model,
          mode,
          workspace,
          task: taskPrompt,
          skills: enabledSkills,
          connectors: enabledConnectors,
          contextReferences,
          attachments,
          customProvider,
          workDirectory,
          outputFormat,
          taskConstraints,
        }),
      });
      const data = await res.json();
      if (!res.ok) {
        setRunError(data.message || data.error || 'The expert could not be summoned.');
        return;
      }
      const workflowId = data.workflow_id || data.workflowId || data.id;
      setRunMessage(workflowId);
      setWorkflows((current) => [
        {
          id: workflowId,
          name: `${selected.name} task`,
          workflow: selected.kind === 'team' ? 'ohc_business_swarm' : 'expert_task',
          task: taskPrompt,
          status: data.status || 'running',
          command: `${selected.name} via ${model}`,
        },
        ...current.filter((workflow) => workflow.id !== workflowId),
      ]);
      setPanel('results');
    } catch (err) {
      setRunError('Expert service is unavailable.');
    } finally {
      setRunning(false);
    }
  }
  async function decideApproval(id: string, approved: boolean) {
    try {
      const res = await fetch(`/api/agents/approvals/${id}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ approved }),
      });
      if (res.ok) {
        setApprovals((current) => current.filter((item) => item.id !== id));
      }
    } catch (err) {
      console.error('Failed to process approval:', err);
    }
  }
  return (
    <div className="min-h-screen bg-stone-50 text-zinc-950">
      <header className="border-b border-zinc-200 bg-white">
        <div className="mx-auto flex w-full max-w-7xl flex-col gap-5 px-4 py-5 sm:px-6 lg:px-8">
          <div className="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
            <div>
              <Link href="/dashboard" className="text-sm font-semibold text-teal-700">
                Back to Dashboard
              </Link>
              <h1 className="mt-2 text-3xl font-bold tracking-normal text-zinc-950">Expert Center</h1>
              <h2 className="mt-1 text-base font-bold text-zinc-800">AI Departments</h2>
              <p className="mt-1 max-w-3xl text-sm text-zinc-600">
                Hire experts, summon expert teams, attach skills and connectors, schedule recurring work, and inspect generated results from one workspace.
              </p>
            </div>
            <div className="space-y-3">
              <div className="flex items-center justify-end gap-2">
                <span className="text-xs font-semibold uppercase text-zinc-500">Pro Mode</span>
                <button
                  type="button"
                  aria-label="Toggle Pro Mode"
                  aria-pressed={hasPro}
                  onClick={() => {
                    if (!hasPro) {
                      setShowPaywall(true);
                      return;
                    }
                    setHasPro(false);
                  }}
                  className={`h-6 w-10 rounded-full p-1 transition-colors ${hasPro ? 'bg-teal-700' : 'bg-zinc-300'}`}
                >
                  <span className={`block h-4 w-4 rounded-full bg-white transition-transform ${hasPro ? 'translate-x-4' : ''}`} />
                </button>
              </div>
              <div className="grid grid-cols-3 gap-2 text-center">
                <div className="rounded-lg border border-zinc-200 bg-zinc-50 px-3 py-2">
                  <div className="text-lg font-bold">{allCatalog.length}</div>
                  <div className="text-xs text-zinc-500">Experts</div>
                </div>
                <div className="rounded-lg border border-zinc-200 bg-zinc-50 px-3 py-2">
                  <div className="text-lg font-bold">{skillMarket.length}</div>
                  <div className="text-xs text-zinc-500">Skills</div>
                </div>
                <div className="rounded-lg border border-zinc-200 bg-zinc-50 px-3 py-2">
                  <div className="text-lg font-bold">{connectors.length}</div>
                  <div className="text-xs text-zinc-500">Connectors</div>
                </div>
              </div>
            </div>
          </div>
          <nav className="flex gap-2 overflow-x-auto pb-1" aria-label="Agent feature sections">
            {[
              ['browse', 'Browse experts'],
              ['teams', 'Expert Teams'],
              ['skills', 'Skills'],
              ['connectors', 'Connectors'],
              ['automations', 'Automations'],
              ['memory', 'Memory'],
              ['explore', 'Explore'],
              ['results', 'Results'],
              ['feed', 'Activity Feed'],
              ['approvals', 'Needs Approval'],
              ['operations', 'My Team'],
            ].map(([id, label]) => (
              <button
                key={id}
                type="button"
                onClick={() => setPanel(id as Panel)}
                aria-pressed={panel === id}
                className={`whitespace-nowrap rounded-md border px-3 py-2 text-sm font-semibold ${
                  panel === id
                    ? 'border-teal-700 bg-teal-700 text-white'
                    : 'border-zinc-200 bg-white text-zinc-700 hover:border-teal-300'
                }`}
              >
                {label}
                {id === 'approvals' && approvals.length > 0 ? ` ${approvals.length}` : ''}
              </button>
            ))}
          </nav>
          <div className="flex flex-wrap items-center gap-2 rounded-lg border border-zinc-200 bg-zinc-50 px-3 py-2 text-sm text-zinc-700">
            <span className="font-bold text-zinc-950">Operational team:</span>
            <span>The Manager</span>
            <span>The Ambassador</span>
            <span>The Promoter</span>
          </div>
        </div>
      </header>
      {showPaywall && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-zinc-950/50 p-4">
          <div className="w-full max-w-sm rounded-lg bg-white p-5 shadow-2xl">
            <h2 className="text-2xl font-bold text-zinc-950">Upgrade to Pro</h2>
            <p className="mt-2 text-sm text-zinc-600">Unlock advanced model routing, connector automation, and higher agent budgets.</p>
            <Link href="/pricing" className="mt-4 block rounded-md bg-teal-700 px-4 py-3 text-center text-sm font-bold text-white">
              Upgrade to Pro
            </Link>
            <button
              type="button"
              onClick={() => {
                setHasPro(true);
                setShowPaywall(false);
                if (typeof window !== 'undefined') {
                  window.open?.('https://twitter.com/intent/tweet?text=I%20am%20trying%20OHC%20Expert%20Center', '_blank');
                }
              }}
              className="mt-3 w-full rounded-md border border-amber-200 bg-amber-50 px-4 py-3 text-sm font-bold text-amber-900"
            >
              Share on X to get 7 Days Free
            </button>
            <button type="button" onClick={() => setShowPaywall(false)} className="mt-3 w-full text-sm font-semibold text-zinc-500">
              Close
            </button>
          </div>
        </div>
      )}
      <main className="mx-auto grid w-full max-w-7xl gap-5 px-4 py-5 sm:px-6 lg:grid-cols-[minmax(0,1fr)_380px] lg:px-8">
        <section className="space-y-5">
          {(panel === 'browse' || panel === 'teams') && (
            <CatalogPanel
              panel={panel}
              query={query}
              setQuery={setQuery}
              visibleExperts={visibleExperts}
              mostUsed={mostUsed}
              summon={summon}
            />
          )}
          {panel === 'skills' && (
            <SkillsPanel enabledSkills={enabledSkills} setEnabledSkills={setEnabledSkills} />
          )}
          {panel === 'connectors' && (
            <ConnectorsPanel enabledConnectors={enabledConnectors} setEnabledConnectors={setEnabledConnectors} />
          )}
          {panel === 'automations' && <AutomationsPanel />}
          {panel === 'memory' && <MemoryPanel />}
          {panel === 'results' && (
            <ResultsPanel
              selected={selected}
              workflowId={runMessage}
              resultTab={selectedResultTab}
              setResultTab={setSelectedResultTab}
            />
          )}
          {panel === 'explore' && <ExplorePanel summon={summon} />}
          {panel === 'remote' && <RemotePanel />}
          {panel === 'data' && <DataPanel />}
          {panel === 'operations' && <OperationsPanel />}
          {panel === 'workflows' && <WorkflowsPanel workflows={workflows} setWorkflows={setWorkflows} />}
          {panel === 'feed' && <FeedPanel feed={feed} />}
          {panel === 'approvals' && <ApprovalsPanel approvals={approvals} decideApproval={decideApproval} />}
        </section>
        <aside className="space-y-5">
          <ComposerPanel
            selected={selected}
            mode={mode}
            setMode={setMode}
            model={model}
            setModel={setModel}
            workspace={workspace}
            setWorkspace={setWorkspace}
            taskPrompt={taskPrompt}
            setTaskPrompt={setTaskPrompt}
            enabledSkills={enabledSkills}
            enabledConnectors={enabledConnectors}
            summonMessage={summonMessage}
            running={running}
            runError={runError}
            runMessage={runMessage}
            contextReferences={contextReferences}
            setContextReferences={setContextReferences}
            attachments={attachments}
            setAttachments={setAttachments}
            customProvider={customProvider}
            setCustomProvider={setCustomProvider}
            workDirectory={workDirectory}
            setWorkDirectory={setWorkDirectory}
            outputFormat={outputFormat}
            setOutputFormat={setOutputFormat}
            taskConstraints={taskConstraints}
            setTaskConstraints={setTaskConstraints}
            startTask={startTask}
          />
          <ResultsPanel
            selected={selected}
            workflowId={runMessage}
            resultTab={selectedResultTab}
            setResultTab={setSelectedResultTab}
            compact
          />
          <ExtensionShortcuts setPanel={setPanel} />
        </aside>
      </main>
    </div>
  );
}
function CatalogPanel({
  panel,
  query,
  setQuery,
  visibleExperts,
  mostUsed,
  summon,
}: {
  panel: Panel;
  query: string;
  setQuery: (value: string) => void;
  visibleExperts: ExpertCatalogItem[];
  mostUsed: ExpertCatalogItem[];
  summon: (item: ExpertCatalogItem) => void;
}) {
  return (
    <>
      <div className="rounded-lg border border-zinc-200 bg-white p-4">
        <SectionHeader
          title={panel === 'teams' ? 'Expert Teams' : 'Browse experts'}
          detail="Search by job, pick a single expert or a coordinated team, then summon it into the task composer."
        />
        <input
          aria-label="Search experts"
          value={query}
          onChange={(event) => setQuery(event.target.value)}
          className="h-11 w-full rounded-md border border-zinc-300 px-3 text-sm outline-none focus:border-teal-600 focus:ring-2 focus:ring-teal-100"
          placeholder="Search experts, roles, skills, connectors"
        />
      </div>
      <div className="rounded-lg border border-amber-200 bg-amber-50 p-4">
        <h3 className="text-sm font-bold text-amber-950">Most used</h3>
        <div className="mt-3 grid gap-3 md:grid-cols-3">
          {mostUsed.map((item) => (
            <button
              key={item.id}
              type="button"
              onClick={() => summon(item)}
              className="rounded-md border border-amber-200 bg-white px-3 py-2 text-left"
            >
              <div className="text-sm font-bold text-zinc-950">{item.name}</div>
              <div className="text-xs text-zinc-500">{item.usageCount} runs</div>
            </button>
          ))}
        </div>
      </div>
      <div className="grid gap-4 md:grid-cols-2">
        {visibleExperts.map((item) => (
          <ExpertCard key={item.id} item={item} summon={summon} />
        ))}
      </div>
    </>
  );
}
function ExpertCard({ item, summon }: { item: ExpertCatalogItem; summon: (item: ExpertCatalogItem) => void }) {
  const [showDetail, setShowDetail] = useState(false);
  return (
    <article data-testid={slugTestId(item.id)} className="rounded-lg border border-zinc-200 bg-white p-4 shadow-sm">
      <div className="flex items-start gap-3">
        <div className="flex h-11 w-11 shrink-0 items-center justify-center rounded-md bg-teal-50 text-sm font-bold text-teal-800">
          {itemInitials(item.name)}
        </div>
        <div className="min-w-0 flex-1">
          <div className="flex items-center justify-between gap-2">
            <h3 className="text-lg font-bold text-zinc-950">{item.name}</h3>
            <StatusPill>{item.kind === 'team' ? 'Team' : 'Expert'}</StatusPill>
          </div>
          <p className="text-xs font-semibold uppercase text-rose-700">{item.category}</p>
        </div>
      </div>
      <p className="mt-3 text-sm leading-6 text-zinc-650">{item.summary}</p>
      <div className="mt-3 flex flex-wrap gap-2">
        {item.strengths.map((strength) => (
          <span key={strength} className="rounded-md bg-zinc-100 px-2 py-1 text-xs font-medium text-zinc-700">
            {strength}
          </span>
        ))}
      </div>
      {item.members && (
        <div className="mt-4 rounded-md border border-zinc-200 bg-zinc-50 p-3">
          <div className="text-xs font-bold uppercase text-zinc-500">Team members</div>
          <p className="mt-1 text-sm text-zinc-700">{item.members.join(', ')}</p>
        </div>
      )}
      <div className="mt-4">
        <div className="text-xs font-bold uppercase text-zinc-500">Use cases</div>
        <ul className="mt-2 space-y-1 text-sm text-zinc-700">
          {item.examples.map((example) => (
            <li key={example}>{example}</li>
          ))}
        </ul>
      </div>
      {showDetail && (
        <div className="mt-4 rounded-md border border-teal-200 bg-teal-50 p-3">
          <div className="text-xs font-bold uppercase text-teal-900">Expert detail</div>
          <p className="mt-1 text-sm text-teal-900">{item.name} uses {item.model} with {item.skills.join(', ')}.</p>
          <div className="mt-3 flex flex-wrap gap-2">
            <button type="button" onClick={() => summon(item)} className="rounded-md bg-teal-700 px-3 py-2 text-xs font-bold text-white">
              Summon into chat
            </button>
            <button type="button" className="rounded-md border border-teal-200 bg-white px-3 py-2 text-xs font-bold text-teal-800">
              Favorite
            </button>
          </div>
        </div>
      )}
      <div className="mt-4 grid grid-cols-2 gap-2">
        <button
          type="button"
          onClick={() => setShowDetail((current) => !current)}
          className="h-10 rounded-md border border-zinc-200 px-4 text-sm font-bold text-zinc-700"
        >
          Details
        </button>
        <button
          type="button"
          onClick={() => summon(item)}
          className="h-10 rounded-md bg-teal-700 px-4 text-sm font-bold text-white hover:bg-teal-800"
        >
          Summon
        </button>
      </div>
    </article>
  );
}
function ComposerPanel({
  selected,
  mode,
  setMode,
  model,
  setModel,
  workspace,
  setWorkspace,
  taskPrompt,
  setTaskPrompt,
  enabledSkills,
  enabledConnectors,
  summonMessage,
  running,
  runError,
  runMessage,
  contextReferences,
  setContextReferences,
  attachments,
  setAttachments,
  customProvider,
  setCustomProvider,
  workDirectory,
  setWorkDirectory,
  outputFormat,
  setOutputFormat,
  taskConstraints,
  setTaskConstraints,
  startTask,
}: {
  selected: ExpertCatalogItem;
  mode: Mode;
  setMode: (mode: Mode) => void;
  model: string;
  setModel: (model: string) => void;
  workspace: string;
  setWorkspace: (workspace: string) => void;
  taskPrompt: string;
  setTaskPrompt: (taskPrompt: string) => void;
  enabledSkills: string[];
  enabledConnectors: string[];
  summonMessage: string;
  running: boolean;
  runError: string;
  runMessage: string;
  contextReferences: string;
  setContextReferences: (value: string) => void;
  attachments: string;
  setAttachments: (value: string) => void;
  customProvider: string;
  setCustomProvider: (value: string) => void;
  workDirectory: string;
  setWorkDirectory: (value: string) => void;
  outputFormat: string;
  setOutputFormat: (value: string) => void;
  taskConstraints: string;
  setTaskConstraints: (value: string) => void;
  startTask: () => void;
}) {
  return (
    <section className="rounded-lg border border-zinc-200 bg-white p-4 shadow-sm">
      <div className="flex items-start justify-between gap-3">
        <div>
          <h2 className="text-lg font-bold text-zinc-950">Task Composer</h2>
          <p className="text-sm font-semibold text-teal-700">{summonMessage}</p>
        </div>
        <StatusPill>{selected.model}</StatusPill>
      </div>
      <div className="mt-4 grid grid-cols-3 gap-2">
        {(['Ask', 'Craft', 'Plan'] as Mode[]).map((item) => (
          <button
            key={item}
            type="button"
            onClick={() => setMode(item)}
            aria-pressed={mode === item}
            className={`h-9 rounded-md border text-sm font-bold ${
              mode === item ? 'border-rose-700 bg-rose-700 text-white' : 'border-zinc-200 text-zinc-700'
            }`}
          >
            {item}
          </button>
        ))}
      </div>
      <div className="mt-4 grid gap-3">
        <label className="text-sm font-bold text-zinc-700">
          Model
          <select
            value={model}
            onChange={(event) => setModel(event.target.value)}
            className="mt-1 h-10 w-full rounded-md border border-zinc-300 bg-white px-3 text-sm"
          >
            {modelOptions.map((option) => (
              <option key={option}>{option}</option>
            ))}
          </select>
        </label>
        <div className="grid grid-cols-2 gap-2">
          <label className="text-sm font-bold text-zinc-700">
            Custom provider
            <input
              aria-label="Custom provider"
              value={customProvider}
              onChange={(event) => setCustomProvider(event.target.value)}
              className="mt-1 h-10 w-full rounded-md border border-zinc-300 px-3 text-sm"
              placeholder="OpenAI-compatible endpoint"
            />
          </label>
          <label className="text-sm font-bold text-zinc-700">
            Work directory
            <input
              aria-label="Work directory"
              value={workDirectory}
              onChange={(event) => setWorkDirectory(event.target.value)}
              className="mt-1 h-10 w-full rounded-md border border-zinc-300 px-3 text-sm"
              placeholder="/workspace/current-task"
            />
          </label>
        </div>
        <div className="flex flex-wrap gap-2">
          {['Local Ollama', 'Vision', 'Tool use', 'Long context', 'Parallel tasks'].map((capability) => (
            <span key={capability} className="rounded-md border border-zinc-200 bg-white px-2 py-1 text-xs font-bold text-zinc-700">
              {capability}
            </span>
          ))}
        </div>
        <label className="text-sm font-bold text-zinc-700">
          Workspace
          <select
            value={workspace}
            onChange={(event) => setWorkspace(event.target.value)}
            className="mt-1 h-10 w-full rounded-md border border-zinc-300 bg-white px-3 text-sm"
          >
            {workspaces.map((option) => (
              <option key={option}>{option}</option>
            ))}
          </select>
        </label>
        <label htmlFor="task-prompt" className="text-sm font-bold text-zinc-700">
          Task prompt
        </label>
        <textarea
          id="task-prompt"
          aria-label="Task prompt"
          value={taskPrompt}
          onChange={(event) => setTaskPrompt(event.target.value)}
          className="min-h-[120px] resize-none rounded-md border border-zinc-300 p-3 text-sm leading-6 outline-none focus:border-teal-600 focus:ring-2 focus:ring-teal-100"
        />
        <label className="text-sm font-bold text-zinc-700">
          Context references
          <input
            aria-label="Context references"
            value={contextReferences}
            onChange={(event) => setContextReferences(event.target.value)}
            className="mt-1 h-10 w-full rounded-md border border-zinc-300 px-3 text-sm"
            placeholder="@orders @inventory @customer-notes"
          />
        </label>
        <label className="text-sm font-bold text-zinc-700">
          Attachments
          <input
            aria-label="Attachments"
            value={attachments}
            onChange={(event) => setAttachments(event.target.value)}
            className="mt-1 h-10 w-full rounded-md border border-zinc-300 px-3 text-sm"
            placeholder="Drop files, screenshots, PDFs, CSVs"
          />
        </label>
        <div className="grid grid-cols-3 gap-2">
          <button type="button" className="rounded-md border border-zinc-200 bg-zinc-50 px-3 py-2 text-sm font-bold text-zinc-700">Screenshot</button>
          <label className="text-sm font-bold text-zinc-700">
            Output format
            <select
              aria-label="Output format"
              value={outputFormat}
              onChange={(event) => setOutputFormat(event.target.value)}
              className="mt-1 h-10 w-full rounded-md border border-zinc-300 bg-white px-2 text-sm"
            >
              <option>Brief</option>
              <option>Table</option>
              <option>Document</option>
              <option>Spreadsheet</option>
            </select>
          </label>
          <label className="text-sm font-bold text-zinc-700">
            Task constraints
            <input
              aria-label="Task constraints"
              value={taskConstraints}
              onChange={(event) => setTaskConstraints(event.target.value)}
              className="mt-1 h-10 w-full rounded-md border border-zinc-300 px-2 text-sm"
              placeholder="budget, tone, deadline"
            />
          </label>
        </div>
      </div>
      <div className="mt-4 grid gap-3 rounded-md border border-zinc-200 bg-zinc-50 p-3">
        <div>
          <div className="text-xs font-bold uppercase text-zinc-500">Skills</div>
          <p className="mt-1 text-sm text-zinc-700">{enabledSkills.join(', ') || 'None selected'}</p>
        </div>
        <div>
          <div className="text-xs font-bold uppercase text-zinc-500">Connectors</div>
          <p className="mt-1 text-sm text-zinc-700">{enabledConnectors.join(', ') || 'None selected'}</p>
        </div>
        <p className="text-xs text-zinc-500">
          Cost warning: Craft and Plan can call tools and may consume more agent actions. High-risk actions still route to approval.
        </p>
      </div>
      {runError && <p className="mt-3 text-sm font-semibold text-red-700">{runError}</p>}
      {runMessage && <p className="mt-3 text-sm font-semibold text-emerald-700">{runMessage}</p>}
      <button
        type="button"
        onClick={startTask}
        disabled={running || !taskPrompt.trim()}
        className="mt-4 h-11 w-full rounded-md bg-zinc-950 px-4 text-sm font-bold text-white hover:bg-zinc-800 disabled:cursor-not-allowed disabled:bg-zinc-400"
      >
        {running ? 'Starting...' : 'Start task'}
      </button>
    </section>
  );
}
function ResultsPanel({
  selected,
  workflowId,
  resultTab,
  setResultTab,
  compact,
}: {
  selected: ExpertCatalogItem;
  workflowId: string;
  resultTab: string;
  setResultTab: (tab: string) => void;
  compact?: boolean;
}) {
  return (
    <section className="rounded-lg border border-zinc-200 bg-white p-4 shadow-sm">
      <SectionHeader
        title="Results"
        detail={compact ? undefined : 'Inspect generated artifacts, file lists, diffs, and previews before sharing or approving.'}
      />
      <div className="grid grid-cols-4 gap-2">
        {resultTabs.map((tab) => (
          <button
            key={tab}
            type="button"
            onClick={() => setResultTab(tab)}
            aria-pressed={resultTab === tab}
            className={`h-9 rounded-md border text-xs font-bold ${
              resultTab === tab ? 'border-teal-700 bg-teal-700 text-white' : 'border-zinc-200 text-zinc-700'
            }`}
          >
            {tab}
          </button>
        ))}
      </div>
      <div className="mt-4 rounded-md border border-zinc-200 bg-zinc-50 p-3">
        <div className="text-xs font-bold uppercase text-zinc-500">{resultTab}</div>
        <p className="mt-2 text-sm leading-6 text-zinc-700">
          {workflowId
            ? `${selected.name} is producing ${resultTab.toLowerCase()} for workflow ${workflowId}.`
            : `${selected.name} output will appear here after a task starts.`}
        </p>
      </div>
      <div className="mt-4 flex flex-wrap gap-2">
        {['Share result', 'Download file', 'Copy to workspace', 'Archive task', 'Unarchive'].map((action) => (
          <button key={action} type="button" className="rounded-md border border-zinc-200 bg-zinc-50 px-3 py-2 text-xs font-bold text-zinc-700">
            {action}
          </button>
        ))}
      </div>
    </section>
  );
}
function ExtensionShortcuts({ setPanel }: { setPanel: (panel: Panel) => void }) {
  return (
    <section className="rounded-lg border border-zinc-200 bg-white p-4 shadow-sm">
      <h2 className="text-lg font-bold text-zinc-950">Extensions</h2>
      <div className="mt-3 grid grid-cols-2 gap-2">
        {[
          ['remote', 'Remote control'],
          ['data', 'Data management'],
          ['workflows', 'Workflows'],
          ['explore', 'Templates'],
        ].map(([panel, label]) => (
          <button
            key={panel}
            type="button"
            onClick={() => setPanel(panel as Panel)}
            className="h-10 rounded-md border border-zinc-200 bg-zinc-50 text-sm font-semibold text-zinc-700"
          >
            {label}
          </button>
        ))}
      </div>
    </section>
  );
}
function SkillsPanel({
  enabledSkills,
  setEnabledSkills,
}: {
  enabledSkills: string[];
  setEnabledSkills: React.Dispatch<React.SetStateAction<string[]>>;
}) {
  return (
    <section className="rounded-lg border border-zinc-200 bg-white p-4">
      <SectionHeader title="Skill Market" detail="Install, disable, search, upload, or create natural-language skills for experts." />
      <div className="mb-4 flex flex-wrap gap-2">
        <button className="rounded-md bg-teal-700 px-3 py-2 text-sm font-bold text-white" type="button">Find skill</button>
        <button className="rounded-md border border-zinc-200 px-3 py-2 text-sm font-bold text-zinc-700" type="button">Search installed skills</button>
        <button className="rounded-md border border-zinc-200 px-3 py-2 text-sm font-bold text-zinc-700" type="button">Upload local skill</button>
        <button className="rounded-md border border-zinc-200 px-3 py-2 text-sm font-bold text-zinc-700" type="button">Create skill from prompt</button>
        <button className="rounded-md border border-zinc-200 px-3 py-2 text-sm font-bold text-zinc-700" type="button">Disable skill</button>
        <button className="rounded-md border border-zinc-200 px-3 py-2 text-sm font-bold text-zinc-700" type="button">Uninstall skill</button>
        <button className="rounded-md border border-zinc-200 px-3 py-2 text-sm font-bold text-zinc-700" type="button">Bulk uninstall</button>
      </div>
      <div className="grid gap-3 md:grid-cols-2">
        {skillMarket.map((skill) => {
          const enabled = enabledSkills.includes(skill.name);
          return (
            <button
              key={skill.id}
              type="button"
              aria-pressed={enabled}
              onClick={() =>
                setEnabledSkills((current) =>
                  enabled ? current.filter((name) => name !== skill.name) : [...current, skill.name],
                )
              }
              className="rounded-lg border border-zinc-200 bg-zinc-50 p-4 text-left"
            >
              <div className="flex items-center justify-between gap-3">
                <h3 className="font-bold text-zinc-950">{skill.name}</h3>
                <span className="rounded-md bg-white px-2 py-1 text-xs font-semibold text-zinc-600">{enabled ? 'Enabled' : skill.status}</span>
              </div>
              <p className="mt-2 text-sm text-zinc-600">{skill.description}</p>
            </button>
          );
        })}
      </div>
    </section>
  );
}
function ConnectorsPanel({
  enabledConnectors,
  setEnabledConnectors,
}: {
  enabledConnectors: string[];
  setEnabledConnectors: React.Dispatch<React.SetStateAction<string[]>>;
}) {
  return (
    <section className="rounded-lg border border-zinc-200 bg-white p-4">
      <SectionHeader title="Connector Center" detail="Connect external data and service actions for expert tasks." />
      <div className="mb-4 grid gap-3 md:grid-cols-3">
        {['Create custom connector', 'MCP endpoint', 'Notification channel'].map((label) => (
          <label key={label} className="text-sm font-bold text-zinc-700">
            {label}
            <input className="mt-1 h-10 w-full rounded-md border border-zinc-300 px-3 text-sm" placeholder={label} />
          </label>
        ))}
      </div>
      <div className="grid gap-3 md:grid-cols-2">
        {connectors.map((connector) => {
          const enabled = enabledConnectors.includes(connector.name);
          return (
            <button
              key={connector.id}
              type="button"
              aria-pressed={enabled}
              onClick={() =>
                setEnabledConnectors((current) =>
                  enabled ? current.filter((name) => name !== connector.name) : [...current, connector.name],
                )
              }
              className="rounded-lg border border-zinc-200 bg-zinc-50 p-4 text-left"
            >
              <div className="flex items-center justify-between gap-3">
                <h3 className="font-bold text-zinc-950">{connector.name}</h3>
                <span className="rounded-md bg-white px-2 py-1 text-xs font-semibold text-zinc-600">
                  {enabled ? 'Selected' : connector.status}
                </span>
              </div>
              <p className="mt-2 text-sm text-zinc-600">{connector.description}</p>
            </button>
          );
        })}
      </div>
    </section>
  );
}
function AutomationsPanel() {
  return (
    <section className="rounded-lg border border-zinc-200 bg-white p-4">
      <SectionHeader title="Scheduled Tasks" detail="Configure recurring expert runs with prompt, workspace, model, skills, connectors, and notifications." />
      <div className="mb-4 grid gap-3 md:grid-cols-3">
        <label className="text-sm font-bold text-zinc-700">
          Schedule rule
          <input className="mt-1 h-10 w-full rounded-md border border-zinc-300 px-3 text-sm" placeholder="Every Monday 9:00" />
        </label>
        <div className="rounded-md border border-zinc-200 bg-zinc-50 p-3 text-sm font-bold text-zinc-700">Execution history</div>
        <div className="rounded-md border border-zinc-200 bg-zinc-50 p-3 text-sm font-bold text-zinc-700">Push notification</div>
      </div>
      <div className="grid gap-3 md:grid-cols-2">
        {automations.map((automation, index) => (
          <div key={automation} className="rounded-lg border border-zinc-200 bg-zinc-50 p-4">
            <div className="flex items-center justify-between">
              <h3 className="font-bold text-zinc-950">{automation}</h3>
              <StatusPill>{index === 0 ? 'Active' : 'Template'}</StatusPill>
            </div>
            <p className="mt-2 text-sm text-zinc-600">Runs in the selected workspace with push notifications and execution history.</p>
          </div>
        ))}
      </div>
      <div className="mt-4 rounded-md border border-teal-200 bg-teal-50 p-3">
        <div className="font-bold text-teal-950">Approve & Post</div>
        <p className="mt-1 text-sm text-teal-800">Social drafts generated by automations still wait for one-tap approval before publishing.</p>
      </div>
    </section>
  );
}
function MemoryPanel() {
  const [items, setItems] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);

  const fetchMemories = async () => {
    setLoading(true);
    try {
      const res = await fetch('/api/v1/memory');
      const data = await res.json();
      setItems(Array.isArray(data) ? data : []);
    } catch (e) {
      console.error(e);
    }
    setLoading(false);
  };

  useEffect(() => {
    fetchMemories();
  }, []);

  const toggleOverride = async (id: string, currentValue: boolean) => {
    try {
      await fetch(`/api/v1/memory/${id}/override`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ override_value: !currentValue }),
      });
      fetchMemories();
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <section className="rounded-lg border border-zinc-200 bg-white p-4">
      <SectionHeader title="Consolidated Memory" detail="Review and explicitly override what AI agents remember about your business." />

      {loading ? (
        <p className="text-sm text-zinc-500">Loading memories...</p>
      ) : items.length === 0 ? (
        <p className="text-sm text-zinc-500">No consolidated memories found.</p>
      ) : (
        <div className="space-y-3">
          {items.map((memory: any) => (
            <div key={memory.id} className="rounded-md border border-zinc-200 bg-zinc-50 p-3 text-sm text-zinc-700 flex flex-col gap-2">
              <div className="flex justify-between items-start">
                <div className="font-semibold text-zinc-900">{memory.source_type}</div>
                <button
                  type="button"
                  onClick={() => toggleOverride(memory.id, memory.owner_override)}
                  className={`text-xs px-2 py-1 rounded-md font-bold ${memory.owner_override ? 'bg-teal-700 text-white' : 'bg-zinc-200 text-zinc-700 hover:bg-zinc-300'}`}
                >
                  {memory.owner_override ? 'Owner Override: ON' : 'Owner Override: OFF'}
                </button>
              </div>
              <p className="whitespace-pre-wrap">{memory.content}</p>
              <div className="text-xs text-zinc-500 flex gap-4">
                <span>References: {memory.reference_count}</span>
                <span>Reliability: {memory.reliability_score}</span>
              </div>
            </div>
          ))}
        </div>
      )}
    </section>
  );
}
function ExplorePanel({ summon }: { summon: (item: ExpertCatalogItem) => void }) {
  return (
    <section className="rounded-lg border border-zinc-200 bg-white p-4">
      <SectionHeader title="Explore Templates" detail="Curated examples prefill prompt, expert, skills, and connector choices." />
      <div className="grid gap-3 md:grid-cols-2">
        {exploreTemplates.map((template, index) => (
          <button
            key={template}
            type="button"
            onClick={() => summon(index % 2 === 0 ? expertTeams[0] : experts[0])}
            className="rounded-lg border border-zinc-200 bg-zinc-50 p-4 text-left"
          >
            <h3 className="font-bold text-zinc-950">{template}</h3>
            <p className="mt-2 text-sm text-zinc-600">Make my version with one click.</p>
          </button>
        ))}
      </div>
    </section>
  );
}
function RemotePanel() {
  return (
    <section className="rounded-lg border border-zinc-200 bg-white p-4">
      <SectionHeader title="Remote Assistant Control" detail="Summon experts and receive notifications from business chat tools." />
      <div className="mb-4 rounded-md border border-zinc-200 bg-zinc-50 p-3 text-sm font-bold text-zinc-800">
        /summon Growth Strategist
      </div>
      <div className="grid gap-3 md:grid-cols-3">
        {remoteAssistants.map((name) => (
          <div key={name} className="rounded-lg border border-zinc-200 bg-zinc-50 p-4">
            <div className="font-bold text-zinc-950">{name}</div>
            <div className="mt-1 text-sm text-zinc-600">Available</div>
          </div>
        ))}
      </div>
    </section>
  );
}
function DataPanel() {
  return (
    <section className="rounded-lg border border-zinc-200 bg-white p-4">
      <SectionHeader title="Data Management" detail="Manage shared files, task archives, generated outputs, and workspace history." />
      <div className="grid gap-3 md:grid-cols-3">
        {['Shared files', 'Archived tasks', 'Generated outputs', 'Workspace history', 'Download center', 'Unshare queue'].map((item) => (
          <div key={item} className="rounded-lg border border-zinc-200 bg-zinc-50 p-4 font-bold text-zinc-950">
            {item}
          </div>
        ))}
      </div>
    </section>
  );
}
function OperationsPanel() {
  return (
    <section className="rounded-lg border border-zinc-200 bg-white p-4">
      <SectionHeader title="AI Departments" detail="Operational agents stay visible for backwards-compatible business management." />
      <div className="grid gap-3 md:grid-cols-2">
        {departments.map((department) => (
          <div key={department.id} className="rounded-lg border border-zinc-200 bg-zinc-50 p-4">
            <div className="flex items-center justify-between gap-3">
              <h3 className="font-bold text-zinc-950">{department.name}</h3>
              <StatusPill>{department.status}</StatusPill>
            </div>
            <p className="mt-1 text-xs font-bold uppercase text-rose-700">{department.role}</p>
            <p className="mt-2 text-sm text-zinc-600">{department.description}</p>
          </div>
        ))}
      </div>
    </section>
  );
}
function WorkflowsPanel({ workflows, setWorkflows }: { workflows: WorkflowRecord[], setWorkflows: React.Dispatch<React.SetStateAction<WorkflowRecord[]>> }) {
  const handleSaveWorkflow = async (name: string, task: string) => {
    const res = await fetch('/api/agents/workflows', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ name, task }),
    });
    if (!res.ok) {
      throw new Error('Failed to create workflow');
    }
    const data = await res.json();
    setWorkflows(current => [data.workflow, ...current]);
  };

  return (
    <section className="rounded-[16px] border border-[rgba(255,255,255,0.4)] bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] saturate-[210%] p-4">
      <SectionHeader title="Workflows" detail="Active expert and expert-team runs." />

      <div className="mb-8">
        <AgentWorkflowBuilder onSave={handleSaveWorkflow} />
      </div>
      {workflows.length === 0 ? (
        <p className="rounded-[8px] border border-dashed border-zinc-300 p-4 text-sm text-zinc-600">No workflows yet.</p>
      ) : (
        <div className="space-y-3">
          {workflows.map((workflow) => (
            <div key={workflow.id} className="rounded-[16px] border border-[rgba(255,255,255,0.4)] bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] saturate-[210%] p-4">
              <div className="flex items-center justify-between">
                <h3 className="font-bold text-zinc-950">{workflow.name}</h3>
                <StatusPill>{workflow.status}</StatusPill>
              </div>
              <p className="mt-1 text-xs font-bold uppercase text-zinc-500">{workflow.workflow}</p>
              <p className="mt-2 text-sm text-zinc-700">{workflow.task}</p>
              {workflow.command && <p className="mt-2 break-words text-xs text-zinc-500">{workflow.command}</p>}
            </div>
          ))}
        </div>
      )}
    </section>
  );
}
function FeedPanel({ feed }: { feed: ApprovalItem[] }) {
  return (
    <section className="rounded-lg border border-zinc-200 bg-white p-4">
      <SectionHeader title="Activity Feed" detail="Realtime expert and department activity." />
      {feed.length === 0 ? (
        <p className="rounded-md border border-dashed border-zinc-300 p-4 text-sm text-zinc-600">No activity yet.</p>
      ) : (
        <div className="space-y-3">
          {feed.map((item) => (
            <div key={item.id} className="rounded-lg border border-zinc-200 bg-zinc-50 p-4">
              <div className="text-xs font-bold uppercase text-teal-700">{item.department}</div>
              <p className="mt-2 text-sm text-zinc-700">{item.description}</p>
            </div>
          ))}
        </div>
      )}
    </section>
  );
}
function ApprovalsPanel({
  approvals,
  decideApproval,
}: {
  approvals: ApprovalItem[];
  decideApproval: (id: string, approved: boolean) => void;
}) {
  return (
    <section className="rounded-lg border border-zinc-200 bg-white p-4">
      <SectionHeader title="Needs Approval" detail="Review high-risk drafts before experts execute or send." />
      {approvals.length === 0 ? (
        <div className="rounded-md border border-dashed border-zinc-300 p-4">
          <h3 className="font-bold text-zinc-950">All Caught Up!</h3>
          <p className="mt-1 text-sm text-zinc-600">Your AI team has no pending approvals.</p>
        </div>
      ) : (
        <div className="space-y-3">
          {approvals.map((item) => (
            <div key={item.id} className="rounded-lg border border-zinc-200 bg-zinc-50 p-4">
              <div className="text-xs font-bold uppercase text-rose-700">{item.department}</div>
              <p className="mt-2 text-sm text-zinc-700">{item.description}</p>
              <div className="mt-4 flex gap-2">
                <button
                  type="button"
                  onClick={() => decideApproval(item.id, false)}
                  className="h-10 flex-1 rounded-md border border-zinc-300 bg-white text-sm font-bold text-zinc-700"
                >
                  Edit Draft
                </button>
                <button
                  type="button"
                  onClick={() => decideApproval(item.id, true)}
                  className="h-10 flex-1 rounded-md bg-teal-700 text-sm font-bold text-white"
                >
                  Approve & Send
                </button>
              </div>
            </div>
          ))}
        </div>
      )}
    </section>
  );
}
