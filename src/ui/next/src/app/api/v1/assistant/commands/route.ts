import { NextResponse } from 'next/server';
import { listCommands, runCommand } from '../store';

export async function GET() {
  return NextResponse.json(listCommands());
}

export async function POST(request: Request) {
  const payload = await request.json().catch(() => null);
  try {
    return NextResponse.json(runCommand(payload || {}), { status: 202 });
  } catch (error: any) {
    return NextResponse.json({ error: error.message || 'command could not be run' }, { status: 400 });
  }
}
