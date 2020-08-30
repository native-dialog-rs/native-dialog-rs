#[derive(Copy, Clone)]
pub enum MessageType {
    Info,
    Warning,
    Error,
}

pub struct MessageAlert<'a> {
    pub(crate) title: &'a str,
    pub(crate) text: &'a str,
    pub(crate) typ: MessageType,
}

impl<'a> MessageAlert<'a> {
    pub fn new() -> Self {
        MessageAlert {
            title: "",
            text: "",
            typ: MessageType::Info,
        }
    }

    pub fn title(&mut self, title: &'a str) -> &mut Self {
        self.title = title;
        self
    }

    pub fn text(&mut self, text: &'a str) -> &mut Self {
        self.text = text;
        self
    }

    pub fn typ(&mut self, typ: MessageType) -> &mut Self {
        self.typ = typ;
        self
    }
}

pub struct MessageConfirm<'a> {
    pub(crate) title: &'a str,
    pub(crate) text: &'a str,
    pub(crate) typ: MessageType,
}

impl<'a> MessageConfirm<'a> {
    pub fn new() -> Self {
        MessageConfirm {
            title: "",
            text: "",
            typ: MessageType::Info,
        }
    }

    pub fn title(&mut self, title: &'a str) -> &mut Self {
        self.title = title;
        self
    }

    pub fn text(&mut self, text: &'a str) -> &mut Self {
        self.text = text;
        self
    }

    pub fn typ(&mut self, typ: MessageType) -> &mut Self {
        self.typ = typ;
        self
    }
}
