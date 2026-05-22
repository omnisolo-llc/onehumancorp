"use client";
import { SmartBlock } from "../builder/components";
import React from 'react';

export default function TestPage() {
    return (
        <div className="w-[375px] mx-auto h-[812px] bg-gray-50 pt-10 shadow-2xl relative border-x border-gray-200">
            <SmartBlock
                type="Analytics"
                props={{
                    todaySales: "$1,040.00",
                    activeOrders: "24",
                    totalCustomers: "100",
                    url: "https://ohc.store"
                }}
            />
        </div>
    );
}
