"use client";

import React, { useState, useEffect } from "react";
import { useParams } from 'next/navigation';
import { motion } from "framer-motion";
import { FiCheck, FiArrowRight, FiShield, FiCalendar, FiClock, FiLoader } from "react-icons/fi";

interface QuoteItem {
  id: string;
  description: string;
  unit_price_cents: number;
  quantity: number;
  is_optional: boolean;
}

interface Quote {
  id: string;
  status: 'SENT' | 'ACCEPTED' | 'REJECTED';
  valid_until?: string;
}

export default function CustomerProposalPage() {
  const { id } = useParams();
  const [data, setData] = useState<{ quote: Quote, items: QuoteItem[] } | null>(null);
  const [loading, setLoading] = useState(true);
  const [accepting, setAccepting] = useState(false);
  const [accepted, setAccepted] = useState(false);

  useEffect(() => {
    if (id) {
      fetchQuote(id as string);
    }
  }, [id]);

  const fetchQuote = async (quoteId: string) => {
    try {
      setLoading(true);
      const res = await fetch(`/api/v1/quoting/quotes/${quoteId}/public`);
      if (res.ok) {
        const json = await res.json();
        setData(json);
        if (json.quote.status === 'ACCEPTED') {
          setAccepted(true);
        }
      }
    } catch (error) {
      console.error("Failed to fetch proposal:", error);
    } finally {
      setLoading(false);
    }
  };

  const handleAccept = async () => {
    setAccepting(true);
    try {
      const res = await fetch(`/api/v1/quoting/quotes/${id}/approve`, {
        method: 'PATCH'
      });
      if (res.ok) {
        setAccepted(true);
      }
    } catch (error) {
      console.error("Failed to accept proposal:", error);
    } finally {
      setAccepting(false);
    }
  };

  if (loading) {
    return (
      <div className="flex min-h-screen items-center justify-center bg-white">
        <FiLoader className="text-3xl animate-spin text-blue-600" />
      </div>
    );
  }

  if (!data) {
    return (
      <div className="flex min-h-screen items-center justify-center bg-gray-50 text-gray-500">
        <p>Proposal not found or expired.</p>
      </div>
    );
  }

  const { quote, items } = data;
  const total = items.reduce((sum, item) => sum + (item.unit_price_cents * item.quantity), 0) / 100;
  const deposit = total * 0.2;

  return (
    <div className="min-h-screen bg-[#F5F5F7] font-sans pb-12">
      {/* Premium Header */}
      <header className="bg-white/70 backdrop-blur-2xl sticky top-0 z-20 border-b border-gray-200/50">
        <div className="max-w-xl mx-auto px-6 py-5 flex items-center justify-between">
           <div className="flex items-center space-x-2">
             <div className="w-8 h-8 bg-blue-600 rounded-lg flex items-center justify-center text-white font-bold">O</div>
             <span className="font-bold text-gray-900 tracking-tight">OneHumanCorp</span>
           </div>
           <div className="text-xs font-semibold text-gray-400 uppercase tracking-widest">Proposal</div>
        </div>
      </header>

      <main className="max-w-xl mx-auto px-4 py-8 space-y-6">
        {accepted ? (
          <motion.div
            initial={{ opacity: 0, scale: 0.95 }}
            animate={{ opacity: 1, scale: 1 }}
            className="bg-white rounded-3xl p-8 text-center shadow-xl border border-gray-100"
          >
            <div className="w-20 h-20 bg-green-100 text-green-600 rounded-full flex items-center justify-center mx-auto mb-6">
              <FiCheck className="text-4xl" />
            </div>
            <h1 className="text-2xl font-bold text-gray-900 mb-2">Proposal Accepted!</h1>
            <p className="text-gray-500 mb-8">Thank you for your business. We've notified the owner and will be in touch shortly to finalize the schedule.</p>
            <div className="p-4 bg-gray-50 rounded-2xl border border-gray-100 flex items-center justify-between text-left">
               <div>
                 <p className="text-xs font-bold text-gray-400 uppercase">Deposit Paid</p>
                 <p className="text-xl font-bold text-gray-900">${deposit.toFixed(2)}</p>
               </div>
               <FiShield className="text-2xl text-blue-500" />
            </div>
          </motion.div>
        ) : (
          <>
            <section className="space-y-2">
              <h1 className="text-3xl font-bold text-gray-900 tracking-tight px-2">Service Proposal</h1>
              <div className="flex items-center space-x-4 px-2 text-sm text-gray-500">
                 <span className="flex items-center"><FiClock className="mr-1" /> Valid for 48h</span>
                 <span className="flex items-center"><FiShield className="mr-1" /> Secure OHC Deposit</span>
              </div>
            </section>

            {/* Proposal Details Card */}
            <section className="bg-white rounded-[32px] overflow-hidden shadow-sm border border-gray-200/50">
              <div className="p-6 space-y-6">
                <div className="space-y-4">
                  <h2 className="text-xs font-bold text-gray-400 uppercase tracking-widest">Line Items</h2>
                  <div className="space-y-4">
                    {items.map((item) => (
                      <div key={item.id} className="flex justify-between items-start">
                        <div className="max-w-[70%]">
                          <p className="font-semibold text-gray-900">{item.description}</p>
                          <p className="text-sm text-gray-500 italic mt-0.5">Quantity: {item.quantity}</p>
                        </div>
                        <span className="font-bold text-gray-900">${(item.unit_price_cents / 100).toFixed(2)}</span>
                      </div>
                    ))}
                  </div>
                </div>

                <div className="pt-6 border-t border-gray-100 flex justify-between items-end">
                   <div className="space-y-1">
                     <p className="text-sm text-gray-500 font-medium tracking-tight">Total Estimate</p>
                     <p className="text-3xl font-bold text-gray-900 tracking-tighter">${total.toFixed(2)}</p>
                   </div>
                   <div className="text-right space-y-1">
                     <p className="text-xs font-bold text-blue-600 uppercase tracking-wide">Deposit Due Now</p>
                     <p className="text-xl font-bold text-blue-600">${deposit.toFixed(2)}</p>
                   </div>
                </div>
              </div>

              {/* Action Area */}
              <div className="bg-gray-50 p-6 border-t border-gray-100">
                <button
                  onClick={handleAccept}
                  disabled={accepting}
                  className="w-full bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white font-bold py-4 rounded-2xl shadow-lg shadow-blue-200 transition-all flex items-center justify-center space-x-2 group"
                >
                  {accepting ? (
                    <FiLoader className="animate-spin text-xl" />
                  ) : (
                    <>
                      <span>Accept & Pay Deposit</span>
                      <FiArrowRight className="text-xl group-hover:translate-x-1 transition-transform" />
                    </>
                  )}
                </button>
                <p className="text-center text-[10px] text-gray-400 mt-4 px-4 uppercase font-bold tracking-widest leading-relaxed">
                  By paying the deposit, you agree to the service terms. OHC protects your payment until work is confirmed.
                </p>
              </div>
            </section>

            {/* Extra context cards */}
            <div className="grid grid-cols-2 gap-4">
               <div className="bg-white p-5 rounded-3xl border border-gray-200/50 shadow-sm">
                  <FiCalendar className="text-blue-500 mb-3 text-xl" />
                  <p className="font-bold text-gray-900 text-sm">Flexible Slots</p>
                  <p className="text-xs text-gray-500 mt-1">Book your preferred time immediately after payment.</p>
               </div>
               <div className="bg-white p-5 rounded-3xl border border-gray-200/50 shadow-sm">
                  <FiShield className="text-green-500 mb-3 text-xl" />
                  <p className="font-bold text-gray-900 text-sm">OHC Protected</p>
                  <p className="text-xs text-gray-500 mt-1">Funds held in escrow until you're happy with the work.</p>
               </div>
            </div>
          </>
        )}
      </main>
    </div>
  );
}
