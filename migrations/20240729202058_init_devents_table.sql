-- Add migration script here
CREATE TYPE mouse_action_enum AS ENUM ('left', 'right', 'middle', 'button4', 'button5');
CREATE TYPE keyboard_action_key_enum AS ENUM ('caps_lock', 'shift', 'command', 'option', 'control', 'fn', 'alt', 'meta', 'f1', 'f2', 'f3', 'f4', 'f5', 'f6', 'f7', 'f8', 'f9', 'f10', 'f11', 'f12', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'arrow_up', 'arrow_down', 'arrow_left', 'arrow_right', 'home', 'end', 'page_up', 'page_down', 'enter', 'escape', 'tab', 'space', 'backspace', 'insert', 'delete', 'num_lock', 'scroll_lock', 'pause', 'print_screen', 'grave', 'minus', 'equals', 'bracket_left', 'bracket_right', 'backslash', 'semicolon', 'quote', 'comma', 'period', 'slash');
CREATE TYPE keyboard_action AS (
    key keyboard_action_key_enum,
    duration INTEGER
);
CREATE TYPE scroll_action AS (
    x INTEGER,
    y INTEGER,
    duration INTEGER
);

CREATE TABLE devents (
    id UUID PRIMARY KEY,
    session_id UUID NOT NULL,
    recording_id UUID,
    mouse_action mouse_action_enum,
    keyboard_action keyboard_action,
    scroll_action scroll_action,
    mouse_x INTEGER NOT NULL,
    mouse_y INTEGER NOT NULL,
    event_timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE
);
