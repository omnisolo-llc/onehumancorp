import { describe, it, expect } from 'vitest';
import { routeIntent } from './route';

describe('routeIntent', () => {
  it('should route sales messages to sales department', () => {
    const result = routeIntent('Can you generate a quote for John Doe?');
    expect(result.department_assigned).toBe('sales');
    expect(result.agent).toBe('The Salesperson');
    expect(result.description).toContain('quote');
  });

  it('should route lead messages to sales department', () => {
    const result = routeIntent('We got a new lead from the website.');
    expect(result.department_assigned).toBe('sales');
    expect(result.agent).toBe('The Salesperson');
    expect(result.description).toContain('lead');
  });

  it('should route marketing messages to marketing department', () => {
    const result = routeIntent('Draft a welcome email for new newsletter subscribers');
    expect(result.department_assigned).toBe('marketing');
    expect(result.agent).toBe('The Promoter');
    expect(result.description).toContain('email');
  });

  it('should route campaign messages to marketing department', () => {
    const result = routeIntent('Let us start a new campaign for summer.');
    expect(result.department_assigned).toBe('marketing');
    expect(result.agent).toBe('The Promoter');
    expect(result.description).toContain('campaign');
  });

  it('should route finance messages to finance department', () => {
    const result = routeIntent('Refund order #123');
    expect(result.department_assigned).toBe('finance');
    expect(result.agent).toBe('The Accountant');
    expect(result.description.toLowerCase()).toContain('refund');
  });

  it('should route legal messages to legal department', () => {
    const result = routeIntent('Review this new contract');
    expect(result.department_assigned).toBe('legal');
    expect(result.agent).toBe('The Protector');
    expect(result.description.toLowerCase()).toContain('contract');
  });

  it('should route advisory messages to advisory department', () => {
    const result = routeIntent('What are some insights on our performance?');
    expect(result.department_assigned).toBe('business_advisory');
    expect(result.agent).toBe('The Advisor');
    expect(result.description.toLowerCase()).toContain('insight');
  });

  it('should route customer success messages to customer success department', () => {
    const result = routeIntent('Reply to this DM from a customer');
    expect(result.department_assigned).toBe('customer_success');
    expect(result.agent).toBe('The Ambassador');
    expect(result.description.toLowerCase()).toContain('dm');
  });

  it('should default to operations department for unrecognized messages', () => {
    const result = routeIntent('Update the schedule for tomorrow');
    expect(result.department_assigned).toBe('operations');
    expect(result.agent).toBe('The Manager');
    expect(result.description).toContain('schedule');
  });
});
