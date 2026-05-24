"use client";

import React, { useEffect, useState } from "react";
import SwaggerUI from "swagger-ui-react";
import "swagger-ui-react/swagger-ui.css";

// OpenAPI spec for OHC backend
const swaggerSpec = {
  openapi: "3.0.0",
  info: {
    title: "OHC Advanced API Reference",
    version: "1.0.0",
    description: "API Reference for advanced users integrating with OneHumanCorp.",
  },
  servers: [
    {
      url: "http://localhost:8080",
      description: "Local Backend Server"
    }
  ],
  paths: {
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

export default function ApiDocsPage() {
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  return (
    <div className="min-h-screen bg-white">
      <div className="bg-yellow-50 border-l-4 border-yellow-400 p-4 mb-4">
        <div className="flex">
          <div className="flex-shrink-0">
            <svg className="h-5 w-5 text-yellow-400" viewBox="0 0 20 20" fill="currentColor">
              <path fillRule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
            </svg>
          </div>
          <div className="ml-3">
            <p className="text-sm text-yellow-700">
              <strong>Advanced section:</strong> Not promoted to new users. For advanced users integrating with OneHumanCorp directly.
            </p>
          </div>
        </div>
      </div>
      {mounted && <SwaggerUI spec={swaggerSpec} />}
    </div>
  );
}
