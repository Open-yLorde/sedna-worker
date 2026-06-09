CREATE TABLE ping (
    id SERIAL NOT NULL PRIMARY KEY,
    endpoint VARCHAR(32) NOT NULL,
    delay INT DEFAULT -1,
    timeout INT DEFAULT -1,
    success BOOLEAN DEFAULT false,
    status_code INT DEFAULT -1,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('brt'::text, now())
);

CREATE TABLE latency (
    id SERIAL NOT NULL PRIMARY KEY,
    delay INT DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT timezone('brt'::text, now())
);