import { Quiz, novel_count, novel_name, novel_author, questions_per_quiz } from "../pkg/quizpulse.js";
import { createClient } from "@supabase/supabase-js";

// ── Supabase ──────────────────────────────────────────────────────────────────

const supabase = createClient(
  "https://qrtnxbcnohfnihnkhcnc.supabase.co",
  "sb_publishable_LIMiuRKnLYYlQmJunzUr1Q_hoJfkW-Y"
);

let currentUser = null;

// ── Constants ─────────────────────────────────────────────────────────────────

const NOVEL_META = [
  { subtitle: "A Study of Provincial Life", coverBg: "#5c3317", accentColor: "#c8a050", boxShadow: "#8b6914" },
  { subtitle: "A Novel",                     coverBg: "#1a2e4a", accentColor: "#c8a050", boxShadow: "#4a6a8a" },
];

const GRADING = [
  [100, "A perfect score. You know this novel inside out."],
  [80,  "Excellent — a thoroughly well-read performance."],
  [60,  "A solid effort, though a few pages may deserve a second reading."],
  [40,  "Not bad — the novel rewards closer attention on the next round."],
  [0,   "Time to open the book and discover what you missed!"],
];

// ── Quiz state ────────────────────────────────────────────────────────────────

let quiz     = null;
let novelIdx = 0;

// ── Screen management ─────────────────────────────────────────────────────────

function show(id) {
  document.querySelectorAll(".screen").forEach(s => s.classList.remove("active"));
  document.getElementById(id).classList.add("active");
}

// ── Auth bar ──────────────────────────────────────────────────────────────────

function updateAuthBar() {
  const bar = document.getElementById("auth-bar");
  if (currentUser) {
    bar.innerHTML = `
      <span class="user-email">${currentUser.email}</span>
      <span class="auth-sep">·</span>
      <button class="auth-link" id="btn-history">History</button>
      <span class="auth-sep">·</span>
      <button class="auth-link" id="btn-signout">Sign out</button>
    `;
    document.getElementById("btn-history").addEventListener("click", loadHistory);
    document.getElementById("btn-signout").addEventListener("click", handleSignOut);
  } else {
    bar.innerHTML = `<button class="auth-link auth-link-cta" id="btn-open-auth">Sign in</button>`;
    document.getElementById("btn-open-auth").addEventListener("click", () => openAuthModal("signin"));
  }
}

// ── Auth modal ────────────────────────────────────────────────────────────────

function openAuthModal(tab = "signin") {
  document.getElementById("auth-modal").classList.remove("hidden");
  document.getElementById("auth-email").value    = "";
  document.getElementById("auth-password").value = "";
  switchAuthTab(tab);
}

function closeAuthModal() {
  document.getElementById("auth-modal").classList.add("hidden");
  document.getElementById("auth-error").classList.add("hidden");
  document.getElementById("auth-note").classList.add("hidden");
}

function switchAuthTab(tab) {
  document.querySelectorAll(".auth-tab").forEach(t =>
    t.classList.toggle("active", t.dataset.tab === tab)
  );
  document.getElementById("auth-modal").dataset.mode = tab;
  document.getElementById("auth-submit").textContent  = tab === "signin" ? "Sign In" : "Create Account";
  document.getElementById("auth-error").classList.add("hidden");
  document.getElementById("auth-note").classList.add("hidden");
}

async function handleAuthSubmit(e) {
  e.preventDefault();
  const mode     = document.getElementById("auth-modal").dataset.mode;
  const email    = document.getElementById("auth-email").value.trim();
  const password = document.getElementById("auth-password").value;
  const errEl    = document.getElementById("auth-error");
  const noteEl   = document.getElementById("auth-note");
  const btn      = document.getElementById("auth-submit");

  errEl.classList.add("hidden");
  noteEl.classList.add("hidden");
  btn.disabled    = true;
  btn.textContent = "…";

  if (mode === "signin") {
    const { error } = await supabase.auth.signInWithPassword({ email, password });
    if (error) {
      errEl.textContent = error.message;
      errEl.classList.remove("hidden");
    } else {
      closeAuthModal();
    }
  } else {
    const { error } = await supabase.auth.signUp({ email, password });
    if (error) {
      errEl.textContent = error.message;
      errEl.classList.remove("hidden");
    } else {
      noteEl.textContent = "Account created! Check your email to confirm, then sign in.";
      noteEl.classList.remove("hidden");
    }
  }

  btn.disabled    = false;
  btn.textContent = mode === "signin" ? "Sign In" : "Create Account";
}

async function handleSignOut() {
  await supabase.auth.signOut();
}

// ── History ───────────────────────────────────────────────────────────────────

async function saveAttempt(novelName, score, total) {
  if (!currentUser) return;
  const { error } = await supabase
    .from("quiz_attempts")
    .insert({ user_id: currentUser.id, novel_name: novelName, score, total });
  if (error) console.error("Save failed:", error.message);
}

async function loadHistory() {
  show("screen-history");
  const listEl = document.getElementById("history-list");
  listEl.innerHTML = `<p class="history-empty">Loading…</p>`;

  const { data, error } = await supabase
    .from("quiz_attempts")
    .select("novel_name, score, total, created_at")
    .order("created_at", { ascending: false })
    .limit(50);

  if (error) {
    listEl.innerHTML = `<p class="history-empty">Could not load history: ${error.message}</p>`;
    return;
  }
  if (!data.length) {
    listEl.innerHTML = `<p class="history-empty">No attempts yet — complete a quiz and your score will appear here.</p>`;
    return;
  }

  listEl.innerHTML = data.map(row => {
    const pct   = Math.round((row.score / row.total) * 100);
    const grade = pct >= 80 ? "high" : pct >= 60 ? "mid" : "low";
    const d     = new Date(row.created_at);
    const date  = d.toLocaleDateString("en-GB", { day: "numeric", month: "short", year: "numeric" });
    const time  = d.toLocaleTimeString("en-GB", { hour: "2-digit", minute: "2-digit" });
    return `
      <div class="history-item">
        <div>
          <div class="history-novel">${row.novel_name}</div>
          <div class="history-date">${date} · ${time}</div>
        </div>
        <div class="history-right">
          <div class="history-score">${row.score} / ${row.total}</div>
          <span class="pct-badge ${grade}">${pct}%</span>
        </div>
      </div>`;
  }).join("");
}

// ── Novel selection ───────────────────────────────────────────────────────────

function buildBookGrid() {
  const grid  = document.getElementById("book-grid");
  const count = novel_count();
  for (let i = 0; i < count; i++) {
    const meta   = NOVEL_META[i] ?? { subtitle: "", coverBg: "#2c1d0e", accentColor: "#c8a050", boxShadow: "#8b6914" };
    const btn    = document.createElement("button");
    btn.className       = "book-btn";
    btn.dataset.novel   = i;
    btn.innerHTML = `
      <div class="cover" style="background:${meta.coverBg};border-bottom:5px solid ${meta.accentColor};box-shadow:6px 6px 0 ${meta.boxShadow}">
        <div class="cover-ornament" style="color:${meta.accentColor}">— ✦ —</div>
        <h2>${novel_name(i)}</h2>
        <p class="cover-sub">${meta.subtitle}</p>
        <p class="cover-author">${novel_author(i)}</p>
        <div class="cover-ornament" style="color:${meta.accentColor}">— ✦ —</div>
      </div>
      <p class="book-meta">${questions_per_quiz()} questions per play</p>`;
    btn.addEventListener("click", () => startQuiz(i));
    grid.appendChild(btn);
  }
}

// ── Quiz ──────────────────────────────────────────────────────────────────────

function startQuiz(idx) {
  novelIdx = idx;
  quiz     = Quiz.create(idx, Date.now() & 0xFFFFFFFF);
  renderQuestion();
  show("screen-question");
}

function renderQuestion() {
  const idx   = quiz.current_index();
  const total = quiz.total();
  document.getElementById("q-counter").textContent     = `Question ${idx + 1} of ${total}`;
  document.getElementById("score-label").textContent   = `Score: ${quiz.score()}`;
  document.getElementById("progress-fill").style.width = `${(idx / total) * 100}%`;
  document.getElementById("question-text").textContent = quiz.question_text();
  document.querySelectorAll(".opt").forEach((btn, i) => {
    btn.textContent = quiz.option_text(i);
    btn.className   = "opt";
    btn.disabled    = false;
  });
  document.getElementById("feedback").classList.add("hidden");
}

function handleAnswer(choice) {
  const correct     = quiz.submit_answer(choice);
  const rightOpt    = quiz.correct_option();
  document.querySelectorAll(".opt").forEach((btn, i) => {
    btn.disabled = true;
    if (i === rightOpt)                btn.classList.add("correct");
    else if (i === choice && !correct) btn.classList.add("wrong");
  });
  const verdict = document.getElementById("verdict");
  verdict.textContent = correct ? "✓ Correct!" : "✗ Incorrect";
  verdict.className   = "verdict " + (correct ? "ok" : "bad");
  document.getElementById("expl").textContent = quiz.explanation();
  document.getElementById("feedback").classList.remove("hidden");
  document.getElementById("btn-next").textContent =
    quiz.is_finished() ? "See Results →" : "Next Question →";
}

async function showResults() {
  const score = quiz.score();
  const total = quiz.total();
  const pct   = Math.round((score / total) * 100);
  const name  = novel_name(novelIdx);

  document.getElementById("results-novel").textContent  = name;
  document.getElementById("score-pct").textContent      = pct + "%";
  document.getElementById("score-fraction").textContent = `${score} out of ${total} correct`;
  document.getElementById("score-msg").textContent      = GRADING.find(([t]) => pct >= t)[1];

  const statusEl = document.getElementById("save-status");
  if (currentUser) {
    statusEl.textContent = "Saving…";
    statusEl.className   = "save-status";
    await saveAttempt(name, score, total);
    statusEl.textContent = "✓ Saved to your history";
    statusEl.className   = "save-status saved";
  } else {
    statusEl.innerHTML = `<button class="auth-link-inline" id="btn-save-signin">Sign in</button> to save your score`;
    statusEl.className = "save-status";
    document.getElementById("btn-save-signin")
      .addEventListener("click", () => openAuthModal("signin"));
  }

  show("screen-results");
}

// ── Event listeners ───────────────────────────────────────────────────────────

document.querySelectorAll(".opt").forEach(btn =>
  btn.addEventListener("click", () => handleAnswer(+btn.dataset.idx))
);

document.getElementById("btn-next").addEventListener("click", () => {
  quiz.is_finished() ? showResults() : renderQuestion();
});

document.getElementById("btn-restart").addEventListener("click", () => startQuiz(novelIdx));
document.getElementById("btn-choose").addEventListener("click",  () => show("screen-select"));
document.getElementById("btn-back-select").addEventListener("click", () => show("screen-select"));

document.getElementById("auth-close").addEventListener("click", closeAuthModal);
document.getElementById("auth-modal").addEventListener("click", e => {
  if (e.target === e.currentTarget) closeAuthModal();
});
document.querySelectorAll(".auth-tab").forEach(tab =>
  tab.addEventListener("click", () => switchAuthTab(tab.dataset.tab))
);
document.getElementById("auth-form").addEventListener("submit", handleAuthSubmit);

// ── Init ──────────────────────────────────────────────────────────────────────

async function init() {
  const { data: { user } } = await supabase.auth.getUser();
  currentUser = user;
  updateAuthBar();

  supabase.auth.onAuthStateChange((_event, session) => {
    currentUser = session?.user ?? null;
    updateAuthBar();
  });

  buildBookGrid();
}

init().catch(console.error);
