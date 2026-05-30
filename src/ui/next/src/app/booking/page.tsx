'use client';

import React, { useState, useEffect } from 'react';
import Link from 'next/link';

export default function BookingPage() {
    const [bookings, setBookings] = useState([]);
    const [services, setServices] = useState([]);
    const [loading, setLoading] = useState(true);

    const [showBookingModal, setShowBookingModal] = useState(false);
    const [selectedService, setSelectedService] = useState('');
    const [bookingDate, setBookingDate] = useState('');
    const [bookingTime, setBookingTime] = useState('');

    useEffect(() => {
        const fetchBookings = async () => {
            try {
                const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store';
                const token = typeof localStorage !== 'undefined' ? localStorage.getItem('token') || '' : '';
                const response = await fetch('/api/booking', {
                    headers: {
                        'Authorization': `Bearer ${token}`
                    }
                });
                if (response.ok) {
                    const data = await response.json();
                    setBookings(data);
                }
            } catch (e) {
                console.error("Failed to fetch bookings", e);
            } finally {
                setLoading(false);
            }
        };

        const fetchServices = async () => {
            // Mock fetching services
            setServices([
                { id: '1', title: 'Consultation', price_cents: 5000 },
                { id: '2', title: 'Repair', price_cents: 10000 },
            ]);
        }

        fetchBookings();
        fetchServices();
    }, []);

    const handleCreateBooking = async () => {
        if (!selectedService || !bookingDate || !bookingTime) return;

        try {
            const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store';
            const token = typeof localStorage !== 'undefined' ? localStorage.getItem('token') || '' : '';

            const [year, month, day] = bookingDate.split('-');
            const [hour, minute] = bookingTime.split(':');

            const startDate = new Date(parseInt(year), parseInt(month) - 1, parseInt(day), parseInt(hour), parseInt(minute));

            const response = await fetch('/api/booking', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${token}`
                },
                body: JSON.stringify({
                    id: '',
                    tenant_id: tenantId,
                    customer_id: 'dummy-customer',
                    product_id: selectedService,
                    start_time: startDate.toISOString(),
                    end_time: null,
                    status: 'pending'
                })
            });

            if (response.ok) {
                const newBooking = await response.json();
                setBookings([...bookings, newBooking]);
                setShowBookingModal(false);
            } else {
                console.error("Failed to create booking");
            }
        } catch (e) {
            console.error("Error", e);
        }
    }

    if (loading) {
        return <div className="p-8 text-center">Loading bookings...</div>;
    }

    return (
        <div className="flex flex-col min-h-screen font-inter bg-gray-50">
            <header className="px-6 py-4 flex items-center justify-between border-b bg-white sticky top-0 z-50">
                <div className="flex items-center gap-4">
                    <Link href="/dashboard" className="text-gray-500 hover:text-gray-900 transition-colors">
                        <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
                    </Link>
                    <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">Bookings</h1>
                </div>
                <button
                    onClick={() => setShowBookingModal(true)}
                    className="px-4 py-2 bg-blue-600 text-white rounded-lg font-medium text-sm hover:bg-blue-700 transition-colors shadow-sm"
                >
                    + New Booking
                </button>
            </header>

            <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full flex flex-col gap-6">
                {bookings.length === 0 ? (
                    <div className="text-center py-12 bg-white rounded-xl border border-gray-100 shadow-sm">
                        <div className="text-4xl mb-4">📅</div>
                        <h3 className="text-lg font-semibold text-gray-900 mb-2">No bookings yet</h3>
                        <p className="text-gray-500 mb-6">Create your first booking or share your calendar link with clients.</p>
                        <button
                            onClick={() => setShowBookingModal(true)}
                            className="px-4 py-2 bg-blue-600 text-white rounded-lg font-medium hover:bg-blue-700 transition-colors"
                        >
                            Create Booking
                        </button>
                    </div>
                ) : (
                    <div className="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden">
                        <table className="w-full text-left">
                            <thead className="bg-gray-50 border-b border-gray-100">
                                <tr>
                                    <th className="px-6 py-3 text-xs font-semibold text-gray-500 uppercase tracking-wider">Service</th>
                                    <th className="px-6 py-3 text-xs font-semibold text-gray-500 uppercase tracking-wider">Time</th>
                                    <th className="px-6 py-3 text-xs font-semibold text-gray-500 uppercase tracking-wider">Status</th>
                                </tr>
                            </thead>
                            <tbody className="divide-y divide-gray-100">
                                {bookings.map((booking: any) => (
                                    <tr key={booking.id} className="hover:bg-gray-50 transition-colors">
                                        <td className="px-6 py-4">
                                            <div className="font-medium text-gray-900">{booking.product_id}</div>
                                            <div className="text-sm text-gray-500">Client: {booking.customer_id}</div>
                                        </td>
                                        <td className="px-6 py-4 text-sm text-gray-600">
                                            {new Date(booking.start_time).toLocaleString()}
                                        </td>
                                        <td className="px-6 py-4">
                                            <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${booking.status === 'pending' ? 'bg-yellow-100 text-yellow-800' : 'bg-green-100 text-green-800'}`}>
                                                {booking.status}
                                            </span>
                                        </td>
                                    </tr>
                                ))}
                            </tbody>
                        </table>
                    </div>
                )}
            </main>

            {showBookingModal && (
                <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4 backdrop-blur-sm">
                    <div className="bg-white w-full max-w-md rounded-2xl p-6 shadow-2xl relative font-inter">
                        <div className="flex justify-between items-center mb-6">
                            <h2 className="text-xl font-bold text-gray-900">New Booking</h2>
                            <button
                                onClick={() => setShowBookingModal(false)}
                                className="text-gray-400 hover:text-gray-600 transition-colors"
                            >
                                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
                            </button>
                        </div>

                        <div className="space-y-4">
                            <div>
                                <label className="block text-sm font-medium text-gray-700 mb-1">Select Service</label>
                                <select
                                    className="w-full border border-gray-300 rounded-lg p-2.5 text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none"
                                    value={selectedService}
                                    onChange={(e) => setSelectedService(e.target.value)}
                                >
                                    <option value="">-- Choose a service --</option>
                                    {services.map((s: any) => (
                                        <option key={s.id} value={s.id}>{s.title} (${s.price_cents / 100})</option>
                                    ))}
                                </select>
                            </div>

                            <div className="grid grid-cols-2 gap-4">
                                <div>
                                    <label className="block text-sm font-medium text-gray-700 mb-1">Date</label>
                                    <input
                                        type="date"
                                        className="w-full border border-gray-300 rounded-lg p-2.5 text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none"
                                        value={bookingDate}
                                        onChange={(e) => setBookingDate(e.target.value)}
                                    />
                                </div>
                                <div>
                                    <label className="block text-sm font-medium text-gray-700 mb-1">Time</label>
                                    <input
                                        type="time"
                                        className="w-full border border-gray-300 rounded-lg p-2.5 text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none"
                                        value={bookingTime}
                                        onChange={(e) => setBookingTime(e.target.value)}
                                    />
                                </div>
                            </div>
                        </div>

                        <div className="mt-8 flex justify-end gap-3">
                            <button
                                onClick={() => setShowBookingModal(false)}
                                className="px-4 py-2 text-sm font-medium text-gray-600 hover:text-gray-800 transition-colors"
                            >
                                Cancel
                            </button>
                            <button
                                onClick={handleCreateBooking}
                                className="px-6 py-2 bg-blue-600 text-white text-sm font-medium rounded-lg hover:bg-blue-700 transition-colors shadow-sm disabled:opacity-50 disabled:cursor-not-allowed"
                                disabled={!selectedService || !bookingDate || !bookingTime}
                            >
                                Schedule
                            </button>
                        </div>
                    </div>
                </div>
            )}
        </div>
    );
}
