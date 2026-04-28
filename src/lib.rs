mod utils;
use wasm_bindgen::prelude::*;

struct Question {
    text: &'static str,
    options: [&'static str; 4],
    correct: usize,
    explanation: &'static str,
}

static QUESTIONS: &[Question] = &[
    Question {
        text: "Who wrote Middlemarch?",
        options: [
            "Jane Austen",
            "George Eliot",
            "Charlotte Brontë",
            "Thomas Hardy",
        ],
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
        options: [
            "Will Ladislaw",
            "Tertius Lydgate",
            "Edward Casaubon",
            "Nicholas Bulstrode",
        ],
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
        options: [
            "Fred Vincy",
            "Will Ladislaw",
            "James Chettam",
            "Tertius Lydgate",
        ],
        correct: 1,
        explanation: "After Casaubon's death, Dorothea marries the idealistic Will Ladislaw, forfeiting her inheritance to do so.",
    },
    Question {
        text: "What is Tertius Lydgate's profession?",
        options: [
            "Lawyer",
            "Clergyman",
            "Banker",
            "Doctor",
        ],
        correct: 3,
        explanation: "Lydgate is an ambitious young doctor who comes to Middlemarch hoping to pursue medical reform and research.",
    },
    Question {
        text: "Who does Lydgate marry?",
        options: [
            "Dorothea Brooke",
            "Mary Garth",
            "Rosamond Vincy",
            "Celia Brooke",
        ],
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
        options: [
            "Harriet Bulstrode",
            "Mary Garth",
            "Dorothea Brooke",
            "Celia Brooke",
        ],
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
        options: [
            "1815–1820",
            "1829–1832",
            "1850–1855",
            "1870–1875",
        ],
        correct: 1,
        explanation: "The novel is set during the era of the First Reform Act and other social changes in England, roughly 1829–1832.",
    },
    Question {
        text: "What is Caleb Garth's occupation?",
        options: [
            "Schoolteacher",
            "Clergyman",
            "Land agent and surveyor",
            "Physician",
        ],
        correct: 2,
        explanation: "Caleb Garth is an honest, hardworking land agent who serves as a quiet moral touchstone in the novel.",
    },
    Question {
        text: "In what year was Middlemarch first published as a complete book?",
        options: [
            "1865",
            "1848",
            "1880",
            "1872",
        ],
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
];

#[wasm_bindgen]
pub struct Quiz {
    order: Vec<usize>,
    current: usize,
    score: usize,
    last_correct: bool,
}

#[wasm_bindgen]
impl Quiz {
    pub fn create(seed: u32) -> Quiz {
        let mut order: Vec<usize> = (0..QUESTIONS.len()).collect();
        let mut rng = seed as u64;
        for i in (1..order.len()).rev() {
            rng = rng
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            let j = (rng >> 33) as usize % (i + 1);
            order.swap(i, j);
        }
        Quiz { order, current: 0, score: 0, last_correct: false }
    }

    pub fn total(&self) -> usize {
        QUESTIONS.len()
    }

    pub fn current_index(&self) -> usize {
        self.current
    }

    pub fn is_finished(&self) -> bool {
        self.current >= self.order.len()
    }

    pub fn question_text(&self) -> String {
        if self.is_finished() { return String::new(); }
        QUESTIONS[self.order[self.current]].text.to_string()
    }

    pub fn option_text(&self, i: usize) -> String {
        if self.is_finished() || i >= 4 { return String::new(); }
        QUESTIONS[self.order[self.current]].options[i].to_string()
    }

    pub fn submit_answer(&mut self, choice: usize) -> bool {
        if self.is_finished() { return false; }
        let correct = QUESTIONS[self.order[self.current]].correct == choice;
        self.last_correct = correct;
        if correct { self.score += 1; }
        self.current += 1;
        correct
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn explanation(&self) -> String {
        if self.current == 0 { return String::new(); }
        QUESTIONS[self.order[self.current - 1]].explanation.to_string()
    }

    pub fn correct_option(&self) -> usize {
        if self.current == 0 { return 0; }
        QUESTIONS[self.order[self.current - 1]].correct
    }
}
