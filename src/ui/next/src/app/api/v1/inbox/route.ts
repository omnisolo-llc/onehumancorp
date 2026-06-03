import { NextResponse } from 'next/server';

export async function GET() {
  // In a real implementation, this would fetch from the database via gRPC
  return NextResponse.json({ messages: [] });
}
