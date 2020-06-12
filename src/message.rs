pub enum MessageType {
    Info,
    Warning,
    Error,
}

pub struct MessageAlert<'a> {
    pub title: &'a str,
    pub text: &'a str,
    pub typ: MessageType,
}

pub struct MessageConfirm<'a> {
    pub title: &'a str,
    pub text: &'a str,
    pub typ: MessageType,
}
