import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json({
    "Context": "Infinite Context",
    "Size": "842.5 MB"
  });
}
