pub struct Message {
    pub user: User,
    pub text: String,
    pub is_addressed: bool,
    pub channel: String,
}