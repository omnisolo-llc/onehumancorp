package domain

import (
	"context"
	"errors"
	"fmt"
	"strconv"

	"github.com/redis/rueidis"
)

var (
	ErrQuotaExceeded = errors.New("quota exceeded")
	ErrTeamNotFound  = errors.New("team not found")
)

type QuotaStatus struct {
	TeamID    string `json:"team_id"`
	Limit     int64  `json:"limit"`
	Used      int64  `json:"used"`
	Remaining int64  `json:"remaining"`
}

type QuotaService struct {
	redis rueidis.Client
}

func NewQuotaService(redis rueidis.Client) *QuotaService {
	return &QuotaService{
		redis: redis,
	}
}

func (s *QuotaService) CheckQuota(ctx context.Context, teamID string) (QuotaStatus, error) {
	limitKey := fmt.Sprintf("quota:limit:%s", teamID)
	usageKey := fmt.Sprintf("quota:usage:%s", teamID)

	cmd := s.redis.B().Mget().Key(limitKey, usageKey).Build()
	res := s.redis.Do(ctx, cmd)
	if err := res.Error(); err != nil {
		return QuotaStatus{}, err
	}
	vals, _ := res.AsStrSlice()
	if len(vals) < 2 {
		return QuotaStatus{}, ErrTeamNotFound
	}

	limitStr := vals[0]
	if limitStr == "" {
		return QuotaStatus{}, ErrTeamNotFound
	}

	limit, err := strconv.ParseInt(limitStr, 10, 64)
	if err != nil {
		return QuotaStatus{}, err
	}

	usageStr := vals[1]
	used := int64(0)
	if usageStr != "" {
		used, err = strconv.ParseInt(usageStr, 10, 64)
		if err != nil {
			return QuotaStatus{}, err
		}
	}

	return QuotaStatus{
		TeamID:    teamID,
		Limit:     limit,
		Used:      used,
		Remaining: limit - used,
	}, nil
}

func (s *QuotaService) IncrementUsage(ctx context.Context, teamID string) error {
	limitKey := fmt.Sprintf("quota:limit:%s", teamID)
	usageKey := fmt.Sprintf("quota:usage:%s", teamID)

	// In a real system, we might use a Lua script for atomicity
	cmd := s.redis.B().Get().Key(limitKey).Build()
	res := s.redis.Do(ctx, cmd)
	if err := res.Error(); err != nil {
		if rueidis.IsRedisNil(err) {
			return ErrTeamNotFound
		}
		return err
	}

	limitStr, err := res.ToString()
	if err != nil {
		return err
	}
	limit, err := strconv.ParseInt(limitStr, 10, 64)
	if err != nil {
		return err
	}

	cmdIncr := s.redis.B().Incr().Key(usageKey).Build()
	resIncr := s.redis.Do(ctx, cmdIncr)
	if err := resIncr.Error(); err != nil {
		return err
	}

	used, err := resIncr.AsInt64()
	if err != nil {
		return err
	}

	if used > limit {
		// Rollback increment if exceeded
		cmdDecr := s.redis.B().Decr().Key(usageKey).Build()
		_ = s.redis.Do(ctx, cmdDecr)
		return ErrQuotaExceeded
	}

	return nil
}

// For testing purposes
func (s *QuotaService) SetLimit(ctx context.Context, teamID string, limit int64) error {
	limitKey := fmt.Sprintf("quota:limit:%s", teamID)
	cmd := s.redis.B().Set().Key(limitKey).Value(strconv.FormatInt(limit, 10)).Build()
	return s.redis.Do(ctx, cmd).Error()
}
