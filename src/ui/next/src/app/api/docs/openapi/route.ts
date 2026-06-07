import { NextResponse } from 'next/server';

export async function GET() {
  const spec = {
    openapi: "3.0.0",
    info: {
      title: "OHC Advanced API Reference",
      version: "1.0.0",
      description: "API Reference for advanced users integrating with OneHumanCorp.",
    },
    servers: [
      {
        url: "http://localhost:8080",
        description: "Backend Server"
      }
    ],
    paths: {
      "/api/help": {
        get: {
          summary: "Get Help Articles",
          description: "Retrieves a list of available help articles for the Help Center.",
          tags: ["Documentation"],
          responses: {
            "200": {
              description: "Success",
              content: {
                "application/json": {
                  schema: {
                    type: "array",
                    items: {
                      type: "object",
                      properties: {
                        title: { type: "string" },
                        desc: { type: "string" },
                        link: { type: "string" }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      },
      "/api/tooltips": {
        get: {
          summary: "Get Tooltips Registry",
          description: "Retrieves the key-value dictionary of all UI tooltips.",
          tags: ["Documentation"],
          responses: {
            "200": {
              description: "Success",
              content: {
                "application/json": {
                  schema: {
                    type: "object",
                    additionalProperties: { type: "string" }
                  }
                }
              }
            }
          }
        }
      },
      "/api/orgs/register": {
        post: {
          summary: "Register an Organization",
          description: "Registers a new tenant organization in the multi-tenant OHC environment.",
          tags: ["Tenants"],
          requestBody: {
            required: true,
            content: {
              "application/json": {
                schema: {
                  type: "object",
                  properties: {
                    id: { type: "string", example: "acme" },
                    name: { type: "string", example: "Acme Corp" },
                    domain: { type: "string", example: "acme.com" }
                  }
                }
              }
            }
          },
          responses: {
            "200": {
              description: "Success",
              content: {
                "application/json": {
                  schema: {
                    type: "object",
                    properties: {
                      success: { type: "boolean" },
                      tenant_id: { type: "string" },
                      message: { type: "string" }
                    }
                  }
                }
              }
            }
          }
        }
      },
      "/api/agents/task": {
        post: {
          summary: "Dispatch a task",
          description: "Dispatches a new task to the AI Swarm Orchestrator.",
          tags: ["Agents"],
          requestBody: {
            required: true,
            content: {
              "application/json": {
                schema: {
                  type: "object",
                  properties: {
                    task_description: { type: "string", example: "Build a landing page for a dog groomer" },
                    priority: { type: "string", example: "high" }
                  }
                }
              }
            }
          },
          responses: {
            "202": {
              description: "Accepted",
              content: {
                "application/json": {
                  schema: {
                    type: "object",
                    properties: {
                      task_id: { type: "string" },
                      status: { type: "string" }
                    }
                  }
                }
              }
            }
          }
        }
      },
      "/api/videos": {
        get: {
          summary: "Get video tutorials",
          description: "Retrieves a list of video tutorial metadata for the Help Center.",
          tags: ["Documentation"],
          responses: {
            "200": {
              description: "Success",
              content: {
                "application/json": {
                  schema: {
                    type: "array",
                    items: {
                      type: "object",
                      properties: {
                        id: { type: "integer" },
                        title: { type: "string" },
                        duration: { type: "string" }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      },
      "/api/agents/status": {
        get: {
          summary: "Get workforce status",
          description: "Retrieves the current status of the agent swarm workforce.",
          tags: ["Agents"],
          parameters: [
            {
              name: "tenant_id",
              in: "query",
              description: "Optional. Filter by organization.",
              required: false,
              schema: {
                type: "string"
              }
            }
          ],
          responses: {
            "200": {
              description: "Success",
              content: {
                "application/json": {
                  schema: {
                    type: "object",
                    properties: {
                      active_agents: { type: "integer" },
                      queued_tasks: { type: "integer" },
                      system_health: { type: "string" }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  };

  return NextResponse.json(spec);
}
