import React from 'react';

export default function BookingWidget() {
    return (
        <div className="booking-widget">
            <h2>Book a Service</h2>
            <form>
                {/* Form fields for booking an appointment, taking deposits etc */}
                <input type="text" placeholder="Your Name" />
                <button type="submit">Book Now</button>
            </form>
        </div>
    );
}
