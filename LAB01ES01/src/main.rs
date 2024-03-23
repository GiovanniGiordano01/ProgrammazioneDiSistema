use clap::Parser;
#[derive(Parser, Debug)]
struct Args {
    // input string
    slug_in: String,
    }

fn conv(c:char) -> char{
    let mut i:usize=0;
    const SUBS_I : &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";

    const SUBS_O: &str  = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";
    let var=SUBS_O.as_bytes();
for x in SUBS_I.chars(){
        if c==x {
            return var[i] as char;    
        }
        i+=1;
            
}
 if c.is_alphanumeric(){
    return c.to_ascii_lowercase();
 }else{
    return '-';
 }
}

fn slugify(s:&str) -> String {
    let mut result=String::new();
    let mut previous_char :char=' ';
    let mut current_char :char=' ';
    for c in s.chars(){
        current_char=conv(c);
        if !(previous_char=='-' && current_char =='-'){
            result.push(current_char);
            previous_char=current_char;
        }
    } 
    if result.len()!=1 && current_char=='-' {
        result.pop();
    }
    return result;
    

}

fn main() {
    let args=Args::parse();
    //let s:&str="Hèllo wòòrld!";
    println!("{}",slugify(&args.slug_in));

}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_lettera_accentata() {
        assert_eq!(slugify("à"), "a")
    }

    #[test]
    fn test_lettera() {
        assert_eq!(slugify("a"), "a")
    }

    #[test]
    fn test_lettera_inesistente() {
        assert_eq!(slugify("💀💀💀"), "-")
    }

    #[test]
    fn test_parole_spazio() {
        assert_eq!(slugify("Oggi esplodo💀"), "oggi-esplodo")
    }

    #[test]
    fn test_vuota() {
        assert_eq!(slugify(""), "")
    }

    #[test]
    fn test_piu_spazi_consecutivi() {
        assert_eq!(slugify("Oggi      Esplodo"), "oggi-esplodo")
    }

    #[test]
    fn test_caratteri_nv_consec() {
        assert_eq!(slugify("Sos💖💖💀💖💖s"), "sos-s")
    }

    #[test]
    fn test_solo_caratteri_nv() {
        assert_eq!(slugify("💀💖💀💖💀"), "-")
    }

    #[test]
    fn test_spazio_fine() {
        assert_eq!(slugify("💖💖💖💖💖 "), "-")
    }
}

