import { NextResponse } from 'next/server';
import { proxyBackendStream } from '../../../../ui/backendProxy';

export async function GET(req: Request) {
  // Let the backend handle the real stream via Rust
  return proxyBackendStream(req, "/api/v1/orchestration/mesh/stream");
}
