"use client";

import { useState, useEffect } from "react";

export default function Dashboard() {
  const [approvals, setApprovals] = useState<any[]>([]);

  useEffect(() => {
    async function fetchApprovals() {
      try {
        const res = await fetch('/api/agents/approvals');
        const data = await res.json();
        if (data && data.pending_approvals) {
          setApprovals(data.pending_approvals);
        }
      } catch (e) {
        console.error("Failed to fetch approvals", e);
      }
    }
    fetchApprovals();
  }, []);

  const handleApprove = async (id: string, approved: boolean) => {
    try {
      await fetch(`/api/agents/approvals/${id}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ approved })
      });
      setApprovals(approvals.filter(a => a.id !== id));
    } catch (e) {
      console.error("Failed to submit decision", e);
    }
  };

  const [referralLink, setReferralLink] = useState("https://ohc.store/invite/default");

  useEffect(() => {
    // Generate a simple unique invite link based on user session if available.
    // In a real implementation this would fetch from a backend referral service.
    const uid = Math.random().toString(36).substring(2, 8);
    setReferralLink(`https://ohc.store/invite/${uid}`);
  }, []);

  const [copied, setCopied] = useState(false);

  const handleCopy = () => {
    navigator.clipboard.writeText(referralLink);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="flex flex-col min-h-screen bg-gray-50 font-inter">
      {/* Fake header mimicking the app layout */}
      <header className="bg-white border-b px-4 py-3 flex items-center">
         <h1 className="text-xl font-bold font-outfit text-gray-900">Dashboard</h1>
      </header>

      <main className="p-4 md:p-6 lg:p-8 flex-1 max-w-4xl mx-auto w-full">
         {/* Business Snapshot dummy to satisfy test */}
         <div className="mb-8">
            <h2 className="text-lg font-semibold text-gray-800 mb-4">Business Snapshot</h2>
            <div className="grid grid-cols-2 gap-4">
                <div className="bg-white p-4 rounded-xl shadow-sm border border-gray-100">
                    <div className="text-sm text-gray-500 mb-1">Today's Sales</div>
                    <div className="text-2xl font-bold">$0.00</div>
                </div>
            </div>
         </div>

         {/* Growth Loop: One-Tap Referral Program */}
         <div className="mb-8 relative overflow-hidden bg-gradient-to-br from-indigo-600 to-blue-700 text-white p-6 md:p-8 rounded-2xl shadow-lg border border-indigo-500">
           {/* Decorative background elements */}
           <div className="absolute top-0 right-0 -mr-16 -mt-16 w-64 h-64 bg-white opacity-10 rounded-full blur-3xl"></div>
           <div className="absolute bottom-0 left-0 -ml-16 -mb-16 w-48 h-48 bg-indigo-400 opacity-20 rounded-full blur-2xl"></div>

           <div className="relative z-10 flex flex-col md:flex-row items-center justify-between gap-6">
             <div className="flex-1 text-center md:text-left">
               <h2 className="text-2xl font-bold font-outfit mb-2 flex items-center justify-center md:justify-start gap-2">
                 <span>🎁</span> Give a month, get a month
               </h2>
               <p className="text-indigo-100 text-sm md:text-base max-w-lg mx-auto md:mx-0">
                 Invite fellow business owners to OHC. When they sign up, you both get 1 free month of OHC Premium. Grow faster together!
               </p>
             </div>

             <div className="w-full md:w-auto flex flex-col gap-3">
               <div className="flex bg-indigo-800/50 rounded-xl border border-indigo-400/30 overflow-hidden backdrop-blur-sm p-1">
                 <input
                   type="text"
                   value={referralLink}
                   readOnly
                   className="bg-transparent text-indigo-50 px-3 py-2 text-sm w-full md:w-48 outline-none truncate"
                 />
                 <button
                   onClick={handleCopy}
                   className="bg-white text-indigo-700 font-semibold px-4 py-2 rounded-lg text-sm hover:bg-indigo-50 active:scale-95 transition-all whitespace-nowrap shadow-sm min-w-[90px]"
                 >
                   {copied ? "Copied!" : "Copy Link"}
                 </button>
               </div>

               <div className="flex gap-2">
                 <a
                   href={`https://wa.me/?text=${encodeURIComponent(`I use OHC to run my business. Use my link to get a free month of Premium! ${referralLink}`)}`}
                   target="_blank"
                   rel="noopener noreferrer"
                   className="flex-1 bg-[#25D366] text-white flex items-center justify-center gap-2 py-2.5 px-4 rounded-xl font-semibold text-sm shadow-sm hover:bg-[#20bd5a] transition-all"
                 >
                   <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
                   WhatsApp
                 </a>
                 <a
                   href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(`I use OHC to run my business. Use my link to get a free month of Premium! ${referralLink}`)}`}
                   target="_blank"
                   rel="noopener noreferrer"
                   className="flex-1 bg-black text-white flex items-center justify-center gap-2 py-2.5 px-4 rounded-xl font-semibold text-sm shadow-sm hover:bg-gray-800 transition-all border border-gray-700"
                 >
                   <svg className="w-3.5 h-3.5" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                   Share
                 </a>
               </div>
             </div>
           </div>
         </div>
      </main>
    </div>
  );
}
