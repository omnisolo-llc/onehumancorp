# Title: Auto-Generated Meeting Links via Zoom Integration

## Problem Statement
Tutors, coaches, and consultants spend too much time manually creating Zoom links for every single booking, copying them, and emailing them to clients. If they forget, the client shows up to an empty meeting. They need a unique link auto-generated for every virtual appointment.

## Research Report
- **Tool Evaluated**: Zoom API
- **Benefit to Users**: Zero manual work for online meetings. Every booking automatically gets a secure, unique video room.
- **Ease of Use**: Owner clicks "Connect Zoom" once. Every new virtual appointment automatically includes a "Join Meeting" button for both the owner and the client.
- **Pricing**: The API is free to use. The business owner must have their own Zoom account (free or paid).
- **Integration Risks**: Zoom's Server-to-Server OAuth apps have strict expiration and rotation rules. Ensuring the token is always fresh before a booking is made is critical.
- **Environment**: Cloud and Standalone compatible.

## Design Doc
- **Trigger**: A new appointment is booked where the location is set to "Virtual / Video Call".
- **Action**: OHC calls the Zoom API using the owner's linked account to generate a new Meeting ID and password.
- **User Interface**: The OHC dashboard schedule shows a bright blue "Start Zoom Meeting" button next to the appointment. The customer's confirmation email includes the join link and passcode.

## Implementation Prompt
Integrate the Zoom API to automatically provision meeting rooms for virtual appointments. Allow users to link their Zoom account via OAuth. When an appointment is scheduled, generate a unique meeting link and attach it to the appointment record. Expose the link in the dashboard UI and include it in the customer confirmation notifications.

## Priority
P2

## Estimated Scope
Medium