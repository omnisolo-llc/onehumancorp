import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const q = searchParams.get('q') || '';

  const allAgents = [
    {
      id: 'agent-1',
      name: 'Data Analyst',
      description: 'Analyzes CSV files and generates beautiful charts. (AutoGPT Agent Marketplace)',
      author: 'AutoGPT',
      version: '1.0.0',
      endpoint: 'https://marketplace.example.com/agents/agent-1',
    },
    {
      id: 'agent-2',
      name: 'SEO Specialist',
      description: 'Optimizes content for search engines',
      author: 'Growth Team',
      version: '1.2.0',
      endpoint: 'https://marketplace.example.com/agents/agent-2',
    }
  ];

  let filtered = allAgents;
  if (q) {
    const qLower = q.toLowerCase();
    filtered = allAgents.filter(a => a.name.toLowerCase().includes(qLower) || a.description.toLowerCase().includes(qLower));
  }

  return NextResponse.json(filtered);
}
