import { NextResponse } from 'next/server';
import { createAssistantTask, getAssistantCapabilities, listAssistantTasks } from '../store';

export async function GET(request?: Request) {
  try {
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const tenantId = request?.headers?.get('x-tenant-id') || 'storefront';
    
    const headers: Record<string, string> = {
      'x-tenant-id': tenantId,
    };
    const authHeader = request?.headers?.get('Authorization');
    if (authHeader) {
      headers['Authorization'] = authHeader;
    }

    const tasksRes = await fetch(`${backendUrl}/api/assistant/tasks`, { headers });
    if (tasksRes.ok) {
      const dbTasks = await tasksRes.json();
      
      const tasks = await Promise.all(dbTasks.map(async (t: any) => {
        const [messagesRes, artifactsRes, changesRes] = await Promise.all([
          fetch(`${backendUrl}/api/assistant/tasks/${t.id}/messages`, { headers }).catch(() => null),
          fetch(`${backendUrl}/api/assistant/tasks/${t.id}/artifacts`, { headers }).catch(() => null),
          fetch(`${backendUrl}/api/assistant/tasks/${t.id}/file_changes`, { headers }).catch(() => null),
        ]);

        const dbMessages = messagesRes?.ok ? await messagesRes.json() : [];
        const dbArtifacts = artifactsRes?.ok ? await artifactsRes.json() : [];
        const dbChanges = changesRes?.ok ? await changesRes.json() : [];

        return {
          id: t.id,
          title: t.title,
          prompt: t.prompt,
          workspace: t.workspace_id,
          status: t.status,
          mode: t.mode || 'Ask',
          model: t.model_config_json?.model || 'Auto',
          provider: t.model_config_json?.provider || 'Auto',
          workDirectory: t.model_config_json?.workDirectory || '',
          outputFormat: t.model_config_json?.outputFormat || 'Document',
          constraints: t.model_config_json?.constraints || '',
          contextReferences: t.model_config_json?.contextReferences || '',
          attachments: t.model_config_json?.attachments || [],
          skills: t.model_config_json?.skills || [],
          connectors: t.model_config_json?.connectors || [],
          permissionProfile: t.permission_profile || 'Guarded',
          currentStep: t.current_step || '',
          riskSummary: t.model_config_json?.riskSummary || (t.permission_profile === 'Guarded' ? ['External sends require approval'] : []),
          artifacts: dbArtifacts.map((art: any) => ({
            id: art.id,
            type: art.type_,
            filename: art.filename,
            mimeType: art.mime_type,
            preview: art.preview_ref || '',
          })),
          changes: dbChanges.map((ch: any) => ({
            id: ch.id,
            path: ch.path,
            changeType: ch.change_type,
            summary: ch.summary || '',
            approvalStatus: ch.approval_status,
          })),
          messages: dbMessages.map((msg: any) => ({
            id: msg.id,
            role: msg.role,
            content: msg.content,
            createdAt: new Date(msg.created_at_unix * 1000).toISOString(),
          })),
          actions: t.model_config_json?.outputFormat === 'Code App' ? [
            { id: 'act-preview', label: 'Open Preview', kind: 'preview', approvalRequired: false },
            { id: 'act-run', label: 'Run Locally', kind: 'execute', approvalRequired: true }
          ] : [],
          archived: t.archived,
          createdAt: new Date(t.created_at_unix * 1000).toISOString(),
          updatedAt: new Date(t.updated_at_unix * 1000).toISOString(),
        };
      }));

      return NextResponse.json({ tasks, capabilities: getAssistantCapabilities() });
    }
  } catch (error) {
    console.error('Failed to fetch tasks from backend:', error);
  }

  return NextResponse.json({ tasks: listAssistantTasks(), capabilities: getAssistantCapabilities() });
}

export async function POST(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    const backendUrl = process.env.BACKEND_URL || 'http://localhost:8080';
    const tenantId = request.headers.get('x-tenant-id') || 'storefront';
    const headers: Record<string, string> = {
      'x-tenant-id': tenantId,
      'Content-Type': 'application/json',
    };
    const authHeader = request.headers.get('Authorization');
    if (authHeader) {
      headers['Authorization'] = authHeader;
    }

    const backendTask = {
      id: '',
      workspace_id: payload.workspace || 'Personal OS',
      title: payload.prompt || 'New Task',
      prompt: payload.prompt || '',
      status: 'running',
      mode: payload.mode || 'Ask',
      permission_profile: payload.permissionProfile || 'Guarded',
      model_config_json: {
        model: payload.model || 'Auto',
        provider: payload.provider || 'Auto',
        workDirectory: payload.workDirectory || '',
        outputFormat: payload.outputFormat || 'Document',
        constraints: payload.constraints || '',
        contextReferences: payload.contextReferences || '',
        attachments: payload.attachments || [],
        skills: payload.skills || [],
        connectors: payload.connectors || [],
        riskSummary: payload.riskSummary || (payload.permissionProfile === 'Guarded' ? ['External sends require approval'] : []),
      },
      current_step: 'Initializing task...',
      archived: false,
      created_at_unix: 0,
      updated_at_unix: 0,
    };

    const res = await fetch(`${backendUrl}/api/assistant/tasks`, {
      method: 'POST',
      headers,
      body: JSON.stringify(backendTask),
    });

    if (res.ok) {
      const createdTask = await res.json();
      
      if (payload.prompt) {
        const userMsg = {
          id: '',
          task_id: createdTask.id,
          role: 'user',
          content: payload.prompt,
          tool_metadata_json: null,
          created_at_unix: 0,
        };
        await fetch(`${backendUrl}/api/assistant/tasks/${createdTask.id}/messages`, {
          method: 'POST',
          headers,
          body: JSON.stringify(userMsg),
        }).catch(() => null);

        const assistantMsg = {
          id: '',
          task_id: createdTask.id,
          role: 'assistant',
          content: `I've received your request: "${payload.prompt}" and planned the task in workspace "${payload.workspace}". I'm starting to execute it now under the "${payload.permissionProfile || 'Guarded'}" permission profile.`,
          tool_metadata_json: null,
          created_at_unix: 0,
        };
        await fetch(`${backendUrl}/api/assistant/tasks/${createdTask.id}/messages`, {
          method: 'POST',
          headers,
          body: JSON.stringify(assistantMsg),
        }).catch(() => null);
      }

      if (payload.outputFormat === 'Code App') {
        const codeArt = {
          id: '',
          task_id: createdTask.id,
          type_: 'code',
          filename: 'app/index.html',
          path: '/workspace/app/index.html',
          mime_type: 'text/html',
          size: 1024,
          preview_ref: '<html><body>Preview</body></html>',
          created_at_unix: 0,
        };
        await fetch(`${backendUrl}/api/assistant/tasks/${createdTask.id}/artifacts`, {
          method: 'POST',
          headers,
          body: JSON.stringify(codeArt),
        }).catch(() => null);

        const docArt = {
          id: '',
          task_id: createdTask.id,
          type_: 'document',
          filename: 'app-preview.html',
          path: '/workspace/app-preview.html',
          mime_type: 'text/html',
          size: 1024,
          preview_ref: '<html><body>Preview document</body></html>',
          created_at_unix: 0,
        };
        await fetch(`${backendUrl}/api/assistant/tasks/${createdTask.id}/artifacts`, {
          method: 'POST',
          headers,
          body: JSON.stringify(docArt),
        }).catch(() => null);
      } else if (payload.outputFormat === 'Presentation') {
        const presArt = {
          id: '',
          task_id: createdTask.id,
          type_: 'presentation',
          filename: 'presentation.pptx',
          path: '/workspace/presentation.pptx',
          mime_type: 'application/vnd.openxmlformats-officedocument.presentationml.presentation',
          size: 2048,
          preview_ref: 'presentation',
          created_at_unix: 0,
        };
        await fetch(`${backendUrl}/api/assistant/tasks/${createdTask.id}/artifacts`, {
          method: 'POST',
          headers,
          body: JSON.stringify(presArt),
        }).catch(() => null);

        const chartArt = {
          id: '',
          task_id: createdTask.id,
          type_: 'chart',
          filename: 'chart.png',
          path: '/workspace/chart.png',
          mime_type: 'image/png',
          size: 512,
          preview_ref: 'chart',
          created_at_unix: 0,
        };
        await fetch(`${backendUrl}/api/assistant/tasks/${createdTask.id}/artifacts`, {
          method: 'POST',
          headers,
          body: JSON.stringify(chartArt),
        }).catch(() => null);
      }

      const [messagesRes, artifactsRes, changesRes] = await Promise.all([
        fetch(`${backendUrl}/api/assistant/tasks/${createdTask.id}/messages`, { headers }).catch(() => null),
        fetch(`${backendUrl}/api/assistant/tasks/${createdTask.id}/artifacts`, { headers }).catch(() => null),
        fetch(`${backendUrl}/api/assistant/tasks/${createdTask.id}/file_changes`, { headers }).catch(() => null),
      ]);

      const dbMessages = messagesRes?.ok ? await messagesRes.json() : [];
      const dbArtifacts = artifactsRes?.ok ? await artifactsRes.json() : [];
      const dbChanges = changesRes?.ok ? await changesRes.json() : [];

      const mappedTask = {
        id: createdTask.id,
        title: createdTask.title,
        prompt: createdTask.prompt,
        workspace: createdTask.workspace_id,
        status: createdTask.status,
        mode: createdTask.mode || 'Ask',
        model: createdTask.model_config_json?.model || 'Auto',
        provider: createdTask.model_config_json?.provider || 'Auto',
        workDirectory: createdTask.model_config_json?.workDirectory || '',
        outputFormat: createdTask.model_config_json?.outputFormat || 'Document',
        constraints: createdTask.model_config_json?.constraints || '',
        contextReferences: createdTask.model_config_json?.contextReferences || '',
        attachments: createdTask.model_config_json?.attachments || [],
        skills: createdTask.model_config_json?.skills || [],
        connectors: createdTask.model_config_json?.connectors || [],
        permissionProfile: createdTask.permission_profile || 'Guarded',
        currentStep: createdTask.current_step || '',
        riskSummary: createdTask.model_config_json?.riskSummary || (payload.permissionProfile === 'Guarded' ? ['External sends require approval'] : []),
        artifacts: dbArtifacts.map((art: any) => ({
          id: art.id,
          type: art.type_,
          filename: art.filename,
          mimeType: art.mime_type,
          preview: art.preview_ref || '',
        })),
        changes: dbChanges.map((ch: any) => ({
          id: ch.id,
          path: ch.path,
          changeType: ch.change_type,
          summary: ch.summary || '',
          approvalStatus: ch.approval_status,
        })),
        messages: dbMessages.map((msg: any) => ({
          id: msg.id,
          role: msg.role,
          content: msg.content,
          createdAt: new Date(msg.created_at_unix * 1000).toISOString(),
        })),
        actions: payload.outputFormat === 'Code App' ? [
          { id: 'act-preview', label: 'Open Preview', kind: 'preview', approvalRequired: false },
          { id: 'act-run', label: 'Run Locally', kind: 'execute', approvalRequired: true }
        ] : [],
        archived: createdTask.archived,
        createdAt: new Date(createdTask.created_at_unix * 1000).toISOString(),
        updatedAt: new Date(createdTask.updated_at_unix * 1000).toISOString(),
      };

      return NextResponse.json({ task: mappedTask }, { status: 201 });
    }
  } catch (error) {
    console.error('Failed to create task in backend:', error);
  }

  try {
    const task = createAssistantTask(payload || {});
    return NextResponse.json({ task }, { status: 201 });
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'task could not be created' }, { status: 400 });
  }
}
