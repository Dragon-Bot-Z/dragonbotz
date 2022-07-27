
// lib
    // std
use std::collections::HashMap;


pub struct UniqueId;

impl UniqueId {

    const BASE: [char; 62] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
        'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D',
        'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
        'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
        'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7',
        '8', '9',
    ];

    /// Encodes the unique id
    /// 
    /// ## Arguments:
    /// * id - the id to encode
    pub fn encode(id: &i64) -> String {
        let mut dividend = id.clone();
        let mut code = Vec::<char>::new();
        loop {
            let quotient: i64 = dividend / 62;
            let remainder: i64 = dividend % 62;
    
            code.push(UniqueId::BASE[remainder as usize]);
            if quotient == 0 {
                break;
            }
    
            dividend = quotient;
        }
    
        code.reverse();
        String::from_iter(code)
    }

    /// Decoes the unique id
    /// 
    /// ## Arguments:
    /// * unique_id - the unique id to decode
    pub fn decode(unique_id: &String) -> i64 {
        // convert base to map
        let mut base = HashMap::<char, i8>::new();
        for i in 0..62 {
            base.insert(UniqueId::BASE[i], i as i8);
        }

        let mut value: i64 = 0;
        for letter in unique_id.chars().enumerate() {
            value += (base[&letter.1] as i64) * i64::pow(62, unique_id.len() as u32 - ((letter.0 as u32) + 1));
        }

        value
    }

}
