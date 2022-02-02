use crate::models::color::Color;
use crate::models::game::Game;
use crate::models::player::Player;
use crate::utils::bot::{create_bot_name, make_a_move_bot};
use crate::utils::enums::MoveResult;
use crate::utils::player::make_a_move_player;

pub fn initialize_players(player_names: Vec<String>) -> Vec<Player> {
  let mut colors = [Color::Red, Color::Green, Color::Blue, Color::Yellow].iter();
  let mut players = vec![];
  for name in player_names {
    players.push(Player::new(
      "0".to_string(),
      name,
      *colors.next().unwrap(),
      false,
    ))
  }
  while players.len() < 4 {
    players.push(Player::new(
      "0".to_string(),
      create_bot_name(),
      *colors.next().unwrap(),
      true,
    ))
  }
  players
}

pub fn play_round() {
  let mut game = Game::new(); // TODO replace with find_game(id); (from DB)

  let player = game.get_current_player_mut();

  let move_result = match player.is_bot {
    true => make_a_move_bot(&mut game),
    false => make_a_move_player(&mut game),
  };

  match move_result {
    MoveResult::Success(_) => {
      game.update_current_player();
      // <<update db>> since field(s) have changed (at least current_player has changed,
      //    even if player's move was skipped)
      // send message to client(s) ?
    }
    MoveResult::Error(_) => {
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
