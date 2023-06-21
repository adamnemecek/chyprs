use pom::char_class::hex_digit;
use pom::parser::*;

use pom::parser::{
    call,
    is_a,
    one_of,
    sym,
    tag,
    Parser,
};

#[derive(Debug)]
enum Stmt {
    Import(Import),
    Gen(Gen),
    Let(Let),
    Def(Def),
    Rule(Rule),
    Rewrite(Rewrite),
    Show(Show),
}

#[derive(Debug)]
struct Gen {
    name: String,
    from: i64,
    to: i64,
    colors: Vec<Color>,
}

#[derive(Debug)]

struct Def {
    //
    var: String,
    term: Term,
    colors: Vec<Color>,
}

// #[derive(Debug)]
// struct Perm {
// 	//
// }

#[derive(Debug)]

struct Let {
    var: String,
    term: Term,
}

#[derive(Debug)]
struct Rule {
    //
    var: String,
    lhs: Term,
    op: Le,
    rhs: Term,
}

#[derive(Debug)]

struct Rewrite {
    //
    conv: bool,
    var: String,
    term: Term,
    rewrite: RewritePart, // multiple?
}

enum Tactic {
    // Ident(),
}

#[derive(Debug)]
struct RewritePart {
    //
}

#[derive(Debug)]
struct Import {
    //
    module_name: String,
    as_: Option<String>,
}

#[derive(Debug)]
struct Show {
    //
}

// #[derive(Debug)]
// enum AST {
// 	//
// 	Statement(Box<Self>),
// 	Gen(),
// 	Def(),
// 	Neg,
// 	Let(String),
// 	Rule(),
// 	Rewrite(),
// 	Show(),
// 	// Id,
// 	// Id0,
// 	Tactic(),
// }

// eq : "=" | "=="
// le : "<=" | "~>"
// num : INT
// module_name : IDENT
// var : IDENT
// term_ref : IDENT
// rule_ref : IDENT
// term_hole : term | "?"
// color : HEXDIGIT HEXDIGIT HEXDIGIT HEXDIGIT HEXDIGIT HEXDIGIT
// IDENT: ("_"|LETTER) ("_"|"."|LETTER|DIGIT)*
// TACTIC_ARG: /[^(),]+/

// start : statement*
// ?statement : import_statement | gen | let | def_statement | rule | rewrite | show
// gen : "gen" var ":" num "->" num [ gen_color ]
// def_statement : "def" var "=" term [ gen_color ]
// gen_color : "\\\"" color "\\\"" | "\\\"" color "\\\"" "\\\"" color "\\\""
// let : "let" var "=" term
// rule : "rule" var ":" term (eq | le) term
// rewrite : "rewrite" [converse] var ":" term rewrite_part*
// rewrite_part : (eq | le) term_hole [ "by" tactic ]
// converse : "-"

// LPAREN: "("
// tactic : [ converse ] IDENT | IDENT LPAREN [ TACTIC_ARG ("," TACTIC_ARG)* ] ")"
// ?term  : par_term | seq
// ?par_term : "(" term ")" | par | perm | id | id0 | term_ref
// par : par_term "*" par_term
// seq : term ";" term
// perm : "sw" [ "[" num ("," num)* "]" ]
// id : "id"
// id0 : "id0"
// show : "show" rule_ref

// import_statement : "import" module_name [ "as" var ] [ "(" import_let ("," import_let)* ")" ]
// import_let : var "=" term

// what is the secord argument for parser?

// fn let_<'a>() -> Parser<'a, u8, String> {
// 	//
// 	seq(b"stuff").map(|x| x.to_str().unwrap().to_owned())
// }

fn underscore(ch: u8) -> bool {
    ch == b'_'
}

fn alpha_or_underscore(ch: u8) -> bool {
    pom::char_class::alpha(ch) || underscore(ch)
}

fn alphanum_or_underscore(ch: u8) -> bool {
    pom::char_class::alphanum(ch) || underscore(ch)
}

fn space<'a>() -> Parser<'a, u8, ()> {
    one_of(b" ").repeat(0..).discard()
}

fn tee<T: std::fmt::Debug>(v: T) -> T {
    println!("{:?}", v);
    v
}

// fn id<'a>() -> Parser<'a, u8, Term> {
// 	seq(b"id").map(|_| Term::Id)
// }

// fn id0<'a>() -> Parser<'a, u8, Term> {
// 	seq(b"id0").map(|_| Term::Id0)
// }
// fn ident<'a>() -> Parser<'a, char, String> {
// 	//
// 	tag("let").map(|x| AST::Let(x.to_owned())) - space()
// }
#[derive(Debug)]

struct Color {
    s: String,
    // r: u8,
    // b: u8,
    // g: u8
}

// fn hex_digit1<'a>() -> Parser<'a, u8, u8> {
//     is_a(hex_digit)
// }

fn color<'a>() -> Parser<'a, u8, Color> {
    is_a(hex_digit).repeat(6).map(|x| Color {
        s: String::from_utf8(x).unwrap(),
    })
}

fn gen_colors<'a>() -> Parser<'a, u8, Vec<Color>> {
    color().repeat(0..)
}
fn ident<'a>() -> Parser<'a, u8, String> {
    (is_a(alpha_or_underscore)
        + is_a(alphanum_or_underscore).repeat(0..))
    .map(|(ch, tail)| {
        let iter =
            std::iter::once(ch).chain(tail.iter().cloned());
        String::from_utf8(iter.collect()).unwrap()
    })
}

// fn int_from_slice(s: &[u8]) -> i64 {
//     let z = String::from_utf8(x.to_owned();

// }

fn integer<'a>() -> Parser<'a, u8, i64> {
    let int = one_of(b"123456789")
        - one_of(b"0123456789").repeat(0..)
        | sym(b'0');
    int.collect()
        .map(|x| String::from_utf8(x.to_owned()))
        .convert(|s| i64::from_str_radix(&s.unwrap(), 10))
}

fn gen<'a>() -> Parser<'a, u8, Gen> {
    (seq(b"gen") * ws(ident()) - sym(b':') + ws(integer())
        - seq(b"~>")
        + ws(integer())
        + gen_colors().opt())
    .map(|(((name, from), to), colors)| Gen {
        name,
        from,
        to,
        colors: colors.unwrap_or_else(|| vec![]),
    })
    // unimplemented!()
}

fn import<'a>() -> Parser<'a, u8, Import> {
    unimplemented!()
}

fn def<'a>() -> Parser<'a, u8, Def> {
    (seq(b"seq") * ws(ident()) - sym(b'=')
        + ws(term())
        + gen_colors().opt())
    .map(|((var, term), cols)| Def {
        var,
        term,
        colors: cols.unwrap_or_else(|| vec![]),
    })
}

fn rewrite<'a>() -> Parser<'a, u8, Rewrite> {
    (seq(b"rewrite") * sym(b'-').opt() + ws(ident())
        - sym(b':')
        + ws(term())
        + rewrite_part())
    .map(|(((conv, var), term), rewrite)| Rewrite {
        conv: conv.is_some(),
        var,
        term,
        rewrite,
    })
}

fn rewrite_part<'a>() -> Parser<'a, u8, RewritePart> {
    unimplemented!()
}

fn show<'a>() -> Parser<'a, u8, Show> {
    unimplemented!()
}

fn stmt<'a>() -> Parser<'a, u8, Stmt> {
    import().map(Stmt::Import)
        | gen().map(Stmt::Gen)
        | let_().map(Stmt::Let)
        | def().map(Stmt::Def)
        | rule().map(Stmt::Rule)
        | rewrite().map(Stmt::Rewrite)
        | show().map(Stmt::Show)
}

fn rule<'a>() -> Parser<'a, u8, Rule> {
    (seq(b"rule") * ws(ident()) - sym(b':')
        + ws(term())
        + le()
        + ws(term()))
    .map(|(((var, lhs), op), rhs)| Rule {
        var,
        lhs,
        op,
        rhs,
    })
}

fn seq_<'a>() -> Parser<'a, char, String> {
    unimplemented!()
}

fn integer_list<'a>() -> Parser<'a, u8, Vec<i64>> {
    sym(b'[') * list(ws(integer()), sym(b',')) - sym(b']')
}

fn perm<'a>() -> Parser<'a, u8, Vec<i64>> {
    ws(seq(b"sw")) * integer_list()
}

#[derive(Debug)]
enum Term {
    // ParTerm(Box<Self>),
    // Seq(Box<Self>, Box<Self>),
    Par(Box<Self>, Box<Self>),
    Perm(Vec<i64>),
    Id,
    Id0,
    TermRef(String),
}

enum Eq {
    Assign,
    Eq,
}

fn eq<'a>() -> Parser<'a, u8, Eq> {
    sym(b'=').map(|_| Eq::Assign)
        | seq(b"==").map(|_| Eq::Eq)
}

#[derive(Debug)]
enum Le {
    Le,
    Tilde,
}

fn le<'a>() -> Parser<'a, u8, Le> {
    ws(seq(b"<=")).map(|_| Le::Le)
        | ws(seq(b"~>")).map(|_| Le::Tilde)
}

fn ws<'a, T: 'a>(
    p: Parser<'a, u8, T>,
) -> Parser<'a, u8, T> {
    space() * p - space()
}

fn parens<'a, T: 'a>(
    p: Parser<'a, u8, T>,
) -> Parser<'a, u8, T> {
    sym(b'(') * p - sym(b')')
}

fn term<'a>() -> Parser<'a, u8, Term> {
    parens(term())
        | (ws(term()) - sym(b'*') + ws(term()))
            .map(|(a, b)| Term::Par(a.into(), b.into()))
        | perm().map(Term::Perm)
        | seq(b"id").map(|_| Term::Id)
        | seq(b"id0").map(|_| Term::Id0)
}

fn stmts<'a>() -> Parser<'a, u8, Vec<Stmt>> {
    list(ws(stmt()), ws(sym(b';')))
}

fn let_<'a>() -> Parser<'a, u8, Let> {
    (seq(b"let") * ws(ident()) - sym(b'=') + ws(term()))
        .map(|(var, term)| Let { var, term })
}

fn parse<'a, T>(
    input: &'a [u8],
    parser: Parser<'a, u8, T>,
) -> T {
    let x = parser.parse(input).unwrap();
    x
}

// fn test_gen() {
// 	let input = "gen variable ";
// 	let input: Vec<char> = input.chars().collect();
// 	let p = integer();
// 	let x = p.parse(&input);
// 	println!("{:?}", x);
// }

fn test_integer_list() {
    //
    tee(parse(
        b"[  10  ,   2,3,4,5, 6   ]",
        integer_list(),
    ));
}

fn main1() {
    // test_int();
    test_integer_list();
}

pub fn parse_test() {
    use std::{
        fs::File,
        io::Read,
    };

    let mut f = File::open(
        "/Users/adamnemecek/Code2/chyprs/hopf.chyp",
    )
    .unwrap();

    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    println!("{:?}", s);

    let s = parse(s.as_bytes(), stmts());
}

// fn parse<'a, T>(input: &'a str, parser: Parser<'a, char, T>) -> T {
// 	// let i: Vec<char> = input.chars().collect();
// 	let l = input.chars().count();
// 	let p = input.as_bytes().as_ptr() as *const char;

// 	let s = unsafe { std::slice::from_raw_parts(p, l) };
// 	let x = parser.parse(s).unwrap();
// 	x
// }

// macro_rules! parse {
// 	($i: expr, $p: expr) => {
// 		let input: Vec<char> = input.chars().collect();
// 		let x = parser.parse(&input).unwrap();
// 		x
// 	};
// }

// type P = Parser<>

fn test_old() {
    //
    // let z = tag("null");
    // let input = "let name = ";
    // let input: Vec<char> = input.chars().collect();
    // let p = let_();
    // let x = p.parse(&input);
    // let z = let_().parse(b"stuff");
    // println!("{:?}", x);
}

// fn perm<'a>() -> Parser<'a, u8, Term> {
// 	let num = is_a(|x| x.is_digit()).repeat(1..).convert(|s| {
// 		std::str::from_utf8(s).parse::<i32>()
// 		// .map_err(|_| Error::Custom("Invalid number"))
// 	});

// 	seq((tag("sw"), sym('['), num.repeat(1..), sym(']'))).map(|(_, _, nums, _)| Term::Perm(nums))
// }

// fn with_spaces<T: 'static>(p: Parser<char, T>) -> Parser<char, T> {
// 	space() * p - space()
// }

// fn int<'a>() -> Parser<'a, u8, usize> {
// 	let p = one_of(b"123456789") + one_of(b"0123456789").repeat(0..);
// 	let int = sym(b'0') | p;

// 	// int.map(|x| )
// 	// int.convert(std::str::from_utf8)
// 	unimplemented!()
// }
// fn integer<'a>() -> Parser<'a, char, i64> {
// 	let int = one_of("123456789") - one_of("0123456789").repeat(0..) | sym('0');
// 	int.collect()
// 		.map(String::from_iter)
// 		.convert(|s| i64::from_str_radix(&s, 10))
// }

// fn test_int() {
// 	let input = "12321321";
// 	let input: Vec<char> = input.chars().collect();
// 	let p = integer();
// 	let x = p.parse(&input);
// 	println!("{:?}", x);
// }

// fn color<'a>() -> Parser<'a, u8, Color> {
// 	is_a(hex_digit)
// 		.repeat(6)
// 		.convert(String::from_utf8)
// 		.map(|x| Color { s: x })
// }
