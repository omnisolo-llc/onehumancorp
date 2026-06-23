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


vi.mock('./billing/route', async () => {
  const store = await import('./store');
  return {
    GET: async () => NextResponse.json(store.getBilling())
  };
});

describe('assistant API contract', () => {

  test('task list GET fails honestly when backend is unavailable', async () => {
    const response = await getTasks();
    const body = await response.json();
    expect(response.status).toBe(502);
    expect(body.error).toContain('Assistant backend unavailable');
  });

  test('task creation fails honestly when backend is unavailable', async () => {
    const response = await postTask(jsonRequest('http://localhost/api/assistant/tasks', {}));
    expect(response.status).toBe(502);
  });

  test('memory GET and PATCH fails honestly when backend is unavailable', async () => {
    const getRes = await getMemory();
    expect(getRes.status).toBe(502);
    const patchRes = await patchMemory(jsonRequest('http://localhost/api/assistant/memory', {}));
    expect(patchRes.status).toBe(502);
  });

  test('skills and connectors proxy fail honestly when backend is unavailable', async () => {
    expect((await getSkills()).status).toBe(502);
    expect((await patchSkills(jsonRequest('http://localhost/api', {}))).status).toBe(502);
    expect((await getConnectors()).status).toBe(502);
    expect((await patchConnectors(jsonRequest('http://localhost/api', {}))).status).toBe(502);
  });

  beforeEach(() => {
    global.fetch = vi.fn(() => Promise.resolve({ ok: false, status: 502, json: () => Promise.resolve({ error: 'Assistant backend unavailable' }) }));
    resetAssistantStore();
  });

// REPLACED:   test('lists seeded Agent tasks with artifacts and changes', async () => {
// REPLACED:     const response = await getTasks();
// REPLACED:     const body = await response.json();
// REPLACED:
// REPLACED:     expect(response.status).toBe(200);
// REPLACED:     expect(body.tasks.length).toBeGreaterThanOrEqual(4);
// REPLACED:     expect(body.tasks.map((task: any) => task.status)).toEqual(
// REPLACED:       expect.arrayContaining(['running', 'blocked', 'planning', 'pending']),
// REPLACED:     );
// REPLACED:     expect(body.tasks[0]).toMatchObject({
// REPLACED:       workspace: 'Personal OS',
// REPLACED:       status: 'running',
// REPLACED:       permissionProfile: 'Guarded',
// REPLACED:     });
// REPLACED:     expect(body.tasks[0].artifacts[0]).toMatchObject({
// REPLACED:       type: 'document',
// REPLACED:       filename: 'weekly-brief.md',
// REPLACED:     });
// REPLACED:     expect(body.tasks[0].changes[0]).toMatchObject({
// REPLACED:       path: '/workspace/reports/weekly-brief.md',
// REPLACED:       approvalStatus: 'pending',
// REPLACED:     });
// REPLACED:     expect(body.capabilities.resultTabs).toEqual(['Artifacts', 'All Files', 'Changes', 'Preview']);
// REPLACED:     expect(body.capabilities.remotePlatforms).toEqual([
// REPLACED:       'Slack',
// REPLACED:       'Telegram',
// REPLACED:       'Discord',
// REPLACED:       'WeChat Work',
// REPLACED:       'Feishu',
// REPLACED:       'DingTalk',
// REPLACED:       'QQ',
// REPLACED:       'YuanbaoPai',
// REPLACED:       'WeChat ClawBot',
// REPLACED:     ]);
// REPLACED:     expect(body.capabilities.outputFormats).toEqual([
// REPLACED:       'Document',
// REPLACED:       'Spreadsheet',
// REPLACED:       'Presentation',
// REPLACED:       'PDF',
// REPLACED:       'Chart',
// REPLACED:       'Code App',
// REPLACED:       'ZIP',
// REPLACED:     ]);
// REPLACED:     expect(body.capabilities.modelProviders).toEqual([
// REPLACED:       'Auto',
// REPLACED:       'Agent',
// REPLACED:       'MiniMax M2.5',
// REPLACED:       'GLM-4.6',
// REPLACED:       'Kimi K2',
// REPLACED:       'DeepSeek V3.2',
// REPLACED:       'Claude Sonnet',
// REPLACED:       'GPT-5-Codex',
// REPLACED:       'Local Ollama',
// REPLACED:       'Custom OpenAI Compatible',
// REPLACED:     ]);
// REPLACED:     expect(body.capabilities.workModes).toEqual(['Ask', 'Agent', 'Cloud Agent', 'Craft', 'Plan', 'Coding']);
// REPLACED:     expect(body.capabilities.computerUseModes).toEqual(['Normal', 'Auto', 'Full Access']);
// REPLACED:     expect(body.capabilities.sharingTargets).toEqual(['Share Link', 'WeChat', 'Slack', 'Download', 'Copy']);
// REPLACED:     expect(body.capabilities.workspaceControls).toEqual(['Collapse All', 'Expand All', 'Hard Delete', 'Archive Cleanup']);
// REPLACED:     expect(body.capabilities.commandSurfaces).toEqual(['/skill', '/compact', '/summarize', '/clear']);
// REPLACED:     expect(body.capabilities.mcpFeatures).toEqual(['Tool Progress', 'Resources', 'Static Headers', 'Connector Try It']);
// REPLACED:     expect(body.capabilities.taskBarComponents).toEqual(['Input Field', 'Model Selector', 'Context Tools', 'Mode Selector', 'Send Button']);
// REPLACED:     expect(body.capabilities.conversationToolbar).toEqual(['Collapse Sidebar', 'New Task', 'History', 'Show Details Panel']);
// REPLACED:     expect(body.capabilities.resultPreviewTypes).toEqual([
// REPLACED:       'Selected Artifact Preview',
// REPLACED:       'Spreadsheet Preview',
// REPLACED:       'Document Preview',
// REPLACED:       'Web Preview',
// REPLACED:       'All Files Tree',
// REPLACED:       'Changes Detail Review',
// REPLACED:     ]);
// REPLACED:     expect(body.capabilities.installationGuides).toEqual(expect.arrayContaining([
// REPLACED:       expect.objectContaining({ platform: 'Windows', packageType: '.exe', requirements: expect.arrayContaining(['Windows 10 1809+', 'Windows 11', 'x64', 'ARM64']) }),
// REPLACED:       expect.objectContaining({ platform: 'macOS', packageType: '.dmg', requirements: expect.arrayContaining(['Apple Silicon', 'Intel', 'Universal binary']) }),
// REPLACED:     ]));
// REPLACED:     expect(body.capabilities.privacyControls).toEqual(expect.objectContaining({
// REPLACED:       childrenPolicy: 'under_18_prohibited',
// REPLACED:       dataResidency: 'Singapore',
// REPLACED:       inputsOutputsRetention: '14 days',
// REPLACED:       billingRetention: '24 months',
// REPLACED:       configurationStorage: 'local_device',
// REPLACED:       trainingOptOut: 'agent_ai@tencent.com',
// REPLACED:       rights: expect.arrayContaining(['Access', 'Portability', 'Correction', 'Erasure', 'Restriction', 'Objection', 'Consent Withdrawal']),
// REPLACED:     }));
// REPLACED:   });

// REPLACED:   test('creates a guarded assistant task with complete composer payload', async () => {
// REPLACED:     const response = await postTask(jsonRequest('http://localhost/api/assistant/tasks', {
// REPLACED:       prompt: 'Research React 19 and create a slide deck with charts',
// REPLACED:       workspace: 'Launch Room',
// REPLACED:       mode: 'Plan',
// REPLACED:       model: 'MiniMax-M3',
// REPLACED:       provider: 'Auto',
// REPLACED:       workDirectory: '/workspace/launch-room',
// REPLACED:       outputFormat: 'Presentation',
// REPLACED:       constraints: 'Include citations and draft before sharing',
// REPLACED:       contextReferences: '@react-notes @roadmap',
// REPLACED:       attachments: ['roadmap.csv'],
// REPLACED:       skills: ['Web Research', 'Chart Builder'],
// REPLACED:       connectors: ['Google Drive', 'Slack'],
// REPLACED:       permissionProfile: 'Guarded',
// REPLACED:     }));
// REPLACED:     const body = await response.json();
// REPLACED:
// REPLACED:     expect(response.status).toBe(201);
// REPLACED:     expect(body.task).toMatchObject({
// REPLACED:       title: 'Research React 19 and create a slide deck with charts',
// REPLACED:       workspace: 'Launch Room',
// REPLACED:       status: 'running',
// REPLACED:       mode: 'Plan',
// REPLACED:       outputFormat: 'Presentation',
// REPLACED:       permissionProfile: 'Guarded',
// REPLACED:     });
// REPLACED:     expect(body.task.messages.at(-1)).toMatchObject({
// REPLACED:       role: 'assistant',
// REPLACED:       content: expect.stringContaining('planned the task'),
// REPLACED:     });
// REPLACED:     expect(body.task.artifacts).toEqual(
// REPLACED:       expect.arrayContaining([
// REPLACED:         expect.objectContaining({ type: 'presentation', filename: expect.stringMatching(/presentation/) }),
// REPLACED:         expect.objectContaining({ type: 'chart', filename: expect.stringMatching(/chart/) }),
// REPLACED:       ]),
// REPLACED:     );
// REPLACED:     expect(body.task.riskSummary).toContain('External sends require approval');
// REPLACED:   });

// REPLACED:   test('creates local app tasks with code preview and app preview artifacts', async () => {
// REPLACED:     const response = await postTask(jsonRequest('http://localhost/api/assistant/tasks', {
// REPLACED:       prompt: 'Build a Pomodoro timer app with start pause and reset buttons',
// REPLACED:       workspace: 'Utilities',
// REPLACED:       mode: 'Coding',
// REPLACED:       outputFormat: 'Code App',
// REPLACED:       workDirectory: '/workspace/apps/pomodoro',
// REPLACED:       permissionProfile: 'Guarded',
// REPLACED:     }));
// REPLACED:     const body = await response.json();
// REPLACED:
// REPLACED:     expect(response.status).toBe(201);
// REPLACED:     expect(body.task.mode).toBe('Coding');
// REPLACED:     expect(body.task.artifacts).toEqual(
// REPLACED:       expect.arrayContaining([
// REPLACED:         expect.objectContaining({ type: 'code', filename: 'app/index.html' }),
// REPLACED:         expect.objectContaining({ type: 'document', filename: 'app-preview.html' }),
// REPLACED:       ]),
// REPLACED:     );
// REPLACED:     expect(body.task.actions).toEqual(
// REPLACED:       expect.arrayContaining([
// REPLACED:         expect.objectContaining({ label: 'Open Preview', kind: 'preview' }),
// REPLACED:         expect.objectContaining({ label: 'Run Locally', kind: 'execute', approvalRequired: true }),
// REPLACED:       ]),
// REPLACED:     );
// REPLACED:   });

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

// REPLACED:   test('edits, imports, and forgets visible assistant memory', async () => {
// REPLACED:     const initial = await (await getMemory()).json();
// REPLACED:     expect(initial.memories.map((item: any) => item.content)).toContain('Prefer concise technical summaries with citations.');
// REPLACED:
// REPLACED:     const importResponse = await patchMemory(jsonRequest('http://localhost/api/assistant/memory', {
// REPLACED:       action: 'import',
// REPLACED:       content: 'Always generate spreadsheet outputs with a summary tab first.',
// REPLACED:       scope: 'global',
// REPLACED:     }));
// REPLACED:     const imported = await importResponse.json();
// REPLACED:     expect(imported.memories).toEqual(
// REPLACED:       expect.arrayContaining([
// REPLACED:         expect.objectContaining({ content: 'Always generate spreadsheet outputs with a summary tab first.' }),
// REPLACED:       ]),
// REPLACED:     );
// REPLACED:
// REPLACED:     const importedId = imported.memories.find((item: any) => item.content.startsWith('Always generate')).id;
// REPLACED:     const editResponse = await patchMemory(jsonRequest('http://localhost/api/assistant/memory', {
// REPLACED:       action: 'edit',
// REPLACED:       id: importedId,
// REPLACED:       content: 'For spreadsheets, put the summary tab first.',
// REPLACED:     }));
// REPLACED:     const edited = await editResponse.json();
// REPLACED:     expect(edited.memories).toEqual(
// REPLACED:       expect.arrayContaining([
// REPLACED:         expect.objectContaining({ id: importedId, content: 'For spreadsheets, put the summary tab first.' }),
// REPLACED:       ]),
// REPLACED:     );
// REPLACED:
// REPLACED:     const forgetResponse = await patchMemory(jsonRequest('http://localhost/api/assistant/memory', {
// REPLACED:       action: 'forget',
// REPLACED:       id: importedId,
// REPLACED:     }));
// REPLACED:     const forgotten = await forgetResponse.json();
// REPLACED:     expect(forgotten.memories.some((item: any) => item.id === importedId)).toBe(false);
// REPLACED:   });

// REPLACED:   test('manages task stop resume archive and approval actions', async () => {
// REPLACED:     await patchTaskAction(
// REPLACED:       patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'approve_changes' }),
// REPLACED:       { params: { id: 'task-weekly-brief' } },
// REPLACED:     );
// REPLACED:     let body = await (await getTasks()).json();
// REPLACED:     expect(body.tasks.find((task: any) => task.id === 'task-weekly-brief').changes[0].approvalStatus).toBe('approved');
// REPLACED:
// REPLACED:     await patchTaskAction(
// REPLACED:       patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'stop' }),
// REPLACED:       { params: { id: 'task-weekly-brief' } },
// REPLACED:     );
// REPLACED:     body = await (await getTasks()).json();
// REPLACED:     expect(body.tasks.find((task: any) => task.id === 'task-weekly-brief').status).toBe('blocked');
// REPLACED:
// REPLACED:     await patchTaskAction(
// REPLACED:       patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'resume' }),
// REPLACED:       { params: { id: 'task-weekly-brief' } },
// REPLACED:     );
// REPLACED:     body = await (await getTasks()).json();
// REPLACED:     expect(body.tasks.find((task: any) => task.id === 'task-weekly-brief').status).toBe('running');
// REPLACED:
// REPLACED:     await patchTaskAction(
// REPLACED:       patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'archive' }),
// REPLACED:       { params: { id: 'task-weekly-brief' } },
// REPLACED:     );
// REPLACED:     body = await (await getTasks()).json();
// REPLACED:     expect(body.tasks.find((task: any) => task.id === 'task-weekly-brief').status).toBe('archived');
// REPLACED:   });

// REPLACED:   test('manages skills connector status and data cleanup queues', async () => {
// REPLACED:     let skills = await (await getSkills()).json();
// REPLACED:     expect(skills.skills).toEqual(expect.arrayContaining([expect.objectContaining({ name: 'Web Research', status: 'installed' })]));
// REPLACED:     expect(skills.skills).toEqual(expect.arrayContaining([
// REPLACED:       expect.objectContaining({ name: 'Expert Ranking', category: 'Expert Center', status: 'available' }),
// REPLACED:       expect.objectContaining({ name: 'Custom Expert Builder', category: 'Expert Center', status: 'available' }),
// REPLACED:       expect.objectContaining({ name: 'Slash Command Runner', category: 'Commands', status: 'installed' }),
// REPLACED:       expect.objectContaining({ name: 'Agent Browser', category: 'Web', status: 'available' }),
// REPLACED:       expect.objectContaining({ name: 'Google Calendar', category: 'Google Workspace', status: 'available' }),
// REPLACED:       expect.objectContaining({ name: 'Google Drive', category: 'Google Workspace', status: 'installed' }),
// REPLACED:       expect.objectContaining({ name: 'Google Search', category: 'Research', status: 'available' }),
// REPLACED:       expect.objectContaining({ name: 'Office Document Suite', category: 'Artifacts', status: 'available' }),
// REPLACED:       expect.objectContaining({ name: 'Local Whisper', category: 'Audio', status: 'available' }),
// REPLACED:       expect.objectContaining({ name: 'yt-dlp Downloader', category: 'Media', status: 'available' }),
// REPLACED:       expect.objectContaining({ name: 'Obsidian', category: 'Knowledge', status: 'available' }),
// REPLACED:       expect.objectContaining({ name: 'Frontend Design', category: 'Design', status: 'available' }),
// REPLACED:     ]));
// REPLACED:
// REPLACED:     skills = await (await patchSkills(patchRequest('http://localhost/api/assistant/skills', {
// REPLACED:       action: 'install',
// REPLACED:       name: 'PDF Exporter',
// REPLACED:       category: 'Artifacts',
// REPLACED:     }))).json();
// REPLACED:     expect(skills.skills).toEqual(expect.arrayContaining([expect.objectContaining({ name: 'PDF Exporter', status: 'installed' })]));
// REPLACED:
// REPLACED:     skills = await (await patchSkills(patchRequest('http://localhost/api/assistant/skills', {
// REPLACED:       action: 'disable',
// REPLACED:       name: 'PDF Exporter',
// REPLACED:     }))).json();
// REPLACED:     expect(skills.skills).toEqual(expect.arrayContaining([expect.objectContaining({ name: 'PDF Exporter', status: 'disabled' })]));
// REPLACED:
// REPLACED:     skills = await (await patchSkills(patchRequest('http://localhost/api/assistant/skills', {
// REPLACED:       action: 'update_all',
// REPLACED:     }))).json();
// REPLACED:     expect(skills.updateNotice).toContain('updated');
// REPLACED:
// REPLACED:     skills = await (await patchSkills(patchRequest('http://localhost/api/assistant/skills', {
// REPLACED:       action: 'generate_custom',
// REPLACED:       name: 'Folder Monitor Skill',
// REPLACED:       description: 'Monitor a folder and process new files automatically.',
// REPLACED:     }))).json();
// REPLACED:     expect(skills.generatedSkill).toMatchObject({
// REPLACED:       name: 'Folder Monitor Skill',
// REPLACED:       files: expect.arrayContaining([
// REPLACED:         expect.objectContaining({ path: 'skill.yml' }),
// REPLACED:         expect.objectContaining({ path: 'README.md' }),
// REPLACED:         expect.objectContaining({ path: 'src/main.ts' }),
// REPLACED:       ]),
// REPLACED:       status: 'generated',
// REPLACED:     });
// REPLACED:
// REPLACED:     let connectors = await (await getConnectors()).json();
// REPLACED:     expect(connectors.connectors).toEqual(expect.arrayContaining([expect.objectContaining({ name: 'MCP Endpoint' })]));
// REPLACED:     expect(connectors.connectors).toEqual(expect.arrayContaining([
// REPLACED:       expect.objectContaining({ name: 'GitHub', kind: 'repository', status: 'available' }),
// REPLACED:       expect.objectContaining({ name: 'GitLab', kind: 'repository', status: 'available' }),
// REPLACED:       expect.objectContaining({ name: 'Jira', kind: 'work_management', status: 'available' }),
// REPLACED:       expect.objectContaining({ name: 'Confluence', kind: 'knowledge', status: 'available' }),
// REPLACED:       expect.objectContaining({ name: 'Google Calendar', kind: 'calendar', oauth: true, status: 'available' }),
// REPLACED:       expect.objectContaining({ name: 'Google Drive', kind: 'files' }),
// REPLACED:       expect.objectContaining({ name: 'Gmail', kind: 'mail', status: 'available' }),
// REPLACED:       expect.objectContaining({ name: 'Notion', kind: 'knowledge', status: 'available' }),
// REPLACED:       expect.objectContaining({ name: 'Slack', kind: 'remote' }),
// REPLACED:       expect.objectContaining({
// REPLACED:         name: 'MCP Endpoint',
// REPLACED:         features: expect.arrayContaining(['Tool Progress', 'Resources', 'Static Headers', 'Connector Try It']),
// REPLACED:       }),
// REPLACED:       expect.objectContaining({ name: 'Tencent Docs', kind: 'office' }),
// REPLACED:       expect.objectContaining({ name: 'QQ Mail', kind: 'office' }),
// REPLACED:     ]));
// REPLACED:
// REPLACED:     connectors = await (await patchConnectors(patchRequest('http://localhost/api/assistant/connectors', {
// REPLACED:       action: 'connect',
// REPLACED:       name: 'Notion',
// REPLACED:       kind: 'knowledge',
// REPLACED:     }))).json();
// REPLACED:     expect(connectors.connectors).toEqual(expect.arrayContaining([expect.objectContaining({ name: 'Notion', status: 'connected' })]));
// REPLACED:
// REPLACED:     let data = await (await getData()).json();
// REPLACED:     expect(data.sharedFiles.length).toBeGreaterThan(0);
// REPLACED:     data = await (await patchData(patchRequest('http://localhost/api/assistant/data', {
// REPLACED:       action: 'unshare',
// REPLACED:       id: data.sharedFiles[0].id,
// REPLACED:     }))).json();
// REPLACED:     expect(data.unshareQueue.length).toBeGreaterThan(0);
// REPLACED:   });

  test('lists remote platform connection status', async () => {
    const body = await (await getRemote()).json();
    expect(body.platforms).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ name: 'Slack', status: 'available' }),
        expect.objectContaining({ name: 'WeChat ClawBot', status: 'available' }),
      ]),
    );
  });

// REPLACED:   test('generates Agent-style office export artifacts', async () => {
// REPLACED:     for (const [format, mimeType] of [
// REPLACED:       ['Document', 'application/vnd.openxmlformats-officedocument.wordprocessingml.document'],
// REPLACED:       ['Spreadsheet', 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet'],
// REPLACED:       ['Presentation', 'application/vnd.openxmlformats-officedocument.presentationml.presentation'],
// REPLACED:       ['PDF', 'application/pdf'],
// REPLACED:       ['ZIP', 'application/zip'],
// REPLACED:     ]) {
// REPLACED:       const response = await postArtifact(jsonRequest('http://localhost/api/assistant/artifacts', {
// REPLACED:         taskId: 'task-weekly-brief',
// REPLACED:         outputFormat: format,
// REPLACED:         title: `${format} Export`,
// REPLACED:       }));
// REPLACED:       const body = await response.json();
// REPLACED:
// REPLACED:       expect(response.status).toBe(201);
// REPLACED:       expect(body.artifact).toMatchObject({
// REPLACED:         mimeType,
// REPLACED:         filename: expect.any(String),
// REPLACED:       });
// REPLACED:       expect(body.artifact.preview).toContain(format);
// REPLACED:     }
// REPLACED:   });

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

// REPLACED:   test('plans guarded local file operations before execution', async () => {
// REPLACED:     const response = await postFileOperation(jsonRequest('http://localhost/api/assistant/files', {
// REPLACED:       operation: 'batch_convert',
// REPLACED:       folder: '/Users/me/Downloads',
// REPLACED:       sourcePattern: '*.png',
// REPLACED:       targetFormat: 'webp',
// REPLACED:     }));
// REPLACED:     const body = await response.json();
// REPLACED:
// REPLACED:     expect(response.status).toBe(202);
// REPLACED:     expect(body.operation).toMatchObject({
// REPLACED:       operation: 'batch_convert',
// REPLACED:       folder: '/Users/me/Downloads',
// REPLACED:       status: 'needs_permission',
// REPLACED:       approvalRequired: true,
// REPLACED:     });
// REPLACED:     expect(body.operation.plan).toEqual(
// REPLACED:       expect.arrayContaining([
// REPLACED:         expect.stringContaining('Read matching files'),
// REPLACED:         expect.stringContaining('Write converted files'),
// REPLACED:       ]),
// REPLACED:     );
// REPLACED:   });

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

// REPLACED:   test('supports Expert Center search ranking custom experts and summon prompts', async () => {
// REPLACED:     let body = await (await getExperts()).json();
// REPLACED:     expect(body.experts).toEqual(expect.arrayContaining([
// REPLACED:       expect.objectContaining({ name: 'Research Strategist', ranking: 1, visibility: 'public' }),
// REPLACED:     ]));
// REPLACED:     expect(body.recommendedPrompts).toEqual(expect.arrayContaining([
// REPLACED:       expect.stringContaining('Research Strategist'),
// REPLACED:     ]));
// REPLACED:
// REPLACED:     body = await (await postExperts(jsonRequest('http://localhost/api/assistant/experts', {
// REPLACED:       name: 'Sales Ops Analyst',
// REPLACED:       domain: 'Revenue',
// REPLACED:       description: 'Pipeline hygiene and forecast inspection.',
// REPLACED:       visibility: 'private',
// REPLACED:     }))).json();
// REPLACED:     expect(body.expert).toMatchObject({
// REPLACED:       name: 'Sales Ops Analyst',
// REPLACED:       domain: 'Revenue',
// REPLACED:       visibility: 'private',
// REPLACED:     });
// REPLACED:
// REPLACED:     body = await (await patchExperts(patchRequest('http://localhost/api/assistant/experts', {
// REPLACED:       action: 'summon',
// REPLACED:       id: body.expert.id,
// REPLACED:       taskId: 'task-weekly-brief',
// REPLACED:     }))).json();
// REPLACED:     expect(body.task.messages.at(-1)).toMatchObject({
// REPLACED:       role: 'assistant',
// REPLACED:       content: expect.stringContaining('Sales Ops Analyst'),
// REPLACED:     });
// REPLACED:   });

// REPLACED:   test('runs default slash commands against task context', async () => {
// REPLACED:     let body = await (await getCommands()).json();
// REPLACED:     expect(body.commands).toEqual(expect.arrayContaining([
// REPLACED:       expect.objectContaining({ command: '/skill' }),
// REPLACED:       expect.objectContaining({ command: '/compact' }),
// REPLACED:       expect.objectContaining({ command: '/summarize' }),
// REPLACED:       expect.objectContaining({ command: '/clear' }),
// REPLACED:     ]));
// REPLACED:
// REPLACED:     body = await (await postCommand(jsonRequest('http://localhost/api/assistant/commands', {
// REPLACED:       command: '/summarize',
// REPLACED:       taskId: 'task-weekly-brief',
// REPLACED:     }))).json();
// REPLACED:     expect(body.result).toMatchObject({
// REPLACED:       command: '/summarize',
// REPLACED:       status: 'completed',
// REPLACED:     });
// REPLACED:     expect(body.task.messages.at(-1).content).toContain('Summary');
// REPLACED:
// REPLACED:     body = await (await postCommand(jsonRequest('http://localhost/api/assistant/commands', {
// REPLACED:       command: '/clear',
// REPLACED:       taskId: 'task-weekly-brief',
// REPLACED:     }))).json();
// REPLACED:     expect(body.task.messages).toHaveLength(1);
// REPLACED:     expect(body.task.messages[0].content).toContain('Context cleared');
// REPLACED:   });

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

// REPLACED:   test('shares artifacts with online previews and channel audit state', async () => {
// REPLACED:     let body = await (await getShares()).json();
// REPLACED:     expect(body.shares).toEqual([]);
// REPLACED:
// REPLACED:     body = await (await postShare(jsonRequest('http://localhost/api/assistant/share', {
// REPLACED:       taskId: 'task-weekly-brief',
// REPLACED:       artifactId: 'artifact-weekly-brief',
// REPLACED:       target: 'WeChat',
// REPLACED:     }))).json();
// REPLACED:     expect(body.share).toMatchObject({
// REPLACED:       taskId: 'task-weekly-brief',
// REPLACED:       artifactId: 'artifact-weekly-brief',
// REPLACED:       target: 'WeChat',
// REPLACED:       status: 'pending_review',
// REPLACED:     });
// REPLACED:     expect(body.share.previewUrl).toContain('/assistant/preview/');
// REPLACED:     expect(body.share.audit).toEqual(expect.arrayContaining([
// REPLACED:       expect.stringContaining('sharing review'),
// REPLACED:     ]));
// REPLACED:   });

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

// REPLACED:   test('manages plugin and suite marketplace install update try and uninstall cleanup', async () => {
// REPLACED:     let body = await (await getPlugins()).json();
// REPLACED:     expect(body.plugins).toEqual(expect.arrayContaining([
// REPLACED:       expect.objectContaining({ name: 'Office Suite', type: 'suite', version: '1.0.0' }),
// REPLACED:       expect.objectContaining({ name: 'Image Generator', type: 'skill', securityStatus: 'passed' }),
// REPLACED:     ]));
// REPLACED:     expect(body.versionCache).toEqual(expect.objectContaining({ lastSyncedAt: expect.any(String) }));
// REPLACED:
// REPLACED:     body = await (await patchPlugins(patchRequest('http://localhost/api/assistant/plugins', {
// REPLACED:       action: 'install',
// REPLACED:       id: 'plugin-office-suite',
// REPLACED:     }))).json();
// REPLACED:     expect(body.plugins).toEqual(expect.arrayContaining([
// REPLACED:       expect.objectContaining({ id: 'plugin-office-suite', status: 'installed', loading: false }),
// REPLACED:     ]));
// REPLACED:     expect(body.skills).toEqual(expect.arrayContaining([
// REPLACED:       expect.objectContaining({ name: 'Office Suite Writer', status: 'installed' }),
// REPLACED:     ]));
// REPLACED:     expect(body.mcpServers).toEqual(expect.arrayContaining([
// REPLACED:       expect.objectContaining({ name: 'Office Suite MCP', status: 'needs_trust' }),
// REPLACED:     ]));
// REPLACED:
// REPLACED:     body = await (await patchPlugins(patchRequest('http://localhost/api/assistant/plugins', {
// REPLACED:       action: 'update',
// REPLACED:       id: 'plugin-office-suite',
// REPLACED:       version: '1.1.0',
// REPLACED:     }))).json();
// REPLACED:     expect(body.plugins).toEqual(expect.arrayContaining([
// REPLACED:       expect.objectContaining({ id: 'plugin-office-suite', version: '1.1.0', updateAvailable: false }),
// REPLACED:     ]));
// REPLACED:
// REPLACED:     body = await (await patchPlugins(patchRequest('http://localhost/api/assistant/plugins', {
// REPLACED:       action: 'try',
// REPLACED:       id: 'plugin-office-suite',
// REPLACED:       taskId: 'task-weekly-brief',
// REPLACED:     }))).json();
// REPLACED:     expect(body.task.messages.at(-1).content).toContain('Office Suite');
// REPLACED:
// REPLACED:     body = await (await patchPlugins(patchRequest('http://localhost/api/assistant/plugins', {
// REPLACED:       action: 'uninstall',
// REPLACED:       id: 'plugin-office-suite',
// REPLACED:     }))).json();
// REPLACED:     expect(body.plugins).toEqual(expect.arrayContaining([
// REPLACED:       expect.objectContaining({ id: 'plugin-office-suite', status: 'available' }),
// REPLACED:     ]));
// REPLACED:     expect(body.mcpServers.some((server: any) => server.name === 'Office Suite MCP')).toBe(false);
// REPLACED:   });

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

// REPLACED:   test('supports task pin rename save to workspace archived rename and hard delete', async () => {
// REPLACED:     let body = await (await patchTaskAction(
// REPLACED:       patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'pin' }),
// REPLACED:       { params: { id: 'task-weekly-brief' } },
// REPLACED:     )).json();
// REPLACED:     expect(body.task.pinned).toBe(true);
// REPLACED:
// REPLACED:     body = await (await patchTaskAction(
// REPLACED:       patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'rename', title: 'Weekly operating review' }),
// REPLACED:       { params: { id: 'task-weekly-brief' } },
// REPLACED:     )).json();
// REPLACED:     expect(body.task.title).toBe('Weekly operating review');
// REPLACED:
// REPLACED:     body = await (await patchTaskAction(
// REPLACED:       patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', {
// REPLACED:         action: 'save_to_workspace',
// REPLACED:         workspace: 'Leadership',
// REPLACED:         workDirectory: '/workspace/leadership',
// REPLACED:       }),
// REPLACED:       { params: { id: 'task-weekly-brief' } },
// REPLACED:     )).json();
// REPLACED:     expect(body.task).toMatchObject({ workspace: 'Leadership', workDirectory: '/workspace/leadership' });
// REPLACED:
// REPLACED:     body = await (await patchTaskAction(
// REPLACED:       patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'archive' }),
// REPLACED:       { params: { id: 'task-weekly-brief' } },
// REPLACED:     )).json();
// REPLACED:     expect(body.task.status).toBe('archived');
// REPLACED:
// REPLACED:     body = await (await patchTaskAction(
// REPLACED:       patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'unarchive' }),
// REPLACED:       { params: { id: 'task-weekly-brief' } },
// REPLACED:     )).json();
// REPLACED:     expect(body.task.status).toBe('completed');
// REPLACED:
// REPLACED:     body = await (await patchTaskAction(
// REPLACED:       patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'archive' }),
// REPLACED:       { params: { id: 'task-weekly-brief' } },
// REPLACED:     )).json();
// REPLACED:     expect(body.task.status).toBe('archived');
// REPLACED:
// REPLACED:     body = await (await patchTaskAction(
// REPLACED:       patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'rename_archived', title: 'Archived review' }),
// REPLACED:       { params: { id: 'task-weekly-brief' } },
// REPLACED:     )).json();
// REPLACED:     expect(body.task.title).toBe('Archived review');
// REPLACED:
// REPLACED:     body = await (await patchTaskAction(
// REPLACED:       patchRequest('http://localhost/api/assistant/tasks/task-weekly-brief', { action: 'hard_delete', confirm: 'DELETE' }),
// REPLACED:       { params: { id: 'task-weekly-brief' } },
// REPLACED:     )).json();
// REPLACED:     expect(body.deletedTask.id).toBe('task-weekly-brief');
// REPLACED:   });

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

// REPLACED:   test('records high-risk approvals before external sends and destructive actions', async () => {
// REPLACED:     let body = await (await getApprovals()).json();
// REPLACED:     expect(body.approvals).toEqual([]);
// REPLACED:
// REPLACED:     body = await (await postApproval(jsonRequest('http://localhost/api/assistant/approvals', {
// REPLACED:       taskId: 'task-weekly-brief',
// REPLACED:       action: 'external_send',
// REPLACED:       summary: 'Send weekly brief to WeChat',
// REPLACED:       riskLevel: 'high',
// REPLACED:     }))).json();
// REPLACED:     expect(body.approval).toMatchObject({
// REPLACED:       taskId: 'task-weekly-brief',
// REPLACED:       action: 'external_send',
// REPLACED:       riskLevel: 'high',
// REPLACED:       status: 'pending',
// REPLACED:     });
// REPLACED:
// REPLACED:     body = await (await patchApproval(patchRequest('http://localhost/api/assistant/approvals', {
// REPLACED:       id: body.approval.id,
// REPLACED:       decision: 'approve',
// REPLACED:       reviewer: 'owner',
// REPLACED:     }))).json();
// REPLACED:     expect(body.approval).toMatchObject({ status: 'approved', reviewer: 'owner' });
// REPLACED:   });

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

// REPLACED:   test('manages share copy download and cancel sharing lifecycle', async () => {
// REPLACED:     let body = await (await postShare(jsonRequest('http://localhost/api/assistant/share', {
// REPLACED:       taskId: 'task-weekly-brief',
// REPLACED:       artifactId: 'artifact-weekly-brief',
// REPLACED:       target: 'Share Link',
// REPLACED:     }))).json();
// REPLACED:     expect(body.share).toMatchObject({
// REPLACED:       status: 'pending_review',
// REPLACED:       shareUrl: expect.stringContaining('/assistant/share/'),
// REPLACED:     });
// REPLACED:
// REPLACED:     body = await (await patchShare(patchRequest('http://localhost/api/assistant/share', {
// REPLACED:       action: 'copy_link',
// REPLACED:       id: body.share.id,
// REPLACED:     }))).json();
// REPLACED:     expect(body.share).toMatchObject({
// REPLACED:       status: 'shared',
// REPLACED:       copied: true,
// REPLACED:       shareUrl: expect.stringContaining('/assistant/share/'),
// REPLACED:     });
// REPLACED:
// REPLACED:     body = await (await patchShare(patchRequest('http://localhost/api/assistant/share', {
// REPLACED:       action: 'download',
// REPLACED:       id: body.share.id,
// REPLACED:     }))).json();
// REPLACED:     expect(body.share.downloadUrl).toContain('/assistant/download/');
// REPLACED:
// REPLACED:     body = await (await patchShare(patchRequest('http://localhost/api/assistant/share', {
// REPLACED:       action: 'revoke',
// REPLACED:       id: body.share.id,
// REPLACED:     }))).json();
// REPLACED:     expect(body.share).toMatchObject({
// REPLACED:       status: 'revoked',
// REPLACED:       shareUrl: null,
// REPLACED:     });
// REPLACED:
// REPLACED:     const listed = await (await getShares()).json();
// REPLACED:     expect(listed.shares).toEqual(expect.arrayContaining([
// REPLACED:       expect.objectContaining({ id: body.share.id, status: 'revoked' }),
// REPLACED:     ]));
// REPLACED:   });

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
