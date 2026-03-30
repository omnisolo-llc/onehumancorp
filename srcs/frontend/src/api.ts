// API definitions for One Human Corp React Frontend

export interface Agent {
  id: string;
  name: string;
  role: string;
  status: string;
  organizationId: string;
}

export interface DashboardSnapshot {
  organization: {
    id: string;
    name: string;
    domain: string;
  };
  meetings: any[];
  costs: any;
  agents: Agent[];
  statuses: { status: string; count: number }[];
  updatedAt: string;
}

const getAuthHeaders = () => {
  const token = localStorage.getItem('flutter.auth_token') || '{"value":"seeded-admin-token"}';
  try {
    const parsed = JSON.parse(token);
    return { 'Authorization': `Bearer ${parsed.value || parsed}`, 'Content-Type': 'application/json' };
  } catch (e) {
    return { 'Authorization': `Bearer ${token}`, 'Content-Type': 'application/json' };
  }
};

export const fetchDashboard = async (): Promise<DashboardSnapshot> => {
  const response = await fetch('/api/dashboard', { headers: getAuthHeaders() });
  if (!response.ok) {
    throw new Error('Failed to fetch dashboard data');
  }
  return response.json();
};

export const hireAgent = async (name: string, role: string, providerType: string = 'builtin'): Promise<void> => {
  const response = await fetch('/api/agents/hire', {
    method: 'POST',
    headers: getAuthHeaders(),
    body: JSON.stringify({ name, role, providerType }),
  });
  if (!response.ok) {
    throw new Error('Failed to hire agent');
  }
};

export const fireAgent = async (agentId: string): Promise<void> => {
  const response = await fetch('/api/agents/fire', {
    method: 'POST',
    headers: getAuthHeaders(),
    body: JSON.stringify({ agentId }),
  });
  if (!response.ok) {
    throw new Error('Failed to fire agent');
  }
};

export const seedDevData = async (): Promise<void> => {
  const response = await fetch('/api/dev/seed', {
    method: 'POST',
    headers: getAuthHeaders(),
    body: JSON.stringify({ scenario: 'launch-readiness' }),
  });
  if (!response.ok) {
    throw new Error('Failed to seed dev data');
  }
};
