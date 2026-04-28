import { Quiz } from "../pkg/quizpulse.js";

let quiz = null;

function show(id) {
  document.querySelectorAll(".screen").forEach(s => s.classList.remove("active"));
  document.getElementById(id).classList.add("active");
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

  document.getElementById("score-pct").textContent      = pct + "%";
  document.getElementById("score-fraction").textContent = `${score} out of ${total} correct`;

  const grading = [
    [100, "\"Her full nature spent itself in channels which had no great name on the earth.\" A perfect score!"],
    [80,  "Excellent — Dorothea herself would approve of such thorough study."],
    [60,  "A solid effort. A few more evenings at Lowick Manor would sharpen the rest."],
    [40,  "Not bad, but the novel rewards closer attention. Time to return to George Eliot!"],
    [0,   "Time to open Middlemarch and discover one of the greatest novels ever written."],
  ];
  document.getElementById("score-msg").textContent =
    grading.find(([t]) => pct >= t)[1];

  show("screen-results");
}

document.getElementById("btn-start").addEventListener("click", () => {
  quiz = Quiz.create(Date.now() & 0xFFFFFFFF);
  renderQuestion();
  show("screen-question");
});

document.querySelectorAll(".opt").forEach(btn => {
  btn.addEventListener("click", () => handleAnswer(+btn.dataset.idx));
});

document.getElementById("btn-next").addEventListener("click", () => {
  quiz.is_finished() ? showResults() : renderQuestion();
});

document.getElementById("btn-restart").addEventListener("click", () => {
  show("screen-start");
});
