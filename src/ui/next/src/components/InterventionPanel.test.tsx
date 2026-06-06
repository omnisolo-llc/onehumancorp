import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { InterventionPanel } from "./InterventionPanel";
import { expect, test, vi } from "vitest";

test("InterventionPanel renders reason and handles input", async () => {
  const onResolve = vi.fn().mockResolvedValue(undefined);
  const onClose = vi.fn();

  render(
    <InterventionPanel
      taskId="task_123"
      toolCallId="call_456"
      reason="Please provide your API key to continue."
      onResolve={onResolve}
      onClose={onClose}
    />
  );

  expect(screen.getByText(/Please provide your API key to continue/)).toBeDefined();

  const textarea = screen.getByPlaceholderText(/Provide information or instructions/);
  fireEvent.change(textarea, { target: { value: "my-secret-key" } });

  const submitButton = screen.getByText("Send to Agent");
  fireEvent.click(submitButton);

  await waitFor(() => {
    expect(onResolve).toHaveBeenCalledWith("my-secret-key", "input");
  });
});

test("InterventionPanel handles quick approve", async () => {
  const onResolve = vi.fn().mockResolvedValue(undefined);
  const onClose = vi.fn();

  render(
    <InterventionPanel
      taskId="task_123"
      toolCallId="call_456"
      reason="Is this okay?"
      onResolve={onResolve}
      onClose={onClose}
    />
  );

  const approveButton = screen.getByText("Quick Approve");
  fireEvent.click(approveButton);

  await waitFor(() => {
    expect(onResolve).toHaveBeenCalledWith("", "approve");
  });
});

test("InterventionPanel handles abort", async () => {
  const onResolve = vi.fn().mockResolvedValue(undefined);
  const onClose = vi.fn();

  render(
    <InterventionPanel
      taskId="task_123"
      toolCallId="call_456"
      reason="Should I proceed?"
      onResolve={onResolve}
      onClose={onClose}
    />
  );

  const abortButton = screen.getByText("Abort Task");
  fireEvent.click(abortButton);

  await waitFor(() => {
    expect(onResolve).toHaveBeenCalledWith("", "abort");
  });
});
