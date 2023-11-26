use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen};
use std::collections::{HashMap, HashSet};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    widget_feedbacks: HashMap<String, Vec<String>>,
    widget_ratings: HashMap<String, Vec<(u64, u64)>>,
    voted_accounts: HashSet<String>,
}

#[near_bindgen]
impl Contract {
    pub fn get_feedbacks(&self, widget_id: String) -> Option<Vec<String>> {
        self.widget_feedbacks.get(&widget_id).cloned()
    }

    pub fn add_feedback(
        &mut self,
        feedback: String,
        widget_id: String,
        widget_link: String,
        account_id_givefeedback: String,
        star: u64,
    ) {
        log!(
            "Adding message {} with widget ID {} widget link {} from account {}",
            feedback,
            widget_id,
            widget_link,
            account_id_givefeedback
        );

        // Get or create the vector for the widget ID
        let widget_feedbacks = self.widget_feedbacks.entry(widget_id.clone()).or_insert(vec![]);

        // Append the new feedback
        widget_feedbacks.push(format!(
            "{} said {} ",
            account_id_givefeedback, feedback
        ));

        // Log the feedback
        log!("{} said {} to {}", account_id_givefeedback, feedback, widget_id);

        // Add star rating
        if !self.voted_accounts.contains(&account_id_givefeedback) {
            let widget_rating = self.widget_ratings.entry(widget_id.clone()).or_insert(vec![]);
            let count_star = widget_rating.len() as u64 + 1;

            widget_rating.push((star, count_star));

            // Clone the account ID before inserting it into the HashSet
            self.voted_accounts.insert(account_id_givefeedback.clone());

            env::log(format!(
                "Added {} stars to {} with count {} by account {}",
                star, widget_id, count_star, account_id_givefeedback
            ).as_bytes());
        } else {
            env::log(format!(
                "Account {} has already voted for this widget",
                account_id_givefeedback
            ).as_bytes());
        }
    }

    pub fn get_star(&self, widget_id: String) -> Option<(String, f64)> {
        if let Some(widget_ratings) = self.widget_ratings.get(&widget_id) {
            // Calculate the sum of stars and total count
            let (total_stars, total_count) = widget_ratings.iter().fold((0, 0), |acc, x| {
                (acc.0 + x.0 * x.1, acc.1 + x.1)
            });

            let average_star = if total_count > 0 {
                total_stars as f64 / total_count as f64
            } else {
                0.0
            };

            Some((widget_id, average_star))
        } else {
            None
        }
    }
}
