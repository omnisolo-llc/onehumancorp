import { NextRequest } from "next/server";
import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { jsonRpcRequestTransform } from "@/lib/auth/jsonRpc";

type Agent = { id: string; name: string; description: string; author: string; version: string; endpoint: string };
type Publication = Omit<Agent, 'id'> & { id?: string };
const privateHeaders = { "cache-control": "private, no-store", pragma: "no-cache" };
function privateJson(value: unknown, status = 200): Response { return Response.json(value, { status, headers: privateHeaders }); }
function privateBackendResponse(response: Response): Response {
  const headers = new Headers(response.headers);
  for (const [name, value] of Object.entries(privateHeaders)) headers.set(name, value);
  return new Response(response.body, { status: response.status, headers });
}
function agentReceipt(value: unknown): value is Agent {
  if (!value || typeof value !== 'object' || Array.isArray(value)) return false;
  const agent = value as Record<string, unknown>;
  if (agent.success === false || agent.error != null) return false;
  if (['id', 'name', 'author', 'version', 'endpoint'].some(key => typeof agent[key] !== 'string' || !(agent[key] as string).trim())) return false;
  if (typeof agent.description !== 'string') return false;
  try { const endpoint = new URL(agent.endpoint as string); return ['https:', 'http:'].includes(endpoint.protocol) && !endpoint.username && !endpoint.password; }
  catch { return false; }
}
function publicationParams(input: Record<string, unknown>): Publication {
  const fields = ['id', 'name', 'description', 'author', 'version', 'endpoint'];
  if (Object.keys(input).some(key => !fields.includes(key)) ||
      (input.id !== undefined && typeof input.id !== 'string') ||
      !agentReceipt({ ...input, id: input.id || 'new-publication' })) {
    throw new Error('Only a complete marketplace descriptor can be published');
  }
  return input as Publication;
}
function correlatedRequest(method: string, params: (input: Record<string, unknown>) => Record<string, unknown>) {
  const encode = jsonRpcRequestTransform(method, params);
  let requestId: string | undefined;
  return {
    id: () => requestId,
    transform: (body: Uint8Array<ArrayBuffer>) => {
      const encoded = encode(body);
      requestId = JSON.parse(new TextDecoder().decode(encoded)).id;
      return encoded;
    },
  };
}
async function unwrapResult(response: Response, kind: 'search' | 'fetch' | 'publish', requestId: string | undefined, publication?: Publication, requestedAgentId?: string): Promise<Response> {
  if (!response.ok) return privateBackendResponse(response);
  try {
    const payload = await response.json();
    if (!requestId || !payload || payload.id !== requestId || payload.jsonrpc !== '2.0' || payload.error != null || payload.success === false) throw new Error('Invalid RPC acknowledgement');
    const result: unknown = payload.result;
    const record = result && typeof result === 'object' && !Array.isArray(result) ? result as Record<string, unknown> : null;
    if (record?.error != null || record?.success === false) throw new Error('Rejected RPC result');
    const valid = kind === 'search' ? Array.isArray(result) && result.every(agentReceipt) : agentReceipt(kind === 'publish' && record?.agent !== undefined ? record.agent : result);
    if (!valid) throw new Error('Missing marketplace receipt');
    if (kind === 'fetch' && (!requestedAgentId || (result as Agent).id !== requestedAgentId)) throw new Error('Fetched agent does not match the request');
    if (kind === 'publish') {
      const receipt = (record?.agent !== undefined ? record.agent : result) as Agent;
      if (!publication || ['name', 'description', 'author', 'version', 'endpoint'].some(key => receipt[key as keyof Agent] !== publication[key as keyof Publication]) ||
          (publication.id && receipt.id !== publication.id)) throw new Error('Publication acknowledgement does not match the reviewed descriptor');
    }
    return privateJson(result);
  } catch {
    return privateJson({ error: 'Marketplace returned an invalid acknowledgement' }, 502);
  }
}

export async function GET(req: NextRequest) {
  const url = req.nextUrl ?? new URL(req.url);
  const fetchOne = url.searchParams.get('method') === 'fetch';
  const requestedAgentId = url.searchParams.get('agent_id');
  const rpc = fetchOne
    ? correlatedRequest('am_fetch_agent', () => {
      if (!requestedAgentId?.trim()) throw new Error('An agent ID is required');
      return { agent_id: requestedAgentId };
    })
    : correlatedRequest('am_search_agents', () => ({ query: url.searchParams.get('q') ?? url.searchParams.get('query') ?? '' }));
  try {
    const response = await proxyBackendRequest(req, '/api/v1/rpc', {
      backendMethod: 'POST', forwardQuery: false, requestContentType: 'application/json', transformRequestBody: rpc.transform,
    });
    return unwrapResult(response, fetchOne ? 'fetch' : 'search', rpc.id(), undefined, requestedAgentId ?? undefined);
  } catch {
    return privateJson({ error: 'Marketplace service unavailable' }, 503);
  }
}

export async function POST(request: Request) {
  let publication: Publication | undefined;
  const rpc = correlatedRequest('am_publish_agent', input => {
    publication = publicationParams(input);
    return publication;
  });
  try {
    const response = await proxyBackendRequest(request, '/api/v1/rpc', {
      requestContentType: 'application/json', transformRequestBody: rpc.transform,
    });
    return unwrapResult(response, 'publish', rpc.id(), publication);
  } catch {
    return privateJson({ error: 'Marketplace publication could not be confirmed' }, 503);
  }
}
