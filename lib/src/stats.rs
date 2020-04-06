use anyhow::{anyhow, Result};
use byteorder::{LittleEndian, ReadBytesExt};
use std::{collections::HashMap, convert::TryFrom, fmt, io::Cursor, str::from_utf8};

use crate::{bitmask, validate_sram};

pub enum Z3RStat {
    Meta(Option<String>),
    Number(u32),
    Fraction(String),
    Time(String),
}

impl Z3RStat {
    fn new_time<T: Into<u64>>(cur: Option<&mut Cursor<&[u8]>>, num: T) -> Self {
        let value: u32 = match cur {
            Some(cur) => {
                cur.set_position(num.into());
                cur.read_u32::<LittleEndian>().unwrap()
            }
            None => num.into() as u32,
        };
        let hours: u32 = value / (216000u32);
        let mut rem = value % 216000u32;
        let minutes: u32 = rem / 3600u32;
        rem %= 3600u32;
        let seconds: u32 = rem / 60u32;
        rem %= 60u32;

        let time = format!("{:0>2}:{:0>2}:{:0>2}.{:0>2}", hours, minutes, seconds, rem);

        Z3RStat::Time(time)
    }
}

impl fmt::Display for Z3RStat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Meta(Some(m)) => write!(f, "{}", *m),
            Self::Meta(None) => write!(f, "none"),
            Self::Number(n) => write!(f, "{}", *n),
            Self::Fraction(n) => write!(f, "{}", *n),
            Self::Time(t) => write!(f, "{}", *t),
        }
    }
}

impl TryFrom<&Z3RStat> for u32 {
    type Error = anyhow::Error;

    fn try_from(stat: &Z3RStat) -> Result<Self> {
        match stat {
            Z3RStat::Number(n) => Ok(*n),
            Z3RStat::Time(t) => {
                let time: Vec<u32> = t
                    .split(|c: char| c.is_numeric() == false)
                    .map(|x| u32::from_str_radix(x, 10).unwrap())
                    .collect();
                Ok((time[0] * 216000u32) + (time[1] * 3600u32) + (time[2] * 60) + time[3])
            }
            Z3RStat::Fraction(f) => {
                let fraction: Vec<u32> = f
                    .split("/")
                    .map(|x| u32::from_str_radix(x, 10).unwrap())
                    .collect();
                Ok(fraction[0])
            }
            _ => Err(anyhow!("Can't convert non-numeric Z3Rstat to u32")),
        }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn read_stats(sram: &[u8], validate: bool) -> Result<HashMap<&str, Z3RStat>> {
    if validate == true {
        validate_sram(&sram)?;
    }
    let mut cur = Cursor::new(sram);
    let mut sram_stats: HashMap<&str, Z3RStat> = HashMap::with_capacity(58);  
    
    let hash_id: Z3RStat = get_hash_id(&sram);
    match hash_id {
        Z3RStat::Meta(Some(id)) => {
            sram_stats.insert("hash id", Z3RStat::Meta(Some(id.clone())));
            sram_stats.insert("permalink", Z3RStat::Meta(Some(format!("https://alttpr.com/h/{}", id))));
        },
        Z3RStat::Meta(None) => {
            sram_stats.insert("hash id", Z3RStat::Meta(None));
            sram_stats.insert("permalink", Z3RStat::Meta(None));
        },
        _ => (),
    };
    let file_name = match z3rfile_to_unicode(&sram) {
        Ok(name) => name,
        Err(_) => return Err(anyhow!("Invalid File Name")),
    };
    sram_stats.insert("filename", file_name);
    let total = get_stat(&mut cur, 0x423, 8, 0, Some(216))?;
    let chests = get_stat(&mut cur, 0x442, 8, 0, None)?;
    sram_stats.insert("other locations", Z3RStat::Number(u32::try_from(&total)? - u32::try_from(&chests)?));
    sram_stats.insert("collection rate", total);
    sram_stats.insert("chest locations", chests);
    sram_stats.insert("y items", get_stat(&mut cur, 0x421, 5, 3, Some(27))?);
    sram_stats.insert("a items", get_stat(&mut cur, 0x421, 3, 0, Some(5))?);
    sram_stats.insert("swords", get_stat(&mut cur, 0x422, 3, 5, Some(4))?);
    sram_stats.insert("shields", get_stat(&mut cur, 0x422, 2, 3, Some(3))?);
    sram_stats.insert("mails", get_stat(&mut cur, 0x424, 2, 6, Some(3))?);
    sram_stats.insert("capacity upgrades", get_stat(&mut cur, 0x452, 4, 0, Some(15))?);
    sram_stats.insert("heart containers", get_stat(&mut cur, 0x429, 4, 4, Some(11))?);
    sram_stats.insert("heart pieces", get_stat(&mut cur, 0x448, 8, 0, Some(24))?);
    sram_stats.insert("maps", get_stat(&mut cur, 0x428, 4, 4, Some(12))?);
    sram_stats.insert("compasses", get_stat(&mut cur, 0x428, 4, 0, Some(11))?);
    sram_stats.insert("small keys", get_stat(&mut cur, 0x424, 6, 0, Some(61))?);
    sram_stats.insert("big keys", get_stat(&mut cur, 0x427, 4, 4, Some(12))?);
    sram_stats.insert("big chests", get_stat(&mut cur, 0x427, 4, 0, Some(11))?);
    sram_stats.insert("pendants", get_stat(&mut cur, 0x429, 2, 0, Some(3))?);
    sram_stats.insert("crystals", get_stat(&mut cur, 0x422, 3, 0, Some(7))?);
    sram_stats.insert("hyrule castle", get_stat(&mut cur, 0x434, 4, 4, Some(8))?);
    sram_stats.insert("eastern palace", get_stat(&mut cur, 0x436, 3, 0, Some(6))?);
    sram_stats.insert("desert palace", get_stat(&mut cur, 0x435, 3, 5, Some(6))?);
    sram_stats.insert("tower of hera", get_stat(&mut cur, 0x435, 3, 2, Some(5))?);
    sram_stats.insert("castle tower", get_stat(&mut cur, 0x435, 2, 0, Some(2))?);
    sram_stats.insert("palace of darkness", get_stat(&mut cur, 0x434, 4, 0, Some(14))?);
    sram_stats.insert("swamp palace", get_stat(&mut cur, 0x439, 4, 0, Some(10))?);
    sram_stats.insert("skull woods", get_stat(&mut cur, 0x437, 4, 4, Some(8))?);
    sram_stats.insert("thieves town", get_stat(&mut cur, 0x437, 4, 0, Some(8))?);
    sram_stats.insert("ice palace", get_stat(&mut cur, 0x438, 4, 4, Some(8))?);
    sram_stats.insert("misery mire", get_stat(&mut cur, 0x438, 4, 0, Some(8))?);
    sram_stats.insert("turtle rock", get_stat(&mut cur, 0x439, 4, 4, Some(12))?);
    sram_stats.insert("ganons tower", get_stat(&mut cur, 0x436, 5, 3, Some(27))?);
    sram_stats.insert("ganons tower big key", get_stat(&mut cur, 0x42A, 5, 0, Some(22))?);
    sram_stats.insert("swordless bosses", get_stat(&mut cur, 0x452, 4, 4, Some(13))?);
    sram_stats.insert("fighter sword bosses", get_stat(&mut cur, 0x426, 4, 4, Some(13))?);
    sram_stats.insert("master sword bosses", get_stat(&mut cur, 0x426, 4, 0, Some(13))?);
    sram_stats.insert("tempered sword bosses", get_stat(&mut cur, 0x425, 4, 4, Some(13))?);
    sram_stats.insert("golden sword bosses", get_stat(&mut cur, 0x425, 4, 0, Some(13))?);
    sram_stats.insert("locations pre boots", get_stat(&mut cur, 0x432, 8, 0, None)?);
    sram_stats.insert("locations pre mirror", get_stat(&mut cur, 0x433, 8, 0, None)?);
    sram_stats.insert("bonks", get_stat(&mut cur, 0x420, 8, 0, None)?);
    sram_stats.insert("overworld mirrors", get_stat(&mut cur, 0x43A, 8, 0, None)?);
    sram_stats.insert("underworld mirrors", get_stat(&mut cur, 0x43B, 8, 0, None)?);
    sram_stats.insert("times fluted", get_stat(&mut cur, 0x44B, 8, 0, None)?);
    sram_stats.insert("screen transitions", get_stat(&mut cur, 0x43C, 16, 0, None)?);
    sram_stats.insert("rupees spent", get_stat(&mut cur, 0x42B, 16, 0, None)?);
    sram_stats.insert("save and quits", get_stat(&mut cur, 0x42D, 8, 0, None)?);
    sram_stats.insert("deaths", get_stat(&mut cur, 0x449, 8, 0, None)?);
    let total_time = Z3RStat::new_time(Some(&mut cur), 0x43Eu64);
    let menu_time = Z3RStat::new_time(Some(&mut cur), 0x444u64);
    let loop_time = Z3RStat::new_time(Some(&mut cur), 0x42Eu64);
    sram_stats.insert("lag time", Z3RStat::new_time(None, u32::try_from(&total_time)? - u32::try_from(&loop_time)?));
    sram_stats.insert("total time", total_time);
    sram_stats.insert("menu time", menu_time);
    sram_stats.insert("first sword", Z3RStat::new_time(Some(&mut cur), 0x458u64));
    sram_stats.insert("boots found", Z3RStat::new_time(Some(&mut cur), 0x45Cu64));
    sram_stats.insert("flute found", Z3RStat::new_time(Some(&mut cur), 0x460u64));
    sram_stats.insert("mirror found", Z3RStat::new_time(Some(&mut cur), 0x464u64));
    sram_stats.insert("faerie revivals", get_stat(&mut cur, 0x453, 8, 0, None)?);

    Ok(sram_stats)
}

fn get_stat(
    cur: &mut Cursor<&[u8]>,
    offset: u64,
    bits: u32,
    shift: u32,
    max: Option<u8>,
) -> Result<Z3RStat> {
    cur.set_position(offset);
    let bytes: f32 = ((bits as f32 + shift as f32) / 8f32).ceil();
    let mut value = match bytes as u8 {
        1 => cur.read_u8().unwrap() as u32,
        2 => cur.read_u16::<LittleEndian>().unwrap() as u32,
        _ => return Err(anyhow!("Tried reading more than two bytes at {}", offset)),
    };
    value >>= shift;
    value &= bitmask(bits);

    match max {
        Some(max) => Ok(Z3RStat::Fraction(format!("{}/{}", value, max))),
        None => Ok(Z3RStat::Number(value)),
    }
}

fn z3rfile_to_unicode(sram: &[u8]) -> Result<Z3RStat> {
    const NAME_ENCODING: [&str; 207] = [
        "あ", "い", "う", "え", "お", "や", "ゆ", "よ", "か", "き", "く", "け", "こ", "わ", "を",
        "ん", "さ", "し", "す", "せ", "そ", "が", "ぎ", "ぐ", "た", "ち", "つ", "て", "と", "げ",
        "ご", "ざ", "な", "に", "ぬ", "ね", "の", "じ", "ず", "ぜ", "は", "ひ", "ふ", "へ", "ほ",
        "ぞ", "だ", "ぢ", "ま", "み", "む", "め", "も", "づ", "で", "ど", "ら", "り", "る", "れ",
        "ろ", "ば", "び", "ぶ", "べ", "ぼ", "ぱ", "ぴ", "ぷ", "ぺ", "ぽ", "ゃ", "ゅ", "ょ", "っ",
        "ぁ", "ぃ", "ぅ", "ぇ", "ぉ", "ア", "イ", "ウ", "エ", "オ", "ヤ", "ユ", "ヨ", "カ", "キ",
        "ク", "ケ", "コ", "ワ", "ヲ", "ン", "サ", "シ", "ス", "セ", "ソ", "ガ", "ギ", "グ", "タ",
        "チ", "ツ", "テ", "ト", "ゲ", "ゴ", "ザ", "ナ", "ニ", "ヌ", "ネ", "ノ", "ジ", "ズ", "ゼ",
        "ハ", "ヒ", "フ", "ヘ", "ホ", "ゾ", "ダ", "ヂ", "マ", "ミ", "ム", "メ", "モ", "ヅ", "デ",
        "ド", "ラ", "リ", "ル", "レ", "ロ", "バ", "ビ", "ブ", "ベ", "ボ", "パ", "ピ", "プ", "ペ",
        "ポ", "ャ", "ュ", "ョ", "ッ", "ァ", "ィ", "ゥ", "ェ", "ォ", "0", "1", "2", "3", "4", "5",
        "6", "7", "8", "9", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N",
        "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "「", "」", "?", "!", ",", "-",
        "<", ">", " ", "。", "~",
    ];
    let mut file_name = String::with_capacity(36); // Avoid re-allocation w/ multi byte characters
    let mut cur = Cursor::new(sram);
    cur.set_position(0x3d9);
    for _ in 0..4 {
        let character = cur.read_u16::<LittleEndian>()?;
        let char_index = (character & 0xF) | ((character >> 1) & 0xF0);
        file_name.push_str(NAME_ENCODING[char_index as usize]);
    }
    cur.set_position(0x500);
    for _ in 0..8 {
        let character = cur.read_u16::<LittleEndian>()?;
        let char_index = (character & 0xF) | ((character >> 1) & 0xF0);
        file_name.push_str(NAME_ENCODING[char_index as usize]);
    }

    Ok(Z3RStat::Meta(Some(file_name)))
}

fn get_hash_id(sram: &[u8]) -> Z3RStat {
    let rom_name = &sram[0x2000..0x2002];
    match from_utf8(&rom_name).unwrap() {
        "VT" => {
            let hash_id = from_utf8(&sram[0x2003..0x200D]).unwrap();
            return Z3RStat::Meta(Some(hash_id.to_string()));
        }
        _ => return Z3RStat::Meta(None),
    };
}
