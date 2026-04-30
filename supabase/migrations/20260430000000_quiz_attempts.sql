-- Quiz attempt history per user
CREATE TABLE IF NOT EXISTS public.quiz_attempts (
    id          UUID        DEFAULT gen_random_uuid() PRIMARY KEY,
    user_id     UUID        NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    novel_name  TEXT        NOT NULL,
    score       INTEGER     NOT NULL CHECK (score >= 0),
    total       INTEGER     NOT NULL CHECK (total > 0),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS quiz_attempts_user_created
    ON public.quiz_attempts (user_id, created_at DESC);

ALTER TABLE public.quiz_attempts ENABLE ROW LEVEL SECURITY;

-- Each user sees and writes only their own rows
CREATE POLICY "select_own_attempts"
    ON public.quiz_attempts FOR SELECT
    USING (auth.uid() = user_id);

CREATE POLICY "insert_own_attempts"
    ON public.quiz_attempts FOR INSERT
    WITH CHECK (auth.uid() = user_id);
