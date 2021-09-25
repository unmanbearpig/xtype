use std::fmt;

#[derive(Debug)]
pub struct Mapping<'a>(pub &'a str, pub &'a [&'a str]);

impl fmt::Display for Mapping<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let names: String = self.1.join(" ");
        write!(f, "{}  - {}", self.0, names)
    }
}

#[test]
/// Make sure there are no duplicate characters and duplicate aliases in our
/// dictionary
fn test_no_dups_in_dict() {
    use std::collections::BTreeMap;
    let mut by_alias: BTreeMap<&str, Vec<&Mapping>> = BTreeMap::new();
    let mut by_char: BTreeMap<&str, Vec<&Mapping>> = BTreeMap::new();

    for map in DICT.iter() {
        if let Some(m) = by_char.get_mut(map.0) {
            m.push(map);
        } else {
            by_char.insert(map.0, vec![map]);
        }

        for alias in map.1.iter() {
            if let Some(m) = by_alias.get_mut(alias) {
                m.push(map);
            } else {
                by_alias.insert(alias, vec![map]);
            }
        }
    }

    let mut err = String::new();
    for (ch, maps) in by_char.iter() {
        if maps.len() == 0 {
            unreachable!("whaat");
        }
        if maps.len() > 1 {
            err.push_str(
                format!("Multiple maps for char \"{}\": {:?}\n", ch, maps)
                .as_ref());
        }
    }

    for (alias, maps) in by_alias.iter() {
        if maps.len() == 0 {
            unreachable!("whaat");
        }
        if maps.len() > 1 {
            err.push_str(
                format!("Multiple maps for alias \"{}\": {:?}\n", alias, maps)
                .as_ref());
        }
    }

    if !err.is_empty() {
        panic!("{}", err);
    }
}

/// "Database" of our characters and their aliases in no particular order
pub const DICT: &[Mapping] = &[
    Mapping("¨", &["uml", "umlaut"]),
    Mapping("ä", &["auml", "aumlaut"]),
    Mapping("Ä", &["Auml", "Aumlaut"]),
    Mapping("ö", &["ouml", "oumlaut"]),
    Mapping("Ö", &["Ouml", "Oumlaut"]),
    Mapping("ü", &["uuml", "uumlaut"]),
    Mapping("Ü", &["Uuml", "Uumlaut"]),
    Mapping("ÿ", &["yuml", "yumlaut"]),
    Mapping("Ÿ", &["Yuml", "Yumlaut"]),
    Mapping("ß", &["eszett", "eszet", "ss"]),
    Mapping("ẞ", &["Eszett", "Eszet", "SS"]),
    Mapping("€", &["eur", "euro"]),
    Mapping("₽", &["rub", "rur"]),
    Mapping("£", &["gbp", "pound"]),
    Mapping("¥", &["yen"]),
    Mapping("¢", &["cent"]),
    Mapping("α", &["alpha"]),
    Mapping("β", &["beta"]),
    Mapping("γ", &["gamma"]),
    Mapping("Γ", &["Gamma"]),
    Mapping("δ", &["delta"]),
    Mapping("Δ", &["Delta"]),
    Mapping("ε", &["epsilon"]),
    Mapping("Ε", &["Epsilon"]),
    Mapping("ζ", &["zeta"]),
    Mapping("Ζ", &["Zeta"]),
    Mapping("η", &["eta"]),
    Mapping("Η", &["Eta"]),
    Mapping("θ", &["theta"]),
    Mapping("Θ", &["Theta"]),
    Mapping("ι", &["iota"]),
    Mapping("Ι", &["Iota"]),
    Mapping("κ", &["kappa"]),
    Mapping("Κ", &["Kappa"]),
    Mapping("λ", &["lambda"]),
    Mapping("Λ", &["Lambda"]),
    Mapping("μ", &["mu"]),
    Mapping("Μ", &["Mu"]),
    Mapping("ν", &["nu"]),
    Mapping("Ν", &["Nu"]),
    Mapping("ξ", &["xi"]),
    Mapping("Ξ", &["Xi"]),
    Mapping("ο", &["omicron"]),
    Mapping("Ο", &["Omicron"]),
    Mapping("π", &["pi"]),
    Mapping("Π", &["Pi"]),
    Mapping("ρ", &["rho"]),
    Mapping("Ρ", &["Rho"]),
    Mapping("ς", &["sigma"]),
    Mapping("Σ", &["Sigma"]),
    Mapping("τ", &["tau"]),
    Mapping("Τ", &["Tau"]),
    Mapping("υ", &["upsilon"]),
    Mapping("Υ", &["Upsilon"]),
    Mapping("φ", &["phi"]),
    Mapping("Φ", &["Phi"]),
    Mapping("χ", &["chi"]),
    Mapping("Χ", &["Chi"]),
    Mapping("ψ", &["psi"]),
    Mapping("Ψ", &["Psi"]),
    Mapping("ω", &["omega"]),
    Mapping("Ω", &["Omega"]),
    Mapping("ℵ", &["alef"]),
    Mapping("æ", &["ae", "aelig"]),
    Mapping("♭", &["flat"]),
    Mapping("♯", &["sharp"]),
    Mapping("♮", &["natural"]),
    Mapping("✓", &["check"]),
    Mapping("✗", &["cross"]),
    Mapping("µ", &["micro"]),
    Mapping("—", &["mdash"]),
    Mapping("–", &["ndash"]),
    Mapping("‐", &["hyphen", "hyph"]),
    Mapping("¬", &["not"]),
    Mapping("-", &["p"]),
    Mapping("±", &["+-", "plusmn", "plusminus"]),
    Mapping("≤", &["<=", "geq"]),
    Mapping("≥", &[">=", "leq"]),
    Mapping("≠", &["!=", "neq"]),
    Mapping("≈", &["almost", "=~", "~=", "asymp"]),
    Mapping("≶", &["lg"]),
    Mapping("∀", &["forall"]),
    Mapping("∃", &["exists", "exist", "there_exists"]),
    Mapping("∿", &["sine", "acd"]),
    Mapping("∫", &["int", "integral"]),
    Mapping("∬", &["Int", "dint"]),
    Mapping("∭", &["iiint", "tint"]),
    Mapping("⌀", &["diameter"]),
    Mapping("°", &["deg", "dergee"]),
    Mapping("∅", &["empty"]),
    Mapping("∈", &["isin"]),
    Mapping("∉", &["notin"]),
    Mapping("⊂", &["sub"]),
    Mapping("⊃", &["sup"]),
    Mapping("⌈", &["lceil"]),
    Mapping("⌉", &["rceil"]),
    Mapping("⌊", &["lfloor"]),
    Mapping("⌋", &["rfloor"]),
    Mapping("∑", &["sum"]),
    Mapping("∗", &["lowast"]),
    Mapping("∝", &["prop", "proportional_to"]),
    Mapping("∟", &["angrt"]),
    Mapping("∠", &["ang"]),
    Mapping("∡", &["angmsd"]),
    Mapping("∥", &["parallel"]),
    Mapping("∦", &["npar"]),
    Mapping("∧", &["and"]),
    Mapping("∨", &["or"]),
    Mapping("∩", &["cap", "intersection"]),
    Mapping("∪", &["cup", "union"]),
    Mapping("∏", &["prod"]),
    Mapping("√", &["sqrt"]),
    Mapping("♲", &["recycle"]),
    Mapping("∞", &["inf", "infinity"]),
    Mapping("↑", &["up"]),
    Mapping("↓", &["down"]),
    Mapping("™", &["tm"]),
    Mapping("©", &["copy"]),
    Mapping("§", &["section"]),
    Mapping("÷", &["division"]),
    Mapping("…", &["ellipsis", "hellip"]),
    Mapping("⋆", &["Star"]),
    Mapping("⋅", &["sdot"]),
    Mapping("′", &["prime"]),
    Mapping("″", &["Prime", "dprime"]),
    Mapping("‴", &["tprime"]),
    Mapping("‵", &["bprime"]),
    Mapping("⁗", &["qprime"]),
    Mapping("ƒ", &["fnof", "func"]),
    Mapping("‽", &["interrobang"]),
    Mapping("¶", &["pilcrow", "para"]),
    Mapping("⁋", &["rpilcrow"]),
    Mapping("•", &["bullet", "bull"]),
    Mapping("⁃", &["hybull", "hyphen_bullet"]),
    Mapping("¦", &["brvbar"]),
    Mapping("⁰", &["sup0"]),
    Mapping("¹", &["sup1"]),
    Mapping("²", &["sup2"]),
    Mapping("³", &["sup3"]),
    Mapping("⁴", &["sup4"]),
    Mapping("⁵", &["sup5"]),
    Mapping("⁶", &["sup6"]),
    Mapping("⁷", &["sup7"]),
    Mapping("⁸", &["sup8"]),
    Mapping("⁹", &["sup9"]),
    Mapping("á", &["aacute"]),
    Mapping("Á", &["Aacute"]),
    Mapping("é", &["eacute"]),
    Mapping("É", &["Eacute"]),
    Mapping("í", &["iacute"]),
    Mapping("Í", &["Iacute"]),
    Mapping("ý", &["yacute"]),
    Mapping("Ý", &["Yacute"]),
    Mapping("ú", &["uacute"]),
    Mapping("Ú", &["Uacute"]),
    Mapping("ø", &["o/"]),
    Mapping("Ø", &["O/"]),
    Mapping("ℕ", &["natural"]),
    Mapping("ℙ", &["primes"]),
    Mapping("ℚ", &["rationals"]),
    Mapping("ℤ", &["integers"]),
    Mapping("¼", &["1/4"]),
    Mapping("½", &["1/2"]),
    Mapping("¾", &["3/4"]),
    Mapping("⅔", &["2/3"]),
    Mapping("⅓", &["1/3"]),
    Mapping("⅔", &["2/3"]),
    Mapping("⅕", &["1/5"]),
    Mapping("⅖", &["2/5"]),
    Mapping("⅗", &["3/5"]),
    Mapping("⅘", &["4/5"]),
    Mapping("⅙", &["1/6"]),
    Mapping("⅚", &["5/6"]),
    Mapping("⅛", &["1/8"]),
    Mapping("⅜", &["3/8"]),
    Mapping("⅝", &["5/8"]),
    Mapping("⅞", &["7/8"]),
    Mapping("← ", &["larr", "leftarrow"]),
    Mapping("↑ ", &["uarr", "uparrow"]),
    Mapping("→ ", &["rarr", "rightarrow"]),
    Mapping("↓ ", &["darr", "downarrow"]),
];
