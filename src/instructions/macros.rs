
#[macro_export]
macro_rules! parse_arg {
    ($instr_itr:ident) => {{
        let raw_part = $instr_itr.next();
        let parse = match raw_part {
            Some(rd) => OpParam::parse(rd.trim()),
            None => OpParam::Blank
        };
        parse
    }};
}

#[macro_export]
macro_rules! parse_args {
    ($ln:ident, 1) => {{
        let mut without_comment = $ln.splitn(2, "//").next().unwrap_or("").trim();
        let mut instruction_itr =  without_comment.splitn(2, ",");
        parse_arg!(instruction_itr)
    }};
    ($ln:ident, 2) => {{
        let mut without_comment = $ln.splitn(2, "//").next().unwrap_or("").trim();
        let mut instruction_itr =  without_comment.splitn(2, ",");

        let parsed_arg1 = parse_arg!(instruction_itr); 
        let parsed_arg2 = parse_arg!(instruction_itr);

        (parsed_arg1, parsed_arg2)
    }};
    ($ln:ident, 3) => {{
        let mut without_comment = $ln.splitn(2, "//").next().unwrap_or("").trim();
        let mut instruction_itr =  without_comment.splitn(3, ",");

        let parsed_arg1 = parse_arg!(instruction_itr); 
        let parsed_arg2 = parse_arg!(instruction_itr);
        let parse_length = parse_arg!(instruction_itr);

        (parsed_arg1, parsed_arg2, parse_length)
    }};
}
