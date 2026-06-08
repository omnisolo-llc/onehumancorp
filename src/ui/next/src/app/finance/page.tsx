"use client";

import { useRouter } from 'next/navigation';

export default function FinancePage() {
    const router = useRouter();

    return (
        <div className="min-h-screen bg-gray-50 flex items-center justify-center p-4">
            <div className="w-[375px] max-w-full space-y-6">
                <div className="bg-white/70 backdrop-blur-xl border border-white/20 shadow-xl rounded-3xl p-6 text-gray-800">
                    <h1 className="text-2xl font-semibold mb-6">Finance</h1>
                    <button
                        onClick={() => router.push('/finance/invoices/new')}
                        className="w-full py-4 bg-blue-600 hover:bg-blue-500 text-white rounded-xl font-medium shadow-lg transition-all active:scale-95"
                    >
                        New Invoice
                    </button>
                </div>
            </div>
        </div>
    );
}
