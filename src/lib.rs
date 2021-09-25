#![allow(dead_code)]

extern crate rustunicodetests;
use rustunicodetests::*;
//use rustunicodetests::hgk_toggle_diacritic_str;
use rustunicodetests::hgk_strip_diacritics;
//use rustunicodetests::hgk_transliterate;
//use rustunicodetests::hgk_convert;

#[derive(Eq, PartialEq, Debug)]
enum HcEndings {
PresentActiveInd = 0,
ImperfectActiveInd,
AoristActiveInd,
PerfectActiveInd,
PluperfectActiveInd,
FutureActiveInd,
PresentActiveSubj,
AoristActiveSubj,
PresentActiveOpt,
AoristActiveOpt,
PresentMidpassInd,
ImperfectMidpassInd,
AoristPassiveInd,
AoristMidInd,
AoristPassiveSubj,
AoristPassiveOpt,
AoristMiddleSubj,
AoristMiddleOpt,
PerfectMidpassInd,
PluperfectMidpassInd,
PresentMidpassSubj,
PresentMidpassOpt,
PresentActiveImperative,
PresentMidpassImperative,
AoristActiveImperative,
AoristMiddleImperative,
AoristPassiveImperative,
FutureMidpassInd,
PresentActiveIndicAContracted,
PresentMidpassIndicAContracted,
ImperfectActiveIndicAContracted,
ImperfectMidpassIndicAContracted,
PresentActiveSubjAContracted,
PresentMidpassSubjAContracted,
PresentActiveOptAContracted,
PresentMidpassOptAContracted,
PresentActiveIndicEContracted,
PresentMidpassIndicEContracted,
ImperfectActiveIndicEContracted,
ImperfectMidpassIndicEContracted,
PresentActiveSubjEContracted,
PresentMidpassSubjEContracted,
PresentActiveOptEContracted,
PresentMidpassOptEContracted,
PresentActiveIndicOContracted,
PresentMidpassIndicOContracted,
ImperfectActiveIndicOContracted,
ImperfectMidpassIndicOContracted,
PresentActiveSubjOContracted,
PresentMidpassSubjOContracted,
PresentActiveOptOContracted,
PresentMidpassOptOContracted,
PresentActiveImperativeAContracted,
PresentMidpassImperativeAContracted,
PresentActiveImperativeEContracted,
PresentMidpassImperativeEContracted,
PresentActiveImperativeOContracted,
PresentMidpassImperativeOContracted,
PresentActiveIndicativeMi,
PresentActiveOptativeContractedNotPrecontracted,
AoristActiveImperativesMi,
AoristActiveImperativesMiRoot,
AoristMiddleImperativesMi,
AoristActiveIndicativeMiRoot,
SecondAoristMiddleImperative,
PresentMidpassOptTithhmi,
ImperfectActiveContractedDecomposed,
NotImplemented,
NumEndings,
}

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

#[derive(Eq, PartialEq, Debug)]
enum HcCase {
    Nominative,
    Genitive,
    Dative,
    Accusative,
    Vocative,
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
pub struct HcGreekVerb {
    id: u32,
    pps: Vec<String>,
    properties: String,
}

impl HcGreekVerb {
    fn from_string(id:u32, pps:&str, properties:&str) -> Option<HcGreekVerb> {
        let x: Vec<String> = pps.split(',').map(|s| s.trim().to_owned()).collect();
        if x.len() == 6 {
            Some(HcGreekVerb {
                id,
                pps: x,
                properties: properties.to_string()
            })
        }
        else {
            None
        }
    }
}

#[derive(Default)]
struct Step {
    form: String,
    explanation: String,
}

#[derive(Eq, PartialEq, Debug)]
pub struct HcGreekVerbForm<'a> {
    verb: &'a HcGreekVerb,
    person: HcPerson,
    number: HcNumber,
    tense: HcTense,
    voice: HcVoice,
    mood: HcMood,
    gender: Option<HcGender>,
    case: Option<HcCase>,
}

trait HcVerbForms {
    fn get_form(&self) -> Result<Vec<Step>, &str>;
    fn get_pp_num(&self) -> HcGreekPrincipalParts;
    fn get_pp(&self) -> String;
    fn strip_ending(&self, pp_num:usize, form:String) -> Result<String, &str>;
    fn add_ending(&self, stem:&str, ending:&str) -> Result<String, &str>;
    fn get_endings(&self) -> Vec<&str>;
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

    fn strip_ending(&self, pp_num:usize, form:String) -> Result<String, &str> {
        match pp_num {
            1..=2 => {
                if form.ends_with('ω') {
                    return Ok(form.strip_suffix('ω').unwrap().to_string());
                }
                else if form.ends_with("ομαι") {
                    return Ok(form.strip_suffix("ομαι").unwrap().to_string());
                }
                else if form.ends_with("μι") {
                    return Ok(form.strip_suffix("μι").unwrap().to_string());
                }
            },
            3 => {
                if form.ends_with("αμην") {
                    return Ok(form.strip_suffix("αμην").unwrap().to_string());
                }
                else if form.ends_with('α') {
                    return Ok(form.strip_suffix('α').unwrap().to_string());
                }              
            },
            4 => {
                if form.ends_with('α') {
                    return Ok(form.strip_suffix('α').unwrap().to_string());
                }               
            },
            5 => {
                if form.ends_with("μαι") {
                    return Ok(form.strip_suffix("μαι").unwrap().to_string());
                }               
            },
            6 => {
                if form.ends_with("ην") {
                    return Ok(form.strip_suffix("ην").unwrap().to_string());
                }               
            },
            _ => { return Err("error stripping ending 1"); }   
        }
        Err("error stripping ending 2")
    }

    fn add_ending(&self, stem:&str, ending:&str) -> Result<String, &str> {
        let r = format!("{}{}", stem, ending);
        Ok(r)
    }

    fn get_form(&self) -> Result<Vec<Step>, &str> {
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
        let f = hgk_strip_diacritics(f, HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE);
        let each_alt = f.split(" / ");
        for x in each_alt {
            let y = self.strip_ending(pp_num, x.to_string());
            if y.is_err() {
                panic!("error stripping ending");
            }
            z.push(y.unwrap());
        }

        let f = z.join(" / ");
        let e = "Remove ending from Principal Part".to_string();
        steps.push(Step{form:f, explanation:e});

        let mut zz = Vec::new();
        for a in z {
            let endings_for_form = self.get_endings();
            for e in endings_for_form {
                let y = self.add_ending(&a, e);
                if y.is_err() {
                    panic!("oops");
                }
                zz.push(y.unwrap());
            }
        }
        let f = zz.join(", ");
        let e = "Add ending".to_string();
        steps.push(Step{form:f, explanation:e});        

        Ok(steps)
    }

    fn get_pp(&self) -> String {
        let num = self.get_pp_num() as usize;
        if (1..=6).contains(&num) {
            self.verb.pps[num - 1].to_string()
        }
        else {
            panic!("no pp???");
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
    fn get_endings(&self) -> Vec<&str> {
        let ending:usize = match self.tense {
            HcTense::Present => {
                match self.voice {
                    HcVoice::Active => {
                        match self.mood {
                            HcMood::Indicative => HcEndings::PresentActiveInd,
                            HcMood::Subjunctive => HcEndings::PresentActiveSubj,
                            HcMood::Optative => HcEndings::PresentActiveOpt,
                            HcMood::Imperative => HcEndings::PresentActiveImperative,
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    },
                    HcVoice::Middle | HcVoice::Passive => {
                        match self.mood {
                            HcMood::Indicative => HcEndings::PresentMidpassInd,
                            HcMood::Subjunctive => HcEndings::PresentMidpassSubj,
                            HcMood::Optative => HcEndings::PresentMidpassOpt,
                            HcMood::Imperative => HcEndings::PresentMidpassImperative,
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    }
                }
            },
            HcTense::Imperfect => {
                match self.voice {
                    HcVoice::Active => {
                        match self.mood {
                            HcMood::Indicative => HcEndings::PresentActiveInd,
                            HcMood::Subjunctive => HcEndings::PresentActiveSubj,
                            HcMood::Optative => HcEndings::PresentActiveOpt,
                            HcMood::Imperative => HcEndings::PresentActiveImperative,
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    },
                    HcVoice::Middle | HcVoice::Passive => {
                        match self.mood {
                            HcMood::Indicative => HcEndings::PresentMidpassInd,
                            HcMood::Subjunctive => HcEndings::PresentMidpassSubj,
                            HcMood::Optative => HcEndings::PresentMidpassOpt,
                            HcMood::Imperative => HcEndings::PresentMidpassImperative,
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    }
                }
            },
            HcTense::Future => {
                match self.voice {
                    HcVoice::Active => {
                        match self.mood {
                            HcMood::Indicative => HcEndings::PresentActiveInd,
                            HcMood::Subjunctive => HcEndings::PresentActiveSubj,
                            HcMood::Optative => HcEndings::PresentActiveOpt,
                            HcMood::Imperative => HcEndings::PresentActiveImperative,
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    },
                    HcVoice::Middle | HcVoice::Passive => {
                        match self.mood {
                            HcMood::Indicative => HcEndings::PresentMidpassInd,
                            HcMood::Subjunctive => HcEndings::PresentMidpassSubj,
                            HcMood::Optative => HcEndings::PresentMidpassOpt,
                            HcMood::Imperative => HcEndings::PresentMidpassImperative,
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    }
                }
            },
            HcTense::Aorist => {
                match self.voice {
                    HcVoice::Active => {
                        match self.mood {
                            HcMood::Indicative => HcEndings::PresentActiveInd,
                            HcMood::Subjunctive => HcEndings::PresentActiveSubj,
                            HcMood::Optative => HcEndings::PresentActiveOpt,
                            HcMood::Imperative => HcEndings::PresentActiveImperative,
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    },
                    HcVoice::Middle | HcVoice::Passive => {
                        match self.mood {
                            HcMood::Indicative => HcEndings::PresentMidpassInd,
                            HcMood::Subjunctive => HcEndings::PresentMidpassSubj,
                            HcMood::Optative => HcEndings::PresentMidpassOpt,
                            HcMood::Imperative => HcEndings::PresentMidpassImperative,
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    }
                }
            },
            HcTense::Perfect => {
                match self.voice {
                    HcVoice::Active => {
                        match self.mood {
                            HcMood::Indicative => HcEndings::PresentActiveInd,
                            HcMood::Subjunctive => HcEndings::PresentActiveSubj,
                            HcMood::Optative => HcEndings::PresentActiveOpt,
                            HcMood::Imperative => HcEndings::PresentActiveImperative,
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    },
                    HcVoice::Middle | HcVoice::Passive => {
                        match self.mood {
                            HcMood::Indicative => HcEndings::PresentMidpassInd,
                            HcMood::Subjunctive => HcEndings::PresentMidpassSubj,
                            HcMood::Optative => HcEndings::PresentMidpassOpt,
                            HcMood::Imperative => HcEndings::PresentMidpassImperative,
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    }
                }
            },
            HcTense::Pluperfect => {
                match self.voice {
                    HcVoice::Active => {
                        match self.mood {
                            HcMood::Indicative => HcEndings::PresentActiveInd,
                            HcMood::Subjunctive => HcEndings::PresentActiveSubj,
                            HcMood::Optative => HcEndings::PresentActiveOpt,
                            HcMood::Imperative => HcEndings::PresentActiveImperative,
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    },
                    HcVoice::Middle | HcVoice::Passive => {
                        match self.mood {
                            HcMood::Indicative => HcEndings::PresentMidpassInd,
                            HcMood::Subjunctive => HcEndings::PresentMidpassSubj,
                            HcMood::Optative => HcEndings::PresentMidpassOpt,
                            HcMood::Imperative => HcEndings::PresentMidpassImperative,
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    }
                }
            },
        } as usize;

        let person_number:usize = match self.person {
            HcPerson::First => {
                match self.number {
                    HcNumber::Singular => 0,
                    HcNumber::Dual => 0,
                    HcNumber::Plural => 3,
                }    
            },
            HcPerson::Second => {
                match self.number {
                    HcNumber::Singular => 1,
                    HcNumber::Dual => 0,
                    HcNumber::Plural => 4,
                }    
            },
            HcPerson::Third => {
                match self.number {
                    HcNumber::Singular => 2,
                    HcNumber::Dual => 0,
                    HcNumber::Plural => 5,
                }    
            }, 
        };

        ENDINGS[ending][person_number].split(',').collect()
    }
}

static ENDINGS: &[[&str; 6]; 67] = &[["ω", "εις", "ει", "ομεν", "ετε", "ουσι(ν)"],//, "Present Active Indicative" },
    ["ον", "ες", "ε(ν)", "ομεν", "ετε", "ον"],//, "Imperfect Active Indicative" },
    ["α", "ας", "ε(ν)", "αμεν", "ατε", "αν"],//, "Aorist Active Indicative" },
    ["α", "ας", "ε(ν)", "αμεν", "ατε", "ᾱσι(ν)"],//, "Perfect Active Indicative" },
    ["η", "ης", "ει(ν)", "εμεν", "ετε", "εσαν"],//, "Pluperfect Active Indicative" },
    ["ω", "εις", "ει", "ομεν", "ετε", "ουσι(ν)"],//, "Future Active Indicative" },
    ["ω", "ῃς", "ῃ", "ωμεν", "ητε", "ωσι(ν)"],//, "Present Active Subjunctive" },
    ["ω", "ῃς", "ῃ", "ωμεν", "ητε", "ωσι(ν)"],//, "Aorist Active Subjunctive" },
    ["οιμι", "οις", "οι", "οιμεν", "οιτε", "οιεν"],//, "Present Active Optative" },
    ["αιμι", "αις, ειας", "αι, ειε(ν)", "αιμεν", "αιτε", "αιεν, ειαν"],//, "Aorist Active Optative" },
    ["ομαι", "ει,ῃ", "εται", "ομεθα", "εσθε", "ονται"],//, "Present Middle/Passive Indicative" },
    ["ομην", "ου", "ετο", "ομεθα", "εσθε", "οντο"],//, "Imperfect Middle/Passive Indicative" },
    ["ην", "ης", "η", "ημεν", "ητε", "ησαν"],//, "Aorist Passive Indicative" },
    ["αμην", "ω", "ατο", "αμεθα", "ασθε", "αντο"],//, "Aorist Middle Indicative" },
    ["ῶ", "ῇς", "ῇ", "ῶμεν", "ῆτε", "ῶσι(ν)"],//, "Aorist Passive Subjunctive" },
    ["ειην", "ειης", "ειη", "εῖμεν, ειημεν", "εῖτε, ειητε", "εῖεν, ειησαν"],//, "Aorist Passive Optative" },
    ["ωμαι", "ῃ", "ηται", "ωμεθα", "ησθε", "ωνται"],//, "Aorist Middle Subjunctive" },
    ["αιμην", "αιο", "αιτο", "αιμεθα", "αισθε", "αιντο"],//, "Aorist Middle Optative" },
    ["μαι", "σαι", "ται", "μεθα", "σθε", "νται"],//, "Perfect Middle/Passive Indicative" },
    ["μην", "σο", "το", "μεθα", "σθε", "ντο"],//, "Pluperfect Middle/Passive Indicative" },
    ["ωμαι", "ῃ", "ηται", "ωμεθα", "ησθε", "ωνται"],//, "Present Middle/Passive Subjunctive" },
    ["οιμην", "οιο", "οιτο", "οιμεθα", "οισθε", "οιντο"],//, "Present Middle/Passive Optative" },
    ["", "ε", "ετω",   "", "ετε", "οντων"],//, "Present Active Imperative" },
    ["", "ου", "εσθω", "", "εσθε", "εσθων"],//, "Present Middle/Passive Imperative" },
    ["", "ον", "ατω",  "", "ατε", "αντων"],//, "Aorist Active Imperative" },
    ["", "αι", "ασθω", "", "ασθε", "ασθων"],//, "Aorist Middle Imperative" },
    ["", "ητι, ηθι", "ητω", "", "ητε", "εντων"],//, "Aorist Passive Imperative" },
    ["ομαι", "ει,ῃ", "εται", "ομεθα", "εσθε", "ονται"],//, "Future Middle/Passive Indicative" },
    
    ["ῶ", "ᾷς", "ᾷ", "ῶμεν", "ᾶτε", "ῶσι(ν)"],//, ""],// },         //pres active indic a
    ["ῶμαι", "ᾷ", "ᾶται", "ώμεθα", "ᾶσθε", "ῶνται"],//, "" },   //pres mid/pass indic a
    ["ων", "ᾱς", "ᾱ", "ῶμεν", "ᾶτε", "ων"],//, "" },            //impf active indic a
    ["ώμην", "ῶ", "ᾶτο", "ώμεθα", "ᾶσθε", "ῶντο"],//, "" },     //impf mid/pass indic a
    ["ῶ", "ᾷς", "ᾷ", "ῶμεν", "ᾶτε", "ῶσι(ν)"],//, "" },         //pres active subj a
    ["ῶμαι", "ᾷ", "ᾶται", "ώμεθα", "ᾶσθε", "ῶνται"],//, "" },   //pres mid/pass subj a
    ["ῷμι, ῴην", "ῷς, ῴης", "ῷ, ῴη", "ῷμεν, ῴημεν", "ῷτε, ῴητε", "ῷεν, ῴησαν"],//, "" }, //pres active opt a
    ["ῴμην", "ῷο", "ῷτο", "ῴμεθα", "ῷσθε", "ῷντο"],//, "" },   //pres mid/pass opt a
    
    ["ῶ", "εῖς", "εῖ", "οῦμεν", "εῖτε", "οῦσι(ν)"],//, "" },         //pres active indic e
    ["οῦμαι", "εῖ, ῇ", "εῖται", "ουμεθα", "εῖσθε", "οῦνται"],//, "" },   //pres mid/pass indic e
    ["ουν", "εις", "ει", "οῦμεν", "εῖτε", "ουν"],//, "" },            //impf active indic e
    ["ούμην", "οῦ", "εῖτο", "ούμεθα", "εῖσθε", "οῦντο"],//, "" },     //impf mid/pass indic e
    ["ῶ", "ῇς", "ῇ", "ῶμεν", "ῆτε", "ῶσι(ν)"],//, "" },         //pres active subj e
    ["ῶμαι", "ῇ", "ῆται", "ώμεθα", "ῆσθε", "ῶνται"],//, "" },   //pres mid/pass subj e
    ["οῖμι, οίην", "οῖς, οίης", "οῖ, οίη", "οῖμεν, οίημεν", "οῖτε, οίητε", "οῖεν, οίησαν"],//, "" },//pres act opt e
    ["οίμην", "οῖο", "οῖτο", "οίμεθα", "οῖσθε", "οῖντο"],//, "" },   //pres mid/ass opt e
    
    ["ῶ", "οῖς", "οῖ", "οῦμεν", "οῦτε", "οῦσι(ν)"],//, "" },         //pres active indic o
    ["οῦμαι", "οῖ", "οῦται", "ουμεθα", "οῦσθε", "οῦνται"],//, "" },   //pres mid/pass indic o
    ["ουν", "ους", "ου", "οῦμεν", "οῦτε", "ουν"],//, "" },            //impf active indic o
    ["ούμην", "οῦ", "οῦτο", "ούμεθα", "οῦσθε", "οῦντο"],//, "" },     //impf mid/pass indic o
    ["ῶ", "οῖς", "οῖ", "ῶμεν", "ῶτε", "ῶσι(ν)"],//, "" },         //pres active subj o
    ["ῶμαι", "οῖ", "ῶται", "ώμεθα", "ῶσθε", "ῶνται"],//, "" },   //pres mid/pass subj o
    ["οῖμι, οίην", "οῖς, οίης", "οῖ, οίη", "οῖμεν, οίημεν", "οῖτε, οίητε", "οῖεν, οίησαν"],//, "" },//pres act opt o
    ["οίμην", "οῖο", "οῖτο", "οίμεθα", "οῖσθε", "οῖντο"],//, "" },   //pres mid/ass opt o
    
    ["", "ᾱ", "ᾱ́τω",   "", "ᾶτε", "ώντων"],//, "Present Active Imperative" }, //pres. active imper a
    ["", "ῶ", "ᾱ́σθω", "", "ᾶσθε", "ᾱ́σθων"],//, "Present Middle/Passive Imperative" }, //pres. mid/pass imper a
    ["", "ει", "είτω",   "", "εῖτε", "ούντων"],//, "Present Active Imperative" }, //pres. active imper e
    ["", "οῦ", "είσθω", "", "εῖσθε", "είσθων"],//, "Present Middle/Passive Imperative" }, //pres. mid/pass imper e
    ["", "ου", "ούτω",   "", "οῦτε", "ούντων"],//, "Present Active Imperative" }, //pres. active imper o
    ["", "οῦ", "ούσθω", "", "οῦσθε", "ούσθων"],//, "Present Middle/Passive Imperative" }, //pres. mid/pass imper o
    
    ["μι", "ς", "σι(ν)", "μεν", "τε", "ᾱσι(ν)"],//, "" },   //mi
    
    ["οιμι, οιην", "οις, οιης", "οι, οιη", "οιμεν, οιημεν", "οιτε, οιητε", "οιεν, οιησαν"],//, "" },//pres act opt o
    ["", "ς", "τω", "", "τε", "ντων"],//, "" },//mi aorist active imperatives
    ["", "θι", "τω", "", "τε", "ντων"],//", "" },//mi root aorist active imperatives
    
    ["", "ο", "σθω", "", "σθε", "σθων"],//, "Root Aorist Middle Imperative" },//mi root aorist middle imperatives
    ["ν", "ς", "", "μεν", "τε", "σαν"],//, "Root Aorist Indicative" },//mi root aorist indicative
    
    ["", "οῦ", "εσθω", "", "εσθε", "εσθων"],//, "Present Middle/Passive Imperative" }, //second aorist middle/passive imperatives
    ["ειμην", "εῖο", "εῖτο, οῖτο", "ειμεθα, οιμεθα", "εῖσθε, οῖσθε", "εῖντο, οῖντο"],//, "Present Middle/Passive Optative Tithemi" }, //Exception: H&Q page 347
    ["ον", "ες", "ε", "ομεν", "ετε", "ον"],//, "Imperfect Active Indicative" } //this is only for contracted verbs when decomposed so the nu moveable doesn't show up
];



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let luw = "λω, λσω, ἔλῡσα, λέλυκα, λέλυμαι, ἐλύθην";
        let blaptw = "βλάπτω, βλάψω, ἔβλαψα, βέβλαφα, βέβλαμμαι, ἐβλάβην / ἐβλάφθην";

        let a = HcGreekVerb::from_string(1, luw, "").unwrap();
        let a1 = HcGreekVerb {id:1,pps:vec!["λω".to_string(), "λσω".to_string(), "ἔλῡσα".to_string(), "λέλυκα".to_string(), "λέλυμαι".to_string(), "ἐλύθην".to_string()],properties:"".to_string()};
        assert_eq!(a, a1);
        
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Aorist, voice:HcVoice::Active, mood:HcMood::Indicative, gender:None, case:None};
        let c = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Aorist, voice:HcVoice::Active, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b, c);
        
        assert_eq!(b.get_form().unwrap()[0].form, luw);
        assert_eq!(b.get_form().unwrap()[1].form, "ἔλῡσα");
        
        assert_eq!(b.get_form().unwrap()[2].form, "ἐλῡσ");
        
        assert_eq!(b.get_pp_num(), HcGreekPrincipalParts::Third);
        assert_eq!(b.get_pp_num() as usize, 3);
        assert_eq!(b.verb.pps[b.get_pp_num() as usize - 1], "ἔλῡσα");
        assert_eq!(b.get_pp(), "ἔλῡσα");

        let a = HcGreekVerb::from_string(1, blaptw, "").unwrap();
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Aorist, voice:HcVoice::Passive, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form().unwrap()[2].form, "ἐβλαβ / ἐβλαφθ"); 
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Present, voice:HcVoice::Active, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form().unwrap()[2].form, "βλαπτ");
        assert_eq!(b.get_endings()[0], "ω");

        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Present, voice:HcVoice::Middle, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form().unwrap()[3].form, "βλαπτομαι");
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::Second, number:HcNumber::Singular, tense:HcTense::Present, voice:HcVoice::Middle, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_endings()[0], "ει");
        assert_eq!(b.get_endings()[1], "ῃ");
        assert_eq!(b.get_form().unwrap()[3].form, "βλαπτει, βλαπτῃ");
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::Third, number:HcNumber::Singular, tense:HcTense::Present, voice:HcVoice::Middle, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form().unwrap()[3].form, "βλαπτεται");
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Plural, tense:HcTense::Present, voice:HcVoice::Middle, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form().unwrap()[3].form, "βλαπτομεθα");
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::Second, number:HcNumber::Plural, tense:HcTense::Present, voice:HcVoice::Middle, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form().unwrap()[3].form, "βλαπτεσθε");
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::Third, number:HcNumber::Plural, tense:HcTense::Present, voice:HcVoice::Middle, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form().unwrap()[3].form, "βλαπτονται");

        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Future, voice:HcVoice::Active, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form().unwrap()[2].form, "βλαψ");
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Perfect, voice:HcVoice::Active, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form().unwrap()[2].form, "βεβλαφ");
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Perfect, voice:HcVoice::Passive, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form().unwrap()[2].form, "βεβλαμ");
    }
}

