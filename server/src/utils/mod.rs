

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ClientMessage {
    ThrowDice,
    MoveFigure(usize),
    PromotePiece // shouldn't need to pass color, since server should has attr current_player
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ServerMessage {
    DiceValue(usize), // response to ThrowDice
    SkipPlayer, // - if there are no moves, we have to move on to the next player?
                // - is handled in FE? (we can use 'GetAvailableMoves' util)
                //   since player can't really choose a piece/position to move (i.e. we ?
    MoveSuccessful(String),  // we successfully moved a piece; msg = "Piece is now secure in your home"
                             // or MoveSuccess
    MoveFailed(String),  // or MoveFail / MoveError - if we can't make a certain move
                         // - would overjump home; field is blocked by our piece ...
    PiecePromoted, // response to PromotePiece - is also MoveSuccessful, but
    Information(String),

    // NewPlayer(String),
    // PlayerReconnected(usize),
    // PlayerDisconnected(usize),
    // GameOver(Color),  // winner of a game
}

use crate::models::color::Color;
use crate::types::Field;
use crate::models::game::{Game, MoveResult};
use rand::Rng;
use crate::models::player::Player;


pub fn get_dice_value() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..7)
}

pub fn throw_dice() -> usize {
    let mut dice_value: usize = 0;
    // player/client sends MessageType::ThrowDice
    // << message exchange >>
    match get_dice_value() {
        6 => {
            dice_value += 6;
            // << message exchange >>;
            match get_dice_value() {
                6 => {
                    dice_value += 6;
                    // << message exchange >>;
                    match get_dice_value() {
                        // if we throw 6 three times, it gets reset
                        6 => {
                            dice_value = 0;
                            // << message exchange >>
                        },
                        n => dice_value += n
                    }
                },
                n => dice_value += n
            }
        },
        n => dice_value += n
    }

    dice_value
}

pub fn throw_dice_bot(game: &Game) -> usize {

    match get_dice_value() {
        // if AI can promote, it should (always?) promote
        //   - or should we also consider getting our piece to home / jumping on opponents' pieces?
        6 => {
            match game.can_promote_piece() {
                true => 6,
                // if can't promote, keep throwing dice
                false => match get_dice_value() {
                    6 => match get_dice_value() {
                        // 3x6 => 0
                        6 => 0,
                        n => 6 + 6 + n,
                    },
                    n => 6 + n
                }
            }
        },
        n => n
    }
}


pub fn make_a_move_bot(game: &mut Game, player: &mut Player) -> MoveResult {

    let dice_value = throw_dice_bot(game);

    if dice_value == 6 {
        game.promote_piece();
        return MoveResult::Success(String::from("Piece promoted."))
    }

    // if dice_value = 6, we can promote (was checked)
    // otherwise, check available moves

    let piece_positions = game.get_players_pieces_positions(game.current_player);

    let piece_positions_to_jump_home: Vec<usize> = piece_positions
        .iter()
        .filter(|&position| game.can_jump_to_home(*position, dice_value))
        .collect();

    // we can choose a random piece to move, since all of them will end up in home
    if !piece_positions_to_jump_home.is_empty() {
        return game.execute_move(piece_positions_to_jump_home[0], dice_value)
    }

    // otherwise, we will check if we can move any piece at all (currently we won't try to remove
    //   opponents' pieces)
    let piece_positions_to_move: Vec<usize> = piece_positions
        .iter()
        .filter(|&position| game.can_jump(*position, dice_value))
        .collect();

    // we can choose a random piece to move (i.e. is not blocked).. or a piece that's closest to home for example
    match !piece_positions_to_move.is_empty() {
        true => game.execute_move(piece_positions_to_move[0], dice_value),
        // no valid move
        false => MoveResult::Error(String::from("No valid move."))
    }

}


// we could check if any moves are possible for the player - that has to be done on FE?
// if player has no valid moves, he should be skipped (as it's done for AI),
//    but when/how we inform the client ?
// player/client can keep sending us positions, but if he has no valid moves at all,
//   we will keep giving MoveResult::Error
// idealne by client/player hned vedel, ze hrac nemoze tiahnut figurkou - teoreticky by
//    od clienta mohla prist poziadavka GetValidPositions - pozicie figuriek, s ktorymi moze
//    hrac tiahnut, a server ich posle na clienta (alebo message NoValidPositions)
pub fn make_a_move_player(game: &mut Game, player: &mut Player) -> MoveResult {
    let dice_value = await throw_dice();
    let position: usize = await message_from_client/player();
    game.execute_move(position, dice_value)
}


pub fn make_a_move() {

    let mut game: Game = find_game(id);
    let mut player = game.get_current_player_mut();

    let move_result = match player.is_bot {
        true => make_a_move_bot(&mut game, &mut player),
        false => make_a_move_player(&mut game, &mut player)
    };

    match move_result {
        MoveResult::Success(msg) => {
            game.update_current_player();
            // <<update db>> since field(s) have changed (at least current_player has changed,
            //    even if player's move was skipped)
            // send message to client(s) ?
        },
        MoveResult::Error(msg) => {
            // <<move was invalid>> - we dont need to update db?
            // inform player/client(s) about error ?
        }
    }

    // upravit check_winner - pridat atribut pawns_at_home
    // skontrolovat najskor, ci ide o move v ramci home column
    //   - t.j. funkcii predame len home_offset namiesto position

    // je plan z DC - musime sa dostat do domecku (za home column)
    // home column ma 5 policek - ak stojime na prvom policku, musime hodit 5,
    //   aby sme sa dostali do home


    // aky je nakoniec ten plan - ako velky je domecek a ci sa mozu figurky v domecku posuvat,
    //  pripadne, ci potrebuju 'skocit' do ciela (za home column) - potom treba pridat nejaky atribut
    //  'pawns_at_finish' / 'pawns_in_home'
    // na FE by malo byt tlacitko na 'Promote piece/pawn/figure' alebo len oznacenie celeho startovaneho bloku?
    // najskor hrac hodi kockou (poziada server o vygenerovanie hodnoty 1-6),
    //    ten hodnotu posle clientovi
    //    - ak hodi 1-5, musi zvolit figurku s ktorou chce tiahnut

    // napr. client caka, az obdrzi spravu od serveru (ci je ValidMove, ...):
    //   - ak obdrzi MoveFailed, tak sa hrac nezmeni
    //   - ak obdrzi MoveSuccessful / SkipPlayer / ..., tak sa zmeni hrac a musime nasledne
    //     vyslat na server spravu MakeMove, aby sme zavolali na serveri make_move()
    //     - v ramci MakeMove server caka na ThrowDice message od clienta,
    //       a nasledne bud MovePiece(position) alebo PromotePiece atd.
    //   - ak obrdzi GameOver(winner), tak moze oznamit vitaza a uz neposiela MakeMove spravu serveru

    // pre zacatie hry mozeme mat StartGame - ma aj 'argumenty', ci uz je vsetko ulozene v DB ?
    // definovat ake messages moze posielat client / server



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
    //    po aktualizacii) - a napr. 'gray out' tlacitko, ktore normalne umozni hodit kostkou
    // zasleme spravu nasledujucemu hracovi, ze je na rade (napr. CurrentPlayer)
    // a hraci, ktory skoncil tah teraz posleme spravu, ze nie je na rade (NotCurrentPlayer)
}
