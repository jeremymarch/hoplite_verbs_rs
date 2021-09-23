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
}

impl HcVerbForms for HcGreekVerbForm<'_> {
    /*
    fn new() -> HcGreekVerbForm {

    }*/

    fn get_form(&self) -> Vec<Step> {
        let mut a = Vec::new();
        let b = self.verb.pps;
        let c = "blah2".to_string();
        a.push(Step{form:b,explanation:c});
        a
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
        assert_eq!(b.get_form()[0].form, "blah");
        assert_eq!(b.get_pp_num(), HcGreekPrincipalParts::Third);
        assert_eq!(b.get_pp_num() as usize, 3);
        assert_eq!(b.verb.pps[b.get_pp_num() as usize - 1], "ἔλῡσα");
        assert_eq!(b.get_pp(), "ἔλῡσα");
    }
}

