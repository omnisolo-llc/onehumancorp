import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json([
    {
      id: 1,
      title: "Set up your store",
      duration: "1:20",
      description: "A quick walkthrough on how to generate your storefront using our AI tools.",
      url: "https://www.w3schools.com/html/mov_bbb.mp4"
    },
    {
      id: 2,
      title: "Accept your first payment",
      duration: "0:55",
      description: "Learn how to link your bank account and start taking money from customers.",
      url: "https://www.w3schools.com/html/mov_bbb.mp4"
    },
    {
      id: 3,
      title: "Activate your AI Support Agent",
      duration: "1:10",
      description: "See how to hire an AI helper to answer customer messages for you.",
      url: "https://www.w3schools.com/html/mov_bbb.mp4"
    },
    {
      id: 4,
      title: "Add your first product",
      duration: "1:05",
      description: "Add an item to sell, complete with a picture, price, and description.",
      url: "https://www.w3schools.com/html/mov_bbb.mp4"
    },
    {
      id: 5,
      title: "Run a weekend sale",
      duration: "0:45",
      description: "Create a discount code to send to your best customers.",
      url: "https://www.w3schools.com/html/mov_bbb.mp4"
    },
    {
      id: 6,
      title: "Track your inventory",
      duration: "0:50",
      description: "Make sure you never sell an item you don't actually have in stock.",
      url: "https://www.w3schools.com/html/mov_bbb.mp4"
    },
    {
      id: 7,
      title: "Check your daily analytics",
      duration: "1:15",
      description: "Understand your sales dashboard and what it means for your business.",
      url: "https://www.w3schools.com/html/mov_bbb.mp4"
    },
    {
      id: 8,
      title: "Send an email newsletter",
      duration: "1:30",
      description: "Use our AI tools to draft a beautiful email to send to your subscribers.",
      url: "https://www.w3schools.com/html/mov_bbb.mp4"
    },
    {
      id: 9,
      title: "Change your store design",
      duration: "1:00",
      description: "Update the colors and fonts of your storefront easily.",
      url: "https://www.w3schools.com/html/mov_bbb.mp4"
    },
    {
      id: 10,
      title: "Manage team members",
      duration: "0:40",
      description: "Invite your partners or employees to help manage your store.",
      url: "https://www.w3schools.com/html/mov_bbb.mp4"
    }
  ]);
}