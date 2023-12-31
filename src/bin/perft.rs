use clap::Parser;
use rust_chess::GameState;
use std::{cell::OnceCell, collections::HashMap, hash::Hash, time::Instant};

type Depth = u32;
type ScenarioId = u32;
type Perft = u128;

const FEN_MAP: LazyMap<ScenarioId, &'static str> = LazyMap::new(init_fen_map);
const PERFT_MAP: LazyMap<(ScenarioId, Depth), Perft> = LazyMap::new(init_perft_map);

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = 1)]
    scenario: ScenarioId,
    #[arg(short, long)]
    depth: Depth,
}

fn calc_perft(fen: &str, depth: Depth) -> Result<u128, &'static str> {
    let mut gs = GameState::try_from_fen(fen)?;
    let perft = gs.perft(depth);
    Ok(perft)
}

fn calc_scenario_perft(scenario: ScenarioId, depth: Depth) -> Result<u128, &'static str> {
    calc_perft(FEN_MAP.get(&scenario).ok_or("couldn't find fen")?, depth)
}

fn main() {
    let Cli { scenario, depth } = Cli::parse();
    let start = Instant::now();
    let perft = calc_scenario_perft(scenario, depth).unwrap();
    println!("Elapsed time: {:.2?}", start.elapsed());
    println!("Calculated perft: {perft}");
    if let Some(&ref_perft) = PERFT_MAP.get(&(scenario, depth)) {
        assert_eq!(perft, ref_perft);
        println!("Result matches expected output");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_pos(scenario: ScenarioId, depth: Depth) {
        let perft = calc_scenario_perft(scenario, depth).unwrap();
        let ref_perft = *PERFT_MAP.get(&(scenario, depth)).unwrap();
        assert_eq!(perft, ref_perft);
    }

    #[test]
    fn test_pos_1() {
        test_pos(1, 4)
    }

    #[test]
    fn test_pos_3() {
        test_pos(3, 2)
    }
}

fn init_perft_map() -> HashMap<(ScenarioId, Depth), Perft> {
    HashMap::from([
        ((1, 3), 8_902),
        ((1, 4), 197_281),
        ((1, 5), 4_865_609),
        ((3, 2), 191),
        ((3, 3), 2_812),
        ((5, 2), 1_486),
        ((5, 3), 62_379),
    ])
}

#[rustfmt::skip]
fn init_fen_map() -> HashMap<ScenarioId, &'static str> {
    HashMap::from([
        (1, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
        (2, "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"),
        (3, "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1"),
        (4, "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1"),
        (5, "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8"),
    ])
}

struct LazyMap<K, V> {
    inner: OnceCell<HashMap<K, V>>,
    func: fn() -> HashMap<K, V>,
}

impl<K, V> LazyMap<K, V>
where
    K: Hash + Eq,
{
    const fn new(func: fn() -> HashMap<K, V>) -> Self {
        Self {
            inner: OnceCell::new(),
            func,
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.inner.get_or_init(self.func).get(key)
    }
}
