import { NextResponse } from 'next/server';
import { listAssistantTasks, createAssistantTask, getAssistantCapabilities } from '../store';

export async function GET() {
  return NextResponse.json({ tasks: listAssistantTasks(), capabilities: getAssistantCapabilities() });
}

export async function POST(request: Request) {
  const body = await request.json();
  const task = createAssistantTask(body);
  return NextResponse.json({ task }, { status: 201 });
}
