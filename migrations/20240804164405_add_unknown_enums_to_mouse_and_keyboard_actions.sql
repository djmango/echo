-- Add migration script here
-- Remove 'button4' and 'button5', add 'other' to mouse_action_enum
ALTER TYPE mouse_action_enum RENAME TO mouse_action_enum_old;
CREATE TYPE mouse_action_enum AS ENUM ('left', 'right', 'middle', 'other');
ALTER TABLE devents
    ALTER COLUMN mouse_action TYPE mouse_action_enum USING (
        CASE
            WHEN mouse_action::text IN ('button4', 'button5') THEN 'other'::mouse_action_enum
            ELSE mouse_action::text::mouse_action_enum
        END
    );
DROP TYPE mouse_action_enum_old;

-- Add 'unknown' to keyboard_action_key_enum
ALTER TYPE keyboard_action_key_enum ADD VALUE 'unknown';
