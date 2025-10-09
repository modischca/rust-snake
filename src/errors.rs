use thiserror::Error;
#[derive(Debug, Error)]
pub enum GameErr {
    #[error("Illigal move. Snake coalition")]
    SnakeCrashedIntoItself,

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Rusqlite(#[from] rusqlite::Error),
}

pub type GameResult<T> = std::result::Result<T, GameErr>;

/*
 *
 * CUSTOM ERROR WITHOUT CARGO (LIB)
impl Display for GameErr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            GameErr::SnakeCrashedIntoItself => {
                write!(f, "Illegal move. Snake coalition.")
            }
        }
    }
}
impl stdErr for GameErr {}
*/
