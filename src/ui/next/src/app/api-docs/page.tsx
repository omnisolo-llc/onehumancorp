"use client";

import React, { useEffect, useState } from "react";
import SwaggerUI from "swagger-ui-react";
import "swagger-ui-react/swagger-ui.css";

// OpenAPI spec for OHC backend
const swaggerSpec = {
  openapi: "3.0.0",
  info: {
    title: "OHC Connection Reference",
    version: "1.0.0",
    description: "Guide for advanced users connecting tools with OneHumanCorp.",
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
        description: "Registers a new organization.",
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
        summary: "Assign a task",
        description: "Assigns a new task to your AI team.",
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
    "/api/agents/status": {
      get: {
        summary: "Get team status",
        description: "Retrieves the current status of your AI team.",
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
      {mounted && <SwaggerUI spec={swaggerSpec} />}
    </div>
  );
}
