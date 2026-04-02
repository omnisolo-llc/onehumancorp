<<<<<<< SEARCH
	res, err := m.db.Exec(ctx, query, agentID, taskID)
	if err != nil {
		return false, fmt.Errorf("failed to claim task in db: %w", err)
	}

	rowsAffected, err := res.RowsAffected()
	if err != nil {
		return false, err
	}

	return rowsAffected > 0, nil
}
=======
	rowsAffected, err := m.db.Exec(ctx, query, agentID, taskID)
	if err != nil {
		return false, fmt.Errorf("failed to claim task in db: %w", err)
	}

	return rowsAffected > 0, nil
}
>>>>>>> REPLACE
