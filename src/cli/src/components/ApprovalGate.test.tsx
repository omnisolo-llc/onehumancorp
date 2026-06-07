import React from 'react';
import { render } from 'ink-testing-library';
import { ApprovalGate } from './ApprovalGate';
import { describe, it, expect, vi } from 'vitest';

describe('ApprovalGate', () => {
  it('renders correctly', () => {
    const request = {
      toolName: 'test_tool',
      argsJson: '{"test": 123}',
      reason: 'Requires approval',
      isHighRisk: false
    };

    const { lastFrame } = render(<ApprovalGate request={request} onApprove={() => {}} onReject={() => {}} />);
    const frame = lastFrame();

    expect(frame).toContain('Approval Required');
    expect(frame).toContain('test_tool');
    expect(frame).toContain('{"test": 123}');
    expect(frame).toContain('Requires approval');
  });

  it('renders high risk correctly', () => {
    const request = {
      toolName: 'delete_db',
      argsJson: '{}',
      reason: 'High risk tool',
      isHighRisk: true
    };

    const { lastFrame } = render(<ApprovalGate request={request} onApprove={() => {}} onReject={() => {}} />);
    const frame = lastFrame();

    expect(frame).toContain('HIGH RISK APPROVAL REQUIRED');
    expect(frame).toContain('delete_db');
  });

  it('handles approve input', () => {
    const request = { toolName: 't', argsJson: '{}', reason: 'r', isHighRisk: false };
    const onApprove = vi.fn();
    const { stdin } = render(<ApprovalGate request={request} onApprove={onApprove} onReject={() => {}} />);
    stdin.write('y');
    expect(onApprove).toHaveBeenCalled();
  });

  it('handles reject input', () => {
    const request = { toolName: 't', argsJson: '{}', reason: 'r', isHighRisk: false };
    const onReject = vi.fn();
    const { stdin } = render(<ApprovalGate request={request} onApprove={() => {}} onReject={onReject} />);
    stdin.write('n');
    expect(onReject).toHaveBeenCalled();
  });

  it('handles edit input and switches mode', async () => {
    const request = { toolName: 't', argsJson: '{}', reason: 'r', isHighRisk: false };
    const onEdit = vi.fn();
    const { stdin, lastFrame } = render(<ApprovalGate request={request} onApprove={() => {}} onReject={() => {}} onEdit={onEdit} />);

    // allow render cycle
    await new Promise(r => setTimeout(r, 50));

    stdin.write('e');

    // allow state update
    await new Promise(r => setTimeout(r, 50));

    const frame = lastFrame();
    expect(frame).toContain('Editing arguments feature is not fully implemented in CLI mockup yet.');
  });

  it('ignores input in edit mode', async () => {
    const request = { toolName: 't', argsJson: '{}', reason: 'r', isHighRisk: false };
    const onEdit = vi.fn();
    const onApprove = vi.fn();
    const onReject = vi.fn();
    const { stdin, lastFrame } = render(<ApprovalGate request={request} onApprove={onApprove} onReject={onReject} onEdit={onEdit} />);

    await new Promise(r => setTimeout(r, 50));
    stdin.write('e');
    await new Promise(r => setTimeout(r, 50));

    stdin.write('y');
    await new Promise(r => setTimeout(r, 50));
    expect(onApprove).not.toHaveBeenCalled();

    stdin.write('n');
    await new Promise(r => setTimeout(r, 50));
    expect(onReject).not.toHaveBeenCalled();
  });

  it('ignores unknown input in view mode', async () => {
    const request = { toolName: 't', argsJson: '{}', reason: 'r', isHighRisk: false };
    const onEdit = vi.fn();
    const onApprove = vi.fn();
    const onReject = vi.fn();
    const { stdin, lastFrame } = render(<ApprovalGate request={request} onApprove={onApprove} onReject={onReject} onEdit={onEdit} />);

    await new Promise(r => setTimeout(r, 50));
    stdin.write('z');
    await new Promise(r => setTimeout(r, 50));

    expect(onApprove).not.toHaveBeenCalled();
    expect(onReject).not.toHaveBeenCalled();
  });
});
