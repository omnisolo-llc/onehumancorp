package analytics

import "log"

// RecordEvent records an analytics event for a given user.
func RecordEvent(eventName, userID string) {
	log.Printf("Event: %s, User: %s", eventName, userID)
}
