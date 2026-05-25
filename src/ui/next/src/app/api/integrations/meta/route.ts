import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  // Simulate AI scanning and parsing process
  await new Promise(resolve => setTimeout(resolve, 2000));

  return NextResponse.json({
    status: 'connected',
    message: 'We found 12 products. Added to your OHC store.'
  });
}
