#![allow(dead_code)]

#[derive(Eq, PartialEq, Debug)]
enum HcPerson {
    First,
    Second,
    Third,
}

#[derive(Eq, PartialEq, Debug)]
enum HcNumber {
    Singular,
    Dual,
    Plural,
}

#[derive(Eq, PartialEq, Debug)]
enum HcTense {
    Present,
    Future,
    Imperfect,
    Aorist,
    Perfect,
    Pluperfect,
}

#[derive(Eq, PartialEq, Debug)]
enum HcVoice {
    Active,
    Middle,
    Passive,
}

#[derive(Eq, PartialEq, Debug)]
enum HcMood {
    Indicative,
    Subjunctive,
    Optative,
    Imperative,
    Infinitive,
    Participle,
}

#[derive(Eq, PartialEq, Debug)]
enum HcGender {
    Masculine,
    Feminine,
    Neuter,

}

#[derive(PartialEq, Debug)]
enum HcGreekPrincipalParts {
    First = 1,
    Second = 2,
    Third = 3,
    Fourth = 4,
    Fifth = 5,
    Sixth = 6
}

#[derive(Eq, PartialEq, Debug)]
struct HcGreekVerb {
    id: u32,
    pps: [String; 6],
    properties: String,
}

#[derive(Default)]
struct Step {
    form: String,
    explanation: String,
}

#[derive(Eq, PartialEq, Debug)]
struct HcGreekVerbForm<'a> {
    verb: &'a HcGreekVerb,
    person: HcPerson,
    number: HcNumber,
    tense: HcTense,
    voice: HcVoice,
    mood: HcMood,
    gender: Option<HcGender>,
    //steps: Vec<Step>,
}

trait HcVerbForms {
    fn get_form(&self) -> Vec<Step>;
    fn explain_form(&self) -> String;
    fn get_pp_num(&self) -> HcGreekPrincipalParts;
    fn get_pp(&self) -> String;
    fn strip_ending(&self, pp_num:usize, form:String) -> String;
}
/*
//https://stackoverflow.com/questions/59330671/how-do-i-remove-a-single-trailing-string-from-another-string-in-rust
fn remove_suffix<'a>(s: &'a str, p: &str) -> &'a str {
    if s.ends_with(p) {
        &s[..s.len() - p.len()]
    } else {
        s
    }
}
*/
impl HcVerbForms for HcGreekVerbForm<'_> {
    /*
    fn new() -> HcGreekVerbForm {

    }*/

    fn strip_ending(&self, pp_num:usize, form:String) -> String {
        match pp_num {
            1..=2 => {
                if form.ends_with("ω") {
                    form.strip_suffix("ω").unwrap().to_string()
                }
                else if form.ends_with("ομαι") {
                    form.strip_suffix("ομαι").unwrap().to_string()
                }
                else if form.ends_with("μι") {
                    form.strip_suffix("μι").unwrap().to_string()
                }
                else {
                    "".to_string()
                }
            },
            3 => {
                if form.ends_with("αμην") {
                    form.strip_suffix("αμην").unwrap().to_string()
                }
                else if form.ends_with("α") {
                    form.strip_suffix("α").unwrap().to_string()
                }
                else {
                    "".to_string()
                }                
            },
            4 => {
                if form.ends_with("α") {
                    form.strip_suffix("α").unwrap().to_string()
                }
                else {
                    "".to_string()
                }                
            },
            5 => {
                if form.ends_with("μαι") {
                    form.strip_suffix("μαι").unwrap().to_string()
                }
                else {
                    "".to_string()
                }                
            },
            6 => {
                if form.ends_with("ην") {
                    form.strip_suffix("ην").unwrap().to_string()
                }
                else {
                    "".to_string()
                }                
            },
            _ => { form.to_string() }   
        }
    }

    fn get_form(&self) -> Vec<Step> {
        let mut steps = Vec::new();
        let f = self.verb.pps.join(", ");
        let e = "Principal Parts".to_string();
        steps.push(Step{form:f, explanation:e});

        let pp_num = self.get_pp_num() as usize;
        let f = &self.verb.pps[pp_num - 1];
        let e = "Choose Principal Part".to_string();
        steps.push(Step{form:f.to_string(), explanation:e});

        //internally (not as a step) strip accent

        let mut z = Vec::new();
        let each_alt = f.split(" / ");
        for x in each_alt {
            z.push(self.strip_ending(pp_num, x.to_string()))
        }

        let f = z.join(" / ");
        let e = "Remove ending from Principal Part".to_string();
        steps.push(Step{form:f, explanation:e});

        steps
    }

    fn explain_form(&self) -> String {
        "a".to_string()
    }

    fn get_pp(&self) -> String {
        let num = self.get_pp_num() as usize;
        if (1..=6).contains(&num) {
            self.verb.pps[num - 1].to_string()
        }
        else {
            "".to_string()
        }
    }

    fn get_pp_num(&self) -> HcGreekPrincipalParts {
        match self.tense {
            HcTense::Present => HcGreekPrincipalParts::First,
            HcTense::Imperfect => HcGreekPrincipalParts::First,
            HcTense::Future => {
                match self.voice {
                    HcVoice::Active => HcGreekPrincipalParts::Second,
                    HcVoice::Middle => HcGreekPrincipalParts::Second,
                    HcVoice::Passive => HcGreekPrincipalParts::Sixth
                }
            },
            HcTense::Perfect => {
                match self.voice {
                    HcVoice::Active => HcGreekPrincipalParts::Fourth,
                    HcVoice::Middle => HcGreekPrincipalParts::Fifth,
                    HcVoice::Passive => HcGreekPrincipalParts::Fifth
                }
            },
            HcTense::Pluperfect => {
                match self.voice {
                    HcVoice::Active => HcGreekPrincipalParts::Fourth,
                    HcVoice::Middle => HcGreekPrincipalParts::Fifth,
                    HcVoice::Passive => HcGreekPrincipalParts::Fifth
                }
            },
            HcTense::Aorist => {
                match self.voice {
                    HcVoice::Active => HcGreekPrincipalParts::Third,
                    HcVoice::Middle => HcGreekPrincipalParts::Third,
                    HcVoice::Passive => HcGreekPrincipalParts::Sixth
                }  
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {

        let a = HcGreekVerb {id:1,pps:["λω".to_string(), "λσω".to_string(), "ἔλῡσα".to_string(), "λέλυκα".to_string(), "λέλυμαι".to_string(), "ἐλύθην".to_string()],properties:"blah".to_string()};
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Aorist, voice:HcVoice::Active, mood:HcMood::Indicative, gender:None};
        let c = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Aorist, voice:HcVoice::Active, mood:HcMood::Indicative, gender:None};
        assert_eq!(b, c);
        
        assert_eq!(b.get_form()[0].form, "λω, λσω, ἔλῡσα, λέλυκα, λέλυμαι, ἐλύθην");
        assert_eq!(b.get_form()[1].form, "ἔλῡσα");
        
        assert_eq!(b.get_form()[2].form, "ἔλῡσ");
        
        assert_eq!(b.get_pp_num(), HcGreekPrincipalParts::Third);
        assert_eq!(b.get_pp_num() as usize, 3);
        assert_eq!(b.verb.pps[b.get_pp_num() as usize - 1], "ἔλῡσα");
        assert_eq!(b.get_pp(), "ἔλῡσα");

        let a = HcGreekVerb {id:1,pps:["βλάπτω".to_string(), "βλάψω".to_string(), "ἔβλαψα".to_string(), "βέβλαφα".to_string(), "βέβλαμμαι".to_string(), "ἐβλάβην / ἐβλάφθην".to_string()],properties:"blah".to_string()};
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Aorist, voice:HcVoice::Passive, mood:HcMood::Indicative, gender:None};
        assert_eq!(b.get_form()[2].form, "ἐβλάβ / ἐβλάφθ");    
    }
}

