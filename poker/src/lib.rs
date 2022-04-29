use std::cmp::{Ordering, Reverse};
use std::collections::{HashMap, HashSet};

/// representation of poker ranks
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum PokerRank {
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKinid,
    TwoPair,
    Pair,
    HighCard,
}

#[derive(Debug, Eq)]
struct PokerHand<'a> {
    source: &'a str,
    groups: Vec<(u8, u8)>,
    rank: PokerRank,
}

impl<'a> PartialEq for PokerHand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.groups == other.groups
    }
}

impl<'a> PartialOrd for PokerHand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.rank.cmp(&other.rank) {
            Ordering::Equal => Some(other.groups.cmp(&self.groups)),
            c => Some(c),
        }
    }
}

impl<'a> From<&'a str> for PokerHand<'a> {
    fn from(source: &'a str) -> Self {
        let mut counter: HashMap<u8, u8> = HashMap::new();
        let mut suits = HashSet::new();
        // 分割每一个牌
        source.split_whitespace().for_each(|s| {
            let (face, suit) = s.split_at(s.len() - 1);
            // 牌面值，全部转成数字，用于比较大小
            let value: u8 = match face {
                "A" => 14,
                "K" => 13,
                "Q" => 12,
                "J" => 11,
                _ => face
                    .parse()
                    .unwrap_or_else(|_| panic!("\"{}\" is not a valid card face", face)),
            };
            // 存储牌数字出现的次数
            *counter.entry(value).or_insert(0) += 1;
            // 存储牌色的种类
            suits.insert(suit);
        });
        // HashMap key-value => (key, value) as an iterator.
        // (牌面值, 出现次数)
        let mut groups: Vec<(u8, u8)> = counter.drain().collect();
        // 根据牌面数字大小，倒序排序
        groups.sort_by_key(|&(value, count)| (Reverse(count), Reverse(value)));

        // determine what rank we have
        // 牌面花色集合
        let counts: Vec<u8> = groups.iter().map(|g| g.1).collect();
        let rank = match counts[..] {
            [1, 1, 1, 1, 1] => {
                // 牌面数字, 5个不一样
                let faces: Vec<u8> = groups.iter().map(|g| g.0).collect();
                let mut result = PokerRank::HighCard;
                // we may have an Ace-first straight 顺子
                // 牌面数字已排序，由大到小
                if faces[..] == [14, 5, 4, 3, 2] {
                    // move the ace
                    groups.remove(0);
                    groups.push((1, 1));
                    result = PokerRank::Straight;
                    // we may have any other straight
                    // ??
                } else if faces
                    .iter()
                    .cloned()
                    .eq((faces[faces.len() - 1]..=faces[0]).rev())
                {
                    result = PokerRank::Straight
                }

                // suit may be involved in the ranking
                if suits.len() == 1 {
                    if result == PokerRank::Straight {
                        result = PokerRank::StraightFlush;
                    } else {
                        result = PokerRank::Flush;
                    }
                }
                result
            }
            [4, 1] => PokerRank::FourOfAKind,
            [3, 2] => PokerRank::FullHouse,
            [3, 1, 1] => PokerRank::ThreeOfAKinid,
            [2, 2, 1] => PokerRank::TwoPair,
            [2, 1, 1, 1] => PokerRank::Pair,
            _ => PokerRank::HighCard,
        };
        Self {
            source,
            groups,
            rank,
        }
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    // unimplemented!("Out of {:?}, which hand wins?", hands)
    let mut ranked: Vec<PokerHand> = hands.iter().map(|h| PokerHand::from(*h)).collect();
    ranked.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Greater));

    ranked
        .iter()
        .filter(|h| ranked[0].eq(h))
        .map(|h| h.source)
        .collect::<Vec<&'a str>>()
}
