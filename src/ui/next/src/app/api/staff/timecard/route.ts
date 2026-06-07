import { NextResponse } from 'next/server';

let timecardEvents: any[] = [];

export async function GET() {
  return NextResponse.json(timecardEvents);
}

export async function POST(request: Request) {
  const body = await request.json();
  const newEvents = Array.isArray(body) ? body : [body];

  const processedEvents = newEvents.map((event) => ({
    id: `event_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
    ...event,
    sync_status: 'SYNCED',
    created_at: new Date().toISOString()
  }));

  timecardEvents.push(...processedEvents);
  return NextResponse.json(processedEvents, { status: 201 });
}
