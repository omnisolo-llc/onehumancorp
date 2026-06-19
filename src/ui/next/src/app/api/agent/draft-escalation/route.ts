import { NextResponse } from 'next/server';

export async function POST() {
  return NextResponse.json({
    draft: 'Spike in pickup complaints at Location A'
  });
}
