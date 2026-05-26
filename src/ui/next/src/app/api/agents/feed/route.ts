import { NextResponse } from 'next/server';

let feed = [
  { id: '1', department: 'The Ambassador', description: 'Replied to 3 Instagram DMs overnight', timestamp: '2h ago' },
  { id: '2', department: 'The Promoter', description: 'Scheduled weekend promo post', timestamp: '4h ago' },
];

export async function GET() {
  return NextResponse.json({ feed });
}
