#![allow(dead_code)]

enum HcPerson {
    First,
    Second,
    Third,
}

enum HcNumber {
    Singular,
    Dual,
    Plural,
}

enum HcTense {
    Present,
    Future,
    Imperfect,
    Aorist,
    Perfect,
    Pluperfect,
}

enum HcVoice {
    Active,
    Middle,
    Passive,
}

enum HcMood {
    Indicative,
    Subjunctive,
    Optative,
    Imperative,
    Infinitive,
    Participle,
}

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


struct HcGreekVerb {
    id: u32,
    pps: [String; 6],
    properties: String,
}

struct HcGreekVerbForm<'a> {
    verb: &'a HcGreekVerb,
    person: HcPerson,
    number: HcNumber,
    tense: HcTense,
    voice: HcVoice,
    mood: HcMood,
    gender: Option<HcGender>,
}

trait HcVerbForms {
    fn get_form(&self) -> String;
    fn explain_form(&self) -> String;
    fn get_pp_num(&self) -> HcGreekPrincipalParts;
    fn get_pp(&self) -> String;
}

impl HcVerbForms for HcGreekVerbForm<'_> {
    fn get_form(&self) -> String {
        "a".to_string()
    }

    fn explain_form(&self) -> String {
        "a".to_string()
    }

    fn get_pp(&self) -> String {
        self.verb.pps[self.get_pp_num() as usize - 1].to_string()
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
        //let c = GreekVerbForm {verb:&a, person:GreekPerson::First, number:GreekNumber::Singular, tense:GreekTense::Aorist, voice:GreekVoice::Active, mood:GreekMood::Indicative};
        assert_eq!(b.get_form(), "a");
        assert_eq!(b.get_pp_num(), HcGreekPrincipalParts::Third);
        assert_eq!(b.get_pp_num() as usize, 3);
        assert_eq!(b.verb.pps[b.get_pp_num() as usize - 1], "ἔλῡσα");
        assert_eq!(b.get_pp(), "ἔλῡσα");
    }
}

