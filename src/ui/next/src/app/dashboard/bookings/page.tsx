import React from 'react';
import Link from 'next/link';

export default function DashboardBookingsPage() {
    return (
        <div className="min-h-screen bg-gray-50 flex items-start justify-center p-0 sm:p-4 font-inter overflow-x-hidden" data-testid="owner-dashboard-bookings">
            <div className="bg-[rgba(255,255,255,0.65)] backdrop-blur-[30px] saturate-[210%] border border-[rgba(255,255,255,0.4)] rounded-2xl shadow-xl max-w-[800px] w-full p-8">
                <h1 className="text-3xl font-bold font-outfit tracking-tight mb-4">Bookings Management</h1>
                <p className="text-gray-600 mb-8">Manage your schedule, upcoming bookings, and view AI-suggested follow-ups here.</p>
                <div className="space-y-4">
                    <div className="p-4 border rounded-xl bg-white shadow-sm flex items-center justify-between">
                        <div>
                            <h3 className="font-semibold text-gray-800">Pending Bookings</h3>
                            <p className="text-sm text-gray-500">View and approve requested appointments.</p>
                        </div>
                        <Link href="/feed" className="bg-blue-600 text-white px-4 py-2 rounded-lg font-medium hover:bg-blue-700 transition-colors">
                            Go to Feed
                        </Link>
                    </div>
                </div>
            </div>
        </div>
    );
}
