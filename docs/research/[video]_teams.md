# [Video] Microsoft Teams for Professional Consultations

## Title
🔍 Scout: Integrate Microsoft Teams for Auto-Generated Consultation Links

## Problem Statement
Leo (Music Tutor) is moving his lessons online. Currently, he has to manually create a meeting and share links for every student. It looks unprofessional. He needs OHC to do this for him automatically the moment a student books a virtual lesson.

## Research Report
- **Tool**: Microsoft Teams
- **Target Persona**: Leo (Music Tutor), Consultants, Online Coaches.
- **Value Proposition**: Teams is the professional standard for video. For businesses using Microsoft 365, it is a built-in, highly secure classroom environment.
- **Key Advantages**:
  - **One-Click Generation**: Meeting links are created automatically.
  - **Professional Features**: Waiting rooms and high-quality recording are standard.
  - **Ecosystem Integration**: Works seamlessly with professional calendars.
- **Risks**: Requires a stable internet connection.
- **Pricing**: Included in standard professional subscriptions.
- **Compatibility**: Fully supported in both Cloud and Standalone modes.

## Design Doc
- **User Experience**:
  - The owner connects their Microsoft account.
  - They mark a service as "Virtual."
  - When a student books, OHC creates a unique Teams link.
  - Confirmation messages to the student include a "Join" button.
- **Visuals**: A professional "Join Meeting" interface within OHC.

## Implementation Prompt
Extend the Microsoft integration to support virtual meeting generation. When a virtual service is booked, the system should automatically generate a unique Microsoft Teams meeting link. This link must be included in all confirmation messages. Ensure the system handles meeting lifecycle changes correctly.

## Priority
P1

## Estimated Scope
Medium
