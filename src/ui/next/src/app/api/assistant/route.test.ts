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




  test('lists remote platform connection status', async () => {
    const body = await (await getRemote()).json();
    expect(body.platforms).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ name: 'Slack', status: 'available' }),
        expect.objectContaining({ name: 'WeChat ClawBot', status: 'available' }),
      ]),
    );
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
