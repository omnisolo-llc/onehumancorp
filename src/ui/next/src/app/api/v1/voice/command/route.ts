import { NextResponse } from 'next/server';
import { proxyBackendPost } from '../../../ui/backendProxy';

export async function POST(req: Request) {
  return proxyBackendPost(req, '/api/v1/voice/command');
}
