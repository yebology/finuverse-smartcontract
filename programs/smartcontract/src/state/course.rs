use anchor_lang::prelude::*;

#[account]
pub struct Course {
    pub id: u64,
    pub name: String,
    pub creator: Pubkey,
    pub description: String,
    pub price: u64,
    pub buyer: u64,
    pub thumbnail: String,
    pub section_title: [String; 3],
    pub section_description: [String; 3],
    pub section_duration: [u64; 3],
    pub section_video: [String; 3],
    pub question_list: [String; 3],
    pub answer_list: [String; 3],
    pub first_answer_options: [String; 4],
    pub second_answer_options: [String; 4],
    pub third_answer_options: [String; 4],
}

impl Course {
    const ID_SPACE: usize = 8;
    const NAME_SPACE: usize = 4 + 100;
    const CREATOR_SPACE: usize = 32;
    const DESCRIPTION_SPACE: usize = 4 + 300;
    const PRICE_SPACE: usize = 8;
    const BUYER_SPACE: usize = 8;
    const THUMBNAIL_SPACE: usize = 4 + 150;
    const SECTION_TITLE_SPACE: usize = 3 * (4 + 100);
    const SECTION_DESCRIPTION_SPACE: usize = 3 * (4 + 300);
    const SECTION_DURATION_SPACE: usize = 3 * 8;
    const SECTION_VIDEO_SPACE: usize = 3 * (4 + 150);
    const QUESTION_LIST_SPACE: usize = 3 * (4 + 100);
    const ANSWER_LIST_SPACE: usize = 3 * (4 + 100);
    const FIRST_ANSWER_OPTIONS_SPACE: usize = 4 * (4 + 50);
    const SECOND_ANSWER_OPTIONS_SPACE: usize = 4 * (4 + 50);
    const THIRD_ANSWER_OPTIONS_SPACE: usize = 4 * (4 + 50);

    pub const MAXIMUM_SIZE: usize = Self::ID_SPACE
        + Self::NAME_SPACE
        + Self::CREATOR_SPACE
        + Self::DESCRIPTION_SPACE
        + Self::PRICE_SPACE
        + Self::BUYER_SPACE
        + Self::THUMBNAIL_SPACE
        + Self::SECTION_TITLE_SPACE
        + Self::SECTION_DESCRIPTION_SPACE
        + Self::SECTION_DURATION_SPACE
        + Self::SECTION_VIDEO_SPACE
        + Self::QUESTION_LIST_SPACE
        + Self::ANSWER_LIST_SPACE
        + Self::FIRST_ANSWER_OPTIONS_SPACE
        + Self::SECOND_ANSWER_OPTIONS_SPACE
        + Self::THIRD_ANSWER_OPTIONS_SPACE;
}