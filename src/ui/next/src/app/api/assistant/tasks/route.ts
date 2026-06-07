import { NextResponse } from 'next/server';
import { createAssistantTask, listAssistantTasks } from '../store';

export async function GET() {
  return NextResponse.json({ tasks: listAssistantTasks() });
}

export async function POST(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    const task = createAssistantTask(payload || {});
    return NextResponse.json({ task }, { status: 201 });
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'task could not be created' }, { status: 400 });
  }
}
