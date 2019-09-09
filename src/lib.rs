#[macro_use]
extern crate failure;
extern crate num_bigint;

use num_bigint::{BigUint, ToBigUint};
use num_traits::pow::Pow;
use std::fmt;

#[derive(Debug, Fail, PartialEq)]
pub struct KanjiNumberParseError;
impl fmt::Display for KanjiNumberParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A kanji number parse error occured")
    }
}

macro_rules! num {
    ($buf4: ident, $buf1: ident, $x:expr) => {{
        $buf4 += $buf1;
        $buf1 = $x;
    }};
}

macro_rules! base_disit {
    ($buf4: ident, $buf1: ident, $x:expr) => {{
        if $buf1 == 0 {
            $buf1 = 1;
        }
        $buf4 += $buf1 * $x;
        $buf1 = 0;
    }};
}
macro_rules! pow_disit {
    ($buf: ident, $buf4: ident, $buf1: ident, $x:expr) => {{
        $buf4 += $buf1;
        $buf1 = 0;
        $buf += $buf4.to_biguint().unwrap() * 10.to_biguint().unwrap().pow($x as usize);
        $buf4 = 0;
    }};
}


/// 漢数字で書かれた文字列 `String` を `BigUInt` に変換する
///
/// # Examples
///
/// ```
/// use num_bigint::{BigUint, ToBigUint};
/// use kanji_number_parser::{parse, KanjiNumberParseError};
///
/// assert_eq!(
///    parse(String::from("一億五千万")),
///    Ok(150000000.to_biguint().unwrap()) as Result<BigUint, KanjiNumberParseError>
/// );
/// ```
pub fn parse(s: String) -> Result<BigUint, KanjiNumberParseError> {
    let mut buf = 0.to_biguint().unwrap();
    let mut buf1 = 0;
    let mut buf4 = 0;
    let mut pre_c = '_';

    for c in s.chars() {
        match c {
            '零' => num!(buf4, buf1, 0),
            '一' => num!(buf4, buf1, 1),
            '二' => num!(buf4, buf1, 2),
            '三' => num!(buf4, buf1, 3),
            '四' => num!(buf4, buf1, 4),
            '五' => num!(buf4, buf1, 5),
            '六' => num!(buf4, buf1, 6),
            '七' => num!(buf4, buf1, 7),
            '八' => num!(buf4, buf1, 8),
            '九' => num!(buf4, buf1, 9),
            '十' => base_disit!(buf4, buf1, 10),
            '百' => base_disit!(buf4, buf1, 100),
            '千' => base_disit!(buf4, buf1, 1000),
            '万' => pow_disit!(buf, buf4, buf1, 4),
            '億' => pow_disit!(buf, buf4, buf1, 8),
            '兆' => pow_disit!(buf, buf4, buf1, 12),
            '京' => pow_disit!(buf, buf4, buf1, 16),
            '垓' => pow_disit!(buf, buf4, buf1, 20),
            '𥝱' => pow_disit!(buf, buf4, buf1, 24),
            '穣' => pow_disit!(buf, buf4, buf1, 28),
            '溝' => pow_disit!(buf, buf4, buf1, 32),
            '澗' => pow_disit!(buf, buf4, buf1, 36),
            '正' => pow_disit!(buf, buf4, buf1, 40),
            '載' => pow_disit!(buf, buf4, buf1, 44),
            '極' => pow_disit!(buf, buf4, buf1, 48),
            '恒' => {
                if pre_c == '_' {
                    pre_c = '恒';
                } else {
                    return Err(KanjiNumberParseError);
                }
            }
            '河' => {
                if pre_c == '恒' {
                    pre_c = '河';
                } else {
                    return Err(KanjiNumberParseError);
                }
            }
            '沙' => {
                if pre_c == '河' {
                    pre_c = '_';
                    pow_disit!(buf, buf4, buf1, 52);
                } else {
                    return Err(KanjiNumberParseError);
                }
            }
            '阿' => {
                if pre_c == '_' {
                    pre_c = '阿';
                } else {
                    return Err(KanjiNumberParseError);
                }
            }
            '僧' => {
                if pre_c == '阿' {
                    pre_c = '僧';
                } else {
                    return Err(KanjiNumberParseError);
                }
            }
            '祇' => {
                if pre_c == '僧' {
                    pre_c = '_';
                    pow_disit!(buf, buf4, buf1, 56);
                } else {
                    return Err(KanjiNumberParseError);
                }
            }

            '那' => {
                if pre_c == '_' {
                    pre_c = '那';
                } else {
                    return Err(KanjiNumberParseError);
                }
            }
            '由' => {
                if pre_c == '那' {
                    pre_c = '由';
                } else {
                    return Err(KanjiNumberParseError);
                }
            }
            '他' => {
                if pre_c == '由' {
                    pre_c = '_';
                    pow_disit!(buf, buf4, buf1, 60);
                } else {
                    return Err(KanjiNumberParseError);
                }
            }

            '不' => {
                if pre_c == '_' {
                    pre_c = '不';
                } else {
                    return Err(KanjiNumberParseError);
                }
            }
            '可' => {
                if pre_c == '不' {
                    pre_c = '可';
                } else {
                    return Err(KanjiNumberParseError);
                }
            }
            '思' => {
                if pre_c == '可' {
                    pre_c = '思';
                } else {
                    return Err(KanjiNumberParseError);
                }
            }
            '議' => {
                if pre_c == '思' {
                    pre_c = '_';
                    pow_disit!(buf, buf4, buf1, 64);
                } else {
                    return Err(KanjiNumberParseError);
                }
            }

            '無' => {
                if pre_c == '_' {
                    pre_c = '無';
                } else {
                    return Err(KanjiNumberParseError);
                }
            }
            '量' => {
                if pre_c == '無' {
                    pre_c = '量';
                } else {
                    return Err(KanjiNumberParseError);
                }
            }
            '大' => {
                if pre_c == '量' {
                    pre_c = '大';
                } else {
                    return Err(KanjiNumberParseError);
                }
            }
            '数' => {
                if pre_c == '大' {
                    pre_c = '_';
                    pow_disit!(buf, buf4, buf1, 68);
                } else {
                    return Err(KanjiNumberParseError);
                }
            }
            _ => return Err(KanjiNumberParseError),
        };
    }
    buf4 += buf1;
    if let Some(buf4) = buf4.to_biguint() {
        buf += buf4;
    }
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_works() {
        assert_eq!(parse(String::from("零")), Ok(0.to_biguint().unwrap()));
        assert_eq!(
            parse(String::from("千百十")),
            Ok(1110.to_biguint().unwrap()) as Result<BigUint, KanjiNumberParseError>
        );
        assert_eq!(
            parse(String::from("一千万")),
            Ok(10000000.to_biguint().unwrap()) as Result<BigUint, KanjiNumberParseError>
        );
        assert_eq!(
            parse(String::from("四千三百二十一")),
            Ok(4321.to_biguint().unwrap()) as Result<BigUint, KanjiNumberParseError>
        );
        assert_eq!(
            parse(String::from("五千三十")),
            Ok(5030.to_biguint().unwrap()) as Result<BigUint, KanjiNumberParseError>
        );
        assert_eq!(
            parse(String::from("一億五千万")),
            Ok(150000000.to_biguint().unwrap()) as Result<BigUint, KanjiNumberParseError>
        );
        assert_eq!(
            parse(String::from("一億五千万")),
            Ok(BigUint::from_radix_be(&vec![1, 50, 0, 0, 0], 100).unwrap())
                as Result<BigUint, KanjiNumberParseError>
        );
        assert_eq!(
            parse(String::from("一兆五千億")),
            Ok(BigUint::from_radix_be(&vec![1, 50, 0, 0, 0, 0, 0], 100).unwrap())
                as Result<BigUint, KanjiNumberParseError>
        );
        assert_eq!(
            parse(String::from("五千六百七十八溝九千十二穣三千四百五十六𥝱七千八百九十垓千二百三十四京五千六百七十八兆九千十二億三千四百五十六万七千八百九十")),
            Ok(BigUint::from_radix_be(&vec![
                56,78, // 32:溝
                90,12, // 28:穣
                34,56, // 24:𥝱
                78,90, // 20:垓
                12,34, // 16:京
                56,78, // 12:兆
                90,12, //  8:億
                34,56, //  4:万
                78,90
                ], 100).unwrap()) as Result<BigUint, KanjiNumberParseError>
        );
        assert_eq!(
            parse(String::from("十二無量大数三千四百五十六不可思議七千八百九十那由他千二百三十四阿僧祇五千六百七十八恒河沙")),
            Ok(BigUint::from_radix_be(&vec![
                   12, // 68:無量大数
                34,56, // 64:不可思議
                78,90, // 60:那由他
                12,34, // 56:阿僧祇
                56,78, // 52:恒河沙
                 0, 0, // 48:極
                 0, 0, // 44:載
                 0, 0, // 40:正
                 0, 0, // 36:澗
                 0, 0, // 32:溝
                 0, 0, // 28:穣
                 0, 0, // 24:𥝱
                 0, 0, // 20:垓
                 0, 0, // 16:京
                 0, 0, // 12:兆
                 0, 0, //  8:億
                 0, 0, //  4:万
                 0, 0
                ], 100).unwrap()) as Result<BigUint, KanjiNumberParseError>
        );
        assert_eq!(
            parse(String::from("数ではない")),
            Err(KanjiNumberParseError)
        )
    }
}
