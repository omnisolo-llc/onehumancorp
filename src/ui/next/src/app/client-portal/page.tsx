"use client";

import { useState, useEffect } from "react";
import Link from "next/link";

interface CourseLesson {
  id: string;
  title: string;
  duration: string;
  completed: boolean;
}

interface Course {
  id: string;
  title: string;
  instructor: string;
  progress: number;
  lessons: CourseLesson[];
}

interface PodcastEpisode {
  id: string;
  title: string;
  duration: string;
  published: string;
  description: string;
}

interface Quote {
  id: string;
  service: string;
  amount: number;
  deposit: number;
  status: "DRAFT" | "APPROVED" | "PAID";
  details: string;
}

interface Invoice {
  id: string;
  description: string;
  amount: number;
  dueDate: string;
  status: "UNPAID" | "PROCESSING" | "PAID";
}

interface TaskItem {
  id: string;
  name: string;
  status: "Completed" | "In Progress" | "Pending";
  updatedAt: string;
}

interface ChatMessage {
  id: string;
  sender: "user" | "agent";
  text: string;
  timestamp: string;
}

export default function ClientPortalPage() {
  const [activeTab, setActiveTab] = useState<
    "overview" | "proposals" | "payments" | "digital" | "projects" | "support"
  >("overview");

  // Portal State
  const [clientName, setClientName] = useState("Acme Corporation");
  const [clientEmail, setClientEmail] = useState("contact@acme.com");
  
  // Proposals State
  const [quotes, setQuotes] = useState<Quote[]>([
    {
      id: "QT-9021",
      service: "Automated Supply Chain & MRP Integration",
      amount: 4500,
      deposit: 1500,
      status: "DRAFT",
      details: "Full visual workflow setup connecting supplier inventory, purchase order generation, and shipping APIs.",
    },
    {
      id: "QT-8812",
      service: "Omnichannel Lead Score & CRM Customization",
      amount: 2800,
      deposit: 933,
      status: "APPROVED",
      details: "Configuration of Salesforce routing and AI agent automatic email drafting based on incoming lead scores.",
    },
  ]);
  const [selectedQuote, setSelectedQuote] = useState<Quote | null>(quotes[0]);
  const [signatureText, setSignatureText] = useState("");
  const [hasConsented, setHasConsented] = useState(false);
  const [proposalSuccess, setProposalSuccess] = useState("");

  // Invoices & Payments State
  const [invoices, setInvoices] = useState<Invoice[]>([
    {
      id: "INV-1024",
      description: "Initial Deposit - Supply Chain Setup",
      amount: 1500,
      dueDate: "2026-08-15",
      status: "UNPAID",
    },
    {
      id: "INV-0982",
      description: "CRM Milestone 1 Completion",
      amount: 933,
      dueDate: "2026-07-20",
      status: "PAID",
    },
  ]);
  const [selectedInvoice, setSelectedInvoice] = useState<Invoice | null>(invoices[0]);
  const [cardNumber, setCardNumber] = useState("");
  const [cardExpiry, setCardExpiry] = useState("");
  const [cardCvc, setCardCvc] = useState("");
  const [paymentLoading, setPaymentLoading] = useState(false);
  const [paymentSuccess, setPaymentSuccess] = useState("");
  const [paymentGateway, setPaymentGateway] = useState<"stripe" | "square">("stripe");

  // Digital Products & Subscriptions State
  const [courses, setCourses] = useState<Course[]>([
    {
      id: "course-1",
      title: "Swarm Intelligence & Business Automations Masterclass",
      instructor: "Dr. K. Sona, AI Director",
      progress: 60,
      lessons: [
        { id: "l1", title: "Introduction to Multi-Agent Swarms", duration: "12:14", completed: true },
        { id: "l2", title: "Designing Custom Visual Workflows", duration: "18:45", completed: true },
        { id: "l3", title: "Integrating Financial and POS APIs", duration: "25:10", completed: true },
        { id: "l4", title: "Optimizing Lead Gen & Auto-Scoring", duration: "14:22", completed: false },
        { id: "l5", title: "Advanced Local-to-Cloud DB Syncing", duration: "22:05", completed: false },
      ],
    },
  ]);
  const [currentCourse, setCurrentCourse] = useState<Course>(courses[0]);
  const [activePodcastEpisode, setActivePodcastEpisode] = useState<PodcastEpisode | null>(null);
  const [isPlayingPodcast, setIsPlayingPodcast] = useState(false);
  const [podcastProgress, setPodcastPodcastProgress] = useState(35);

  const podcastEpisodes: PodcastEpisode[] = [
    {
      id: "pod-1",
      title: "Episode 14: Scaling Multi-Agent Systems Offline-First",
      duration: "42:18",
      published: "2026-07-18",
      description: "A deep-dive discussion on PowerSync and local SQLite memory architecture for zero-latency AI execution.",
    },
    {
      id: "pod-2",
      title: "Episode 13: Zero-Trust Security & Cryptographic Verifications",
      duration: "38:40",
      published: "2026-07-04",
      description: "Exploring SPIFFE/SPIRE agent identities and secure sandboxing mechanisms to prevent code injection.",
    },
  ];

  // Project Tracker State
  const [tasks, setTasks] = useState<TaskItem[]>([
    { id: "TSK-01", name: "Configure PostgreSQL Schema & Schema Migrations", status: "Completed", updatedAt: "2026-07-20 09:12" },
    { id: "TSK-02", name: "Provision Multi-Agent Swarm Actors", status: "Completed", updatedAt: "2026-07-20 14:45" },
    { id: "TSK-03", name: "Deploy Sandbox Environment & Secure Integrations", status: "In Progress", updatedAt: "2026-07-21 10:30" },
    { id: "TSK-04", name: "Integrate QuickBooks Online & Stripe POS Gateways", status: "Pending", updatedAt: "Never" },
  ]);

  // Support / Live Chat State
  const [chatMessages, setChatMessages] = useState<ChatMessage[]>([
    { id: "m1", sender: "agent", text: "Hello! Welcome to your secure Client Portal. I am your automated Customer Success Agent. How can I help you with your proposals, invoices, or subscriptions today?", timestamp: "10:30 AM" },
  ]);
  const [currentMessageText, setCurrentMessageText] = useState("");
  const [ticketDescription, setTicketDescription] = useState("");
  const [ticketCategory, setTicketCategory] = useState("billing");
  const [ticketPriority, setTicketPriority] = useState("medium");
  const [ticketSuccess, setTicketSuccess] = useState("");

  const handleSendChatMessage = (e: React.FormEvent) => {
    e.preventDefault();
    if (!currentMessageText.trim()) return;

    const userMsg: ChatMessage = {
      id: `m-u-${Date.now()}`,
      sender: "user",
      text: currentMessageText,
      timestamp: new Date().toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" }),
    };

    setChatMessages((prev) => [...prev, userMsg]);
    const inputMsg = currentMessageText;
    setCurrentMessageText("");

    // Simulate Agent response based on input
    setTimeout(() => {
      let reply = "I've logged your request in our central message feed. An expert agent from our team will address it shortly!";
      const lower = inputMsg.toLowerCase();
      if (lower.includes("invoice") || lower.includes("pay") || lower.includes("billing")) {
        reply = "I see you're asking about billing. You can view your current invoices in the 'Invoices & Receipts' tab and make secure simulated payments right away!";
      } else if (lower.includes("proposal") || lower.includes("quote") || lower.includes("sign")) {
        reply = "You can view, review, sign, and authorize your active estimates and proposals under the 'Proposals & Quotes' tab.";
      } else if (lower.includes("course") || lower.includes("podcast") || lower.includes("download")) {
        reply = "Digital products, video courses, coaching guidelines, and podcast streams are available in the 'Digital Products' tab of this portal.";
      } else if (lower.includes("project") || lower.includes("task") || lower.includes("progress")) {
        reply = "You can track real-time task completion, milestones, and visual workflows in the 'Project Tracker' tab.";
      }

      setChatMessages((prev) => [
        ...prev,
        {
          id: `m-a-${Date.now()}`,
          sender: "agent",
          text: reply,
          timestamp: new Date().toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" }),
        },
      ]);
    }, 1000);
  };

  const handleSignProposal = (e: React.FormEvent) => {
    e.preventDefault();
    if (!selectedQuote) return;
    if (!signatureText.trim() || !hasConsented) {
      setProposalSuccess("Please provide signature text and check the consent box.");
      return;
    }

    setQuotes((prev) =>
      prev.map((q) => (q.id === selectedQuote.id ? { ...q, status: "APPROVED" } : q))
    );
    setSelectedQuote((prev) => (prev ? { ...prev, status: "APPROVED" } : null));
    setProposalSuccess(`Successfully approved proposal ${selectedQuote.id}! Electronic signature registered: ${signatureText}`);
  };

  const handleProcessPayment = (e: React.FormEvent) => {
    e.preventDefault();
    if (!selectedInvoice) return;
    if (cardNumber.length < 15 || !cardExpiry || cardCvc.length < 3) {
      setPaymentSuccess("Please fill in valid card details.");
      return;
    }

    setPaymentLoading(true);
    setPaymentSuccess("");

    setTimeout(() => {
      setPaymentLoading(false);
      setInvoices((prev) =>
        prev.map((inv) => (inv.id === selectedInvoice.id ? { ...inv, status: "PAID" } : inv))
      );
      setSelectedInvoice((prev) => (prev ? { ...prev, status: "PAID" } : null));
      setPaymentSuccess(`Payment of $${selectedInvoice.amount}.00 received successfully! Receipt INV-REC-${selectedInvoice.id} generated.`);
      setCardNumber("");
      setCardExpiry("");
      setCardCvc("");
    }, 1500);
  };

  const handleSubmitTicket = (e: React.FormEvent) => {
    e.preventDefault();
    if (!ticketDescription.trim()) return;

    setTicketSuccess(`Support ticket created successfully! Priority: ${ticketPriority.toUpperCase()}. Our automated agent is routing it to the engineering triage queue.`);
    setTicketDescription("");
  };

  const toggleLesson = (lessonId: string) => {
    const updatedLessons = currentCourse.lessons.map((lesson) =>
      lesson.id === lessonId ? { ...lesson, completed: !lesson.completed } : lesson
    );
    const completedCount = updatedLessons.filter((l) => l.completed).length;
    const progress = Math.round((completedCount / updatedLessons.length) * 100);

    const updatedCourse = {
      ...currentCourse,
      lessons: updatedLessons,
      progress,
    };

    setCourses((prev) => prev.map((c) => (c.id === currentCourse.id ? updatedCourse : c)));
    setCurrentCourse(updatedCourse);
  };

  return (
    <div className="min-h-screen font-inter bg-slate-50 text-slate-800">
      {/* Client Header */}
      <header className="sticky top-0 z-40 bg-white/70 backdrop-blur-md border-b border-slate-200/80 px-6 py-4 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
        <div className="flex items-center gap-3">
          <Link href="/dashboard" className="text-blue-600 hover:text-blue-800 font-medium flex items-center gap-1 transition-colors">
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" />
            </svg>
            Back
          </Link>
          <div className="h-4 w-px bg-slate-300" />
          <div>
            <h1 className="text-xl font-bold font-outfit text-slate-900 tracking-tight">Client Hub Portal</h1>
            <p className="text-xs text-slate-500">Secure digital interface for {clientName}</p>
          </div>
        </div>
        <div className="flex items-center gap-4 bg-slate-100 p-2 rounded-lg border border-slate-200">
          <div className="text-right">
            <p className="text-xs font-semibold text-slate-700">{clientName}</p>
            <p className="text-[10px] text-slate-500">{clientEmail}</p>
          </div>
          <div className="w-8 h-8 rounded-full bg-blue-600 flex items-center justify-center text-white font-bold text-xs">
            AC
          </div>
        </div>
      </header>

      {/* Main Body Grid */}
      <div className="max-w-6xl mx-auto px-4 py-6 grid grid-cols-1 md:grid-cols-4 gap-6">
        {/* Navigation Sidebar */}
        <aside className="md:col-span-1 space-y-2">
          <button
            onClick={() => setActiveTab("overview")}
            className={`w-full text-left px-4 py-3 rounded-xl transition-all font-medium text-sm flex items-center gap-3 ${
              activeTab === "overview"
                ? "bg-blue-600 text-white shadow-sm"
                : "bg-white hover:bg-slate-100 text-slate-600 border border-slate-200"
            }`}
          >
            <span>🏠</span> Overview Hub
          </button>
          <button
            onClick={() => setActiveTab("proposals")}
            className={`w-full text-left px-4 py-3 rounded-xl transition-all font-medium text-sm flex items-center gap-3 ${
              activeTab === "proposals"
                ? "bg-blue-600 text-white shadow-sm"
                : "bg-white hover:bg-slate-100 text-slate-600 border border-slate-200"
            }`}
          >
            <span>📜</span> Proposals & Quotes
          </button>
          <button
            onClick={() => setActiveTab("payments")}
            className={`w-full text-left px-4 py-3 rounded-xl transition-all font-medium text-sm flex items-center gap-3 ${
              activeTab === "payments"
                ? "bg-blue-600 text-white shadow-sm"
                : "bg-white hover:bg-slate-100 text-slate-600 border border-slate-200"
            }`}
          >
            <span>💳</span> Invoices & Billing
          </button>
          <button
            onClick={() => setActiveTab("digital")}
            className={`w-full text-left px-4 py-3 rounded-xl transition-all font-medium text-sm flex items-center gap-3 ${
              activeTab === "digital"
                ? "bg-blue-600 text-white shadow-sm"
                : "bg-white hover:bg-slate-100 text-slate-600 border border-slate-200"
            }`}
          >
            <span>🎓</span> Digital Products
          </button>
          <button
            onClick={() => setActiveTab("projects")}
            className={`w-full text-left px-4 py-3 rounded-xl transition-all font-medium text-sm flex items-center gap-3 ${
              activeTab === "projects"
                ? "bg-blue-600 text-white shadow-sm"
                : "bg-white hover:bg-slate-100 text-slate-600 border border-slate-200"
            }`}
          >
            <span>⚙️</span> Project Tracker
          </button>
          <button
            onClick={() => setActiveTab("support")}
            className={`w-full text-left px-4 py-3 rounded-xl transition-all font-medium text-sm flex items-center gap-3 ${
              activeTab === "support"
                ? "bg-blue-600 text-white shadow-sm"
                : "bg-white hover:bg-slate-100 text-slate-600 border border-slate-200"
            }`}
          >
            <span>💬</span> Help & Live Chat
          </button>
        </aside>

        {/* Dynamic Display Area */}
        <section className="md:col-span-3 space-y-6">
          {/* TAB 1: OVERVIEW */}
          {activeTab === "overview" && (
            <div className="space-y-6">
              <div className="bg-gradient-to-r from-blue-700 to-indigo-800 text-white p-6 rounded-2xl shadow-md border border-indigo-900">
                <h2 className="text-2xl font-bold font-outfit mb-2">Welcome Back, {clientName}!</h2>
                <p className="text-blue-100 text-sm max-w-xl">
                  Through this client portal, you can securely sign outstanding contracts, process milestone payments, streams courses/podcasts, and contact support in real time.
                </p>
                <div className="mt-4 flex gap-3">
                  <span className="bg-blue-500/50 backdrop-blur-sm px-3 py-1.5 rounded-lg text-xs font-semibold">
                    🔐 Encrypted Sandboxing
                  </span>
                  <span className="bg-green-500/50 backdrop-blur-sm px-3 py-1.5 rounded-lg text-xs font-semibold">
                    ✓ Zero-Trust ID Verification
                  </span>
                </div>
              </div>

              {/* Grid of Key Actions */}
              <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                {/* Pending Tasks */}
                <div className="bg-white border border-slate-200 p-5 rounded-xl shadow-sm">
                  <h3 className="font-bold text-slate-800 font-outfit mb-3">Today's Priorities</h3>
                  <div className="space-y-3">
                    {quotes.some((q) => q.status === "DRAFT") && (
                      <div className="p-3 bg-amber-50 border border-amber-200 rounded-lg flex items-center justify-between">
                        <div>
                          <p className="text-xs font-bold text-amber-900">1 Outstanding Proposal</p>
                          <p className="text-[10px] text-amber-700">Needs signature and authorization</p>
                        </div>
                        <button
                          onClick={() => setActiveTab("proposals")}
                          className="bg-amber-600 hover:bg-amber-700 text-white text-[10px] font-bold px-3 py-1 rounded"
                        >
                          Review
                        </button>
                      </div>
                    )}
                    {invoices.some((inv) => inv.status === "UNPAID") && (
                      <div className="p-3 bg-rose-50 border border-rose-200 rounded-lg flex items-center justify-between">
                        <div>
                          <p className="text-xs font-bold text-rose-900">1 Pending Invoice Payment</p>
                          <p className="text-[10px] text-rose-700">Initial Deposit is due</p>
                        </div>
                        <button
                          onClick={() => setActiveTab("payments")}
                          className="bg-rose-600 hover:bg-rose-700 text-white text-[10px] font-bold px-3 py-1 rounded"
                        >
                          Pay Now
                        </button>
                      </div>
                    )}
                  </div>
                </div>

                {/* Subscriptions Card */}
                <div className="bg-white border border-slate-200 p-5 rounded-xl shadow-sm flex flex-col justify-between">
                  <div>
                    <h3 className="font-bold text-slate-800 font-outfit mb-2">My Subscriptions & Perks</h3>
                    <p className="text-xs text-slate-600">Active Membership: <span className="font-bold text-blue-600">Elite Agent Automation Hub</span></p>
                    <p className="text-[10px] text-slate-500 mt-1">Status: Active (Auto-renews next month)</p>
                  </div>
                  <div className="mt-4 pt-3 border-t border-slate-100 flex justify-between items-center text-xs">
                    <span>Course Access: <span className="font-bold text-green-600">Granted</span></span>
                    <span>Podcast Access: <span className="font-bold text-green-600">Granted</span></span>
                  </div>
                </div>
              </div>
            </div>
          )}

          {/* TAB 2: PROPOSALS & QUOTES */}
          {activeTab === "proposals" && (
            <div className="bg-white border border-slate-200 p-6 rounded-2xl shadow-sm space-y-6">
              <div className="border-b border-slate-100 pb-4">
                <h2 className="text-lg font-bold font-outfit text-slate-900">Active Proposals & Cost Estimations</h2>
                <p className="text-xs text-slate-500">Review outstanding plans, pricing structures, and execute biometric-friendly signatures.</p>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                {/* List of Quotes */}
                <div className="md:col-span-1 space-y-3">
                  {quotes.map((q) => (
                    <button
                      key={q.id}
                      onClick={() => {
                        setSelectedQuote(q);
                        setProposalSuccess("");
                      }}
                      className={`w-full text-left p-4 rounded-xl border transition-all ${
                        selectedQuote?.id === q.id
                          ? "border-blue-500 bg-blue-50/50 shadow-sm"
                          : "border-slate-200 hover:bg-slate-50 bg-white"
                      }`}
                    >
                      <div className="flex justify-between items-start mb-1">
                        <span className="text-[10px] font-bold text-slate-400">{q.id}</span>
                        <span
                          className={`text-[9px] font-extrabold px-2 py-0.5 rounded-full ${
                            q.status === "APPROVED"
                              ? "bg-green-100 text-green-700"
                              : "bg-amber-100 text-amber-700"
                          }`}
                        >
                          {q.status}
                        </span>
                      </div>
                      <p className="text-xs font-bold text-slate-800 line-clamp-1">{q.service}</p>
                      <p className="text-xs font-bold text-blue-600 mt-2">${q.amount}.00</p>
                    </button>
                  ))}
                </div>

                {/* Selected Quote Detail */}
                <div className="md:col-span-2 bg-slate-50 border border-slate-200 p-5 rounded-xl space-y-4">
                  {selectedQuote ? (
                    <>
                      <div className="flex justify-between items-start">
                        <div>
                          <h3 className="font-bold text-sm text-slate-900">{selectedQuote.service}</h3>
                          <p className="text-xs text-blue-600 font-bold mt-1">Total: ${selectedQuote.amount}.00 | Deposit: ${selectedQuote.deposit}.00</p>
                        </div>
                        <span className="text-[10px] font-bold bg-slate-200 text-slate-600 px-2 py-1 rounded">
                          {selectedQuote.id}
                        </span>
                      </div>
                      <p className="text-xs text-slate-600 leading-relaxed bg-white p-3 rounded-lg border border-slate-200">
                        {selectedQuote.details}
                      </p>

                      {selectedQuote.status === "DRAFT" ? (
                        <form onSubmit={handleSignProposal} className="space-y-4 bg-white p-4 rounded-lg border border-slate-200">
                          <h4 className="text-xs font-bold text-slate-800">Review & Approve Proposal</h4>
                          <div className="space-y-2">
                            <label className="block text-xs font-medium text-slate-700">Type Name to Sign Electronically</label>
                            <input
                              type="text"
                              value={signatureText}
                              onChange={(e) => setSignatureText(e.target.value)}
                              placeholder="Full Legal Name"
                              className="w-full text-xs p-2.5 border border-slate-300 rounded focus:ring-1 focus:ring-blue-500 outline-none"
                            />
                          </div>
                          <div className="flex items-start gap-2">
                            <input
                              type="checkbox"
                              id="consent"
                              checked={hasConsented}
                              onChange={(e) => setHasConsented(e.target.checked)}
                              className="mt-0.5"
                            />
                            <label htmlFor="consent" className="text-[10px] text-slate-500 leading-normal">
                              I consent to sign digitally and agree to authorize One Human Corp and its AI Swarms to initiate the scheduled work upon verification.
                            </label>
                          </div>
                          <button
                            type="submit"
                            className="w-full bg-blue-600 hover:bg-blue-700 text-white text-xs font-bold py-2 px-4 rounded transition-all"
                          >
                            Sign & Approve Proposal
                          </button>
                        </form>
                      ) : (
                        <div className="bg-green-50 border border-green-200 p-4 rounded-lg text-center">
                          <span className="text-xl">🛡️</span>
                          <p className="text-xs font-bold text-green-900 mt-1">Proposal Signed and Authorized</p>
                          <p className="text-[10px] text-green-700 mt-1">This project has been activated. Swarm agents are working on related active workflows.</p>
                        </div>
                      )}

                      {proposalSuccess && (
                        <div className={`text-xs p-3 rounded-lg border ${
                          proposalSuccess.includes("Successfully")
                            ? "bg-green-50 text-green-800 border-green-200"
                            : "bg-red-50 text-red-800 border-red-200"
                        }`}>
                          {proposalSuccess}
                        </div>
                      )}
                    </>
                  ) : (
                    <p className="text-xs text-slate-500 text-center py-10">Select a proposal from the left sidebar to begin review.</p>
                  )}
                </div>
              </div>
            </div>
          )}

          {/* TAB 3: INVOICES & PAYMENTS */}
          {activeTab === "payments" && (
            <div className="bg-white border border-slate-200 p-6 rounded-2xl shadow-sm space-y-6">
              <div className="border-b border-slate-100 pb-4">
                <h2 className="text-lg font-bold font-outfit text-slate-900">Invoices & Milestone Billing</h2>
                <p className="text-xs text-slate-500">Track paid and outstanding milestone payments, and make instant secure deposits.</p>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                {/* Left: Invoices List */}
                <div className="md:col-span-1 space-y-3">
                  {invoices.map((inv) => (
                    <button
                      key={inv.id}
                      onClick={() => {
                        setSelectedInvoice(inv);
                        setPaymentSuccess("");
                      }}
                      className={`w-full text-left p-4 rounded-xl border transition-all ${
                        selectedInvoice?.id === inv.id
                          ? "border-blue-500 bg-blue-50/50 shadow-sm"
                          : "border-slate-200 hover:bg-slate-50 bg-white"
                      }`}
                    >
                      <div className="flex justify-between items-start mb-1">
                        <span className="text-[10px] font-bold text-slate-400">{inv.id}</span>
                        <span
                          className={`text-[9px] font-extrabold px-2 py-0.5 rounded-full ${
                            inv.status === "PAID"
                              ? "bg-green-100 text-green-700"
                              : "bg-rose-100 text-rose-700"
                          }`}
                        >
                          {inv.status}
                        </span>
                      </div>
                      <p className="text-xs font-bold text-slate-800 line-clamp-1">{inv.description}</p>
                      <p className="text-xs font-bold text-slate-500 mt-2">Due by {inv.dueDate}</p>
                      <p className="text-xs font-bold text-blue-600 mt-1">${inv.amount}.00</p>
                    </button>
                  ))}
                </div>

                {/* Right: Payment Terminal */}
                <div className="md:col-span-2 bg-slate-50 border border-slate-200 p-5 rounded-xl space-y-4">
                  {selectedInvoice ? (
                    <>
                      <div className="flex justify-between items-start">
                        <div>
                          <h3 className="font-bold text-sm text-slate-900">{selectedInvoice.description}</h3>
                          <p className="text-xs text-slate-500">Due Date: {selectedInvoice.dueDate}</p>
                        </div>
                        <span className="text-lg font-bold text-blue-600">${selectedInvoice.amount}.00</span>
                      </div>

                      {selectedInvoice.status === "UNPAID" ? (
                        <form onSubmit={handleProcessPayment} className="space-y-4 bg-white p-4 rounded-lg border border-slate-200">
                          <h4 className="text-xs font-bold text-slate-800 flex items-center gap-1.5">
                            💳 Credit Card Payment
                          </h4>
                          <div className="space-y-3">
                            <div>
                              <label className="block text-[10px] font-bold text-slate-500 uppercase mb-1">Card Number</label>
                              <input
                                type="text"
                                maxLength={16}
                                value={cardNumber}
                                onChange={(e) => setCardNumber(e.target.value.replace(/\D/g, ""))}
                                placeholder="4000 1234 5678 9010"
                                className="w-full text-xs p-2.5 border border-slate-300 rounded focus:ring-1 focus:ring-blue-500 outline-none"
                              />
                            </div>
                            <div className="grid grid-cols-2 gap-3">
                              <div>
                                <label className="block text-[10px] font-bold text-slate-500 uppercase mb-1">Expiry Date</label>
                                <input
                                  type="text"
                                  maxLength={5}
                                  value={cardExpiry}
                                  onChange={(e) => setCardExpiry(e.target.value)}
                                  placeholder="MM/YY"
                                  className="w-full text-xs p-2.5 border border-slate-300 rounded focus:ring-1 focus:ring-blue-500 outline-none"
                                />
                              </div>
                              <div>
                                <label className="block text-[10px] font-bold text-slate-500 uppercase mb-1">CVC / CVV</label>
                                <input
                                  type="text"
                                  maxLength={4}
                                  value={cardCvc}
                                  onChange={(e) => setCardCvc(e.target.value.replace(/\D/g, ""))}
                                  placeholder="123"
                                  className="w-full text-xs p-2.5 border border-slate-300 rounded focus:ring-1 focus:ring-blue-500 outline-none"
                                />
                              </div>
                            </div>
                          </div>
                          <button
                            type="submit"
                            disabled={paymentLoading}
                            className={`w-full bg-slate-900 hover:bg-black text-white text-xs font-bold py-2 px-4 rounded transition-all flex items-center justify-center gap-2 ${
                              paymentLoading ? "opacity-75 cursor-not-allowed" : ""
                            }`}
                          >
                            {paymentLoading ? (
                              <>
                                <span className="animate-spin text-sm">⌛</span> Processing Payment...
                              </>
                            ) : (
                              `Authorize Payment ($${selectedInvoice.amount}.00)`
                            )}
                          </button>
                        </form>
                      ) : (
                        <div className="bg-green-50 border border-green-200 p-4 rounded-lg flex flex-col items-center text-center">
                          <span className="text-xl">✅</span>
                          <p className="text-xs font-bold text-green-900 mt-1">Invoice Fully Paid</p>
                          <p className="text-[10px] text-green-700 mt-1">Thank you! Your payment of ${selectedInvoice.amount}.00 has been verified.</p>
                          <button
                            onClick={() => alert(`Downloading Receipt PDF for ${selectedInvoice.id}...`)}
                            className="mt-3 bg-white text-slate-700 border border-slate-300 hover:bg-slate-50 text-[10px] font-bold px-4 py-1.5 rounded transition-all"
                          >
                            Download Invoice Receipt PDF
                          </button>
                        </div>
                      )}

                      {paymentSuccess && (
                        <div className="text-xs p-3 rounded-lg border bg-green-50 text-green-800 border-green-200">
                          {paymentSuccess}
                        </div>
                      )}
                    </>
                  ) : (
                    <p className="text-xs text-slate-500 text-center py-10">Select an invoice from the left sidebar to view billing terminal.</p>
                  )}
                </div>
              </div>
            </div>
          )}

          {/* TAB 4: DIGITAL PRODUCTS & SUBSCRIPTIONS */}
          {activeTab === "digital" && (
            <div className="bg-white border border-slate-200 p-6 rounded-2xl shadow-sm space-y-6">
              <div className="border-b border-slate-100 pb-4">
                <h2 className="text-lg font-bold font-outfit text-slate-900">Digital Products, Online Courses & Podcasts</h2>
                <p className="text-xs text-slate-500">Access exclusive digital subscriptions, membership content, coaching programs, and podcasts.</p>
              </div>

              {/* Course Access Block */}
              <div className="border border-slate-200 rounded-xl p-5 space-y-4">
                <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-3">
                  <div>
                    <span className="text-[9px] font-extrabold uppercase bg-blue-100 text-blue-800 px-2 py-0.5 rounded">
                      Online Course Included
                    </span>
                    <h3 className="font-bold text-sm text-slate-800 mt-1">{currentCourse.title}</h3>
                    <p className="text-xs text-slate-500">{currentCourse.instructor}</p>
                  </div>
                  <div className="text-right">
                    <p className="text-xs font-bold text-blue-600">{currentCourse.progress}% Completed</p>
                    <div className="w-24 bg-slate-200 h-2 rounded-full overflow-hidden mt-1">
                      <div className="bg-blue-600 h-full" style={{ width: `${currentCourse.progress}%` }} />
                    </div>
                  </div>
                </div>

                {/* Lessons list */}
                <div className="space-y-2 bg-slate-50 p-3 rounded-lg border border-slate-200">
                  <h4 className="text-xs font-bold text-slate-700">Course Syllabus & Chapters:</h4>
                  <div className="divide-y divide-slate-200">
                    {currentCourse.lessons.map((lesson) => (
                      <div key={lesson.id} className="flex items-center justify-between py-2 text-xs">
                        <div className="flex items-center gap-2">
                          <input
                            type="checkbox"
                            checked={lesson.completed}
                            onChange={() => toggleLesson(lesson.id)}
                            className="rounded"
                          />
                          <span className={lesson.completed ? "line-through text-slate-400" : "font-medium text-slate-700"}>
                            {lesson.title}
                          </span>
                        </div>
                        <span className="text-slate-400 text-[10px]">{lesson.duration}</span>
                      </div>
                    ))}
                  </div>
                </div>
              </div>

              {/* Podcasts Block */}
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div className="border border-slate-200 rounded-xl p-4 flex flex-col justify-between">
                  <div>
                    <span className="text-[9px] font-extrabold uppercase bg-indigo-100 text-indigo-800 px-2 py-0.5 rounded">
                      Podcast & Audio Guides
                    </span>
                    <h4 className="font-bold text-xs text-slate-800 mt-2">The One Human Podcast</h4>
                    <p className="text-[11px] text-slate-500 mt-1">Tune in to regular insights on business process automations.</p>

                    <div className="mt-3 space-y-2">
                      {podcastEpisodes.map((ep) => (
                        <button
                          key={ep.id}
                          onClick={() => {
                            setActivePodcastEpisode(ep);
                            setIsPlayingPodcast(true);
                          }}
                          className="w-full text-left p-2 hover:bg-slate-50 border border-slate-100 rounded flex items-center justify-between"
                        >
                          <div>
                            <p className="text-[10px] font-bold text-slate-700 truncate max-w-[200px]">{ep.title}</p>
                            <p className="text-[9px] text-slate-400">{ep.duration} | Published {ep.published}</p>
                          </div>
                          <span className="text-xs">▶️</span>
                        </button>
                      ))}
                    </div>
                  </div>
                </div>

                {/* Active Player */}
                <div className="border border-slate-200 rounded-xl p-4 bg-slate-950 text-white flex flex-col justify-between">
                  {activePodcastEpisode ? (
                    <>
                      <div>
                        <div className="flex justify-between items-start">
                          <span className="text-[9px] bg-red-600 px-2 py-0.5 rounded font-extrabold">NOW PLAYING</span>
                          <button onClick={() => setActivePodcastEpisode(null)} className="text-slate-400 hover:text-white text-xs">✕</button>
                        </div>
                        <h4 className="font-bold text-xs text-slate-100 mt-3 truncate">{activePodcastEpisode.title}</h4>
                        <p className="text-[10px] text-slate-400 mt-1 line-clamp-2">{activePodcastEpisode.description}</p>
                      </div>

                      <div className="mt-4 space-y-2">
                        {/* Stream Controls */}
                        <div className="flex items-center justify-between text-xs">
                          <button onClick={() => setIsPlayingPodcast(!isPlayingPodcast)} className="bg-white text-slate-900 rounded-full w-8 h-8 flex items-center justify-center font-bold">
                            {isPlayingPodcast ? "⏸" : "▶️"}
                          </button>
                          <div className="flex-1 mx-3">
                            <input
                              type="range"
                              value={podcastProgress}
                              onChange={(e) => setPodcastPodcastProgress(Number(e.target.value))}
                              className="w-full accent-blue-500 h-1 bg-slate-800 rounded-lg cursor-pointer"
                            />
                          </div>
                          <span className="text-[9px] text-slate-400">14:55 / {activePodcastEpisode.duration}</span>
                        </div>
                      </div>
                    </>
                  ) : (
                    <div className="h-full flex flex-col items-center justify-center py-8 text-center">
                      <span className="text-2xl">📻</span>
                      <p className="text-xs font-bold text-slate-300 mt-2">No Episode Selected</p>
                      <p className="text-[10px] text-slate-500">Pick an audio guide from the list to stream.</p>
                    </div>
                  )}
                </div>
              </div>
            </div>
          )}

          {/* TAB 5: PROJECT TRACKER */}
          {activeTab === "projects" && (
            <div className="bg-white border border-slate-200 p-6 rounded-2xl shadow-sm space-y-6">
              <div className="border-b border-slate-100 pb-4">
                <h2 className="text-lg font-bold font-outfit text-slate-900">Project Tracker & Active Workflows</h2>
                <p className="text-xs text-slate-500">View real-time status of multi-agent swarm task executions and technical deliverables.</p>
              </div>

              {/* Progress Overview */}
              <div className="bg-slate-50 p-4 border border-slate-200 rounded-xl flex items-center justify-between">
                <div>
                  <p className="text-xs text-slate-500">Current Phase</p>
                  <p className="text-sm font-bold text-slate-800">Phase 2: Sandbox Integrations</p>
                </div>
                <div className="text-right">
                  <p className="text-xs text-slate-500">Overall Completion</p>
                  <p className="text-sm font-extrabold text-blue-600">55% Complete</p>
                </div>
              </div>

              {/* Tasks List */}
              <div className="space-y-3">
                {tasks.map((task) => (
                  <div key={task.id} className="p-4 border border-slate-100 rounded-xl flex items-center justify-between hover:bg-slate-50 transition-all bg-white shadow-sm">
                    <div className="flex items-center gap-3">
                      <span className="text-lg">
                        {task.status === "Completed" && "✅"}
                        {task.status === "In Progress" && "⚙️"}
                        {task.status === "Pending" && "⏳"}
                      </span>
                      <div>
                        <h4 className="text-xs font-bold text-slate-800">{task.name}</h4>
                        <p className="text-[9px] text-slate-400">Updated: {task.updatedAt}</p>
                      </div>
                    </div>
                    <span
                      className={`text-[9px] font-extrabold px-2 py-0.5 rounded-full ${
                        task.status === "Completed"
                          ? "bg-green-100 text-green-700"
                          : task.status === "In Progress"
                          ? "bg-blue-100 text-blue-700 animate-pulse"
                          : "bg-slate-100 text-slate-500"
                      }`}
                    >
                      {task.status}
                    </span>
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* TAB 6: SUPPORT & MESSAGING */}
          {activeTab === "support" && (
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
              {/* Support Tickets Builder */}
              <div className="md:col-span-1 bg-white border border-slate-200 p-5 rounded-2xl shadow-sm space-y-4">
                <div className="border-b border-slate-100 pb-3">
                  <h3 className="font-bold text-sm text-slate-900">Create Helpdesk Ticket</h3>
                  <p className="text-[10px] text-slate-500">File a support inquiry directly with our automated agent dispatch.</p>
                </div>

                <form onSubmit={handleSubmitTicket} className="space-y-3">
                  <div>
                    <label className="block text-[10px] font-bold text-slate-500 uppercase mb-1">Inquiry Category</label>
                    <select
                      value={ticketCategory}
                      onChange={(e) => setTicketCategory(e.target.value)}
                      className="w-full text-xs p-2 border border-slate-300 rounded focus:ring-1 focus:ring-blue-500 outline-none bg-white"
                    >
                      <option value="billing">Billing & Invoices</option>
                      <option value="technical">Technical Integration</option>
                      <option value="courses">Digital Subscriptions & Courses</option>
                    </select>
                  </div>

                  <div>
                    <label className="block text-[10px] font-bold text-slate-500 uppercase mb-1">Priority Level</label>
                    <div className="flex gap-2">
                      {["low", "medium", "high"].map((p) => (
                        <button
                          key={p}
                          type="button"
                          onClick={() => setTicketPriority(p)}
                          className={`flex-1 text-center text-[10px] py-1.5 rounded font-bold uppercase border transition-all ${
                            ticketPriority === p
                              ? "bg-slate-900 text-white border-slate-900"
                              : "bg-white text-slate-500 border-slate-200 hover:bg-slate-50"
                          }`}
                        >
                          {p}
                        </button>
                      ))}
                    </div>
                  </div>

                  <div>
                    <label className="block text-[10px] font-bold text-slate-500 uppercase mb-1">Description</label>
                    <textarea
                      rows={3}
                      value={ticketDescription}
                      onChange={(e) => setTicketDescription(e.target.value)}
                      placeholder="Detail your request..."
                      className="w-full text-xs p-2.5 border border-slate-300 rounded focus:ring-1 focus:ring-blue-500 outline-none resize-none"
                    />
                  </div>

                  <button
                    type="submit"
                    className="w-full bg-slate-900 hover:bg-black text-white text-xs font-bold py-2 rounded transition-all"
                  >
                    Submit Support Ticket
                  </button>
                </form>

                {ticketSuccess && (
                  <div className="text-xs p-3 rounded-lg border bg-blue-50 text-blue-800 border-blue-200">
                    {ticketSuccess}
                  </div>
                )}
              </div>

              {/* Chat Interface */}
              <div className="md:col-span-2 bg-white border border-slate-200 rounded-2xl shadow-sm flex flex-col justify-between h-[450px] overflow-hidden">
                <div className="bg-slate-900 px-4 py-3 flex items-center justify-between text-white">
                  <div className="flex items-center gap-2">
                    <span className="w-2.5 h-2.5 rounded-full bg-green-500" />
                    <div>
                      <h3 className="font-bold text-xs">OHC Automated Support</h3>
                      <p className="text-[9px] text-slate-400">Response time: &lt; 1 minute</p>
                    </div>
                  </div>
                  <span className="text-[10px] bg-slate-800 px-2 py-1 rounded">Live Chat</span>
                </div>

                {/* Messages Body */}
                <div className="flex-1 p-4 overflow-y-auto space-y-4 bg-slate-50/50">
                  {chatMessages.map((msg) => (
                    <div
                      key={msg.id}
                      className={`flex flex-col ${
                        msg.sender === "user" ? "items-end" : "items-start"
                      }`}
                    >
                      <div
                        className={`max-w-[80%] p-3 rounded-2xl text-xs leading-relaxed ${
                          msg.sender === "user"
                            ? "bg-blue-600 text-white rounded-tr-none"
                            : "bg-white border border-slate-200 text-slate-800 rounded-tl-none shadow-sm"
                        }`}
                      >
                        {msg.text}
                      </div>
                      <span className="text-[8px] text-slate-400 mt-1 px-1">{msg.timestamp}</span>
                    </div>
                  ))}
                </div>

                {/* Input Area */}
                <form onSubmit={handleSendChatMessage} className="p-3 border-t border-slate-200 flex gap-2 bg-white">
                  <input
                    type="text"
                    value={currentMessageText}
                    onChange={(e) => setCurrentMessageText(e.target.value)}
                    placeholder="Type a message or ask about your bills/proposals..."
                    className="flex-1 text-xs px-3 py-2 border border-slate-300 rounded-full focus:ring-1 focus:ring-blue-500 outline-none"
                  />
                  <button
                    type="submit"
                    className="bg-blue-600 hover:bg-blue-700 text-white font-bold rounded-full w-8 h-8 flex items-center justify-center text-xs transition-all"
                  >
                    ➤
                  </button>
                </form>
              </div>
            </div>
          )}
        </section>
      </div>
    </div>
  );
}
