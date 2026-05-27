import { NextResponse } from 'next/server';
import { routeIntent } from './routeIntent';

export async function POST(req: Request) {
  const { message } = await req.json();

  if (!message || typeof message !== 'string') {
    return NextResponse.json({ error: 'Message is required' }, { status: 400 });
  }

  const result = routeIntent(message);

  return NextResponse.json(result);
}
