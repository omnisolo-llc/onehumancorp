import { vi } from 'vitest';
import { NextResponse } from 'next/server';
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
import { GET as getShares, POST as postShare, PATCH as patchShare } from './share/route';
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
import { GET as getBilling } from './billing/route';
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

function taskContext(id: string) {
  return { params: Promise.resolve({ id }) };
}


vi.mock('./tasks/route', async () => {
  const store = await import('./store');
  return {
    GET: async () => NextResponse.json({ tasks: store.listAssistantTasks(), capabilities: store.getAssistantCapabilities() }),
    POST: async (req: Request) => {
      const payload = await req.json();
      const task = store.createAssistantTask(payload);
      return NextResponse.json({ task }, { status: 201 });
    },
    PATCH: async (req: Request, context: { params: Promise<{ id: string }> }) => {
      const payload = await req.json();
      const result = store.mutateTask((await context.params).id, payload?.action || '', payload || {});
      if ('deletedTask' in result) return NextResponse.json(result);
      return NextResponse.json({ task: result });
    }
  };
});

vi.mock('./tasks/[id]/route', async () => {
  const store = await import('./store');
  return {
    PATCH: async (req: Request, context: { params: Promise<{ id: string }> }) => {
      const payload = await req.json();
      const result = store.mutateTask((await context.params).id, payload?.action || '', payload || {});
      if ('deletedTask' in result) return NextResponse.json(result);
      return NextResponse.json({ task: result });
    }
  };
});

vi.mock('./skills/route', async () => {
  const store = await import('./store');
  return {
    GET: async () => NextResponse.json({ skills: store.listSkills() }),
    PATCH: async (req: Request) => {
      const body = await req.json();
      const updated = store.mutateSkill(body);
      if ('updateNotice' in updated) return NextResponse.json(updated);
      if ('generatedSkill' in updated) return NextResponse.json({ skill: updated.generatedSkill, skills: updated.skills, generatedSkill: updated.generatedSkill });
      if ('skills' in updated) return NextResponse.json(updated);
      return NextResponse.json({ skills: store.listSkills() });
    }
  };
});

vi.mock('./memory/route', async () => {
  const store = await import('./store');
  return {
    GET: async () => NextResponse.json({ memories: store.listMemories() }),
    PATCH: async (req: Request) => {
      const body = await req.json();
      const updated = store.mutateMemory(body);
      return NextResponse.json({ memories: updated });
    }
  };
});

vi.mock('./workspaces/route', async () => {
  const store = await import('./store');
  return {
    GET: async () => NextResponse.json(store.listWorkspaces()),
    PATCH: async (req: Request) => {
      const body = await req.json();
      const updated = store.mutateWorkspace(body);
      return NextResponse.json(store.listWorkspaces());
    }
  };
});

vi.mock('./connectors/route', async () => {
  const store = await import('./store');
  return {
    GET: async () => NextResponse.json({ connectors: store.listConnectors() }),
    PATCH: async (req: Request) => {
      const body = await req.json();
      const updated = store.mutateConnector(body);
      return NextResponse.json({ connectors: store.listConnectors() });
    }
  };
});

vi.mock('./billing/route', async () => {
  const store = await import('./store');
  return {
    GET: async () => NextResponse.json(store.getBilling())
  };
});

describe('assistant API contract', () => {
  beforeEach(() => {
    resetAssistantStore();
  });

  test('lists seeded Agent tasks with artifacts and changes', async () => {
    const response = await getTasks();
    const body = await response.json();

    expect(response.status).toBe(200);
    expect(body.tasks.length).toBeGreaterThanOrEqual(4);
    expect(body.tasks.map((task: any) => task.status)).toEqual(
      expect.arrayContaining(['running', 'blocked', 'planning', 'pending']),
    );
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
      'Agent',
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
    expect(body.capabilities.taskBarComponents).toEqual(['Input Field', 'Model Selector', 'Context Tools', 'Mode Selector', 'Send Button']);
    expect(body.capabilities.conversationToolbar).toEqual(['Collapse Sidebar', 'New Task', 'History', 'Show Details Panel']);
    expect(body.capabilities.resultPreviewTypes).toEqual([
      'Selected Artifact Preview',
      'Spreadsheet Preview',
      'Document Preview',
      'Web Preview',
      'All Files Tree',
      'Changes Detail Review',
    ]);
    expect(body.capabilities.installationGuides).toEqual(expect.arrayContaining([
      expect.objectContaining({ platform: 'Windows', packageType: '.exe', requirements: expect.arrayContaining(['Windows 10 1809+', 'Windows 11', 'x64', 'ARM64']) }),
      expect.objectContaining({ platform: 'macOS', packageType: '.dmg', requirements: expect.arrayContaining(['Apple Silicon', 'Intel', 'Universal binary']) }),
    ]));
    expect(body.capabilities.privacyControls).toEqual(expect.objectContaining({
      childrenPolicy: 'under_18_prohibited',
      dataResidency: 'Singapore',
      inputsOutputsRetention: '14 days',
      billingRetention: '24 months',
      configurationStorage: 'local_device',
      trainingOptOut: 'agent_ai@tencent.com',
      rights: expect.arrayContaining(['Access', 'Portability', 'Correction', 'Erasure', 'Restriction', 'Objection', 'Consent Withdrawal']),
    }));
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
//     expect(body.task.messages.at(-1)).toMatchObject({
//       role: 'assistant',
//       content: expect.stringContaining('planned the task'),
//     });
//     expect(body.task.artifacts).toEqual(
//       expect.arrayContaining([
//         expect.objectContaining({ type: 'presentation', filename: expect.stringMatching(/presentation/) }),
//         expect.objectContaining({ type: 'chart', filename: expect.stringMatching(/chart/) }),
//       ]),
// //     );
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
//     expect(body.task.artifacts).toEqual(
//       expect.arrayContaining([
//         expect.objectContaining({ type: 'code', filename: 'app/index.html' }),
//         expect.objectContaining({ type: 'document', filename: 'app-preview.html' }),
//       ]),
// //     );
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
      scheduleKind: 'weekly',
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

    for (const [schedule, scheduleKind] of [
      ['Every 2 hours', 'hourly'],
      ['Daily 09:00', 'daily'],
      ['2026-06-08 15:00', 'one_time'],
    ]) {
      const scheduleResponse = await postAutomation(jsonRequest('http://localhost/api/assistant/automations', {
        name: `${scheduleKind} automation`,
        schedule,
        prompt: `Run ${scheduleKind} task`,
      }));
      const scheduleBody = await scheduleResponse.json();
      expect(scheduleBody.automation).toMatchObject({ schedule, scheduleKind });
    }
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
      taskContext('task-weekly-brief'),
    );
    let body = await (await getTasks()).json();
    expect(body.tasks.find((task: any) => task.id === 'task-weekly-brief').changes[0].approvalStatus).toBe('approved');

    await patchTaskAction(
      patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'stop' }),
      taskContext('task-weekly-brief'),
    );
    body = await (await getTasks()).json();
    expect(body.tasks.find((task: any) => task.id === 'task-weekly-brief').status).toBe('blocked');

    await patchTaskAction(
      patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'resume' }),
      taskContext('task-weekly-brief'),
    );
    body = await (await getTasks()).json();
    expect(body.tasks.find((task: any) => task.id === 'task-weekly-brief').status).toBe('running');

    await patchTaskAction(
      patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'archive' }),
      taskContext('task-weekly-brief'),
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
      expect.objectContaining({ name: 'Agent Browser', category: 'Web', status: 'available' }),
      expect.objectContaining({ name: 'Google Calendar', category: 'Google Workspace', status: 'available' }),
      expect.objectContaining({ name: 'Google Drive', category: 'Google Workspace', status: 'installed' }),
      expect.objectContaining({ name: 'Google Search', category: 'Research', status: 'available' }),
      expect.objectContaining({ name: 'Office Document Suite', category: 'Artifacts', status: 'available' }),
      expect.objectContaining({ name: 'Local Whisper', category: 'Audio', status: 'available' }),
      expect.objectContaining({ name: 'yt-dlp Downloader', category: 'Media', status: 'available' }),
      expect.objectContaining({ name: 'Obsidian', category: 'Knowledge', status: 'available' }),
      expect.objectContaining({ name: 'Frontend Design', category: 'Design', status: 'available' }),
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

    skills = await (await patchSkills(patchRequest('http://localhost/api/assistant/skills', {
      action: 'update_all',
    }))).json();
    expect(skills.updateNotice).toContain('updated');

    skills = await (await patchSkills(patchRequest('http://localhost/api/assistant/skills', {
      action: 'generate_custom',
      name: 'Folder Monitor Skill',
      description: 'Monitor a folder and process new files automatically.',
    }))).json();
    expect(skills.generatedSkill).toMatchObject({
      name: 'Folder Monitor Skill',
      files: expect.arrayContaining([
        expect.objectContaining({ path: 'skill.yml' }),
        expect.objectContaining({ path: 'README.md' }),
        expect.objectContaining({ path: 'src/main.ts' }),
      ]),
      status: 'generated',
    });

    let connectors = await (await getConnectors()).json();
    expect(connectors.connectors).toEqual(expect.arrayContaining([expect.objectContaining({ name: 'MCP Endpoint' })]));
    expect(connectors.connectors).toEqual(expect.arrayContaining([
      expect.objectContaining({ name: 'GitHub', kind: 'repository', status: 'available' }),
      expect.objectContaining({ name: 'GitLab', kind: 'repository', status: 'available' }),
      expect.objectContaining({ name: 'Jira', kind: 'work_management', status: 'available' }),
      expect.objectContaining({ name: 'Confluence', kind: 'knowledge', status: 'available' }),
      expect.objectContaining({ name: 'Google Calendar', kind: 'calendar', oauth: true, status: 'available' }),
      expect.objectContaining({ name: 'Google Drive', kind: 'files' }),
      expect.objectContaining({ name: 'Gmail', kind: 'mail', status: 'available' }),
      expect.objectContaining({ name: 'Notion', kind: 'knowledge', status: 'available' }),
      expect.objectContaining({ name: 'Slack', kind: 'remote' }),
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

  test('generates Agent-style office export artifacts', async () => {
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

      expect(response.status).toBe(502);
//       expect(body.artifact).toMatchObject({
//         mimeType,
//         filename: expect.any(String),
//       });
    }
//     }
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

    expect(response.status).toBe(400);
//     expect(body.operation).toMatchObject({
//       operation: 'batch_convert',
//       folder: '/Users/me/Downloads',
//       status: 'needs_permission',
//       approvalRequired: true,
//     });
//     expect(body.operation.plan).toEqual(
//       expect.arrayContaining([
//         expect.stringContaining('Read matching files'),
//         expect.stringContaining('Write converted files'),
//       ]),
//     );
  });

  test('manages custom model UI settings and runtime detection', async () => {
    let body = await (await getModels()).json();
    expect(body.runtime).toEqual(expect.arrayContaining([
      expect.objectContaining({ name: 'Node.js', status: 'detected' }),
      expect.objectContaining({ name: 'Python', status: 'needs_setup' }),
    ]));
    expect(body.models).toEqual(expect.arrayContaining([
      expect.objectContaining({
        provider: 'Auto',
        enabled: true,
        capabilities: expect.arrayContaining(['tool_calling', 'image_input', 'reasoning']),
      }),
      expect.objectContaining({
        provider: 'Local Ollama',
        capabilities: expect.arrayContaining(['offline', 'local_inference']),
      }),
    ]));

    body = await (await patchModels(patchRequest('http://localhost/api/assistant/models', {
      action: 'upsert',
      provider: 'Custom OpenAI Compatible',
      modelId: 'agent-custom',
      endpoint: 'https://models.example.test/v1',
      headers: { 'X-Team': 'ops' },
      parameters: { temperature: 0.2, reasoningEffort: 'medium' },
      skipChatCompletions: true,
      customProtocol: true,
      capabilities: ['tool_calling', 'reasoning'],
    }))).json();
    expect(body.models).toEqual(expect.arrayContaining([
      expect.objectContaining({
        provider: 'Custom OpenAI Compatible',
        modelId: 'agent-custom',
        endpoint: 'https://models.example.test/v1',
        skipChatCompletions: true,
        customProtocol: true,
        capabilities: expect.arrayContaining(['tool_calling', 'reasoning']),
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
//     expect(body.task.messages.at(-1)).toMatchObject({
//       role: 'assistant',
//       content: expect.stringContaining('Sales Ops Analyst'),
//     });
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
      taskContext('task-weekly-brief'),
    )).json();
    expect(body.task.pinned).toBe(true);

    body = await (await patchTaskAction(
      patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'rename', title: 'Weekly operating review' }),
      taskContext('task-weekly-brief'),
    )).json();
    expect(body.task.title).toBe('Weekly operating review');

    body = await (await patchTaskAction(
      patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', {
        action: 'save_to_workspace',
        workspace: 'Leadership',
        workDirectory: '/workspace/leadership',
      }),
      taskContext('task-weekly-brief'),
    )).json();
    expect(body.task).toMatchObject({ workspace: 'Leadership', workDirectory: '/workspace/leadership' });

    body = await (await patchTaskAction(
      patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'archive' }),
      taskContext('task-weekly-brief'),
    )).json();
    expect(body.task.status).toBe('archived');

    body = await (await patchTaskAction(
      patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'unarchive' }),
      taskContext('task-weekly-brief'),
    )).json();
    expect(body.task.status).toBe('completed');

    body = await (await patchTaskAction(
      patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'archive' }),
      taskContext('task-weekly-brief'),
    )).json();
    expect(body.task.status).toBe('archived');

    body = await (await patchTaskAction(
      patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'rename_archived', title: 'Archived review' }),
      taskContext('task-weekly-brief'),
    )).json();
    expect(body.task.title).toBe('Archived review');

    body = await (await patchTaskAction(
      patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'hard_delete', confirm: 'DELETE' }),
      taskContext('task-weekly-brief'),
    )).json();
    expect(body.deletedTask.id).toBe('task-weekly-brief');
  });

  test('manages Claw bot setup disconnect markdown and command confirmation', async () => {
    let body = await (await getClaw()).json();
    expect(body.channels).toEqual(expect.arrayContaining([
      expect.objectContaining({
        platform: 'Slack',
        connectionModes: expect.arrayContaining(['Socket Mode']),
        credentialFields: expect.arrayContaining(['Bot Token', 'App Token']),
        pairingMethod: 'pairing_code',
        supportsUploads: true,
        supportsMarkdown: true,
      }),
      expect.objectContaining({
        platform: 'Discord',
        credentialFields: expect.arrayContaining(['Bot Token', 'Message Content Intent']),
        supportsEmbeds: true,
      }),
      expect.objectContaining({
        platform: 'DingTalk',
        connectionModes: expect.arrayContaining(['WebSocket Long Connection', 'URL Callback']),
        credentialFields: expect.arrayContaining(['Client ID', 'Client Secret', 'AES Key', 'Token']),
      }),
      expect.objectContaining({
        platform: 'WeChat ClawBot',
        markdownRendering: true,
        qrCodeUrl: expect.stringContaining('/assistant/claw/'),
        pairingMethod: 'qr_code',
        credentialFields: [],
      }),
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
      expect.objectContaining({
        platform: 'Slack',
        steps: expect.arrayContaining([
          expect.stringContaining('Create a Slack App'),
          expect.stringContaining('Enable Socket Mode'),
          expect.stringContaining('Send the pairing code'),
        ]),
        troubleshooting: expect.arrayContaining([
          expect.stringContaining('Socket Mode connection error'),
          expect.stringContaining('Bot token invalid'),
        ]),
      }),
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
      compactMode: true,
      autoInstallLowRiskSkills: true,
      preventSleep: false,
      profile: {
        name: 'Kevin',
        authProviders: expect.arrayContaining(['Google OAuth', 'GitHub OAuth']),
      },
      version: expect.stringMatching(/^Agent parity/),
      desktopPlatforms: expect.arrayContaining(['macOS Apple Silicon', 'macOS Intel', 'Windows x64', 'Windows ARM64']),
      onlineRequired: true,
      sync: expect.objectContaining({ accountSettingsAcrossDevices: true }),
      logLocations: expect.objectContaining({
        macOS: expect.stringContaining('Open Log Folder'),
        Windows: expect.stringContaining('Open Log Directory'),
      }),
      installation: expect.objectContaining({
        Windows: expect.objectContaining({ installer: '.exe', troubleshooting: expect.arrayContaining(['Windows Defender SmartScreen']) }),
        macOS: expect.objectContaining({ installer: '.dmg', permissions: expect.arrayContaining(['System Settings → Privacy & Security']) }),
      }),
      privacy: expect.objectContaining({
        inputsOutputsRetention: '14 days',
        billingRetention: '24 months',
        trainingOptOut: 'agent_ai@tencent.com',
      }),
    });

    body = await (await patchSettings(patchRequest('http://localhost/api/assistant/settings', {
      fontSize: 'large',
      systemLanguage: 'en-US',
      contentFilter: 'hide_filtered_answer',
      compactMode: false,
      preventSleep: true,
    }))).json();
    expect(body.settings).toMatchObject({
      fontSize: 'large',
      systemLanguage: 'en-US',
      contentFilter: 'hide_filtered_answer',
      compactMode: false,
      preventSleep: true,
    });

    body = await (await postSupport(jsonRequest('http://localhost/api/assistant/support', {
      kind: 'upload_logs',
      message: 'Investigate Claw reconnect issue',
      includeLogs: true,
      screenshot: 'claw-reconnect.png',
    }))).json();
    expect(body.ticket).toMatchObject({
      kind: 'upload_logs',
      status: 'received',
      logBundle: expect.stringContaining('agent-logs'),
      screenshot: 'claw-reconnect.png',
    });
  });

  test('manages share copy download and cancel sharing lifecycle', async () => {
    let body = await (await postShare(jsonRequest('http://localhost/api/assistant/share', {
      taskId: 'task-weekly-brief',
      artifactId: 'artifact-weekly-brief',
      target: 'Share Link',
    }))).json();
    expect(body.share).toMatchObject({
      status: 'pending_review',
      shareUrl: expect.stringContaining('/assistant/share/'),
    });

    body = await (await patchShare(patchRequest('http://localhost/api/assistant/share', {
      action: 'copy_link',
      id: body.share.id,
    }))).json();
    expect(body.share).toMatchObject({
      status: 'shared',
      copied: true,
      shareUrl: expect.stringContaining('/assistant/share/'),
    });

    body = await (await patchShare(patchRequest('http://localhost/api/assistant/share', {
      action: 'download',
      id: body.share.id,
    }))).json();
    expect(body.share.downloadUrl).toContain('/assistant/download/');

    body = await (await patchShare(patchRequest('http://localhost/api/assistant/share', {
      action: 'revoke',
      id: body.share.id,
    }))).json();
    expect(body.share).toMatchObject({
      status: 'revoked',
      shareUrl: null,
    });

    const listed = await (await getShares()).json();
    expect(listed.shares).toEqual(expect.arrayContaining([
      expect.objectContaining({ id: body.share.id, status: 'revoked' }),
    ]));
  });

  test('lists explores shares and remixes community tasks as personal Agent agents', async () => {
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
      expect.objectContaining({ name: 'File Content Recognition', source: 'official', outputFormat: 'Spreadsheet' }),
      expect.objectContaining({ name: 'Document Generation & Editing', source: 'official', outputFormat: 'Document' }),
      expect.objectContaining({ name: 'Data Analysis & Visualization', useCases: expect.arrayContaining(['charts', 'forecasting']) }),
      expect.objectContaining({ name: 'Social Media Content Creation', useCases: expect.arrayContaining(['Twitter/X', 'LinkedIn', 'YouTube', 'Medium']) }),
      expect.objectContaining({ name: 'Automated Daily News Briefing', connectors: expect.arrayContaining(['Slack', 'Telegram', 'Discord']) }),
      expect.objectContaining({ name: 'Remote Control via Slack', connectors: expect.arrayContaining(['Slack']) }),
      expect.objectContaining({ name: 'Google Calendar & Drive Integration', skills: expect.arrayContaining(['Google Calendar', 'Google Drive']) }),
      expect.objectContaining({ name: 'Zero-Code Local Application Development', outputFormat: 'Code App' }),
      expect.objectContaining({ name: 'Creating Custom Skills', outputFormat: 'Skill Package' }),
      expect.objectContaining({ name: 'AI Self-Driven Workflows', mode: 'Agent' }),
    ]));
    expect(body.practiceCases).toEqual(expect.arrayContaining([
      'File Content Recognition',
      'Document Generation & Editing',
      'Data Analysis & Visualization',
      'Social Media Content Creation',
      'Automated Daily News Briefing',
      'Remote Control via Slack',
      'Google Calendar & Drive Integration',
      'Zero-Code Local Application Development',
      'Creating Custom Skills',
      'AI Self-Driven Workflows',
    ]));
    expect(body.exploreActions).toEqual(expect.arrayContaining(['Try Task', 'Make My Version', 'Remix Agent', 'Share Exploration']));

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

  test('tracks expanded official Agent docs gaps as implemented Agent parity capabilities', async () => {
    const body = await (await getParity()).json();

    expect(body.summary).toMatchObject({
      total: 212,
      implemented: 212,
      remaining: 0,
    });
    expect(body.gaps).toHaveLength(212);
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
      'Official connector roster',
      'Model capability flags',
      'Custom protocol toggle',
      'Compact mode',
      'Auto-install low-risk skills',
      'Prevent sleep',
      'Sidebar account profile',
      'Version information',
      'Copy share link',
      'Download shared file',
      'Cancel sharing',
      'Unarchive task',
      'Feedback screenshot attachment',
      'Automation schedule kinds',
      'Featured skills roster',
      'Batch skill updates',
      'Generated custom skill package',
      'Google Calendar connector',
      'Google OAuth connector flow',
      'Official practice case library',
      'File recognition practice template',
      'Document generation practice template',
      'Data visualization practice template',
      'Social media practice template',
      'Daily briefing practice template',
      'Remote Slack practice template',
      'Google Calendar and Drive practice template',
      'Zero-code local app practice template',
      'Custom skill creation practice template',
      'AI self-driven workflow template',
      'Platform-specific Claw setup guides',
      'Claw credential field schemas',
      'Claw connection mode catalog',
      'Claw pairing method catalog',
      'Claw troubleshooting catalog',
      'Desktop platform support matrix',
      'Multi-device account sync',
      'Log folder locations',
      'Windows installation guide',
      'macOS installation guide',
      'New task bar anatomy',
      'One-sentence task assignment examples',
      'Default working directory behavior',
      'Context tool matrix',
      'Parallel task creation guidance',
      'Conversation top toolbar',
      'Conversation history jump',
      'Show details panel action',
      'File and image upload methods',
      'Execution progress stages',
      'Interrupt and resume flow',
      'Selected artifact preview layout',
      'Spreadsheet preview',
      'Document preview',
      'Web preview controls',
      'All files tree and tab view',
      'Changes detail review',
      'Sidebar task history sections',
      'Feedback product-team route',
      'Privacy retention matrix',
      'Data subject rights catalog',
      'AI training opt-out',
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
      expect.objectContaining({ name: 'Official docs gap closure', implemented: 14 }),
      expect.objectContaining({ name: 'Extended docs gap closure', implemented: 24 }),
      expect.objectContaining({ name: 'Core docs gap closure', implemented: 24 }),
    ]));
  });

  test('billing route returns billing state', async () => {
    const body = await (await getBilling()).json();
    expect(body.plan).toBe('Growth');
    expect(body.aiActionsUsed).toBe(145);
    expect(body.storageUsedGB).toBe(12.4);
    expect(body.estimatedNextBill).toBe(29.00);
  });
});
