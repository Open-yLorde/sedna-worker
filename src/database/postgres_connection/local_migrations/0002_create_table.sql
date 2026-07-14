CREATE TABLE IF NOT EXISTS heartbeat (
    id SERIAL NOT NULL PRIMARY KEY,
    endpoint VARCHAR(32) NOT NULL,
    delay INT DEFAULT -1,
    timeout INT DEFAULT -1,
    success BOOLEAN DEFAULT false,
    status_code INT DEFAULT -1,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('brt'::text, now())
);

CREATE TABLE IF NOT EXISTS latency (
    id SERIAL NOT NULL PRIMARY KEY,
    delay INT DEFAULT -1,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('brt'::text, now())
);

CREATE TABLE IF NOT EXISTS status (
    id SERIAL NOT NULL PRIMARY KEY,
    uptime INT DEFAULT -1,
    latency INT DEFAULT -1,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('brt'::text, now())
);

CREATE TABLE IF NOT EXISTS gb_commits (
    id SERIAL NOT NULL PRIMARY KEY,
    sha TEXT NOT NULL UNIQUE
);