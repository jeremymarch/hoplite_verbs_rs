#![allow(dead_code)]

extern crate rustunicodetests;
use rustunicodetests::*;
//use rustunicodetests::hgk_toggle_diacritic_str;
use rustunicodetests::hgk_strip_diacritics;
//use rustunicodetests::hgk_strip_diacritics_and_replace_circumflex_with_macron;
use rustunicodetests::hgk_has_diacritics;
//use rustunicodetests::hgk_transliterate;
//use rustunicodetests::hgk_convert;

use itertools::Itertools;

trait RReplacen {
    fn rreplacen(&self, pat: &str, to: &str, count: usize) -> Self;
}
impl RReplacen for String {

    /// Replaces last N matches of a pattern with another string.
    ///
    /// `rreplacen` creates a new [`String`], and copies the data from this string slice into it.
    /// While doing so, it attempts to find matches of a pattern. If it finds any, it
    /// replaces them with the replacement string slice at most `count` times.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let s = "foo foo 123 foo";
    /// assert_eq!("new new 123 foo", s.rreplacen("foo", "new", 2));
    /// assert_eq!("faa fao 123 foo", s.rreplacen('o', "a", 3));
    /// assert_eq!("foo foo new23 foo", s.rreplacen(char::is_numeric, "new", 1));
    /// ```
    ///
    /// When the pattern doesn't match:
    ///
    /// ```
    /// let s = "this is old";
    /// assert_eq!(s, s.rreplacen("cookie monster", "little lamb", 10));
    /// ```
    fn rreplacen(&self, pat: &str, to: &str, count: usize) -> String {
        //fn rreplacen<'a, P: Pattern<'a>>(&'a self, pat: P, to: &str, count: usize) -> String {
        // Hope to reduce the times of re-allocation
        let mut result = String::with_capacity(32);
        let mut last_end = 0;
        
        let matches:Vec<(usize, &str)> = self.rmatch_indices(pat).take(count).collect();
        for (start,part) in matches.into_iter().rev() {
            //println!("start {}, part {}", start, part);
            result.push_str(unsafe { self.get_unchecked(last_end..start) });
            result.push_str(to);
            last_end = start + part.len();
        }
        result.push_str(unsafe { self.get_unchecked(last_end..self.len()) });
        result
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum HcFormError {
    InternalError,
    BlankPrincipalPartForForm,
    UnexpectedPrincipalPartEnding,
    Deponent,
    IllegalForm,
    DoesNotExist,
    NotAvailableInUnit,
    NotImplemented,
}

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
    PresentActiveOptEContracted,
    PresentActiveIndicativeMi,
    AoristActiveImperativesMi,
    AoristActiveImperativesMiRoot,
    AoristMiddleImperativesMi,
    AoristActiveIndicativeMiRoot,
    SecondAoristMiddleImperative,
    PresentMidpassOptTithhmi,
    //ImperfectActiveContracteddecompose,
    PresentMidpassImperativeMi,
    ImperfectActiveMi,
    MixedAoristMi,
    MiddleOptMi,
    PresentActiveOptMi,
    AoristOptativeEchw,
    NotImplemented,
    
    //NumEndings,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum HcPerson {
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
pub enum HcNumber {
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
pub enum HcTense {
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
pub enum HcVoice {
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
pub enum HcMood {
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

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum HcGender {
    Masculine,
    Feminine,
    Neuter,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum HcCase {
    Nominative,
    Genitive,
    Dative,
    Accusative,
    Vocative,
}

#[derive(PartialEq, Debug)]
pub enum HcGreekPrincipalParts {
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

pub const REGULAR                           :u32 = 0x0000;
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
    pub id: u32,
    pub pps: Vec<String>,
    pub properties: u32,
}

impl HcGreekVerb {
    pub fn from_string(id:u32, pps:&str, props:u32) -> Option<HcGreekVerb> {
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

    pub fn from_string_with_properties(id:u32, ppstring:&str) -> Option<HcGreekVerb> {
        let mut properties = 0;
        let mut ll = ppstring.split('%');
        let pps = ll.next().unwrap();
        if let Some(s) = ll.next() {
            if s.contains("CONSONANT_STEM_PERFECT_PI") {
                properties |= CONSONANT_STEM_PERFECT_PI;
            }
            else if s.contains("CONSONANT_STEM_PERFECT_GAMMA") {
                properties |= CONSONANT_STEM_PERFECT_GAMMA;
            }
            else if s.contains("CONSONANT_STEM_PERFECT_CHI") {
                properties |= CONSONANT_STEM_PERFECT_CHI;
            }
            else if s.contains("CONSONANT_STEM_PERFECT_BETA") {
                properties |= CONSONANT_STEM_PERFECT_BETA;
            }
            else if s.contains("CONSONANT_STEM_PERFECT_LAMBDA") {
                properties |= CONSONANT_STEM_PERFECT_LAMBDA;
            }
            else if s.contains("CONSONANT_STEM_PERFECT_NU") {
                properties |= CONSONANT_STEM_PERFECT_NU;
            }
            if s.contains("PREFIXED") {
                properties |= PREFIXED;
            }
        }

        HcGreekVerb::from_string(id, pps, properties)
    }

    //page 316 in h&q
    fn deponent_type(&self) -> HcDeponentType {
        if self.pps[0].ends_with("????????????????") { //and ????????????????????????
            //From Hardy: "I guess ????????????????? is technically a partial deponent, though in practice I don't think we're in the habit of calling it that.?? We simply say that's a deponent (i.e. a middle deponent) with one active PP."
            HcDeponentType::GignomaiDeponent //see H&Q page 382. fix me, there may be a better way to do this without separate case
        }
        /*else if ( utf8HasSuffix(v->present, "??????")) {
            return MIDDLE_DEPONENT;
        }*/
        else if self.pps[0].ends_with("??????") && self.pps[1].ends_with("??????") && self.pps[2].ends_with("??????") && self.pps[3] == "???" /* && utf8HasSuffix(v->perfmid, "??????") */ && self.pps[5] == "???" {
            HcDeponentType::MiddleDeponent
        }
        //this gets ???????????????????????? and ???????????????????????: middle deponents which happen to have an active perfect and root aorist
        else if self.pps[0].ends_with("??????") && self.pps[1].ends_with("??????") && self.pps[2].ends_with("????") /* && utf8HasSuffix(v->perfmid, "??????") */ && self.pps[5] == "???" {
            HcDeponentType::MiddleDeponent
        }
        else if self.pps[0].ends_with("??????") && self.pps[1].ends_with("??????") && self.pps[2] == "???" && self.pps[3] == "???" && self.pps[4].ends_with("??????") && self.pps[5] != "???" {
            HcDeponentType::PassiveDeponent
        }
        else if self.pps[0].ends_with("???????????????????") {
            HcDeponentType::PassiveDeponent //close enough
        }
        else if self.pps[0].ends_with("???????????????") { //doesn't seem to have future passive, though?
            HcDeponentType::MiddleDeponentHgeomai //we call it a middle deponent which happens to also have a 6th pp
        }
        else if self.pps[0].ends_with("??????") || self.pps[1].ends_with("??????") || self.pps[2].ends_with("??????") {
            HcDeponentType::PartialDeponent
        }
        else {
            HcDeponentType::NotDeponent
        }
    }
}

#[derive(Default)]
pub struct Step {
    pub form: String,
    pub explanation: String,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct HcGreekVerbForm<'a> {
    pub verb: &'a HcGreekVerb,
    pub person: HcPerson,
    pub number: HcNumber,
    pub tense: HcTense,
    pub voice: HcVoice,
    pub mood: HcMood,
    pub gender: Option<HcGender>,
    pub case: Option<HcCase>,
}

static SEPARATOR: &str = "???";
static BLANK: &str = "???";

pub trait HcVerbForms {
    fn get_form(&self, decompose:bool) -> Result<Vec<Step>, HcFormError>;
    fn get_pp_num(&self) -> HcGreekPrincipalParts;
    fn get_pp(&self) -> Option<String>;
    fn strip_ending(&self, pp_num:usize, form:String) -> Result<String, &str>;
    fn add_ending(&self, stem:&str, ending:&str, decompose:bool) -> Result<String, &str>;
    fn get_endings(&self, stem: &str) -> Option<Vec<&str>>;
    fn accent_verb(&self, form:&str) -> String;
    fn accent_verb_contracted(&self, word:&str, orig_syllables:Vec<SyllableAnalysis>, ending:&str) -> String;
    fn accent_syllable(&self, word:&str, syllable:u8, accent:u32) -> String;
    fn accent_syllable_start(&self, word:&str, letter_index_from_end:u8, accent:u32) -> String;
    fn get_label(&self) -> String;
    fn add_augment(&self, stem:&str, decompose:bool) -> String;
    fn deaugment(&self, stem:&str, decompose:bool) -> String;
    fn contract_verb(&self, unaccented_form:&str, ending:&str) -> String;
    fn is_deponent(&self, stem:&str) -> bool;
    fn separate_prefix(&self, stem:&str) ->String;
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

fn get_voice_label(tense:HcTense, voice:HcVoice, mood:HcMood, _deponent_type:HcDeponentType) -> String {
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

    fn contract_verb(&self, unaccented_form:&str, ending:&str) -> String {
        let mut form = hgk_strip_diacritics(unaccented_form, HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE);
        let orig_syl = analyze_syllable_quantities(&form, self.person, self.number, self.tense, self.mood, self.verb.properties);

        if form.contains("??????") {               // h&q p236
            form = form.replacen("??????", "????", 1);
        }
        else if form.contains("????") {
            form = form.replacen("????", "????", 1);
        }
        else if form.contains("????") {
            form = form.replacen("????", "??", 1);
        }
        else if form.contains("?????") {
            form = form.replacen("?????", "???", 1);
        }
        else if form.contains("??????") {
            form = form.replacen("??????", "????", 1);
        }
        else if form.contains("??????") {
            form = form.replacen("??????", "????", 1);
        }
        else if form.contains("????") {
            form = form.replacen("????", "????", 1);
        }
        else if form.contains("????") {
            form = form.replacen("????", "??", 1);
        }

        else if form.contains("??????") {          // h&q p232
            form = form.replacen("??????", "?????", 1);
        }
        else if form.contains("????") {
            form = form.replacen("????", "???", 1);
        }
        else if form.contains("????") {
            form = form.replacen("????", "???", 1);
        }
        else if form.contains("?????") {
            form = form.replacen("?????", "?????", 1);
        }
        else if form.contains("??????") {
            form = form.replacen("??????", "???", 1);
        }
        else if form.contains("??????") {
            form = form.replacen("??????", "??", 1);
        }
        else if form.contains("????") {
            form = form.replacen("????", "??", 1);
        }
        else if form.contains("????") {
            form = form.replacen("????", "??", 1);
        }

        else if form.contains("??????") {          // h&q p264
            form = form.replacen("??????", "????", 1);
        }
        else if form.contains("????") {
            form = form.replacen("????", "????", 1);
        }
        else if form.contains("????") {
            form = form.replacen("????", "??", 1);
        }
        else if form.contains("?????") {
            form = form.replacen("?????", "????", 1);
        }
        else if form.contains("??????") {
            form = form.replacen("??????", "????", 1);
        }
        else if form.contains("??????") {
            form = form.replacen("??????", "????", 1);
        }
        else if form.contains("????") {
            form = form.replacen("????", "????", 1);
        }
        else if form.contains("????") {
            form = form.replacen("????", "??", 1);
        }

        self.accent_verb_contracted(&form, orig_syl, ending)

        //unaccented_form.to_string()
    }

    fn strip_ending(&self, pp_num:usize, form:String) -> Result<String, &str> {
        //println!("form: {}", form);
        match pp_num {
            1..=2 => {
                if form.ends_with('??') {
                    if self.tense == HcTense::Future && self.voice != HcVoice::Passive && (self.verb.pps[1].ends_with('???') || (form.starts_with("?????") && self.verb.pps[1].starts_with("????????"))) {
                        // contracted future
                        if self.verb.pps[1].ends_with("????????") {
                            return Ok(format!("{}??", form.strip_suffix('??').unwrap()));
                        }
                        else {
                            return Ok(format!("{}??", form.strip_suffix('??').unwrap()));
                        }
                    }
                    else {
                        return Ok(form.strip_suffix('??').unwrap().to_string());
                    }
                }
                else if form.ends_with("??????????") && self.verb.pps[1].ends_with("???????????") {
                    // contracted future
                    return Ok(format!("{}??", form.strip_suffix("??????????").unwrap()));
                }
                else if form.ends_with("????????") {
                    return Ok(form.strip_suffix("????????").unwrap().to_string());
                }
                else if form.ends_with("??????") {
                    return Ok(form.strip_suffix("??????").unwrap().to_string());
                }
                else if form.ends_with("????") {
                    return Ok(form.strip_suffix("????").unwrap().to_string());
                }
                else if form.ends_with("??????(??)") {
                    return Ok(form.strip_suffix("????(??)").unwrap().to_string());
                }
                else if form.ends_with("??????") {
                    return Ok(form.strip_suffix("??????").unwrap().to_string());
                }
                else if form.ends_with("?????????") || form.ends_with("????????") {
                    return Ok("???????".to_string());
                }
                else if form.ends_with("??????") {
                    return Ok("????".to_string());
                }
                else if form.ends_with("????????????") {
                    return Ok("????????".to_string());
                }
                else if form.ends_with("??????") {
                    return Ok("????".to_string());
                }
            },
            3 => {
                if form.ends_with("????????") {
                    return Ok(form.strip_suffix("????????").unwrap().to_string());
                }
                else if form.ends_with('??') {
                    return Ok(form.strip_suffix('??').unwrap().to_string());
                }
                else if form.ends_with("????") {
                    return Ok(form.strip_suffix("????").unwrap().to_string());
                }
                else if form.ends_with("????????") {
                    return Ok(form.strip_suffix("????????").unwrap().to_string());
                }    
                else if form.ends_with("??") {
                    return Ok(form.strip_suffix("??").unwrap().to_string());
                }  
                else if form.ends_with("??(??)") {
                    return Ok(form.strip_suffix("??(??)").unwrap().to_string());
                }              
            },
            4 => {
                if form.ends_with('??') {
                    return Ok(form.strip_suffix('??').unwrap().to_string());
                }               
            },
            5 => {
                if form.ends_with("??????") {
                    return Ok(form.strip_suffix("??????").unwrap().to_string());
                }               
            },
            6 => {
                if form.ends_with("????") {
                    return Ok(form.strip_suffix("????").unwrap().to_string());
                }               
            },
            _ => { return Err("error stripping ending 1"); }
        }
        Err("error stripping ending 2")
    }

    fn is_deponent(&self, stem:&str) -> bool {   
        if (self.tense == HcTense::Present || self.tense == HcTense::Imperfect || self.tense == HcTense::Future) && stem.ends_with("??????") {
            true
        }
        else if self.tense == HcTense::Aorist && self.voice != HcVoice::Passive && stem.ends_with("????????") {
            true
        }
        else {
            false
        }
    }

    fn add_ending(&self, stem:&str, ending:&str, decompose:bool) -> Result<String, &str> {
        let mut local_stem = stem.to_string();
        let mut local_ending = ending.to_string();

        //for contracted verbs remove nu movable for imperfect 3rd sing. active
        if self.tense == HcTense::Imperfect && ( self.verb.pps[0].ends_with("????") || self.verb.pps[0].ends_with("????") || self.verb.pps[0].ends_with("????") ) && self.person == HcPerson::Third && self.number == HcNumber::Singular && self.voice == HcVoice::Active {
            local_ending = local_ending.replacen("(??)", "", 1);
        }

        //add macron to ????????????????????? perfect and pluperfect
        if self.verb.pps[0].ends_with("?????????????????????") && (self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect) && self.mood == HcMood::Indicative && self.voice != HcVoice::Active {
            local_stem = local_stem.replacen("??", "???", 1);
        }

        if self.verb.pps[0].ends_with("????") || self.verb.pps[0].ends_with("????????") {
            if self.tense == HcTense::Present || self.tense == HcTense::Imperfect {
                if self.number == HcNumber::Plural || self.mood != HcMood::Indicative || self.voice != HcVoice::Active {
                    if self.verb.pps[0].ends_with("??????") {
                        local_stem.pop();
                        local_stem.push_str("??");
                    }
                    else if self.verb.pps[0].ends_with("??????????") {
                        local_stem.pop();
                        local_stem.push_str("??");
                    }
                    else if self.verb.pps[0].ends_with("????????????") || self.verb.pps[0].ends_with("?????????????") || self.verb.pps[0].ends_with("???????????") { 
                        local_stem.pop();
                        local_stem.push_str("??");

                        if (self.verb.pps[0].ends_with("?????????????") || self.verb.pps[0].ends_with("???????????")) && self.tense == HcTense::Present && self.person == HcPerson::Third && self.number == HcNumber::Plural && self.voice == HcVoice::Active && self.mood == HcMood::Indicative {
                            if !decompose {
                                local_stem.pop();
                            }
                            local_ending = if decompose { String::from("???????(??)") } else { String::from("???????(??)") };
                        }    
                    }
                    else if self.verb.pps[0].ends_with("???????") {
                        local_stem = local_stem.replacen("???", "??", 1);
                    }
                }
            }

            if self.tense == HcTense::Present {
                if self.voice == HcVoice::Active {
                    if self.mood == HcMood::Subjunctive {
                        if !decompose {
                            if self.verb.pps[0].ends_with("??????") {
                                // didwmi / gignwskw subjunctive contraction
                                if local_ending.contains("???") {
                                    local_ending = local_ending.replacen("???", "???", 1);
                                }
                                else if local_ending.contains("???") {
                                    local_ending = local_ending.replacen("???", "???", 1);
                                }
                            }

                            if !self.verb.pps[0].ends_with("???????") {
                                local_stem.pop();
                            }
                        }
                        else {
                            //isthmi subjunctive stem
                            if self.verb.pps[0].ends_with("??????????") {
                                local_stem.pop();
                                local_stem.push_str("??");
                            }
                        }
                    }
                    else if self.mood == HcMood::Imperative {
                        if decompose {
                            if !(self.person == HcPerson::Second && self.number == HcNumber::Singular) {
                                local_ending.remove(0);
                            }
                            else if self.verb.pps[0].ends_with("???????") { 
                                local_stem = local_stem.replacen("??", "???", 1); //fix me
                                local_ending = String::from(""); // fix me
                            }
                        }
                        else {
                            if self.person == HcPerson::Second && self.number == HcNumber::Singular {
                                if self.verb.pps[0].ends_with("??????") {
                                    local_ending = String::from("??");
                                }
                                else if self.verb.pps[0].ends_with("??????????") { 
                                    local_stem.pop();
                                    local_ending = String::from("??");
                                }
                                else if self.verb.pps[0].ends_with("???????") { 
                                    local_stem = local_stem.replacen("??", "???", 1);
                                    local_ending = String::from("");
                                }
                                else {
                                    local_ending = String::from("??");
                                }
                            }
                            else {
                                local_ending.remove(0);
                            }
                        }
                    }
                    else if self.verb.pps[0].ends_with("??????????") && self.person == HcPerson::Third && self.number == HcNumber::Plural &&self.mood == HcMood::Indicative && !decompose {
                        local_stem.pop();
                        local_ending = local_ending.replacen("????", "???", 1);
                    }
                }
                else { // middle/passive
                    if self.mood == HcMood::Subjunctive {
                        if !decompose {
                            if !self.verb.pps[0].ends_with("???????") { 
                                local_stem.pop();
                            }
                            if self.verb.pps[0].ends_with("??????") {
                                // didwmi / gignwskw subjunctive contraction
                                if local_ending.contains("???") {
                                    local_ending = local_ending.replacen("???", "???", 1);
                                }
                                else if local_ending.contains("??") {
                                    local_ending = local_ending.replacen("??", "???", 1);
                                }
                            }
    
                            if local_ending != "??????????" && !self.verb.pps[0].ends_with("???????") && !self.verb.pps[0].ends_with("??????????????") && !self.verb.pps[0].ends_with("???????????????????") {
                                local_ending = self.accent_syllable_start(&local_ending, 0, HGK_CIRCUMFLEX );
                            }
                        }
                        else {
                            //isthmi subjunctive stem
                            if self.verb.pps[0].ends_with("??????????????") || self.verb.pps[0].ends_with("???????????????????") {
                                local_stem.pop();
                            }
                            else if self.verb.pps[0].ends_with("??????????") || self.verb.pps[0].ends_with("????????") {
                                local_stem.pop();
                                local_stem.push_str("??");
                            }
                        }
                    }
                    else if self.mood == HcMood::Optative {
                        if !decompose {
                            if self.verb.pps[0].ends_with("??????????????") || self.verb.pps[0].ends_with("???????????????????") {
                                local_ending = hgk_strip_diacritics(&local_ending, HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE);
                            }
                            if local_ending.starts_with("??") && !self.verb.pps[0].ends_with("???????") { //alt endings for tithhmi and ihmi
                                local_stem.pop();
                            }
                        }
                    }    
                }
            }
            else if self.tense == HcTense::Imperfect {
                if self.verb.pps[0].ends_with("??????") {
                    if self.number == HcNumber::Singular {
                        if decompose {
                            local_stem = local_stem.replacen("??", "??", 1); //use short stem when using thematic endings
                            if self.person == HcPerson::First && self.voice == HcVoice::Active{
                                local_ending = local_ending.replacen("??", "????", 1);
                            }
                            else {
                                local_ending = local_ending.replacen("??", "????", 1);
                                if self.person == HcPerson::Third && self.voice == HcVoice::Active {
                                    local_ending = String::from("??");
                                }
                            }
                        }
                        else {
                            local_stem = local_stem.replacen("??", "????", 1);
                        }
                    }
                }
                else if self.verb.pps[0].ends_with("????????????") || self.verb.pps[0].ends_with("?????????????") || self.verb.pps[0].ends_with("???????????") {
                    if (self.person == HcPerson::Second || self.person == HcPerson::Third) && self.number == HcNumber::Singular {
                        if decompose {
                            local_stem = local_stem.replacen("??", "??", 1); //use short stem when using thematic endings
                            local_ending = local_ending.replacen("??", "????", 1);
                            if self.person == HcPerson::Third && self.voice == HcVoice::Active {
                                local_ending = String::from("??");
                            }
                        }
                        else {
                            local_stem = local_stem.replacen("??", "????", 1);
                        }
                    }
                }
                if (self.verb.pps[0] == "??????????????" || self.verb.pps[0] == "???????????????????") && self.tense == HcTense::Imperfect && self.person == HcPerson::Second && self.number == HcNumber::Singular {
                    if decompose {
                        local_ending = String::from("??"); //fix me
                    }
                    else {
                        local_stem.pop();
                        local_ending = String::from("??");
                    }
                    
                }
            }
            else if self.tense == HcTense::Aorist {
                //mixed aorist
                if self.verb.pps[2].ends_with("????") && (self.number == HcNumber::Plural || self.mood != HcMood::Indicative || self.voice != HcVoice::Active) {
                        
                    if self.verb.pps[0].ends_with("????????????") {
                        local_stem = local_stem.replacen("????", "??", 1);
                    }
                    else if self.verb.pps[0].ends_with("????????????") || self.verb.pps[0].ends_with("?????????????") || self.verb.pps[0].ends_with("???????????") {
                        if self.verb.pps[0].ends_with("???????????") && !decompose && (self.number == HcNumber::Plural || self.voice != HcVoice::Active) {
                            local_stem = local_stem.replacen("????", "????", 1);
                        }
                        else {
                            if self.verb.pps[0].ends_with("?????????????") && !decompose {
                                local_stem = local_stem.replacen("?????", "?????", 1);
                            }
                            else {
                                local_stem = local_stem.replacen("????", "??", 1);
                            }
                        }
                    }

                    if self.mood == HcMood::Subjunctive && !decompose && self.voice != HcVoice::Passive {
                        local_stem.pop();
                    }

                    if self.voice == HcVoice::Active {
                        if self.mood != HcMood::Indicative {
                            if !decompose {
                                if self.mood == HcMood::Subjunctive {
                                    if self.verb.pps[0].ends_with("??????") {
                                        // didwmi / gignwskw subjunctive contraction
                                        if local_ending.contains("???") {
                                            local_ending = local_ending.replacen("???", "???", 1);
                                        }
                                        else if local_ending.contains("??") {
                                            local_ending = local_ending.replacen("??", "???", 1);
                                        }
                                    }
                                    else if self.verb.pps[0].ends_with("?????????????") {
                                        let (stem, ending) = match (self.person, self.number) {
                                            (HcPerson::First, HcNumber::Singular) => ("-", "???"),
                                            (HcPerson::Second, HcNumber::Singular) => ("-", "?????"),
                                            (HcPerson::Third, HcNumber::Singular) => ("-", "???"),
                                            (HcPerson::First, HcNumber::Plural) => ("-", "?????????"),
                                            (HcPerson::Second, HcNumber::Plural) => ("-", "???????"),
                                            (HcPerson::Third, HcNumber::Plural) => ("-", "???????(??)"),
                                            _ => ("", "")
                                        };
                                        local_stem = stem.to_string();
                                        local_ending = ending.to_string();
                                    }
                                    local_ending = self.accent_syllable_start(&local_ending, 0,  HGK_CIRCUMFLEX );
                                }
                                else if self.mood == HcMood::Imperative {
                                    // ana/thes
                                    if self.verb.pps[0].ends_with("???????????????????") && self.person == HcPerson::Second && self.number == HcNumber::Singular {
                                        local_stem = self.accent_syllable(&local_stem, 2, HGK_ACUTE );
                                    }// apo/dos
                                    else if self.verb.pps[0].ends_with("???????????????????") && self.person == HcPerson::Second && self.number == HcNumber::Singular {
                                        local_stem = self.accent_syllable(&local_stem, 2, HGK_ACUTE );
                                    }
                                    else if self.verb.pps[0].ends_with("????????????????????") && self.person == HcPerson::Second && self.number == HcNumber::Singular {
                                        local_stem = self.accent_syllable(&local_stem, 2, HGK_ACUTE );
                                    }
                                    else if self.verb.pps[0].ends_with("????????????????????") && self.person == HcPerson::Second && self.number == HcNumber::Singular {
                                        local_stem = self.accent_syllable(&local_stem, 2, HGK_ACUTE );
                                    }
                                }
                            }
                            if self.mood == HcMood::Optative {
                                local_ending.remove(0);
                                if self.verb.pps[0].ends_with("?????????????") && !decompose {
                                    local_ending.remove(0);
                                    local_stem = "-?????".to_string();
                                }
                            }
                        }
                    }
                    else if self.voice == HcVoice::Middle {
                        if self.mood == HcMood::Indicative {
                            if ( self.verb.pps[0].ends_with("?????????????") || self.verb.pps[0].ends_with("???????????")) && self.person == HcPerson::Second && self.number == HcNumber::Singular {
                                local_ending = String::from("????");
                            }
                            else {
                                local_ending.remove(0);
                                if self.person == HcPerson::Second && self.number == HcNumber::Singular {
                                    if decompose {
                                        local_ending = String::from("??");
                                    }
                                    else if local_stem.ends_with("??") {
                                        local_stem = local_stem.rreplacen("??", "??", 1);
                                    }
                                }
                            }
                        }
                        else if self.mood == HcMood::Subjunctive {
                            if self.verb.pps[0].ends_with("??????") && !decompose {
                                // didwmi / gignwskw subjunctive contraction
                                if local_ending.contains("???") {
                                    local_ending = local_ending.replacen("???", "???", 1);
                                }
                                else if local_ending.contains("??") {
                                    local_ending = local_ending.replacen("??", "???", 1);
                                }
                            }
                            else if self.verb.pps[0].ends_with("?????????????") && !decompose {
                                let (stem, ending) = match (self.person, self.number) {
                                    (HcPerson::First, HcNumber::Singular) => ("-", "?????????"),
                                    (HcPerson::Second, HcNumber::Singular) => ("-", "???"),
                                    (HcPerson::Third, HcNumber::Singular) => ("-", "?????????"),
                                    (HcPerson::First, HcNumber::Plural) => ("-", "???????????"),
                                    (HcPerson::Second, HcNumber::Plural) => ("-", "?????????"),
                                    (HcPerson::Third, HcNumber::Plural) => ("-", "???????????"),
                                    _ => ("", "")
                                };
                            
                                local_stem = stem.to_string();
                                local_ending = ending.to_string();
                            }
                            if !decompose && local_ending != "??????????" && local_ending != "???????????" {
                                local_ending = self.accent_syllable_start(&local_ending, 0, HGK_CIRCUMFLEX );
                            }
                        }
                        else if self.mood == HcMood::Optative {
                            if !decompose {
                                if self.verb.pps[0].ends_with("?????????????") {
                                    if local_ending.starts_with("??") {
                                        local_ending.remove(0);
                                        local_ending.remove(0);
                                        local_stem = "-?????".to_string();
                                    }
                                    else {
                                        local_ending.remove(0);
                                        local_stem = "-?????".to_string();
                                    }
                                }
                                else if local_ending.starts_with("??") {
                                    local_stem.pop();
                                }
                            }
                        }
                        else if self.mood == HcMood::Imperative {
                            if self.person == HcPerson::Second && self.number == HcNumber::Singular {
                                if decompose {
                                    if !self.verb.pps[0].ends_with("???????????") && !self.verb.pps[0].ends_with("?????????????") {
                                        local_ending.remove(0);
                                    }
                                    else {
                                        local_ending = local_ending.replacen("????", "????", 1);
                                    }
                                }
                                else {
                                    local_stem.pop();
                                    
                                    if local_stem.starts_with("??????") || self.verb.pps[0].ends_with("???????????") {
                                        local_ending = local_ending.replacen("????", "?????", 1);
                                    }
                                    else if self.verb.pps[0].ends_with("?????????????") {
                                        local_ending = local_ending.replacen("????", "?????", 1);
                                    }
                                    else {
                                        local_ending = local_ending.replacen("????", "????", 1);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            else if self.tense == HcTense::Perfect {
                if self.number == HcNumber::Plural && local_stem.ends_with("????????") {
                    local_stem = local_stem.replacen("????", "??", 1);
                    if self.person == HcPerson::Third {
                        if decompose {

                        }
                        else {
                            local_stem.pop();
                            local_ending = local_ending.replacen("????", "???", 1);
                        }
                    }
                    else {
                        local_ending.remove(0);
                    }
                }
            }
            else if self.tense == HcTense::Pluperfect {
                if self.number == HcNumber::Plural && local_stem.ends_with("????????") {
                    local_stem = local_stem.replacen("????", "??", 1);
                    local_ending.remove(0);
                }
            }
        }

        // root aorist
        if (self.tense == HcTense::Aorist && self.voice == HcVoice::Active) && local_stem.ends_with("??????") || local_stem.ends_with("??????") || local_stem.ends_with("????") || local_stem.ends_with("??????") {
            if self.mood == HcMood::Subjunctive {
                if decompose {
                    if local_stem.ends_with("??????") {
                        local_stem.pop();
                        local_stem.push_str("??");
                    }
                    else {
                        local_stem.pop();
                        local_stem.push_str("??");
                    }
                }
                else { 
                    if local_stem.ends_with("??????") {
                        // didwmi / gignwskw subjunctive contraction
                        if local_ending.contains("???") {
                            local_ending = local_ending.replacen("???", "???", 1);
                        }
                        else if local_ending.contains("???") {
                            local_ending = local_ending.replacen("???", "???", 1);
                        }
                    }
                    local_stem.pop();
                }
            }
            else if self.mood == HcMood::Optative {
                if local_stem.ends_with("??????") {
                    local_stem.pop();
                    local_stem.push_str("??");
                }
                else {
                    local_stem.pop();
                    local_stem.push_str("??");
                }
            }
            else if self.mood == HcMood::Imperative {
                if self.person == HcPerson::Second && self.number == HcNumber::Singular && local_stem.ends_with("??????") {
                    local_ending = local_ending.replacen("??", "??", 1);
                }
                else if self.person == HcPerson::Third && self.number == HcNumber::Plural {
                    if local_stem.ends_with("??????") {
                        local_stem.pop();
                        local_stem.push_str("??");
                    }
                    else {
                        local_stem.pop();
                        local_stem.push_str("??");
                    }
                }
            }
        }

        // consonant stem perfects and pluperfects
        if ((self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect) && (self.voice == HcVoice::Middle || self.voice == HcVoice::Passive)) && local_stem == "??????????" || local_stem == "?????????????" || local_stem == format!("?? {} ??????????", SEPARATOR) {
            if local_ending.starts_with("????") {
                return Ok(String::from(BLANK));
            }
            else if decompose {
                local_stem = format!("{}??", local_stem);
            }
            else if local_ending.starts_with("????") {
                local_ending.remove(0);
                local_ending = format!("??{}", local_ending);
            }
            else if local_ending.starts_with('??') {
                local_ending.remove(0);
                local_ending = format!("??{}", local_ending);
            }
            else if local_ending.starts_with('??') {
                local_ending = format!("??{}", local_ending);
            }
        }
        else if ((self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect) && 
            (self.voice == HcVoice::Middle || self.voice == HcVoice::Passive)) && (local_stem.ends_with('??') || self.verb.pps[4].ends_with("????????????")){

            if self.verb.properties & CONSONANT_STEM_PERFECT_NU == CONSONANT_STEM_PERFECT_NU && self.person == HcPerson::Second && self.number == HcNumber::Singular {
                return Ok(String::from(BLANK));
            }

            if local_ending.starts_with("????") {
                return Ok(String::from(BLANK));
            }
            else if decompose {
                local_stem.pop();
                if self.verb.properties & CONSONANT_STEM_PERFECT_PI == CONSONANT_STEM_PERFECT_PI {
                    local_stem = format!("{}??", local_stem);    
                }
                else if self.verb.properties & CONSONANT_STEM_PERFECT_BETA == CONSONANT_STEM_PERFECT_BETA {
                    local_stem = format!("{}??", local_stem);    
                }
                else if self.verb.properties & CONSONANT_STEM_PERFECT_NU == CONSONANT_STEM_PERFECT_NU {
                    local_stem = format!("{}??", local_stem);    
                }
                else {
                    local_stem = format!("{}??", local_stem);
                }
            }
            else if local_ending.starts_with("????") {
                local_ending.remove(0);
                local_stem.pop();
                if self.verb.properties & CONSONANT_STEM_PERFECT_NU == CONSONANT_STEM_PERFECT_NU {
                    local_ending = format!("??{}", local_ending);
                }
                else {
                    local_ending = format!("??{}", local_ending);
                }
            }
            else if local_ending.starts_with('??') {
                local_stem.pop();
                local_ending.remove(0);
                local_ending = format!("??{}", local_ending);
            }
            else if local_ending.starts_with('??') {
                local_stem.pop();
                if self.verb.properties & CONSONANT_STEM_PERFECT_NU == CONSONANT_STEM_PERFECT_NU {
                    local_ending = format!("??{}", local_ending);
                }
                else {
                    local_ending = format!("??{}", local_ending);
                }
            }
        }
        else if ((self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect) && 
            (self.voice == HcVoice::Middle || self.voice == HcVoice::Passive)) && local_stem.ends_with('??') {

            if local_ending.starts_with("????") {
                return Ok(String::from(BLANK));
            }
            else if decompose {
                local_stem.pop();
                if self.verb.properties & CONSONANT_STEM_PERFECT_GAMMA == CONSONANT_STEM_PERFECT_GAMMA {
                    local_stem = format!("{}??", local_stem);    
                }
                else if self.verb.properties & CONSONANT_STEM_PERFECT_CHI == CONSONANT_STEM_PERFECT_CHI {
                    local_stem = format!("{}??", local_stem);    
                }
                else {
                    local_stem = format!("{}??", local_stem);
                }
            }
            else if local_ending.starts_with("????") {
                local_ending.remove(0);
                local_stem.pop();
                local_ending = format!("??{}", local_ending);
            }
            else if local_ending.starts_with('??') {
                local_stem.pop();
                local_ending.remove(0);
                local_ending = format!("??{}", local_ending);
            }
            else if local_ending.starts_with('??') {
                local_stem.pop();
                local_ending = format!("??{}", local_ending);
            }
        }
        else if ((self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect) && 
            (self.voice == HcVoice::Middle || self.voice == HcVoice::Passive)) && local_stem.ends_with('??') {
            
            if local_ending.starts_with("????") {
                return Ok(String::from(BLANK));
            }
            else if local_ending.starts_with('??') && !decompose {
                local_ending.remove(0);
            }
        }
        else if ((self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect) && 
            (self.voice == HcVoice::Middle || self.voice == HcVoice::Passive)) && local_stem.ends_with('??') {
            
            if local_ending.starts_with("????") {
                return Ok(String::from(BLANK));
            }
            else if local_ending.starts_with('??') && !decompose && self.number == HcNumber::Plural {
                local_ending.remove(0);
            }
        }

        let future_passive_suffix = if self.tense == HcTense::Future && self.voice == HcVoice::Passive {
            if decompose {
                format!("???? {} ", SEPARATOR)
            }
            else {
                String::from("????")
            }
        }
        else {
            String::from("")
        };

        if self.verb.pps[0].ends_with("???????") && self.person == HcPerson::Second && self.number == HcNumber::Singular && self.tense == HcTense::Aorist && self.mood == HcMood::Imperative && self.voice == HcVoice::Active { 
            local_ending = String::from("????");
        }

        if decompose {
            Ok(format!("{} {} {}{}", local_stem, SEPARATOR, future_passive_suffix, local_ending))
        }
        else { //come take see say find: elthe/ labe/ eide/ eipe/ eyre/
            if local_stem == "???????" && local_ending == "??" {
                local_ending = "??".to_string();
            }
            else if local_stem == "??????" && local_ending == "??" {
                local_ending = "??".to_string();
            }
            else if local_stem == "?????" && local_ending == "??" {
                local_ending = "??".to_string();
            }
            else if local_stem == "???????" && local_ending == "??" {
                local_ending = "??".to_string();
            }
            else if local_stem == "???????" && local_ending == "??" {
                local_ending = "??".to_string();
            }
            Ok(format!("{}{}{}", local_stem, future_passive_suffix, local_ending))
        }
    }

    fn add_augment(&self, stem:&str, decompose:bool) -> String {
        let mut local_stem = stem.to_string();
        if decompose {
            if local_stem.starts_with('???') || local_stem.starts_with('???') || local_stem.starts_with("?????") || local_stem.starts_with("?????") {
                local_stem
            }
            // else if local_stem.starts_with("?????") {        
            //     local_stem.replacen("?????", format!("?? {} ?????", SEPARATOR).as_str(), 1)
            // }
            else if local_stem.starts_with("???????") && self.verb.pps[0].starts_with("?????????????????????") && self.tense == HcTense::Pluperfect {        
                local_stem.replacen("???????", format!("??????? {} ?????", SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("?????????") {        
                local_stem.replacen("?????????", format!("??????? {} ?? {} ????", SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("???????") {        
                local_stem.replacen("???????", format!("??????? {} ?? {} ", SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("?????????") {        
                local_stem.replacen("?????????", format!("??????? {} ????", SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("-???") {
                local_stem
            }
            else if local_stem.starts_with("-?????") {
                local_stem
            }
            else if local_stem.starts_with("?????") {
                local_stem
            }
            else if local_stem.starts_with("?????") {        
                local_stem.replacen("?????", format!("????? {} ?? {} ", SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("??????") {        
                local_stem.replacen("??????", format!("?????? {} ?? {} ", SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("??????") {        
                local_stem.replacen("??????", format!("?????? {} ?? {} ", SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("?????????") {        
                local_stem.replacen("?????????", format!("?????? {} ?????", SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("??????????") {        
                local_stem.replacen("??????????", format!("?????? {} ?????", SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("????????") {        
                local_stem.replacen("????????", format!("?????? {} ??", SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("??????") {        
                local_stem.replacen("??????", format!("?????? {} ?? {} ", SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("??????????") {        
                local_stem.replacen("??????????", format!("??????????"/* FIX ME */).as_str(), 1)
            }
            else if local_stem.starts_with("???????") {        
                local_stem.replacen("???????", format!("??????? {} ?? {} ", SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("???????") {        
                local_stem.replacen("???????", format!("??????? {} ?? {} ??" /* FIX ME ??? */, SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("???????") {     
                local_stem.replacen("???????", format!("??????? {} ?? {} ", SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("????????") {     
                local_stem.replacen("????????", format!("???????? {} ?? {} ", SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("???????????") {     
                local_stem.replacen("???????????", format!("?? {} ???????????", SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("???????") {     
                local_stem.replacen("???????", format!("??????? {} ?? {} ", SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("????????") {     
                local_stem.replacen("????????", format!("???????? {} ?? {} ", SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("????????????") {     
                local_stem.replacen("????????????", format!("???????? {} ?????? {} ?? {} ???", SEPARATOR, SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("???????????") {     
                local_stem.replacen("???????????", format!("??????? {} ?????? {} ?? {} ???", SEPARATOR, SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("???????????") {    
                if self.number == HcNumber::Singular { 
                    local_stem.replacen("???????????", format!("??????? {} ?????? {} ?? {} ???", SEPARATOR, SEPARATOR, SEPARATOR).as_str(), 1)
                } 
                else {
                    local_stem.replacen("???????????", format!("??????? {} ?????? {} ???", SEPARATOR, SEPARATOR).as_str(), 1)
                }
            }
            else if local_stem.starts_with("????????????") {    
                if self.number == HcNumber::Singular { 
                    local_stem.replacen("????????????", format!("???????? {} ?????? {} ?? {} ???", SEPARATOR, SEPARATOR, SEPARATOR).as_str(), 1)
                } 
                else {
                    local_stem.replacen("????????????", format!("???????? {} ?????? {} ???", SEPARATOR, SEPARATOR).as_str(), 1)
                }
            }
            else if local_stem.starts_with("????????") {     
                local_stem.replacen("????????", format!("???????? {} ?? {} ", SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("????????") {     
                local_stem.replacen("????????", format!("??????? {} ?????", SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("???????") {     
                local_stem.replacen("???????", format!("??????? {} ?? {} ???", SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("????????") {
                local_stem.replacen("????????", format!("???????? {} ?? {} ???", SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("?????????") {    
                local_stem.replacen("?????????", format!("??????? {} ?????" /* FIX ME breathing position */, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("???????") {    
                if self.number == HcNumber::Singular /*|| self.voice != HcVoice::Active FIX ME */ {
                    local_stem.replacen("???????", format!("??????? {} ?? {} ???", SEPARATOR, SEPARATOR).as_str(), 1)
                }
                else {
                    local_stem.replacen("???????", format!("??????? {} ???", SEPARATOR).as_str(), 1)
                }
            }
            else if local_stem.starts_with("????????") {    
                if self.number == HcNumber::Singular /*|| self.voice != HcVoice::Active FIX ME */ {
                    local_stem.replacen("????????", format!("???????? {} ?? {} ???", SEPARATOR, SEPARATOR).as_str(), 1)
                }
                else {
                    local_stem.replacen("????????", format!("???????? {} ???", SEPARATOR).as_str(), 1)
                }
            }
            else if local_stem.starts_with("?????") { //isthmi
                if self.number == HcNumber::Singular /*|| self.voice != HcVoice::Active FIX ME */ {
                    local_stem.replacen("?????", format!("?? {} ?????", SEPARATOR).as_str(), 1)
                }
                else {
                    local_stem
                }
            }
            else if local_stem.starts_with("???") { //epomai
                local_stem.replacen("???", format!("?? {} ???", SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("???") {    
                if self.tense != HcTense::Pluperfect { 
                    local_stem.replacen("???", format!("?? {} ???", SEPARATOR).as_str(), 1)
                }
                else {
                    local_stem
                }
            }
            else if local_stem.starts_with("???") {    
                local_stem
            }
            else if local_stem.starts_with("???") {
                local_stem
            }
            else {
                format!("?? {} {}", SEPARATOR, local_stem)
            }
        }
        else {
            if local_stem.starts_with("?????????") {
                local_stem.replacen("?????????", "?????????", 1)
            }
            else if local_stem.starts_with("???????") {
                local_stem.replacen("???????", "???????", 1)
            }
            else if local_stem.starts_with("?????") {
                local_stem.replacen("?????", "?????", 1)
            }
            else if local_stem.starts_with("?????") {
                local_stem
            }
            else if local_stem.starts_with("?????????") {
                local_stem.replacen("?????????", "?????????", 1)
            }
            else if local_stem.starts_with("???") {
                local_stem.replacen("???", "?????", 1)
            }
            else if local_stem.starts_with("??????") {
                local_stem.replacen("??????", "????????", 1)
            }
            else if local_stem.starts_with("??????") {
                local_stem.replacen("??????", "??????", 1)
            }
            else if local_stem.starts_with("-?????") {
                local_stem
            }
            else if local_stem.starts_with("?????") {
                local_stem.replacen("?????", "???????", 1)
            }
            else if local_stem.starts_with("????????") {
                local_stem
            }
            else if local_stem.starts_with("?????????") {
                local_stem
            }
            else if local_stem.starts_with("??????") {
                local_stem.replacen("??????", "????????", 1)
            }
            else if local_stem.starts_with("??????") {
                local_stem.replacen("??????", "??????", 1)
            }
            else if local_stem.starts_with("???????") {
                local_stem.replacen("???????", "???????", 1)
            }
            else if local_stem.starts_with("???????") {
                local_stem.replacen("???????", "???????", 1)
            }
            else if local_stem.starts_with("???????") {
                local_stem.replacen("???????", "???????", 1)
            }
            else if local_stem.starts_with("????????") {
                local_stem.replacen("????????", "????????", 1)
            }
            else if local_stem.starts_with("???????????") {
                local_stem.replacen("???????????", "???????????", 1)
            }
            else if local_stem.starts_with("???????") {
                local_stem.replacen("???????", "???????", 1)
            }
            else if local_stem.starts_with("????????") {
                local_stem.replacen("????????", "????????", 1)
            }
            else if local_stem.starts_with("????????????") {
                if self.number == HcNumber::Singular || self.voice != HcVoice::Active {
                    local_stem.replacen("????????????", "??????????????", 1)
                }
                else {
                    local_stem
                }
            }
            else if local_stem.starts_with("????????????") {
                local_stem.replacen("????????????", "??????????????", 1)
            }
            else if local_stem.starts_with("???????????") {
                if self.number == HcNumber::Singular || self.voice != HcVoice::Active {
                    local_stem.replacen("???????????", "?????????????", 1)
                }
                else {
                    local_stem
                }
            }
            else if local_stem.starts_with("???????????") {
                local_stem.replacen("???????????", "?????????????", 1)
            }
            else if local_stem.starts_with("????????") {
                local_stem.replacen("????????", "????????", 1)
            }
            else if local_stem.starts_with("????????") {
                local_stem
            }
            else if local_stem.starts_with("???????") {
                local_stem.replacen("???????", "?????????", 1)
            }
            else if local_stem.starts_with("????????") {
                local_stem.replacen("????????", "??????????", 1)
            }
            else if local_stem.starts_with("?????????") {
                local_stem
            }
            else if local_stem.starts_with("???????") {
                if self.number == HcNumber::Singular || self.voice != HcVoice::Active {
                    local_stem.replacen("???????", "?????????", 1)
                }
                else {
                    local_stem
                }
            }
            else if local_stem.starts_with("????????") {
                if self.number == HcNumber::Singular || self.voice != HcVoice::Active {
                    local_stem.replacen("????????", "??????????", 1)
                }
                else {
                    local_stem
                }
            }
            else if local_stem.starts_with("???") {
                local_stem
            }  
            else if local_stem.starts_with("???") {
                local_stem
            }   
            else if local_stem.starts_with("???") {
                local_stem.replacen("???", "???", 1)
            }   
            else if local_stem.starts_with("?????") {
                local_stem.replacen("?????", "???", 1)
            }   
            else if local_stem.starts_with("?????") {
                local_stem.replacen("?????", "???", 1)
            }  
            else if local_stem.starts_with("?????") {
                local_stem.replacen("?????", "?????", 1)
            }   
            else if local_stem.starts_with("?????") {
                local_stem.replacen("?????", "?????", 1)
            }   
            else if local_stem.starts_with("-???") {
                local_stem
            }   
            else if local_stem.starts_with("???") {
                local_stem
            }   
            else if local_stem.starts_with("???") {
                if self.number == HcNumber::Singular || self.voice != HcVoice::Active {
                    local_stem.replacen("???", "?????", 1)
                }
                else {
                    local_stem
                }
            }
            else if local_stem.starts_with("?????") {
                local_stem
            }
            else if local_stem.starts_with("?????") {
                local_stem.replacen("?????", "???????", 1)
            }   
            else if local_stem.starts_with("???") {
                if self.tense != HcTense::Pluperfect {
                    local_stem.replacen("???", "???", 1)
                }
                else {
                    local_stem
                }
            }
            else if local_stem.starts_with("?????") {
                local_stem
            }
            else if local_stem.starts_with("???") {
                local_stem.replacen("???", "??????", 1)
            }
            else if (self.verb.pps[0].starts_with('???') || self.verb.pps[0].starts_with('???') || self.verb.pps[0].starts_with('???')) && !self.verb.pps[0].starts_with("????????????????????") {
                local_stem.remove(0);
                format!("???{}", local_stem)
            }
            else if local_stem.starts_with('???') || local_stem.starts_with('???') {
                local_stem
            }
            else {
                format!("???{}", local_stem)
            }
        }
    }

    fn deaugment(&self, a:&str, decompose:bool) -> String {
        let mut loc = a.to_string();

        if decompose {
            if loc.starts_with("???????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("???????", format!("??????? {} ?? {} ", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("???????", format!("??????? {} ", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("???????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("???????", format!("??????? {} ?? {} ??", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("???????", format!("??????? {} ??", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("?????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("?????", format!("?? {} ?????", SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("?????", format!("?????").as_str(), 1);
                }
            }
            else if loc.starts_with("?????????")  && (self.mood != HcMood::Indicative || self.number == HcNumber::Plural || self.voice != HcVoice::Active) {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("?????????", format!("??????? {} ?? {} ???", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("?????????", format!("??????? {} ???", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("?????????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("?????????", format!("??????? {} ?? {} ???", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("?????????", format!("??????? {} ???", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("????????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("????????", format!("??????? {} ?? {} ???", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("????????", format!("??????? {} ???", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("???????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("???????", format!("??????? {} ?? {} ??", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("???????", format!("??????? {} ??", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("???????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("???????", format!("??????? {} ???", SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("???????", format!("??????? {} ???", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("-???????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("-???????", format!("- ?? {} ?????", SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("-???????", format!("-?????").as_str(), 1);
                }
            }
            else if loc.starts_with("-?????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    if self.number == HcNumber::Plural || self.voice != HcVoice::Active {
                        loc = loc.replacen("-?????", format!("- ?? {} ???", SEPARATOR).as_str(), 1); //fix me cf -hka
                    }
                }
                else {
                    loc = loc.replacen("-?????", format!("-???").as_str(), 1);
                }
            }
            else if loc.starts_with("????????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("????????", format!("?????? {} ?? {} ", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("????????", format!("?????? {} ", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("????????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("????????", format!("?????? {} ?? {} ", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("????????", format!("?????? {} ", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("???????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("???????", format!("????? {} ?? {} ", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("???????", format!("????? {} ", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("-???") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("-???", format!("- ?? {} ", SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("-???", format!("-").as_str(), 1);
                }
            }
            else if loc.starts_with("??????????")  && (self.mood != HcMood::Indicative || self.number == HcNumber::Plural || self.voice != HcVoice::Active) {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("??????????", format!("?????? {} ?? {} ???", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("??????????", format!("?????? {} ???", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("????????") && self.verb.pps[0].ends_with("???????????") && self.number == HcNumber::Singular {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("????????", format!("?????? {} ???", SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("????????", format!("?????? {} ???", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("????????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("????????", format!("?????? {} ?? {} ??", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("????????", format!("?????? {} ??", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("??????????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("??????????", format!("?????? {} ?? {} ???", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("??????????", format!("?????? {} ???", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("????????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("????????", format!("?????? {} ?? {} ", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("????????", format!("?????? {} ", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("??????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("??????", format!("?????? {} ?? {} ??", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("??????", format!("?????? {} ??", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("??????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("??????", format!("?????? {} ?? {} ??", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("??????", format!("?????? {} ??", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("???????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("???????", format!("??????? {} ?? {} ", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("???????", format!("??????? {} ", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("???????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("???????", format!("??????? {} ?? {} ??"/* FIX ME ??? */, SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("???????", format!("??????? {} ??"/* FIX ME ??? */, SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("?????????") || loc.starts_with("?????????") {  //because pempw and epideiknumi
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("???????", format!("??????? {} ?? {} ", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("???????", format!("??????? {} ", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("???????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("???????", format!("??????? {} ?? {} ", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("???????", format!("??????? {} ", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("????????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("????????", format!("???????? {} ?? {} ", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("????????", format!("???????? {} ", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("????????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("????????", format!("???????? {} ?? {} ", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("????????", format!("???????? {} ", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("????????????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("????????????", format!("???????? {} ?????? {} ?? {} ", SEPARATOR, SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("????????????", format!("???????? {} ?????? {} ", SEPARATOR, SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("???????????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("???????????", format!("??????? {} ?????? {} ?? {} ", SEPARATOR, SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("???????????", format!("??????? {} ?????? {} ", SEPARATOR, SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("????????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("????????", format!("???????? {} ?? {} ", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("????????", format!("???????? {} ", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("?????") && self.verb.pps[0].starts_with("?????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("?????", format!("?? {} ???", SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("?????", "???", 1);
                }
            }
            else if loc.starts_with("?????") && self.verb.pps[0].starts_with("????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("?????", format!("?? {} ?????", SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("?????", "?????", 1);
                }
            }
            else if loc.starts_with('???') {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("???", format!("?? {} ???", SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("???", "???", 1);
                }
            }
            else if loc.starts_with('???') && self.verb.pps[0].starts_with("?????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("???", format!("?? {} ?????", SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("???", "?????", 1);
                }
            }
            else if loc.starts_with('???') && self.verb.pps[0].starts_with("?????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("???", format!("?? {} ?????", SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("???", "?????", 1);
                }
            }
            else if loc.starts_with('???') && (self.verb.pps[0].starts_with('???') || self.verb.pps[0].starts_with("????????") || self.verb.pps[1].starts_with("?????????????????")) {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("???", format!("?? {} ???", SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("???", "???", 1);
                }
            }
            else if loc.starts_with('???') && self.verb.pps[0].starts_with('???') {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("???", format!("?? {} ???", SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("???", "???", 1);
                }
            }
            else if loc.starts_with('???') && (self.verb.pps[0].starts_with('???')) {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("???", format!("?? {} ???", SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("???", "???", 1);
                }
            }
            else if loc.starts_with('???') && (self.verb.pps[0].starts_with('???') || self.verb.pps[0].starts_with('???')) {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("???", format!("?? {} ???", SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("???", "???", 1);
                }
            }
            else if loc.starts_with('???') {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("???", format!("???").as_str(), 1);
                }
                else {
                    return loc;
                }
            }
            else if loc.starts_with("???????") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("???????", format!("?? {} ???", SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("???????", "???", 1);
                }
            }
            else {
                loc.remove(0);
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = format!("?? {} {}", SEPARATOR, loc);
                }
            }
            loc
        }
        else {
            if loc.starts_with("???????") {
                loc = loc.replacen("???????", "???????", 1);
            }
            else if loc.starts_with("???????") {
                loc = loc.replacen("???????", "???????", 1);
            }
            else if loc.starts_with("????????") {
                loc = loc.replacen("????????", "???????", 1);
            }
            else if loc.starts_with("?????") {
                loc = loc.replacen("?????", "?????", 1);
            }
            else if loc.starts_with("???????") {
                loc = loc.replacen("???????", "???????", 1);
            }
            else if loc.starts_with("?????????")  && (self.mood != HcMood::Indicative || self.number == HcNumber::Plural || self.voice != HcVoice::Active) {
                loc = loc.replacen("?????????", "???????", 1);
            }
            else if loc.starts_with("??????????")  && (self.mood != HcMood::Indicative || self.number == HcNumber::Plural || self.voice != HcVoice::Active) {
                loc = loc.replacen("??????????", "????????", 1);
            }
            else if loc.starts_with("?????????") {
                loc = loc.replacen("?????????", "???????", 1);
            }
            else if loc.starts_with("-?????") {
                loc = loc.replacen("-?????", "-???", 1);
            }
            else if loc.starts_with("-???") {
                loc = loc.replacen("-???", "-", 1);
            }
            else if loc.starts_with("-???????") {
                loc = loc.replacen("-???????", "-?????", 1);
            }
            else if loc.starts_with("????????") {
                loc = loc.replacen("????????", "??????", 1);
            }
            else if loc.starts_with("????????") {
                loc = loc.replacen("????????", "??????", 1);
            }
            else if loc.starts_with("???????") {
                loc = loc.replacen("???????", "?????", 1);
            }
            else if loc.starts_with("??????") {
                loc = loc.replacen("??????", "??????", 1);
            }
            else if loc.starts_with("????????") {
                loc = loc.replacen("????????", "????????", 1);
            }
            else if loc.starts_with("??????????") {
                loc = loc.replacen("??????????", "????????", 1);
            }
            else if loc.starts_with("????????") {
                loc = loc.replacen("????????", "??????", 1);
            }
            else if loc.starts_with("???????") {
                loc = loc.replacen("???????", "???????", 1);
            }
            else if loc.starts_with("???????") {
                loc = loc.replacen("???????", "???????", 1);
            }
            else if loc.starts_with("?????????") || loc.starts_with("?????????") { //because pempw and epideiknumi, ?????????????????????
                loc = loc.replacen("???????", "???????", 1);
            }
            else if loc.starts_with("???????") {
                loc = loc.replacen("???????", "???????", 1);
            }
            else if loc.starts_with("????????") {
                loc = loc.replacen("????????", "????????", 1);
            }
            else if loc.starts_with("????????") {
                loc = loc.replacen("????????", "????????", 1);
            }
            else if loc.starts_with("????????????") {
                loc = loc.replacen("????????????", "????????????", 1);
            }
            else if loc.starts_with("???????????") {
                loc = loc.replacen("???????????", "???????????", 1);
            }
            else if loc.starts_with("????????") {
                loc = loc.replacen("????????", "????????", 1);
            }
            else if loc.starts_with("?????") {
                loc = loc.replacen("?????", "???", 1);
            }
            else if loc.starts_with("?????") && self.verb.pps[0].starts_with("????") {
                loc = loc.replacen("?????", "?????", 1);
            }
            else if loc.starts_with("???????") {
                loc = loc.replacen("???????", "???", 1);
            }
            else if loc.starts_with('???') {
                loc = loc.replacen('???', "?????", 1);
            }
            else if loc.starts_with('???') {
                loc = loc.replacen('???', "?????", 1);
            }
            else if loc.starts_with('???') {
                loc = loc.replacen('???', "???", 1);
            }
            else if loc.starts_with('???') && (self.verb.pps[0].starts_with('???') || self.verb.pps[0].starts_with("????????") || self.verb.pps[1].starts_with("?????????????????")) {
                loc.remove(0);
                loc = format!("???{}", loc);
            }
            else if loc.starts_with('???') && self.verb.pps[0].starts_with('???') {
                loc.remove(0);
                loc = format!("???{}", loc);
            }
            else if loc.starts_with('???') && (self.verb.pps[0].starts_with('???')) {
                loc.remove(0);
                loc = format!("???{}", loc);
            }
            else if loc.starts_with('???') && (self.verb.pps[0].starts_with('???') || self.verb.pps[0].starts_with('???')) {
                loc.remove(0);
                loc = format!("???{}", loc);
            }
            else if loc.starts_with('???') {
                return loc;
            }
            else {
                loc.remove(0);
            }
            loc
        }
    }



    fn separate_prefix(&self, stem:&str) ->String {
        // let pre = vec![("???????", vec!["???????"], "")];
        // for p in pre {
        //     if stem.starts_with(p.0) {
        //         return stem.replacen(p.0, format!("{} {}", p.1.join(" - "), p.2).as_str(), 1);
        //     }
        // }

        if stem.starts_with("?????????") {
            return stem.replacen("?????????", format!("??????? {} ????", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("???????") {
            return stem.replacen("???????", format!("??????? {} ", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("????????") {
            return stem.replacen("????????", format!("??????? {} ?????", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("?????????") {
            return stem.replacen("?????????", format!("??????? {} ?????", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("???????") {
            return stem.replacen("???????", format!("??????? {} ???", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("???????") {
            return stem.replacen("???????", format!("??????? {} ???", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("?????????") {
            return stem.replacen("?????????", format!("??????? {} ????", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("???????") {
            return stem.replacen("???????", format!("??????? {} ??", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("?????") {
            return stem.replacen("?????", format!("????? {} ", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("??????") {
            return stem.replacen("??????", format!("?????? {} ", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("??????") {
            return stem.replacen("??????", format!("?????? {} ", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("?????????") {
            return stem.replacen("?????????", format!("?????? {} ?????", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("????????") && self.verb.pps[0].ends_with("???????????"){
            return stem.replacen("????????", format!("?????? {} ???", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("??????????") {
            return stem.replacen("??????????", format!("?????? {} ?????", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("??????") {
            return stem.replacen("??????", format!("?????? {} ", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("??????") { // ????????????
            return stem.replacen("??????", format!("?????? {} ??", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("??????") {
            return stem.replacen("??????", format!("?????? {} ", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("???????") {
            return stem.replacen("???????", format!("??????? {} ", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("???????") {
            return stem.replacen("???????", format!("??????? {} ??"/* FIX ME ??? */, SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("???????") {
            return stem.replacen("???????", format!("??????? {} ", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("????????") {
            return stem.replacen("????????", format!("???????? {} ", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("???????????") { //fix me
            return stem.to_string();
        }
        else if stem.starts_with("???????") {
            return stem.replacen("???????", format!("??????? {} ", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("???????") {
            return stem.replacen("???????", format!("??????? {} ???", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("????????") {
            return stem.replacen("????????", format!("???????? {} ???", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("????????") {
            return stem.replacen("????????", format!("???????? {} ???", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("????????") {
            return stem.replacen("????????", format!("???????? {} ", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("????????????") {
            return stem.replacen("????????????", format!("???????? {} ?????? {} ", SEPARATOR, SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("????????????") {
            return stem.replacen("????????????", format!("???????? {} ?????? {} ???", SEPARATOR, SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("????????????") {
            return stem.replacen("????????????", format!("???????? {} ?????? {} ???", SEPARATOR, SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("???????????") {
            return stem.replacen("???????????", format!("??????? {} ?????? {} ", SEPARATOR, SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("???????????") {
            return stem.replacen("???????????", format!("??????? {} ?????? {} ???", SEPARATOR, SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("???????????") {
            return stem.replacen("???????????", format!("??????? {} ?????? {} ???", SEPARATOR, SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("????????") {
            return stem.replacen("????????", format!("???????? {} ", SEPARATOR).as_str(), 1);
        }
        stem.to_string()
    }

    // fn get_eimi(&self, decompose:bool) -> String {
    //     if self.person == HcPerson::First && self.number == HcNumber::Singular {

    //     }
    // }

    fn get_form(&self, decompose:bool) -> Result<Vec<Step>, HcFormError> {
        if self.mood == HcMood::Subjunctive && self.tense != HcTense::Present && self.tense != HcTense::Aorist {
            if !(self.verb.pps[0].ends_with("????") && (self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect)) {
                return Err(HcFormError::IllegalForm);
            }
        }
        else if self.mood == HcMood::Optative && self.tense != HcTense::Present && self.tense != HcTense::Aorist && self.tense != HcTense::Future {
            if !(self.verb.pps[0].ends_with("????") && (self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect)) {
                return Err(HcFormError::IllegalForm);
            }
        }
        else if self.mood == HcMood::Imperative && self.tense != HcTense::Present && self.tense != HcTense::Aorist {
            if !(self.verb.pps[0].ends_with("????") && (self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect)) {
                return Err(HcFormError::IllegalForm);
            }
        }

        let mut steps = Vec::new();
        //eliminate first person imperatives
        if self.mood == HcMood::Imperative && self.person == HcPerson::First {
            return Err(HcFormError::IllegalForm);
        }

        let f = self.verb.pps.join(", ");
        let e = "Principal Parts".to_string();
        steps.push(Step{form:f, explanation:e});

        let pp_num = self.get_pp_num() as usize;
        let f = &self.verb.pps[pp_num - 1];
        let e = "Choose Principal Part".to_string();
        steps.push(Step{form:f.to_string(), explanation:e});

        if f == BLANK {
            return Err(HcFormError::BlankPrincipalPartForForm);
        }

        if self.voice == HcVoice::Active && self.is_deponent(f) {
            return Err(HcFormError::Deponent);
        }

        //block future passive for passive deponents
        if self.verb.deponent_type() == HcDeponentType::PassiveDeponent && self.tense == HcTense::Future && self.voice == HcVoice::Passive {
            return Err(HcFormError::Deponent);
        }
        
        //abd
        //no passive for middle deponent present or imperfect
        //this does not need to be done for future, aorist because from different pp,
        if self.voice == HcVoice::Passive && (self.tense == HcTense::Present || self.tense == HcTense::Imperfect) && self.verb.pps[0].ends_with("??????") {
            return Err(HcFormError::Deponent);
        }
        
        //for perfect and pluperfect we need to block passive if middle or passive deponent
        if self.voice == HcVoice::Passive && (self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect) && (self.verb.deponent_type() == HcDeponentType::MiddleDeponent || self.verb.deponent_type() == HcDeponentType::PassiveDeponent || self.verb.deponent_type() == HcDeponentType::MiddleDeponentHgeomai) {
            return Err(HcFormError::Deponent);
        }
        
        //middle deponents do not have a passive voice.  H&Q page 316
        if self.voice == HcVoice::Passive && (self.verb.deponent_type() == HcDeponentType::MiddleDeponent || self.verb.deponent_type() == HcDeponentType::GignomaiDeponent) {
            return Err(HcFormError::Deponent);
        }

        if self.voice == HcVoice::Active && (self.verb.deponent_type() == HcDeponentType::MiddleDeponent || self.verb.deponent_type() == HcDeponentType::PassiveDeponent)  && !self.verb.pps[2].ends_with("????????") {
            return Err(HcFormError::Deponent);
        }

        if self.voice == HcVoice::Active && self.tense != HcTense::Perfect && self.tense != HcTense::Pluperfect && self.verb.deponent_type() == HcDeponentType::GignomaiDeponent {
            return Err(HcFormError::Deponent);
        }
        /* 
        if (vf->voice == PASSIVE && deponentType(vf->verb) == PASSIVE_DEPONENT && (vf->tense == PRESENT || vf->tense == IMPERFECT || vf->tense == PERFECT || vf->tense == PLUPERFECT)) //aorist or future are ok
        {
            return 0;
        }
        */

    
        //let mut pps_without_ending = Vec::new();
        //strip accent: internally (not as a step)
        //let f = hgk_strip_diacritics_and_replace_circumflex_with_macron(f, HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE);
        let f = hgk_strip_diacritics(f, HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE);

        let mut pps_without_ending = f.split(" / ").map(|e| e.to_string()).collect::<Vec<String>>();
        // for alt_pp in alt_pps {
        //     let y = self.strip_ending(pp_num, alt_pp.to_string());
        //     if y.is_err() {
        //         return Err("error stripping ending");
        //     }
        //     pps_without_ending.push(y.unwrap());
        // }

        // let f = pps_without_ending.join(" / ");
        // let e = "Remove ending from Principal Part".to_string();
        // steps.push(Step{form:f, explanation:e});

        let mut pps_add_augment = Vec::new();
        //add augment
        if self.tense == HcTense::Imperfect || self.tense == HcTense::Pluperfect {
            for a in &pps_without_ending {
                pps_add_augment.push(self.add_augment(a, decompose));
            }
            pps_without_ending = pps_add_augment;
        }
        else /* remove augment */ if (self.tense == HcTense::Aorist && self.mood == HcMood::Indicative && decompose) || 
            (self.tense == HcTense::Aorist && self.mood != HcMood::Indicative) || 
            (self.tense == HcTense::Future && self.voice == HcVoice::Passive) {
            
            for a in &pps_without_ending {
                pps_add_augment.push(self.deaugment(a, decompose));
            }
            pps_without_ending = pps_add_augment;
        }

        let mut add_ending_collector = Vec::new();
        let mut add_accent_collector = Vec::new();
        for a in pps_without_ending {
            let endings_for_form = self.get_endings(&a);
            if endings_for_form == None {
                return Err(HcFormError::InternalError);//("Illegal form ending");
            }
            
            for e in endings_for_form.unwrap() {

                if a.ends_with("????????") && self.voice == HcVoice::Active {
                    continue;
                }

                let a = self.strip_ending(pp_num, a.to_string());
                if a.is_err() {
                    return Err(HcFormError::UnexpectedPrincipalPartEnding);//("error stripping ending");
                }
                let a = a.unwrap();

                // let f = a.join(" / ");
                // let e = "Remove ending from Principal Part".to_string();
                // steps.push(Step{form:f, explanation:e});

                if self.tense == HcTense::Aorist && self.voice == HcVoice::Passive && self.mood == HcMood::Imperative && self.person == HcPerson::Second && self.number == HcNumber::Singular {
                    if a.ends_with('??') || a.ends_with('??') || a.ends_with('??') {
                        if e == "??????" {
                            continue;
                        }
                    }
                    else {
                        if e == "??????" {
                            continue;
                        }
                    }
                }

                // root aorist: skip middle voice
                if (a.ends_with("??????") || a.ends_with("??????") || a.ends_with("????") || a.ends_with("??????")) && self.tense == HcTense::Aorist && self.voice == HcVoice::Middle {
                    continue;
                }

                if self.verb.pps[0].starts_with("????????????") && a == "??????????" && self.tense == HcTense::Future && self.voice == HcVoice::Passive {
                    continue;
                }

                // skip alternate here because same, could remove this now that we're removing duplicates later?
                if self.verb.pps[0].starts_with("?????????") && ((a.ends_with("????????") && self.person == HcPerson::Second) || (a.ends_with("??????????") && self.person == HcPerson::Third && self.number == HcNumber::Plural)) {
                    continue;
                }
                
                let ending = if decompose { hgk_strip_diacritics(e, HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE) } else { e.to_string() };
                let stem = if decompose && self.tense == HcTense::Aorist && self.voice == HcVoice::Passive && self.mood == HcMood::Subjunctive { format!("{}??", a.to_owned()) } else { a.to_owned() };
                let y = self.add_ending(&stem, &ending, decompose);
                
                let y = match y {
                    Ok(y) => y,
                    _ => return Err(HcFormError::InternalError)//("Error adding ending")
                };
                
                if decompose && self.tense != HcTense::Imperfect && self.tense != HcTense::Pluperfect && self.tense != HcTense::Aorist && !(self.tense == HcTense::Future && self.voice == HcVoice::Passive) {
                    add_ending_collector.push( self.separate_prefix(&y) );
                }
                else {
                    add_ending_collector.push(y.to_string());
                }
                
                if !decompose {
                    let accented_form = if !hgk_has_diacritics(&y, HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE) { self.accent_verb(&y) } else { y };
                    /* contracted future and present */
                    if ((self.tense == HcTense::Imperfect || self.tense == HcTense::Present) && 
                        ( self.verb.pps[0].ends_with("????") || self.verb.pps[0].ends_with("????") || self.verb.pps[0].ends_with("????") || self.verb.pps[0].ends_with("??????????") || self.verb.pps[0].ends_with("??????????") || self.verb.pps[0].ends_with("??????????") )) || 
                        (self.tense == HcTense::Future && self.voice != HcVoice::Passive && (self.verb.pps[1].ends_with('???') || (accented_form.starts_with("?????") && self.verb.pps[1].starts_with("????????")) || self.verb.pps[1].ends_with("???????????"))) {

                        add_accent_collector.push( self.contract_verb(&accented_form, e) );
                    }
                    else {
                        add_accent_collector.push( accented_form );
                    }
                }
            }
        }

        //remove duplicate decomposed forms for proe / prou
        if decompose && self.verb.pps[0] == "??????????????????" && ((self.tense == HcTense::Future && self.voice == HcVoice::Passive) || self.tense == HcTense::Aorist) {
            if add_ending_collector.len() == 2 {
                add_ending_collector.remove(1);
            }
            else if add_ending_collector.len() == 4 {
                add_ending_collector.remove(3);
                add_ending_collector.remove(2);
            }
        }

        //dynamai
        if self.verb.pps[0] == "??????????????" && decompose && self.mood == HcMood::Indicative && (self.tense == HcTense::Imperfect || self.tense == HcTense::Aorist || self.tense == HcTense::Pluperfect) {
            let alt = add_ending_collector[0].replacen('??', "??", 1);
            add_ending_collector.push(alt);
        }

        //euriskw
        if self.verb.pps[0] == "???????????????" && decompose && self.mood == HcMood::Indicative {
            if self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect {
                let alt = add_ending_collector[0].replacen("??", "??", 1);
                add_ending_collector.push(alt);
            }
            else if self.tense == HcTense::Imperfect || self.tense == HcTense::Aorist || self.tense == HcTense::Pluperfect {
                let alt = add_ending_collector[0].replacen("?? ??? ", "", 1);
                add_ending_collector.push(alt);
            }
        }

        //aphihmi
        if self.verb.pps[0] == "????????????????" && decompose && self.person == HcPerson::Second && self.number == HcNumber::Singular && self.tense == HcTense::Present && self.voice == HcVoice::Active && self.mood == HcMood::Indicative {
            let alt = String::from("??????? ??? ??????? ??? ??????");
            add_ending_collector.push(alt);
        }
        else if self.verb.pps[0] == "?????????????????" && decompose && self.person == HcPerson::Second && self.number == HcNumber::Singular && self.tense == HcTense::Present && self.voice == HcVoice::Active && self.mood == HcMood::Indicative {
            let alt = String::from("?????? ??? ??????? ??? ??????");
            add_ending_collector.push(alt);
        }
        else if self.verb.pps[0] == "?????????????" && decompose && self.person == HcPerson::Second && self.number == HcNumber::Singular && self.tense == HcTense::Present && self.voice == HcVoice::Active && self.mood == HcMood::Indicative {
            let alt = String::from("??????? ??? ??????");
            add_ending_collector.push(alt);
        }

        //add alts for ????????????????????
        if self.verb.pps[0] == "????????????????????" && decompose && (self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect) {
            if add_ending_collector.len() > 0 && add_ending_collector[0] == "???????????? ??? ????????" {
                let alt = String::from("???????? ??? ????????");
                add_ending_collector.push(alt);
            }
            else if add_ending_collector.len() > 0 && add_ending_collector[0] == "???????????? ??? ??????" {
                let alt = String::from("???????? ??? ??????");
                add_ending_collector.push(alt);
            }
            else if add_ending_collector.len() > 0 && add_ending_collector[0] == "???????????? ??? ???????(??)" {
                let alt = String::from("???????? ??? ???????(??)");
                add_ending_collector.push(alt);
            }
            else if add_ending_collector.len() > 0 && add_ending_collector[0] == "?? ??? ???????????? ??? ????????" {
                let alt = String::from("?? ??? ???????? ??? ????????");
                add_ending_collector.push(alt);
            }
        }
        
        if add_ending_collector.len() == 0 { //this catches meanesthn in aorist middle, etc.; fix me? should be better way to catch these
            return Err(HcFormError::InternalError);
        }
        let f = add_ending_collector.join(" / ");
        let e = "Add ending".to_string();
        steps.push(Step{form:f, explanation:e});
    
        if !decompose {
            //add proe / prou forms for imperfect
            if self.verb.pps[0] == "??????????????????" && (self.tense == HcTense::Imperfect || self.tense == HcTense::Pluperfect) {
                let alt = add_accent_collector[0].replacen("????????", "????????", 1);
                add_accent_collector.push(alt);
            }

            //aphihmi
            if self.verb.pps[0] == "????????????????" && self.person == HcPerson::Second && self.number == HcNumber::Singular && self.tense == HcTense::Present && self.voice == HcVoice::Active && self.mood == HcMood::Indicative {
                let alt = String::from("???????????????");
                add_accent_collector.push(alt);
            }
            else if self.verb.pps[0] == "?????????????????" && self.person == HcPerson::Second && self.number == HcNumber::Singular && self.tense == HcTense::Present && self.voice == HcVoice::Active && self.mood == HcMood::Indicative {
                let alt = String::from("????????????????");
                add_accent_collector.push(alt);
            }
            else if self.verb.pps[0] == "?????????????" && self.person == HcPerson::Second && self.number == HcNumber::Singular && self.tense == HcTense::Present && self.voice == HcVoice::Active && self.mood == HcMood::Indicative {
                let alt = String::from("????????????");
                add_accent_collector.push(alt);
            }

            //add alts for ????????????????????
            if self.verb.pps[0] == "????????????????????" && (self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect) {
                if add_accent_collector.len() > 0 && add_accent_collector[0] == "????????????????????" {
                    let alt = String::from("????????????????");
                    add_accent_collector.push(alt);
                }
                else if add_accent_collector.len() > 0 && add_accent_collector[0] == "??????????????????" {
                    let alt = String::from("??????????????");
                    add_accent_collector.push(alt);
                }
                else if add_accent_collector.len() > 0 && add_accent_collector[0] == "???????????????????(??)" {
                    let alt = String::from("???????????????(??)");
                    add_accent_collector.push(alt);
                }
                else if add_accent_collector.len() > 0 && add_accent_collector[0] == "???????????????????????" {
                    let alt = String::from("???????????????????");
                    add_accent_collector.push(alt);
                }
            }

            //dynamai
            if self.verb.pps[0] == "??????????????" && (self.tense == HcTense::Imperfect || self.tense == HcTense::Aorist || self.tense == HcTense::Pluperfect) {
                let alt = add_accent_collector[0].replacen('???', "???", 1);
                add_accent_collector.push(alt);
            }

            //euriskw
            if self.verb.pps[0] == "???????????????" && self.mood == HcMood::Indicative && (self.tense == HcTense::Perfect || self.tense == HcTense::Imperfect || self.tense == HcTense::Aorist || self.tense == HcTense::Pluperfect) {
                let alt = add_accent_collector[0].replacen('??', "??", 1);
                add_accent_collector.push(alt);
            }

            //remove duplicate and then join alternates with /
            let f = add_accent_collector.into_iter().unique().collect::<Vec<String>>().join(" / ");
            let e = "Accent verb".to_string();
            steps.push(Step{form:f, explanation:e});   
        }

        Ok(steps)
    }

    fn accent_verb(&self, word:&str) -> String {
        let syl = analyze_syllable_quantities(word, self.person, self.number, self.tense, self.mood, self.verb.properties);

        let accent;
        let letter_index;
        if syl.len() > 2 && !syl.last().unwrap().is_long { //acute on antepenult
            accent = HGK_ACUTE;
            letter_index = syl[0].index;
        }
        else if syl.len() == 2 && syl[0].is_long && !syl[1].is_long {
            if (syl[1].letters == "????" || syl[1].letters == "????") && self.mood == HcMood::Optative {
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
        else if syl.len() == 1 { //acute on ultima. e.g. do/s
            if syl[0].is_long {
                accent = HGK_CIRCUMFLEX;
            }
            else {
                accent = HGK_ACUTE;
            }
            letter_index = syl[0].index;
        }
        else {
            return String::from(word);
        }

        self.accent_syllable(word, letter_index, accent)
    }

    fn accent_verb_contracted(&self, word:&str, orig_syllables:Vec<SyllableAnalysis>, ending:&str) -> String {
        let syl = analyze_syllable_quantities(word, self.person, self.number, self.tense, self.mood, self.verb.properties);

        let esyl = analyze_syllable_quantities(ending, self.person, self.number, self.tense, self.mood, self.verb.properties);
        let accent;
        let letter_index;
        if orig_syllables.len() > 2 && !orig_syllables.last().unwrap().is_long { //acute on antepenult
            //println!("AAAAA1 {}", word);
            //accent = HGK_ACUTE;
            //letter_index = orig_syllables[0].index;
            //***was acute on antepenult, now acute on penult
            if esyl.len() == 3 {
                accent = HGK_ACUTE;
                letter_index = syl[syl.len() - 3].index;
            }
            else {
                if syl.last().unwrap().is_long {
                    accent = HGK_ACUTE;
                    letter_index = syl[syl.len() - 2].index;
                }
                else {
                    accent = HGK_CIRCUMFLEX;
                    letter_index = syl[syl.len() - 2].index;
                }
            }
        }
        /* 
        else if orig_syllables.len() == 2 && orig_syllables[0].is_long && !orig_syllables[1].is_long {
            //println!("AAAAA2 {}", word);
            if (orig_syllables[1].letters == "????" || orig_syllables[1].letters == "????") && self.mood == HcMood::Optative {
                accent = HGK_ACUTE; //exception to the exception for optative 3rd singular: acute on penult
                // ***same?
            }
            else {
                //println!("AAAAA3 {}", word);
                accent = HGK_CIRCUMFLEX; //circumflex on penult
                // ***now acute on penult?
            }
            letter_index = orig_syllables[].index;
        }
        */
        else if orig_syllables.len() > 1 { //acute on penult
            //println!("AAAAA4 {}", word);
            //accent = HGK_ACUTE;
            //letter_index = orig_syllables[orig_syllables.len() - 2].index;
            //***now circumflex on ultima
            if esyl.len() == 2 && esyl[0].is_long && esyl[1].is_long {
                accent = HGK_ACUTE;
                letter_index = syl[syl.len() - 2].index;
            }
            else if esyl.len() == 2 && !esyl[0].is_long && esyl[1].is_long {
                accent = HGK_ACUTE;
                letter_index = syl[syl.len() - 2].index;
            }
            else {
                accent = HGK_CIRCUMFLEX;
                letter_index = syl[syl.len() - 1].index;
            }

        }
        else {
            //println!("AAAAA5");
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

    fn accent_syllable_start(&self, word:&str, letter_index_from_end:u8, accent:u32) -> String {
        let v = word.gkletters().enumerate().map(|(x, mut a)| { 
            if x == letter_index_from_end as usize {
                a.toggle_diacritic(accent, true);
                //println!("abc {:?} {:?} {:?}", a.letter, accent, letter_index_from_end);
            } 
            a}).collect::<Vec<HGKLetter>>();

            let s = v.iter().map(|a|{ a.to_string(HgkUnicodeMode::Precomposed)}).collect::<String>();
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

    fn get_endings(&self, stem: &str) -> Option<Vec<&str>> {
        let ending = match self.tense {
            HcTense::Present => {
                match self.voice {
                    HcVoice::Active => {
                        match self.mood {
                            HcMood::Indicative => if self.verb.pps[0].ends_with("????") { HcEndings::PresentActiveIndicativeMi } else { HcEndings::PresentActiveInd },
                            HcMood::Subjunctive => if self.verb.pps[0].ends_with("????") && !self.verb.pps[0].ends_with("???????") { HcEndings::AoristPassiveSubj } else { HcEndings::PresentActiveSubj },
                            HcMood::Optative => if self.verb.pps[0].ends_with("????") && !self.verb.pps[0].ends_with("???????") { HcEndings::PresentActiveOptMi } else { if self.verb.pps[0].ends_with("????") || self.verb.pps[0].ends_with("????") || self.verb.pps[0].ends_with("????") { HcEndings::PresentActiveOptEContracted} else { HcEndings::PresentActiveOpt } },
                            HcMood::Imperative => HcEndings::PresentActiveImperative,
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    },
                    HcVoice::Middle | HcVoice::Passive => {
                        match self.mood {
                            HcMood::Indicative => if self.verb.pps[0].ends_with("????") || self.verb.pps[0].ends_with("????????") || self.verb.pps[0].ends_with("????????")  { HcEndings::PerfectMidpassInd } else { HcEndings::PresentMidpassInd },
                            HcMood::Subjunctive => HcEndings::PresentMidpassSubj,
                            HcMood::Optative => if self.verb.pps[0].ends_with("??????") && !self.verb.pps[0].ends_with("??????????") && !self.verb.pps[0].ends_with("???????????") && !self.verb.pps[0].ends_with("?????????????") { HcEndings::PresentMidpassOptTithhmi } else if (self.verb.pps[0].ends_with("????") && !self.verb.pps[0].ends_with("???????")) || self.verb.pps[0].ends_with("????????") { HcEndings::MiddleOptMi } else { HcEndings::PresentMidpassOpt },
                            HcMood::Imperative => if self.verb.pps[0].ends_with("????") || self.verb.pps[0].ends_with("????????") || self.verb.pps[0].ends_with("????????") { HcEndings::PresentMidpassImperativeMi } else { HcEndings::PresentMidpassImperative },
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
                            HcMood::Indicative => if self.verb.pps[0].ends_with("????") { HcEndings::ImperfectActiveMi } else { HcEndings::ImperfectActiveInd },
                            HcMood::Subjunctive => HcEndings::NotImplemented,
                            HcMood::Optative => HcEndings::NotImplemented,
                            HcMood::Imperative => HcEndings::NotImplemented,
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    },
                    HcVoice::Middle | HcVoice::Passive => {
                        match self.mood {
                            HcMood::Indicative => if self.verb.pps[0].ends_with("????") || self.verb.pps[0].ends_with("????????") || self.verb.pps[0].ends_with("????????") { HcEndings::PluperfectMidpassInd } else { HcEndings::ImperfectMidpassInd },
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

                            HcMood::Optative => if /* contracted future */ self.verb.pps[1].ends_with('???') || ( stem.starts_with("?????") && self.verb.pps[1].starts_with("????????")) { HcEndings::PresentActiveOptEContracted} else { HcEndings::PresentActiveOpt },
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
                        if stem.ends_with("????") {
                            match self.mood {
                                HcMood::Indicative => HcEndings::ImperfectActiveInd,
                                HcMood::Subjunctive => HcEndings::PresentActiveSubj,
                                HcMood::Optative => if self.verb.pps[0].ends_with("???????") { HcEndings::AoristOptativeEchw } else { HcEndings::PresentActiveOpt },
                                HcMood::Imperative => HcEndings::PresentActiveImperative,
                                HcMood::Infinitive => HcEndings::NotImplemented,
                                HcMood::Participle => HcEndings::NotImplemented,
                            }
                        }
                        else {
                            match self.mood {
                                
                                HcMood::Indicative => if stem.ends_with("????????") || stem.ends_with("????????") || stem.ends_with("??????") || stem.ends_with("????????") { 
                                        HcEndings::AoristActiveIndicativeMiRoot } 
                                    else if self.verb.pps[0].ends_with("????") && self.verb.pps[2].ends_with("????") {
                                        HcEndings::MixedAoristMi }
                                    else { HcEndings::AoristActiveInd },
                                HcMood::Subjunctive => if stem.ends_with("????????") || stem.ends_with("????????") || stem.ends_with("??????") || stem.ends_with("????????") { HcEndings::AoristPassiveSubj } else { HcEndings::PresentActiveSubj },
                                HcMood::Optative => if stem.ends_with("????????") || stem.ends_with("????????") || stem.ends_with("??????") || stem.ends_with("????????") { HcEndings::PresentActiveOptMi } else if self.verb.pps[0].ends_with("????") && self.verb.pps[2].ends_with("????") { HcEndings::AoristPassiveOpt } else { HcEndings::AoristActiveOpt },
                                HcMood::Imperative => if stem.ends_with("????????") || stem.ends_with("????????") || stem.ends_with("??????") || stem.ends_with("????????") { HcEndings::AoristActiveImperativesMiRoot } else if self.verb.pps[0].ends_with("????") && self.verb.pps[2].ends_with("????") { HcEndings::AoristActiveImperativesMi } else { HcEndings::AoristActiveImperative },
                                HcMood::Infinitive => HcEndings::NotImplemented,
                                HcMood::Participle => HcEndings::NotImplemented,
                            }                            
                        }
                    },
                    HcVoice::Middle => {
                        if stem.ends_with("????") || stem.ends_with("????????") {
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
                                HcMood::Indicative => if self.verb.pps[0].ends_with("????") && self.verb.pps[2].ends_with("????") { HcEndings::ImperfectMidpassInd } else { HcEndings::AoristMidInd },
                                HcMood::Subjunctive => HcEndings::PresentMidpassSubj,
                                HcMood::Optative => if self.verb.pps[0].ends_with("??????") && self.verb.pps[2].ends_with("????") { HcEndings::PresentMidpassOptTithhmi } else if self.verb.pps[0].ends_with("????") && stem.ends_with("????") { HcEndings::MiddleOptMi } else { HcEndings::AoristMiddleOpt },
                                HcMood::Imperative => if self.verb.pps[0].ends_with("????") && self.verb.pps[2].ends_with("????") { HcEndings::PresentMidpassImperativeMi } else { HcEndings::AoristMiddleImperative },
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

pub struct SyllableAnalysis {
    letters: String,
    is_long: bool,
    index: u8,
}

use unicode_segmentation::UnicodeSegmentation;
static PREFIXES: &[&str; 16] = &["?????", "???????", "??????", "??????", "??????", "??????", "???????", "?????", "?????", "??????", "????????", "????????????", "??????????", "????????", "?????????", "???????"];

fn analyze_syllable_quantities(word:&str, p:HcPerson, n:HcNumber, t:HcTense, m:HcMood, props:u32) -> Vec<SyllableAnalysis> {
    let mut letters = word.gkletters();

    //    /*
    //  For prefixes, find where the prefix ends and don't look past that character
    //  */
    // if ((vf->verb->verbclass & PREFIXED) == PREFIXED && !utf8HasSuffix(vf->verb->present, "??????????????") && ((vf->tense == AORIST && vf->mood == INDICATIVE) || vf->tense == PERFECT || vf->tense == PLUPERFECT))
    // {

        let mut area = word.len();
        if (props & PREFIXED) == PREFIXED && ((t == HcTense::Aorist && m == HcMood::Indicative) || t == HcTense::Perfect || t == HcTense::Pluperfect) {
            for p in PREFIXES {
                if word.starts_with(p) {
                    area = p.graphemes(true).count();
                    println!("area: {} {}", p, area);
                    break;
                }
            }
        
    //     if (hasPrefix(tempUcs2String, *len, ek, 2))
    //         accentableRegionStart = 2;
    //     else if (hasPrefix(tempUcs2String, *len, ana, 3))
    //         accentableRegionStart = 3;
    //     else if (hasPrefix(tempUcs2String, *len, sum, 3))
    //         accentableRegionStart = 3;
    //     else if (hasPrefix(tempUcs2String, *len, sun, 3))
    //         accentableRegionStart = 3;
    //     else if (hasPrefix(tempUcs2String, *len, dia, 3))
    //         accentableRegionStart = 3;
    //     else if (hasPrefix(tempUcs2String, *len, apo, 3) && !utf8HasSuffix(vf->verb->present, "????????????????????"))
    //         accentableRegionStart = 3;
    //     else if (hasPrefix(tempUcs2String, *len, ap, 2))
    //         accentableRegionStart = 2;
    //     else if (hasPrefix(tempUcs2String, *len, aph, 2))
    //         accentableRegionStart = 2;
    //     else if (hasPrefix(tempUcs2String, *len, kath, 3))
    //         accentableRegionStart = 3;
    //     else if (hasPrefix(tempUcs2String, *len, kata, 4))
    //         accentableRegionStart = 4;
    //     else if (hasPrefix(tempUcs2String, *len, metana, 6))
    //         accentableRegionStart = 6;
    //     else if (hasPrefix(tempUcs2String, *len, metan, 5))
    //         accentableRegionStart = 5;
    //     else if (hasPrefix(tempUcs2String, *len, meta, 4))
    //         accentableRegionStart = 4;
    //     else if (hasPrefix(tempUcs2String, *len, epan, 4))
    //         accentableRegionStart = 4;
    //     else if (hasPrefix(tempUcs2String, *len, epi, 3))
    //         accentableRegionStart = 3;
    //     else
    //         accentableRegionStart = 0;
    // }
        }

    let mut letter_num = 0;
    let mut last_letter = '\u{0000}';
    let mut res = Vec::new();
    let word_len = word.graphemes(true).count();
    loop {
        match letters.next_back() {
            Some(x) => { 
                //println!("letter: {:?}", x);
                match x.letter_type() {
                    HgkLetterType::HgkLongVowel => {
                        if last_letter == '??' && x.letter == '??' {
                            res.pop();
                            let mut s = String::from(x.letter);
                            s.push(last_letter);
                            res.push(SyllableAnalysis {letters: s, is_long: true, index: letter_num - 1});
                        }
                        else {
                            last_letter = '\u{0000}';
                            res.push(SyllableAnalysis {letters: x.to_string(HgkUnicodeMode::Precomposed), is_long: true, index: letter_num});
                        }
                    },
                    HgkLetterType::HgkShortVowel => {
                        if x.letter == '??' || x.letter == '??' && (x.diacritics & HGK_DIAERESIS) != HGK_DIAERESIS {
                            last_letter = x.letter;
                            //res.push((x.letter.to_string(), false, letter_num)); //add short, might be replaced by diphthong
                            res.push(SyllableAnalysis {letters: x.letter.to_string(), is_long: false, index: letter_num});
                        }
                        else {
                            if last_letter != '\u{0000}' && (x.letter == '??' || x.letter == '??' || x.letter == '??') {
                                res.pop();
                                let mut s = String::from(x.letter);
                                s.push(last_letter);

                                let is_short = letter_num == 1 && (x.letter == '??' || x.letter == '??') && last_letter == '??';//final diphthongs short accent
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
                //println!("len {}, num {}, area {}", word_len, letter_num, area);
                if word_len - letter_num as usize == area {
                    break;
                }
            },
            None => {
                break;
            },
        }
    }
    res.reverse();
    res
}

static ENDINGS: &[[&str; 6]; 38] = &[
    ["??", "??????", "????", "????????", "??????", "????????(??)"],//, "Present Active Indicative" },
    ["????", "????", "??(??)", "????????", "??????", "????"],//, "Imperfect Active Indicative" },
    ["??", "????", "??(??)", "????????", "??????", "????"],//, "Aorist Active Indicative" },
    ["??", "????", "??(??)", "????????", "??????", "????????(??)"],//, "Perfect Active Indicative" },
    ["??", "????", "????(??)", "????????", "??????", "????????"],//, "Pluperfect Active Indicative" },
    ["??", "?????", "???", "????????", "??????", "??????(??)"],//, "Present Active Subjunctive" },
    ["????????", "??????", "????", "??????????", "????????", "????????"],//, "Present Active Optative" },
    ["????????", "??????,????????", "????,??????(??)", "??????????", "????????", "????????,????????"],//, "Aorist Active Optative" },
    ["????????", "????,???", "????????", "??????????", "????????", "??????????"],//, "Present Middle/Passive Indicative" },
    ["????????", "????", "??????", "??????????", "????????", "????????"],//, "Imperfect Middle/Passive Indicative" },
    ["????", "????", "??", "????????", "??????", "????????"],//, "Aorist Passive Indicative" },
    ["????????", "??", "??????", "??????????", "????????", "????????"],//, "Aorist Middle Indicative" },
    ["???", "?????", "???", "?????????", "???????", "???????(??)"],//***, "Aorist Passive Subjunctive" },
    ["????????", "????????", "??????", "???????????,????????????", "?????????,??????????", "?????????,????????????"],//, "Aorist Passive Optative" },
    ["??????????", "??????", "????????", "????????????", "??????????", "??????????"],//, "Aorist Middle Optative" },
    ["??????", "??????", "??????", "????????", "??????", "????????"],//, "Perfect Middle/Passive Indicative" },
    ["??????", "????", "????", "????????", "??????", "??????"],//, "Pluperfect Middle/Passive Indicative" },
    ["????????", "???", "????????", "??????????", "????????", "??????????"],//, "Present Middle/Passive Subjunctive" },
    ["??????????", "??????", "????????", "????????????", "??????????", "??????????"],//, "Present Middle/Passive Optative" },
    ["", "??", "??????",   "", "??????", "??????????"],//, "Present Active Imperative" },
    ["", "????", "????????", "", "????????", "??????????"],//, "Present Middle/Passive Imperative" },
    ["", "????", "??????",  "", "??????", "??????????"],//, "Aorist Active Imperative" },
    ["", "????", "????????", "", "????????", "??????????"],//, "Aorist Middle Imperative" },
    ["", "??????,??????", "??????", "", "??????", "??????????"],//, "Aorist Passive Imperative" },
    
    ["????????,????????", "??????,????????", "????,??????", "??????????,????????????", "????????,??????????", "????????,????????????"],//, "" },//pres act opt e
    
    ["????", "??", "????(??)", "??????", "????", "????????(??)"],//, "" },   //mi
    
    ["", "??", "????", "", "????", "????????"],//, "" },//mi aorist active imperatives
    ["", "????", "????", "", "????", "????????"],//", "" },//mi root aorist active imperatives
    
    ["", "??", "??????", "", "??????", "????????"],//, "Root Aorist Middle Imperative" },//mi root aorist middle imperatives
    ["??", "??", "", "??????", "????", "??????"],//, "Root Aorist Indicative" },//mi root aorist indicative
    
    ["", "?????", "????????", "", "????????", "??????????"],//, "Present Middle/Passive Imperative" }, //second aorist middle/passive imperatives
    ["????????", "?????", "???????,?????????", "??????????,????????????", "?????????,???????????", "?????????,???????????"],//, "Present Middle/Passive Optative Tithemi" }, //Exception: H&Q page 347
    //["????", "????", "??", "????????", "??????", "????"],//***, "Imperfect Active Indicative" } //this is only for contracted verbs when decompose so the nu moveable doesn't show up
    ["", "????", "??????", "", "??????", "????????"],
    ["??", "??", "", "??????", "????", "??????"],
    ["??", "????", "??(??)", "??????", "????", "??????"],
    ["????????", "?????", "???????", "??????????", "?????????", "?????????"],
    ["??????", "??????", "????", "?????????,??????????", "???????,????????", "???????,??????????"],//, "Aorist Passive Optative" },
    ["????????", "????????", "??????", "??????????", "????????", "????????"],
    ];

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    use unicode_normalization::UnicodeNormalization;

    // #[test]
    // fn separate_prefix_test() {
    //     let stem = "???????????????????";
    //     let stem = "????????????????????????";
    //     let pre = vec![("???????", vec!["???????", ""], ""), ("????????????", vec!["????????", "??????", ""], "???")];
    //     for p in pre {
    //         if stem.starts_with(p.0) {
    //             assert_eq!(stem.replacen(p.0, format!("{}{}", p.1.join(" - "), p.2).as_str(), 1), "??????? - ????????????");
    //         }
    //     }
    // }

    #[test]
    fn test_rreplacen() {
        let s = "f2o f2o 123 foo".to_string();
        assert_eq!("f2o f2o 1new3 foo", s.rreplacen("2", "new", 1));
        assert_eq!("f2o fnewo 1new3 foo", s.rreplacen("2", "new", 2));
    }

    #[test]
    fn accent_tests() {
        let luw = "???????, ?????????, ????????????, ????????????, ??????????????, ?????????????";
        let a = HcGreekVerb::from_string(1, luw, REGULAR).unwrap();
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Aorist, voice:HcVoice::Active, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form(false).unwrap()[1].form, "????????????");
        assert_eq!(b.accent_verb("??????????????"), "??????????????");
        assert_eq!(b.accent_verb("????\u{0304}??"), "???????");
        assert_eq!(b.accent_verb("??\u{1FE1}??"), "???????");
        assert_eq!(b.accent_verb("???????????????"), "???????????????");
    }

    #[test]
    fn normalization_tests() {
        let alphamacron_acute = "\u{1FB1}\u{0301}"; //alpha with macron + acute
        assert_eq!(alphamacron_acute.nfc().collect::<String>(), alphamacron_acute);

        let alpha_macron_acute = "\u{03B1}\u{0304}\u{0301}"; //alpha + macron + acute
        assert_eq!(alpha_macron_acute.nfc().collect::<String>(), alphamacron_acute); //composes to alpha with macron + acute
        assert_ne!(alpha_macron_acute.nfc().collect::<String>(), alpha_macron_acute);//does not compose to alpha + macron + acute

        assert_eq!(alphamacron_acute.nfd().collect::<String>(), alpha_macron_acute); //decomposes to alpha + macron + acute

        //order matters here
        let alpha_acute_macron = "\u{03B1}\u{0301}\u{0304}"; //alpha + acute + macron 
        assert_ne!(alpha_acute_macron.nfc().collect::<String>(), alphamacron_acute); //does not compose to alpha with macron + acute, = alpha with acute + macron
        
        //order matters here too
        let alpha_smooth_acute = "\u{03B1}\u{0313}\u{0301}";
        assert_eq!(alpha_smooth_acute.nfc().collect::<String>(), "\u{1F04}");
        let alpha_acute_smooth = "\u{03B1}\u{0301}\u{0313}";
        assert_ne!(alpha_acute_smooth.nfc().collect::<String>(), "\u{1F04}");
    }

    #[test]
    fn it_works() {
        let luw = "???????, ?????????, ????????????, ????????????, ??????????????, ?????????????";
        let blaptw = "????????????, ??????????, ?????????????, ??????????????, ??????????????????, ??????????????? / ?????????????????";

        let luwverb = HcGreekVerb::from_string(1, luw, REGULAR).unwrap();
        let a1 = HcGreekVerb {id:1,pps:vec!["???????".to_string(), "?????????".to_string(), "????????????".to_string(), "????????????".to_string(), "??????????????".to_string(), "?????????????".to_string()],properties: REGULAR};
        assert_eq!(luwverb, a1);
        
        let b = HcGreekVerbForm {verb:&luwverb, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Aorist, voice:HcVoice::Active, mood:HcMood::Indicative, gender:None, case:None};
        let c = HcGreekVerbForm {verb:&luwverb, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Aorist, voice:HcVoice::Active, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b, c);
        
        assert_eq!(b.get_form(false).unwrap()[0].form, luw);
        assert_eq!(b.get_form(false).unwrap()[1].form, "????????????");
        
        assert_eq!(b.get_form(false).unwrap()[2].form, "????????????");
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "????????????");
        
        assert_eq!(b.get_pp_num(), HcGreekPrincipalParts::Third);
        assert_eq!(b.get_pp_num() as usize, 3);
        assert_eq!(b.verb.pps[b.get_pp_num() as usize - 1], "????????????");
        assert_eq!(b.get_pp(), Some(String::from("????????????")));

        let a = HcGreekVerb::from_string(1, blaptw, REGULAR).unwrap();
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Aorist, voice:HcVoice::Passive, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form(false).unwrap()[2].form, "??????????????? / ?????????????????"); 
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Present, voice:HcVoice::Active, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form(false).unwrap()[2].form, "????????????");
        assert_eq!(b.get_endings("").unwrap()[0], "??");

        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Present, voice:HcVoice::Middle, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form(false).unwrap()[3].form, "??????????????????");
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::Second, number:HcNumber::Singular, tense:HcTense::Present, voice:HcVoice::Middle, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_endings("").unwrap()[0], "????");
        assert_eq!(b.get_endings("").unwrap()[1], "???");
        assert_eq!(b.get_form(false).unwrap()[3].form, "?????????????? / ?????????????");
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::Third, number:HcNumber::Singular, tense:HcTense::Present, voice:HcVoice::Middle, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form(false).unwrap()[3].form, "??????????????????");
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Plural, tense:HcTense::Present, voice:HcVoice::Middle, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form(false).unwrap()[3].form, "????????????????????");
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::Second, number:HcNumber::Plural, tense:HcTense::Present, voice:HcVoice::Middle, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form(false).unwrap()[3].form, "??????????????????");
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::Third, number:HcNumber::Plural, tense:HcTense::Present, voice:HcVoice::Middle, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form(false).unwrap()[3].form, "????????????????????");

        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Future, voice:HcVoice::Active, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form(false).unwrap()[2].form, "??????????");
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Perfect, voice:HcVoice::Active, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form(false).unwrap()[2].form, "??????????????");
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Perfect, voice:HcVoice::Passive, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form(false).unwrap()[2].form, "??????????????????");

        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Pluperfect, voice:HcVoice::Passive, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "?????????????????????");

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

                        let verb = HcGreekVerb::from_string_with_properties(idx as u32, &line).unwrap();

                        if paradigm_reader.read_line(&mut paradigm_line).unwrap() == 0 { return; }
                        paradigm_line.clear();

                        let partial = if verb.deponent_type() == HcDeponentType::PartialDeponent { " (Partial Deponent)" } 
                            else if verb.deponent_type() == HcDeponentType::MiddleDeponent { " (Middle Deponent)"} 
                            else if verb.deponent_type() == HcDeponentType::PassiveDeponent { " (Passive Deponent)"} 
                            else if verb.deponent_type() == HcDeponentType::GignomaiDeponent { " (Deponent gignomai)"} 
                            else if verb.deponent_type() == HcDeponentType::MiddleDeponentHgeomai { " (Middle Deponent with 6th pp)"} 
                            else { "" };
     
                        let verb_section = format!("Verb {}. {}{}", idx, if verb.pps[0] != "???" { verb.pps[0].clone() } else { verb.pps[1].clone() }, partial);
                        println!("\n{}", verb_section);
                        if paradigm_reader.read_line(&mut paradigm_line).unwrap() != 0 && idx != 76 && idx != 77 && idx != 78 && idx != 91 && idx != 95 && idx != 118 && idx != 119 && idx != 121 && idx != 122 && idx != 126 { 
                            assert_eq!(paradigm_line[0..paradigm_line.len() - 1], verb_section);
                        }
                        paradigm_line.clear();

                            for x in [HcTense::Present, HcTense::Imperfect, HcTense::Future, HcTense::Aorist, HcTense::Perfect, HcTense::Pluperfect] {   
                                for v in [HcVoice::Active,HcVoice::Middle,HcVoice::Passive] { 
                                for m in [HcMood::Indicative, HcMood::Subjunctive,HcMood::Optative,HcMood::Imperative] {
                                    
                                    if  ((m == HcMood::Subjunctive || m == HcMood::Optative || m == HcMood::Imperative) && (x == HcTense::Imperfect || x == HcTense::Perfect || x == HcTense::Pluperfect)) || x == HcTense::Future && (m == HcMood::Subjunctive || m == HcMood::Imperative) {
                                        //allow moods for oida, synoida
                                        if !((m == HcMood::Subjunctive || m == HcMood::Optative || m == HcMood::Imperative )&& x == HcTense::Perfect && v == HcVoice::Active && (verb.pps[0] == "?????????" || verb.pps[0] == "??????????????")) {
                                            continue;
                                        }
                                    }

                                    if paradigm_reader.read_line(&mut paradigm_line).unwrap() == 0 { return; }
                                    paradigm_line.clear();

                                    let section = format!("{} {} {}", x.value(), get_voice_label(x, v, m, verb.deponent_type()), m.value());
                                    //if m == HcMood::Imperative { section = section.replacen(" (Middle/Passive)", "", 1)};
                                    println!("\n{}", section);
                                    if paradigm_reader.read_line(&mut paradigm_line).unwrap() != 0 { 
                                        //assert_eq!(paradigm_line[0..paradigm_line.len() - 1], section);
                                    }
                                    paradigm_line.clear();

                                    for z in [HcNumber::Singular, HcNumber::Plural] {
                                        for y in [HcPerson::First, HcPerson::Second, HcPerson::Third] {

                                            let form = HcGreekVerbForm {verb:&verb, person:y, number:z, tense:x, voice:v, mood:m, gender:None, case:None};
                                            let r = match form.get_form(false) {
                                                Ok(res) => res.last().unwrap().form.to_string(),
                                                Err(_a) => "NF".to_string()
                                            };
                                            
                                            let r_d = match form.get_form(true) {
                                                Ok(res) => res.last().unwrap().form.to_string(),
                                                Err(_a) => "NDF".to_string()
                                            };

                                            let form_line = format!("{}{}: {} ; {}", y.value(), z.value(), 
                                                str::replace(&r, " /", ","),
                                                str::replace(&r_d, " /", ","));

                                            println!("{}", form_line);

                                            if paradigm_reader.read_line(&mut paradigm_line).unwrap() != 0 && idx != 76 && idx != 77 && idx != 78 && idx != 91 && idx != 95 && idx != 118 && idx != 119 && idx != 121 && idx != 122 && idx != 126 { 
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
