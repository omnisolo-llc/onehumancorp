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
    <div className="min-h-screen bg-stone-50 dark:bg-zinc-950 text-zinc-950 dark:text-zinc-50 transition-colors duration-200">
      <header className="border-b border-zinc-200 dark:border-zinc-850 bg-white/70 dark:bg-zinc-900/70 backdrop-blur-[30px] saturate-[210%] sticky top-0 z-30">
        <div className="mx-auto flex w-full max-w-7xl flex-col gap-5 px-4 py-5 sm:px-6 lg:px-8">
          <div className="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
            <div>
              <Link href="/dashboard" className="text-sm font-bold text-teal-600 dark:text-teal-400 hover:underline">
                ← Back to Dashboard
              </Link>
              <h1 className="mt-2 text-3xl font-extrabold tracking-tight text-zinc-900 dark:text-white">AI Departments</h1>
              <h2 className="mt-1 text-sm font-bold text-zinc-500 dark:text-zinc-400">Expert Center</h2>
              <p className="mt-1 max-w-3xl text-sm text-zinc-600 dark:text-zinc-450">
                Hire experts, summon expert teams, attach skills and connectors, schedule recurring work, and inspect generated results from one workspace.
              </p>
            </div>
            <div className="space-y-3">
              <div className="flex items-center justify-end gap-2">
                <span className="text-xs font-semibold uppercase text-zinc-500 dark:text-zinc-400">Pro Mode</span>
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
                  className={`h-6 w-10 rounded-full p-1 transition-colors outline-none ${hasPro ? 'bg-teal-600' : 'bg-zinc-300 dark:bg-zinc-700'}`}
                >
                  <span className={`block h-4 w-4 rounded-full bg-white transition-transform ${hasPro ? 'translate-x-4' : ''}`} />
                </button>
              </div>
              <div className="grid grid-cols-3 gap-2 text-center">
                <div className="rounded-2xl border border-zinc-200/80 dark:border-zinc-800/80 bg-white/60 dark:bg-zinc-900/60 backdrop-blur px-3 py-2">
                  <div className="text-lg font-bold text-zinc-900 dark:text-white">{allCatalog.length}</div>
                  <div className="text-xs text-zinc-500 dark:text-zinc-400">Experts</div>
                </div>
                <div className="rounded-2xl border border-zinc-200/80 dark:border-zinc-800/80 bg-white/60 dark:bg-zinc-900/60 backdrop-blur px-3 py-2">
                  <div className="text-lg font-bold text-zinc-900 dark:text-white">{skillMarket.length}</div>
                  <div className="text-xs text-zinc-500 dark:text-zinc-400">Skills</div>
                </div>
                <div className="rounded-2xl border border-zinc-200/80 dark:border-zinc-800/80 bg-white/60 dark:bg-zinc-900/60 backdrop-blur px-3 py-2">
                  <div className="text-lg font-bold text-zinc-900 dark:text-white">{connectors.length}</div>
                  <div className="text-xs text-zinc-500 dark:text-zinc-400">Connectors</div>
                </div>
              </div>
            </div>
          </div>
          <nav className="flex gap-2 overflow-x-auto pb-1 custom-scrollbar" aria-label="Agent feature sections">
            {[
              ['browse', 'Browse experts'],
              ['teams', 'Expert Teams'],
              ['skills', 'Skills'],
              ['connectors', 'Connectors'],
              ['automations', 'Automations'],
              ['memory', 'Memory'],
              ['explore', 'Templates'],
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
                className={`whitespace-nowrap rounded-full px-4 py-2 text-sm font-semibold transition-all border outline-none ${
                  panel === id
                    ? 'border-teal-600 bg-teal-600 text-white shadow-sm'
                    : 'border-zinc-200 dark:border-zinc-800 bg-white/80 dark:bg-zinc-900/80 text-zinc-700 dark:text-zinc-300 hover:border-teal-300 dark:hover:border-teal-700'
                }`}
              >
                {label}
                {id === 'approvals' && approvals.length > 0 ? ` (${approvals.length})` : ''}
              </button>
            ))}
          </nav>
          <div className="flex flex-wrap items-center gap-2 rounded-2xl border border-zinc-250/70 dark:border-zinc-800/70 bg-zinc-50/50 dark:bg-zinc-900/50 px-4 py-2 text-sm text-zinc-700 dark:text-zinc-300">
            <span className="font-bold text-zinc-900 dark:text-white">Operational team:</span>
            <span className="bg-zinc-200/80 dark:bg-zinc-800/80 px-2 py-0.5 rounded-md text-xs font-semibold">The Manager</span>
            <span className="bg-zinc-200/80 dark:bg-zinc-800/80 px-2 py-0.5 rounded-md text-xs font-semibold">The Ambassador</span>
            <span className="bg-zinc-200/80 dark:bg-zinc-800/80 px-2 py-0.5 rounded-md text-xs font-semibold">The Promoter</span>
          </div>
        </div>
      </header>
      {showPaywall && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-zinc-950/60 backdrop-blur-[30px] saturate-[210%] p-4">
          <div className="w-full max-w-sm rounded-2xl bg-white dark:bg-zinc-900 border border-zinc-200 dark:border-zinc-800 p-6 shadow-2xl">
            <h2 className="text-2xl font-bold text-zinc-950 dark:text-white">Upgrade to Pro</h2>
            <p className="mt-2 text-sm text-zinc-650 dark:text-zinc-400">Unlock advanced model routing, connector automation, and higher agent budgets.</p>
            <Link href="/pricing" className="mt-4 block rounded-xl bg-teal-600 hover:bg-teal-700 px-4 py-3 text-center text-sm font-bold text-white transition-colors">
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
              className="mt-3 w-full rounded-xl border border-amber-250 bg-amber-50/50 dark:bg-amber-900/25 px-4 py-3 text-sm font-bold text-amber-900 dark:text-amber-200 hover:bg-amber-100/50 dark:hover:bg-amber-900/40 transition-colors"
            >
              Share on X to get 7 Days Free
            </button>
            <button type="button" onClick={() => setShowPaywall(false)} className="mt-3 w-full text-sm font-semibold text-zinc-500 hover:text-zinc-700 dark:text-zinc-400 dark:hover:text-zinc-300">
              Close
            </button>
          </div>
        </div>
      )}
      <main className="mx-auto grid w-full max-w-7xl gap-6 px-4 py-6 sm:px-6 lg:grid-cols-[minmax(0,1fr)_380px] lg:px-8">
        <section className="space-y-6">
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
  const featuredScenarios = [
    {
      title: "Content Creation",
      desc: "Specialists in posts, copy matching, and social outreach.",
      tag: "Marketing",
      team: "Product Swarm",
      members: ["Growth Strategist", "Customer Ambassador"]
    },
    {
      title: "Investment Analysis",
      desc: "Financial margins, Scenario audits, and growth margins.",
      tag: "Finance",
      team: "Revenue Swarm",
      members: ["Finance Controller", "Revenue Strategist"]
    },
    {
      title: "Legal & Compliance",
      desc: "Legal review, safety checks, and regulatory policy gates.",
      tag: "Legal",
      team: "Corporate Counsel",
      members: ["Operations Manager", "Policy Checker"]
    },
    {
      title: "Operations & Supply",
      desc: "Order dispatch, stock recovery, and team workspace handoffs.",
      tag: "Ops",
      team: "Operations Swarm",
      members: ["Operations Manager", "Customer Ambassador"]
    }
  ];

  return (
    <>
      {/* Featured Scenarios Carousel */}
      <div className="rounded-2xl border border-zinc-200/80 dark:border-zinc-800/80 bg-white/60 dark:bg-zinc-900/60 backdrop-blur-[30px] saturate-[210%] p-6 shadow-sm">
        <h3 className="text-lg font-bold text-zinc-900 dark:text-white mb-4">Featured Scenarios</h3>
        <div className="flex gap-4 overflow-x-auto pb-2 scrollbar-thin scrollbar-thumb-zinc-350 dark:scrollbar-thumb-zinc-700 scrollbar-track-transparent snap-x">
          {featuredScenarios.map((scenario) => (
            <div
              key={scenario.title}
              className="min-w-[280px] md:min-w-[320px] snap-start p-5 rounded-2xl border border-zinc-200/60 dark:border-zinc-800/60 bg-white/80 dark:bg-zinc-900/80 hover:shadow-md hover:border-teal-500/50 dark:hover:border-teal-500/50 transition-all flex flex-col justify-between"
            >
              <div>
                <div className="flex items-center justify-between mb-2">
                  <span className="text-xs font-semibold px-2 py-0.5 rounded-full bg-teal-50 dark:bg-teal-950/50 text-teal-700 dark:text-teal-400">
                    {scenario.tag}
                  </span>
                  <span className="text-xs text-zinc-500 dark:text-zinc-400 font-medium">{scenario.team}</span>
                </div>
                <h4 className="text-base font-bold text-zinc-900 dark:text-white mb-1">{scenario.title}</h4>
                <p className="text-xs text-zinc-650 dark:text-zinc-400 leading-relaxed mb-4">{scenario.desc}</p>
              </div>
              <div className="flex items-center gap-1 mt-auto pt-3 border-t border-zinc-100 dark:border-zinc-800">
                <span className="text-[10px] uppercase font-bold text-zinc-400 dark:text-zinc-550 mr-1">Includes:</span>
                {scenario.members.map((member) => (
                  <span key={member} className="text-[10px] font-semibold bg-zinc-100 dark:bg-zinc-800 text-zinc-600 dark:text-zinc-300 px-1.5 py-0.5 rounded-md truncate max-w-[90px]">
                    {member}
                  </span>
                ))}
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Search Filter Panel */}
      <div className="rounded-2xl border border-zinc-200/80 dark:border-zinc-800/80 bg-white/60 dark:bg-zinc-900/60 backdrop-blur-[30px] saturate-[210%] p-5 shadow-sm">
        <SectionHeader
          title={panel === 'teams' ? 'Expert Teams' : 'Browse experts'}
          detail="Search by job, pick a single expert or a coordinated team, then summon it into the task composer."
        />
        <div className="relative mt-3">
          <input
            aria-label="Search experts"
            value={query}
            onChange={(event) => setQuery(event.target.value)}
            className="h-11 w-full rounded-xl border border-zinc-300 dark:border-zinc-700 bg-white/80 dark:bg-zinc-900/80 pl-10 pr-4 text-sm outline-none focus:border-teal-550 focus:ring-2 focus:ring-teal-100 dark:focus:ring-teal-950/30 transition-all dark:text-white"
            placeholder="Search experts, roles, skills, connectors..."
          />
          <div className="absolute left-3 top-1/2 -translate-y-1/2 text-zinc-400">
            🔍
          </div>
        </div>
      </div>

      {/* Most Used Scenarios */}
      <div className="rounded-2xl border border-amber-205/65 dark:border-amber-900/35 bg-amber-50/40 dark:bg-amber-950/10 p-5">
        <h3 className="text-xs font-bold uppercase tracking-wider text-amber-900 dark:text-amber-300">Most used</h3>
        <div className="mt-3 grid gap-3 md:grid-cols-3">
          {mostUsed.map((item) => (
            <button
              key={item.id}
              type="button"
              onClick={() => summon(item)}
              className="rounded-xl border border-amber-200 dark:border-amber-900/30 bg-white dark:bg-zinc-900 px-4 py-3 text-left hover:shadow-sm transition-all outline-none"
            >
              <div className="text-sm font-bold text-zinc-900 dark:text-zinc-100">{item.name}</div>
              <div className="text-xs text-zinc-500 dark:text-zinc-400 mt-1">{item.usageCount} runs</div>
            </button>
          ))}
        </div>
      </div>

      {/* Grid List */}
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
    <article data-testid={slugTestId(item.id)} className="rounded-2xl border border-zinc-200 dark:border-zinc-800/80 bg-white/80 dark:bg-zinc-900/80 p-5 shadow-sm hover:shadow-md transition-all duration-205 flex flex-col justify-between">
      <div>
        <div className="flex items-start gap-3">
          <div className="flex h-11 w-11 shrink-0 items-center justify-center rounded-xl bg-teal-50 dark:bg-teal-950/40 text-sm font-extrabold text-teal-700 dark:text-teal-450">
            {itemInitials(item.name)}
          </div>
          <div className="min-w-0 flex-1">
            <div className="flex items-center justify-between gap-2">
              <h3 className="text-lg font-bold text-zinc-900 dark:text-white leading-normal">{item.name}</h3>
              <StatusPill>{item.kind === 'team' ? 'Team' : 'Expert'}</StatusPill>
            </div>
            <p className="text-[10px] font-bold uppercase tracking-wider text-rose-600 dark:text-rose-455">{item.category}</p>
          </div>
        </div>
        <p className="mt-3 text-xs leading-relaxed text-zinc-650 dark:text-zinc-400">{item.summary}</p>
        <div className="mt-3 flex flex-wrap gap-2">
          {item.strengths.map((strength) => (
            <span key={strength} className="rounded-lg bg-zinc-100 dark:bg-zinc-800/85 px-2.5 py-1 text-xs font-semibold text-zinc-700 dark:text-zinc-300">
              {strength}
            </span>
          ))}
        </div>
        {item.members && (
          <div className="mt-4 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-zinc-50/50 dark:bg-zinc-950/30 p-3.5">
            <div className="text-[10px] font-bold uppercase tracking-wider text-zinc-400 dark:text-zinc-500">Team members</div>
            <p className="mt-1 text-xs font-semibold text-zinc-700 dark:text-zinc-350">{item.members.join(', ')}</p>
          </div>
        )}
        <div className="mt-4">
          <div className="text-[10px] font-bold uppercase tracking-wider text-zinc-400 dark:text-zinc-550">Use cases</div>
          <ul className="mt-2 space-y-1.5 text-xs text-zinc-650 dark:text-zinc-400">
            {item.examples.map((example) => (
              <li key={example} className="flex items-start gap-1.5">
                <span className="text-teal-650 dark:text-teal-500 select-none">•</span>
                <span>{example}</span>
              </li>
            ))}
          </ul>
        </div>
        {showDetail && (
          <div className="mt-4 rounded-xl border border-teal-200/50 dark:border-teal-900/30 bg-teal-50/30 dark:bg-teal-950/20 p-4">
            <div className="text-[10px] font-bold uppercase tracking-wider text-teal-800 dark:text-teal-400">Expert detail</div>
            <p className="mt-1 text-xs text-teal-900 dark:text-teal-300">{item.name} uses {item.model} with {item.skills.join(', ')}.</p>
            <div className="mt-3 flex flex-wrap gap-2">
              <button type="button" onClick={() => summon(item)} className="rounded-lg bg-teal-600 hover:bg-teal-700 text-white px-3 py-1.5 text-xs font-bold transition-all">
                Summon into chat
              </button>
              <button type="button" className="rounded-lg border border-teal-200 dark:border-teal-900/40 bg-white dark:bg-zinc-900 px-3 py-1.5 text-xs font-bold text-teal-800 dark:text-teal-300 hover:bg-zinc-50 dark:hover:bg-zinc-800 transition-all">
                Favorite
              </button>
            </div>
          </div>
        )}
      </div>
      <div className="mt-5 grid grid-cols-2 gap-3 pt-3 border-t border-zinc-100 dark:border-zinc-800/80">
        <button
          type="button"
          onClick={() => setShowDetail((current) => !current)}
          className="h-9 rounded-xl border border-zinc-350 dark:border-zinc-700 text-xs font-bold text-zinc-700 dark:text-zinc-300 hover:bg-zinc-50 dark:hover:bg-zinc-800/60 transition-colors outline-none"
        >
          Details
        </button>
        <button
          type="button"
          onClick={() => summon(item)}
          className="h-9 rounded-xl bg-teal-600 hover:bg-teal-700 dark:bg-teal-500 dark:hover:bg-teal-600 text-xs font-bold text-white transition-colors outline-none shadow-sm"
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
    <section className="rounded-2xl border border-zinc-200/80 dark:border-zinc-800/80 bg-white/60 dark:bg-zinc-900/60 backdrop-blur-[30px] saturate-[210%] p-5 shadow-sm">
      <div className="flex items-center justify-between gap-3 mb-4">
        <div>
          <h2 className="text-base font-bold text-zinc-900 dark:text-white">Task Composer</h2>
          <p className="text-xs font-semibold text-teal-600 dark:text-teal-400">{summonMessage}</p>
        </div>
        <span className="text-[10px] font-bold px-2 py-0.5 bg-zinc-100 dark:bg-zinc-800 text-zinc-600 dark:text-zinc-300 rounded-md">
          {selected.model}
        </span>
      </div>

      {/* Unified Chat Input Frame (Screenshot 2 Style) */}
      <div className="rounded-2xl border border-zinc-250 dark:border-zinc-700 bg-white dark:bg-zinc-900 p-3 shadow-inner hover:border-zinc-350 dark:hover:border-zinc-650 transition-all flex flex-col gap-2">
        <textarea
          id="task-prompt"
          aria-label="Task prompt"
          value={taskPrompt}
          onChange={(event) => setTaskPrompt(event.target.value)}
          className="min-h-[100px] w-full resize-none bg-transparent text-sm leading-relaxed outline-none dark:text-white placeholder-zinc-400"
          placeholder="What can I help you with today? Reference files with @, summon tools with /"
        />
        
        {/* Input Tool Bar */}
        <div className="flex flex-wrap items-center justify-between gap-2 pt-2 border-t border-zinc-100 dark:border-zinc-800">
          <div className="flex flex-wrap items-center gap-1.5 text-xs">
            {/* Mode Selector Option */}
            <select
              value={mode}
              onChange={(e) => setMode(e.target.value as Mode)}
              className="h-7 px-2 rounded-lg bg-zinc-50 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 text-zinc-700 dark:text-zinc-300 font-semibold cursor-pointer outline-none hover:bg-zinc-100 dark:hover:bg-zinc-700"
            >
              <option value="Ask">Ask</option>
              <option value="Craft">Craft</option>
              <option value="Plan">Plan</option>
            </select>

            {/* Model Selector Option */}
            <label className="flex items-center">
              <span className="sr-only">Model</span>
              <select
                value={model}
                onChange={(e) => setModel(e.target.value)}
                className="h-7 px-2 rounded-lg bg-zinc-50 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 text-zinc-700 dark:text-zinc-300 font-semibold cursor-pointer outline-none hover:bg-zinc-100 dark:hover:bg-zinc-700"
              >
                {modelOptions.map((opt) => (
                  <option key={opt} value={opt}>{opt}</option>
                ))}
              </select>
            </label>

            {/* Skills Status */}
            <span className="h-7 px-2 rounded-lg bg-zinc-50 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 text-zinc-600 dark:text-zinc-400 inline-flex items-center gap-1 font-medium select-none">
              ⚙️ Skills ({enabledSkills.length})
            </span>

            {/* Permission Indicator */}
            <span className="h-7 px-2 rounded-lg bg-zinc-50 dark:bg-zinc-800 border border-zinc-200 dark:border-zinc-700 text-zinc-500 dark:text-zinc-400 inline-flex items-center gap-1 select-none">
              🔒 Default Safe
            </span>
          </div>

          {/* Action Icons right aligned */}
          <div className="flex items-center gap-2">
            <button
              type="button"
              title="Add Attachments"
              onClick={() => document.getElementById('attachments-ref')?.focus()}
              className="p-1 text-zinc-400 hover:text-zinc-600 dark:hover:text-zinc-200 transition-colors"
            >
              📎
            </button>
            <button
              type="button"
              title="Voice Input"
              className="p-1 text-zinc-400 hover:text-zinc-600 dark:hover:text-zinc-200 transition-colors"
            >
              🎙️
            </button>
            <button
              type="button"
              title="Refine Prompt"
              className="p-1 text-zinc-400 hover:text-zinc-600 dark:hover:text-zinc-200 transition-colors animate-pulse"
            >
              ✨
            </button>
          </div>
        </div>
      </div>

      <div className="mt-4 grid gap-3">
        <div className="grid grid-cols-2 gap-2">
          <label className="text-xs font-bold text-zinc-500 dark:text-zinc-400 uppercase">
            Custom provider
            <input
              aria-label="Custom provider"
              value={customProvider}
              onChange={(event) => setCustomProvider(event.target.value)}
              className="mt-1 h-9 w-full rounded-xl border border-zinc-300 dark:border-zinc-700 bg-white/70 dark:bg-zinc-900/70 px-3 text-xs outline-none dark:text-white"
              placeholder="OpenAI-compatible URL"
            />
          </label>
          <label className="text-xs font-bold text-zinc-500 dark:text-zinc-400 uppercase">
            Work directory
            <input
              aria-label="Work directory"
              value={workDirectory}
              onChange={(event) => setWorkDirectory(event.target.value)}
              className="mt-1 h-9 w-full rounded-xl border border-zinc-300 dark:border-zinc-700 bg-white/70 dark:bg-zinc-900/70 px-3 text-xs outline-none dark:text-white"
              placeholder="/workspace/current-task"
            />
          </label>
        </div>
        
        <label className="text-xs font-bold text-zinc-500 dark:text-zinc-400 uppercase">
          Workspace Scoping
          <select
            value={workspace}
            onChange={(event) => setWorkspace(event.target.value)}
            className="mt-1 h-9 w-full rounded-xl border border-zinc-300 dark:border-zinc-700 bg-white/70 dark:bg-zinc-900/70 px-3 text-xs outline-none dark:text-white"
          >
            {workspaces.map((option) => (
              <option key={option} className="dark:bg-zinc-900">{option}</option>
            ))}
          </select>
        </label>

        <label className="text-xs font-bold text-zinc-500 dark:text-zinc-400 uppercase">
          Context references (@ tags)
          <input
            aria-label="Context references"
            value={contextReferences}
            onChange={(event) => setContextReferences(event.target.value)}
            className="mt-1 h-9 w-full rounded-xl border border-zinc-300 dark:border-zinc-700 bg-white/70 dark:bg-zinc-900/70 px-3 text-xs outline-none dark:text-white"
            placeholder="@orders @inventory @customer-notes"
          />
        </label>
        <label className="text-xs font-bold text-zinc-500 dark:text-zinc-400 uppercase">
          Attachments
          <input
            id="attachments-ref"
            aria-label="Attachments"
            value={attachments}
            onChange={(event) => setAttachments(event.target.value)}
            className="mt-1 h-9 w-full rounded-xl border border-zinc-300 dark:border-zinc-700 bg-white/70 dark:bg-zinc-900/70 px-3 text-xs outline-none dark:text-white"
            placeholder="Drop files, screenshots, PDFs, CSVs"
          />
        </label>
        <div className="grid grid-cols-3 gap-2">
          <button type="button" className="h-9 rounded-xl border border-zinc-200 dark:border-zinc-700 bg-zinc-50 dark:bg-zinc-800 text-xs font-bold text-zinc-700 dark:text-zinc-300 hover:bg-zinc-100 dark:hover:bg-zinc-700 transition-colors">
            Screenshot
          </button>
          <label className="text-xs font-bold text-zinc-500 dark:text-zinc-400 uppercase">
            Output format
            <select
              aria-label="Output format"
              value={outputFormat}
              onChange={(event) => setOutputFormat(event.target.value)}
              className="mt-1 h-9 w-full rounded-xl border border-zinc-300 dark:border-zinc-700 bg-white/70 dark:bg-zinc-900/70 px-2 text-xs outline-none dark:text-white"
            >
              <option className="dark:bg-zinc-900">Brief</option>
              <option className="dark:bg-zinc-900">Table</option>
              <option className="dark:bg-zinc-900">Document</option>
              <option className="dark:bg-zinc-900">Spreadsheet</option>
            </select>
          </label>
          <label className="text-xs font-bold text-zinc-500 dark:text-zinc-400 uppercase">
            Task constraints
            <input
              aria-label="Task constraints"
              value={taskConstraints}
              onChange={(event) => setTaskConstraints(event.target.value)}
              className="mt-1 h-9 w-full rounded-xl border border-zinc-300 dark:border-zinc-700 bg-white/70 dark:bg-zinc-900/70 px-2 text-xs outline-none dark:text-white"
              placeholder="budget, tone"
            />
          </label>
        </div>
        <div className="flex flex-wrap gap-1.5 mt-3">
          {['Local Ollama', 'Vision', 'Tool use', 'Long context', 'Parallel tasks'].map((capability) => (
            <span key={capability} className="rounded-lg border border-zinc-200 dark:border-zinc-800 bg-zinc-50 dark:bg-zinc-850 px-2 py-0.5 text-[10px] font-bold text-zinc-650 dark:text-zinc-300">
              {capability}
            </span>
          ))}
        </div>
      </div>
      
      <div className="mt-4 grid gap-3 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-zinc-50/50 dark:bg-zinc-900/50 p-4 text-xs">
        <div>
          <div className="font-bold text-zinc-500 dark:text-zinc-400 uppercase tracking-wider text-[10px]">Active Skills</div>
          <p className="mt-1 font-semibold text-zinc-800 dark:text-zinc-200">{enabledSkills.join(', ') || 'None selected'}</p>
        </div>
        <div>
          <div className="font-bold text-zinc-500 dark:text-zinc-400 uppercase tracking-wider text-[10px]">Active Connectors</div>
          <p className="mt-1 font-semibold text-zinc-800 dark:text-zinc-200">{enabledConnectors.join(', ') || 'None selected'}</p>
        </div>
        <p className="text-[10px] text-zinc-450 dark:text-zinc-500 leading-normal pt-2 border-t border-zinc-100 dark:border-zinc-850">
          ⚠️ Cost warning: Craft and Plan modes can invoke automatic tools and consume more agent budget.
        </p>
      </div>

      {runError && <p className="mt-3 text-xs font-bold text-red-650 dark:text-red-400">{runError}</p>}
      {runMessage && <p className="mt-3 text-xs font-bold text-emerald-600 dark:text-emerald-400">{runMessage}</p>}
      
      <button
        type="button"
        onClick={startTask}
        disabled={running || !taskPrompt.trim()}
        className="mt-4 h-11 w-full rounded-xl bg-teal-600 hover:bg-teal-700 dark:bg-teal-500 dark:hover:bg-teal-600 text-sm font-bold text-white transition-colors disabled:cursor-not-allowed disabled:bg-zinc-300 dark:disabled:bg-zinc-800 dark:disabled:text-zinc-500 shadow-sm"
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
    <section className="rounded-2xl border border-zinc-200/80 dark:border-zinc-800/80 bg-white/60 dark:bg-zinc-900/60 backdrop-blur-[30px] saturate-[210%] p-5 shadow-sm">
      <SectionHeader title="Skill Market" detail="Install, disable, search, upload, or create natural-language skills for experts." />
      <div className="mb-4 flex flex-wrap gap-2">
        <button className="rounded-full bg-teal-600 hover:bg-teal-700 text-white px-4 py-2 text-xs font-bold transition-colors" type="button">Find skill</button>
        <button className="rounded-full border border-zinc-250 dark:border-zinc-800 px-4 py-2 text-xs font-bold text-zinc-700 dark:text-zinc-300 hover:bg-zinc-50 dark:hover:bg-zinc-900 transition-colors" type="button">Search installed skills</button>
        <button className="rounded-full border border-zinc-250 dark:border-zinc-800 px-4 py-2 text-xs font-bold text-zinc-700 dark:text-zinc-300 hover:bg-zinc-50 dark:hover:bg-zinc-900 transition-colors" type="button">Upload local skill</button>
        <button className="rounded-full border border-zinc-250 dark:border-zinc-800 px-4 py-2 text-xs font-bold text-zinc-700 dark:text-zinc-300 hover:bg-zinc-50 dark:hover:bg-zinc-900 transition-colors" type="button">Create skill from prompt</button>
        <button className="rounded-full border border-zinc-250 dark:border-zinc-800 px-4 py-2 text-xs font-bold text-zinc-700 dark:text-zinc-300 hover:bg-zinc-50 dark:hover:bg-zinc-900 transition-colors" type="button">Disable skill</button>
        <button className="rounded-full border border-zinc-250 dark:border-zinc-800 px-4 py-2 text-xs font-bold text-zinc-700 dark:text-zinc-300 hover:bg-zinc-50 dark:hover:bg-zinc-900 transition-colors" type="button">Uninstall skill</button>
        <button className="rounded-full border border-zinc-250 dark:border-zinc-800 px-4 py-2 text-xs font-bold text-zinc-700 dark:text-zinc-300 hover:bg-zinc-50 dark:hover:bg-zinc-900 transition-colors" type="button">Bulk uninstall</button>
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
              className={`rounded-2xl border p-5 text-left transition-all hover:shadow-sm ${
                enabled
                  ? 'border-teal-500/80 bg-teal-50/20 dark:bg-teal-950/20'
                  : 'border-zinc-200/80 dark:border-zinc-800/80 bg-white/80 dark:bg-zinc-900/80'
              }`}
            >
              <div className="flex items-center justify-between gap-3">
                <h3 className="font-bold text-zinc-900 dark:text-white text-sm">{skill.name}</h3>
                <span className={`text-[10px] font-bold px-2 py-0.5 rounded-md ${enabled ? 'bg-teal-100 dark:bg-teal-900/60 text-teal-800 dark:text-teal-300' : 'bg-zinc-100 dark:bg-zinc-800 text-zinc-650 dark:text-zinc-400'}`}>
                  {enabled ? 'Enabled' : skill.status}
                </span>
              </div>
              <p className="mt-2 text-xs text-zinc-650 dark:text-zinc-400 leading-relaxed">{skill.description}</p>
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
    <section className="rounded-2xl border border-zinc-200/80 dark:border-zinc-800/80 bg-white/60 dark:bg-zinc-900/60 backdrop-blur-[30px] saturate-[210%] p-5 shadow-sm">
      <SectionHeader title="Connector Center" detail="Connect external data and service actions for expert tasks." />
      <div className="mb-4 grid gap-3 md:grid-cols-3">
        {['Create custom connector', 'MCP endpoint', 'Notification channel'].map((label) => (
          <label key={label} className="text-xs font-bold text-zinc-500 dark:text-zinc-400 uppercase">
            {label}
            <input className="mt-1 h-9 w-full rounded-xl border border-zinc-350 dark:border-zinc-700 bg-white/70 dark:bg-zinc-900/70 px-3 text-xs outline-none dark:text-white" placeholder={label} />
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
              className={`rounded-2xl border p-5 text-left transition-all hover:shadow-sm ${
                enabled
                  ? 'border-teal-500/80 bg-teal-50/20 dark:bg-teal-950/20'
                  : 'border-zinc-200/80 dark:border-zinc-800/80 bg-white/80 dark:bg-zinc-900/80'
              }`}
            >
              <div className="flex items-center justify-between gap-3">
                <h3 className="font-bold text-zinc-900 dark:text-white text-sm">{connector.name}</h3>
                <span className={`text-[10px] font-bold px-2 py-0.5 rounded-md ${enabled ? 'bg-teal-100 dark:bg-teal-900/60 text-teal-800 dark:text-teal-300' : 'bg-zinc-100 dark:bg-zinc-800 text-zinc-650 dark:text-zinc-400'}`}>
                  {enabled ? 'Selected' : connector.status}
                </span>
              </div>
              <p className="mt-2 text-xs text-zinc-650 dark:text-zinc-400 leading-relaxed">{connector.description}</p>
            </button>
          );
        })}
      </div>
    </section>
  );
}
function AutomationsPanel() {
  const scheduledTasks = [
    { name: "Weekly business review", rule: "Every Monday 9:00", nextRun: "Starts in 18 hours", status: "Active" },
    { name: "Daily inbox risk scan", rule: "Every 1 Hour", nextRun: "Paused", status: "Paused" },
    { name: "Low inventory recovery", rule: "Every Day 20:00", nextRun: "Paused", status: "Paused" }
  ];

  const completedHistory = [
    { name: "Weekly stats execution", status: "Success", time: "5 hours ago" },
    { name: "Weekly archive extraction", status: "Success", time: "1 day ago" },
    { name: "Daily inbox risk scan", status: "Success", time: "1 day ago" }
  ];

  return (
    <section className="rounded-2xl border border-zinc-200/80 dark:border-zinc-800/80 bg-white/60 dark:bg-zinc-900/60 backdrop-blur-[30px] saturate-[210%] p-5 shadow-sm">
      <SectionHeader title="Scheduled Tasks" detail="Configure recurring expert runs with prompt, workspace, model, skills, connectors, and notifications." />
      
      <div className="mb-4 grid gap-3 md:grid-cols-3">
        <label className="text-xs font-bold text-zinc-500 dark:text-zinc-400 uppercase">
          Schedule rule
          <input className="mt-1 h-9 w-full rounded-xl border border-zinc-350 dark:border-zinc-700 bg-white/70 dark:bg-zinc-900/70 px-3 text-xs outline-none dark:text-white" placeholder="Every Monday 9:00" />
        </label>
        <div className="rounded-xl border border-zinc-250 dark:border-zinc-800 bg-zinc-50/50 dark:bg-zinc-900/50 p-3 text-xs font-bold text-zinc-700 dark:text-zinc-300">Execution history</div>
        <div className="rounded-xl border border-zinc-250 dark:border-zinc-800 bg-zinc-50/50 dark:bg-zinc-900/50 p-3 text-xs font-bold text-zinc-700 dark:text-zinc-300">Push notification</div>
      </div>

      <div className="flex items-center gap-2 mb-6">
        <button className="rounded-full bg-teal-600 hover:bg-teal-700 text-white px-4 py-2 text-xs font-bold transition-colors">+ Add New</button>
        <button className="rounded-full border border-zinc-250 dark:border-zinc-800 px-4 py-2 text-xs font-bold text-zinc-700 dark:text-zinc-300 hover:bg-zinc-50 dark:hover:bg-zinc-900 transition-colors">From Template</button>
      </div>

      <div className="space-y-6">
        <div>
          <h3 className="text-xs font-extrabold uppercase tracking-wider text-zinc-400 dark:text-zinc-500 mb-3">Scheduled Runs</h3>
          <div className="space-y-3">
            {scheduledTasks.map((task) => (
              <div key={task.name} className="flex items-center justify-between p-4 rounded-2xl border border-zinc-200/80 dark:border-zinc-800/80 bg-white/80 dark:bg-zinc-900/80">
                <div className="flex items-center gap-3">
                  <span className={`w-2.5 h-2.5 rounded-full ${task.status === 'Active' ? 'bg-emerald-500' : 'bg-zinc-400 dark:bg-zinc-650'}`} />
                  <div>
                    <h4 className="text-sm font-bold text-zinc-900 dark:text-white leading-normal">{task.name}</h4>
                    <p className="text-[11px] text-zinc-500 dark:text-zinc-400 mt-0.5">{task.rule}</p>
                  </div>
                </div>
                <span className="text-[11px] font-semibold text-zinc-500 dark:text-zinc-400">{task.nextRun}</span>
              </div>
            ))}
          </div>
        </div>

        <div>
          <h3 className="text-xs font-extrabold uppercase tracking-wider text-zinc-400 dark:text-zinc-550 mb-3">Completed Executions</h3>
          <div className="space-y-2">
            {completedHistory.map((history, idx) => (
              <div key={idx} className="flex items-center justify-between py-2.5 border-b border-zinc-100 dark:border-zinc-850 last:border-b-0 text-xs">
                <div className="flex items-center gap-2">
                  <span className="w-1.5 h-1.5 rounded-full bg-emerald-500" />
                  <span className="font-bold text-zinc-800 dark:text-zinc-200">{history.name}</span>
                </div>
                <div className="flex items-center gap-4 text-zinc-500 dark:text-zinc-400">
                  <span className="bg-emerald-50 dark:bg-emerald-950/30 text-emerald-700 dark:text-emerald-400 px-2 py-0.5 rounded-md text-[10px] font-bold">Success</span>
                  <span>{history.time}</span>
                </div>
              </div>
            ))}
          </div>
        </div>
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
    <section className="rounded-2xl border border-zinc-200/80 dark:border-zinc-800/80 bg-white/60 dark:bg-zinc-900/60 backdrop-blur-[30px] saturate-[210%] p-5 shadow-sm">
      <SectionHeader title="Consolidated Memory" detail="Review and override what AI agents remember about your business." />

      {loading ? (
        <p className="text-xs text-zinc-550 dark:text-zinc-400">Loading memories...</p>
      ) : items.length === 0 ? (
        <p className="text-xs text-zinc-555 dark:text-zinc-400">No consolidated memories found.</p>
      ) : (
        <div className="space-y-3">
          {items.map((memory: any) => (
            <div key={memory.id} className="rounded-2xl border border-zinc-200/80 dark:border-zinc-800/80 bg-white/80 dark:bg-zinc-900/80 p-4 text-xs flex flex-col gap-2">
              <div className="flex justify-between items-start">
                <div className="font-bold text-zinc-900 dark:text-white">{memory.source_type}</div>
                <button
                  type="button"
                  onClick={() => toggleOverride(memory.id, memory.owner_override)}
                  className={`text-[10px] px-2 py-1 rounded-lg font-bold transition-colors ${memory.owner_override ? 'bg-teal-600 text-white hover:bg-teal-700' : 'bg-zinc-100 dark:bg-zinc-800 text-zinc-700 dark:text-zinc-350 hover:bg-zinc-200 dark:hover:bg-zinc-700'}`}
                >
                  {memory.owner_override ? 'Owner Override: ON' : 'Owner Override: OFF'}
                </button>
              </div>
              <p className="whitespace-pre-wrap text-zinc-650 dark:text-zinc-405 leading-relaxed">{memory.content}</p>
              <div className="text-[10px] text-zinc-450 dark:text-zinc-500 flex gap-4 pt-2 border-t border-zinc-100 dark:border-zinc-850">
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
    <section className="rounded-2xl border border-zinc-200/80 dark:border-zinc-800/80 bg-white/60 dark:bg-zinc-900/60 backdrop-blur-[30px] saturate-[210%] p-5 shadow-sm">
      <SectionHeader title="Explore Templates" detail="Curated templates prefill prompt, expert, skills, and connector choices." />
      <div className="grid gap-3 md:grid-cols-2">
        {exploreTemplates.map((template, index) => (
          <button
            key={template}
            type="button"
            onClick={() => summon(index % 2 === 0 ? expertTeams[0] : experts[0])}
            className="rounded-2xl border border-zinc-200/85 dark:border-zinc-800 bg-white/80 dark:bg-zinc-900/80 hover:shadow-md hover:border-teal-500/50 p-5 text-left transition-all outline-none"
          >
            <h3 className="font-bold text-zinc-900 dark:text-white text-sm">{template}</h3>
            <p className="mt-2 text-xs text-zinc-500 dark:text-zinc-400">Make my version with one click.</p>
          </button>
        ))}
      </div>
    </section>
  );
}
function RemotePanel() {
  return (
    <section className="rounded-2xl border border-zinc-200/80 dark:border-zinc-800/80 bg-white/60 dark:bg-zinc-900/60 backdrop-blur-[30px] saturate-[210%] p-5 shadow-sm">
      <SectionHeader title="Remote Assistant Control" detail="Summon experts and receive notifications from business chat tools." />
      <div className="mb-4 rounded-xl border border-zinc-200 dark:border-zinc-800 bg-zinc-50/50 dark:bg-zinc-950/30 p-3 text-xs font-bold text-teal-700 dark:text-teal-400 font-mono">
        /summon Growth Strategist
      </div>
      <div className="grid gap-3 md:grid-cols-3">
        {remoteAssistants.map((name) => (
          <div key={name} className="rounded-2xl border border-zinc-200/80 dark:border-zinc-800/80 bg-white/80 dark:bg-zinc-900/80 p-4 text-center">
            <div className="font-bold text-zinc-900 dark:text-white text-xs">{name}</div>
            <div className="mt-1 text-[10px] text-zinc-450 dark:text-zinc-500 font-medium">Available</div>
          </div>
        ))}
      </div>
    </section>
  );
}
function DataPanel() {
  return (
    <section className="rounded-2xl border border-zinc-200/80 dark:border-zinc-800/80 bg-white/60 dark:bg-zinc-900/60 backdrop-blur-[30px] saturate-[210%] p-5 shadow-sm">
      <SectionHeader title="Data Management" detail="Manage shared files, task archives, generated outputs, and workspace history." />
      <div className="grid gap-3 md:grid-cols-3">
        {['Shared files', 'Archived tasks', 'Generated outputs', 'Workspace history', 'Download center', 'Unshare queue'].map((item) => (
          <div key={item} className="rounded-2xl border border-zinc-200/80 dark:border-zinc-800/80 bg-white/80 dark:bg-zinc-900/80 p-4 font-bold text-zinc-900 dark:text-white text-xs text-center cursor-pointer hover:border-teal-500/50 transition-all">
            {item}
          </div>
        ))}
      </div>
    </section>
  );
}
function OperationsPanel() {
  return (
    <section className="rounded-2xl border border-zinc-200/80 dark:border-zinc-800/80 bg-white/60 dark:bg-zinc-900/60 backdrop-blur-[30px] saturate-[210%] p-5 shadow-sm">
      <SectionHeader title="AI Departments" detail="Operational agents stay visible for backwards-compatible business management." />
      <div className="grid gap-3 md:grid-cols-2">
        {departments.map((department) => (
          <div key={department.id} className="rounded-2xl border border-zinc-200/80 dark:border-zinc-800/80 bg-white/80 dark:bg-zinc-900/80 p-5">
            <div className="flex items-center justify-between gap-3 mb-2">
              <h3 className="font-bold text-zinc-900 dark:text-white text-sm">{department.name}</h3>
              <StatusPill>{department.status}</StatusPill>
            </div>
            <p className="text-[10px] font-bold uppercase tracking-wider text-rose-600 dark:text-rose-455 mb-2">{department.role}</p>
            <p className="text-xs text-zinc-650 dark:text-zinc-400 leading-relaxed">{department.description}</p>
          </div>
        ))}
      </div>
    </section>
  );
}
function WorkflowsPanel({ workflows, setWorkflows }: { workflows: WorkflowRecord[], setWorkflows: React.Dispatch<React.SetStateAction<WorkflowRecord[]>> }) {
  const handleSaveWorkflow = async (name: string, task: string) => {
    // 1. Try to run it as a visual workflow via our new bridge API
    try {
      const parsedTask = JSON.parse(task);
      if (parsedTask.nodes && parsedTask.version) {
        const wfRes = await fetch('/api/workflow/run', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(parsedTask),
        });

        if (wfRes.ok) {
           const wfData = await wfRes.json();
           setWorkflows(current => [{
               id: Date.now().toString(),
               name,
               workflow: 'visual_workflow',
               task: "Visual Workflow Result: " + (wfData.result || JSON.stringify(wfData)),
               status: wfData.success ? 'completed' : 'failed',
               command: '',
               created_at: new Date().toISOString()
           }, ...current]);
           return;
        } else {
           console.warn("Visual workflow API failed, falling back to legacy workflow endpoint");
        }
      }
    } catch (e) {
      // Not JSON or other error, fallback to legacy
    }

    // 2. Fallback to standard ohc_review_branch workflow task string
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
    <section className="border border-[rgba(255,255,255,0.4)] bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] saturate-[210%] p-4">
      <SectionHeader title="Workflows" detail="Active expert and expert-team runs." />

      <div className="mb-8">
        <AgentWorkflowBuilder onSave={handleSaveWorkflow} />
      </div>
      {workflows.length === 0 ? (
        <p className="border border-dashed border-zinc-300 p-4 text-sm text-zinc-600">No workflows yet.</p>
      ) : (
        <div className="space-y-3">
          {workflows.map((workflow) => (
            <div key={workflow.id} className="border border-[rgba(255,255,255,0.4)] bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] saturate-[210%] p-4">
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
    <section className="rounded-2xl border border-zinc-200/80 dark:border-zinc-800/80 bg-white/60 dark:bg-zinc-900/60 backdrop-blur-[30px] saturate-[210%] p-5 shadow-sm">
      <SectionHeader title="Activity Feed" detail="Realtime expert and department activity." />
      {feed.length === 0 ? (
        <p className="rounded-xl border border-dashed border-zinc-350 dark:border-zinc-750 p-4 text-xs text-zinc-500 dark:text-zinc-400">No activity yet.</p>
      ) : (
        <div className="space-y-3">
          {feed.map((item) => (
            <div key={item.id} className="rounded-2xl border border-zinc-200/80 dark:border-zinc-800/80 bg-white/80 dark:bg-zinc-900/80 p-4">
              <div className="text-[10px] font-bold uppercase tracking-wider text-teal-600 dark:text-teal-400">{item.department}</div>
              <p className="mt-2 text-xs text-zinc-700 dark:text-zinc-300 leading-relaxed">{item.description}</p>
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
    <section className="rounded-2xl border border-zinc-200/80 dark:border-zinc-800/80 bg-white/60 dark:bg-zinc-900/60 backdrop-blur-[30px] saturate-[210%] p-5 shadow-sm">
      <SectionHeader title="Needs Approval" detail="Review high-risk drafts before experts execute or send." />
      {approvals.length === 0 ? (
        <div className="rounded-xl border border-dashed border-zinc-350 dark:border-zinc-750 p-5 text-center">
          <h3 className="font-bold text-zinc-900 dark:text-white text-sm">All Caught Up!</h3>
          <p className="mt-1 text-xs text-zinc-500 dark:text-zinc-400">Your AI team has no pending approvals.</p>
        </div>
      ) : (
        <div className="space-y-3">
          {approvals.map((item) => (
            <div key={item.id} className="rounded-2xl border border-zinc-200/80 dark:border-zinc-800/80 bg-white/80 dark:bg-zinc-900/80 p-5">
              <div className="text-[10px] font-bold uppercase tracking-wider text-rose-600 dark:text-rose-455">{item.department}</div>
              <p className="mt-2 text-xs text-zinc-700 dark:text-zinc-300 leading-relaxed">{item.description}</p>
              <div className="mt-4 flex gap-2">
                <button
                  type="button"
                  onClick={() => decideApproval(item.id, false)}
                  className="h-9 flex-1 rounded-xl border border-zinc-300 dark:border-zinc-700 bg-white dark:bg-zinc-900 text-xs font-bold text-zinc-700 dark:text-zinc-300 hover:bg-zinc-50 dark:hover:bg-zinc-800 transition-colors"
                >
                  Edit Draft
                </button>
                <button
                  type="button"
                  onClick={() => decideApproval(item.id, true)}
                  className="h-9 flex-1 rounded-xl bg-teal-600 hover:bg-teal-700 dark:bg-teal-500 dark:hover:bg-teal-600 text-xs font-bold text-white transition-colors"
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
