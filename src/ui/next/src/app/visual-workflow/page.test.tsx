import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import VisualWorkflowPage from "./page";
import { vi, describe, it, expect } from "vitest";

// Mock the walkthrough context
vi.mock("../../components/help", () => ({
  useWalkthrough: () => ({ startWalkthrough: vi.fn() }),
}));
vi.mock("../../components/Walkthrough", () => ({
  WalkthroughTarget: ({ children }: any) => <>{children}</>,
}));

global.fetch = vi.fn();

describe("VisualWorkflowPage", () => {
  it("renders the page and allows adding an LLM node", () => {
    render(<VisualWorkflowPage />);
    expect(screen.getByText("Visual Workflow Orchestrator")).toBeInTheDocument();

    const addButton = screen.getByText("+ Add LLM Node");
    fireEvent.click(addButton);

    expect(screen.getByText(/node-1/)).toBeInTheDocument();
  });

  it("allows adding an Input node", () => {
    render(<VisualWorkflowPage />);
    const addButton = screen.getByText("+ Add Input Node");
    fireEvent.click(addButton);

    expect(screen.getByText(/node-1/)).toBeInTheDocument();
  });

  it("allows adding an Output node", () => {
    render(<VisualWorkflowPage />);
    const addButton = screen.getByText("+ Add Output Node");
    fireEvent.click(addButton);

    expect(screen.getByText(/node-1/)).toBeInTheDocument();
  });

  it("allows adding an edge between nodes", () => {
    render(<VisualWorkflowPage />);
    fireEvent.click(screen.getByText("+ Add Input Node"));
    fireEvent.click(screen.getByText("+ Add LLM Node"));

    expect(screen.getByText(/node-1/)).toBeInTheDocument();
    expect(screen.getByText(/node-2/)).toBeInTheDocument();

    const connectButton = screen.getByText("Connect from previous");
    fireEvent.click(connectButton);

    expect(screen.getByText("node-1 → node-2")).toBeInTheDocument();
  });

  it("calls fetch when running the workflow", async () => {
    (global.fetch as any).mockResolvedValueOnce({
      json: async () => ({ success: true, result: "mocked result" })
    });

    render(<VisualWorkflowPage />);
    fireEvent.click(screen.getByText("Run Workflow"));

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith("/api/workflow/run", expect.any(Object));
      expect(screen.getByText(/mocked result/)).toBeInTheDocument();
    });
  });

  it("handles fetch errors when running the workflow", async () => {
    (global.fetch as any).mockRejectedValueOnce(new Error("Network Error"));

    render(<VisualWorkflowPage />);
    fireEvent.click(screen.getByText("Run Workflow"));

    await waitFor(() => {
      expect(screen.getByText(/Error: Network Error/)).toBeInTheDocument();
    });
  });

  it("verifies macOS Translucent Glass styling", () => {
    render(<VisualWorkflowPage />);
    // Verify TDD check for the new Translucent Glass container that wraps nodes
    const container = screen.getByTestId("glass-nodes-container");
    expect(container).toHaveClass("backdrop-blur-2xl");
    expect(container).toHaveClass("bg-white/40");
  });
});
