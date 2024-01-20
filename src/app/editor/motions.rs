#[derive(Debug, Clone)]
pub enum Motion {
    CharForward,
    CharBackward,
    NextWordStart,
    NextWordProperStart,
    NextWordEnd,
    NextWordProperEnd,
    LastWordStart,
    LastWordProperStart,
    LastWordEnd,
    LastWordProperEnd,
    NextLineStart,
    LineStart,
    LineEnd,
}
