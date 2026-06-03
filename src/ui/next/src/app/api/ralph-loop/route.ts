import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  try {
    const { task, taskId } = await req.json();

    if (!task) {
      return NextResponse.json({ error: 'Task is required' }, { status: 400 });
    }

    const effectiveTaskId = taskId || `ralph-${Date.now()}`;

    return NextResponse.json({
      status: "STARTED",
      taskId: effectiveTaskId,
      message: "Ralph Mission initialized (Mock)."
    });
  } catch (error: any) {
    return NextResponse.json({ error: error.message }, { status: 500 });
  }
}

import fs from 'fs/promises';
import path from 'path';

export async function GET(req: Request) {
  const { searchParams } = new URL(req.url);
  const taskId = searchParams.get('taskId');

  if (!taskId) {
    return NextResponse.json({ error: 'taskId is required' }, { status: 400 });
  }

  // Attempt to read the actual progress file from the agent workspace
  // Default workspace is usually the current directory or OHC_AGENT_WORKSPACE
  const workspacePath = process.env.OHC_AGENT_WORKSPACE || process.cwd();
  const progressFilePath = path.join(workspacePath, `.ralph_progress_${taskId}.json`);

  try {
    const data = await fs.readFile(progressFilePath, 'utf8');
    return NextResponse.json(JSON.parse(data));
  } catch (e) {
    return NextResponse.json({
      error: 'Mission progress not found',
      details: `Could not read ${progressFilePath}`
    }, { status: 404 });
  }
}
