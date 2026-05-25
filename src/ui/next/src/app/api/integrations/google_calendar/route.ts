import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  // Simulate OAuth connection and sync process
  await new Promise(resolve => setTimeout(resolve, 1500));

  return NextResponse.json({
    status: 'connected',
    message: 'Your calendar is synced and protecting your time slots.'
  });
}
