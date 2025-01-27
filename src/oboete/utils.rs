// SPDX-License-Identifier: GPL-3.0-only

use rand::{seq::SliceRandom, thread_rng};

use super::models::flashcard::Flashcard;

pub fn select_random_flashcard(flashcards: &Vec<Flashcard>) -> Option<Flashcard> {
    let mut rng = thread_rng();
    let mut weighted_flashcards = Vec::new();

    for flashcard in flashcards {
        let weight = match flashcard.status {
            1 => 4, // High chance (status = 1 = flashcard Bad)
            2 => 3, // Medium chance (status = 2 = flashcard Ok)
            3 => 1, // Low chance (status = 3 = flashcard Good)
            _ => 2, // Default chance for other statuses
        };

        for _ in 0..weight {
            weighted_flashcards.push(flashcard);
        }
    }

    // Select a random flashcard from the weighted list
    weighted_flashcards.choose(&mut rng).copied().cloned()
}

pub fn parse_import_content(
    line_delimiter: &String,
    term_delimiter: &String,
    content: &str,
) -> Vec<Flashcard> {
    content
        .split(line_delimiter)
        .filter_map(|line| {
            let mut terms = line.split(term_delimiter);
            if let (Some(front), Some(back)) = (terms.next(), terms.next()) {
                Some(Flashcard {
                    id: None,
                    front: front.to_string(),
                    back: back.to_string(),
                    status: 0,
                })
            } else {
                None
            }
        })
        .collect()
}
