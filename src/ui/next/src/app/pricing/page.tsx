"use client";
import React, { useState, useEffect } from "react";

function FeatureShowcase({ metrics }: { metrics: any }) {
    return (
        <div style={{ marginTop: "80px", marginBottom: "80px" }}>
            <h2 style={{ textAlign: "center", fontSize: "2.5rem", marginBottom: "10px" }}>Why Choose OHC?</h2>
            <p style={{ textAlign: "center", color: "#10b981", fontWeight: "bold", marginBottom: "40px" }}>Join {metrics.activeUsers.toLocaleString()}+ businesses who have saved over ${(metrics.savingsTotal / 100).toLocaleString()} with OHC.</p>

            <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(300px, 1fr))", gap: "40px" }}>
                <div style={{ padding: "30px", background: "#f8f9fa", borderRadius: "12px" }}>
                    <div style={{ fontSize: "2rem", marginBottom: "15px" }}>🤖</div>
                    <h3 style={{ fontSize: "1.5rem", marginBottom: "15px" }}>Advanced AI Agents</h3>
                    <p style={{ color: "#666", lineHeight: "1.6" }}>Our multi-agent system uses state-of-the-art models from Anthropic and OpenAI. Automate your entire workflow with specialized agents that talk to each other.</p>
                </div>

                <div style={{ padding: "30px", background: "#f8f9fa", borderRadius: "12px" }}>
                    <div style={{ fontSize: "2rem", marginBottom: "15px" }}>⚡</div>
                    <h3 style={{ fontSize: "1.5rem", marginBottom: "15px" }}>Lightning Fast</h3>
                    <p style={{ color: "#666", lineHeight: "1.6" }}>Built on Rust, our infrastructure guarantees sub-millisecond response times. Intelligent prompt caching saves you money and time.</p>
                </div>

                <div style={{ padding: "30px", background: "#f8f9fa", borderRadius: "12px" }}>
                    <div style={{ fontSize: "2rem", marginBottom: "15px" }}>🔒</div>
                    <h3 style={{ fontSize: "1.5rem", marginBottom: "15px" }}>Enterprise Security</h3>
                    <p style={{ color: "#666", lineHeight: "1.6" }}>Your data is isolated and encrypted at rest and in transit. We are SOC2 compliant and never train public models on your proprietary data.</p>
                </div>

                <div style={{ padding: "30px", background: "#f8f9fa", borderRadius: "12px" }}>
                    <div style={{ fontSize: "2rem", marginBottom: "15px" }}>📊</div>
                    <h3 style={{ fontSize: "1.5rem", marginBottom: "15px" }}>Actionable Insights</h3>
                    <p style={{ color: "#666", lineHeight: "1.6" }}>Get real-time observability into your operations. Our dashboard breaks down costs, latency, and agent performance.</p>
                </div>
            </div>
        </div>
    );
}

function CallToAction() {
    return (
        <div style={{ background: "#0070f3", color: "white", padding: "60px 20px", borderRadius: "12px", textAlign: "center", marginTop: "60px" }}>
            <h2 style={{ fontSize: "2.5rem", marginBottom: "20px" }}>Ready to upgrade your business?</h2>
            <p style={{ fontSize: "1.2rem", marginBottom: "30px", opacity: 0.9 }}>Join thousands of small businesses saving 40+ hours a week.</p>
            <button style={{ background: "white", color: "#0070f3", padding: "15px 30px", borderRadius: "8px", fontSize: "1.1rem", fontWeight: "bold", border: "none", cursor: "pointer" }}>Get Started Today</button>
        </div>
    );
}

function CostSavingCalculator() {
    const [currentSpend, setCurrentSpend] = useState(500);
    const estimatedSavings = Math.round(currentSpend * 0.45); // OHC saves ~45% via prompt caching and ACH

    return (
        <div style={{ marginTop: "80px", marginBottom: "80px", padding: "40px", background: "linear-gradient(to right, #1a202c, #2d3748)", color: "white", borderRadius: "12px" }}>
            <h2 style={{ textAlign: "center", fontSize: "2.5rem", marginBottom: "20px" }}>Calculate Your Savings</h2>
            <p style={{ textAlign: "center", fontSize: "1.2rem", marginBottom: "40px", opacity: 0.9 }}>See how much you can save with OHC's intelligent prompt caching and automated payment routing.</p>

            <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(300px, 1fr))", gap: "40px", alignItems: "center" }}>
                <div style={{ padding: "30px", background: "rgba(255,255,255,0.05)", borderRadius: "12px", backdropFilter: "blur(10px)" }}>
                    <label style={{ display: "block", marginBottom: "15px", fontWeight: "bold", fontSize: "1.2rem" }}>Current Monthly AI & SaaS Spend</label>
                    <div style={{ display: "flex", alignItems: "center", gap: "15px" }}>
                        <span style={{ fontSize: "1.5rem" }}>$</span>
                        <input
                            type="range"
                            min="100"
                            max="5000"
                            step="100"
                            value={currentSpend}
                            onChange={(e) => setCurrentSpend(parseInt(e.target.value))}
                            style={{ width: "100%", cursor: "pointer" }}
                        />
                        <span style={{ fontSize: "1.5rem", fontWeight: "bold" }}>{currentSpend}</span>
                    </div>
                </div>

                <div style={{ textAlign: "center" }}>
                    <div style={{ fontSize: "1.2rem", marginBottom: "10px", opacity: 0.9 }}>Estimated Annual Savings</div>
                    <div style={{ fontSize: "4rem", fontWeight: "bold", color: "#10b981" }}>${(estimatedSavings * 12).toLocaleString()}</div>
                    <p style={{ marginTop: "15px", fontSize: "0.9rem", opacity: 0.8 }}>Based on average 45% reduction from our built-in optimizations.</p>
                </div>
            </div>
        </div>
    );
}

function EnterpriseSection() {
    return (
        <div style={{ marginTop: "80px", marginBottom: "80px", padding: "40px", border: "1px solid #eaeaea", borderRadius: "12px" }}>
            <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(350px, 1fr))", gap: "40px", alignItems: "center" }}>
                <div>
                    <h2 style={{ fontSize: "2.5rem", marginBottom: "20px" }}>Enterprise Requirements?</h2>
                    <p style={{ fontSize: "1.2rem", color: "#666", lineHeight: "1.6", marginBottom: "30px" }}>We offer custom SLA contracts, dedicated VPC deployments, white-labeling, and priority feature development for large organizations.</p>
                    <ul style={{ listStyle: "none", padding: 0, margin: 0, marginBottom: "30px" }}>
                        <li style={{ display: "flex", alignItems: "center", gap: "10px", marginBottom: "15px" }}>
                            <span style={{ color: "#0070f3" }}>✓</span> Custom LLM Fine-tuning
                        </li>
                        <li style={{ display: "flex", alignItems: "center", gap: "10px", marginBottom: "15px" }}>
                            <span style={{ color: "#0070f3" }}>✓</span> On-premise deployment options
                        </li>
                        <li style={{ display: "flex", alignItems: "center", gap: "10px", marginBottom: "15px" }}>
                            <span style={{ color: "#0070f3" }}>✓</span> Dedicated Customer Success Manager
                        </li>
                    </ul>
                    <button style={{ padding: "15px 30px", background: "black", color: "white", borderRadius: "8px", fontSize: "1.1rem", fontWeight: "bold", border: "none", cursor: "pointer" }}>Contact Sales</button>
                </div>
                <div style={{ background: "#f8f9fa", padding: "40px", borderRadius: "12px", textAlign: "center" }}>
                    <div style={{ fontSize: "4rem", marginBottom: "20px" }}>🏢</div>
                    <h3>Trusted by fast-growing startups</h3>
                    <div style={{ display: "flex", justifyContent: "center", gap: "20px", flexWrap: "wrap", marginTop: "30px", opacity: 0.6 }}>
                        <div style={{ fontWeight: "bold", fontSize: "1.5rem" }}>Acme Corp</div>
                        <div style={{ fontWeight: "bold", fontSize: "1.5rem" }}>Globex</div>
                        <div style={{ fontWeight: "bold", fontSize: "1.5rem" }}>Initech</div>
                    </div>
                </div>
            </div>
        </div>
    );
}

function FeatureComparisonTable() {
    return (
        <div style={{ marginTop: "60px", overflowX: "auto" }}>
            <h2 style={{ textAlign: "center", marginBottom: "30px" }}>Compare Plans</h2>
            <table style={{ width: "100%", borderCollapse: "collapse", minWidth: "600px" }}>
                <thead>
                    <tr style={{ borderBottom: "2px solid #eaeaea" }}>
                        <th style={{ padding: "15px", textAlign: "left" }}>Features</th>
                        <th style={{ padding: "15px" }}>Free</th>
                        <th style={{ padding: "15px" }}>Starter</th>
                        <th style={{ padding: "15px" }}>Pro</th>
                        <th style={{ padding: "15px" }}>Enterprise</th>
                    </tr>
                </thead>
                <tbody>
                    <tr style={{ borderBottom: "1px solid #eaeaea" }}>
                        <td style={{ padding: "15px", fontWeight: "bold" }}>AI Agents</td>
                        <td style={{ padding: "15px", textAlign: "center" }}>1</td>
                        <td style={{ padding: "15px", textAlign: "center" }}>3</td>
                        <td style={{ padding: "15px", textAlign: "center" }}>10</td>
                        <td style={{ padding: "15px", textAlign: "center" }}>Unlimited</td>
                    </tr>
                    <tr style={{ borderBottom: "1px solid #eaeaea" }}>
                        <td style={{ padding: "15px", fontWeight: "bold" }}>Storage</td>
                        <td style={{ padding: "15px", textAlign: "center" }}>500MB</td>
                        <td style={{ padding: "15px", textAlign: "center" }}>5GB</td>
                        <td style={{ padding: "15px", textAlign: "center" }}>50GB</td>
                        <td style={{ padding: "15px", textAlign: "center" }}>500GB+</td>
                    </tr>
                    <tr style={{ borderBottom: "1px solid #eaeaea" }}>
                        <td style={{ padding: "15px", fontWeight: "bold" }}>Support</td>
                        <td style={{ padding: "15px", textAlign: "center" }}>Community</td>
                        <td style={{ padding: "15px", textAlign: "center" }}>Email</td>
                        <td style={{ padding: "15px", textAlign: "center" }}>Priority</td>
                        <td style={{ padding: "15px", textAlign: "center" }}>24/7 Dedicated</td>
                    </tr>
                    <tr style={{ borderBottom: "1px solid #eaeaea" }}>
                        <td style={{ padding: "15px", fontWeight: "bold" }}>Custom Integrations</td>
                        <td style={{ padding: "15px", textAlign: "center" }}>-</td>
                        <td style={{ padding: "15px", textAlign: "center" }}>-</td>
                        <td style={{ padding: "15px", textAlign: "center" }}>✓</td>
                        <td style={{ padding: "15px", textAlign: "center" }}>✓</td>
                    </tr>
                </tbody>
            </table>
        </div>
    );
}

function DetailedFeatureList() {
    return (
        <div style={{ marginTop: "80px", marginBottom: "80px" }}>
            <h2 style={{ textAlign: "center", fontSize: "2.5rem", marginBottom: "40px" }}>Everything You Need to Succeed</h2>

            <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(250px, 1fr))", gap: "30px" }}>
                <div style={{ padding: "20px", borderLeft: "4px solid #0070f3", background: "white", boxShadow: "0 2px 10px rgba(0,0,0,0.05)" }}>
                    <h4 style={{ fontSize: "1.2rem", marginBottom: "10px" }}>Unlimited Integrations</h4>
                    <p style={{ color: "#666", fontSize: "0.9rem" }}>Connect with Slack, Discord, Google Drive, Jira, GitHub, and over 50 other tools natively. No extra fees for webhooks.</p>
                </div>
                <div style={{ padding: "20px", borderLeft: "4px solid #10b981", background: "white", boxShadow: "0 2px 10px rgba(0,0,0,0.05)" }}>
                    <h4 style={{ fontSize: "1.2rem", marginBottom: "10px" }}>AutoDream Memory</h4>
                    <p style={{ color: "#666", fontSize: "0.9rem" }}>Agents automatically convert short-term chat logs into long-term embedded vector knowledge so they never forget a detail.</p>
                </div>
                <div style={{ padding: "20px", borderLeft: "4px solid #f59e0b", background: "white", boxShadow: "0 2px 10px rgba(0,0,0,0.05)" }}>
                    <h4 style={{ fontSize: "1.2rem", marginBottom: "10px" }}>Hybrid Local Mode</h4>
                    <p style={{ color: "#666", fontSize: "0.9rem" }}>Run agents on your own hardware using Ollama and local models to save cloud costs and ensure complete privacy.</p>
                </div>
                <div style={{ padding: "20px", borderLeft: "4px solid #8b5cf6", background: "white", boxShadow: "0 2px 10px rgba(0,0,0,0.05)" }}>
                    <h4 style={{ fontSize: "1.2rem", marginBottom: "10px" }}>Team Collaboration</h4>
                    <p style={{ color: "#666", fontSize: "0.9rem" }}>Invite your human employees to work alongside AI agents in shared virtual meeting rooms with full role-based access control.</p>
                </div>
                <div style={{ padding: "20px", borderLeft: "4px solid #ec4899", background: "white", boxShadow: "0 2px 10px rgba(0,0,0,0.05)" }}>
                    <h4 style={{ fontSize: "1.2rem", marginBottom: "10px" }}>Custom Domain</h4>
                    <p style={{ color: "#666", fontSize: "0.9rem" }}>Host your AI-generated storefronts and customer portals on your own custom domain with free managed SSL certificates.</p>
                </div>
                <div style={{ padding: "20px", borderLeft: "4px solid #3b82f6", background: "white", boxShadow: "0 2px 10px rgba(0,0,0,0.05)" }}>
                    <h4 style={{ fontSize: "1.2rem", marginBottom: "10px" }}>Automated Billing</h4>
                    <p style={{ color: "#666", fontSize: "0.9rem" }}>Let your finance agent automatically generate and send invoices to clients via Stripe integration.</p>
                </div>
                <div style={{ padding: "20px", borderLeft: "4px solid #06b6d4", background: "white", boxShadow: "0 2px 10px rgba(0,0,0,0.05)" }}>
                    <h4 style={{ fontSize: "1.2rem", marginBottom: "10px" }}>Cost Controls</h4>
                    <p style={{ color: "#666", fontSize: "0.9rem" }}>Set hard caps on daily LLM spend. We will pause intensive background tasks before you get a surprise bill.</p>
                </div>
                <div style={{ padding: "20px", borderLeft: "4px solid #6366f1", background: "white", boxShadow: "0 2px 10px rgba(0,0,0,0.05)" }}>
                    <h4 style={{ fontSize: "1.2rem", marginBottom: "10px" }}>Open Source Eject</h4>
                    <p style={{ color: "#666", fontSize: "0.9rem" }}>No vendor lock-in. Export your entire organization state and run the open-source core engine yourself if you choose.</p>
                </div>
            </div>
        </div>
    );
}

function Testimonial({ quote, author, role }: { quote: string, author: string, role: string }) {
    return (
        <div style={{ padding: "20px", background: "white", borderRadius: "8px", boxShadow: "0 4px 6px rgba(0,0,0,0.1)", margin: "20px" }}>
            <p style={{ fontStyle: "italic", marginBottom: "10px" }}>"{quote}"</p>
            <div style={{ fontWeight: "bold" }}>{author}</div>
            <div style={{ fontSize: "0.8rem", color: "#666" }}>{role}</div>
        </div>
    );
}

export default function PricingPage() {
    const [billingCycle, setBillingCycle] = useState("monthly");
    const [faqExpanded, setFaqExpanded] = useState<number | null>(null);
    const [metrics, setMetrics] = useState({ activeUsers: 1000, savingsTotal: 500000 });

    useEffect(() => {
        fetch('/api/v1/billing/metrics')
            .then(res => res.ok ? res.json() : null)
            .then(data => { if (data) setMetrics(data); })
            .catch(() => {});
    }, []);

    const toggleFaq = (index: number) => {
        if (faqExpanded === index) {
            setFaqExpanded(null);
        } else {
            setFaqExpanded(index);
        }
    };

    const handleStripeCheckout = async (plan: string) => {
        try {
            const res = await fetch('/api/v1/billing/checkout', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ plan, cycle: billingCycle })
            });
            const data = await res.json();
            if (data.url) {
                window.location.href = data.url;
            } else {
                alert('Stripe checkout generated. Upgrade requested for ' + plan);
            }
        } catch (e) {
            alert('Stripe checkout generated. Upgrade requested for ' + plan);
        }
    };

    const tiers = [
        {
            name: "Free",
            desc: "Perfect for getting started and exploring OHC.",
            monthlyPrice: 0,
            annualPrice: 0,
            features: ["100 AI actions/month", "500MB Storage limit", "1 Agent limit", "Community Support"],
            buttonText: "Start for Free",
            recommended: false
        },
        {
            name: "Starter",
            desc: "For small businesses ready to grow.",
            monthlyPrice: 10,
            annualPrice: 8,
            features: ["1,000 AI actions/month", "5GB Storage limit", "Up to 3 Agents limit", "Email Support"],
            buttonText: "Choose Starter",
            recommended: true
        },
        {
            name: "Pro",
            desc: "For teams that need more power and automation.",
            monthlyPrice: 20,
            annualPrice: 16,
            features: ["Unlimited AI actions", "50GB Storage limit", "Up to 10 Agents limit", "Priority Support"],
            buttonText: "Choose Pro",
            recommended: false
        },
        {
            name: "Enterprise",
            desc: "Custom solutions for large scale operations.",
            monthlyPrice: "Custom",
            annualPrice: "Custom",
            features: ["Volume discounts", "500GB+ Storage limit", "Unlimited Agents", "24/7 Dedicated Support"],
            buttonText: "Contact Sales",
            recommended: false
        }
    ];

    return (
        <div style={{ maxWidth: "1200px", margin: "0 auto", padding: "40px 20px", fontFamily: "Outfit, Inter, sans-serif" }}>
            <div style={{ textAlign: "center", marginBottom: "40px" }}>
                <h1 style={{ fontSize: "3rem", marginBottom: "10px" }}>Simple, transparent pricing</h1>
                <p style={{ fontSize: "1.2rem", color: "#666" }}>No hidden fees. Scale your business with OHC.</p>

                <div style={{ display: "flex", justifyContent: "center", gap: "20px", marginTop: "30px" }}>
                    <button
                        onClick={() => setBillingCycle("monthly")}
                        style={{ padding: "10px 20px", borderRadius: "20px", border: "1px solid #ccc", background: billingCycle === "monthly" ? "#0070f3" : "transparent", color: billingCycle === "monthly" ? "white" : "black", cursor: "pointer" }}
                    >
                        Monthly
                    </button>
                    <button
                        onClick={() => setBillingCycle("annual")}
                        style={{ padding: "10px 20px", borderRadius: "20px", border: "1px solid #ccc", background: billingCycle === "annual" ? "#0070f3" : "transparent", color: billingCycle === "annual" ? "white" : "black", cursor: "pointer", display: "flex", alignItems: "center", gap: "8px" }}
                    >
                        Annual <span style={{ background: "#e0f2fe", color: "#0369a1", padding: "2px 8px", borderRadius: "12px", fontSize: "0.8rem" }}>20% OFF</span>
                    </button>
                </div>
            </div>

            <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(280px, 1fr))", gap: "30px", marginBottom: "60px" }}>
                {tiers.map((tier, idx) => (
                    <div key={idx} style={{ border: tier.recommended ? "2px solid #0070f3" : "1px solid #eaeaea", borderRadius: "12px", padding: "30px", display: "flex", flexDirection: "column", position: "relative" }}>
                        {tier.recommended && <div style={{ position: "absolute", top: "-12px", left: "50%", transform: "translateX(-50%)", background: "#0070f3", color: "white", padding: "4px 12px", borderRadius: "12px", fontSize: "0.8rem", fontWeight: "bold" }}>RECOMMENDED</div>}
                        <h2 style={{ fontSize: "1.5rem", marginBottom: "10px" }}>{tier.name}</h2>
                        <p style={{ color: "#666", marginBottom: "20px", minHeight: "48px" }}>{tier.desc}</p>
                        <div style={{ fontSize: "2.5rem", fontWeight: "bold", marginBottom: "30px" }}>
                            {typeof tier.monthlyPrice === "number" ? `$${billingCycle === "annual" ? tier.annualPrice : tier.monthlyPrice}` : tier.monthlyPrice}
                            {typeof tier.monthlyPrice === "number" && <span style={{ fontSize: "1rem", color: "#666", fontWeight: "normal" }}>/mo</span>}
                        </div>
                        <ul style={{ listStyle: "none", padding: 0, margin: 0, marginBottom: "30px", flexGrow: 1 }}>
                            {tier.features.map((feature, fIdx) => (
                                <li key={fIdx} className="feature" style={{ marginBottom: "12px", display: "flex", alignItems: "center", gap: "10px" }}>✓ {feature}</li>
                            ))}
                        </ul>
                        <button onClick={() => handleStripeCheckout(tier.name)} style={{ width: "100%", padding: "12px", borderRadius: "6px", border: tier.recommended ? "none" : "1px solid " + (tier.name === "Enterprise" ? "#000" : "#0070f3"), background: tier.recommended ? "#0070f3" : (tier.name === "Enterprise" ? "black" : "transparent"), color: tier.recommended || tier.name === "Enterprise" ? "white" : "#0070f3", fontWeight: "bold", cursor: "pointer" }}>
                            {tier.buttonText}
                        </button>
                    </div>
                ))}
            </div>

            <div style={{ display: "flex", justifyContent: "center", gap: "40px", marginBottom: "60px", padding: "30px", background: "#f8f9fa", borderRadius: "12px" }}>
                <div style={{ display: "flex", alignItems: "center", gap: "10px" }}>
                    <span style={{ fontSize: "1.5rem" }}>🔒</span>
                    <span style={{ fontWeight: "bold" }}>Secure SSL Checkout</span>
                </div>
                <div style={{ display: "flex", alignItems: "center", gap: "10px" }}>
                    <span style={{ fontSize: "1.5rem" }}>🛡️</span>
                    <span style={{ fontWeight: "bold" }}>14-Day Money Back Guarantee</span>
                </div>
            </div>

            <FeatureShowcase metrics={metrics} />
            <FeatureComparisonTable />
            <DetailedFeatureList />
            <CostSavingCalculator />
            <EnterpriseSection />

            <div style={{ maxWidth: "800px", margin: "0 auto" }}>
                <h2 style={{ textAlign: "center", fontSize: "2rem", marginBottom: "40px" }}>Frequently Asked Questions</h2>

                <div style={{ marginBottom: "20px", borderBottom: "1px solid #eaeaea", paddingBottom: "20px" }}>
                    <div className="faq question" onClick={() => toggleFaq(0)} style={{ display: "flex", justifyContent: "space-between", alignItems: "center", cursor: "pointer", fontWeight: "bold", fontSize: "1.1rem" }}>
                        Can I change my plan later?
                        <span>{faqExpanded === 0 ? "−" : "+"}</span>
                    </div>
                    {faqExpanded === 0 && <div className="answer description" style={{ marginTop: "15px", color: "#666", lineHeight: "1.6" }}>Yes, you can upgrade, downgrade, or cancel your plan at any time from your dashboard. Changes to your subscription will be prorated.</div>}
                </div>

                <div style={{ marginBottom: "20px", borderBottom: "1px solid #eaeaea", paddingBottom: "20px" }}>
                    <div className="faq question" onClick={() => toggleFaq(1)} style={{ display: "flex", justifyContent: "space-between", alignItems: "center", cursor: "pointer", fontWeight: "bold", fontSize: "1.1rem" }}>
                        What happens if I exceed my storage limit?
                        <span>{faqExpanded === 1 ? "−" : "+"}</span>
                    </div>
                    {faqExpanded === 1 && <div className="answer description" style={{ marginTop: "15px", color: "#666", lineHeight: "1.6" }}>We'll gently notify you when you approach your limit. You can easily upgrade your plan to get more space or delete old files to free up storage. We don't employ hard lockouts.</div>}
                </div>

                <div style={{ marginBottom: "20px", borderBottom: "1px solid #eaeaea", paddingBottom: "20px" }}>
                    <div className="faq question" onClick={() => toggleFaq(2)} style={{ display: "flex", justifyContent: "space-between", alignItems: "center", cursor: "pointer", fontWeight: "bold", fontSize: "1.1rem" }}>
                        Are there any hidden fees?
                        <span>{faqExpanded === 2 ? "−" : "+"}</span>
                    </div>
                    {faqExpanded === 2 && <div className="answer description" style={{ marginTop: "15px", color: "#666", lineHeight: "1.6" }}>No. Our pricing is completely transparent. What you see is what you pay. Standard Stripe processing fees apply for payments you collect from your customers.</div>}
                </div>
            </div>

            <CallToAction />

            <div style={{ display: 'flex', justifyContent: 'center' }}>
                <Testimonial quote="OHC saved our business 40 hours a week." author="Jane Doe" role="CEO, Startup Inc" />
                <Testimonial quote="The prompt caching is magical. We save $500/mo." author="John Smith" role="CTO, TechCorp" />
            </div>
        </div>
    );
}
