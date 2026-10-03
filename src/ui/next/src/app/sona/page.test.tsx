import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import SonaPatternsPage from "./page";
import { vi, describe, it, expect, beforeEach } from "vitest";
import React from 'react';

global.fetch = vi.fn();

describe("SonaPatternsPage", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders loading initially", () => {
    vi.mocked(global.fetch, { partial: true }).mockImplementationOnce(() => new Promise(() => {}));
    render(<SonaPatternsPage />);
    expect(screen.getByText("Loading patterns...")).toBeInTheDocument();
  });

  it("renders patterns", async () => {
    vi.mocked(global.fetch, { partial: true }).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ patterns: [{ id: "1", initial_context: "Test context", outcome_score: 0.9, successful_tools: ["tool1"] }] })
    });

    render(<SonaPatternsPage />);

    await waitFor(() => {
      expect(screen.getByText("Test context")).toBeInTheDocument();
      expect(screen.getByText("1. tool1")).toBeInTheDocument();
    });
  });

  it("handles empty patterns", async () => {
    vi.mocked(global.fetch, { partial: true }).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ patterns: [] })
    });

    render(<SonaPatternsPage />);

    await waitFor(() => {
      expect(screen.getByText("No patterns recorded yet.")).toBeInTheDocument();
    });
  });

  it("submits a new pattern", async () => {
    vi.mocked(global.fetch, { partial: true }).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ patterns: [] })
    });

    render(<SonaPatternsPage />);

    await waitFor(() => {
      expect(screen.getByText("No patterns recorded yet.")).toBeInTheDocument();
    });

    let submitted: unknown;
    vi.mocked(global.fetch)
      .mockImplementationOnce(async (_url, init) => {
        submitted = JSON.parse(String(init?.body));
        return Response.json({ status: 'success' });
      })
      .mockImplementationOnce(async () => Response.json({ patterns: [submitted] }));

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

it('shows an unavailable read instead of falsely reporting an empty pattern store', async () => {
  vi.mocked(fetch).mockResolvedValueOnce(Response.json({ error: 'Agent runtime is not configured; no work was dispatched' }, { status: 503 }));
  render(<SonaPatternsPage />);
  expect(await screen.findByRole('alert')).toHaveTextContent('Agent runtime is not configured; no work was dispatched');
  expect(screen.queryByText('No patterns recorded yet.')).not.toBeInTheDocument();
});

it('retains inputs and never invents a pattern when recording is rejected', async () => {
  vi.mocked(fetch).mockResolvedValueOnce(Response.json({ patterns: [] }));
  render(<SonaPatternsPage />);
  await screen.findByText('No patterns recorded yet.');
  vi.mocked(fetch).mockResolvedValueOnce(Response.json({ error: 'Runtime unavailable' }, { status: 503 }));
  fireEvent.change(screen.getByPlaceholderText('Task Context (e.g. Fix null pointer)'), { target: { value: 'Unsaved task' } });
  fireEvent.change(screen.getByPlaceholderText('Tool used (e.g. edit_file)'), { target: { value: 'bash' } });
  fireEvent.click(screen.getByRole('button', { name: 'Record Pattern' }));
  expect(await screen.findByRole('alert')).toHaveTextContent('Runtime unavailable');
  expect(screen.queryByRole('heading', { name: 'Unsaved task' })).not.toBeInTheDocument();
  expect(screen.getByPlaceholderText('Task Context (e.g. Fix null pointer)')).toHaveValue('Unsaved task');
  expect(screen.getByPlaceholderText('Tool used (e.g. edit_file)')).toHaveValue('bash');
});

it('requires readback of the submitted pattern before clearing its inputs', async () => {
  vi.mocked(fetch).mockResolvedValueOnce(Response.json({ patterns: [] }));
  render(<SonaPatternsPage />);
  await screen.findByText('No patterns recorded yet.');
  vi.mocked(fetch).mockResolvedValueOnce(Response.json({ status: 'success' })).mockResolvedValueOnce(Response.json({ patterns: [] }));
  fireEvent.change(screen.getByPlaceholderText('Task Context (e.g. Fix null pointer)'), { target: { value: 'Missing readback' } });
  fireEvent.change(screen.getByPlaceholderText('Tool used (e.g. edit_file)'), { target: { value: 'bash' } });
  fireEvent.click(screen.getByRole('button', { name: 'Record Pattern' }));
  expect(await screen.findByRole('alert')).toHaveTextContent('Pattern recording was not confirmed by the runtime');
  expect(screen.queryByRole('heading', { name: 'Missing readback' })).not.toBeInTheDocument();
  expect(screen.getByPlaceholderText('Task Context (e.g. Fix null pointer)')).toHaveValue('Missing readback');
});
