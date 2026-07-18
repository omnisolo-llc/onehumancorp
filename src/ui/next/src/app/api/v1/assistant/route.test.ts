import { describe, expect, test } from "vitest";
import * as approvals from "./approvals/route";
import * as automations from "./automations/route";
import * as claw from "./claw/route";
import * as cloud from "./cloud/route";
import * as commands from "./commands/route";
import * as data from "./data/route";
import * as experts from "./experts/route";
import * as explore from "./explore/route";
import * as mcp from "./mcp/route";
import * as models from "./models/route";
import * as parity from "./parity/route";
import * as permissions from "./permissions/route";
import * as plugins from "./plugins/route";
import * as previews from "./previews/route";
import * as remote from "./remote/route";
import * as settings from "./settings/route";
import * as share from "./share/route";
import * as support from "./support/route";
import * as uploads from "./uploads/route";

type Handler = () => Response | Promise<Response>;

const unavailableRoutes: Array<[string, Handler[]]> = [
  ["approvals", [approvals.GET, approvals.POST, approvals.PATCH]],
  ["automations", [automations.GET, automations.POST, automations.PATCH]],
  ["claw", [claw.GET, claw.PATCH]],
  ["cloud", [cloud.GET, cloud.POST, cloud.PATCH]],
  ["commands", [commands.GET, commands.POST]],
  ["data", [data.GET, data.PATCH]],
  ["experts", [experts.GET, experts.POST, experts.PATCH]],
  ["explore", [explore.GET, explore.POST, explore.PATCH]],
  ["mcp", [mcp.GET, mcp.POST, mcp.PATCH]],
  ["models", [models.GET, models.PATCH]],
  ["parity", [parity.GET]],
  ["permissions", [permissions.GET, permissions.PATCH]],
  ["plugins", [plugins.GET, plugins.PATCH]],
  ["previews", [previews.GET, previews.PATCH]],
  ["remote", [remote.GET, remote.POST]],
  ["settings", [settings.GET, settings.PATCH]],
  ["share", [share.GET, share.POST, share.PATCH]],
  ["support", [support.POST]],
  ["uploads", [uploads.GET, uploads.POST]],
];

describe("assistant API authority", () => {
  test.each(unavailableRoutes)("%s fails closed without a persistent backend", async (_name, handlers) => {
    for (const handler of handlers) {
      const response = await handler();
      expect(response.status).toBe(501);
      await expect(response.json()).resolves.toEqual(
        expect.objectContaining({ error: expect.any(String) }),
      );
    }
  });
});
