use anyhow::{anyhow, Result};
use byteorder::{LittleEndian, ReadBytesExt};
use std::{collections::HashMap, fmt, io::Cursor};

use crate::{bitmask, validate_sram};

pub enum Z3REquip {
    Has(bool),
    Number(u32),
}

impl Z3REquip {
    pub fn has(&self) -> bool {
        match self {
            Self::Has(b) => *b,
            Self::Number(n) => {
                if *n > 0 {
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn value(&self) -> u32 {
        match self {
            Self::Has(b) => {
                if *b == true {
                    1u32
                } else {
                    0u32
                }
            }
            Self::Number(n) => *n,
        }
    }
}

impl fmt::Display for Z3REquip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Has(true) => write!(f, "True"),
            Self::Has(false) => write!(f, "False"),
            Self::Number(n) => write!(f, "{}", *n),
        }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn read_equipment(sram: &[u8], validate: bool) -> Result<HashMap<&str, Z3REquip>> {
    if validate == true {
        validate_sram(&sram)?;
    }
    let mut cur = Cursor::new(sram);
    let mut sram_equip: HashMap<&str, Z3REquip> = HashMap::with_capacity(25);
    sram_equip.insert("current rupees", get_equipment(&mut cur, 0x362, 16, 0, false)?);
    sram_equip.insert("current arrows", get_equipment(&mut cur, 0x377, 8, 0, false)?);
    sram_equip.insert("current bombs", get_equipment(&mut cur, 0x343, 8, 0, false)?);
    sram_equip.insert("current health", get_equipment(&mut cur, 0x36D, 8, 0, false)?);
    sram_equip.insert("current magic", get_equipment(&mut cur, 0x36E, 8, 0, false)?);
    sram_equip.insert("magic consumption", get_equipment(&mut cur, 0x37B, 8, 0, false)?);
    sram_equip.insert("goal items", get_equipment(&mut cur, 0x418, 8, 0, false)?);
    sram_equip.insert("bomb upgrades", get_equipment(&mut cur, 0x370, 8, 0, false)?);
    sram_equip.insert("arrow upgrades", get_equipment(&mut cur, 0x371, 8, 0, false)?);
    sram_equip.insert("fire rod", get_equipment(&mut cur, 0x345, 8, 0, true)?);
    sram_equip.insert("ice rod", get_equipment(&mut cur, 0x346, 8, 0, true)?);
    sram_equip.insert("bombos", get_equipment(&mut cur, 0x347, 8, 0, true)?);
    sram_equip.insert("ether", get_equipment(&mut cur, 0x348, 8, 0, true)?);
    sram_equip.insert("quake", get_equipment(&mut cur, 0x349, 8, 0, true)?);
    sram_equip.insert("lamp", get_equipment(&mut cur, 0x34A, 8, 0, true)?);
    sram_equip.insert("hammer", get_equipment(&mut cur, 0x34B, 8, 0, true)?);
    sram_equip.insert("hookshot", get_equipment(&mut cur, 0x342, 8, 0, true)?);
    sram_equip.insert("bug net", get_equipment(&mut cur, 0x34D, 8, 0, true)?);
    sram_equip.insert("book", get_equipment(&mut cur, 0x34E, 8, 0, true)?);
    sram_equip.insert("somaria", get_equipment(&mut cur, 0x350, 8, 0, true)?);
    sram_equip.insert("byrna", get_equipment(&mut cur, 0x351, 8, 0, true)?);
    sram_equip.insert("cape", get_equipment(&mut cur, 0x352, 8, 0, true)?);
    sram_equip.insert("mirror", get_equipment(&mut cur, 0x353, 8, 0, false)?);
    sram_equip.insert("gloves", get_equipment(&mut cur, 0x353, 8, 0, false)?);
    sram_equip.insert("boots", get_equipment(&mut cur, 0x355, 8, 0, true)?);
    sram_equip.insert("flippers", get_equipment(&mut cur, 0x356, 8, 0, true)?);
    sram_equip.insert("moon pearl", get_equipment(&mut cur, 0x357, 8, 0, true)?);
    sram_equip.insert("sword", get_equipment(&mut cur, 0x359, 8, 0, false)?);
    sram_equip.insert("shield", get_equipment(&mut cur, 0x35A, 8, 0, false)?);
    sram_equip.insert("mail", get_equipment(&mut cur, 0x35B, 8, 0, false)?);

    let bottle_1 = get_equipment(&mut cur, 0x35C, 8, 0, false)?;
    let bottle_2 = get_equipment(&mut cur, 0x35D, 8, 0, false)?;
    let bottle_3 = get_equipment(&mut cur, 0x35E, 8, 0, false)?;
    let bottle_4 = get_equipment(&mut cur, 0x35F, 8, 0, false)?;
    let bottle_count: u32 = vec![bottle_1.value(), bottle_2.value(), bottle_3.value(), bottle_4.value()]
        .iter()
        .fold(0u32, |c, b| if *b != 0u32 {c + 1u32} else {c});
    sram_equip.insert("bottle 1", bottle_1);
    sram_equip.insert("bottle 2", bottle_2);
    sram_equip.insert("bottle 3", bottle_3);
    sram_equip.insert("bottle 4", bottle_4);
    sram_equip.insert("bottles", Z3REquip::Number(bottle_count));

    let mushroom_current = get_equipment(&mut cur, 0x38C, 1, 5, true)?;
    let mushroom_past = get_equipment(&mut cur, 0x38C, 1, 3, true)?;
    let flute_inactive = get_equipment(&mut cur, 0x38C, 1, 1, true)?;
    let flute_active = get_equipment(&mut cur, 0x38C, 1, 0, true)?;
    let bow = get_equipment(&mut cur, 0x38E, 1, 7, true)?;
    let silver_bow = get_equipment(&mut cur, 0x38E, 1, 6, true)?;
    let second_prog_bow = get_equipment(&mut cur, 0x38E, 1, 5, true)?;
    sram_equip.insert("bow", Z3REquip::Has(bow.has() || silver_bow.has()));
    sram_equip.insert("silver arrows", Z3REquip::Has(silver_bow.has() || second_prog_bow.has()));
    sram_equip.insert("mushroom", Z3REquip::Has(mushroom_current.has() || mushroom_past.has()));
    sram_equip.insert("mushroom turned in", Z3REquip::Has(mushroom_past.has()));
    sram_equip.insert("flute", Z3REquip::Has(flute_inactive.has() || flute_active.has()));
    sram_equip.insert("blue boomerang", get_equipment(&mut cur, 0x38C, 1, 7, true)?);
    sram_equip.insert("red boomerang", get_equipment(&mut cur, 0x38C, 1, 6, true)?);
    sram_equip.insert("powder", get_equipment(&mut cur, 0x38C, 1, 4, true)?);
    sram_equip.insert("shovel", get_equipment(&mut cur, 0x38C, 1, 2, true)?);

    sram_equip.insert("green pendant", get_equipment(&mut cur, 0x374, 1, 2, true)?);
    sram_equip.insert("blue pendant", get_equipment(&mut cur, 0x374, 1, 1, true)?);
    sram_equip.insert("red pendant", get_equipment(&mut cur, 0x374, 1, 0, true)?);
    sram_equip.insert("crystal 1", get_equipment(&mut cur, 0x37A, 1, 1, true)?);
    sram_equip.insert("crystal 2", get_equipment(&mut cur, 0x37A, 1, 4, true)?);
    sram_equip.insert("crystal 3", get_equipment(&mut cur, 0x37A, 1, 6, true)?);
    sram_equip.insert("crystal 4", get_equipment(&mut cur, 0x37A, 1, 5, true)?);
    sram_equip.insert("crystal 5", get_equipment(&mut cur, 0x37A, 1, 2, true)?);
    sram_equip.insert("crystal 6", get_equipment(&mut cur, 0x37A, 1, 0, true)?);
    sram_equip.insert("crystal 7", get_equipment(&mut cur, 0x37A, 1, 3, true)?);
    
    Ok(sram_equip)
}

fn get_equipment(
    cur: &mut Cursor<&[u8]>,
    offset: u64,
    bits: u32,
    shift: u32,
    boolean: bool,
) -> Result<Z3REquip> {
    cur.set_position(offset);
    let bytes: f32 = ((bits as f32 + shift as f32) / 8f32).ceil();
    let mut value = match bytes as u8 {
        1 => cur.read_u8().unwrap() as u32,
        2 => cur.read_u16::<LittleEndian>().unwrap() as u32,
        _ => return Err(anyhow!("Tried reading more than two bytes at {}", offset)),
    };
    value >>= shift;
    value &= bitmask(bits);

    match boolean {
        true => match value {
            0 => Ok(Z3REquip::Has(false)),
            1 => Ok(Z3REquip::Has(true)),
            _ => Err(anyhow!("Expected boolean equipment value")),
        },
        false => Ok(Z3REquip::Number(value)),
    }
}

pub fn map_magic_consumption(v: u32) -> String {
    match v {
        0 => "Normal Magic".to_string(),
        1 => "1/2 Magic".to_string(),
        2 => "1/4 Magic".to_string(),
        _ => "Unknown Magic Consumption".to_string(),
    }
}

pub fn map_sword(v: u32) -> Option<String> {
    match v {
        0 => None,
        1 => Some("Fighter's Sword".to_string()),
        2 => Some("Master Sword".to_string()),
        3 => Some("Tempered Sword".to_string()),
        4 => Some("Gold Sword".to_string()),
        255 => Some("Swordless".to_string()),
        _ => Some("Unknown Sword".to_string()),
    }
}

pub fn map_shield(v: u32) -> Option<String> {
    match v {
        0 => None,
        1 => Some("Blue Shield".to_string()),
        2 => Some("Red Shield".to_string()),
        3 => Some("Mirror Shield".to_string()),
        _ => Some("Unknown Shield".to_string()),
    }
}

pub fn map_mail(v: u32) -> String {
    match v {
        0 => "Green Mail".to_string(),
        1 => "Blue Mail".to_string(),
        2 => "Red Mail".to_string(),
        _ => "Unknown Mail".to_string(),
    }
}

pub fn map_gloves(v: u32) -> Option<String> {
    match v {
        0 => None,
        1 => Some("Power Gloves".to_string()),
        2 => Some("Titan's Mitts".to_string()),
        _ => Some("Unknown Gloves".to_string()),
    }
}

pub fn map_mirror(v: u32) -> Option<String> {
    match v {
        0 => None,
        1 => Some("Mirror Scroll".to_string()),
        2 => Some("Magic Mirror".to_string()),
        _ => Some("Unknown Mirror".to_string()),
    }
}

pub fn map_bottle_contents(v: u32) -> Option<String> {
    match v {
        0 => None,
        1 => Some("Mushroom".to_string()),
        2 => Some("Empty Bottle".to_string()),
        3 => Some("Red Potion".to_string()),
        4 => Some("Green Potion".to_string()),
        5 => Some("Blue Potion".to_string()),
        6 => Some("Fairy".to_string()),
        7 => Some("Bee".to_string()),
        8 => Some("Good Bee".to_string()),
        _ => Some("Unknown Bottle".to_string()),
    }
}
