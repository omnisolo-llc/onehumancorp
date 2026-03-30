export interface Agent {
  id: string;
  name: string;
  role: string;
  organization_id: string;
  status: string;
  provider_type: string;
}

export interface DashboardResponse {
  id: string;
  name: string;
  domain: string;
  agents: Agent[];
}

export async function fetchDashboard(): Promise<DashboardResponse> {
  const res = await fetch("/api/dashboard");
  if (!res.ok) {
    throw new Error(`Failed to fetch dashboard: ${res.statusText}`);
  }
  return res.json();
}

export async function hireAgent(name: string, role: string, providerType?: string): Promise<DashboardResponse> {
  const payload = { name, role, providerType: providerType || "builtin" };
  const res = await fetch("/api/agents/hire", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  });
  if (!res.ok) {
    throw new Error(`Failed to hire agent: ${res.statusText}`);
  }
  return res.json();
}

export async function fireAgent(agentId: string): Promise<DashboardResponse> {
  const payload = { agentId };
  const res = await fetch("/api/agents/fire", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(payload),
  });
  if (!res.ok) {
    throw new Error(`Failed to fire agent: ${res.statusText}`);
  }
  return res.json();
}
