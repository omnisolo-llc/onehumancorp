import { describe, expect, test } from "vitest";
import {
  privateJson,
  taskBackendPath,
  taskBackendPathFromBody,
  withTaskMutationEnvelope,
} from "./assistantBackend";

describe("assistant backend helpers", () => {
  test("confines task identifiers used in backend paths", () => {
    expect(taskBackendPath("task-7", "/artifacts")).toBe("/api/assistant/tasks/task-7/artifacts");
    expect(() => taskBackendPath("../admin", "/artifacts")).toThrow("invalid task ID");
    expect(taskBackendPathFromBody("/files")(new TextEncoder().encode('{"taskId":"task-7"}'))).toBe(
      "/api/assistant/tasks/task-7/files",
    );
  });

  test("marks local JSON responses private", () => {
    const response = privateJson(200, { ok: true });

    expect(response.headers.get("cache-control")).toBe("private, no-store");
    expect(response.headers.get("pragma")).toBe("no-cache");
    expect(response.headers.get("x-content-type-options")).toBe("nosniff");
  });

  test("preserves the task mutation response contract", async () => {
    const updated = await withTaskMutationEnvelope(
      Promise.resolve(Response.json({ id: "task-7", status: "running" })),
    );
    const deleted = await withTaskMutationEnvelope(
      Promise.resolve(Response.json({ deletedTask: { id: "task-7" } })),
    );
    const malformed = await withTaskMutationEnvelope(
      Promise.resolve(new Response("not json")),
    );

    expect(await updated.json()).toEqual({ task: { id: "task-7", status: "running" } });
    expect(await deleted.json()).toEqual({ deletedTask: { id: "task-7" } });
    expect(malformed.status).toBe(502);
  });
});
