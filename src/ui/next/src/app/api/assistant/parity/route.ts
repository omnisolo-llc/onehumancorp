import { NextResponse } from 'next/server';
import { listWorkBuddyParity } from '../store';

export async function GET() {
  return NextResponse.json(listWorkBuddyParity());
}
