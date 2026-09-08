import { beforeEach, describe, expect, it, vi } from "vitest";

const { proxyBackendRequest } = vi.hoisted(() => ({ proxyBackendRequest: vi.fn() }));
vi.mock("@/lib/auth/backendTransport", () => ({ proxyBackendRequest }));

import { GET, POST } from "./route";

const backendTask = {
  id: "task-1",
  workspace_id: "personal-os",
  title: "Build a test report",
  prompt: "Build a test report",
  status: "planning",
  mode: "Plan",
  permission_profile: "Guarded",
  model_config_json: { model: "Auto", provider: "Auto" },
  current_step: "Task created",
  archived: false,
  created_at_unix: 1_786_315_200,
  updated_at_unix: 1_786_315_200,
};

describe("assistant task BFF contract", () => {
  beforeEach(() => proxyBackendRequest.mockReset());

  it("maps backend task records into the assistant page list contract", async () => {
    proxyBackendRequest.mockResolvedValue(Response.json([backendTask]));
    const response = await GET(new Request("https://app.example.test/api/v1/assistant/tasks"));

    expect(response.status).toBe(200);
    await expect(response.json()).resolves.toMatchObject({
      tasks: [{ id: "task-1", title: "Build a test report", currentStep: "Task created" }],
      capabilities: { workModes: ["Ask", "Agent", "Plan", "Coding"] },
    });
  });

  it("creates the complete backend record required by the Rust API and returns a UI task envelope", async () => {
    proxyBackendRequest.mockResolvedValue(Response.json(backendTask));
    const request = new Request("https://app.example.test/api/v1/assistant/tasks", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({
        prompt: "Build a test report",
        workspace: "Personal OS",
        mode: "Plan",
        model: "Auto",
        permissionProfile: "Guarded",
      }),
    });

    const response = await POST(request);
    const options = proxyBackendRequest.mock.calls[0]?.[2];
    const transformed = JSON.parse(new TextDecoder().decode(options.transformRequestBody(
      new TextEncoder().encode(await request.text()),
    )));

    expect(transformed).toMatchObject({
      workspace_id: "personal-os",
      title: "Build a test report",
      prompt: "Build a test report",
      status: "planning",
      permission_profile: "Guarded",
      archived: false,
    });
    expect(transformed.id).toMatch(/^[0-9a-f-]{36}$/);
    expect(response.status).toBe(201);
    await expect(response.json()).resolves.toMatchObject({ task: { id: "task-1", title: "Build a test report" } });
  });
});
