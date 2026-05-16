pub fn render_booking_widget() -> String {
    let mut html = String::from(r#"
        <div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255,255,255,0.1);">
            <h2>Book an Appointment</h2>
            <form>
                <label>Name:</label>
                <input type="text" name="name" />
                <br />
                <label>Date:</label>
                <input type="date" name="date" />
                <br />
    "#);

    html.push_str("<div class='slot-time'>Slot 0: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 1: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 2: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 3: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 4: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 5: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 6: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 7: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 8: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 9: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 10: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 11: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 12: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 13: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 14: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 15: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 16: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 17: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 18: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 19: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 20: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 21: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 22: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 23: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 24: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 25: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 26: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 27: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 28: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 29: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 30: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 31: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 32: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 33: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 34: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 35: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 36: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 37: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 38: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 39: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 40: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 41: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 42: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 43: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 44: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 45: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 46: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 47: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 48: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 49: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 50: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 51: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 52: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 53: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 54: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 55: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 56: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 57: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 58: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 59: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 60: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 61: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 62: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 63: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 64: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 65: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 66: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 67: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 68: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 69: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 70: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 71: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 72: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 73: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 74: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 75: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 76: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 77: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 78: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 79: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 80: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 81: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 82: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 83: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 84: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 85: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 86: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 87: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 88: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 89: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 90: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 91: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 92: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 93: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 94: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 95: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 96: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 97: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 98: Available</div>\n");

    html.push_str("<div class='slot-time'>Slot 99: Available</div>\n");

    html.push_str(r#"
                <button type="submit">Book</button>
            </form>
        </div>
    "#);
    html
}
