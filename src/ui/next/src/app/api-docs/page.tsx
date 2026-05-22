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
    contact: {
      name: "OHC Developer Support",
      url: "https://ohc.store/support"
    }
  },
  servers: [
    {
      url: "https://api.ohc.store",
      description: "Production API"
    },
    {
      url: "http://localhost:8080",
      description: "Local Backend Server"
    }
  ],
  tags: [
    { name: "Storefront", description: "Manage your products and orders" },
    { name: "AI Swarm", description: "Interact with your AI workforce" },
    { name: "Payments", description: "Billing and transaction history" }
  ],
  paths: {
    "/api/v1/builder/generate": {
      post: {
        summary: "Generate Storefront",
        description: "Generates a complete storefront draft based on a natural language description.",
        tags: ["Storefront"],
        requestBody: {
          required: true,
          content: {
            "application/json": {
              schema: {
                type: "object",
                properties: {
                  description: { type: "string", example: "I sell organic dog treats in Seattle." }
                }
              }
            }
          }
        },
        responses: {
          "200": {
            description: "Storefront draft created",
            content: {
              "application/json": {
                schema: {
                  type: "object",
                  properties: {
                    pages: { type: "array", items: { type: "object" } },
                    theme: { type: "object" }
                  }
                }
              }
            }
          }
        }
      }
    },
    "/api/agents/approvals": {
      get: {
        summary: "List Pending Approvals",
        description: "Retrieves a list of tasks that require human approval before the AI Swarm continues.",
        tags: ["AI Swarm"],
        responses: {
          "200": {
            description: "Success",
            content: {
              "application/json": {
                schema: {
                  type: "object",
                  properties: {
                    pending_approvals: {
                       type: "array",
                       items: {
                          type: "object",
                          properties: {
                             id: { type: "string" },
                             department: { type: "string" },
                             description: { type: "string" }
                          }
                       }
                    }
                  }
                }
              }
            }
          }
        }
      }
    },
    "/api/v1/dashboard/sales": {
      post: {
        summary: "Get Sales Metrics",
        description: "Retrieves real-time sales data for a specific tenant.",
        tags: ["Payments"],
        requestBody: {
          required: true,
          content: {
            "application/json": {
              schema: {
                type: "object",
                properties: {
                  tenant_id: { type: "string", example: "acme-corp" }
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
                         total_sales: { type: "number" }
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
