import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const query = searchParams.get('q') || '';

  // Mocked agents from the Agent Marketplace
  const agents = [
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
      name: 'SEO Optimizer',
      description: 'Optimizes your storefront content for better search engine rankings.',
      author: 'AutoGPT',
      version: '2.1.0',
      endpoint: 'https://marketplace.example.com/agents/agent-2',
    },
    {
      id: 'agent-3',
      name: 'Customer Service Bot',
      description: 'Handles basic customer inquiries and manages refunds automatically.',
      author: 'Community',
      version: '1.5.0',
      endpoint: 'https://marketplace.example.com/agents/agent-3',
    },
  ];

  // Filter if query is provided
  const filtered = agents.filter(
    (agent) =>
      agent.name.toLowerCase().includes(query.toLowerCase()) ||
      agent.description.toLowerCase().includes(query.toLowerCase())
  );

  return NextResponse.json(filtered);
}
