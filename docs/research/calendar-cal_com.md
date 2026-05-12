# Automate Appointment Scheduling with Cal.com

**Problem Statement**
Coordinating appointment times with clients involves too much back-and-forth emailing. I need a simple way to let clients see my availability and book time with me directly, without double-booking my personal calendar.

**Research Report**
Cal.com is an open-source, flexible scheduling tool. It syncs with Google Calendar, Outlook, and Apple Calendar. It handles timezone conversions automatically. For a small business owner, it's very easy to share a booking link. The pricing is very attractive: there's a generous free tier for individuals, and team plans start at $12/user/month. It integrates perfectly via API and Webhooks, making it suitable for both Cloud and Standalone environments.

**Design Doc**
Users will be able to connect their Cal.com account in the OHC platform. Once connected, they can generate a booking link that they can share with clients or embed on their website. OHC will listen for booking events so that upcoming appointments are displayed on the business owner's daily dashboard. Conflicting events from their connected calendars will automatically block off time.

**Implementation Prompt**
Create an integration with Cal.com that allows the user to connect their account. Once connected, display their upcoming appointments on the main dashboard. Provide a quick button to copy their booking link. Acceptance criteria: Successful connection to Cal.com, accurate display of upcoming bookings in the dashboard, and proper syncing of new bookings via webhooks.

**Priority:** P1
**Estimated Scope:** Medium
