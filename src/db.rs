use crate::game::{Game, GameStatus};
use chrono::Utc;
use rusqlite::{params, Connection, Error, Result};
use std::time::SystemTime;

pub fn init() -> Result<usize, rusqlite::Error> {
    let db_created = std::path::Path::new("snake.db").exists();
    if db_created {
        return Ok(1);
    }
    println!("Setting up database...");
    let conn = Connection::open("snake.db")?;
    let query = "
            CREATE TABLE IF NOT EXISTS Game (
                ID INTEGER PRIMARY KEY AUTOINCREMENT,
                CreatedAt NUMERIC,
                PlayerName TEXT,
                Score INTEGER,
                LastUpdatedAt NUMERIC,
                Status TEXT,
                SnakePosX INTEGER,
                SnakePosY INTEGER
            );
        ";
    let res = conn.execute(query, ());
    println!("Dataase created.");
    res
}

pub fn get(player_name: String) -> bool {
    println!("Getting game");
    true
}

pub fn update(game: &Game) -> Result<(), rusqlite::Error> {
    let conn = Connection::open("snake.db")?;

    let update_query = "
        UPDATE GAME
        SET
            Score = ?1,
            SnakePosX = ?2,
            SnakePosY = ?3
        WHERE
            Id = ?4
    ";
    let x = &game.snake.parts_x_y[1].x.to_string();
    let y = &game.snake.parts_x_y[1].y.to_string();

    conn.execute(update_query, params![&game.score, x, y, &game.db_id]);
    Ok(())
}

pub fn insert(game: &mut Game) -> Result<(), rusqlite::Error> {
    let conn = Connection::open("snake.db")?;
    let insert_query = "
        INSERT INTO Game (
            CreatedAt,
            PlayerName,
            Score,
            LastUpdatedAt,
            Status,
            SnakePosX,
            SnakePosY
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);
    ";
    let game_status = GameStatus::RUNNING.get_db_value();
    let pos_x_y: (u16, u16) = (game.snake.parts_x_y[0].x, game.snake.parts_x_y[0].y);
    let now = Utc::now().timestamp();
    conn.execute(
        insert_query,
        params![
            now,              // CreatedAt (example: unix timestamp)
            game.player_name, // PlayerName
            game.score,       // Score
            now,              // LastUpdatedAt
            game_status,      // Status
            pos_x_y.0,        // SnakePosX
            pos_x_y.1         // SnakePosY
        ],
    )?;
    let row_id = conn.last_insert_rowid();
    game.db_id = Some(row_id as u16); // or whatever type your db_id is
    Ok(())
}
