import { describe, expect, test, beforeEach } from 'vitest';
import { GET as getTasks, POST as postTask } from './tasks/route';
import { POST as postRemote, GET as getRemote } from './remote/route';
import { POST as postAutomation, GET as getAutomations } from './automations/route';
import { GET as getMemory, PATCH as patchMemory } from './memory/route';
import { PATCH as patchTaskAction } from './tasks/[id]/route';
import { GET as getSkills, PATCH as patchSkills } from './skills/route';
import { GET as getConnectors, PATCH as patchConnectors } from './connectors/route';
import { GET as getData, PATCH as patchData } from './data/route';
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
});
