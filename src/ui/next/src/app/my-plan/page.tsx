"use client";
import React, { useState, useEffect } from "react";

function ProjectedSavingsPanel() {
    return (
        <div style={{ marginTop: "40px", border: "1px solid #10b981", background: "rgba(16, 185, 129, 0.05)", borderRadius: "12px", padding: "30px" }}>
            <div style={{ display: "flex", justifyContent: "space-between", alignItems: "flex-start", marginBottom: "20px" }}>
                <div>
                    <h3 style={{ marginTop: 0, marginBottom: "10px", color: "#065f46" }}>Cost Savings Engine Active</h3>
                    <p style={{ color: "#064e3b", margin: 0 }}>OHC has automatically optimized your background tasks.</p>
                </div>
                <div style={{ fontSize: "2rem" }}>💸</div>
            </div>

            <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(200px, 1fr))", gap: "20px" }}>
                <div style={{ background: "white", padding: "15px", borderRadius: "8px", border: "1px solid #10b981" }}>
                    <div style={{ fontSize: "0.9rem", color: "#666", marginBottom: "5px" }}>Prompt Caching Saved</div>
                    <div style={{ fontSize: "1.5rem", fontWeight: "bold", color: "#10b981" }}>$12.40</div>
                </div>
                <div style={{ background: "white", padding: "15px", borderRadius: "8px", border: "1px solid #10b981" }}>
                    <div style={{ fontSize: "0.9rem", color: "#666", marginBottom: "5px" }}>Image Optimization Saved</div>
                    <div style={{ fontSize: "1.5rem", fontWeight: "bold", color: "#10b981" }}>3.2 GB</div>
                </div>
                <div style={{ background: "white", padding: "15px", borderRadius: "8px", border: "1px solid #10b981" }}>
                    <div style={{ fontSize: "0.9rem", color: "#666", marginBottom: "5px" }}>Recommended Action</div>
                    <button style={{ padding: "5px 10px", background: "#10b981", color: "white", border: "none", borderRadius: "4px", fontSize: "0.9rem", cursor: "pointer", width: "100%" }}>Switch to ACH (Save 2.1%)</button>
                </div>
            </div>
        </div>
    );
}

function StorageDetailedBreakdown() {
    return (
        <div style={{ marginTop: "40px", border: "1px solid #eaeaea", borderRadius: "12px", padding: "30px" }}>
            <h3 style={{ marginTop: 0, marginBottom: "20px" }}>Storage Breakdown</h3>
            <div style={{ display: "flex", gap: "15px", marginBottom: "20px", height: "24px", borderRadius: "12px", overflow: "hidden" }}>
                <div style={{ width: "60%", background: "#3b82f6" }} title="Product Images"></div>
                <div style={{ width: "25%", background: "#10b981" }} title="Vector Embeddings"></div>
                <div style={{ width: "10%", background: "#f59e0b" }} title="Documents"></div>
                <div style={{ width: "5%", background: "#eaeaea" }} title="Free Space"></div>
            </div>

            <ul style={{ listStyle: "none", padding: 0, margin: 0, display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(200px, 1fr))", gap: "20px" }}>
                <li style={{ display: "flex", alignItems: "center", gap: "10px" }}>
                    <span style={{ display: "inline-block", width: "12px", height: "12px", borderRadius: "50%", background: "#3b82f6" }}></span>
                    <div>
                        <div style={{ fontWeight: "bold" }}>Product Images</div>
                        <div style={{ fontSize: "0.85rem", color: "#666" }}>720 MB (60%)</div>
                    </div>
                </li>
                <li style={{ display: "flex", alignItems: "center", gap: "10px" }}>
                    <span style={{ display: "inline-block", width: "12px", height: "12px", borderRadius: "50%", background: "#10b981" }}></span>
                    <div>
                        <div style={{ fontWeight: "bold" }}>Vector Embeddings</div>
                        <div style={{ fontSize: "0.85rem", color: "#666" }}>300 MB (25%)</div>
                    </div>
                </li>
                <li style={{ display: "flex", alignItems: "center", gap: "10px" }}>
                    <span style={{ display: "inline-block", width: "12px", height: "12px", borderRadius: "50%", background: "#f59e0b" }}></span>
                    <div>
                        <div style={{ fontWeight: "bold" }}>Documents</div>
                        <div style={{ fontSize: "0.85rem", color: "#666" }}>120 MB (10%)</div>
                    </div>
                </li>
                <li style={{ display: "flex", alignItems: "center", gap: "10px" }}>
                    <span style={{ display: "inline-block", width: "12px", height: "12px", borderRadius: "50%", background: "#eaeaea" }}></span>
                    <div>
                        <div style={{ fontWeight: "bold" }}>Available Free Space</div>
                        <div style={{ fontSize: "0.85rem", color: "#666" }}>3.86 GB</div>
                    </div>
                </li>
            </ul>
        </div>
    );
}

function AIUsageDetailedBreakdown() {
    return (
        <div style={{ marginTop: "40px", border: "1px solid #eaeaea", borderRadius: "12px", padding: "30px" }}>
            <h3 style={{ marginTop: 0, marginBottom: "20px" }}>AI Usage by Agent</h3>
            <table style={{ width: "100%", borderCollapse: "collapse" }}>
                <thead>
                    <tr style={{ borderBottom: "2px solid #eaeaea", textAlign: "left" }}>
                        <th style={{ padding: "10px" }}>Agent Name</th>
                        <th style={{ padding: "10px" }}>Role</th>
                        <th style={{ padding: "10px" }}>Actions Used</th>
                        <th style={{ padding: "10px" }}>% of Quota</th>
                    </tr>
                </thead>
                <tbody>
                    <tr style={{ borderBottom: "1px solid #eaeaea" }}>
                        <td style={{ padding: "10px", fontWeight: "bold" }}>Marketing Coordinator</td>
                        <td style={{ padding: "10px", color: "#666" }}>Social Media & Email</td>
                        <td style={{ padding: "10px" }}>450</td>
                        <td style={{ padding: "10px" }}>
                            <div style={{ display: "flex", alignItems: "center", gap: "10px" }}>
                                <div style={{ width: "100px", height: "6px", background: "#eaeaea", borderRadius: "3px" }}>
                                    <div style={{ width: "45%", height: "100%", background: "#0070f3", borderRadius: "3px" }}></div>
                                </div>
                                <span>45%</span>
                            </div>
                        </td>
                    </tr>
                    <tr style={{ borderBottom: "1px solid #eaeaea" }}>
                        <td style={{ padding: "10px", fontWeight: "bold" }}>Support Representative</td>
                        <td style={{ padding: "10px", color: "#666" }}>Customer Inbox</td>
                        <td style={{ padding: "10px" }}>320</td>
                        <td style={{ padding: "10px" }}>
                            <div style={{ display: "flex", alignItems: "center", gap: "10px" }}>
                                <div style={{ width: "100px", height: "6px", background: "#eaeaea", borderRadius: "3px" }}>
                                    <div style={{ width: "32%", height: "100%", background: "#10b981", borderRadius: "3px" }}></div>
                                </div>
                                <span>32%</span>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td style={{ padding: "10px", fontWeight: "bold" }}>Data Analyst</td>
                        <td style={{ padding: "10px", color: "#666" }}>Reports & Metrics</td>
                        <td style={{ padding: "10px" }}>72</td>
                        <td style={{ padding: "10px" }}>
                            <div style={{ display: "flex", alignItems: "center", gap: "10px" }}>
                                <div style={{ width: "100px", height: "6px", background: "#eaeaea", borderRadius: "3px" }}>
                                    <div style={{ width: "7%", height: "100%", background: "#f59e0b", borderRadius: "3px" }}></div>
                                </div>
                                <span>7%</span>
                            </div>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    );
}

function UsageOptimizationHints() {
    return (
        <div style={{ marginTop: "40px", border: "1px solid #eaeaea", borderRadius: "12px", padding: "30px", background: "#fdfdfd" }}>
            <h3 style={{ marginTop: 0, marginBottom: "20px" }}>Optimization Suggestions</h3>
            <ul style={{ listStyle: "none", padding: 0, margin: 0 }}>
                <li style={{ display: "flex", alignItems: "flex-start", gap: "15px", padding: "15px 0", borderBottom: "1px solid #eaeaea" }}>
                    <span style={{ fontSize: "1.5rem" }}>💡</span>
                    <div>
                        <div style={{ fontWeight: "bold", marginBottom: "5px" }}>Enable aggressive image compression</div>
                        <p style={{ margin: 0, color: "#666", fontSize: "0.9rem", lineHeight: "1.5" }}>You are currently storing 720MB of product images. Enabling aggressive WebP compression could reduce this to ~350MB without noticeable quality loss.</p>
                        <button style={{ marginTop: "10px", padding: "5px 15px", background: "white", border: "1px solid #0070f3", color: "#0070f3", borderRadius: "4px", fontSize: "0.85rem", cursor: "pointer" }}>Enable Compression</button>
                    </div>
                </li>
                <li style={{ display: "flex", alignItems: "flex-start", gap: "15px", padding: "15px 0" }}>
                    <span style={{ fontSize: "1.5rem" }}>🤖</span>
                    <div>
                        <div style={{ fontWeight: "bold", marginBottom: "5px" }}>Switch default agent to Gemini 1.5 Flash</div>
                        <p style={{ margin: 0, color: "#666", fontSize: "0.9rem", lineHeight: "1.5" }}>Your Marketing Coordinator agent uses GPT-4o for tasks that Gemini 1.5 Flash could handle. Switching could reduce its token cost by 85%.</p>
                        <button style={{ marginTop: "10px", padding: "5px 15px", background: "white", border: "1px solid #0070f3", color: "#0070f3", borderRadius: "4px", fontSize: "0.85rem", cursor: "pointer" }}>Update Model Settings</button>
                    </div>
                </li>
            </ul>
        </div>
    );
}

function InvoicesList() {
    return (
        <div style={{ border: "1px solid #eaeaea", borderRadius: "12px", padding: "25px", marginTop: "30px" }}>
            <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "20px" }}>
                <h3 style={{ margin: 0 }}>Past Invoices</h3>
                <a href="#" style={{ color: "#0070f3", textDecoration: "none", fontWeight: "bold", fontSize: "0.9rem" }}>Download All (ZIP)</a>
            </div>
            <table style={{ width: "100%", borderCollapse: "collapse" }}>
                <thead>
                    <tr style={{ borderBottom: "2px solid #eaeaea", textAlign: "left", fontSize: "0.9rem", color: "#666" }}>
                        <th style={{ padding: "10px" }}>Date</th>
                        <th style={{ padding: "10px" }}>Amount</th>
                        <th style={{ padding: "10px" }}>Status</th>
                        <th style={{ padding: "10px" }}>Invoice</th>
                    </tr>
                </thead>
                <tbody>
                    <tr style={{ borderBottom: "1px solid #eaeaea" }}>
                        <td style={{ padding: "15px 10px", fontWeight: "bold" }}>Sep 1, 2026</td>
                        <td style={{ padding: "15px 10px" }}>$10.00</td>
                        <td style={{ padding: "15px 10px" }}><span style={{ background: "#e0f2fe", color: "#0369a1", padding: "4px 8px", borderRadius: "12px", fontSize: "0.8rem", fontWeight: "bold" }}>Paid</span></td>
                        <td style={{ padding: "15px 10px" }}><button style={{ background: "none", border: "none", color: "#0070f3", cursor: "pointer", padding: 0 }}>INV-2026-09</button></td>
                    </tr>
                    <tr style={{ borderBottom: "1px solid #eaeaea" }}>
                        <td style={{ padding: "15px 10px", fontWeight: "bold" }}>Aug 1, 2026</td>
                        <td style={{ padding: "15px 10px" }}>$10.00</td>
                        <td style={{ padding: "15px 10px" }}><span style={{ background: "#e0f2fe", color: "#0369a1", padding: "4px 8px", borderRadius: "12px", fontSize: "0.8rem", fontWeight: "bold" }}>Paid</span></td>
                        <td style={{ padding: "15px 10px" }}><button style={{ background: "none", border: "none", color: "#0070f3", cursor: "pointer", padding: 0 }}>INV-2026-08</button></td>
                    </tr>
                    <tr style={{ borderBottom: "1px solid #eaeaea" }}>
                        <td style={{ padding: "15px 10px", fontWeight: "bold" }}>Jul 1, 2026</td>
                        <td style={{ padding: "15px 10px" }}>$10.00</td>
                        <td style={{ padding: "15px 10px" }}><span style={{ background: "#e0f2fe", color: "#0369a1", padding: "4px 8px", borderRadius: "12px", fontSize: "0.8rem", fontWeight: "bold" }}>Paid</span></td>
                        <td style={{ padding: "15px 10px" }}><button style={{ background: "none", border: "none", color: "#0070f3", cursor: "pointer", padding: 0 }}>INV-2026-07</button></td>
                    </tr>
                </tbody>
            </table>
        </div>
    );
}

export default function MyPlanPage() {
    const [viewingCostDetails, setViewingCostDetails] = useState(false);
    const [storageUsage, setStorageUsage] = useState(0);

    useEffect(() => {
        // Dynamic fetch example
        fetch('/api/v1/billing/storage')
            .then(r => r.ok ? r.json() : null)
            .then(data => { if (data) setStorageUsage(data.usedMb) })
            .catch(() => setStorageUsage(1200));
    }, []);

    if (viewingCostDetails) {
        return (
            <div style={{ padding: "40px", fontFamily: "Outfit, Inter, sans-serif" }}>
                <button onClick={() => setViewingCostDetails(false)} style={{ marginBottom: "20px", padding: "8px 16px", cursor: "pointer" }}>&larr; Back to My Plan</button>
                <h1>Cost & AI Usage Transparency</h1>

                <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: "20px", marginTop: "30px" }}>
                    <div style={{ border: "1px solid #eaeaea", padding: "20px", borderRadius: "8px" }}>
                        <h3>AI Actions (This Month)</h3>
                        <div style={{ fontSize: "2rem", fontWeight: "bold", color: "#0070f3" }}>842 <span style={{ fontSize: "1rem", color: "#666" }}>/ 1,000</span></div>
                        <div style={{ width: "100%", height: "8px", background: "#eaeaea", borderRadius: "4px", marginTop: "10px" }}>
                            <div style={{ width: "84.2%", height: "100%", background: "#0070f3", borderRadius: "4px" }}></div>
                        </div>
                    </div>

                    <div style={{ border: "1px solid #eaeaea", padding: "20px", borderRadius: "8px" }}>
                        <h3>Storage Used: {(storageUsage / 1000).toFixed(1)}GB</h3>
                        <div style={{ fontSize: "2rem", fontWeight: "bold", color: "#0070f3" }}>{(storageUsage / 1000).toFixed(1)}GB <span style={{ fontSize: "1rem", color: "#666" }}>/ 5GB</span></div>
                        <div style={{ width: "100%", height: "8px", background: "#eaeaea", borderRadius: "4px", marginTop: "10px" }}>
                            <div style={{ width: `${Math.min(100, (storageUsage / 5000) * 100)}%`, height: "100%", background: "#0070f3", borderRadius: "4px" }}></div>
                        </div>
                    </div>
                </div>

                <h3 style={{ marginTop: "40px" }}>Cost Breakdown</h3>
                <table style={{ width: "100%", borderCollapse: "collapse", marginTop: "20px" }}>
                    <thead>
                        <tr style={{ borderBottom: "2px solid #eaeaea", textAlign: "left" }}>
                            <th style={{ padding: "10px" }}>Service</th>
                            <th style={{ padding: "10px" }}>Usage</th>
                            <th style={{ padding: "10px" }}>Estimated Cost</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr style={{ borderBottom: "1px solid #eaeaea" }}>
                            <td style={{ padding: "10px" }}>GPT-4o (Agent Core)</td>
                            <td style={{ padding: "10px" }}>450k tokens</td>
                            <td style={{ padding: "10px" }}>$4.50</td>
                        </tr>
                        <tr style={{ borderBottom: "1px solid #eaeaea" }}>
                            <td style={{ padding: "10px" }}>Claude 3.5 Sonnet</td>
                            <td style={{ padding: "10px" }}>120k tokens</td>
                            <td style={{ padding: "10px" }}>$0.36</td>
                        </tr>
                        <tr style={{ borderBottom: "1px solid #eaeaea" }}>
                            <td style={{ padding: "10px" }}>Asset Storage</td>
                            <td style={{ padding: "10px" }}>1.2 GB</td>
                            <td style={{ padding: "10px" }}>$0.02</td>
                        </tr>
                        <tr>
                            <td style={{ padding: "10px", fontWeight: "bold" }}>Total (Est.)</td>
                            <td></td>
                            <td style={{ padding: "10px", fontWeight: "bold" }}>$4.88</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        );
    }

    return (
        <div style={{ maxWidth: "1000px", margin: "0 auto", padding: "40px 20px", fontFamily: "Outfit, Inter, sans-serif" }}>
            <h1 style={{ fontSize: "2.5rem", marginBottom: "30px" }}>My Plan</h1>

            <div style={{ display: "grid", gridTemplateColumns: "2fr 1fr", gap: "30px" }}>
                <div>
                    <div style={{ border: "1px solid #eaeaea", borderRadius: "12px", padding: "30px", marginBottom: "30px" }}>
                        <div style={{ display: "flex", justifyContent: "space-between", alignItems: "flex-start", marginBottom: "20px" }}>
                            <div>
                                <h2 style={{ fontSize: "1.8rem", margin: 0 }}>Starter Plan</h2>
                                <p style={{ color: "green", fontWeight: "bold", marginTop: "5px", display: "flex", alignItems: "center", gap: "5px" }}>
                                    <span style={{ display: "inline-block", width: "8px", height: "8px", borderRadius: "50%", background: "green" }}></span>
                                    Active status
                                </p>
                            </div>
                            <div style={{ fontSize: "2rem", fontWeight: "bold" }}>
                                $10<span style={{ fontSize: "1rem", color: "#666", fontWeight: "normal" }}>/month</span>
                            </div>
                        </div>

                        <p style={{ color: "#666", marginBottom: "30px" }}>Your next billing renewal date is <strong>Oct 1, 2026</strong> for $10.00.</p>

                        <div style={{ display: "flex", gap: "15px" }}>
                            <button style={{ padding: "10px 20px", background: "#0070f3", color: "white", border: "none", borderRadius: "6px", fontWeight: "bold", cursor: "pointer" }}>Change Plan (Upgrade)</button>
                            <button style={{ padding: "10px 20px", background: "transparent", color: "#d32f2f", border: "1px solid #d32f2f", borderRadius: "6px", fontWeight: "bold", cursor: "pointer" }}>Cancel Subscription</button>
                        </div>
                    </div>

                    <div style={{ border: "1px solid #eaeaea", borderRadius: "12px", padding: "30px" }}>
                        <h3 style={{ marginTop: 0, marginBottom: "20px" }}>Usage Overview</h3>

                        <div style={{ marginBottom: "25px" }}>
                            <div style={{ display: "flex", justifyContent: "space-between", marginBottom: "8px" }}>
                                <span style={{ fontWeight: "bold" }}>Storage Used: {(storageUsage / 1000).toFixed(1)}GB</span>
                                <span style={{ color: "#666" }}>5GB Limit</span>
                            </div>
                            <div style={{ width: "100%", height: "8px", background: "#eaeaea", borderRadius: "4px" }}>
                                <div style={{ width: `${Math.min(100, (storageUsage / 5000) * 100)}%`, minWidth: "4px", height: "100%", background: "#0070f3", borderRadius: "4px" }}></div>
                            </div>
                        </div>

                        <div style={{ marginBottom: "25px" }}>
                            <div style={{ display: "flex", justifyContent: "space-between", marginBottom: "8px" }}>
                                <span style={{ fontWeight: "bold" }}>AI Actions</span>
                                <span style={{ color: "#666" }}>842 / 1,000</span>
                            </div>
                            <div style={{ width: "100%", height: "8px", background: "#eaeaea", borderRadius: "4px" }}>
                                <div style={{ width: "84.2%", height: "100%", background: "#0070f3", borderRadius: "4px" }}></div>
                            </div>
                        </div>

                        <button onClick={() => setViewingCostDetails(true)} style={{ width: "100%", padding: "12px", background: "#f8f9fa", border: "1px solid #eaeaea", borderRadius: "6px", fontWeight: "bold", cursor: "pointer" }}>
                            View Detailed Cost & AI Usage dashboard
                        </button>
                    </div>

                    <StorageDetailedBreakdown />
                    <AIUsageDetailedBreakdown />
                    <ProjectedSavingsPanel />
                    <UsageOptimizationHints />
                </div>

                <div>
                    <div style={{ border: "1px solid #eaeaea", borderRadius: "12px", padding: "25px", marginBottom: "30px" }}>
                        <h3 style={{ marginTop: 0, marginBottom: "20px" }}>Payment Method</h3>
                        <div style={{ display: "flex", alignItems: "center", gap: "15px", marginBottom: "20px", padding: "15px", background: "#f8f9fa", borderRadius: "8px" }}>
                            <div style={{ background: "#1a1f36", color: "white", padding: "4px 8px", borderRadius: "4px", fontWeight: "bold", fontSize: "0.8rem", letterSpacing: "1px" }}>VISA</div>
                            <div>
                                <div style={{ fontWeight: "bold" }}>•••• •••• •••• 4242</div>
                                <div style={{ fontSize: "0.85rem", color: "#666" }}>Expires 12/28</div>
                            </div>
                        </div>
                        <button style={{ width: "100%", padding: "10px", background: "transparent", border: "1px solid #ccc", borderRadius: "6px", fontWeight: "bold", cursor: "pointer" }}>Update Payment Details</button>
                    </div>

                    <InvoicesList />
                </div>
            </div>
        </div>
    );
}
