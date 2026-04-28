import { Quiz, novel_count, novel_name, novel_author } from "../pkg/quizpulse.js";

// Visual metadata per novel (index matches NOVELS order in Rust)
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

let quiz     = null;
let novelIdx = 0;

// ── Screens ──────────────────────────────────────────────────────────────────

function show(id) {
  document.querySelectorAll(".screen").forEach(s => s.classList.remove("active"));
  document.getElementById(id).classList.add("active");
}

// ── Novel selection ───────────────────────────────────────────────────────────

function buildBookGrid() {
  const grid = document.getElementById("book-grid");
  const count = novel_count();

  for (let i = 0; i < count; i++) {
    const meta   = NOVEL_META[i] || { subtitle: "", coverBg: "#2c1d0e", accentColor: "#c8a050", boxShadow: "#8b6914" };
    const name   = novel_name(i);
    const author = novel_author(i);

    const btn = document.createElement("button");
    btn.className = "book-btn";
    btn.dataset.novel = i;
    btn.innerHTML = `
      <div class="cover" style="
        background: ${meta.coverBg};
        border-bottom: 5px solid ${meta.accentColor};
        box-shadow: 6px 6px 0 ${meta.boxShadow};
      ">
        <div class="cover-ornament" style="color:${meta.accentColor}">— ✦ —</div>
        <h2>${name}</h2>
        <p class="cover-sub">${meta.subtitle}</p>
        <p class="cover-author">${author}</p>
        <div class="cover-ornament" style="color:${meta.accentColor}">— ✦ —</div>
      </div>
      <p class="book-meta">15 questions</p>
    `;
    btn.addEventListener("click", () => startQuiz(i));
    grid.appendChild(btn);
  }
}

// ── Quiz ──────────────────────────────────────────────────────────────────────

function startQuiz(idx) {
  novelIdx = idx;
  quiz = Quiz.create(idx, Date.now() & 0xFFFFFFFF);
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
  const explanation = quiz.explanation();

  document.querySelectorAll(".opt").forEach((btn, i) => {
    btn.disabled = true;
    if (i === rightOpt)                btn.classList.add("correct");
    else if (i === choice && !correct) btn.classList.add("wrong");
  });

  const verdict = document.getElementById("verdict");
  verdict.textContent = correct ? "✓ Correct!" : "✗ Incorrect";
  verdict.className   = "verdict " + (correct ? "ok" : "bad");
  document.getElementById("expl").textContent = explanation;

  document.getElementById("feedback").classList.remove("hidden");
  document.getElementById("btn-next").textContent =
    quiz.is_finished() ? "See Results →" : "Next Question →";
}

function showResults() {
  const score = quiz.score();
  const total = quiz.total();
  const pct   = Math.round((score / total) * 100);

  document.getElementById("results-novel").textContent  = novel_name(novelIdx);
  document.getElementById("score-pct").textContent      = pct + "%";
  document.getElementById("score-fraction").textContent = `${score} out of ${total} correct`;
  document.getElementById("score-msg").textContent      = GRADING.find(([t]) => pct >= t)[1];

  show("screen-results");
}

// ── Event listeners ───────────────────────────────────────────────────────────

document.querySelectorAll(".opt").forEach(btn => {
  btn.addEventListener("click", () => handleAnswer(+btn.dataset.idx));
});

document.getElementById("btn-next").addEventListener("click", () => {
  quiz.is_finished() ? showResults() : renderQuestion();
});

document.getElementById("btn-restart").addEventListener("click", () => {
  startQuiz(novelIdx);
});

document.getElementById("btn-choose").addEventListener("click", () => {
  show("screen-select");
});

// ── Init ──────────────────────────────────────────────────────────────────────

buildBookGrid();
