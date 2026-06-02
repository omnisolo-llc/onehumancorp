# Real Business Owner Persona: Leo the Music Tutor

**Business Concept:** Leo is a freelance music tutor offering 1-hour piano lessons. He wants his students to be able to reserve a time slot, get a quote, and pay a deposit seamlessly from their phone.

**Operating Plan:**
1. Log in to OHC Dashboard as Leo.
2. Verify or create the "1-hour Piano Lesson" service/product with a $100 price and a required $25 deposit.
3. Access the newly created Booking UI or generate a quote link for a specific customer.
4. The customer navigates to the booking link to review the quote, pick an available time slot, and confirm the reservation.
5. The system transitions the slot to "Pending Deposit" and waits for payment.

**CUJ Workflow Flowchart:**

[Screen 1: Dashboard Home] -> (Leo logs in)
   -> State: User authenticated, tenant "leo_tutor" active
   -> Action: Navigates to Services/Products page

[Screen 2: Services List] -> (Leo creates service)
   -> Action: Click "Add Service"
   -> State: Modal opens
   -> Validation: Modal is visible

[Screen 3: Service Creation] -> (Leo details service)
   -> Action: Enter "Piano Lesson", Price: $100, Deposit: $25 -> Click "Save"
   -> Expected State: Service "Piano Lesson" is saved to DB.

[Screen 4: Booking Quote Generation] -> (Leo triggers quote)
   -> Action: Navigates to Bookings/Quotes -> Click "New Quote" -> Selects "Piano Lesson"
   -> State: Quote generated, link displayed
   -> Validation: Link is visible and copied to clipboard

[Screen 5: Customer Public Booking Link] -> (Customer views quote)
   -> Action: Customer opens link in browser
   -> Expected State: Clean glassmorphism UI shows Quote for Piano Lesson ($100 total, $25 deposit).
   -> Validation: Calendar widget is visible.

[Screen 6: Customer Timeslot Selection] -> (Customer reserves time)
   -> Action: Customer selects "Tomorrow 4 PM" -> Clicks "Confirm & Pay Deposit"
   -> Expected State: API calls `/api/v1/booking/reserve` with lock acquired, prevents double-booking, and successfully creates `Pending Deposit` booking.
   -> Final Proof Point: Success message is shown, customer is redirected to Stripe Checkout session for the $25 deposit.
