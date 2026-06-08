import { NextResponse } from 'next/server';
import { getBilling } from '../store';

export async function GET() {
  return NextResponse.json(getBilling());
}
