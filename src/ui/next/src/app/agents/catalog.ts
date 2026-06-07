export type AgentKind = 'expert' | 'team';

export type ExpertCatalogItem = {
  id: string;
  kind: AgentKind;
  name: string;
  role: string;
  category: string;
  summary: string;
  strengths: string[];
  examples: string[];
  usageCount: number;
  model: string;
  skills: string[];
  connectors: string[];
  members?: string[];
};

export type SkillItem = {
  id: string;
  name: string;
  category: string;
  status: 'Installed' | 'Recommended' | 'Disabled';
  description: string;
};

export type ConnectorItem = {
  id: string;
  name: string;
  status: 'Connected' | 'Available' | 'Needs setup';
  description: string;
};

export const experts: ExpertCatalogItem[] = [
  {
    id: 'growth-strategist',
    kind: 'expert',
    name: 'Growth Strategist',
    role: 'Business growth operator',
    category: 'Growth',
    summary: 'Finds high leverage revenue opportunities and turns them into concrete operating tasks.',
    strengths: ['positioning', 'offers', 'channel strategy', 'launch planning'],
    examples: ['Create a weekend flash sale plan', 'Find three low-cost growth loops', 'Turn slow sales into a recovery sprint'],
    usageCount: 428,
    model: 'MiniMax-M3',
    skills: ['Web Research', 'Campaign Builder', 'ROI Calculator'],
    connectors: ['Google Analytics', 'Stripe', 'Mailchimp'],
  },
  {
    id: 'customer-ambassador',
    kind: 'expert',
    name: 'Customer Ambassador',
    role: 'Customer success lead',
    category: 'Support',
    summary: 'Drafts replies, reads customer context, and escalates sensitive conversations for approval.',
    strengths: ['tone matching', 'retention', 'review recovery', 'support triage'],
    examples: ['Reply to unhappy customers', 'Draft review recovery messages', 'Summarize today inbox risk'],
    usageCount: 391,
    model: 'Auto',
    skills: ['Inbox Triage', 'Tone Rewriter', 'Policy Checker'],
    connectors: ['Chatwoot', 'Instagram DMs', 'QQ Mail'],
  },
  {
    id: 'finance-controller',
    kind: 'expert',
    name: 'Finance Controller',
    role: 'Finance controller',
    category: 'Finance',
    summary: 'Checks margins, cash flow, pricing, and spend before agents commit to expensive actions.',
    strengths: ['margin checks', 'cash flow', 'forecasting', 'cost controls'],
    examples: ['Audit promotion margin', 'Estimate cash impact', 'Flag overspend risk'],
    usageCount: 266,
    model: 'MiniMax-M3',
    skills: ['Ledger Query', 'Scenario Planner', 'Budget Guard'],
    connectors: ['Stripe', 'Square', 'QuickBooks'],
  },
  {
    id: 'operations-manager',
    kind: 'expert',
    name: 'Operations Manager',
    role: 'Operations',
    category: 'Operations',
    summary: 'Coordinates inventory, fulfillment, scheduling, and handoffs between business departments.',
    strengths: ['inventory', 'staffing', 'process design', 'handoffs'],
    examples: ['Plan order surge staffing', 'Create fulfillment checklist', 'Find bottlenecks'],
    usageCount: 344,
    model: 'Auto',
    skills: ['Inventory Audit', 'Task Planner', 'Vendor Follow-up'],
    connectors: ['Google Calendar', 'Shippo', 'Tencent Docs'],
  },
];

export const expertTeams: ExpertCatalogItem[] = [
  {
    id: 'launch-team',
    kind: 'team',
    name: 'Launch Team',
    role: 'Five-agent launch room',
    category: 'Expert Teams',
    summary: 'A lead agent coordinates research, offer, operations, finance, and quality review into one launch brief.',
    strengths: ['parallel execution', 'leader synthesis', 'quality gates', 'member handoff'],
    examples: ['Launch a new product', 'Build a store opening plan', 'Create a market entry brief'],
    usageCount: 812,
    model: 'MiniMax-M3',
    skills: ['Web Research', 'Campaign Builder', 'Scenario Planner', 'Task Planner'],
    connectors: ['Tencent Docs', 'Google Analytics', 'Stripe', 'Mailchimp'],
    members: ['Strategy Lead', 'Market Researcher', 'Offer Designer', 'Operations Planner', 'Quality Reviewer'],
  },
  {
    id: 'revenue-rescue-room',
    kind: 'team',
    name: 'Revenue Rescue Room',
    role: 'Sales recovery expert team',
    category: 'Expert Teams',
    summary: 'Specialists diagnose stalled demand, validate margin, draft outreach, and propose next actions.',
    strengths: ['diagnosis', 'promotion design', 'risk review', 'execution plan'],
    examples: ['Fix a bad sales week', 'Recover abandoned carts', 'Prioritize win-back offers'],
    usageCount: 574,
    model: 'Auto',
    skills: ['Cart Recovery', 'ROI Calculator', 'Tone Rewriter', 'Budget Guard'],
    connectors: ['Stripe', 'Instagram DMs', 'Mailchimp'],
    members: ['Revenue Strategist', 'Finance Controller', 'Customer Ambassador', 'Operations Manager', 'Risk Reviewer'],
  },
];

export const skillMarket: SkillItem[] = [
  { id: 'web-research', name: 'Web Research', category: 'Research', status: 'Installed', description: 'Collect current facts and citations for planning tasks.' },
  { id: 'campaign-builder', name: 'Campaign Builder', category: 'Marketing', status: 'Installed', description: 'Drafts posts, emails, landing page copy, and launch calendars.' },
  { id: 'roi-calculator', name: 'ROI Calculator', category: 'Finance', status: 'Recommended', description: 'Estimates payback, margin, and expected business impact.' },
  { id: 'tone-rewriter', name: 'Tone Rewriter', category: 'Customer Success', status: 'Installed', description: 'Rewrites drafts in the brand voice before approval.' },
  { id: 'policy-checker', name: 'Policy Checker', category: 'Risk', status: 'Disabled', description: 'Reviews regulated, refund, privacy, and brand safety language.' },
];

export const connectors: ConnectorItem[] = [
  { id: 'qq-mail', name: 'QQ Mail', status: 'Available', description: 'Read and draft business email replies.' },
  { id: 'tencent-docs', name: 'Tencent Docs', status: 'Connected', description: 'Create task briefs, spreadsheets, and shared output files.' },
  { id: 'tapd', name: 'TAPD', status: 'Needs setup', description: 'Sync tasks and delivery status with engineering workspaces.' },
  { id: 'weiyun', name: 'Weiyun', status: 'Available', description: 'Read shared folders and publish generated files.' },
  { id: 'stripe', name: 'Stripe', status: 'Connected', description: 'Query revenue, payments, refunds, and subscription signals.' },
];

export const exploreTemplates = [
  'Make my version of a launch plan',
  'Analyze my slowest product category',
  'Turn this screenshot into a campaign',
  'Build a recurring weekly operator report',
];

export const automations = [
  'Weekly business review',
  'Daily inbox risk scan',
  'Low inventory recovery plan',
  'Abandoned cart win-back draft',
];

export const remoteAssistants = ['Slack', 'Telegram', 'Discord', 'WeChat', 'QQ', 'DingTalk', 'Feishu'];

export const memories = [
  'Brand voice prefers practical, friendly copy.',
  'High risk outbound messages require approval.',
  'Weekend promos usually perform better than weekday promos.',
];
