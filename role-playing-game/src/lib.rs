// This stub file contains items which aren't used yet; feel free to remove this module attribute
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
        if self.health > 0 {
            return None;
        }
        Some(Self {
            health: 100,
            level: self.level,
            mana: if self.level >= 10 { Some(100) } else { None },
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        // unimplemented!("Cast a spell of cost {}", mana_cost)
        if self.mana == None {
            self.health = match self.health < mana_cost {
                true => 0,
                false => self.health - mana_cost
            };
            return 0;
        }
        if self.mana.unwrap() < mana_cost {
            return 0;
        }
        self.mana = Some(self.mana.unwrap() - mana_cost);
        mana_cost * 2
    }
}
