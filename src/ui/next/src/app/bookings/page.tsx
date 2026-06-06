'use client';

import React, { useState } from 'react';
import { Card, Title, Text, Button } from '@tremor/react';

export default function BookingsPage() {
  const [requests, setRequests] = useState([
    { id: 1, customer: 'John Doe', service: 'Guitar Lesson', time: '10:00 AM - 11:00 AM' }
  ]);

  return (
    <div className="p-4 max-w-[375px] mx-auto bg-gray-50 min-h-screen">
      <Title className="text-xl font-bold mb-4">Agenda</Title>

      <div className="space-y-4">
        {requests.map((request) => (
          <Card key={request.id} className="backdrop-blur-xl bg-white/50 shadow-sm border border-gray-100">
            <div className="flex flex-col space-y-2">
              <div className="flex justify-between items-start">
                <Title className="text-base">{request.service}</Title>
                <span className="text-xs font-medium px-2 py-1 bg-blue-100 text-blue-800 rounded-full">New Request</span>
              </div>
              <Text className="text-sm text-gray-600">{request.customer}</Text>
              <Text className="text-sm font-medium text-gray-800">{request.time}</Text>
              <div className="flex space-x-2 pt-2">
                <Button size="sm" variant="primary" className="w-full">Approve</Button>
                <Button size="sm" variant="secondary" color="red" className="w-full">Decline</Button>
              </div>
            </div>
          </Card>
        ))}

        {requests.length === 0 && (
          <Text className="text-center text-gray-500 py-8">No new booking requests</Text>
        )}
      </div>
    </div>
  );
}
