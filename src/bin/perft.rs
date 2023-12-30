use clap::Parser;
use rust_chess::GameState;
use std::collections::HashMap;

type Depth = u32;
type ScenarioId = u32;
type Perft = u128;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = 0)]
    scenario: ScenarioId,
    #[arg(short, long)]
    depth: Depth,
}

fn main() {
    let Cli { scenario, depth } = Cli::parse();
    let fen_map = init_fen_map();
    let fen = fen_map.get(&scenario).expect("Can't find fen");
    let mut gs = GameState::try_from_fen(fen).unwrap();
    let perft = gs.perft(depth);
    println!("Calculated perft: {perft}");
    let perft_map = init_perft_map();
    if let Some(&ref_perft) = perft_map.get(&(scenario, depth)) {
        assert_eq!(perft, ref_perft);
    }
}

fn init_perft_map() -> HashMap<(ScenarioId, Depth), Perft> {
    HashMap::from([
        ((0, 3), 8_902),
        ((0, 4), 197_281),
        ((3, 2), 191),
        ((3, 3), 2_812),
        ((5, 2), 1_486),
        ((5, 3), 62_379),
    ])
}

#[rustfmt::skip]
fn init_fen_map() -> HashMap<ScenarioId, &'static str> {
    HashMap::from([
        (0, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
        (2, "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"),
        (3, "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1"),
        (4, "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1"),
        (5, "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8"),
    ])
}
