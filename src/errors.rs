use thiserror::Error;

/*#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Game(#[from] GameErr),
}*/
#[derive(Debug, Error)]
pub enum GameErr {
    #[error("Illigal move. Snake coalition")]
    SnakeCrashedIntoItself,
}

pub type GameResult<T> = std::result::Result<T, GameErr>;

/*
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
