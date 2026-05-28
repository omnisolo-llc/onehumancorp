import { describe, it, expect } from 'vitest';
import { POST } from './route';

describe('routeIntent via POST', () => {
  it('should route sales messages to sales department', async () => {
    const req = new Request('http://localhost/api/agents/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message: 'Can you generate a quote for John Doe?' })
    });
    const res = await POST(req);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.department_assigned).toBe('sales');
    expect(data.agent).toBe('The Salesperson');
    expect(data.description).toContain('quote');
  });

  it('should route lead messages to sales department', async () => {
    const req = new Request('http://localhost/api/agents/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message: 'We got a new lead from the website.' })
    });
    const res = await POST(req);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.department_assigned).toBe('sales');
    expect(data.agent).toBe('The Salesperson');
    expect(data.description).toContain('lead');
  });

  it('should route marketing messages to marketing department', async () => {
    const req = new Request('http://localhost/api/agents/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message: 'Draft a welcome email for new newsletter subscribers' })
    });
    const res = await POST(req);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.department_assigned).toBe('marketing');
    expect(data.agent).toBe('The Promoter');
    expect(data.description).toContain('email');
  });

  it('should route campaign messages to marketing department', async () => {
    const req = new Request('http://localhost/api/agents/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message: 'Let us start a new campaign for summer.' })
    });
    const res = await POST(req);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.department_assigned).toBe('marketing');
    expect(data.agent).toBe('The Promoter');
    expect(data.description).toContain('campaign');
  });

  it('should route finance messages to finance department', async () => {
    const req = new Request('http://localhost/api/agents/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message: 'Refund order #123' })
    });
    const res = await POST(req);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.department_assigned).toBe('finance');
    expect(data.agent).toBe('The Accountant');
    expect(data.description.toLowerCase()).toContain('refund');
  });

  it('should route legal messages to legal department', async () => {
    const req = new Request('http://localhost/api/agents/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message: 'Review this new contract' })
    });
    const res = await POST(req);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.department_assigned).toBe('legal');
    expect(data.agent).toBe('The Protector');
    expect(data.description.toLowerCase()).toContain('contract');
  });

  it('should route advisory messages to advisory department', async () => {
    const req = new Request('http://localhost/api/agents/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message: 'What are some insights on our performance?' })
    });
    const res = await POST(req);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.department_assigned).toBe('business_advisory');
    expect(data.agent).toBe('The Advisor');
    expect(data.description.toLowerCase()).toContain('insight');
  });

  it('should route customer success messages to customer success department', async () => {
    const req = new Request('http://localhost/api/agents/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message: 'Reply to this DM from a customer' })
    });
    const res = await POST(req);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.department_assigned).toBe('customer_success');
    expect(data.agent).toBe('The Ambassador');
    expect(data.description.toLowerCase()).toContain('dm');
  });

  it('should default to operations department for unrecognized messages', async () => {
    const req = new Request('http://localhost/api/agents/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message: 'Update the schedule for tomorrow' })
    });
    const res = await POST(req);
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.department_assigned).toBe('operations');
    expect(data.agent).toBe('The Manager');
    expect(data.description).toContain('schedule');
  });
});
