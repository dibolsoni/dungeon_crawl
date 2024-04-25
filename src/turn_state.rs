

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    Victory,
    NextLevel,
}
