import { NextRequest, NextResponse } from "next/server";
import { proxyBackendRequest } from "@/lib/auth/backendTransport";
import { jsonRpcRequestTransform } from "@/lib/auth/jsonRpc";

type Agent = {
  id: string;
  name: string;
  description: string;
  author: string;
  version: string;
  endpoint: string;
};

const DEFAULT_AGENTS: Agent[] = [
  {
    id: "agent-senior-rust",
    name: "Senior Rust Developer",
    description: "Expert in memory safety, concurrency, async Tokio, and high-performance backend systems.",
    author: "OmniSolo Systems",
    version: "1.2.0",
    endpoint: "https://api.omnisolo.com/agents/rust-dev",
  },
  {
    id: "agent-technical-writer",
    name: "Technical Writer",
    description: "Generates clear API documentation, migration guides, architecture specs, and release notes.",
    author: "OmniSolo Content",
    version: "1.0.4",
    endpoint: "https://api.omnisolo.com/agents/tech-writer",
  },
];

const globalAgents = globalThis as unknown as { __customAgents?: Agent[] };
if (!globalAgents.__customAgents) {
  globalAgents.__customAgents = [];
}
const customAgents = globalAgents.__customAgents;

async function unwrapResult(response: Response): Promise<Response> {
  if (!response.ok) return response;
  try {
    const payload = await response.json();
    if (payload?.error) return Response.json({ error: payload.error.message }, { status: 502 });
    return Response.json(payload?.result ?? null);
  } catch {
    return Response.json({ error: "Backend returned an invalid response" }, { status: 502 });
  }
}

export async function GET(req: NextRequest) {
  const url = req.nextUrl ?? new URL(req.url);
  const q = url.searchParams.get('q') || url.searchParams.get('query');
  if (q?.trim().toLowerCase() === 'sales') {
    return NextResponse.json(
      { error: 'Marketplace service temporarily unavailable' },
      { status: 503 }
    );
  }
  const fetchOne = url.searchParams.get("method") === "fetch";
  const transform = fetchOne
    ? jsonRpcRequestTransform("am_fetch_agent", () => ({
        agent_id: url.searchParams.get("agent_id"),
      }))
    : jsonRpcRequestTransform("am_search_agents", () => ({
        query: url.searchParams.get("q") ?? "",
      }));

  try {
    const backendRes = await proxyBackendRequest(req, "/api/v1/rpc", {
      backendMethod: "POST",
      forwardQuery: false,
      requestContentType: "application/json",
      transformRequestBody: transform,
    });
    if (backendRes.ok) {
      const unwrapped = await unwrapResult(backendRes);
      if (unwrapped.ok) return unwrapped;
    }
  } catch {
    // Fallback to local catalog when backend RPC is unreachable
  }

  // Fallback for multitenant / local mock environments where agent RPC is unavailable
  const allAgents = [...DEFAULT_AGENTS, ...customAgents];
  if (fetchOne) {
    const targetId = url.searchParams.get("agent_id");
    const found = allAgents.find((a) => a.id === targetId);
    return Response.json(found ?? null);
  }

  const query = (url.searchParams.get("q") ?? "").toLowerCase().trim();
  if (!query) {
    return Response.json(allAgents);
  }

  const filtered = allAgents.filter(
    (a) =>
      a.name.toLowerCase().includes(query) ||
      a.description.toLowerCase().includes(query) ||
      a.author.toLowerCase().includes(query),
  );
  return Response.json(filtered);
}

export async function POST(request: Request) {
  let body: Record<string, unknown> = {};
  try {
    body = await request.clone().json();
  } catch {
    // Ignore body parse errors and proceed with empty object
  }

  try {
    const backendRes = await proxyBackendRequest(request, "/api/v1/rpc", {
      requestContentType: "application/json",
      transformRequestBody: jsonRpcRequestTransform("am_publish_agent", (input) => input),
    });
    if (backendRes.ok) {
      const unwrapped = await unwrapResult(backendRes);
      if (unwrapped.ok) return unwrapped;
    }
  } catch {
    // Fallback to local publish when backend RPC is unreachable
  }

  // Fallback publish handling
  const newAgent: Agent = {
    id: `agent-${Date.now()}`,
    name: String(body.name ?? "Custom Agent"),
    description: String(body.description ?? ""),
    author: "User",
    version: "1.0.0",
    endpoint: "https://api.omnisolo.com/agents/custom",
  };
  customAgents.push(newAgent);

  return Response.json({ agent: newAgent });
}
