

use std::default::Default;
use std::io::{Read, Write, BufRead, BufReader};

#[derive(Debug, PartialEq, Clone)]
pub enum UorbFieldType {
    Bool,
    Char,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Int8,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64,
    Array(Box<UorbFieldType>, usize),
}

impl UorbFieldType {
    pub fn parse_type(s: &str) -> Option<UorbFieldType> {
        use self::UorbFieldType::*;
        match s {
            "bool" => Some(Bool),
            "char" => Some(Char),
            "uint8" => Some(UInt8),
            "uint16" => Some(UInt16),
            "uint32" => Some(UInt32),
            "uint64" => Some(UInt64),
            "int8" => Some(Int8),
            "int16" => Some(Int16),
            "int32" => Some(Int32),
            "int64" => Some(Int64),
            "float32" => Some(Float32),
            "float64" => Some(Float64),
            _ => {
                //Array fields:
                if s.ends_with("]") {
                    let start = s.find("[").unwrap();
                    let size = s[start + 1..(s.len() - 1)].parse::<usize>().unwrap();
                    let mtype = UorbFieldType::parse_type(&s[0..start]).unwrap();
                    Some(Array(Box::new(mtype), size))
                } else {
                    panic!("UNHANDLED {:?}", s);
                }
            }
        }
    }


}

impl Default for UorbFieldType {
    fn default() -> UorbFieldType {
        UorbFieldType::UInt8
    }
}


#[derive(Debug, PartialEq, Clone, Default)]
pub struct UorbMsgField {
    pub uorbtype: UorbFieldType,
    pub name: Option<String>,
    pub description: Option<String>,
    pub const_val: Option<String>,
}

impl UorbMsgField {
    pub fn from_line(desc: String, comment: Option<String>) -> Option<UorbMsgField> {
        let mut field_desc:String ;
        let mut const_val:Option<String> = None;

        let assign_split:Vec<&str> = desc.split("=").collect();
        if assign_split.len() > 1 {
            field_desc = assign_split[0].to_string();
            const_val = Some(assign_split[1].to_string());
        }
        else {
            field_desc = desc;
        }

        println!("field_desc: {:?} const_val: {:?}", field_desc, const_val);

        let toks:Vec<&str> = field_desc.split_whitespace().collect();
        if toks.len() < 2 {
            //invalid field type description
            return None;
        }

        let ftype: UorbFieldType = UorbFieldType::parse_type(toks[0]).unwrap();
        let fname = toks[1];

        Some(
        UorbMsgField {
            uorbtype: ftype,
            name: Some(fname.to_string()),
            description: comment,
            const_val: const_val
        })
    }
}

//
//#[derive(Debug, PartialEq, Clone, Default)]
//pub struct UorbEnumEntry {
//    pub value: Option<i32>,
//    pub name: String,
//    pub description: Option<String>,
//}



#[derive(Debug, PartialEq, Clone, Default)]
pub struct UorbMsg {
    pub name: Option<String>,
    pub description: Option<String>,
    pub fields: Vec<UorbMsgField>,
    pub topics: Vec<String>,
}


impl UorbMsg {
    pub fn from_lines<R: Read>(input: &mut R) -> UorbMsg  {
        let mut msg: UorbMsg = UorbMsg {
            name: None,
            description: None,
            fields: vec![],
            topics: vec![],
        };

        let buf_reader = BufReader::new(input);
        let mut all_topics: Vec<String> = vec![];

        for line in buf_reader.lines() {
            let line = line.unwrap();
            if let Some(_hash_pos) = line.find('#') {
                let comment_split:Vec<&str> = line.split("#").collect();
                match comment_split.len() {
                    1 => { //all comment, as in a TOPICS line
                        let topics_exist:Vec<&str> = comment_split[0].split("TOPICS").collect();
                        if topics_exist.len() > 0 {
                            let topics:Vec<&str> = topics_exist[0].split_whitespace().collect();
                            for topic in topics {
                                all_topics.push(topic.to_string());
                            }
                        }
                        else {
                            println!("Comment? {:?}", line);
                        }
                    },
                    2 => { // half field, half comment
                        let field_desc = comment_split[0].to_string();
                        let comment =  comment_split[1].to_string();
                        if let Some(field) = UorbMsgField::from_line(field_desc, Some(comment)) {
                            msg.fields.push(field);
                        }
                    },
                    _ => {
                        println!("unexpected multi # line: {:?}", line);
                    }
                }
            }
        }

        msg.topics = all_topics;

        msg
    }

}

/// Generate rust representation of uorb message, and corresponding conversion methods
pub fn generate<R: Read, W: Write>(input: &mut R, _output_rust: &mut W) {

    let msg:UorbMsg = UorbMsg::from_lines(input);

    println!("msg: {:?}", msg);


//    let profile = parse_profile(input);
//
//    // rust file
//    let rust_tokens = profile.emit_rust();
//    //writeln!(output_rust, "{}", rust_tokens).unwrap();
//
//    let rust_src = rust_tokens.into_string();
//    let mut cfg = rustfmt::config::Config::default();
//    cfg.set().write_mode(rustfmt::config::WriteMode::Display);
//    rustfmt::format_input(rustfmt::Input::Text(rust_src), &cfg, Some(output_rust)).unwrap();


}