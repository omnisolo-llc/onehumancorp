import { NextResponse } from 'next/server';
import { proxyBackendPost } from '../../ui/backendProxy';

export async function POST(request: Request) {
  // Proxy the request to the Rust backend
  return proxyBackendPost(request, '/api/v1/operations/approve');
}
