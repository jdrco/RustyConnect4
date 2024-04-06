#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct State {
    pub unresolved_card_pairs: u8,
    pub best_score: u32,
    pub status: Status,
    pub last_player: Vec<Card>,
    pub last_letter: Option<RawCard>,
    pub rollback_cards: Option<[RawCard; 2]>,
}
