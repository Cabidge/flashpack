use std::collections::BTreeMap;

use crate::{Card, Completeness, Question, Quiz, QuizQuery, QuizSummary};

#[derive(Default)]
pub struct Sessions {
    uid: u32,
    sessions: BTreeMap<u32, Quiz>,
}

impl Sessions {
    pub fn create(&mut self, name: String, cards: &[Card]) -> u32 {
        let quiz_id = self.uid;
        let id = quiz_id.to_string();

        let questions = cards
            .iter()
            .map(|card| Question {
                question: card.front.to_owned(),
                answer: card.back.to_owned(),
                status: Completeness::Incomplete,
            })
            .collect();

        let quiz = Quiz {
            id,
            name,
            questions,
        };

        self.sessions.insert(self.uid, quiz);

        self.uid += 1;

        quiz_id
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut Quiz> {
        self.sessions.get_mut(&id)
    }

    pub fn summarize(&self) -> Vec<QuizSummary> {
        self.sessions
            .iter()
            .map(|(&id, quiz)| QuizSummary {
                id: QuizQuery::Fleeting(id),
                name: quiz.name.to_string(),
            })
            .collect()
    }
}
