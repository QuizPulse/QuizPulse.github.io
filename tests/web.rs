//! Integration tests — run in a headless browser with:
//!   wasm-pack test --headless --firefox

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use quizpulse::{Quiz, novel_count, novel_name, novel_author};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn novel_count_is_two() {
    assert_eq!(novel_count(), 2);
}

#[wasm_bindgen_test]
fn novel_names_are_correct() {
    assert_eq!(novel_name(0), "Middlemarch");
    assert_eq!(novel_name(1), "Anna Karenina");
}

#[wasm_bindgen_test]
fn novel_authors_are_correct() {
    assert_eq!(novel_author(0), "George Eliot");
    assert_eq!(novel_author(1), "Leo Tolstoy");
}

#[wasm_bindgen_test]
fn quiz_creates_for_both_novels() {
    for i in 0..novel_count() {
        let quiz = Quiz::create(i, 42);
        assert_eq!(quiz.total(), 15);
        assert_eq!(quiz.score(), 0);
        assert!(!quiz.is_finished());
    }
}

#[wasm_bindgen_test]
fn question_text_and_options_are_nonempty() {
    let quiz = Quiz::create(0, 1);
    assert!(!quiz.question_text().is_empty());
    for i in 0..4 {
        assert!(!quiz.option_text(i).is_empty());
    }
}

#[wasm_bindgen_test]
fn answering_all_questions_finishes_quiz() {
    let mut quiz = Quiz::create(0, 0);
    for _ in 0..15 {
        quiz.submit_answer(0);
    }
    assert!(quiz.is_finished());
}

#[wasm_bindgen_test]
fn score_does_not_exceed_total() {
    let mut quiz = Quiz::create(1, 7);
    for _ in 0..15 {
        quiz.submit_answer(0);
    }
    assert!(quiz.score() <= 15);
}

#[wasm_bindgen_test]
fn explanation_is_available_after_answering() {
    let mut quiz = Quiz::create(0, 5);
    quiz.submit_answer(0);
    assert!(!quiz.explanation().is_empty());
}

#[wasm_bindgen_test]
fn correct_option_is_in_range_after_answering() {
    let mut quiz = Quiz::create(1, 99);
    quiz.submit_answer(2);
    assert!(quiz.correct_option() < 4);
}
