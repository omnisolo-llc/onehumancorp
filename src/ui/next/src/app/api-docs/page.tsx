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
    <div className="min-h-screen bg-gray-50 p-6 font-inter relative overflow-hidden">
      {/* Background blobs for premium OHC look */}
      <div className="absolute top-[-10%] left-[-10%] w-[500px] h-[500px] rounded-full bg-blue-300/30 blur-[100px] pointer-events-none mix-blend-multiply"></div>
      <div className="absolute bottom-[-10%] right-[-10%] w-[600px] h-[600px] rounded-full bg-purple-300/20 blur-[120px] pointer-events-none mix-blend-multiply"></div>

      <div className="max-w-5xl mx-auto relative z-10">
        <div className="bg-yellow-50/90 backdrop-blur-[20px] saturate-200 border border-yellow-200 rounded-xl p-4 mb-6 shadow-sm">
          <p className="text-yellow-800 text-sm flex items-center gap-2">
            <svg className="w-5 h-5 text-yellow-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
            <strong>Advanced:</strong> This section is for developers directly integrating with our APIs. Not required for normal use.
          </p>
        </div>

        <div className="bg-white/80 backdrop-blur-[20px] saturate-200 rounded-2xl shadow-xl border border-white/50 overflow-hidden">
          <div className="p-4 bg-gray-900 text-white flex justify-between items-center backdrop-blur-md">
            <h1 className="text-lg font-outfit font-bold">OHC API Explorer</h1>
            <span className="bg-blue-600 px-3 py-1 text-xs rounded-full font-semibold shadow-inner">v1.0.0</span>
          </div>
          <div className="p-6">
            <div className="swagger-container custom-swagger-styles">
              {mounted && <SwaggerUI spec={swaggerSpec} />}
            </div>
          </div>
        </div>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        .custom-swagger-styles .swagger-ui .info {
          margin: 0 0 20px 0;
        }
        .custom-swagger-styles .swagger-ui .info .title {
          font-family: 'Outfit', sans-serif;
          color: #111827;
        }
        .custom-swagger-styles .swagger-ui {
          font-family: 'Inter', sans-serif;
        }
        .custom-swagger-styles .swagger-ui .opblock {
          border-radius: 12px;
          box-shadow: 0 2px 4px rgba(0,0,0,0.05);
          border: 1px solid rgba(0,0,0,0.05);
          margin-bottom: 16px;
        }
        .custom-swagger-styles .swagger-ui .btn {
          border-radius: 8px;
          font-weight: 600;
        }
        .custom-swagger-styles .swagger-ui .opblock .opblock-summary-method {
          border-radius: 6px;
        }
      `}} />
    </div>
  );
}
