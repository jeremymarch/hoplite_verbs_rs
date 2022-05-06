#![allow(dead_code)]

extern crate rustunicodetests;
use rustunicodetests::*;
//use rustunicodetests::hgk_toggle_diacritic_str;
use rustunicodetests::hgk_strip_diacritics;
use rustunicodetests::hgk_has_diacritics;
//use rustunicodetests::hgk_transliterate;
//use rustunicodetests::hgk_convert;

#[derive(Eq, PartialEq, Debug)]
enum HcEndings {
    PresentActiveInd,
    ImperfectActiveInd,
    AoristActiveInd,
    PerfectActiveInd,
    PluperfectActiveInd,
    PresentActiveSubj,
    PresentActiveOpt,
    AoristActiveOpt,
    PresentMidpassInd,
    ImperfectMidpassInd,
    AoristPassiveInd,
    AoristMidInd,
    AoristPassiveSubj,
    AoristPassiveOpt,
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

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum HcPerson {
    First,
    Second,
    Third,
}

impl HcPerson {
    fn value(&self) -> &str {
        match *self {
            HcPerson::First => "1",
            HcPerson::Second => "2",
            HcPerson::Third => "3",
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum HcNumber {
    Singular,
    Dual,
    Plural,
}

impl HcNumber {
    fn value(&self) -> &str {
        match *self {
            HcNumber::Singular => "s",
            HcNumber::Dual => "d",
            HcNumber::Plural => "p",
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum HcTense {
    Present,
    Future,
    Imperfect,
    Aorist,
    Perfect,
    Pluperfect,
}

impl HcTense {
    fn value(&self) -> &str {
        match *self {
            HcTense::Present => "Present",
            HcTense::Future => "Future",
            HcTense::Imperfect => "Imperfect",
            HcTense::Aorist => "Aorist",
            HcTense::Perfect => "Perfect",
            HcTense::Pluperfect => "Pluperfect",
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum HcVoice {
    Active,
    Middle,
    Passive,
}

impl HcVoice {
    fn value(&self) -> &str {
        match *self {
            HcVoice::Active => "Active",
            HcVoice::Middle => "Middle",
            HcVoice::Passive => "Passive",
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum HcMood {
    Indicative,
    Subjunctive,
    Optative,
    Imperative,
    Infinitive,
    Participle,
}

impl HcMood {
    fn value(&self) -> &str {
        match *self {
            HcMood::Indicative => "Indicative",
            HcMood::Subjunctive => "Subjunctive",
            HcMood::Optative => "Optative",
            HcMood::Imperative => "Imperative",
            HcMood::Infinitive => "Infinitive",
            HcMood::Participle => "Participle",
        }
    }
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

#[derive(PartialEq, Debug)]
enum HcDeponentType {
    NotDeponent,
    MiddleDeponent,
    PassiveDeponent,
    PartialDeponent,
    GignomaiDeponent,
    MiddleDeponentHgeomai,
}

pub const REGULAR                           :u32 = 0x000;
pub const CONSONANT_STEM_PERFECT_PHI        :u32 = 0x0001;
pub const CONSONANT_STEM_PERFECT_MU_PI      :u32 = 0x0002;
pub const CONSONANT_STEM_PERFECT_KAPPA      :u32 = 0x0004;
pub const CONSONANT_STEM_PERFECT_SIGMA      :u32 = 0x0008;
pub const CONSONANT_STEM_PERFECT_SIGMA_2    :u32 = 0x0010;
pub const CONSONANT_STEM_PERFECT_LAMBDA     :u32 = 0x0020;
pub const CONSONANT_STEM_PERFECT_PI         :u32 = 0x0040;
pub const CONSONANT_STEM_PERFECT_BETA       :u32 = 0x0080;
pub const CONSONANT_STEM_PERFECT_GAMMA      :u32 = 0x0100;
pub const CONSONANT_STEM_PERFECT_CHI        :u32 = 0x0200;
pub const PREFIXED                          :u32 = 0x0400;
pub const CONTRACTED_FUTURE_ALPHA           :u32 = 0x0800;
pub const CONSONANT_STEM_PERFECT_NU         :u32 = 0x1000;
pub const MI_VERB                           :u32 = 0x2000;

#[derive(Eq, PartialEq, Debug)]
pub struct HcGreekVerb {
    id: u32,
    pps: Vec<String>,
    properties: u32,
}

impl HcGreekVerb {
    fn from_string(id:u32, pps:&str, props:u32) -> Option<HcGreekVerb> {
        let x: Vec<String> = pps.split(',').map(|s| s.trim().to_owned()).collect();
        if x.len() == 6 {
            Some(HcGreekVerb {
                id,
                pps: x,
                properties: props
            })
        }
        else {
            None
        }
    }

    //page 316 in h&q
    fn deponent_type(&self) -> HcDeponentType {
        if self.pps[0].ends_with("γίγνομαι") { //and παραγίγνομαι
            //From Hardy: "I guess γίγνομαι is technically a partial deponent, though in practice I don't think we're in the habit of calling it that.  We simply say that's a deponent (i.e. a middle deponent) with one active PP."
            HcDeponentType::GignomaiDeponent //see H&Q page 382. fix me, there may be a better way to do this without separate case
        }
        /*else if ( utf8HasSuffix(v->present, "μαι")) {
            return MIDDLE_DEPONENT;
        }*/
        else if self.pps[0].ends_with("μαι") && self.pps[1].ends_with("μαι") && self.pps[2].ends_with("μην") && self.pps[3] == "" /* && utf8HasSuffix(v->perfmid, "μαι") */ && self.pps[5] == "" {
            HcDeponentType::MiddleDeponent
        }
        //this gets μετανίσταμαι and ἐπανίσταμαι: middle deponents which happen to have an active perfect and root aorist
        else if self.pps[0].ends_with("μαι") && self.pps[1].ends_with("μαι") && self.pps[2].ends_with("ην") /* && utf8HasSuffix(v->perfmid, "μαι") */ && self.pps[5] == "" {
            HcDeponentType::MiddleDeponent
        }
        else if self.pps[0].ends_with("μαι") && self.pps[1].ends_with("μαι") && self.pps[2] == "" && self.pps[3] == "" && self.pps[4].ends_with("μαι") && self.pps[5] != "" {
            HcDeponentType::PassiveDeponent
        }
        else if self.pps[0].ends_with("ἐπίσταμαι") {
            HcDeponentType::PassiveDeponent //close enough
        }
        else if self.pps[0].ends_with("ἡγέομαι") { //doesn't seem to have future passive, though?
            HcDeponentType::MiddleDeponentHgeomai //we call it a middle deponent which happens to also have a 6th pp
        }
        else if self.pps[0].ends_with("μαι") || self.pps[1].ends_with("μαι") || self.pps[2].ends_with("μην") {
            HcDeponentType::PartialDeponent
        }
        else {
            HcDeponentType::NotDeponent
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

static SEPARATOR: &str = "‐";
static BLANK: &str = "—";

trait HcVerbForms {
    fn get_form(&self, decomposed:bool) -> Result<Vec<Step>, &str>;
    fn get_pp_num(&self) -> HcGreekPrincipalParts;
    fn get_pp(&self) -> Option<String>;
    fn strip_ending(&self, pp_num:usize, form:String) -> Result<String, &str>;
    fn add_ending(&self, stem:&str, ending:&str, decomposed:bool) -> Result<String, &str>;
    fn get_endings(&self) -> Option<Vec<&str>>;
    fn accent_verb(&self, form:&str) -> String;
    fn accent_syllable(&self, word:&str, syllable:u8, accent:u32) -> String;
    fn get_label(&self) -> String;
    fn add_augment(&self, stem:&str, decomposed:bool) -> String;
    fn deaugment(&self, stem:&str, decomposed:bool) -> String;
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

fn get_voice_label(tense:HcTense, voice:HcVoice, mood:HcMood, deponent_type:HcDeponentType) -> String {
    if voice == HcVoice::Middle && mood == HcMood::Imperative {
        String::from("Middle")
    }
    else if voice == HcVoice::Passive && mood == HcMood::Imperative {
        String::from("Passive")
    }
    else if tense != HcTense::Future && tense != HcTense::Aorist && voice == HcVoice::Middle {
        String::from("Middle (Middle/Passive)")
    }
    else if tense != HcTense::Future && tense != HcTense::Aorist && voice == HcVoice::Passive {
        String::from("Passive (Middle/Passive)")
    }
    else {
        String::from(voice.value())
    }
}

impl HcVerbForms for HcGreekVerbForm<'_> {
    /*
    fn new() -> HcGreekVerbForm {

    }*/

    fn get_label(&self) -> String {
        "".to_string()
    }

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
                else if form.ends_with("ον") {
                    return Ok(form.strip_suffix("ον").unwrap().to_string());
                }
                else if form.ends_with("ομην") {
                    return Ok(form.strip_suffix("ομην").unwrap().to_string());
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

    fn add_augment(&self, stem:&str, decomposed:bool) -> String {
        let mut local_stem = stem.to_string();
        if decomposed {
            if local_stem.starts_with('ἠ') {
                String::from(local_stem)
            }
            else {
                format!("ε {} {}", SEPARATOR, local_stem)
            }
        }
        else {
            if self.verb.pps[0].starts_with('ἐ') || self.verb.pps[0].starts_with('ἄ') {
                local_stem.remove(0);
                format!("ἠ{}", local_stem)
            }
            else if local_stem.starts_with('ἠ') {
                String::from(local_stem)
            }
            else {
                format!("ἐ{}", local_stem)
            }
        }
    }

    fn add_ending(&self, stem:&str, ending:&str, decomposed:bool) -> Result<String, &str> {
        let mut local_stem = stem.to_string();
        let mut local_ending = ending.to_string();

        if ((self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect) && (self.voice == HcVoice::Middle || self.voice == HcVoice::Passive)) && local_stem == "πεπεμ" || local_stem == "ἐπεπεμ" || local_stem == format!("ε {} πεπεμ", SEPARATOR) {
            if local_ending.starts_with("ντ") {
                return Ok(String::from(BLANK));
            }
            else if decomposed {
                local_stem = format!("{}π", local_stem);
            }
            else if local_ending.starts_with("σθ") {
                local_ending.remove(0);
                local_ending = format!("φ{}", local_ending);
            }
            else if local_ending.starts_with('σ') {
                local_ending.remove(0);
                local_ending = format!("ψ{}", local_ending);
            }
            else if local_ending.starts_with('τ') {
                local_ending = format!("π{}", local_ending);
            }
        }
        else if ((self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect) && 
            (self.voice == HcVoice::Middle || self.voice == HcVoice::Passive)) && local_stem.ends_with('μ') {

            if local_ending.starts_with("ντ") {
                return Ok(String::from(BLANK));
            }
            else if decomposed {
                local_stem.pop();
                if self.verb.properties & CONSONANT_STEM_PERFECT_PI == CONSONANT_STEM_PERFECT_PI {
                    local_stem = format!("{}π", local_stem);    
                }
                else if self.verb.properties & CONSONANT_STEM_PERFECT_BETA == CONSONANT_STEM_PERFECT_BETA {
                    local_stem = format!("{}β", local_stem);    
                }
                else {
                    local_stem = format!("{}φ", local_stem);
                }
            }
            else if local_ending.starts_with("σθ") {
                local_ending.remove(0);
                local_stem.pop();
                local_ending = format!("φ{}", local_ending);
            }
            else if local_ending.starts_with('σ') {
                local_stem.pop();
                local_ending.remove(0);
                local_ending = format!("ψ{}", local_ending);
            }
            else if local_ending.starts_with('τ') {
                local_stem.pop();
                local_ending = format!("π{}", local_ending);
            }
        }
        else if ((self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect) && 
            (self.voice == HcVoice::Middle || self.voice == HcVoice::Passive)) && local_stem.ends_with('γ') {

            if local_ending.starts_with("ντ") {
                return Ok(String::from(BLANK));
            }
            else if decomposed {
                local_stem.pop();
                if self.verb.properties & CONSONANT_STEM_PERFECT_GAMMA == CONSONANT_STEM_PERFECT_GAMMA {
                    local_stem = format!("{}γ", local_stem);    
                }
                else if self.verb.properties & CONSONANT_STEM_PERFECT_CHI == CONSONANT_STEM_PERFECT_CHI {
                    local_stem = format!("{}χ", local_stem);    
                }
                else {
                    local_stem = format!("{}κ", local_stem);
                }
            }
            else if local_ending.starts_with("σθ") {
                local_ending.remove(0);
                local_stem.pop();
                local_ending = format!("χ{}", local_ending);
            }
            else if local_ending.starts_with('σ') {
                local_stem.pop();
                local_ending.remove(0);
                local_ending = format!("ξ{}", local_ending);
            }
            else if local_ending.starts_with('τ') {
                local_stem.pop();
                local_ending = format!("κ{}", local_ending);
            }
        }
        else if ((self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect) && (self.voice == HcVoice::Middle || self.voice == HcVoice::Passive)) && local_stem.ends_with('σ') {
            if local_ending.starts_with("ντ") {
                return Ok(String::from(BLANK));
            }
            else if local_ending.starts_with('σ') && !decomposed {
                local_ending.remove(0);
            }
        }

        let future_passive_suffix = if self.tense == HcTense::Future && self.voice == HcVoice::Passive {
            if decomposed {
                format!("ησ {} ", SEPARATOR)
            }
            else {
                String::from("ησ")
            }
        }
        else {
            String::from("")
        };

        if decomposed {
            Ok(format!("{} {} {}{}", local_stem, SEPARATOR, future_passive_suffix, local_ending))
        }
        else {
            Ok(format!("{}{}{}", local_stem, future_passive_suffix, local_ending))
        }
    }

    fn deaugment(&self, a:&str, decomposed:bool) -> String {
        let mut loc = a.to_string();
        if loc.starts_with('ἠ') && self.verb.pps[0].starts_with('ἐ') {
            loc.remove(0);
            loc = format!("ἐ{}", loc);
        }
        else if loc.starts_with('ἠ') && self.verb.pps[0].starts_with('ἄ') {
            loc.remove(0);
            loc = format!("ἀ{}", loc);
        }
        else {
            loc.remove(0);
        }
        //add decomposed augment back
        if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative && decomposed {
            format!("ε {} {}", SEPARATOR, loc)
        }
        else {
            loc
        }
    }

    fn get_form(&self, decomposed:bool) -> Result<Vec<Step>, &str> {
        let mut steps = Vec::new();
        if self.mood == HcMood::Imperative && self.person == HcPerson::First {
            steps.push(Step{form:"".to_string(), explanation:"".to_string()});
            return Ok(steps);
        }

        let f = self.verb.pps.join(", ");
        let e = "Principal Parts".to_string();
        steps.push(Step{form:f, explanation:e});

        let pp_num = self.get_pp_num() as usize;
        let f = &self.verb.pps[pp_num - 1];
        let e = "Choose Principal Part".to_string();
        steps.push(Step{form:f.to_string(), explanation:e});

        if f == BLANK {
            steps.push(Step{form:String::from(""), explanation:String::from("Blank principal part")});
            return Ok(steps);
        }
        
        let mut pps_without_ending = Vec::new();
        //strip accent: internally (not as a step)
        let f = hgk_strip_diacritics(f, HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE);
        let alt_pps = f.split(" / ");
        for alt_pp in alt_pps {
            let y = self.strip_ending(pp_num, alt_pp.to_string());
            if y.is_err() {
                return Err("error stripping ending");
            }
            pps_without_ending.push(y.unwrap());
        }

        let f = pps_without_ending.join(" / ");
        let e = "Remove ending from Principal Part".to_string();
        steps.push(Step{form:f, explanation:e});

        let mut pps_add_augment = Vec::new();
        //add augment
        if self.tense == HcTense::Imperfect || self.tense == HcTense::Pluperfect {
            for a in &pps_without_ending {
                pps_add_augment.push(self.add_augment(a, decomposed));
            }
            pps_without_ending = pps_add_augment;
        }
        else /* remove augment */ if (self.tense == HcTense::Aorist && self.mood == HcMood::Indicative && decomposed) || 
            (self.tense == HcTense::Aorist && self.mood != HcMood::Indicative) || 
            (self.tense == HcTense::Future && self.voice == HcVoice::Passive) {
            
            for a in &pps_without_ending {
                pps_add_augment.push(self.deaugment(&a, decomposed));
            }
            pps_without_ending = pps_add_augment;
        }

        let mut add_ending_collector = Vec::new();
        let mut add_accent_collector = Vec::new();
        for a in pps_without_ending {
            let endings_for_form = self.get_endings();
            if endings_for_form == None {
                return Err("Illegal form ending");
            }
            for e in endings_for_form.unwrap() {
                if self.tense == HcTense::Aorist && self.voice == HcVoice::Passive && self.mood == HcMood::Imperative && self.person == HcPerson::Second && self.number == HcNumber::Singular {
                    if a.ends_with('θ') || a.ends_with('φ') || a.ends_with('χ') {
                        if e == "ηθι" {
                            continue;
                        }
                    }
                    else {
                        if e == "ητι" {
                            continue;
                        }
                    }
                }

                if self.verb.pps[0].starts_with("βλάπτω") && a == "βλαφθ" && self.tense == HcTense::Future && self.voice == HcVoice::Passive {
                    continue;
                }

                //skip alternate here because same
                if self.verb.pps[0].starts_with("σῴζω") && ((a.ends_with("σεσω") && self.person == HcPerson::Second) || (a.ends_with("σεσωσ") && self.person == HcPerson::Third && self.number == HcNumber::Plural)) {
                    continue;
                }

                let ending = if decomposed { hgk_strip_diacritics(e, HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE) } else { e.to_string() };
                let stem = if decomposed && self.tense == HcTense::Aorist && self.voice == HcVoice::Passive && self.mood == HcMood::Subjunctive { format!("{}ε", a.to_owned()) } else { a.to_owned() };
                let y = self.add_ending(&stem, &ending, decomposed);
                let y = match y {
                    Ok(y) => y,
                    _ => return Err("Error adding ending")
                };

                add_ending_collector.push(y.to_string());
                if !decomposed {
                    add_accent_collector.push( if !hgk_has_diacritics(&y, HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE) { self.accent_verb(&y) } else { y } );
                }
                //println!("z1 {:?}", z1);
                //imperfect/pluperfect: add augment
                //aorist subj/opt/imper/inf/ptc: remove augment
                //contract contracted verbs
                //accent
            }
        }
        let f = add_ending_collector.join(" / ");
        let e = "Add ending".to_string();
        steps.push(Step{form:f, explanation:e});   
        
        if !decomposed {
            let f = add_accent_collector.join(" / ");
            let e = "Accent verb".to_string();
            steps.push(Step{form:f, explanation:e});   
        }

        Ok(steps)
    }

    fn accent_verb(&self, word:&str) -> String {
        let syl = analyze_syllable_quantities(word, self.person, self.number, self.mood);

        let accent;
        let letter_index;
        if syl.len() > 2 && !syl.last().unwrap().is_long { //acute on antepenult
            accent = HGK_ACUTE;
            letter_index = syl[0].index;
        }
        else if syl.len() == 2 && syl[0].is_long && !syl[1].is_long {
            if (syl[1].letters == "αι" || syl[1].letters == "οι") && self.mood == HcMood::Optative {
                accent = HGK_ACUTE; //exception to the exception for optative 3rd singular: acute on penult
            }
            else {
                accent = HGK_CIRCUMFLEX; //circumflex on penult
            }
            letter_index = syl[0].index;
        }
        else if syl.len() > 1 { //acute on penult
            accent = HGK_ACUTE;
            letter_index = syl[syl.len() - 2].index;
        }
        else {
            return String::from(word);
        }

        self.accent_syllable(word, letter_index, accent)
    }

    fn accent_syllable(&self, word:&str, letter_index_from_end:u8, accent:u32) -> String {
        let v = word.gkletters().rev().enumerate().map(|(x, mut a)| { 
            if x == letter_index_from_end as usize {
                a.toggle_diacritic(accent, true);
                //println!("abc {:?} {:?} {:?}", a.letter, accent, letter_index_from_end);
            } 
            a}).collect::<Vec<HGKLetter>>();

            let s = v.iter().rev().map(|a|{ a.to_string(HgkUnicodeMode::Precomposed)}).collect::<String>();
        s
    }

    fn get_pp(&self) -> Option<String> {
        let num = self.get_pp_num() as usize;
        if (1..=6).contains(&num) {
            Some(self.verb.pps[num - 1].to_string())
        }
        else {
            None
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
    fn get_endings(&self) -> Option<Vec<&str>> {
        let ending = match self.tense {
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
                            HcMood::Indicative => HcEndings::ImperfectActiveInd,
                            HcMood::Subjunctive => HcEndings::NotImplemented,
                            HcMood::Optative => HcEndings::NotImplemented,
                            HcMood::Imperative => HcEndings::NotImplemented,
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    },
                    HcVoice::Middle | HcVoice::Passive => {
                        match self.mood {
                            HcMood::Indicative => HcEndings::ImperfectMidpassInd,
                            HcMood::Subjunctive => HcEndings::NotImplemented,
                            HcMood::Optative => HcEndings::NotImplemented,
                            HcMood::Imperative => HcEndings::NotImplemented,
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
                            HcMood::Subjunctive => HcEndings::NotImplemented,
                            HcMood::Optative => HcEndings::PresentActiveOpt,
                            HcMood::Imperative => HcEndings::NotImplemented,
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    },
                    HcVoice::Middle | HcVoice::Passive => {
                        match self.mood {
                            HcMood::Indicative => HcEndings::PresentMidpassInd,
                            HcMood::Subjunctive => HcEndings::NotImplemented,
                            HcMood::Optative => HcEndings::PresentMidpassOpt,
                            HcMood::Imperative => HcEndings::NotImplemented,
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    }
                }
            },
            HcTense::Aorist => {
                match self.voice {
                    HcVoice::Active => {
                        if self.verb.pps[2].ends_with("ον") {
                            match self.mood {
                                HcMood::Indicative => HcEndings::ImperfectActiveInd,
                                HcMood::Subjunctive => HcEndings::PresentActiveSubj,
                                HcMood::Optative => HcEndings::PresentActiveOpt,
                                HcMood::Imperative => HcEndings::PresentActiveImperative,
                                HcMood::Infinitive => HcEndings::NotImplemented,
                                HcMood::Participle => HcEndings::NotImplemented,
                            }
                        }
                        else {
                            match self.mood {
                                HcMood::Indicative => HcEndings::AoristActiveInd,
                                HcMood::Subjunctive => HcEndings::PresentActiveSubj,
                                HcMood::Optative => HcEndings::AoristActiveOpt,
                                HcMood::Imperative => HcEndings::AoristActiveImperative,
                                HcMood::Infinitive => HcEndings::NotImplemented,
                                HcMood::Participle => HcEndings::NotImplemented,
                            }                            
                        }
                    },
                    HcVoice::Middle => {
                        if self.verb.pps[2].ends_with("ον") || self.verb.pps[2].ends_with("ομην") {
                            match self.mood {
                                HcMood::Indicative => HcEndings::ImperfectMidpassInd,
                                HcMood::Subjunctive => HcEndings::PresentMidpassSubj,
                                HcMood::Optative => HcEndings::PresentMidpassOpt,
                                HcMood::Imperative => HcEndings::SecondAoristMiddleImperative,
                                HcMood::Infinitive => HcEndings::NotImplemented,
                                HcMood::Participle => HcEndings::NotImplemented,
                            }
                        }
                        else {
                            match self.mood {
                                HcMood::Indicative => HcEndings::AoristMidInd,
                                HcMood::Subjunctive => HcEndings::PresentMidpassSubj,
                                HcMood::Optative => HcEndings::AoristMiddleOpt,
                                HcMood::Imperative => HcEndings::AoristMiddleImperative,
                                HcMood::Infinitive => HcEndings::NotImplemented,
                                HcMood::Participle => HcEndings::NotImplemented,
                            }
                        }
                    },
                    HcVoice::Passive => {
                        match self.mood {
                            HcMood::Indicative => HcEndings::AoristPassiveInd,
                            HcMood::Subjunctive => HcEndings::AoristPassiveSubj,
                            HcMood::Optative => HcEndings::AoristPassiveOpt,
                            HcMood::Imperative => HcEndings::AoristPassiveImperative,
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
                            HcMood::Indicative => HcEndings::PerfectActiveInd,
                            HcMood::Subjunctive => HcEndings::NotImplemented,
                            HcMood::Optative => HcEndings::NotImplemented,
                            HcMood::Imperative => HcEndings::NotImplemented,
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    },
                    HcVoice::Middle | HcVoice::Passive => {
                        match self.mood {
                            HcMood::Indicative => HcEndings::PerfectMidpassInd,
                            HcMood::Subjunctive => HcEndings::NotImplemented,
                            HcMood::Optative => HcEndings::NotImplemented,
                            HcMood::Imperative => HcEndings::NotImplemented,
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
                            HcMood::Indicative => HcEndings::PluperfectActiveInd,
                            HcMood::Subjunctive => HcEndings::NotImplemented,
                            HcMood::Optative => HcEndings::NotImplemented,
                            HcMood::Imperative => HcEndings::NotImplemented,
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    },
                    HcVoice::Middle | HcVoice::Passive => {
                        match self.mood {
                            HcMood::Indicative => HcEndings::PluperfectMidpassInd,
                            HcMood::Subjunctive => HcEndings::NotImplemented,
                            HcMood::Optative => HcEndings::NotImplemented,
                            HcMood::Imperative => HcEndings::NotImplemented,
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    }
                }
            },
        };

        if ending == HcEndings::NotImplemented {
            return None;
        }

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

        Some(ENDINGS[ending as usize][person_number].split(',').collect())
    }
}

struct SyllableAnalysis {
    letters: String,
    is_long: bool,
    index: u8,
}

fn analyze_syllable_quantities(word:&str, p:HcPerson, n:HcNumber, m:HcMood) -> Vec<SyllableAnalysis> {
    let mut letters = word.gkletters();

    let mut letter_num = 0;
    let mut last_letter = '\u{0000}';
    let mut res = Vec::new();
    loop {
        match letters.next_back() {
            Some(x) => { 
                //println!("letter: {:?}", x);
                match x.letter_type() {
                    HgkLetterType::HgkLongVowel => {
                        last_letter = '\u{0000}';
                        res.push(SyllableAnalysis {letters: x.to_string(HgkUnicodeMode::Precomposed), is_long: true, index: letter_num});
                    },
                    HgkLetterType::HgkShortVowel => {
                        if x.letter == 'υ' || x.letter == 'ι' && (x.diacritics & HGK_DIAERESIS) != HGK_DIAERESIS {
                            last_letter = x.letter;
                            //res.push((x.letter.to_string(), false, letter_num)); //add short, might be replaced by diphthong
                            res.push(SyllableAnalysis {letters: x.letter.to_string(), is_long: false, index: letter_num});
                        }
                        else {
                            if last_letter != '\u{0000}' && (x.letter == 'ε' || x.letter == 'α' || x.letter == 'ο') {
                                res.pop();
                                let mut s = String::from(x.letter);
                                s.push(last_letter);

                                let is_short = letter_num == 1 && (x.letter == 'α' || x.letter == 'ο') && last_letter == 'ι';//final diphthongs short accent
                                if is_short && p == HcPerson::Third && n == HcNumber::Singular && m == HcMood::Optative {
                                    //res.push((s, true, letter_num - 1));
                                    res.push(SyllableAnalysis {letters: s, is_long: true, index: letter_num - 1});
                                }
                                else {
                                    //res.push((s, !is_short, letter_num - 1));
                                    res.push(SyllableAnalysis {letters: s, is_long: !is_short, index: letter_num - 1});
                                }
                            }
                            else {
                                //res.push((x.letter.to_string(), false, letter_num));
                                res.push(SyllableAnalysis {letters: x.letter.to_string(), is_long: false, index: letter_num});
                            }
                            last_letter = '\u{0000}';
                        }
                    },
                    _ => {
                        last_letter = '\u{0000}';
                    }
                }
                if res.len() > 2 {
                    break;
                }
                letter_num += 1;
            },
            None => {
                break;
            },
        }
    }
    res.reverse();
    res
}

static ENDINGS: &[[&str; 6]; /*63*/63] = &[
    ["ω", "εις", "ει", "ομεν", "ετε", "ουσι(ν)"],//, "Present Active Indicative" },
    ["ον", "ες", "ε(ν)", "ομεν", "ετε", "ον"],//, "Imperfect Active Indicative" },
    ["α", "ας", "ε(ν)", "αμεν", "ατε", "αν"],//, "Aorist Active Indicative" },
    ["α", "ας", "ε(ν)", "αμεν", "ατε", "ᾱσι(ν)"],//, "Perfect Active Indicative" },
    ["η", "ης", "ει(ν)", "εμεν", "ετε", "εσαν"],//, "Pluperfect Active Indicative" },
    ["ω", "ῃς", "ῃ", "ωμεν", "ητε", "ωσι(ν)"],//, "Present Active Subjunctive" },
    ["οιμι", "οις", "οι", "οιμεν", "οιτε", "οιεν"],//, "Present Active Optative" },
    ["αιμι", "αις,ειας", "αι,ειε(ν)", "αιμεν", "αιτε", "αιεν,ειαν"],//, "Aorist Active Optative" },
    ["ομαι", "ει,ῃ", "εται", "ομεθα", "εσθε", "ονται"],//, "Present Middle/Passive Indicative" },
    ["ομην", "ου", "ετο", "ομεθα", "εσθε", "οντο"],//, "Imperfect Middle/Passive Indicative" },
    ["ην", "ης", "η", "ημεν", "ητε", "ησαν"],//, "Aorist Passive Indicative" },
    ["αμην", "ω", "ατο", "αμεθα", "ασθε", "αντο"],//, "Aorist Middle Indicative" },
    ["ῶ", "ῇς", "ῇ", "ῶμεν", "ῆτε", "ῶσι(ν)"],//***, "Aorist Passive Subjunctive" },
    ["ειην", "ειης", "ειη", "εῖμεν,ειημεν", "εῖτε,ειητε", "εῖεν,ειησαν"],//, "Aorist Passive Optative" },
    ["αιμην", "αιο", "αιτο", "αιμεθα", "αισθε", "αιντο"],//, "Aorist Middle Optative" },
    ["μαι", "σαι", "ται", "μεθα", "σθε", "νται"],//, "Perfect Middle/Passive Indicative" },
    ["μην", "σο", "το", "μεθα", "σθε", "ντο"],//, "Pluperfect Middle/Passive Indicative" },
    ["ωμαι", "ῃ", "ηται", "ωμεθα", "ησθε", "ωνται"],//, "Present Middle/Passive Subjunctive" },
    ["οιμην", "οιο", "οιτο", "οιμεθα", "οισθε", "οιντο"],//, "Present Middle/Passive Optative" },
    ["", "ε", "ετω",   "", "ετε", "οντων"],//, "Present Active Imperative" },
    ["", "ου", "εσθω", "", "εσθε", "εσθων"],//, "Present Middle/Passive Imperative" },
    ["", "ον", "ατω",  "", "ατε", "αντων"],//, "Aorist Active Imperative" },
    ["", "αι", "ασθω", "", "ασθε", "ασθων"],//, "Aorist Middle Imperative" },
    ["", "ητι,ηθι", "ητω", "", "ητε", "εντων"],//, "Aorist Passive Imperative" },
    

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
    
    ["οιμι,οιην", "οις,οιης", "οι,οιη", "οιμεν,οιημεν", "οιτε,οιητε", "οιεν,οιησαν"],//, "" },//pres act opt o
    ["", "ς", "τω", "", "τε", "ντων"],//, "" },//mi aorist active imperatives
    ["", "θι", "τω", "", "τε", "ντων"],//", "" },//mi root aorist active imperatives
    
    ["", "ο", "σθω", "", "σθε", "σθων"],//, "Root Aorist Middle Imperative" },//mi root aorist middle imperatives
    ["ν", "ς", "", "μεν", "τε", "σαν"],//, "Root Aorist Indicative" },//mi root aorist indicative
    
    ["", "οῦ", "εσθω", "", "εσθε", "εσθων"],//, "Present Middle/Passive Imperative" }, //second aorist middle/passive imperatives
    ["ειμην", "εῖο", "εῖτο,οῖτο", "ειμεθα,οιμεθα", "εῖσθε,οῖσθε", "εῖντο,οῖντο"],//, "Present Middle/Passive Optative Tithemi" }, //Exception: H&Q page 347
    ["ον", "ες", "ε", "ομεν", "ετε", "ον"],//***, "Imperfect Active Indicative" } //this is only for contracted verbs when decomposed so the nu moveable doesn't show up
];



#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    //use unicode_normalization::UnicodeNormalization;

    #[test]
    fn accent_tests() {
        let luw = "λω, λσω, ἔλῡσα, λέλυκα, λέλυμαι, ἐλύθην";
        let a = HcGreekVerb::from_string(1, luw, REGULAR).unwrap();
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Aorist, voice:HcVoice::Active, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form(false).unwrap()[1].form, "ἔλῡσα");
        assert_eq!(b.accent_verb("λελυμαι"), "λέλυμαι");
        assert_eq!(b.accent_verb("λυ\u{0304}ε"), "λῦε");
        assert_eq!(b.accent_verb("λ\u{1FE1}ε"), "λῦε");
        assert_eq!(b.accent_verb("ἐβλαβην"), "ἐβλάβην");
    }

    #[test]
    fn it_works() {
        let luw = "λω, λσω, ἔλῡσα, λέλυκα, λέλυμαι, ἐλύθην";
        let blaptw = "βλάπτω, βλάψω, ἔβλαψα, βέβλαφα, βέβλαμμαι, ἐβλάβην / ἐβλάφθην";

        let luwverb = HcGreekVerb::from_string(1, luw, REGULAR).unwrap();
        let a1 = HcGreekVerb {id:1,pps:vec!["λω".to_string(), "λσω".to_string(), "ἔλῡσα".to_string(), "λέλυκα".to_string(), "λέλυμαι".to_string(), "ἐλύθην".to_string()],properties: REGULAR};
        assert_eq!(luwverb, a1);
        
        let b = HcGreekVerbForm {verb:&luwverb, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Aorist, voice:HcVoice::Active, mood:HcMood::Indicative, gender:None, case:None};
        let c = HcGreekVerbForm {verb:&luwverb, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Aorist, voice:HcVoice::Active, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b, c);
        
        assert_eq!(b.get_form(false).unwrap()[0].form, luw);
        assert_eq!(b.get_form(false).unwrap()[1].form, "ἔλῡσα");
        
        assert_eq!(b.get_form(false).unwrap()[2].form, "ἐλῡσ");
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "ἔλῡσα");
        
        assert_eq!(b.get_pp_num(), HcGreekPrincipalParts::Third);
        assert_eq!(b.get_pp_num() as usize, 3);
        assert_eq!(b.verb.pps[b.get_pp_num() as usize - 1], "ἔλῡσα");
        assert_eq!(b.get_pp(), Some(String::from("ἔλῡσα")));

        let a = HcGreekVerb::from_string(1, blaptw, REGULAR).unwrap();
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Aorist, voice:HcVoice::Passive, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form(false).unwrap()[2].form, "ἐβλαβ / ἐβλαφθ"); 
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Present, voice:HcVoice::Active, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form(false).unwrap()[2].form, "βλαπτ");
        assert_eq!(b.get_endings().unwrap()[0], "ω");

        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Present, voice:HcVoice::Middle, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form(false).unwrap()[3].form, "βλαπτομαι");
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::Second, number:HcNumber::Singular, tense:HcTense::Present, voice:HcVoice::Middle, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_endings().unwrap()[0], "ει");
        assert_eq!(b.get_endings().unwrap()[1], "ῃ");
        assert_eq!(b.get_form(false).unwrap()[3].form, "βλαπτει / βλαπτῃ");
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::Third, number:HcNumber::Singular, tense:HcTense::Present, voice:HcVoice::Middle, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form(false).unwrap()[3].form, "βλαπτεται");
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Plural, tense:HcTense::Present, voice:HcVoice::Middle, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form(false).unwrap()[3].form, "βλαπτομεθα");
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::Second, number:HcNumber::Plural, tense:HcTense::Present, voice:HcVoice::Middle, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form(false).unwrap()[3].form, "βλαπτεσθε");
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::Third, number:HcNumber::Plural, tense:HcTense::Present, voice:HcVoice::Middle, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form(false).unwrap()[3].form, "βλαπτονται");

        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Future, voice:HcVoice::Active, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form(false).unwrap()[2].form, "βλαψ");
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Perfect, voice:HcVoice::Active, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form(false).unwrap()[2].form, "βεβλαφ");
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Perfect, voice:HcVoice::Passive, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form(false).unwrap()[2].form, "βεβλαμ");

        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Pluperfect, voice:HcVoice::Passive, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "ἐβεβλάμμην");

        for v in [HcVoice::Active,HcVoice::Middle,HcVoice::Passive] {
            for x in [HcTense::Present, HcTense::Imperfect, HcTense::Future, HcTense::Aorist, HcTense::Perfect, HcTense::Pluperfect] {    
                for m in [HcMood::Indicative, HcMood::Subjunctive,HcMood::Optative,HcMood::Imperative] {
                    if ((m == HcMood::Subjunctive || m == HcMood::Optative || m == HcMood::Imperative) && (x == HcTense::Imperfect || x == HcTense::Perfect || x == HcTense::Pluperfect)) || x == HcTense::Future && (m == HcMood::Subjunctive || m == HcMood::Imperative) {
                        continue;
                    }
                    let mut line = Vec::new();     
                    for z in [HcNumber::Singular, HcNumber::Plural] {
                        for y in [HcPerson::First, HcPerson::Second, HcPerson::Third] {
                            if m == HcMood::Imperative && y == HcPerson::First {
                                line.push("---".to_string());
                                continue;
                            }
                            let b = HcGreekVerbForm {verb:&luwverb, person:y, number:z, tense:x, voice:v, mood:m, gender:None, case:None};
                            line.push(b.get_form(false).unwrap().last().unwrap().form.to_string());
                        }
                    }
                    println!("{}", line.join(", "));
                }
            }
        }
    }
    
    #[test]
    fn check_forms() { 
        let mut paradigm_line = String::new();
        if let Ok(pp_file) = File::open("testdata/pp.txt") {
            if let Ok(paradigm_file) = File::open("testdata/new.txt") {
                let pp_reader = BufReader::new(pp_file);
                let mut paradigm_reader = BufReader::new(paradigm_file);
                
                for (idx, pp_line) in pp_reader.lines().enumerate() {
                    if let Ok(line) = pp_line {
                        let properties = if line.starts_with("θάπτω") || line.starts_with("κλέπτω") || line.starts_with("λείπω") {
                            CONSONANT_STEM_PERFECT_PI
                        }
                        else if line.starts_with("τάττω") || line.starts_with("πρᾱ́ττω") {
                            CONSONANT_STEM_PERFECT_GAMMA
                        }
                        else if line.starts_with("ἄρχω") {
                            CONSONANT_STEM_PERFECT_CHI
                        }
                        else if line.starts_with("βλάπτω") {
                            CONSONANT_STEM_PERFECT_BETA
                        }
                        else {
                            REGULAR
                        };
                        let verb = HcGreekVerb::from_string(idx as u32, &line, properties).unwrap();

                        if paradigm_reader.read_line(&mut paradigm_line).unwrap() == 0 { return; }
                        paradigm_line.clear();

                        let verb_section = format!("Verb {}. {}", idx, verb.pps[0]);
                        println!("\n{}", verb_section);
                        if paradigm_reader.read_line(&mut paradigm_line).unwrap() != 0 { 
                            assert_eq!(paradigm_line[0..paradigm_line.len() - 1], verb_section);
                        }
                        paradigm_line.clear();

                            for x in [HcTense::Present, HcTense::Imperfect, HcTense::Future, HcTense::Aorist, HcTense::Perfect, HcTense::Pluperfect] {   
                                for v in [HcVoice::Active,HcVoice::Middle,HcVoice::Passive] { 
                                for m in [HcMood::Indicative, HcMood::Subjunctive,HcMood::Optative,HcMood::Imperative] {
                                    
                                    if ((m == HcMood::Subjunctive || m == HcMood::Optative || m == HcMood::Imperative) && (x == HcTense::Imperfect || x == HcTense::Perfect || x == HcTense::Pluperfect)) || x == HcTense::Future && (m == HcMood::Subjunctive || m == HcMood::Imperative) {
                                        continue;
                                    }

                                    if paradigm_reader.read_line(&mut paradigm_line).unwrap() == 0 { return; }
                                    paradigm_line.clear();

                                    let section = format!("{} {} {}", x.value(), get_voice_label(x, v, m, verb.deponent_type()), m.value());
                                    println!("\n{}", section);
                                    if paradigm_reader.read_line(&mut paradigm_line).unwrap() != 0 { 
                                        //assert_eq!(paradigm_line[0..paradigm_line.len() - 1], section);
                                    }
                                    paradigm_line.clear();

                                    for z in [HcNumber::Singular, HcNumber::Plural] {
                                        for y in [HcPerson::First, HcPerson::Second, HcPerson::Third] {

                                            let form = HcGreekVerbForm {verb:&verb, person:y, number:z, tense:x, voice:v, mood:m, gender:None, case:None};
                                            let r = if form.get_form(false).unwrap().last().unwrap().form == "" { "NF".to_string() } else { form.get_form(false).unwrap().last().unwrap().form.to_string() };
                                            let r_d = if form.get_form(true).unwrap().last().unwrap().form == "" { "NDF".to_string() } else { form.get_form(true).unwrap().last().unwrap().form.to_string() };

                                            let mut form_line = format!("{}{}: {} ; {}", y.value(), z.value(), 
                                                str::replace(&r, " /", ","),
                                                str::replace(&r_d, " /", ","));

                                            println!("{}", form_line);

                                            if paradigm_reader.read_line(&mut paradigm_line).unwrap() != 0 { 
                                                assert_eq!(paradigm_line[0..paradigm_line.len() - 1]/* .nfc().collect::<String>()*/, form_line);
                                            }
                                            paradigm_line.clear();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
