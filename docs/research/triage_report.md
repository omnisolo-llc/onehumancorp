---
issue_category: bug
title: "Agent Missions Noise and Hybrid Probe Triage"
author: "MAINTAINER Triage Agent"
---
<div style="backdrop-filter: blur(15px) saturate(200%); background: rgba(255, 255, 255, 0.03) !important; border: 1px solid rgba(255, 255, 255, 0.1) !important; border-radius: 12px; padding: 24px; font-family: 'Outfit', 'Inter', sans-serif;">
  <h2 style="color: #fff;">MAINTAINER Triage Report</h2>
  <hr style="border-color: rgba(255, 255, 255, 0.1);" />
  <h3 style="color: #e0e0e0;">Identified Faults</h3>
  <ul>
    <li><strong>Bug:</strong> `PruneStaleMissions` caused a tight infinite loop between STUCK and PENDING statuses due to checking the static `created_at` field, generating massive telemetry noise and log spam.</li>
    <li><strong>Bug:</strong> `handleHybridHealthCheck` failed to expose the `cloud_connected` probe status, obscuring hybrid-mode health signals.</li>
  </ul>
  <h3 style="color: #e0e0e0;">Resolutions</h3>
  <ul>
    <li>Refactored `PruneStaleMissions` to use `updated_at < $1` for the STUCK threshold query.</li>
    <li>Added `cloud_connected` to the payload of `/api/health/hybrid`.</li>
  </ul>
  <p style="color: #b0b0b0;"><strong>Status:</strong> Resolved</p>
</div>
