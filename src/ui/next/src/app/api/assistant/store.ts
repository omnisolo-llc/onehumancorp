import { randomUUID } from 'node:crypto';

export type PermissionProfile = 'Guarded' | 'Full Access';
export type AssistantTaskStatus = 'running' | 'completed' | 'blocked' | 'failed' | 'archived';

export type AssistantArtifact = {
  id: string;
  type: 'document' | 'spreadsheet' | 'presentation' | 'chart' | 'pdf' | 'zip' | 'code';
  filename: string;
  mimeType: string;
  preview: string;
};

export type AssistantChange = {
  id: string;
  path: string;
  changeType: 'created' | 'modified' | 'deleted';
  summary: string;
  approvalStatus: 'pending' | 'approved' | 'not_required';
};

export type AssistantMessage = {
  id: string;
  role: 'user' | 'assistant' | 'tool';
  content: string;
  createdAt: string;
};

export type AssistantAction = {
  id: string;
  label: string;
  kind: 'control' | 'approval' | 'permission' | 'preview' | 'download' | 'execute' | 'share' | 'archive';
  approvalRequired: boolean;
};

export type AssistantTask = {
  id: string;
  title: string;
  prompt: string;
  workspace: string;
  status: AssistantTaskStatus;
  mode: string;
  model: string;
  provider: string;
  workDirectory: string;
  outputFormat: string;
  constraints: string;
  contextReferences: string;
  attachments: string[];
  skills: string[];
  connectors: string[];
  permissionProfile: PermissionProfile;
  currentStep: string;
  riskSummary: string[];
  artifacts: AssistantArtifact[];
  changes: AssistantChange[];
  messages: AssistantMessage[];
  actions: AssistantAction[];
  createdAt: string;
  updatedAt: string;
};

export type RemoteTask = {
  id: string;
  platform: string;
  userId: string;
  threadId: string;
  confirmationRequired: boolean;
  taskId: string;
};

export type Automation = {
  id: string;
  name: string;
  schedule: string;
  prompt: string;
  workspace: string;
  status: 'active' | 'paused';
  notificationChannel: string;
  permissionProfile: PermissionProfile;
  nextRunLabel: string;
};

export type MemoryItem = {
  id: string;
  scope: 'global' | 'workspace';
  content: string;
  source: 'seed' | 'import' | 'edit';
  editable: boolean;
};

export type SkillRecord = {
  id: string;
  name: string;
  category: string;
  status: 'installed' | 'disabled' | 'available';
};

export type ConnectorRecord = {
  id: string;
  name: string;
  kind: string;
  status: 'connected' | 'available' | 'needs_setup';
  features?: string[];
};

export type SharedFileRecord = {
  id: string;
  filename: string;
  workspace: string;
  access: 'shared' | 'queued_for_unshare';
};

export type ModelRecord = {
  id: string;
  provider: string;
  modelId: string;
  endpoint: string;
  enabled: boolean;
  headers: Record<string, string>;
  parameters: Record<string, string | number | boolean>;
  skipChatCompletions: boolean;
};

export type RuntimeRecord = {
  name: 'Node.js' | 'Python';
  status: 'detected' | 'needs_setup';
  installAction: string;
};

export type McpServerRecord = {
  id: string;
  name: string;
  url: string;
  status: 'connected' | 'needs_trust' | 'disabled';
  trusted: boolean;
  oauth: boolean;
  headers: Record<string, string>;
  features: string[];
  tools: { name: string; enabled: boolean }[];
};

export type ExpertRecord = {
  id: string;
  name: string;
  domain: string;
  description: string;
  ranking: number;
  visibility: 'public' | 'private' | 'internal';
};

export type WorkspaceRecord = {
  id: string;
  name: string;
  collapsed: boolean;
  pinned: boolean;
  archived: boolean;
  sortOrder: number;
  memoryFile: 'MEMORY.md';
};

export type ShareRecord = {
  id: string;
  taskId: string;
  artifactId: string;
  target: string;
  status: 'pending_review' | 'shared';
  previewUrl: string;
  audit: string[];
};

export type UploadRecord = {
  id: string;
  platform: string;
  userId: string;
  filename: string;
  mimeType: string;
  sizeBytes: number;
  previewText: string;
  status: 'available' | 'attached';
  previewUrl: string;
};

export type PreviewRecord = {
  id: string;
  taskId: string;
  artifactId: string;
  filename: string;
  autoRefresh: boolean;
  displayMode: 'inline' | 'fullscreen' | 'external';
  renderedAt: string;
};

export const assistantCapabilities = {
  resultTabs: ['Artifacts', 'All Files', 'Changes', 'Preview'],
  remotePlatforms: [
    'Slack',
    'Telegram',
    'Discord',
    'WeChat Work',
    'Feishu',
    'DingTalk',
    'QQ',
    'YuanbaoPai',
    'WeChat ClawBot',
  ],
  outputFormats: ['Document', 'Spreadsheet', 'Presentation', 'PDF', 'Chart', 'Code App', 'ZIP'],
  workModes: ['Ask', 'Craft', 'Plan', 'Coding'],
  permissionProfiles: ['Guarded', 'Full Access'],
  modelProviders: ['Auto', 'OpenAI', 'Anthropic', 'MiniMax', 'DeepSeek', 'Kimi', 'Local Ollama', 'Custom OpenAI Compatible'],
  sharingTargets: ['Share Link', 'WeChat', 'Slack', 'Download', 'Copy'],
  workspaceControls: ['Collapse All', 'Expand All', 'Hard Delete', 'Archive Cleanup'],
  commandSurfaces: ['/skill', '/compact', '/summarize', '/clear'],
  mcpFeatures: ['Tool Progress', 'Resources', 'Static Headers', 'Connector Try It'],
} as const;

export type CreateTaskPayload = {
  prompt?: string;
  workspace?: string;
  mode?: string;
  model?: string;
  provider?: string;
  workDirectory?: string;
  outputFormat?: string;
  constraints?: string;
  contextReferences?: string;
  attachments?: string[] | string;
  skills?: string[];
  connectors?: string[];
  permissionProfile?: PermissionProfile;
};

type NormalizedCreateTaskPayload = {
  prompt: string;
  workspace: string;
  mode: string;
  model: string;
  provider: string;
  workDirectory: string;
  outputFormat: string;
  constraints: string;
  contextReferences: string;
  attachments: string[];
  skills: string[];
  connectors: string[];
  permissionProfile: PermissionProfile;
};

let tasks: AssistantTask[] = [];
let remotes: RemoteTask[] = [];
let automations: Automation[] = [];
let memories: MemoryItem[] = [];
let skills: SkillRecord[] = [];
let connectors: ConnectorRecord[] = [];
let sharedFiles: SharedFileRecord[] = [];
let unshareQueue: SharedFileRecord[] = [];
let authorizedFolders: string[] = [];
let models: ModelRecord[] = [];
let runtimes: RuntimeRecord[] = [];
let mcpServers: McpServerRecord[] = [];
let mcpProgress: { id: string; serverId: string; tool: string; stage: 'queued' | 'running' | 'completed'; message: string }[] = [];
let experts: ExpertRecord[] = [];
let workspaces: WorkspaceRecord[] = [];
let deletedWorkspaces: WorkspaceRecord[] = [];
let shares: ShareRecord[] = [];
let uploads: UploadRecord[] = [];
let previews: PreviewRecord[] = [];

function now() {
  return new Date().toISOString();
}

function id(prefix: string) {
  return `${prefix}-${randomUUID()}`;
}

function slug(value: string) {
  return value.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '') || 'item';
}

function normalizeAttachments(raw: CreateTaskPayload['attachments']): string[] {
  if (Array.isArray(raw)) return raw.filter(Boolean);
  if (typeof raw === 'string') {
    return raw.split(',').map((part) => part.trim()).filter(Boolean);
  }
  return [];
}

function titleFromPrompt(prompt: string) {
  return prompt.trim().slice(0, 96);
}

function artifactForFormat(outputFormat: string): AssistantArtifact {
  const normalized = outputFormat.toLowerCase();
  if (normalized.includes('code') || normalized.includes('app')) {
    return {
      id: id('artifact'),
      type: 'code',
      filename: 'app/index.html',
      mimeType: 'text/html',
      preview: 'Runnable local app source generated by Jarvis.',
    };
  }
  if (normalized.includes('presentation') || normalized.includes('ppt')) {
    return {
      id: id('artifact'),
      type: 'presentation',
      filename: 'assistant-presentation.pptx',
      mimeType: 'application/vnd.openxmlformats-officedocument.presentationml.presentation',
      preview: 'Slide deck outline and generated speaker notes.',
    };
  }
  if (normalized.includes('spreadsheet') || normalized.includes('table')) {
    return {
      id: id('artifact'),
      type: 'spreadsheet',
      filename: 'assistant-spreadsheet.xlsx',
      mimeType: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
      preview: 'Workbook with summary, data, and chart tabs.',
    };
  }
  if (normalized.includes('pdf')) {
    return {
      id: id('artifact'),
      type: 'pdf',
      filename: 'assistant-report.pdf',
      mimeType: 'application/pdf',
      preview: 'PDF export ready for review.',
    };
  }
  if (normalized.includes('zip')) {
    return {
      id: id('artifact'),
      type: 'zip',
      filename: 'assistant-output.zip',
      mimeType: 'application/zip',
      preview: 'Bundled multi-file output ready for download.',
    };
  }
  if (normalized.includes('chart')) {
    return chartArtifact();
  }
  return {
    id: id('artifact'),
    type: 'document',
    filename: 'assistant-brief.md',
    mimeType: 'text/markdown',
    preview: 'Draft report ready for review.',
  };
}

function appPreviewArtifact(): AssistantArtifact {
  return {
    id: id('artifact'),
    type: 'document',
    filename: 'app-preview.html',
    mimeType: 'text/html',
    preview: 'Built-in browser preview for the generated local app.',
  };
}

function chartArtifact(): AssistantArtifact {
  return {
    id: id('artifact'),
    type: 'chart',
    filename: 'assistant-chart.png',
    mimeType: 'image/png',
    preview: 'Generated chart preview.',
  };
}

function artifactExportFor(format: string, title: string): AssistantArtifact {
  const normalized = format.toLowerCase();
  if (normalized.includes('spreadsheet')) {
    return {
      id: id('artifact'),
      type: 'spreadsheet',
      filename: `${title.toLowerCase().replace(/[^a-z0-9]+/g, '-')}.xlsx`,
      mimeType: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
      preview: `Spreadsheet export for ${title}.`,
    };
  }
  if (normalized.includes('presentation')) {
    return {
      id: id('artifact'),
      type: 'presentation',
      filename: `${title.toLowerCase().replace(/[^a-z0-9]+/g, '-')}.pptx`,
      mimeType: 'application/vnd.openxmlformats-officedocument.presentationml.presentation',
      preview: `Presentation export for ${title}.`,
    };
  }
  if (normalized.includes('pdf')) {
    return {
      id: id('artifact'),
      type: 'pdf',
      filename: `${title.toLowerCase().replace(/[^a-z0-9]+/g, '-')}.pdf`,
      mimeType: 'application/pdf',
      preview: `PDF export for ${title}.`,
    };
  }
  if (normalized.includes('zip')) {
    return {
      id: id('artifact'),
      type: 'zip',
      filename: `${title.toLowerCase().replace(/[^a-z0-9]+/g, '-')}.zip`,
      mimeType: 'application/zip',
      preview: `ZIP export for ${title}.`,
    };
  }
  return {
    id: id('artifact'),
    type: 'document',
    filename: `${title.toLowerCase().replace(/[^a-z0-9]+/g, '-')}.docx`,
    mimeType: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
    preview: `Document export for ${title}.`,
  };
}

function buildRiskSummary(payload: NormalizedCreateTaskPayload) {
  const risks = ['Guarded mode is active'];
  if (payload.connectors.length > 0) risks.push('External sends require approval');
  if (payload.workDirectory) risks.push(`File writes are limited to ${payload.workDirectory}`);
  if (payload.permissionProfile === 'Full Access') risks.push('Full Access still logs risky actions');
  return risks;
}

function actionsForTask(outputFormat: string, permissionProfile: PermissionProfile): AssistantAction[] {
  const normalized = outputFormat.toLowerCase();
  const actions: AssistantAction[] = [
    { id: id('action'), label: 'Stop', kind: 'control', approvalRequired: false },
    { id: id('action'), label: 'Retry', kind: 'control', approvalRequired: false },
    { id: id('action'), label: 'Approve Changes', kind: 'approval', approvalRequired: permissionProfile === 'Guarded' },
    { id: id('action'), label: 'Download File', kind: 'download', approvalRequired: false },
    { id: id('action'), label: 'Archive Task', kind: 'archive', approvalRequired: false },
  ];
  if (normalized.includes('code') || normalized.includes('app')) {
    actions.push(
      { id: id('action'), label: 'Open Preview', kind: 'preview', approvalRequired: false },
      { id: id('action'), label: 'Run Locally', kind: 'execute', approvalRequired: true },
    );
  }
  return actions;
}

export function createAssistantTask(payload: CreateTaskPayload): AssistantTask {
  const prompt = payload.prompt?.trim();
  if (!prompt) {
    throw new Error('prompt is required');
  }

  const normalized: NormalizedCreateTaskPayload = {
    prompt,
    workspace: payload.workspace || 'Personal OS',
    mode: payload.mode || 'Ask',
    model: payload.model || 'Auto',
    provider: payload.provider || 'Auto',
    workDirectory: payload.workDirectory || '/workspace/assistant',
    outputFormat: payload.outputFormat || 'Document',
    constraints: payload.constraints || '',
    contextReferences: payload.contextReferences || '',
    attachments: normalizeAttachments(payload.attachments),
    skills: payload.skills || [],
    connectors: payload.connectors || [],
    permissionProfile: payload.permissionProfile || 'Guarded',
  };

  const createdAt = now();
  const primaryArtifact = artifactForFormat(normalized.outputFormat);
  const artifacts = [primaryArtifact, chartArtifact()];
  if (normalized.outputFormat.toLowerCase().includes('code') || normalized.outputFormat.toLowerCase().includes('app')) {
    artifacts.push(appPreviewArtifact());
  }
  const task: AssistantTask = {
    id: id('task'),
    title: titleFromPrompt(prompt),
    prompt,
    workspace: normalized.workspace,
    status: 'running',
    mode: normalized.mode,
    model: normalized.model,
    provider: normalized.provider,
    workDirectory: normalized.workDirectory,
    outputFormat: normalized.outputFormat,
    constraints: normalized.constraints,
    contextReferences: normalized.contextReferences,
    attachments: normalized.attachments,
    skills: normalized.skills,
    connectors: normalized.connectors,
    permissionProfile: normalized.permissionProfile,
    currentStep: 'Planning and preparing tools',
    riskSummary: buildRiskSummary(normalized),
    artifacts,
    changes: [
      {
        id: id('change'),
        path: `${normalized.workDirectory.replace(/\/$/, '')}/${primaryArtifact.filename}`,
        changeType: 'created',
        summary: 'Generated output file will be written after approval if required.',
        approvalStatus: normalized.permissionProfile === 'Guarded' ? 'pending' : 'not_required',
      },
    ],
    messages: [
      { id: id('msg'), role: 'user', content: prompt, createdAt },
      {
        id: id('msg'),
        role: 'assistant',
        content: `Jarvis planned the task with ${normalized.skills.length || 'default'} skills and ${normalized.connectors.length || 'no'} connectors.`,
        createdAt,
      },
    ],
    actions: actionsForTask(normalized.outputFormat, normalized.permissionProfile),
    createdAt,
    updatedAt: createdAt,
  };

  tasks.unshift(task);
  return task;
}

export function listAssistantTasks() {
  return tasks;
}

export function getAssistantCapabilities() {
  return assistantCapabilities;
}

export function mutateTask(taskId: string, action: string) {
  const task = tasks.find((item) => item.id === taskId);
  if (!task) throw new Error('task not found');
  if (action === 'approve_changes') {
    task.changes = task.changes.map((change) => ({ ...change, approvalStatus: 'approved' }));
    task.messages.push({ id: id('msg'), role: 'assistant', content: 'Changes approved and ready to apply.', createdAt: now() });
  } else if (action === 'stop') {
    task.status = 'blocked';
    task.currentStep = 'Stopped by user';
  } else if (action === 'resume') {
    task.status = 'running';
    task.currentStep = 'Resumed and preparing next step';
  } else if (action === 'archive') {
    task.status = 'archived';
    task.currentStep = 'Archived';
  } else {
    throw new Error('unsupported task action');
  }
  task.updatedAt = now();
  return task;
}

export function createRemoteTask(payload: {
  platform?: string;
  userId?: string;
  threadId?: string;
  message?: string;
  attachments?: string[];
  uploadIds?: string[];
}) {
  if (!payload.message?.trim()) {
    throw new Error('message is required');
  }
  const uploadedAttachments = (payload.uploadIds || [])
    .map((uploadId) => uploads.find((upload) => upload.id === uploadId))
    .filter((upload): upload is UploadRecord => Boolean(upload))
    .map((upload) => {
      upload.status = 'attached';
      return upload.filename;
    });
  const task = createAssistantTask({
    prompt: payload.message,
    workspace: 'Remote Control',
    mode: 'Plan',
    outputFormat: 'Document',
    attachments: [...(payload.attachments || []), ...uploadedAttachments],
    connectors: [payload.platform || 'Remote'],
    permissionProfile: 'Guarded',
  });
  const confirmationRequired = /\b(send|delete|overwrite|convert all|post|share)\b/i.test(payload.message);
  const remote: RemoteTask = {
    id: id('remote'),
    platform: assistantCapabilities.remotePlatforms.includes(payload.platform as any) ? payload.platform || 'Slack' : 'Slack',
    userId: payload.userId || 'unknown',
    threadId: payload.threadId || task.id,
    confirmationRequired,
    taskId: task.id,
  };
  remotes.unshift(remote);
  return { remote, task, reply: `Jarvis started "${task.title}" from ${remote.platform}.` };
}

export function listRemotePlatforms() {
  return {
    platforms: assistantCapabilities.remotePlatforms.map((name) => ({
      name,
      status: 'available',
      command: `/jarvis ${name.toLowerCase().replace(/\s+/g, '-')}`,
    })),
    remotes,
  };
}

export function createAutomation(payload: {
  name?: string;
  schedule?: string;
  prompt?: string;
  workspace?: string;
  notificationChannel?: string;
  permissionProfile?: PermissionProfile;
}) {
  if (!payload.name?.trim()) throw new Error('name is required');
  if (!payload.prompt?.trim()) throw new Error('prompt is required');
  const automation: Automation = {
    id: id('automation'),
    name: payload.name.trim(),
    schedule: payload.schedule || 'Daily 09:00',
    prompt: payload.prompt.trim(),
    workspace: payload.workspace || 'Personal OS',
    status: 'active',
    notificationChannel: payload.notificationChannel || 'In app',
    permissionProfile: payload.permissionProfile || 'Guarded',
    nextRunLabel: `Next run follows: ${payload.schedule || 'Daily 09:00'}`,
  };
  automations.unshift(automation);
  return automation;
}

export function listAutomations() {
  return automations;
}

export function listMemories() {
  return memories;
}

export function mutateMemory(payload: {
  action?: 'import' | 'edit' | 'forget';
  id?: string;
  content?: string;
  scope?: 'global' | 'workspace';
}) {
  if (payload.action === 'import') {
    if (!payload.content?.trim()) throw new Error('content is required');
    memories.unshift({
      id: id('memory'),
      scope: payload.scope || 'workspace',
      content: payload.content.trim(),
      source: 'import',
      editable: true,
    });
  } else if (payload.action === 'edit') {
    const item = memories.find((memory) => memory.id === payload.id);
    if (!item) throw new Error('memory not found');
    if (!payload.content?.trim()) throw new Error('content is required');
    item.content = payload.content.trim();
    item.source = 'edit';
  } else if (payload.action === 'forget') {
    memories = memories.filter((memory) => memory.id !== payload.id);
  } else {
    throw new Error('unsupported memory action');
  }
  return memories;
}

export function listSkills() {
  return skills;
}

export function mutateSkill(payload: {
  action?: 'install' | 'disable' | 'uninstall';
  name?: string;
  category?: string;
}) {
  if (!payload.name?.trim()) throw new Error('skill name is required');
  const name = payload.name.trim();
  if (payload.action === 'install') {
    const existing = skills.find((skill) => skill.name === name);
    if (existing) {
      existing.status = 'installed';
      existing.category = payload.category || existing.category;
    } else {
      skills.unshift({ id: id('skill'), name, category: payload.category || 'General', status: 'installed' });
    }
  } else if (payload.action === 'disable') {
    const existing = skills.find((skill) => skill.name === name);
    if (!existing) throw new Error('skill not found');
    existing.status = 'disabled';
  } else if (payload.action === 'uninstall') {
    skills = skills.filter((skill) => skill.name !== name);
  } else {
    throw new Error('unsupported skill action');
  }
  return skills;
}

export function listConnectors() {
  return connectors;
}

export function mutateConnector(payload: {
  action?: 'connect' | 'disconnect';
  name?: string;
  kind?: string;
}) {
  if (!payload.name?.trim()) throw new Error('connector name is required');
  const name = payload.name.trim();
  if (payload.action === 'connect') {
    const existing = connectors.find((connector) => connector.name === name);
    if (existing) {
      existing.status = 'connected';
      existing.kind = payload.kind || existing.kind;
    } else {
      connectors.unshift({ id: id('connector'), name, kind: payload.kind || 'custom', status: 'connected' });
    }
  } else if (payload.action === 'disconnect') {
    const existing = connectors.find((connector) => connector.name === name);
    if (!existing) throw new Error('connector not found');
    existing.status = 'available';
  } else {
    throw new Error('unsupported connector action');
  }
  return connectors;
}

export function getDataManagement() {
  return {
    sharedFiles,
    archivedTasks: tasks.filter((task) => task.status === 'archived'),
    unshareQueue,
  };
}

export function mutateDataManagement(payload: {
  action?: 'unshare' | 'archive_cleanup';
  id?: string;
}) {
  if (payload.action === 'unshare') {
    const file = sharedFiles.find((item) => item.id === payload.id);
    if (!file) throw new Error('shared file not found');
    file.access = 'queued_for_unshare';
    if (!unshareQueue.some((item) => item.id === file.id)) {
      unshareQueue.unshift(file);
    }
  } else if (payload.action === 'archive_cleanup') {
    tasks = tasks.filter((task) => task.status !== 'archived');
  } else {
    throw new Error('unsupported data action');
  }
  return getDataManagement();
}

export function createExportArtifact(payload: {
  taskId?: string;
  outputFormat?: string;
  title?: string;
}) {
  const task = tasks.find((item) => item.id === payload.taskId);
  if (!task) throw new Error('task not found');
  const artifact = artifactExportFor(payload.outputFormat || 'Document', payload.title || task.title);
  task.artifacts.unshift(artifact);
  task.updatedAt = now();
  return artifact;
}

export function getPermissions() {
  return {
    permissionProfile: 'Guarded',
    authorizedFolders,
    rules: [
      'Reads are limited to authorized folders and uploaded attachments.',
      'Writes outside task output folders require approval.',
      'External sends and destructive actions require confirmation.',
    ],
  };
}

export function mutatePermissions(payload: {
  action?: 'grant' | 'revoke';
  folder?: string;
}) {
  if (!payload.folder?.trim()) throw new Error('folder is required');
  const folder = payload.folder.trim();
  if (payload.action === 'grant') {
    if (!authorizedFolders.includes(folder)) authorizedFolders.push(folder);
  } else if (payload.action === 'revoke') {
    authorizedFolders = authorizedFolders.filter((item) => item !== folder);
  } else {
    throw new Error('unsupported permission action');
  }
  return getPermissions();
}

export function planFileOperation(payload: {
  operation?: string;
  folder?: string;
  sourcePattern?: string;
  targetFormat?: string;
}) {
  if (!payload.operation?.trim()) throw new Error('operation is required');
  if (!payload.folder?.trim()) throw new Error('folder is required');
  const folder = payload.folder.trim();
  const hasPermission = authorizedFolders.includes(folder);
  return {
    id: id('fileop'),
    operation: payload.operation,
    folder,
    sourcePattern: payload.sourcePattern || '*',
    targetFormat: payload.targetFormat || 'original',
    status: hasPermission ? 'ready_for_approval' : 'needs_permission',
    approvalRequired: true,
    plan: [
      `Read matching files from ${folder} using ${payload.sourcePattern || '*'}.`,
      `Write converted files as ${payload.targetFormat || 'requested format'}.`,
      'Register outputs as task artifacts before sharing or download.',
    ],
  };
}

export function getModelSettings() {
  return { models, runtime: runtimes };
}

export function mutateModelSettings(payload: {
  action?: 'upsert' | 'disable' | 'delete';
  provider?: string;
  modelId?: string;
  endpoint?: string;
  headers?: Record<string, string>;
  parameters?: Record<string, string | number | boolean>;
  skipChatCompletions?: boolean;
}) {
  if (!payload.provider?.trim()) throw new Error('provider is required');
  const provider = payload.provider.trim();
  if (payload.action === 'upsert') {
    const existing = models.find((model) => model.provider === provider && model.modelId === (payload.modelId || provider));
    const nextModel: ModelRecord = {
      id: existing?.id || id('model'),
      provider,
      modelId: payload.modelId || provider,
      endpoint: payload.endpoint || existing?.endpoint || '',
      enabled: true,
      headers: payload.headers || existing?.headers || {},
      parameters: payload.parameters || existing?.parameters || {},
      skipChatCompletions: Boolean(payload.skipChatCompletions ?? existing?.skipChatCompletions),
    };
    if (existing) {
      Object.assign(existing, nextModel);
    } else {
      models.unshift(nextModel);
    }
  } else if (payload.action === 'disable') {
    const existing = models.find((model) => model.provider === provider);
    if (!existing) throw new Error('model not found');
    existing.enabled = false;
  } else if (payload.action === 'delete') {
    models = models.filter((model) => model.provider !== provider);
  } else {
    throw new Error('unsupported model action');
  }
  return getModelSettings();
}

export function listMcpServers() {
  return {
    servers: mcpServers,
    progress: mcpProgress,
    resources: mcpServers.flatMap((server) =>
      server.tools.map((tool) => ({ uri: `mcp://${slug(server.name)}/resources/${tool.name}`, serverId: server.id, name: tool.name })),
    ),
  };
}

export function createMcpServer(payload: {
  name?: string;
  url?: string;
  headers?: Record<string, string>;
  oauth?: boolean;
}) {
  if (!payload.name?.trim()) throw new Error('name is required');
  if (!payload.url?.trim()) throw new Error('url is required');
  const server: McpServerRecord = {
    id: id('mcp'),
    name: payload.name.trim(),
    url: payload.url.trim(),
    status: 'needs_trust',
    trusted: false,
    oauth: Boolean(payload.oauth),
    headers: payload.headers || {},
    features: [...assistantCapabilities.mcpFeatures],
    tools: [
      { name: 'search_issues', enabled: true },
      { name: 'read_resource', enabled: true },
    ],
  };
  mcpServers.unshift(server);
  return server;
}

export function mutateMcpServer(payload: {
  action?: 'trust' | 'disable_tool' | 'try_tool' | 'disconnect';
  id?: string;
  name?: string;
  tool?: string;
}) {
  const server = mcpServers.find((item) => item.id === payload.id || item.name === payload.name);
  if (!server) throw new Error('mcp server not found');
  if (payload.action === 'trust') {
    server.trusted = true;
    server.status = 'connected';
  } else if (payload.action === 'disable_tool') {
    const tool = server.tools.find((item) => item.name === payload.tool);
    if (!tool) throw new Error('tool not found');
    tool.enabled = false;
  } else if (payload.action === 'disconnect') {
    server.status = 'disabled';
  } else if (payload.action === 'try_tool') {
    const toolName = payload.tool || server.tools[0]?.name || 'tool';
    mcpProgress = [
      { id: id('mcpprogress'), serverId: server.id, tool: toolName, stage: 'queued', message: 'Tool call queued.' },
      { id: id('mcpprogress'), serverId: server.id, tool: toolName, stage: 'running', message: 'Tool call running.' },
      { id: id('mcpprogress'), serverId: server.id, tool: toolName, stage: 'completed', message: 'Tool call completed.' },
    ];
  } else {
    throw new Error('unsupported mcp action');
  }
  return listMcpServers();
}

export function listExperts() {
  return {
    experts: [...experts].sort((left, right) => left.ranking - right.ranking),
    recommendedPrompts: experts.slice(0, 3).map((expert) => `Ask ${expert.name} to review this task.`),
  };
}

export function createExpert(payload: {
  name?: string;
  domain?: string;
  description?: string;
  visibility?: ExpertRecord['visibility'];
}) {
  if (!payload.name?.trim()) throw new Error('expert name is required');
  const expert: ExpertRecord = {
    id: id('expert'),
    name: payload.name.trim(),
    domain: payload.domain || 'General',
    description: payload.description || 'Custom Jarvis expert.',
    ranking: experts.length + 1,
    visibility: payload.visibility || 'private',
  };
  experts.push(expert);
  return expert;
}

export function mutateExpert(payload: {
  action?: 'summon' | 'visibility';
  id?: string;
  taskId?: string;
  visibility?: ExpertRecord['visibility'];
}) {
  const expert = experts.find((item) => item.id === payload.id);
  if (!expert) throw new Error('expert not found');
  if (payload.action === 'summon') {
    const task = tasks.find((item) => item.id === payload.taskId);
    if (!task) throw new Error('task not found');
    task.messages.push({
      id: id('msg'),
      role: 'assistant',
      content: `${expert.name} joined the task with ${expert.domain} guidance.`,
      createdAt: now(),
    });
    return { expert, task, experts: listExperts().experts };
  }
  if (payload.action === 'visibility') {
    expert.visibility = payload.visibility || expert.visibility;
    return { expert, experts: listExperts().experts };
  }
  throw new Error('unsupported expert action');
}

export function listCommands() {
  return {
    commands: [
      { command: '/skill', description: 'Open the skill list and select a skill.' },
      { command: '/compact', description: 'Compress long context while preserving key decisions.' },
      { command: '/summarize', description: 'Summarize the current task.' },
      { command: '/clear', description: 'Clear task context after confirmation.' },
    ],
  };
}

export function runCommand(payload: {
  command?: string;
  taskId?: string;
}) {
  const command = payload.command?.trim();
  if (!command) throw new Error('command is required');
  const task = tasks.find((item) => item.id === payload.taskId);
  if (!task) throw new Error('task not found');
  if (command === '/clear') {
    task.messages = [{ id: id('msg'), role: 'assistant', content: 'Context cleared for this task.', createdAt: now() }];
  } else if (command === '/summarize') {
    task.messages.push({ id: id('msg'), role: 'assistant', content: `Summary: ${task.title} is ${task.status} at "${task.currentStep}".`, createdAt: now() });
  } else if (command === '/compact') {
    task.messages.push({ id: id('msg'), role: 'assistant', content: 'Context compacted while preserving decisions, files, and approvals.', createdAt: now() });
  } else if (command === '/skill') {
    task.messages.push({ id: id('msg'), role: 'assistant', content: `Skill list opened with ${skills.length} available skills.`, createdAt: now() });
  } else {
    throw new Error('unsupported command');
  }
  task.updatedAt = now();
  return { result: { command, status: 'completed' }, task };
}

export function listWorkspaces() {
  return { workspaces: [...workspaces].sort((left, right) => Number(right.pinned) - Number(left.pinned) || left.sortOrder - right.sortOrder), deleted: deletedWorkspaces };
}

export function mutateWorkspace(payload: {
  action?: 'collapse_all' | 'expand_all' | 'pin' | 'archive' | 'hard_delete' | 'sort';
  name?: string;
  confirm?: string;
  order?: string[];
}) {
  if (payload.action === 'collapse_all') {
    workspaces.forEach((workspace) => { workspace.collapsed = true; });
  } else if (payload.action === 'expand_all') {
    workspaces.forEach((workspace) => { workspace.collapsed = false; });
  } else if (payload.action === 'pin') {
    const workspace = workspaces.find((item) => item.name === payload.name);
    if (!workspace) throw new Error('workspace not found');
    workspace.pinned = true;
  } else if (payload.action === 'archive') {
    const workspace = workspaces.find((item) => item.name === payload.name);
    if (!workspace) throw new Error('workspace not found');
    workspace.archived = true;
  } else if (payload.action === 'hard_delete') {
    if (payload.confirm !== 'DELETE') throw new Error('hard delete requires confirmation');
    const workspace = workspaces.find((item) => item.name === payload.name);
    if (!workspace) throw new Error('workspace not found');
    deletedWorkspaces.unshift(workspace);
    workspaces = workspaces.filter((item) => item.id !== workspace.id);
    tasks = tasks.filter((task) => task.workspace !== workspace.name);
  } else if (payload.action === 'sort') {
    (payload.order || []).forEach((name, index) => {
      const workspace = workspaces.find((item) => item.name === name);
      if (workspace) workspace.sortOrder = index;
    });
  } else {
    throw new Error('unsupported workspace action');
  }
  return listWorkspaces();
}

export function listShares() {
  return { shares };
}

export function createShare(payload: {
  taskId?: string;
  artifactId?: string;
  target?: string;
}) {
  const task = tasks.find((item) => item.id === payload.taskId);
  if (!task) throw new Error('task not found');
  const artifact = task.artifacts.find((item) => item.id === payload.artifactId);
  if (!artifact) throw new Error('artifact not found');
  const target = payload.target || 'Share Link';
  const share: ShareRecord = {
    id: id('share'),
    taskId: task.id,
    artifactId: artifact.id,
    target,
    status: target === 'Download' || target === 'Copy' ? 'shared' : 'pending_review',
    previewUrl: `/assistant/preview/${artifact.id}`,
    audit: [
      `Created ${target} sharing review for ${artifact.filename}.`,
      'External sends require guarded confirmation.',
    ],
  };
  shares.unshift(share);
  return share;
}

export function listUploads() {
  return { uploads };
}

export function createUpload(payload: {
  platform?: string;
  userId?: string;
  filename?: string;
  mimeType?: string;
  sizeBytes?: number;
  previewText?: string;
}) {
  if (!payload.filename?.trim()) throw new Error('filename is required');
  const upload: UploadRecord = {
    id: id('upload'),
    platform: payload.platform || 'Remote',
    userId: payload.userId || 'unknown',
    filename: payload.filename.trim(),
    mimeType: payload.mimeType || 'application/octet-stream',
    sizeBytes: payload.sizeBytes || 0,
    previewText: payload.previewText || '',
    status: 'available',
    previewUrl: `/assistant/uploads/${slug(payload.filename)}`,
  };
  uploads.unshift(upload);
  return upload;
}

export function listPreviews() {
  return { previews };
}

export function mutatePreview(payload: {
  action?: 'refresh' | 'fullscreen' | 'open_external';
  artifactId?: string;
}) {
  const preview = previews.find((item) => item.artifactId === payload.artifactId);
  if (!preview) throw new Error('preview not found');
  if (payload.action === 'refresh') {
    preview.renderedAt = now();
  } else if (payload.action === 'fullscreen') {
    preview.displayMode = 'fullscreen';
    preview.renderedAt = now();
  } else if (payload.action === 'open_external') {
    preview.displayMode = 'external';
    preview.renderedAt = now();
  } else {
    throw new Error('unsupported preview action');
  }
  return preview;
}

export function resetAssistantStore() {
  tasks = [
    {
      id: 'task-weekly-brief',
      title: 'Create this week\'s operating brief',
      prompt: 'Summarize this week and prepare a brief.',
      workspace: 'Personal OS',
      status: 'running',
      mode: 'Plan',
      model: 'Auto',
      provider: 'Auto',
      workDirectory: '/workspace/reports',
      outputFormat: 'Document',
      constraints: 'Keep it concise.',
      contextReferences: '@calendar @notes',
      attachments: [],
      skills: ['Web Research', 'Document Writer'],
      connectors: ['Google Drive'],
      permissionProfile: 'Guarded',
      currentStep: 'Drafting report',
      riskSummary: ['Guarded mode is active', 'External sends require approval'],
      artifacts: [
        {
          id: 'artifact-weekly-brief',
          type: 'document',
          filename: 'weekly-brief.md',
          mimeType: 'text/markdown',
          preview: 'Weekly brief draft with action items.',
        },
      ],
      changes: [
        {
          id: 'change-weekly-brief',
          path: '/workspace/reports/weekly-brief.md',
          changeType: 'created',
          summary: 'Creates a markdown brief.',
          approvalStatus: 'pending',
        },
      ],
      messages: [
        {
          id: 'msg-weekly-user',
          role: 'user',
          content: 'Summarize this week and prepare a brief.',
          createdAt: '2026-06-07T00:00:00.000Z',
        },
        {
          id: 'msg-weekly-assistant',
          role: 'assistant',
          content: 'I am gathering context and drafting the brief.',
          createdAt: '2026-06-07T00:01:00.000Z',
        },
      ],
      actions: [
        { id: 'action-seed-stop', label: 'Stop', kind: 'control', approvalRequired: false },
        { id: 'action-seed-approve', label: 'Approve Changes', kind: 'approval', approvalRequired: true },
        { id: 'action-seed-download', label: 'Download File', kind: 'download', approvalRequired: false },
      ],
      createdAt: '2026-06-07T00:00:00.000Z',
      updatedAt: '2026-06-07T00:01:00.000Z',
    },
    {
      id: 'task-downloads-cleanup',
      title: 'Organize Downloads by file type',
      prompt: 'Organize Downloads by file type.',
      workspace: 'Files',
      status: 'blocked',
      mode: 'Craft',
      model: 'MiniMax-M3',
      provider: 'Auto',
      workDirectory: '/Users/me/Downloads',
      outputFormat: 'Table',
      constraints: 'Ask before moving files.',
      contextReferences: '@Downloads',
      attachments: [],
      skills: ['File Organizer'],
      connectors: [],
      permissionProfile: 'Guarded',
      currentStep: 'Waiting for folder permission',
      riskSummary: ['Needs permission for Downloads'],
      artifacts: [],
      changes: [],
      messages: [
        {
          id: 'msg-downloads-assistant',
          role: 'assistant',
          content: 'I need permission to read Downloads before continuing.',
          createdAt: '2026-06-07T00:03:00.000Z',
        },
      ],
      actions: [
        { id: 'action-seed-grant', label: 'Grant Folder Access', kind: 'permission', approvalRequired: true },
      ],
      createdAt: '2026-06-07T00:02:00.000Z',
      updatedAt: '2026-06-07T00:03:00.000Z',
    },
  ];
  remotes = [];
  automations = [];
  memories = [
    {
      id: 'memory-concise-citations',
      scope: 'global',
      content: 'Prefer concise technical summaries with citations.',
      source: 'seed',
      editable: true,
    },
    {
      id: 'memory-approval',
      scope: 'workspace',
      content: 'Ask before sending messages or modifying original files.',
      source: 'seed',
      editable: true,
    },
  ];
  skills = [
    { id: 'skill-web-research', name: 'Web Research', category: 'Research', status: 'installed' },
    { id: 'skill-document-writer', name: 'Document Writer', category: 'Artifacts', status: 'installed' },
    { id: 'skill-chart-builder', name: 'Chart Builder', category: 'Data', status: 'installed' },
    { id: 'skill-slash-command-runner', name: 'Slash Command Runner', category: 'Commands', status: 'installed' },
    { id: 'skill-expert-ranking', name: 'Expert Ranking', category: 'Expert Center', status: 'available' },
    { id: 'skill-custom-expert-builder', name: 'Custom Expert Builder', category: 'Expert Center', status: 'available' },
  ];
  connectors = [
    { id: 'connector-google-drive', name: 'Google Drive', kind: 'files', status: 'connected' },
    { id: 'connector-slack', name: 'Slack', kind: 'remote', status: 'available' },
    { id: 'connector-mcp', name: 'MCP Endpoint', kind: 'tools', status: 'available', features: [...assistantCapabilities.mcpFeatures] },
    { id: 'connector-tencent-docs', name: 'Tencent Docs', kind: 'office', status: 'available' },
    { id: 'connector-tencent-meeting', name: 'Tencent Meeting', kind: 'office', status: 'available' },
    { id: 'connector-wecom-docs', name: 'WeCom Docs', kind: 'office', status: 'available' },
    { id: 'connector-qq-mail', name: 'QQ Mail', kind: 'office', status: 'available' },
  ];
  sharedFiles = [
    { id: 'shared-weekly-brief', filename: 'weekly-brief.md', workspace: 'Personal OS', access: 'shared' },
    { id: 'shared-chart', filename: 'assistant-chart.png', workspace: 'Personal OS', access: 'shared' },
  ];
  unshareQueue = [];
  authorizedFolders = ['/workspace/assistant', '/workspace/reports'];
  models = [
    {
      id: 'model-auto',
      provider: 'Auto',
      modelId: 'auto',
      endpoint: 'jarvis://auto',
      enabled: true,
      headers: {},
      parameters: {},
      skipChatCompletions: false,
    },
    {
      id: 'model-openai',
      provider: 'OpenAI',
      modelId: 'gpt-4.1',
      endpoint: 'https://api.openai.com/v1',
      enabled: true,
      headers: {},
      parameters: { temperature: 0.3 },
      skipChatCompletions: false,
    },
    {
      id: 'model-local-ollama',
      provider: 'Local Ollama',
      modelId: 'llama3.1',
      endpoint: 'http://localhost:11434',
      enabled: false,
      headers: {},
      parameters: {},
      skipChatCompletions: true,
    },
  ];
  runtimes = [
    { name: 'Node.js', status: 'detected', installAction: 'Use current runtime' },
    { name: 'Python', status: 'needs_setup', installAction: 'Install Python tool runtime' },
  ];
  mcpServers = [
    {
      id: 'mcp-default',
      name: 'MCP Endpoint',
      url: 'mcp://default',
      status: 'needs_trust',
      trusted: false,
      oauth: false,
      headers: { 'User-Agent': 'Jarvis-Assistant' },
      features: [...assistantCapabilities.mcpFeatures],
      tools: [
        { name: 'read_resource', enabled: true },
        { name: 'run_tool', enabled: true },
      ],
    },
  ];
  mcpProgress = [];
  experts = [
    {
      id: 'expert-research-strategist',
      name: 'Research Strategist',
      domain: 'Research',
      description: 'Frames research plans, source quality, and synthesis.',
      ranking: 1,
      visibility: 'public',
    },
    {
      id: 'expert-operations-analyst',
      name: 'Operations Analyst',
      domain: 'Operations',
      description: 'Turns messy process context into operating plans.',
      ranking: 2,
      visibility: 'public',
    },
    {
      id: 'expert-document-editor',
      name: 'Document Editor',
      domain: 'Writing',
      description: 'Improves business documents and executive summaries.',
      ranking: 3,
      visibility: 'internal',
    },
  ];
  workspaces = [
    { id: 'workspace-personal-os', name: 'Personal OS', collapsed: false, pinned: true, archived: false, sortOrder: 0, memoryFile: 'MEMORY.md' },
    { id: 'workspace-files', name: 'Files', collapsed: false, pinned: false, archived: false, sortOrder: 1, memoryFile: 'MEMORY.md' },
    { id: 'workspace-remote-control', name: 'Remote Control', collapsed: false, pinned: false, archived: false, sortOrder: 2, memoryFile: 'MEMORY.md' },
  ];
  deletedWorkspaces = [];
  shares = [];
  uploads = [];
  previews = [
    {
      id: 'preview-weekly-brief',
      taskId: 'task-weekly-brief',
      artifactId: 'artifact-weekly-brief',
      filename: 'weekly-brief.md',
      autoRefresh: true,
      displayMode: 'inline',
      renderedAt: '2026-06-07T00:01:00.000Z',
    },
  ];
}

resetAssistantStore();
