-- Remove credentials and browser-supplied authority left by legacy onboarding clients.
-- New writes are allowlisted in the application; this migration cleans existing rows.
UPDATE onboarding_state
SET state_json = CASE
    WHEN jsonb_typeof(state_json -> 'wizardState') = 'object' THEN
        jsonb_set(
            state_json
                - 'adminPassword'
                - 'admin_password'
                - 'adminEmail'
                - 'admin_email'
                - 'adminName'
                - 'admin_name'
                - 'userPassword'
                - 'user_password'
                - 'userEmail'
                - 'user_email'
                - 'userName'
                - 'user_name'
                - 'tenant_id'
                - 'tenantId'
                - 'user_id'
                - 'userId'
                - 'authorization'
                - 'auth_token',
            '{wizardState}',
            (state_json -> 'wizardState')
                - 'adminPassword'
                - 'admin_password'
                - 'adminEmail'
                - 'admin_email'
                - 'adminName'
                - 'admin_name'
                - 'userPassword'
                - 'user_password'
                - 'userEmail'
                - 'user_email'
                - 'userName'
                - 'user_name'
                - 'tenant_id'
                - 'tenantId'
                - 'user_id'
                - 'userId'
                - 'authorization'
                - 'auth_token',
            false
        )
    ELSE
        state_json
            - 'adminPassword'
            - 'admin_password'
            - 'adminEmail'
            - 'admin_email'
            - 'adminName'
            - 'admin_name'
            - 'userPassword'
            - 'user_password'
            - 'userEmail'
            - 'user_email'
            - 'userName'
            - 'user_name'
            - 'tenant_id'
            - 'tenantId'
            - 'user_id'
            - 'userId'
            - 'authorization'
            - 'auth_token'
END
WHERE state_json ?| ARRAY[
    'adminPassword', 'admin_password', 'adminEmail', 'admin_email',
    'adminName', 'admin_name', 'userPassword', 'user_password',
    'userEmail', 'user_email', 'userName', 'user_name', 'tenant_id', 'tenantId', 'user_id',
    'userId', 'authorization', 'auth_token'
]
OR (
    jsonb_typeof(state_json -> 'wizardState') = 'object'
    AND (state_json -> 'wizardState') ?| ARRAY[
        'adminPassword', 'admin_password', 'adminEmail', 'admin_email',
        'adminName', 'admin_name', 'userPassword', 'user_password',
        'userEmail', 'user_email', 'userName', 'user_name', 'tenant_id', 'tenantId', 'user_id',
        'userId', 'authorization', 'auth_token'
    ]
);
