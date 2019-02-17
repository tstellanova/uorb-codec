
extern crate proc_macro2;



use quote::{ ToTokens, TokenStreamExt};
use proc_macro2::{Ident, Span, TokenStream};

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
            _ => { //Array fields
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


    /// Return rust equivalent of a given UorbFieldType
    pub fn rust_type(&self) -> String {
        use self::UorbFieldType::*;
        match self.clone() {
            Bool => "bool".into(),
            Char => "char".into(),
            UInt8 => "u8".into(),
            Int8 => "i8".into(),
            UInt16 => "u16".into(),
            Int16 => "i16".into(),
            UInt32 => "u32".into(),
            Int32 => "i32".into(),
            Float32 => "f32".into(),
            UInt64 => "u64".into(),
            Int64 => "i64".into(),
            Float64 => "f64".into(),
            Array(t, size) => {
                if size > 32 {
                    // we have to use a vector to make our lives easier
                    format!("Vec<{}> /* {} elements */", t.rust_type(), size)
                } else {
                    // use a slice, as Rust derives lot of things for slices <= 32 elements
                    format!("[{};{}]", t.rust_type(), size)
                }
            },
        }
    }

    /// Encoded size of a given type, in bytes
    fn encoded_len(&self) -> usize {
        use self::UorbFieldType::*;
        match self.clone() {
            UInt8 | Int8 | Char | Bool => 1,
            UInt16 | Int16 => 2,
            UInt32 | Int32 | Float32 => 4,
            UInt64 | Int64 | Float64 => 8,
            Array(t, size) => t.encoded_len() * size,
        }
    }


    /// Emit writer of a given type
    pub fn rust_writer(&self, name: String, buf_name: String) -> TokenStream {
        use self::UorbFieldType::*;
        let val:TokenStream = name.parse().unwrap();
        let buf:TokenStream = buf_name.parse().unwrap();

        match self.clone() {
            Bool => quote!{#buf.put_u8(#val as u8);},
            Char => quote!{#buf.put_u8(#val as u8);},
            UInt8 => quote!{#buf.put_u8(#val);},
            UInt16 => quote!{#buf.put_u16_le(#val);},
            UInt32 => quote!{#buf.put_u32_le(#val);},
            Int8 => quote!{#buf.put_i8(#val);},
            Int16 => quote!{#buf.put_i16_le(#val);},
            Int32 => quote!{#buf.put_i32_le(#val);},
            Float32 => quote!{#buf.put_f32_le(#val);},
            UInt64 => quote!{#buf.put_u64_le(#val);},
            Int64 => quote!{#buf.put_i64_le(#val);},
            Float64 => quote!{#buf.put_f64_le(#val);},
            Array(t,_size) => {
                let w = t.rust_writer("*val".to_string(), buf_name.clone());
                quote!{
                    #buf.put_u8(#val.len() as u8);
                    for val in &#val {
                        #w
                    }
                }
            },
        }
    }

    /// Emit reader of a given type
    pub fn rust_reader(&self, name: String, buf_name: String) -> TokenStream {
        use self::UorbFieldType::*;
        let val:TokenStream = name.parse().unwrap();
        let buf:TokenStream = buf_name.parse().unwrap();

        match self.clone() {
            Bool => quote!{#val = #buf.get_u8() != 0;},
            Char => quote!{#val = #buf.get_u8() as char;},
            UInt8 => quote!{#val = #buf.get_u8();},
            UInt16 => quote!{#val = #buf.get_u16_le();},
            UInt32 => quote!{#val = #buf.get_u32_le();},
            UInt64 => quote!{#val = #buf.get_u64_le();},
            Int8 => quote!{#val = #buf.get_i8();},
            Int16 => quote!{#val = #buf.get_i16_le();},
            Int32 => quote!{#val = #buf.get_i32_le();},
            Int64 => quote!{#val = #buf.get_i64_le();},
            Float32 => quote!{#val = #buf.get_f32_le();},
            Float64 => quote!{#val = #buf.get_f64_le();},
            Array(t, size) => {
                if size > 32 {
                    // it is a vector
                    let r = t.rust_reader("let val".to_string(), buf_name.clone());
                    quote!{
                        for _ in 0..#size {
                            #r
                            #val.push(val);
                        }
                    }
                } else {
                    // handle as a slice
                    let r = t.rust_reader("let val".to_string(), buf_name.clone());
                    quote!{
                        for idx in 0..#val.len() {
                            #r
                            #val[idx] = val;
                        }
                    }
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
    pub name: String,
    pub description: Option<String>,
}

impl UorbMsgField {
    pub fn from_line(desc: &str, comment: Option<String>) -> Option<UorbMsgField> {
        let field_desc = desc.trim().to_string();
        if field_desc.len() < 2 {
            return None;
        }

        let toks: Vec<&str> = field_desc.split_whitespace().collect();
        if toks.len() < 2 {
            //invalid field type description
            println!("invalid field_desc: {:?} ", field_desc);
            return None;
        }

        let fname = toks[1];

        let ftype = UorbFieldType::parse_type(toks[0]);
        match ftype {
            Some(le_type) => {
                return Some(UorbMsgField {
                    uorbtype: le_type,
                    name: fname.to_string(),
                    description: comment,
                })
            },
            _ => {
                println!("failed to parse: {:?}", toks[0]);
                return None
            }
        }
    }

    /// Emit writer that will write this field to Vec<u8>
    fn rust_writer(&self) -> TokenStream {
        let name = "self.".to_string() + &self.name.clone();
        let buf  = "buf".to_string();

        self.uorbtype.rust_writer(name, buf)
    }

    /// Emit reader that will read this field from a buffer
    fn rust_reader(&self) -> TokenStream {
        let name =  "msg.".to_string() + &self.name.clone();
        let buf  = "buf".to_string();

        self.uorbtype.rust_reader(name, buf)
    }
}

impl ToTokens for UorbMsgField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.name.clone();
        let name:Ident = Ident::new(&name, Span::call_site());

        //whether the field is a primitive or Array type, parse its type into tokens
        let raw_rust_type: String = self.uorbtype.rust_type();
        let rust_type:TokenStream = raw_rust_type.parse().unwrap();

        //TODO description?
        let toks = quote!(
        pub #name: #rust_type,
        );
        tokens.append_all(toks);
    }
}



#[derive(Debug, PartialEq, Clone, Default)]
pub struct UorbMsgConst {
    pub uorbtype: UorbFieldType,
    pub name: Option<String>,
    pub const_val: Option<String>,
    pub description: Option<String>,
}

impl UorbMsgConst {
    pub fn from_line(assign_split: Vec<&str>,  comment: Option<String>) -> Option<UorbMsgConst> {
        let const_desc = assign_split[0].trim().to_string();
        let const_val:Option<String> = Some(assign_split[1].trim().to_string());

//        println!("const desc: {:?} const_val: {:?}", const_desc, const_val);

        let toks:Vec<&str> = const_desc.split_whitespace().collect();
        if toks.len() < 2 {
            //invalid const type description
            return None;
        }

        let ftype: UorbFieldType = UorbFieldType::parse_type(toks[0]).unwrap();
        let fname = toks[1];

        Some(
            UorbMsgConst {
                uorbtype: ftype,
                name: Some(fname.to_string()),
                description: comment,
                const_val: const_val
            })
    }
}

impl ToTokens for UorbMsgConst {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.name.clone().unwrap();
        let name:Ident = Ident::new(&name, Span::call_site());

        let rust_type: String = self.uorbtype.rust_type();
        let rust_type: Ident = Ident::new(&rust_type, Span::call_site());

        let val = self.const_val.clone().unwrap();
        let val:TokenStream = val.parse().unwrap();

        let toks = quote!(
        pub const #name: #rust_type = #val;
        );
        tokens.append_all(toks);
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct UorbMsg {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<UorbMsgField>,
    pub consts: Vec<UorbMsgConst>,
    pub topics: Vec<String>,
}


impl UorbMsg {
    pub fn from_lines<R: Read>(name: String, input: &mut R) -> UorbMsg  {
        let mut msg: UorbMsg = UorbMsg {
            name: name,
            description: None,
            fields: vec![],
            consts: vec![],
            topics: vec![],
        };

        let buf_reader = BufReader::new(input);
        let mut all_topics: Vec<String> = vec![];

        for line in buf_reader.lines() {
            let line = line.unwrap().clone();
            let trimline = line.trim().to_string();


            match trimline.find('#') {
                Some(_hash_pos) => {
                    let comment_split: Vec<&str> = trimline.split("#").collect();
                    match comment_split.len() {
                        1 => { //all comment, as in a TOPICS line
                            println!("all comment: {:?}", comment_split);
                            let topics_exist: Vec<&str> = comment_split[0].split("TOPICS").collect();
                            if topics_exist.len() > 0 {
                                let topics: Vec<&str> = topics_exist[0].split_whitespace().collect();
                                for topic in topics {
                                    all_topics.push(topic.to_string());
                                }
                            } else {
                                println!("Comment? {:?}", trimline);
                            }
                        },
                        2 => { // half field, half comment]
                            println!("halfsies: {:?}", comment_split);
                            let field_desc = comment_split[0];
                            let comment = comment_split[1].to_string();

                            UorbMsg::process_field_desc(field_desc, Some(comment), &mut msg);
                        },
                        _ => {
                            println!("unexpected multi # line: {:?}", trimline);
                        }
                    }
                }
                _ => {
                    //comment-free line
                    UorbMsg::process_field_desc(&trimline, None, &mut msg);
                }
            }

        }

        msg.topics = all_topics;

        msg
    }

    fn process_field_desc( field_desc: &str, comment: Option<String>, msg: &mut UorbMsg) {
        if let Some(_assign_pos) = field_desc.find('=') {
            let assign_split:Vec<&str> = field_desc.split('=').collect();
            if assign_split.len() > 1 {
                if let Some(constant) = UorbMsgConst::from_line(assign_split, comment) {
                    msg.consts.push(constant);
                }
            }
        }
        else if let Some(field) = UorbMsgField::from_line(field_desc, comment) {
            msg.fields.push(field);
        }
    }


    /// Support for tokenizing

    /// Emit rust consts
    fn emit_constants(&self) -> TokenStream {
        let mut tok_stream: TokenStream = TokenStream::new();
        for item in self.consts.clone() {
            item.to_tokens(&mut tok_stream);
        }
        tok_stream
    }

    fn emit_field_defs(&self) -> TokenStream {
        let mut tok_stream: TokenStream = TokenStream::new();
        for item in self.fields.clone() {
            item.to_tokens(&mut tok_stream);
        }
        tok_stream
    }

    fn emit_deserialize_fields(&self) -> TokenStream {
        let deser_fields = self.fields.iter()
            .map(|f| {
                f.rust_reader()
            }).collect::<Vec<TokenStream>>();

        //println!("deser_fields  {:?}",deser_fields);

        if deser_fields.is_empty() {
            // struct has no fields
            quote!{
                    Some(Self::default())
                }
        } else {
            quote!{
                    let mut msg = Self::default();
                    #(#deser_fields)*
                    Some(msg)
                }
        }
    }

    fn emit_serialize_fields(&self) -> TokenStream {
        let ser_fields = self.fields.iter()
            .map(|f| {
                f.rust_writer()
            }).collect::<Vec<TokenStream>>();
        quote!{
                let mut buf = Vec::new();
                #(#ser_fields)*
                buf
            }
    }


}

impl ToTokens for UorbMsg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let const_defs = self.emit_constants();
        let field_defs = self.emit_field_defs();
//        println!("field_defs: {:?}", field_defs);
        let deser_fields = self.emit_deserialize_fields();
        let ser_fields = self.emit_serialize_fields();
        //println!("deser_fields: {:?}",deser_fields);

        let mut encoded_msg_len = 0;
        for field in self.fields.clone() {
            encoded_msg_len += field.uorbtype.encoded_len();
        }
        let encoded_msg_len:TokenStream = format!("{:?}",encoded_msg_len).parse().unwrap();

        let name:TokenStream = self.name.clone().parse().unwrap();

        let toks = quote!(

        #[derive(Debug, Clone, PartialEq, Default)]
        pub struct #name {
            #field_defs
        }

        impl #name {
            pub const ENCODED_LEN: usize = #encoded_msg_len;
            pub const MSG_HASH_CODE: u16 = 0; //TODO
            #const_defs

            pub fn deser(input: &[u8]) -> Option<Self> {
                if input.len() < Self::ENCODED_LEN {
                    None
                }
                else {
                    //fast zero copy
                    let mut buf = Bytes::from(input).into_buf();
                    #deser_fields
                }
            }

            pub fn ser(&self) -> Vec<u8> {
                #ser_fields
            }
        }
        );

        tokens.append_all(toks);
    }


}


/// Generate rust representation of uorb message, and corresponding conversion methods
pub fn generate<R: Read, W: Write>(name: String, input: &mut R, output_rust: &mut W) {

    let msg:UorbMsg = UorbMsg::from_lines(name, input);
//    println!("msg: {:?}", msg);

    let mut top_tokens = TokenStream::new();

    //TODO top_tokens.append_all(header);
    msg.to_tokens(&mut top_tokens);
    let rust_src = top_tokens.to_string();
    println!("rust_src: {:?}", rust_src);

    let mut cfg = rustfmt::config::Config::default();
    cfg.set().write_mode(rustfmt::config::WriteMode::Display);
    let res = rustfmt::format_input(rustfmt::Input::Text(rust_src), &cfg, Some(output_rust));
    if res.is_err() {
        println!("formatting failed");
        panic!("foo");
    }
    output_rust.flush().unwrap();

}

