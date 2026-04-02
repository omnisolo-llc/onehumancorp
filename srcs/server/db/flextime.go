package db

import (
	"database/sql/driver"
	"fmt"
	"time"
)

// FlexTime handles scanning TIMESTAMPTZ columns which may return as
// time.Time from PostgreSQL or string/[]byte from SQLite.
type FlexTime struct {
	time.Time
}

// Scan implements sql.Scanner for FlexTime.
func (ft *FlexTime) Scan(value interface{}) error {
	if value == nil {
		ft.Time = time.Time{}
		return nil
	}

	switch v := value.(type) {
	case time.Time:
		ft.Time = v
		return nil
	case []byte:
		return ft.parseString(string(v))
	case string:
		return ft.parseString(v)
	default:
		return fmt.Errorf("FlexTime: cannot scan type %T into FlexTime", value)
	}
}

func (ft *FlexTime) parseString(s string) error {
	// Try standard RFC3339 first (PostgreSQL default string format)
	t, err := time.Parse(time.RFC3339, s)
	if err == nil {
		ft.Time = t
		return nil
	}

	// Try SQLite default DATETIME string format: "2006-01-02 15:04:05"
	t, err = time.Parse("2006-01-02 15:04:05", s)
	if err == nil {
		ft.Time = t
		return nil
	}

	// Try SQLite format with timezone offset
	t, err = time.Parse("2006-01-02 15:04:05Z07:00", s)
	if err == nil {
		ft.Time = t
		return nil
	}

	// Try SQLite format with fractional seconds
	t, err = time.Parse("2006-01-02 15:04:05.999999999", s)
	if err == nil {
		ft.Time = t
		return nil
	}

	return fmt.Errorf("FlexTime: cannot parse string %q as time.Time", s)
}

// Value implements driver.Valuer for FlexTime.
func (ft FlexTime) Value() (driver.Value, error) {
	if ft.Time.IsZero() {
		return nil, nil
	}
	return ft.Time, nil
}
