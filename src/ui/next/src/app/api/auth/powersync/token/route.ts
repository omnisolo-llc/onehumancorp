import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json({
    token: "mock-powersync-token", // In a real app this comes from the backend
    endpoint: "http://localhost:8080"
  });
}