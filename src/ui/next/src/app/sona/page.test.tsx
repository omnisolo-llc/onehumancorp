import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import SonaPatternsPage from "./page";
import { vi, describe, it, expect, beforeEach } from "vitest";
import "@testing-library/jest-dom/vitest";
import { act } from "react";
import React from 'react';

global.fetch = vi.fn();

describe("SonaPatternsPage", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders loading initially", () => {
    (global.fetch as any).mockImplementationOnce(() => new Promise(() => {}));
    act(() => { render(<SonaPatternsPage />); });
    expect(screen.getByText("Loading patterns...")).toBeInTheDocument();
  });

  it("renders patterns", async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ patterns: [{ id: "1", initial_context: "Test context", outcome_score: 0.9, successful_tools: ["tool1"] }] })
    });

    act(() => { render(<SonaPatternsPage />); });

    await waitFor(() => {
      expect(screen.getByText("Test context")).toBeInTheDocument();
      expect(screen.getByText("1. tool1")).toBeInTheDocument();
    });
  });

  it("handles empty patterns", async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ patterns: [] })
    });

    act(() => { render(<SonaPatternsPage />); });

    await waitFor(() => {
      expect(screen.getByText("No patterns recorded yet.")).toBeInTheDocument();
    });
  });

  it("submits a new pattern", async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ patterns: [] })
    });

    act(() => { render(<SonaPatternsPage />); });

    await waitFor(() => {
      expect(screen.getByText("No patterns recorded yet.")).toBeInTheDocument();
    });

    (global.fetch as any)
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({})
      })
      .mockResolvedValueOnce({
        ok: true,
        json: async () => ({ patterns: [{ id: "2", initial_context: "New task", outcome_score: 1.0, successful_tools: ["new_tool"] }] })
      });

    const contextInput = screen.getByPlaceholderText("Task Context (e.g. Fix null pointer)");
    const toolInput = screen.getByPlaceholderText("Tool used (e.g. edit_file)");
    fireEvent.change(contextInput, { target: { value: "New task" } });
    fireEvent.change(toolInput, { target: { value: "new_tool" } });

    const submitBtn = screen.getByText("Record Pattern");
    fireEvent.click(submitBtn);

    await waitFor(() => {
      expect(screen.getByText("New task")).toBeInTheDocument();
    });
  });
});
