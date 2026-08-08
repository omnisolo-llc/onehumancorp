import React from 'react';
import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { ActionPlanCard, DynamicWorkflowPlan } from './ActionPlanCard';
import '@testing-library/jest-dom';

const mockPlan: DynamicWorkflowPlan = {
  id: "dwf-1",
  prompt: "Set up a marketing campaign for the new summer cakes.",
  status: "AwaitingConfirmation",
  tasks: [
    {
      id: "dwf-1-plan",
      title: "Plan workflow shards",
      description: "Map the requested work into independently executable shards",
      role: "workflow-planner",
      phase: "planning",
      dependencies: []
    },
    {
      id: "dwf-1-exec-1",
      title: "Draft promo email",
      description: "Write promotional marketing newsletter copy",
      role: "marketing-assistant",
      phase: "execution",
      dependencies: ["dwf-1-plan"]
    },
    {
      id: "dwf-1-verify-1",
      title: "Verify campaign assets",
      description: "Independently review the draft quality",
      role: "adversarial-reviewer",
      phase: "verification",
      dependencies: ["dwf-1-exec-1"]
    },
    {
      id: "dwf-1-synthesis",
      title: "Synthesize checked results",
      description: "Coalesce and publish final summer campaign",
      role: "workflow-synthesizer",
      phase: "synthesis",
      dependencies: ["dwf-1-verify-1"]
    }
  ]
};

describe('ActionPlanCard', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it('renders workflow prompt, phases and tasks correctly', () => {
    render(
      <ActionPlanCard
        planId="dwf-1"
        prompt="Set up a marketing campaign for the new summer cakes."
        initialPlan={mockPlan}
        onApprove={async () => {}}
      />
    );

    expect(screen.getByText(/Set up a marketing campaign for the new summer cakes/)).toBeInTheDocument();
    expect(screen.getByText("Plan workflow shards")).toBeInTheDocument();
    expect(screen.getByText("Draft promo email")).toBeInTheDocument();
    expect(screen.getByText("Verify campaign assets")).toBeInTheDocument();
    expect(screen.getByText("Synthesize checked results")).toBeInTheDocument();
    expect(screen.getByText("Awaiting Approval")).toBeInTheDocument();
  });

  it('handles execution and status transitions beautifully through the phases', async () => {
    let resolveApprove: (val: any) => void = () => {};
    const approvePromise = new Promise((resolve) => {
      resolveApprove = resolve;
    });
    const onApproveMock = vi.fn().mockReturnValue(approvePromise);

    render(
      <ActionPlanCard
        planId="dwf-1"
        prompt="Set up a marketing campaign for the new summer cakes."
        initialPlan={mockPlan}
        onApprove={onApproveMock}
      />
    );

    const approveButton = screen.getByTestId("approve-execute-btn");
    expect(approveButton).toBeInTheDocument();

    // Click Approve
    await act(async () => {
      fireEvent.click(approveButton);
    });

    // It should now be in the loading state since onApprove hasn't resolved yet
    expect(screen.getByTestId("approve-execute-btn-loading")).toBeInTheDocument();
    expect(onApproveMock).toHaveBeenCalledWith("dwf-1");

    // Resolve the promise to trigger transition to Running
    await act(async () => {
      resolveApprove({ ok: true });
      await vi.advanceTimersByTimeAsync(0);
    });

    expect(screen.getByTestId("workflow-status-badge")).toHaveTextContent("Running");

    // Planning phase runs
    expect(screen.getByTestId("badge-running-dwf-1-plan")).toBeInTheDocument();
    expect(screen.getByTestId("badge-pending-dwf-1-exec-1")).toBeInTheDocument();

    // Fast-forward 1.2s to complete Planning
    await act(async () => {
      await vi.advanceTimersByTimeAsync(1200);
    });
    expect(screen.getByTestId("badge-completed-dwf-1-plan")).toBeInTheDocument();
    expect(screen.getByTestId("badge-running-dwf-1-exec-1")).toBeInTheDocument();

    // Fast-forward 1.5s to complete Execution
    await act(async () => {
      await vi.advanceTimersByTimeAsync(1500);
    });
    expect(screen.getByTestId("badge-completed-dwf-1-exec-1")).toBeInTheDocument();
    expect(screen.getByTestId("badge-running-dwf-1-verify-1")).toBeInTheDocument();

    // Fast-forward 1.5s to complete Verification
    await act(async () => {
      await vi.advanceTimersByTimeAsync(1500);
    });
    expect(screen.getByTestId("badge-completed-dwf-1-verify-1")).toBeInTheDocument();
    expect(screen.getByTestId("badge-running-dwf-1-synthesis")).toBeInTheDocument();

    // Fast-forward 1.2s to complete Synthesis and finish overall plan
    await act(async () => {
      await vi.advanceTimersByTimeAsync(1200);
    });
    expect(screen.getByTestId("badge-completed-dwf-1-synthesis")).toBeInTheDocument();
    expect(screen.getByTestId("workflow-status-badge")).toHaveTextContent("Completed");
  });

  it('handles flaky network resiliently with automated and manual retries', async () => {
    // Fail on all initial attempts to trigger the manual retry state
    const onApproveMock = vi.fn()
      .mockRejectedValue(new Error("Network timeout"));

    render(
      <ActionPlanCard
        planId="dwf-1"
        prompt="Set up a marketing campaign for the new summer cakes."
        initialPlan={mockPlan}
        onApprove={onApproveMock}
      />
    );

    const approveButton = screen.getByTestId("approve-execute-btn");

    await act(async () => {
      fireEvent.click(approveButton);
    });

    // Let the initial attempt run and fail, which triggers timeout for first retry
    await act(async () => {
      await vi.advanceTimersByTimeAsync(0);
    });

    // Advance 1s to let first retry fire and fail
    await act(async () => {
      await vi.advanceTimersByTimeAsync(1000);
    });

    // Advance another 1s to let second retry fire and fail
    await act(async () => {
      await vi.advanceTimersByTimeAsync(1000);
    });

    // Since we failed 3 times in total (attempt 0, retry 1, retry 2), we should see the flaky network notification
    expect(screen.getByTestId("flaky-network-error")).toBeInTheDocument();
    expect(screen.getByText(/Flaky network: Request timed out. Click retry to resume safely./)).toBeInTheDocument();
    expect(screen.getByTestId("workflow-status-badge")).toHaveTextContent("Failed");

    // Change mock to resolve on next attempt
    onApproveMock.mockResolvedValue({ ok: true });

    // Trigger manual retry
    const retryButton = screen.getByTestId("error-retry-btn");
    await act(async () => {
      fireEvent.click(retryButton);
    });

    await act(async () => {
      await vi.advanceTimersByTimeAsync(0);
    });

    expect(screen.getByTestId("workflow-status-badge")).toHaveTextContent("Running");
    expect(screen.getByTestId("badge-running-dwf-1-plan")).toBeInTheDocument();
  });
});
