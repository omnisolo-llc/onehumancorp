import { describe, expect, test, beforeEach } from 'vitest';
import { GET as getTasks, POST as postTask } from './tasks/route';
import { POST as postRemote, GET as getRemote } from './remote/route';
import { POST as postAutomation, GET as getAutomations, PATCH as patchAutomation } from './automations/route';
import { GET as getMemory, PATCH as patchMemory } from './memory/route';
import { PATCH as patchTaskAction } from './tasks/[id]/route';
import { GET as getSkills, PATCH as patchSkills } from './skills/route';
import { GET as getConnectors, PATCH as patchConnectors } from './connectors/route';
import { GET as getData, PATCH as patchData } from './data/route';
import { POST as postArtifact } from './artifacts/route';
import { GET as getPermissions, PATCH as patchPermissions } from './permissions/route';
import { POST as postFileOperation } from './files/route';
import { GET as getModels, PATCH as patchModels } from './models/route';
import { GET as getMcp, PATCH as patchMcp, POST as postMcp } from './mcp/route';
import { GET as getExperts, PATCH as patchExperts, POST as postExperts } from './experts/route';
import { GET as getCommands, POST as postCommand } from './commands/route';
import { GET as getWorkspaces, PATCH as patchWorkspaces } from './workspaces/route';
import { GET as getShares, POST as postShare } from './share/route';
import { GET as getUploads, POST as postUpload } from './uploads/route';
import { GET as getPreviews, PATCH as patchPreviews } from './previews/route';
import { GET as getPlugins, PATCH as patchPlugins } from './plugins/route';
import { GET as getClaw, PATCH as patchClaw } from './claw/route';
import { GET as getApprovals, POST as postApproval, PATCH as patchApproval } from './approvals/route';
import { GET as getSettings, PATCH as patchSettings } from './settings/route';
import { POST as postSupport } from './support/route';
import { GET as getExplore, POST as postExplore, PATCH as patchExplore } from './explore/route';
import { GET as getCloud, POST as postCloud, PATCH as patchCloud } from './cloud/route';
import { GET as getParity } from './parity/route';
import { resetAssistantStore } from './store';

function jsonRequest(url: string, body: unknown) {
  return new Request(url, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  });
}

function patchRequest(url: string, body: unknown) {
  return new Request(url, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  });
}

describe('assistant API contract', () => {
  beforeEach(() => {
    resetAssistantStore();
  });

  test('lists seeded Jarvis tasks with artifacts and changes', async () => {
    const response = await getTasks();
    const body = await response.json();

    expect(response.status).toBe(200);
    expect(body.tasks.length).toBeGreaterThanOrEqual(2);
    expect(body.tasks[0]).toMatchObject({
      workspace: 'Personal OS',
      status: 'running',
      permissionProfile: 'Guarded',
    });
    expect(body.tasks[0].artifacts[0]).toMatchObject({
      type: 'document',
      filename: 'weekly-brief.md',
    });
    expect(body.tasks[0].changes[0]).toMatchObject({
      path: '/workspace/reports/weekly-brief.md',
      approvalStatus: 'pending',
    });
    expect(body.capabilities.resultTabs).toEqual(['Artifacts', 'All Files', 'Changes', 'Preview']);
    expect(body.capabilities.remotePlatforms).toEqual([
      'Slack',
      'Telegram',
      'Discord',
      'WeChat Work',
      'Feishu',
      'DingTalk',
      'QQ',
      'YuanbaoPai',
      'WeChat ClawBot',
    ]);
    expect(body.capabilities.outputFormats).toEqual([
      'Document',
      'Spreadsheet',
      'Presentation',
      'PDF',
      'Chart',
      'Code App',
      'ZIP',
    ]);
    expect(body.capabilities.modelProviders).toEqual([
      'Auto',
      'WorkBuddy',
      'MiniMax M2.5',
      'GLM-4.6',
      'Kimi K2',
      'DeepSeek V3.2',
      'Claude Sonnet',
      'GPT-5-Codex',
      'Local Ollama',
      'Custom OpenAI Compatible',
    ]);
    expect(body.capabilities.workModes).toEqual(['Ask', 'Agent', 'Cloud Agent', 'Craft', 'Plan', 'Coding']);
    expect(body.capabilities.computerUseModes).toEqual(['Normal', 'Auto', 'Full Access']);
    expect(body.capabilities.sharingTargets).toEqual(['Share Link', 'WeChat', 'Slack', 'Download', 'Copy']);
    expect(body.capabilities.workspaceControls).toEqual(['Collapse All', 'Expand All', 'Hard Delete', 'Archive Cleanup']);
    expect(body.capabilities.commandSurfaces).toEqual(['/skill', '/compact', '/summarize', '/clear']);
    expect(body.capabilities.mcpFeatures).toEqual(['Tool Progress', 'Resources', 'Static Headers', 'Connector Try It']);
  });

  test('creates a guarded assistant task with complete composer payload', async () => {
    const response = await postTask(jsonRequest('http://localhost/api/assistant/tasks', {
      prompt: 'Research React 19 and create a slide deck with charts',
      workspace: 'Launch Room',
      mode: 'Plan',
      model: 'MiniMax-M3',
      provider: 'Auto',
      workDirectory: '/workspace/launch-room',
      outputFormat: 'Presentation',
      constraints: 'Include citations and draft before sharing',
      contextReferences: '@react-notes @roadmap',
      attachments: ['roadmap.csv'],
      skills: ['Web Research', 'Chart Builder'],
      connectors: ['Google Drive', 'Slack'],
      permissionProfile: 'Guarded',
    }));
    const body = await response.json();

    expect(response.status).toBe(201);
    expect(body.task).toMatchObject({
      title: 'Research React 19 and create a slide deck with charts',
      workspace: 'Launch Room',
      status: 'running',
      mode: 'Plan',
      outputFormat: 'Presentation',
      permissionProfile: 'Guarded',
    });
    expect(body.task.messages.at(-1)).toMatchObject({
      role: 'assistant',
      content: expect.stringContaining('planned the task'),
    });
    expect(body.task.artifacts).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ type: 'presentation', filename: expect.stringMatching(/presentation/) }),
        expect.objectContaining({ type: 'chart', filename: expect.stringMatching(/chart/) }),
      ]),
    );
    expect(body.task.riskSummary).toContain('External sends require approval');
  });

  test('creates local app tasks with code preview and app preview artifacts', async () => {
    const response = await postTask(jsonRequest('http://localhost/api/assistant/tasks', {
      prompt: 'Build a Pomodoro timer app with start pause and reset buttons',
      workspace: 'Utilities',
      mode: 'Coding',
      outputFormat: 'Code App',
      workDirectory: '/workspace/apps/pomodoro',
      permissionProfile: 'Guarded',
    }));
    const body = await response.json();

    expect(response.status).toBe(201);
    expect(body.task.mode).toBe('Coding');
    expect(body.task.artifacts).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ type: 'code', filename: 'app/index.html' }),
        expect.objectContaining({ type: 'document', filename: 'app-preview.html' }),
      ]),
    );
    expect(body.task.actions).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ label: 'Open Preview', kind: 'preview' }),
        expect.objectContaining({ label: 'Run Locally', kind: 'execute', approvalRequired: true }),
      ]),
    );
  });

  test('normalizes remote control messages into assistant tasks', async () => {
    const response = await postRemote(jsonRequest('http://localhost/api/assistant/remote', {
      platform: 'Slack',
      userId: 'U123',
      threadId: 'T456',
      message: 'Convert all PNGs in Downloads to WebP and send me the result',
      attachments: ['screenshot.png'],
    }));
    const body = await response.json();

    expect(response.status).toBe(202);
    expect(body.remote).toMatchObject({
      platform: 'Slack',
      userId: 'U123',
      threadId: 'T456',
      confirmationRequired: true,
    });
    expect(body.task.title).toBe('Convert all PNGs in Downloads to WebP and send me the result');
    expect(body.reply).toContain('started');
  });

  test('accepts every Claw-style remote platform as task intake', async () => {
    for (const platform of ['Slack', 'Telegram', 'Discord', 'WeChat Work', 'Feishu', 'DingTalk', 'QQ', 'YuanbaoPai', 'WeChat ClawBot']) {
      const response = await postRemote(jsonRequest('http://localhost/api/assistant/remote', {
        platform,
        userId: `${platform}-user`,
        threadId: `${platform}-thread`,
        message: `Research today's notes from ${platform}`,
      }));
      const body = await response.json();

      expect(response.status).toBe(202);
      expect(body.remote.platform).toBe(platform);
      expect(body.task.workspace).toBe('Remote Control');
    }
  });

  test('creates scheduled automations using the same assistant task contract', async () => {
    const response = await postAutomation(jsonRequest('http://localhost/api/assistant/automations', {
      name: 'Weekly research brief',
      schedule: 'Every Monday 09:00',
      prompt: 'Research AI workstation updates and draft a summary',
      workspace: 'Research',
      notificationChannel: 'Discord',
      permissionProfile: 'Guarded',
    }));
    const body = await response.json();

    expect(response.status).toBe(201);
    expect(body.automation).toMatchObject({
      name: 'Weekly research brief',
      schedule: 'Every Monday 09:00',
      status: 'active',
      notificationChannel: 'Discord',
    });
    expect(body.automation.nextRunLabel).toContain('Every Monday');

    const listed = await (await getAutomations()).json();
    expect(listed.automations).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ name: 'Weekly research brief', status: 'active' }),
      ]),
    );
  });

  test('edits, imports, and forgets visible assistant memory', async () => {
    const initial = await (await getMemory()).json();
    expect(initial.memories.map((item: any) => item.content)).toContain('Prefer concise technical summaries with citations.');

    const importResponse = await patchMemory(jsonRequest('http://localhost/api/assistant/memory', {
      action: 'import',
      content: 'Always generate spreadsheet outputs with a summary tab first.',
      scope: 'global',
    }));
    const imported = await importResponse.json();
    expect(imported.memories).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ content: 'Always generate spreadsheet outputs with a summary tab first.' }),
      ]),
    );

    const importedId = imported.memories.find((item: any) => item.content.startsWith('Always generate')).id;
    const editResponse = await patchMemory(jsonRequest('http://localhost/api/assistant/memory', {
      action: 'edit',
      id: importedId,
      content: 'For spreadsheets, put the summary tab first.',
    }));
    const edited = await editResponse.json();
    expect(edited.memories).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ id: importedId, content: 'For spreadsheets, put the summary tab first.' }),
      ]),
    );

    const forgetResponse = await patchMemory(jsonRequest('http://localhost/api/assistant/memory', {
      action: 'forget',
      id: importedId,
    }));
    const forgotten = await forgetResponse.json();
    expect(forgotten.memories.some((item: any) => item.id === importedId)).toBe(false);
  });

  test('manages task stop resume archive and approval actions', async () => {
    await patchTaskAction(
      patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'approve_changes' }),
      { params: { id: 'task-weekly-brief' } },
    );
    let body = await (await getTasks()).json();
    expect(body.tasks.find((task: any) => task.id === 'task-weekly-brief').changes[0].approvalStatus).toBe('approved');

    await patchTaskAction(
      patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'stop' }),
      { params: { id: 'task-weekly-brief' } },
    );
    body = await (await getTasks()).json();
    expect(body.tasks.find((task: any) => task.id === 'task-weekly-brief').status).toBe('blocked');

    await patchTaskAction(
      patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'resume' }),
      { params: { id: 'task-weekly-brief' } },
    );
    body = await (await getTasks()).json();
    expect(body.tasks.find((task: any) => task.id === 'task-weekly-brief').status).toBe('running');

    await patchTaskAction(
      patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'archive' }),
      { params: { id: 'task-weekly-brief' } },
    );
    body = await (await getTasks()).json();
    expect(body.tasks.find((task: any) => task.id === 'task-weekly-brief').status).toBe('archived');
  });

  test('manages skills connector status and data cleanup queues', async () => {
    let skills = await (await getSkills()).json();
    expect(skills.skills).toEqual(expect.arrayContaining([expect.objectContaining({ name: 'Web Research', status: 'installed' })]));
    expect(skills.skills).toEqual(expect.arrayContaining([
      expect.objectContaining({ name: 'Expert Ranking', category: 'Expert Center', status: 'available' }),
      expect.objectContaining({ name: 'Custom Expert Builder', category: 'Expert Center', status: 'available' }),
      expect.objectContaining({ name: 'Slash Command Runner', category: 'Commands', status: 'installed' }),
    ]));

    skills = await (await patchSkills(patchRequest('http://localhost/api/assistant/skills', {
      action: 'install',
      name: 'PDF Exporter',
      category: 'Artifacts',
    }))).json();
    expect(skills.skills).toEqual(expect.arrayContaining([expect.objectContaining({ name: 'PDF Exporter', status: 'installed' })]));

    skills = await (await patchSkills(patchRequest('http://localhost/api/assistant/skills', {
      action: 'disable',
      name: 'PDF Exporter',
    }))).json();
    expect(skills.skills).toEqual(expect.arrayContaining([expect.objectContaining({ name: 'PDF Exporter', status: 'disabled' })]));

    let connectors = await (await getConnectors()).json();
    expect(connectors.connectors).toEqual(expect.arrayContaining([expect.objectContaining({ name: 'MCP Endpoint' })]));
    expect(connectors.connectors).toEqual(expect.arrayContaining([
      expect.objectContaining({
        name: 'MCP Endpoint',
        features: expect.arrayContaining(['Tool Progress', 'Resources', 'Static Headers', 'Connector Try It']),
      }),
      expect.objectContaining({ name: 'Tencent Docs', kind: 'office' }),
      expect.objectContaining({ name: 'QQ Mail', kind: 'office' }),
    ]));

    connectors = await (await patchConnectors(patchRequest('http://localhost/api/assistant/connectors', {
      action: 'connect',
      name: 'Notion',
      kind: 'knowledge',
    }))).json();
    expect(connectors.connectors).toEqual(expect.arrayContaining([expect.objectContaining({ name: 'Notion', status: 'connected' })]));

    let data = await (await getData()).json();
    expect(data.sharedFiles.length).toBeGreaterThan(0);
    data = await (await patchData(patchRequest('http://localhost/api/assistant/data', {
      action: 'unshare',
      id: data.sharedFiles[0].id,
    }))).json();
    expect(data.unshareQueue.length).toBeGreaterThan(0);
  });

  test('lists remote platform connection status', async () => {
    const body = await (await getRemote()).json();
    expect(body.platforms).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ name: 'Slack', status: 'available' }),
        expect.objectContaining({ name: 'WeChat ClawBot', status: 'available' }),
      ]),
    );
  });

  test('generates WorkBuddy-style office export artifacts', async () => {
    for (const [format, mimeType] of [
      ['Document', 'application/vnd.openxmlformats-officedocument.wordprocessingml.document'],
      ['Spreadsheet', 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet'],
      ['Presentation', 'application/vnd.openxmlformats-officedocument.presentationml.presentation'],
      ['PDF', 'application/pdf'],
      ['ZIP', 'application/zip'],
    ]) {
      const response = await postArtifact(jsonRequest('http://localhost/api/assistant/artifacts', {
        taskId: 'task-weekly-brief',
        outputFormat: format,
        title: `${format} Export`,
      }));
      const body = await response.json();

      expect(response.status).toBe(201);
      expect(body.artifact).toMatchObject({
        mimeType,
        filename: expect.any(String),
      });
      expect(body.artifact.preview).toContain(format);
    }
  });

  test('grants and revokes guarded folder permissions', async () => {
    let body = await (await getPermissions()).json();
    expect(body.permissionProfile).toBe('Guarded');
    expect(body.authorizedFolders).toEqual(expect.arrayContaining(['/workspace/assistant']));

    body = await (await patchPermissions(patchRequest('http://localhost/api/assistant/permissions', {
      action: 'grant',
      folder: '/Users/me/Downloads',
    }))).json();
    expect(body.authorizedFolders).toContain('/Users/me/Downloads');

    body = await (await patchPermissions(patchRequest('http://localhost/api/assistant/permissions', {
      action: 'revoke',
      folder: '/Users/me/Downloads',
    }))).json();
    expect(body.authorizedFolders).not.toContain('/Users/me/Downloads');
  });

  test('plans guarded local file operations before execution', async () => {
    const response = await postFileOperation(jsonRequest('http://localhost/api/assistant/files', {
      operation: 'batch_convert',
      folder: '/Users/me/Downloads',
      sourcePattern: '*.png',
      targetFormat: 'webp',
    }));
    const body = await response.json();

    expect(response.status).toBe(202);
    expect(body.operation).toMatchObject({
      operation: 'batch_convert',
      folder: '/Users/me/Downloads',
      status: 'needs_permission',
      approvalRequired: true,
    });
    expect(body.operation.plan).toEqual(
      expect.arrayContaining([
        expect.stringContaining('Read matching files'),
        expect.stringContaining('Write converted files'),
      ]),
    );
  });

  test('manages custom model UI settings and runtime detection', async () => {
    let body = await (await getModels()).json();
    expect(body.runtime).toEqual(expect.arrayContaining([
      expect.objectContaining({ name: 'Node.js', status: 'detected' }),
      expect.objectContaining({ name: 'Python', status: 'needs_setup' }),
    ]));
    expect(body.models).toEqual(expect.arrayContaining([
      expect.objectContaining({ provider: 'Auto', enabled: true }),
      expect.objectContaining({ provider: 'Local Ollama' }),
    ]));

    body = await (await patchModels(patchRequest('http://localhost/api/assistant/models', {
      action: 'upsert',
      provider: 'Custom OpenAI Compatible',
      modelId: 'jarvis-custom',
      endpoint: 'https://models.example.test/v1',
      headers: { 'X-Team': 'ops' },
      parameters: { temperature: 0.2, reasoningEffort: 'medium' },
      skipChatCompletions: true,
    }))).json();
    expect(body.models).toEqual(expect.arrayContaining([
      expect.objectContaining({
        provider: 'Custom OpenAI Compatible',
        modelId: 'jarvis-custom',
        endpoint: 'https://models.example.test/v1',
        skipChatCompletions: true,
      }),
    ]));
  });

  test('manages MCP connectors, trust, oauth, resources, and tool progress', async () => {
    let body = await (await getMcp()).json();
    expect(body.servers).toEqual(expect.arrayContaining([
      expect.objectContaining({ name: 'MCP Endpoint', trusted: false }),
    ]));

    body = await (await postMcp(jsonRequest('http://localhost/api/assistant/mcp', {
      name: 'Linear MCP',
      url: 'https://mcp.example.test/sse',
      headers: { Authorization: 'Bearer token' },
      oauth: true,
    }))).json();
    expect(body.server).toMatchObject({
      name: 'Linear MCP',
      status: 'needs_trust',
      trusted: false,
      oauth: true,
    });
    expect(body.server.features).toEqual(expect.arrayContaining(['Static Headers', 'Tool Progress']));

    body = await (await patchMcp(patchRequest('http://localhost/api/assistant/mcp', {
      action: 'trust',
      id: body.server.id,
    }))).json();
    expect(body.servers).toEqual(expect.arrayContaining([
      expect.objectContaining({ name: 'Linear MCP', trusted: true, status: 'connected' }),
    ]));

    body = await (await patchMcp(patchRequest('http://localhost/api/assistant/mcp', {
      action: 'try_tool',
      name: 'Linear MCP',
      tool: 'search_issues',
    }))).json();
    expect(body.progress).toEqual(expect.arrayContaining([
      expect.objectContaining({ stage: 'queued' }),
      expect.objectContaining({ stage: 'completed', tool: 'search_issues' }),
    ]));
    expect(body.resources).toEqual(expect.arrayContaining([
      expect.objectContaining({ uri: 'mcp://linear-mcp/resources/search_issues' }),
    ]));
  });

  test('supports Expert Center search ranking custom experts and summon prompts', async () => {
    let body = await (await getExperts()).json();
    expect(body.experts).toEqual(expect.arrayContaining([
      expect.objectContaining({ name: 'Research Strategist', ranking: 1, visibility: 'public' }),
    ]));
    expect(body.recommendedPrompts).toEqual(expect.arrayContaining([
      expect.stringContaining('Research Strategist'),
    ]));

    body = await (await postExperts(jsonRequest('http://localhost/api/assistant/experts', {
      name: 'Sales Ops Analyst',
      domain: 'Revenue',
      description: 'Pipeline hygiene and forecast inspection.',
      visibility: 'private',
    }))).json();
    expect(body.expert).toMatchObject({
      name: 'Sales Ops Analyst',
      domain: 'Revenue',
      visibility: 'private',
    });

    body = await (await patchExperts(patchRequest('http://localhost/api/assistant/experts', {
      action: 'summon',
      id: body.expert.id,
      taskId: 'task-weekly-brief',
    }))).json();
    expect(body.task.messages.at(-1)).toMatchObject({
      role: 'assistant',
      content: expect.stringContaining('Sales Ops Analyst'),
    });
  });

  test('runs default slash commands against task context', async () => {
    let body = await (await getCommands()).json();
    expect(body.commands).toEqual(expect.arrayContaining([
      expect.objectContaining({ command: '/skill' }),
      expect.objectContaining({ command: '/compact' }),
      expect.objectContaining({ command: '/summarize' }),
      expect.objectContaining({ command: '/clear' }),
    ]));

    body = await (await postCommand(jsonRequest('http://localhost/api/assistant/commands', {
      command: '/summarize',
      taskId: 'task-weekly-brief',
    }))).json();
    expect(body.result).toMatchObject({
      command: '/summarize',
      status: 'completed',
    });
    expect(body.task.messages.at(-1).content).toContain('Summary');

    body = await (await postCommand(jsonRequest('http://localhost/api/assistant/commands', {
      command: '/clear',
      taskId: 'task-weekly-brief',
    }))).json();
    expect(body.task.messages).toHaveLength(1);
    expect(body.task.messages[0].content).toContain('Context cleared');
  });

  test('manages workspaces collapse pin archive filter sort and hard delete', async () => {
    let body = await (await getWorkspaces()).json();
    expect(body.workspaces).toEqual(expect.arrayContaining([
      expect.objectContaining({ name: 'Personal OS', memoryFile: 'MEMORY.md' }),
    ]));

    body = await (await patchWorkspaces(patchRequest('http://localhost/api/assistant/workspaces', {
      action: 'collapse_all',
    }))).json();
    expect(body.workspaces.every((workspace: any) => workspace.collapsed)).toBe(true);

    body = await (await patchWorkspaces(patchRequest('http://localhost/api/assistant/workspaces', {
      action: 'pin',
      name: 'Files',
    }))).json();
    expect(body.workspaces).toEqual(expect.arrayContaining([
      expect.objectContaining({ name: 'Files', pinned: true }),
    ]));

    body = await (await patchWorkspaces(patchRequest('http://localhost/api/assistant/workspaces', {
      action: 'hard_delete',
      name: 'Files',
      confirm: 'DELETE',
    }))).json();
    expect(body.workspaces.some((workspace: any) => workspace.name === 'Files')).toBe(false);
    expect(body.deleted).toEqual(expect.arrayContaining([
      expect.objectContaining({ name: 'Files' }),
    ]));
  });

  test('shares artifacts with online previews and channel audit state', async () => {
    let body = await (await getShares()).json();
    expect(body.shares).toEqual([]);

    body = await (await postShare(jsonRequest('http://localhost/api/assistant/share', {
      taskId: 'task-weekly-brief',
      artifactId: 'artifact-weekly-brief',
      target: 'WeChat',
    }))).json();
    expect(body.share).toMatchObject({
      taskId: 'task-weekly-brief',
      artifactId: 'artifact-weekly-brief',
      target: 'WeChat',
      status: 'pending_review',
    });
    expect(body.share.previewUrl).toContain('/assistant/preview/');
    expect(body.share.audit).toEqual(expect.arrayContaining([
      expect.stringContaining('sharing review'),
    ]));
  });

  test('tracks remote uploaded files and attaches them to follow-up tasks', async () => {
    let body = await (await postUpload(jsonRequest('http://localhost/api/assistant/uploads', {
      platform: 'WeChat ClawBot',
      userId: 'wechat-user',
      filename: 'receipt.png',
      mimeType: 'image/png',
      sizeBytes: 4567,
      previewText: 'receipt image',
    }))).json();
    expect(body.upload).toMatchObject({
      platform: 'WeChat ClawBot',
      filename: 'receipt.png',
      status: 'available',
    });

    const uploads = await (await getUploads()).json();
    expect(uploads.uploads).toEqual(expect.arrayContaining([
      expect.objectContaining({ filename: 'receipt.png', previewUrl: expect.stringContaining('/assistant/uploads/') }),
    ]));

    const remote = await (await postRemote(jsonRequest('http://localhost/api/assistant/remote', {
      platform: 'WeChat ClawBot',
      userId: 'wechat-user',
      threadId: 'wechat-thread',
      message: 'Extract totals from the uploaded receipt',
      uploadIds: [body.upload.id],
    }))).json();
    expect(remote.task.attachments).toEqual(expect.arrayContaining(['receipt.png']));
  });

  test('refreshes artifact previews and supports fullscreen external open state', async () => {
    let body = await (await getPreviews()).json();
    expect(body.previews).toEqual(expect.arrayContaining([
      expect.objectContaining({ artifactId: 'artifact-weekly-brief', autoRefresh: true }),
    ]));

    body = await (await patchPreviews(patchRequest('http://localhost/api/assistant/previews', {
      action: 'open_external',
      artifactId: 'artifact-weekly-brief',
    }))).json();
    expect(body.preview).toMatchObject({
      artifactId: 'artifact-weekly-brief',
      displayMode: 'external',
      autoRefresh: true,
    });
    expect(body.preview.renderedAt).toEqual(expect.any(String));
  });

  test('manages plugin and suite marketplace install update try and uninstall cleanup', async () => {
    let body = await (await getPlugins()).json();
    expect(body.plugins).toEqual(expect.arrayContaining([
      expect.objectContaining({ name: 'Office Suite', type: 'suite', version: '1.0.0' }),
      expect.objectContaining({ name: 'Image Generator', type: 'skill', securityStatus: 'passed' }),
    ]));
    expect(body.versionCache).toEqual(expect.objectContaining({ lastSyncedAt: expect.any(String) }));

    body = await (await patchPlugins(patchRequest('http://localhost/api/assistant/plugins', {
      action: 'install',
      id: 'plugin-office-suite',
    }))).json();
    expect(body.plugins).toEqual(expect.arrayContaining([
      expect.objectContaining({ id: 'plugin-office-suite', status: 'installed', loading: false }),
    ]));
    expect(body.skills).toEqual(expect.arrayContaining([
      expect.objectContaining({ name: 'Office Suite Writer', status: 'installed' }),
    ]));
    expect(body.mcpServers).toEqual(expect.arrayContaining([
      expect.objectContaining({ name: 'Office Suite MCP', status: 'needs_trust' }),
    ]));

    body = await (await patchPlugins(patchRequest('http://localhost/api/assistant/plugins', {
      action: 'update',
      id: 'plugin-office-suite',
      version: '1.1.0',
    }))).json();
    expect(body.plugins).toEqual(expect.arrayContaining([
      expect.objectContaining({ id: 'plugin-office-suite', version: '1.1.0', updateAvailable: false }),
    ]));

    body = await (await patchPlugins(patchRequest('http://localhost/api/assistant/plugins', {
      action: 'try',
      id: 'plugin-office-suite',
      taskId: 'task-weekly-brief',
    }))).json();
    expect(body.task.messages.at(-1).content).toContain('Office Suite');

    body = await (await patchPlugins(patchRequest('http://localhost/api/assistant/plugins', {
      action: 'uninstall',
      id: 'plugin-office-suite',
    }))).json();
    expect(body.plugins).toEqual(expect.arrayContaining([
      expect.objectContaining({ id: 'plugin-office-suite', status: 'available' }),
    ]));
    expect(body.mcpServers.some((server: any) => server.name === 'Office Suite MCP')).toBe(false);
  });

  test('runs one-time and temporary-workspace automations through pause resume run and delete lifecycle', async () => {
    let body = await (await postAutomation(jsonRequest('http://localhost/api/assistant/automations', {
      name: 'One-time invoice cleanup',
      schedule: '2026-06-08T09:00:00.000Z',
      prompt: 'Clean invoice attachments once',
      type: 'one_time',
      temporaryWorkspace: true,
      peakScheduling: true,
      notificationChannel: 'WeChat ClawBot',
    }))).json();
    expect(body.automation).toMatchObject({
      type: 'one_time',
      temporaryWorkspace: true,
      peakScheduling: true,
      workspace: 'Temporary Workspace',
    });

    const automationId = body.automation.id;
    body = await (await patchAutomation(patchRequest('http://localhost/api/assistant/automations', {
      action: 'run_now',
      id: automationId,
    }))).json();
    expect(body.automation.runHistory).toEqual(expect.arrayContaining([
      expect.objectContaining({ status: 'completed' }),
    ]));

    body = await (await patchAutomation(patchRequest('http://localhost/api/assistant/automations', {
      action: 'pause',
      id: automationId,
    }))).json();
    expect(body.automation.status).toBe('paused');

    body = await (await patchAutomation(patchRequest('http://localhost/api/assistant/automations', {
      action: 'resume',
      id: automationId,
    }))).json();
    expect(body.automation.status).toBe('active');

    body = await (await patchAutomation(patchRequest('http://localhost/api/assistant/automations', {
      action: 'delete',
      id: automationId,
    }))).json();
    expect(body.automations.some((automation: any) => automation.id === automationId)).toBe(false);
  });

  test('supports task pin rename save to workspace archived rename and hard delete', async () => {
    let body = await (await patchTaskAction(
      patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'pin' }),
      { params: { id: 'task-weekly-brief' } },
    )).json();
    expect(body.task.pinned).toBe(true);

    body = await (await patchTaskAction(
      patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'rename', title: 'Weekly operating review' }),
      { params: { id: 'task-weekly-brief' } },
    )).json();
    expect(body.task.title).toBe('Weekly operating review');

    body = await (await patchTaskAction(
      patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', {
        action: 'save_to_workspace',
        workspace: 'Leadership',
        workDirectory: '/workspace/leadership',
      }),
      { params: { id: 'task-weekly-brief' } },
    )).json();
    expect(body.task).toMatchObject({ workspace: 'Leadership', workDirectory: '/workspace/leadership' });

    body = await (await patchTaskAction(
      patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'archive' }),
      { params: { id: 'task-weekly-brief' } },
    )).json();
    expect(body.task.status).toBe('archived');

    body = await (await patchTaskAction(
      patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'rename_archived', title: 'Archived review' }),
      { params: { id: 'task-weekly-brief' } },
    )).json();
    expect(body.task.title).toBe('Archived review');

    body = await (await patchTaskAction(
      patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'hard_delete', confirm: 'DELETE' }),
      { params: { id: 'task-weekly-brief' } },
    )).json();
    expect(body.deletedTask.id).toBe('task-weekly-brief');
  });

  test('manages Claw bot setup disconnect markdown and command confirmation', async () => {
    let body = await (await getClaw()).json();
    expect(body.channels).toEqual(expect.arrayContaining([
      expect.objectContaining({ platform: 'WeChat ClawBot', markdownRendering: true, qrCodeUrl: expect.stringContaining('/assistant/claw/') }),
    ]));

    body = await (await patchClaw(patchRequest('http://localhost/api/assistant/claw', {
      action: 'connect',
      platform: 'Slack',
      credentials: { appId: 'A123', botToken: 'xoxb-token' },
    }))).json();
    expect(body.channels).toEqual(expect.arrayContaining([
      expect.objectContaining({ platform: 'Slack', status: 'connected' }),
    ]));
    expect(body.guides).toEqual(expect.arrayContaining([
      expect.objectContaining({ platform: 'Slack', steps: expect.arrayContaining([expect.stringContaining('app')]) }),
    ]));

    body = await (await patchClaw(patchRequest('http://localhost/api/assistant/claw', {
      action: 'confirm_command',
      platform: 'WeChat ClawBot',
      commandId: 'cmd-danger-1',
      decision: 'approve',
    }))).json();
    expect(body.confirmations).toEqual(expect.arrayContaining([
      expect.objectContaining({ commandId: 'cmd-danger-1', decision: 'approve' }),
    ]));
  });

  test('records high-risk approvals before external sends and destructive actions', async () => {
    let body = await (await getApprovals()).json();
    expect(body.approvals).toEqual([]);

    body = await (await postApproval(jsonRequest('http://localhost/api/assistant/approvals', {
      taskId: 'task-weekly-brief',
      action: 'external_send',
      summary: 'Send weekly brief to WeChat',
      riskLevel: 'high',
    }))).json();
    expect(body.approval).toMatchObject({
      taskId: 'task-weekly-brief',
      action: 'external_send',
      riskLevel: 'high',
      status: 'pending',
    });

    body = await (await patchApproval(patchRequest('http://localhost/api/assistant/approvals', {
      id: body.approval.id,
      decision: 'approve',
      reviewer: 'owner',
    }))).json();
    expect(body.approval).toMatchObject({ status: 'approved', reviewer: 'owner' });
  });

  test('updates UI settings content filter font size language and support uploads', async () => {
    let body = await (await getSettings()).json();
    expect(body.settings).toMatchObject({
      fontSize: 'medium',
      systemLanguage: 'auto',
      aiGeneratedMarker: true,
      contentFilter: 'friendly_notice',
    });

    body = await (await patchSettings(patchRequest('http://localhost/api/assistant/settings', {
      fontSize: 'large',
      systemLanguage: 'en-US',
      contentFilter: 'hide_filtered_answer',
    }))).json();
    expect(body.settings).toMatchObject({
      fontSize: 'large',
      systemLanguage: 'en-US',
      contentFilter: 'hide_filtered_answer',
    });

    body = await (await postSupport(jsonRequest('http://localhost/api/assistant/support', {
      kind: 'upload_logs',
      message: 'Investigate Claw reconnect issue',
      includeLogs: true,
    }))).json();
    expect(body.ticket).toMatchObject({
      kind: 'upload_logs',
      status: 'received',
      logBundle: expect.stringContaining('jarvis-logs'),
    });
  });

  test('lists explores shares and remixes community tasks as personal Jarvis agents', async () => {
    let body = await (await getExplore()).json();
    expect(body.templates).toEqual(expect.arrayContaining([
      expect.objectContaining({
        name: 'Investor Update Agent',
        source: 'community',
        remixable: true,
      }),
      expect.objectContaining({
        name: 'Local File Cleanup Agent',
        useCases: expect.arrayContaining(['batch_rename', 'merge_pdfs']),
      }),
    ]));
    expect(body.exploreActions).toEqual(expect.arrayContaining(['Try Task', 'Remix Agent', 'Share Exploration']));

    body = await (await postExplore(jsonRequest('http://localhost/api/assistant/explore', {
      templateId: 'explore-investor-update',
      workspace: 'Founder OS',
      ownerGoal: 'Prepare my own investor update every Friday',
    }))).json();
    expect(body.remix).toMatchObject({
      sourceTemplateId: 'explore-investor-update',
      workspace: 'Founder OS',
      visibility: 'private',
      attribution: expect.stringContaining('Investor Update Agent'),
    });
    expect(body.task).toMatchObject({
      workspace: 'Founder OS',
      mode: 'Agent',
      permissionProfile: 'Guarded',
    });
    expect(body.task.skills).toEqual(expect.arrayContaining(['Web Research', 'Document Writer']));

    body = await (await patchExplore(patchRequest('http://localhost/api/assistant/explore', {
      action: 'share',
      remixId: body.remix.id,
      target: 'Share Link',
    }))).json();
    expect(body.remixes).toEqual(expect.arrayContaining([
      expect.objectContaining({ target: 'Share Link', visibility: 'shared' }),
    ]));
  });

  test('runs Cloud Agent sessions in background with pause resume cancel lifecycle', async () => {
    let body = await (await getCloud()).json();
    expect(body.sessions).toEqual(expect.arrayContaining([
      expect.objectContaining({ mode: 'Cloud Agent', status: 'running', background: true }),
    ]));
    expect(body.runtime).toMatchObject({
      isolation: 'cloud',
      asyncTasks: true,
      uploadedFiles: expect.arrayContaining(['workspace-notes.md']),
    });

    body = await (await postCloud(jsonRequest('http://localhost/api/assistant/cloud', {
      prompt: 'Monitor competitors and prepare a launch brief',
      workspace: 'Launch Room',
      model: 'DeepSeek V3.2',
      files: ['competitors.csv'],
      screenshot: 'launch-dashboard.png',
    }))).json();
    expect(body.session).toMatchObject({
      workspace: 'Launch Room',
      mode: 'Cloud Agent',
      model: 'DeepSeek V3.2',
      status: 'running',
      background: true,
      files: expect.arrayContaining(['competitors.csv', 'launch-dashboard.png']),
    });
    expect(body.task).toMatchObject({
      workspace: 'Launch Room',
      mode: 'Cloud Agent',
      model: 'DeepSeek V3.2',
    });

    body = await (await patchCloud(patchRequest('http://localhost/api/assistant/cloud', {
      action: 'pause',
      id: body.session.id,
    }))).json();
    expect(body.session.status).toBe('paused');

    body = await (await patchCloud(patchRequest('http://localhost/api/assistant/cloud', {
      action: 'resume',
      id: body.session.id,
    }))).json();
    expect(body.session.status).toBe('running');

    body = await (await patchCloud(patchRequest('http://localhost/api/assistant/cloud', {
      action: 'cancel',
      id: body.session.id,
    }))).json();
    expect(body.session.status).toBe('canceled');
  });

  test('tracks 150 additional WorkBuddy gaps as implemented Jarvis parity capabilities', async () => {
    const body = await (await getParity()).json();

    expect(body.summary).toMatchObject({
      total: 150,
      implemented: 150,
      remaining: 0,
    });
    expect(body.gaps).toHaveLength(150);
    expect(body.gaps.every((gap: any) => gap.status === 'implemented')).toBe(true);
    expect(body.gaps.map((gap: any) => gap.name)).toEqual(expect.arrayContaining([
      'Runtime sandbox filesystem',
      'ACP/SSE streaming transcript',
      'Checkpoint creation',
      'Version rollback',
      'Manifest editor',
      'Secret injection',
      'Cloud/local execution switch',
      'Expert team decomposition',
      'Hook plugins',
      'Rule plugins',
      'Dedicated remote folder',
      'Automation task templates',
      'Concurrency and runtime limits',
      'Task search box',
      'Task status filtering',
      'Nightly memory summary',
      'User-level MCP config',
      'Project-level MCP config',
      'Mini app voice input',
      'Mini app artifact sharing',
      'Permission risk boundary',
      'Clipboard screenshot paste',
      'Hook event family',
      '/doctor environment check',
      '/btw non-interrupting question',
      '/model text-to-image switch',
      'User settings.json',
      'Project shared settings',
      'Disable bypass permissions',
      'TaskOutput retrieval',
      'Notebook editing',
      'Project subagent directory',
      'Camera attachment',
      'WeChat file attachment',
      'Shared link expiry',
      'Account phone rebinding',
    ]));
    expect(body.categories).toEqual(expect.arrayContaining([
      expect.objectContaining({ name: 'Cloud Agent lifecycle', implemented: 24 }),
      expect.objectContaining({ name: 'Home execution controls', implemented: 4 }),
      expect.objectContaining({ name: 'Expert teams', implemented: 6 }),
      expect.objectContaining({ name: 'Plugin system', implemented: 7 }),
      expect.objectContaining({ name: 'Remote assistant', implemented: 5 }),
      expect.objectContaining({ name: 'Automation governance', implemented: 4 }),
      expect.objectContaining({ name: 'Task management', implemented: 10 }),
      expect.objectContaining({ name: 'Memory governance', implemented: 6 }),
      expect.objectContaining({ name: 'MCP configuration', implemented: 10 }),
      expect.objectContaining({ name: 'Mobile mini app', implemented: 10 }),
      expect.objectContaining({ name: 'Permission safety', implemented: 6 }),
      expect.objectContaining({ name: 'Create task context', implemented: 4 }),
      expect.objectContaining({ name: 'Hook lifecycle', implemented: 4 }),
      expect.objectContaining({ name: 'Slash command coverage', implemented: 16 }),
      expect.objectContaining({ name: 'CLI settings governance', implemented: 10 }),
      expect.objectContaining({ name: 'Built-in tool inventory', implemented: 8 }),
      expect.objectContaining({ name: 'Subagent governance', implemented: 6 }),
      expect.objectContaining({ name: 'Mobile attachment sources', implemented: 6 }),
      expect.objectContaining({ name: 'Account and sharing settings', implemented: 4 }),
    ]));
  });
});
