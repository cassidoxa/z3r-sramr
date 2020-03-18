use anyhow::{anyhow, Result};
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use std::{collections::HashMap, io::Cursor, str::from_utf8};

struct Z3RStat {
    val: u32,
    repr: String,
}

struct Z3RTime {
    val: u32,
    repr: String,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn parse_sram(sram: &[u8], validate: bool) -> Result<HashMap<&str, String>> {
    if validate == true {
        validate_sram(&sram)?;
    }
    let mut cur = Cursor::new(sram);
    let mut sram_info: HashMap<&str, String> = HashMap::with_capacity(58);

    sram_info.insert("filename", z3rfile_to_unicode(&sram)?);
    sram_info.insert("permalink", get_permalink(&sram));
    let total = get_stat(&mut cur, 0x423, 8, 0, Some(216))?;
    let chests = get_stat(&mut cur, 0x442, 8, 0, None)?;
    sram_info.insert("current rupees", get_stat(&mut cur, 0x362, 16, 0, None)?.repr);
    sram_info.insert("collection rate", total.repr);
    sram_info.insert("chest locations", chests.repr);
    sram_info.insert("other locations", (total.val - chests.val).to_string());
    sram_info.insert("y items", get_stat(&mut cur, 0x421, 5, 3, Some(27))?.repr);
    sram_info.insert("a items", get_stat(&mut cur, 0x421, 3, 0, Some(5))?.repr);
    sram_info.insert("swords", get_stat(&mut cur, 0x422, 3, 5, Some(4))?.repr);
    sram_info.insert("shields", get_stat(&mut cur, 0x422, 2, 3, Some(3))?.repr);
    sram_info.insert("mails", get_stat(&mut cur, 0x424, 2, 6, Some(3))?.repr);
    sram_info.insert("capacity upgrades", get_stat(&mut cur, 0x452, 4, 0, Some(15))?.repr);
    sram_info.insert("heart pieces", get_stat(&mut cur, 0x448, 4, 0, Some(24))?.repr);
    sram_info.insert("heart containers", get_stat(&mut cur, 0x429, 4, 4, Some(11))?.repr);
    sram_info.insert("maps", get_stat(&mut cur, 0x428, 4, 4, Some(12))?.repr);
    sram_info.insert("compasses", get_stat(&mut cur, 0x428, 4, 0, Some(11))?.repr);
    sram_info.insert("small keys", get_stat(&mut cur, 0x424, 6, 0, Some(61))?.repr);
    sram_info.insert("big keys", get_stat(&mut cur, 0x427, 4, 4, Some(12))?.repr);
    sram_info.insert("big chests", get_stat(&mut cur, 0x427, 4, 0, Some(11))?.repr);
    sram_info.insert("pendants", get_stat(&mut cur, 0x429, 2, 0, Some(3))?.repr);
    sram_info.insert("crystals", get_stat(&mut cur, 0x422, 3, 0, Some(7))?.repr);
    sram_info.insert("hyrule castle", get_stat(&mut cur, 0x434, 4, 4, Some(8))?.repr);
    sram_info.insert("eastern palace", get_stat(&mut cur, 0x436, 3, 0, Some(6))?.repr);
    sram_info.insert("desert palace", get_stat(&mut cur, 0x435, 3, 5, Some(6))?.repr);
    sram_info.insert("tower of hera", get_stat(&mut cur, 0x435, 3, 2, Some(5))?.repr);
    sram_info.insert("castle tower", get_stat(&mut cur, 0x435, 2, 0, Some(2))?.repr);
    sram_info.insert("palace of darkness", get_stat(&mut cur, 0x434, 4, 0, Some(14))?.repr);
    sram_info.insert("swamp palace", get_stat(&mut cur, 0x439, 4, 0, Some(10))?.repr);
    sram_info.insert("skull woods", get_stat(&mut cur, 0x437, 4, 4, Some(8))?.repr);
    sram_info.insert("thieves town", get_stat(&mut cur, 0x437, 4, 0, Some(8))?.repr);
    sram_info.insert("ice palace", get_stat(&mut cur, 0x438, 4, 4, Some(8))?.repr);
    sram_info.insert("misery mire", get_stat(&mut cur, 0x438, 4, 0, Some(8))?.repr);
    sram_info.insert("turtle rock", get_stat(&mut cur, 0x439, 4, 4, Some(12))?.repr);
    sram_info.insert("ganons tower", get_stat(&mut cur, 0x436, 5, 3, Some(27))?.repr);
    sram_info.insert("ganons tower big key", get_stat(&mut cur, 0x42A, 5, 0, Some(22))?.repr);
    sram_info.insert("swordless bosses", get_stat(&mut cur, 0x452, 4, 4, Some(13))?.repr);
    sram_info.insert("fighter sword bosses", get_stat(&mut cur, 0x425, 4, 12, Some(13))?.repr);
    sram_info.insert("master sword bosses", get_stat(&mut cur, 0x425, 4, 8, Some(13))?.repr);
    sram_info.insert("tempered sword bosses", get_stat(&mut cur, 0x425, 4, 4, Some(13))?.repr);
    sram_info.insert("golden sword bosses", get_stat(&mut cur, 0x425, 4, 0, Some(13))?.repr);
    sram_info.insert("locations pre boots", get_stat(&mut cur, 0x432, 8, 0, None)?.repr);
    sram_info.insert("locations pre mirror", get_stat(&mut cur, 0x433, 8, 0, None)?.repr);
    sram_info.insert("bonks", get_stat(&mut cur, 0x420, 8, 0, None)?.repr);
    sram_info.insert("overworld mirrors", get_stat(&mut cur, 0x43A, 8, 0, None)?.repr);
    sram_info.insert("underworld mirrors", get_stat(&mut cur, 0x43B, 8, 0, None)?.repr);
    sram_info.insert("times fluted", get_stat(&mut cur, 0x44B, 8, 0, None)?.repr);
    sram_info.insert("screen transitions", get_stat(&mut cur, 0x43C, 16, 0, None)?.repr);
    sram_info.insert("rupees spent", get_stat(&mut cur, 0x42B, 16, 0, None)?.repr);
    sram_info.insert("save and quits", get_stat(&mut cur, 0x42D, 8, 0, None)?.repr);
    sram_info.insert("deaths", get_stat(&mut cur, 0x449, 8, 0, None)?.repr);
    let total_time = Z3RTime::new(&mut cur, 0x43E);
    let menu_time = Z3RTime::new(&mut cur, 0x444);
    let loop_time = Z3RTime::new(&mut cur, 0x42E);
    sram_info.insert("total time", total_time.repr);
    sram_info.insert("menu time", menu_time.repr);
    sram_info.insert("lag time", Z3RTime::from_frames(total_time.val - loop_time.val).repr);
    sram_info.insert("first sword", Z3RTime::new(&mut cur, 0x458).repr);
    sram_info.insert("boots found", Z3RTime::new(&mut cur, 0x45C).repr);
    sram_info.insert("flute found", Z3RTime::new(&mut cur, 0x460).repr);
    sram_info.insert("mirror found", Z3RTime::new(&mut cur, 0x464).repr);
    sram_info.insert("faerie revivals", get_stat(&mut cur, 0x453, 8, 0, None)?.repr);

    Ok(sram_info)
}

pub fn validate_sram(sram: &[u8]) -> Result<()> {
    let mut cur = Cursor::new(sram);

    // Check the length. 32768 bytes as of v30.0.4, 2019-11-15 ROM build
    if sram.len() != 32768 {
        return Err(anyhow!("Validation Error: Unexpected file size"));
    }
    // Check the checksum validity value and the rando-specific file marker
    // 0x55AA and 0xFF
    let checksum_validity: u16 = LittleEndian::read_u16(&sram[0x3E1..0x3E3]);
    if checksum_validity != 0x55AA || sram[0x4F0] != 0xFF {
        return Err(anyhow!("Validation Error: Invalid file"));
    }

    // Check the first two characters of the rom name for VT or ER
    let valid_names: [&str; 2] = ["VT", "ER"];
    let rom_name = &sram[0x2000..0x2002];
    if valid_names
        .iter()
        .any(|&x| x == from_utf8(&rom_name).unwrap())
        == false
    {
        return Err(anyhow!("Validation Error: Invalid ROM name"));
    }

    // Now we check the SRAM's own "inverse" checksum
    let mut checksum = 0u16;
    cur.set_position(0x00);
    while cur.position() < 0x4FE {
        let bytes = cur.read_u16::<LittleEndian>()?;
        checksum = checksum.overflowing_add(bytes).0;
    }
    let expected_inv_checksum = 0x5A5Au16.overflowing_sub(checksum).0;
    let inv_checksum: u16 = LittleEndian::read_u16(&sram[0x4FE..0x500]);
    if inv_checksum != expected_inv_checksum {
        return Err(anyhow!("Validation Error: Invalid checksum"));
    }

    return Ok(());
}

fn bitmask(bits: u32) -> u32 {
    (1u32 << bits) - 1u32
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
    let mut val = match bytes as u8 {
        1 => cur.read_u8().unwrap() as u32,
        2 => cur.read_u16::<LittleEndian>().unwrap() as u32,
        _ => return Err(anyhow!("Error reading value: {}", offset)),
    };
    val >>= shift;
    val &= bitmask(bits);

    let repr: String = match max {
        Some(max) => format!("{}/{}", val, max),
        None => val.to_string(),
    };

    Ok(Z3RStat { val, repr })
}

impl Z3RTime {
    fn new(cur: &mut Cursor<&[u8]>, offset: u64) -> Self {
        cur.set_position(offset);
        // time is stored as number of frames, in a 32 bit integer
        let val = cur.read_u32::<LittleEndian>().unwrap();
        let hours: u32 = val / (216000u32);
        let mut rem = val % 216000u32;
        let minutes: u32 = rem / 3600u32;
        rem %= 3600u32;
        let seconds: u32 = rem / 60u32;
        rem %= 60u32;

        let repr = format!("{:0>2}:{:0>2}:{:0>2}.{:0>2}", hours, minutes, seconds, rem);

        Z3RTime { val, repr }
    }

    fn from_frames(frames: u32) -> Self {
        let val = frames;
        let hours: u32 = val / (216000u32);
        let mut rem = val % 216000u32;
        let minutes: u32 = rem / 3600u32;
        rem %= 3600u32;
        let seconds: u32 = rem / 60u32;
        rem %= 60u32;

        let repr = format!("{:0>2}:{:0>2}:{:0>2}.{:0>2}", hours, minutes, seconds, rem);

        Z3RTime { val, repr }
    }
}

fn z3rfile_to_unicode(sram: &[u8]) -> Result<String> {
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

    Ok(file_name)
}

fn get_permalink(sram: &[u8]) -> String {
    let rom_name = &sram[0x2000..0x2002];
    match from_utf8(&rom_name).unwrap() {
        "VT" => {
            let hash_id = from_utf8(&sram[0x2003..0x200D]).unwrap();
            return format!("https://alttpr.com/h/{}", hash_id);
        }
        _ => return "none".to_string(),
    };
}
