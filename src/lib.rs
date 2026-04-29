mod utils;
use wasm_bindgen::prelude::*;

const QUESTIONS_PER_QUIZ: usize = 10;

struct Question {
    text: &'static str,
    options: [&'static str; 4],
    correct: usize,
    explanation: &'static str,
}

struct Novel {
    name: &'static str,
    author: &'static str,
    questions: &'static [Question],
}

// ── Middlemarch (20 questions) ────────────────────────────────────────────────

static MIDDLEMARCH: &[Question] = &[
    Question {
        text: "Who wrote Middlemarch?",
        options: ["Jane Austen", "George Eliot", "Charlotte Brontë", "Thomas Hardy"],
        correct: 1,
        explanation: "George Eliot is the pen name of Mary Ann Evans, who published Middlemarch as eight installments in 1871–72.",
    },
    Question {
        text: "What is the subtitle of Middlemarch?",
        options: [
            "A Story of English Life",
            "A Portrait of Society",
            "A Study of Provincial Life",
            "A Tale of Two Sisters",
        ],
        correct: 2,
        explanation: "The full title is Middlemarch: A Study of Provincial Life.",
    },
    Question {
        text: "Who does Dorothea Brooke marry first?",
        options: ["Will Ladislaw", "Tertius Lydgate", "Edward Casaubon", "Nicholas Bulstrode"],
        correct: 2,
        explanation: "Dorothea's first marriage to the elderly scholar Edward Casaubon is a central storyline of the novel.",
    },
    Question {
        text: "What is Edward Casaubon's great unfinished intellectual project?",
        options: [
            "A History of Rome",
            "The Key to All Mythologies",
            "A Commentary on the Bible",
            "A Dictionary of Ancient Languages",
        ],
        correct: 1,
        explanation: "Casaubon devotes his life to writing The Key to All Mythologies, a grand synthesis he never completes.",
    },
    Question {
        text: "Who does Dorothea ultimately marry at the end of the novel?",
        options: ["Fred Vincy", "Will Ladislaw", "James Chettam", "Tertius Lydgate"],
        correct: 1,
        explanation: "After Casaubon's death, Dorothea marries the idealistic Will Ladislaw, forfeiting her inheritance to do so.",
    },
    Question {
        text: "What is Tertius Lydgate's profession?",
        options: ["Lawyer", "Clergyman", "Banker", "Doctor"],
        correct: 3,
        explanation: "Lydgate is an ambitious young doctor who comes to Middlemarch hoping to pursue medical reform and research.",
    },
    Question {
        text: "Who does Lydgate marry?",
        options: ["Dorothea Brooke", "Mary Garth", "Rosamond Vincy", "Celia Brooke"],
        correct: 2,
        explanation: "Lydgate marries the beautiful but self-centered Rosamond Vincy, whose expensive tastes contribute to his downfall.",
    },
    Question {
        text: "What condition does Casaubon's will impose on Dorothea?",
        options: [
            "She must complete his manuscript",
            "She must never remarry",
            "She must remain in Middlemarch",
            "She loses her inheritance if she marries Will Ladislaw",
        ],
        correct: 3,
        explanation: "Casaubon added a codicil cutting Dorothea off if she married Ladislaw, whom he suspected she loved.",
    },
    Question {
        text: "Who is Fred Vincy in love with throughout the novel?",
        options: ["Harriet Bulstrode", "Mary Garth", "Dorothea Brooke", "Celia Brooke"],
        correct: 1,
        explanation: "Fred Vincy, Rosamond's brother, is devoted to the practical and plain-spoken Mary Garth.",
    },
    Question {
        text: "Who is Nicholas Bulstrode?",
        options: [
            "The local magistrate",
            "Dorothea's uncle",
            "A retired army officer",
            "A wealthy banker with a hidden criminal past",
        ],
        correct: 3,
        explanation: "Bulstrode is a pious banker whose past crimes as a receiver of stolen goods are exposed by the blackmailer Raffles.",
    },
    Question {
        text: "What does the famous final paragraph of Middlemarch celebrate?",
        options: [
            "The triumph of love over circumstance",
            "The importance of education for women",
            "The inevitability of social progress",
            "The power of unhistoric acts of goodness",
        ],
        correct: 3,
        explanation: "The Finale ends by celebrating \"unhistoric acts\" — small, private deeds of good that still make the world better.",
    },
    Question {
        text: "Approximately when is Middlemarch set?",
        options: ["1815–1820", "1829–1832", "1850–1855", "1870–1875"],
        correct: 1,
        explanation: "The novel is set during the era of the First Reform Act and other social changes in England, roughly 1829–1832.",
    },
    Question {
        text: "What is Caleb Garth's occupation?",
        options: ["Schoolteacher", "Clergyman", "Land agent and surveyor", "Physician"],
        correct: 2,
        explanation: "Caleb Garth is an honest, hardworking land agent who serves as a quiet moral touchstone in the novel.",
    },
    Question {
        text: "In what year was Middlemarch first published as a complete book?",
        options: ["1865", "1848", "1880", "1872"],
        correct: 3,
        explanation: "Middlemarch was serialized in 1871–72, then issued as a four-volume book in December 1872.",
    },
    Question {
        text: "What does Dorothea sacrifice by marrying Will Ladislaw?",
        options: [
            "Her friendship with Celia",
            "Her home at Lowick Manor",
            "Her social standing in Middlemarch",
            "Her inheritance from Casaubon",
        ],
        correct: 3,
        explanation: "By marrying Ladislaw, Dorothea triggers Casaubon's punitive codicil and loses the bulk of his fortune.",
    },
    Question {
        text: "What is the name of Dorothea's younger sister?",
        options: ["Clara", "Celia", "Catherine", "Caroline"],
        correct: 1,
        explanation: "Celia Brooke is Dorothea's practical, conventional younger sister. She later marries Sir James Chettam.",
    },
    Question {
        text: "What political venture does Mr. Brooke, Dorothea's uncle, attempt?",
        options: [
            "Starting a reform newspaper",
            "Running for Parliament",
            "Becoming a local magistrate",
            "Campaigning for prison reform",
        ],
        correct: 1,
        explanation: "Mr. Brooke stands as a parliamentary candidate but is humiliated at a public meeting when the crowd mocks him back with his own words.",
    },
    Question {
        text: "Where do Dorothea and Casaubon spend their honeymoon?",
        options: ["Paris", "Florence", "Rome", "Venice"],
        correct: 2,
        explanation: "They honeymoon in Rome, where Dorothea experiences a profound emotional crisis surrounded by the weight of classical history.",
    },
    Question {
        text: "What is Fred Vincy's great failing at the start of the novel?",
        options: [
            "Gambling debts and idleness",
            "Heavy drinking",
            "A secret engagement",
            "Embezzling from his father",
        ],
        correct: 0,
        explanation: "Fred is idle and runs up debts based on an expected inheritance that never materialises, nearly ruining the Garth family who co-sign for him.",
    },
    Question {
        text: "What is distinctive about Will Ladislaw's background?",
        options: [
            "He is a French émigré with no English connections",
            "He served in the army under Wellington",
            "His grandmother was disinherited by Casaubon's family for marrying a Polish Jew",
            "He is Casaubon's illegitimate son",
        ],
        correct: 2,
        explanation: "Ladislaw's grandmother was disinherited when she married a Polish Jewish man — a secret Casaubon keeps, and which makes Ladislaw the moral heir to what Dorothea eventually sacrifices.",
    },
];

// ── Anna Karenina (20 questions) ──────────────────────────────────────────────

static ANNA_KARENINA: &[Question] = &[
    Question {
        text: "Who wrote Anna Karenina?",
        options: ["Fyodor Dostoevsky", "Leo Tolstoy", "Ivan Turgenev", "Anton Chekhov"],
        correct: 1,
        explanation: "Leo Tolstoy published Anna Karenina as a serial from 1875 to 1877; the complete book appeared in 1878.",
    },
    Question {
        text: "According to the novel's famous opening line, what is true of all happy families?",
        options: [
            "They are all alike",
            "They each find happiness differently",
            "They are all deceptive",
            "They are short-lived",
        ],
        correct: 0,
        explanation: "'All happy families are alike; each unhappy family is unhappy in its own way.' — the novel's celebrated first sentence.",
    },
    Question {
        text: "Who is Anna Karenina's husband?",
        options: ["Stepan Oblonsky", "Count Vronsky", "Alexei Karenin", "Konstantin Levin"],
        correct: 2,
        explanation: "Alexei Alexandrovich Karenin is a senior government official — formal, rigidly correct, and unable to understand his wife's passions.",
    },
    Question {
        text: "Who does Anna begin a passionate affair with?",
        options: ["Konstantin Levin", "Count Vronsky", "Stepan Oblonsky", "Nikolai Shcherbatsky"],
        correct: 1,
        explanation: "Anna falls in love with the dashing cavalry officer Count Alexei Vronsky after they meet on a train journey.",
    },
    Question {
        text: "How does Anna Karenina die?",
        options: [
            "She drowns in the Neva River",
            "She dies of fever after childbirth",
            "She is killed in a carriage accident",
            "She throws herself under a train",
        ],
        correct: 3,
        explanation: "Anna ends her life by throwing herself beneath a freight train — a death foreshadowed by a railwayman's fatal accident she witnesses early in the novel.",
    },
    Question {
        text: "Who does Kitty Shcherbatskaya eventually marry?",
        options: ["Konstantin Levin", "Count Vronsky", "Stepan Oblonsky", "Sergei Koznyshev"],
        correct: 0,
        explanation: "After initially refusing Levin and being snubbed by Vronsky, Kitty comes to love Levin and they marry — their happiness contrasts with Anna's tragedy.",
    },
    Question {
        text: "What is the nickname 'Stiva' short for?",
        options: ["Alexei", "Nikolai", "Stepan", "Ivan"],
        correct: 2,
        explanation: "Prince Stepan Arkadyevich Oblonsky — Anna's charming, irresponsible brother — is universally known as Stiva.",
    },
    Question {
        text: "In what year was Anna Karenina first published as a complete book?",
        options: ["1865", "1878", "1886", "1870"],
        correct: 1,
        explanation: "After serial publication from 1875 to 1877, the complete novel appeared as a book in 1878.",
    },
    Question {
        text: "What is the name of Anna's son by Karenin?",
        options: ["Seryozha", "Alexei", "Nikolai", "Dmitri"],
        correct: 0,
        explanation: "Sergei — nicknamed Seryozha — is Anna's beloved son; her forced separation from him after the affair becomes public is one of the novel's most heartbreaking elements.",
    },
    Question {
        text: "What is Levin's central preoccupation throughout the novel?",
        options: [
            "Advancing his military career",
            "Writing a philosophical book",
            "Farming his estate and searching for life's meaning",
            "Reforming the Russian legal system",
        ],
        correct: 2,
        explanation: "Levin is a landowner passionately absorbed in agricultural work and, more broadly, in finding genuine moral truth and meaning in life.",
    },
    Question {
        text: "What mishap at a horse race does Anna watch in public anguish?",
        options: [
            "Vronsky falls and breaks his collarbone",
            "Vronsky's mare Frou-Frou collapses and must be destroyed",
            "Vronsky is disqualified for foul riding",
            "Vronsky's horse bolts and injures a bystander",
        ],
        correct: 1,
        explanation: "Frou-Frou collapses after Vronsky rides her badly; Anna's visible distress betrays her feelings to Karenin — a turning point in the novel.",
    },
    Question {
        text: "Who is Dolly (Darya Alexandrovna)?",
        options: [
            "Stiva's wife and Kitty's older sister",
            "Anna's closest friend in St. Petersburg",
            "Vronsky's sister",
            "Levin's sister",
        ],
        correct: 0,
        explanation: "Dolly is Stepan Oblonsky's long-suffering wife and Kitty's older sister; the novel opens with Stiva's infidelity to her.",
    },
    Question {
        text: "What spiritual experience does Levin have near the novel's end?",
        options: [
            "He converts to the Orthodox priesthood",
            "He emigrates to escape Russian society",
            "He finds simple faith through a peasant's chance remark",
            "He rejects all religion",
        ],
        correct: 2,
        explanation: "A peasant's offhand remark about 'living for God, not for your belly' triggers a quiet but profound moral awakening in Levin.",
    },
    Question {
        text: "Where do Anna and Vronsky go after leaving Russian society?",
        options: ["Paris", "Italy", "England", "Germany"],
        correct: 1,
        explanation: "After the scandal breaks, Anna and Vronsky travel to Italy — Florence and Venice — before restlessness drives them back to Russia.",
    },
    Question {
        text: "What does Karenin refuse to grant Anna when her affair becomes known?",
        options: [
            "A divorce",
            "Access to her son Seryozha",
            "Financial support",
            "The right to remain in St. Petersburg",
        ],
        correct: 0,
        explanation: "Karenin refuses to divorce Anna on religious and social grounds, leaving her trapped — legally his wife but socially an outcast.",
    },
    Question {
        text: "What is the name of Anna and Vronsky's daughter?",
        options: ["Natasha", "Annie", "Sonya", "Masha"],
        correct: 1,
        explanation: "Anna and Vronsky have a daughter called Annie; her birth nearly kills Anna and prompts Karenin's unexpected moment of forgiveness.",
    },
    Question {
        text: "Where does Anna first encounter Vronsky?",
        options: [
            "At a ball in St. Petersburg",
            "At a railway station in Moscow",
            "At Stiva's house in Moscow",
            "At a country estate in summer",
        ],
        correct: 1,
        explanation: "Anna and Vronsky first meet at a Moscow railway station when she arrives to help reconcile Stiva and Dolly — a fatal encounter foreshadowed by a worker's death on the tracks.",
    },
    Question {
        text: "What does Karenin do unexpectedly when Anna nearly dies giving birth to Vronsky's child?",
        options: [
            "He files for immediate divorce",
            "He forgives Anna and Vronsky",
            "He sends Vronsky into military exile",
            "He takes Seryozha away at once",
        ],
        correct: 1,
        explanation: "Believing Anna is dying, Karenin experiences a sudden Christian compassion and forgives both Anna and Vronsky — a grace that leaves Vronsky feeling deeply ashamed.",
    },
    Question {
        text: "What is Anna's emotional state in the final chapters before her death?",
        options: [
            "Peaceful and resigned",
            "Consumed by jealousy and paranoid despair",
            "Hopeful about reconciling with Karenin",
            "Focused on securing her children's future",
        ],
        correct: 1,
        explanation: "Anna spirals into jealous paranoia, convinced Vronsky is losing interest and planning to marry another woman — a delusion that drives her to her final act.",
    },
    Question {
        text: "Which character most clearly represents Tolstoy's own spiritual views?",
        options: ["Count Vronsky", "Konstantin Levin", "Alexei Karenin", "Stepan Oblonsky"],
        correct: 1,
        explanation: "Levin is widely seen as the most autobiographical character — his struggles with faith, meaning, death, and agricultural reform mirror Tolstoy's own preoccupations closely.",
    },
];

// ── Novel registry ────────────────────────────────────────────────────────────

static NOVELS: &[Novel] = &[
    Novel { name: "Middlemarch",   author: "George Eliot", questions: MIDDLEMARCH    },
    Novel { name: "Anna Karenina", author: "Leo Tolstoy",  questions: ANNA_KARENINA  },
];

// ── Exports ───────────────────────────────────────────────────────────────────

#[wasm_bindgen]
pub fn novel_count() -> usize { NOVELS.len() }

#[wasm_bindgen]
pub fn novel_name(i: usize) -> String { NOVELS[i].name.to_string() }

#[wasm_bindgen]
pub fn novel_author(i: usize) -> String { NOVELS[i].author.to_string() }

#[wasm_bindgen]
pub fn questions_per_quiz() -> usize { QUESTIONS_PER_QUIZ }

// ── Quiz ──────────────────────────────────────────────────────────────────────

#[wasm_bindgen]
pub struct Quiz {
    novel_idx: usize,
    order: Vec<usize>,
    current: usize,
    score: usize,
    last_correct: bool,
}

#[wasm_bindgen]
impl Quiz {
    /// Shuffle the full question pool, then keep only QUESTIONS_PER_QUIZ of them.
    pub fn create(novel_idx: usize, seed: u32) -> Quiz {
        let questions = NOVELS[novel_idx].questions;
        let mut order: Vec<usize> = (0..questions.len()).collect();
        let mut rng = seed as u64;
        for i in (1..order.len()).rev() {
            rng = rng
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            let j = (rng >> 33) as usize % (i + 1);
            order.swap(i, j);
        }
        order.truncate(QUESTIONS_PER_QUIZ);
        Quiz { novel_idx, order, current: 0, score: 0, last_correct: false }
    }

    pub fn total(&self) -> usize { self.order.len() }

    pub fn current_index(&self) -> usize { self.current }

    pub fn is_finished(&self) -> bool { self.current >= self.order.len() }

    pub fn question_text(&self) -> String {
        if self.is_finished() { return String::new(); }
        self.questions()[self.order[self.current]].text.to_string()
    }

    pub fn option_text(&self, i: usize) -> String {
        if self.is_finished() || i >= 4 { return String::new(); }
        self.questions()[self.order[self.current]].options[i].to_string()
    }

    pub fn submit_answer(&mut self, choice: usize) -> bool {
        if self.is_finished() { return false; }
        let correct = self.questions()[self.order[self.current]].correct == choice;
        self.last_correct = correct;
        if correct { self.score += 1; }
        self.current += 1;
        correct
    }

    pub fn score(&self) -> usize { self.score }

    pub fn explanation(&self) -> String {
        if self.current == 0 { return String::new(); }
        self.questions()[self.order[self.current - 1]].explanation.to_string()
    }

    pub fn correct_option(&self) -> usize {
        if self.current == 0 { return 0; }
        self.questions()[self.order[self.current - 1]].correct
    }
}

impl Quiz {
    fn questions(&self) -> &'static [Question] {
        NOVELS[self.novel_idx].questions
    }
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn novel_count_is_two() {
        assert_eq!(NOVELS.len(), 2);
    }

    #[test]
    fn all_novels_have_twenty_questions() {
        for novel in NOVELS {
            assert_eq!(
                novel.questions.len(), 20,
                "novel '{}' has {} questions, expected 20",
                novel.name, novel.questions.len()
            );
        }
    }

    #[test]
    fn pool_is_larger_than_quiz_size() {
        for novel in NOVELS {
            assert!(
                novel.questions.len() > QUESTIONS_PER_QUIZ,
                "novel '{}': pool ({}) must exceed quiz size ({})",
                novel.name, novel.questions.len(), QUESTIONS_PER_QUIZ
            );
        }
    }

    #[test]
    fn all_correct_indices_are_in_range() {
        for novel in NOVELS {
            for (i, q) in novel.questions.iter().enumerate() {
                assert!(
                    q.correct < 4,
                    "novel '{}' question {}: correct={} out of range 0..4",
                    novel.name, i, q.correct
                );
            }
        }
    }

    #[test]
    fn all_question_fields_are_nonempty() {
        for novel in NOVELS {
            for (i, q) in novel.questions.iter().enumerate() {
                assert!(!q.text.is_empty(),        "novel '{}' q{}: empty text", novel.name, i);
                assert!(!q.explanation.is_empty(), "novel '{}' q{}: empty explanation", novel.name, i);
                for (j, opt) in q.options.iter().enumerate() {
                    assert!(!opt.is_empty(), "novel '{}' q{} option {}: empty", novel.name, i, j);
                }
            }
        }
    }

    #[test]
    fn quiz_uses_questions_per_quiz_constant() {
        let quiz = Quiz::create(0, 42);
        assert_eq!(quiz.total(), QUESTIONS_PER_QUIZ);
    }

    #[test]
    fn quiz_initial_state() {
        let quiz = Quiz::create(0, 42);
        assert_eq!(quiz.score(), 0);
        assert_eq!(quiz.current_index(), 0);
        assert!(!quiz.is_finished());
    }

    #[test]
    fn correct_answer_returns_true_and_increments_score() {
        let mut quiz = Quiz::create(0, 42);
        let correct_opt = NOVELS[0].questions[quiz.order[0]].correct;
        assert!(quiz.submit_answer(correct_opt));
        assert_eq!(quiz.score(), 1);
    }

    #[test]
    fn wrong_answer_returns_false_and_does_not_score() {
        let mut quiz = Quiz::create(0, 42);
        let correct_opt = NOVELS[0].questions[quiz.order[0]].correct;
        let wrong_opt = (correct_opt + 1) % 4;
        assert!(!quiz.submit_answer(wrong_opt));
        assert_eq!(quiz.score(), 0);
    }

    #[test]
    fn quiz_finishes_after_all_questions_answered() {
        let mut quiz = Quiz::create(0, 0);
        for _ in 0..QUESTIONS_PER_QUIZ {
            quiz.submit_answer(0);
        }
        assert!(quiz.is_finished());
        assert_eq!(quiz.current_index(), QUESTIONS_PER_QUIZ);
    }

    #[test]
    fn perfect_score_when_every_answer_is_correct() {
        let mut quiz = Quiz::create(1, 7);
        for i in 0..QUESTIONS_PER_QUIZ {
            let correct_opt = NOVELS[1].questions[quiz.order[i]].correct;
            quiz.submit_answer(correct_opt);
        }
        assert_eq!(quiz.score(), QUESTIONS_PER_QUIZ);
    }

    #[test]
    fn shuffle_is_deterministic_for_same_seed() {
        let a = Quiz::create(0, 999);
        let b = Quiz::create(0, 999);
        assert_eq!(a.order, b.order);
    }

    #[test]
    fn shuffle_varies_for_different_seeds() {
        let a = Quiz::create(0, 1);
        let b = Quiz::create(0, 0xDEAD_BEEF);
        assert_ne!(a.order, b.order);
    }

    #[test]
    fn different_seeds_yield_different_question_subsets() {
        // With 20 questions choosing 10, different seeds should often pick different subsets
        let mut same_count = 0;
        for seed in 0..20_u32 {
            let a = Quiz::create(0, seed);
            let b = Quiz::create(0, seed + 1000);
            if a.order == b.order { same_count += 1; }
        }
        // Allow at most 1 collision out of 20 attempts
        assert!(same_count <= 1, "too many identical subsets across seeds: {}", same_count);
    }

    #[test]
    fn explanation_and_correct_option_reflect_last_answered_question() {
        let mut quiz = Quiz::create(0, 3);
        let q_idx           = quiz.order[0];
        let expected_correct = NOVELS[0].questions[q_idx].correct;
        let expected_expl    = NOVELS[0].questions[q_idx].explanation;
        quiz.submit_answer(0);
        assert_eq!(quiz.correct_option(), expected_correct);
        assert_eq!(quiz.explanation(), expected_expl);
    }

    #[test]
    fn both_novels_are_quizzable_to_perfect_score() {
        for novel_idx in 0..NOVELS.len() {
            let mut quiz = Quiz::create(novel_idx, 1);
            for i in 0..QUESTIONS_PER_QUIZ {
                let correct = NOVELS[novel_idx].questions[quiz.order[i]].correct;
                quiz.submit_answer(correct);
            }
            assert_eq!(quiz.score(), QUESTIONS_PER_QUIZ);
            assert!(quiz.is_finished());
        }
    }
}
