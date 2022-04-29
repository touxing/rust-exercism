// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        // unimplemented!("Revive this player")
        (self.health == 0).then(|| Player {
            health: 100,
            mana: (self.level >= 10).then(|| 100),
            level: self.level,
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        // unimplemented!("Cast a spell of cost {}", mana_cost)
        match self.mana {
            Some(mana) => {
                if mana > mana_cost {
                    self.mana = Some(mana - mana_cost);
                    mana_cost * 2
                } else {
                    0
                }
            }
            None => {
                self.health = self.health.saturating_sub(mana_cost);
                0
            }
        }
    }
}
