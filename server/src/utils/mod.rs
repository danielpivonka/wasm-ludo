

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ClientMessage {
    // CreateRoom(...),
    // JoinRoom(...),
    ThrowDice,
    MoveFigure(i32),
    PlaceFigure
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ServerMessage {
    JoinedRoom {
        room_name: String,
        players: Vec<(String, u32, bool)>,
        active_player: usize,
        player_index: usize,
        board: Vec<((i32, i32), Piece)>,
        pieces: Vec<Piece>,
    },
    JoinFailed(String),
    Chat {
        from: String,
        message: String,
    },
    Information(String),
    NewPlayer(String),
    PlayerReconnected(usize),
    PlayerDisconnected(usize),
    PlayerTurn(usize),
    Played(Vec<(Piece, i32, i32)>),
    Swapped(usize),
    MoveAccepted(Vec<Piece>),
    MoveRejected,
    PlayerScore {
        delta: u32,
        total: u32,
    },
    PiecesRemaining(usize),
    ItsOver(usize),
}

use crate::models::color::Color;
use crate::types::Field;
use crate::models::game::{Game, MoveResultType};
use rand::Rng;


//
// // could be a method of Game
// // returns player's home Vec<Field> based on their color
// pub fn get_home(color: Color) -> Vec<Field> {
//
// }


pub fn get_dice_value() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..7)
}

pub fn throw_dice() -> usize {
    let mut dice_value: usize = 0;
    // player/client sends MessageType::ThrowDice
    << message exchange >>
    match get_dice_value() {
        6 => {
            dice_value += 6;
            << message exchange >>;
            match get_dice_value() {
                6 => {
                    dice_value += 6;
                    << message exchange >>;
                    match get_dice_value() {
                        // if we throw 6 three times, it gets reset
                        6 => {
                            dice_value = 0;
                            << message exchange >>
                        },
                        n => dice_value += n;
                    }
                },
                n => dice_value += n;
            }
        },
        n => dice_value += n
    }

    dice_value
}

pub fn make_a_move() {

    let mut game: Game = find_game(id);

    let mut player = game.get_player();

    if player.is_bot {
        return make_a_move_bot()
    }

    // dice_value
    let dice_value = throw_dice();
    let position: usize = message_from_client/player();


    // throw a dice and get position (which piece to move) from player
    // different behaviour for AI (special attribute in Game? AI_player?
    //    - if AI_players contains game.current_player => it is AI

    match game.is_a_valid_move(position, dice_value) {
        MoveResultType::Success => {
            game.execute_move();
            game.update_current_player();
            update_db(...)
        },
        MoveResultType::Error(err) => send/broadcast_error_message(err)
    }

    // throw dice (generate 1-6, and inform the player(s) - send a message)
    // wait for a message from player (his choice of figure for example)

    // je zalozene na loopoch? vzdy cakame na urcity typ spravy od klienta:

    // vzdy ked obdrzime message - deserializovat, a podla typu message nieco spravit
    //    MessageType::ThrowDice
    //    MessageType::MoveFigure(position)
    // napr. ak klient posle ThrowDice message, tak musi nasledovat MoveFigure message s poziciou figurky

    // ak klient posle zlu poziciu (napr. field je empty alebo figurka patri superovi - ak to umozni frontend),
    // tak posleme klientovi spravu o 'chybe' - 'You can only move your own pieces.'


    // loop kym nedostaneme ThrowDice message (cez match MessageType) {
    //    ThrowDice => 1. vygenerujeme hodnotu 1-6
    //                 2. checkneme, ci ma hrac valid moves:
    //                     - ak nie, posleme NoMoves message, nastavime dalsieho hraca a return
    //                     - ak ano, len breakneme loop a cakame na dalsiu spravu od klienta
    //    _ => 1. odosleme message, ze najskor treba hodit kostkou? stale sme v loope
    // }
    //
    // << mame dice_value >>
    //
    // loop kym nedostaneme validnu MovePiece message {} - ci position oznacuje policko s nasou figurkou
    //

    // co ak klienta nema ziadne volne tahy? automaticky by sme ho mali skipnut
    //   (t.j. message pre klienta A / broadcast pre vsetkych klientov, ze:
    //    > 'Player A has no available moves, skipping.'
    //    > 'Next player - Player B.'

    // player chooses a piece to move (might choose figure at start)
    //    - special coordinate (-1), or a specific message?
    //    - if the player doesn't throw a 6, should the choice for getting a piece into a field be
    //      grayed out?
    // >>> Use a special MessageType (PlaceFigure)

    // if a player throws a 6, he can:
    //   a) get a piece from start to field - doesn't get a bonus throw
    //   b) decides to move one of his pieces in the field - gets an extra throw (applies to the same figure ??)


    // ako ukladat aktualneho / nasledujuceho hraca? v DB
    // pri ukonceni tahu by sa mal vo frontende prepnut dalsi hrac (napr. podla svojej farby vs. current_player
    //    po aktualizacii) - a napr. 'zasednut' tlacitko, ktore normalne umozni hodit kostkou
    // zasleme spravu nasledujucemu hracovi, ze je na rade (napr. CurrentPlayer)
    // a hraci, ktory skoncil tah teraz posleme spravu, ze nie je na rade (NotCurrentPlayer)


    // pridat .idea do gitignore
}
