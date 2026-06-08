import { NextResponse } from 'next/server';
import { listAgentParity } from '../store';

export async function GET() {
  return NextResponse.json(listAgentParity());
}
