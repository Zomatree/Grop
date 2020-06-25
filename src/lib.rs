pub mod args {
    pub fn collect_args(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!(":peepee:");
        }
        let filename = args[1].clone();
        let query = args[2].clone();

        Config {filename, query}
    }

    #[derive(Debug)]
    pub struct Config {
        pub filename: String,
        pub query: String,
    }

}

pub mod search {
    pub fn search_string(text: String, substring: &str) -> Vec<Match> {
        let mut output: Vec<Match> = vec![];
        let mut ln = 1;
        let mut cn = 0;

        for (i, chr) in text.chars().enumerate() {
            if chr == '\n' {
                ln += 1;
                cn = 0;
            } else { cn += 1; }


            if &text[i..std::cmp::min(i+substring.len(), text.len())] == substring {
                output.push(Match { line: ln, start_char: cn, word: substring })
            }
        }
        output
    }

    #[derive(Debug)]
    pub struct Match<'a> {
        pub line: u32,
        pub start_char:  usize,
        pub word: &'a str,
    }
}