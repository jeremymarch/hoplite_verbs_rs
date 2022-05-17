#![allow(dead_code)]

extern crate rustunicodetests;
use rustunicodetests::*;
//use rustunicodetests::hgk_toggle_diacritic_str;
use rustunicodetests::hgk_strip_diacritics;
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
    NotImplemented,
    
    //NumEndings,
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
        if self.pps[0].ends_with("γίγνομαι") { //and παραγίγνομαι
            //From Hardy: "I guess γίγνομαι is technically a partial deponent, though in practice I don't think we're in the habit of calling it that.  We simply say that's a deponent (i.e. a middle deponent) with one active PP."
            HcDeponentType::GignomaiDeponent //see H&Q page 382. fix me, there may be a better way to do this without separate case
        }
        /*else if ( utf8HasSuffix(v->present, "μαι")) {
            return MIDDLE_DEPONENT;
        }*/
        else if self.pps[0].ends_with("μαι") && self.pps[1].ends_with("μαι") && self.pps[2].ends_with("μην") && self.pps[3] == "—" /* && utf8HasSuffix(v->perfmid, "μαι") */ && self.pps[5] == "—" {
            HcDeponentType::MiddleDeponent
        }
        //this gets μετανίσταμαι and ἐπανίσταμαι: middle deponents which happen to have an active perfect and root aorist
        else if self.pps[0].ends_with("μαι") && self.pps[1].ends_with("μαι") && self.pps[2].ends_with("ην") /* && utf8HasSuffix(v->perfmid, "μαι") */ && self.pps[5] == "—" {
            HcDeponentType::MiddleDeponent
        }
        else if self.pps[0].ends_with("μαι") && self.pps[1].ends_with("μαι") && self.pps[2] == "—" && self.pps[3] == "—" && self.pps[4].ends_with("μαι") && self.pps[5] != "—" {
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
    fn get_form(&self, decompose:bool) -> Result<Vec<Step>, &str>;
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
        let orig_syl = analyze_syllable_quantities(&form, self.person, self.number, self.mood);

        if form.contains("εει") {               // h&q p236
            form = form.replacen("εει", "ει", 1);
        }
        else if form.contains("εε") {
            form = form.replacen("εε", "ει", 1);
        }
        else if form.contains("εη") {
            form = form.replacen("εη", "η", 1);
        }
        else if form.contains("εῃ") {
            form = form.replacen("εῃ", "ῃ", 1);
        }
        else if form.contains("εοι") {
            form = form.replacen("εοι", "οι", 1);
        }
        else if form.contains("εου") {
            form = form.replacen("εου", "ου", 1);
        }
        else if form.contains("εο") {
            form = form.replacen("εο", "ου", 1);
        }
        else if form.contains("εω") {
            form = form.replacen("εω", "ω", 1);
        }

        else if form.contains("αει") {          // h&q p232
            form = form.replacen("αει", "ᾱͅ", 1);
        }
        else if form.contains("αε") {
            form = form.replacen("αε", "ᾱ", 1);
        }
        else if form.contains("αη") {
            form = form.replacen("αη", "ᾱ", 1);
        }
        else if form.contains("αῃ") {
            form = form.replacen("αῃ", "ᾱͅ", 1);
        }
        else if form.contains("αοι") {
            form = form.replacen("αοι", "ῳ", 1);
        }
        else if form.contains("αου") {
            form = form.replacen("αου", "ω", 1);
        }
        else if form.contains("αο") {
            form = form.replacen("αο", "ω", 1);
        }
        else if form.contains("αω") {
            form = form.replacen("αω", "ω", 1);
        }

        else if form.contains("οει") {          // h&q p264
            form = form.replacen("οει", "οι", 1);
        }
        else if form.contains("οε") {
            form = form.replacen("οε", "ου", 1);
        }
        else if form.contains("οη") {
            form = form.replacen("οη", "ω", 1);
        }
        else if form.contains("οῃ") {
            form = form.replacen("οῃ", "οι", 1);
        }
        else if form.contains("οοι") {
            form = form.replacen("οοι", "οι", 1);
        }
        else if form.contains("οου") {
            form = form.replacen("οου", "ου", 1);
        }
        else if form.contains("οο") {
            form = form.replacen("οο", "ου", 1);
        }
        else if form.contains("οω") {
            form = form.replacen("οω", "ω", 1);
        }

        self.accent_verb_contracted(&form, orig_syl, ending)

        //unaccented_form.to_string()
    }

    fn strip_ending(&self, pp_num:usize, form:String) -> Result<String, &str> {
        //println!("form: {}", form);
        match pp_num {
            1..=2 => {
                if form.ends_with('ω') {
                    if self.tense == HcTense::Future && self.voice != HcVoice::Passive && self.verb.pps[1].ends_with('ῶ') {
                        // contracted future
                        return Ok(format!("{}ε", form.strip_suffix('ω').unwrap()));
                    }
                    else {
                        return Ok(form.strip_suffix('ω').unwrap().to_string());
                    }
                }
                else if form.ends_with("ουμαι") && self.verb.pps[1].ends_with("οῦμαι") {
                    // contracted future
                    return Ok(format!("{}ε", form.strip_suffix("ουμαι").unwrap()));
                }
                else if form.ends_with("ομαι") {
                    return Ok(form.strip_suffix("ομαι").unwrap().to_string());
                }
                else if form.ends_with("μαι") {
                    return Ok(form.strip_suffix("μαι").unwrap().to_string());
                }
                else if form.ends_with("μι") {
                    return Ok(form.strip_suffix("μι").unwrap().to_string());
                }
                else if form.ends_with("στι(ν)") {
                    return Ok(form.strip_suffix("τι(ν)").unwrap().to_string());
                }
                else if form.ends_with("ται") {
                    return Ok(form.strip_suffix("ται").unwrap().to_string());
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
                else if form.ends_with("ν") {
                    return Ok(form.strip_suffix("ν").unwrap().to_string());
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

    fn is_deponent(&self, stem:&str) -> bool {   
        if (self.tense == HcTense::Present || self.tense == HcTense::Imperfect || self.tense == HcTense::Future) && stem.ends_with("μαι") {
            true
        }
        else if self.tense == HcTense::Aorist && self.voice != HcVoice::Passive && stem.ends_with("άμην") {
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
        if self.tense == HcTense::Imperfect && ( self.verb.pps[0].ends_with("άω") || self.verb.pps[0].ends_with("έω") || self.verb.pps[0].ends_with("όω") ) && self.person == HcPerson::Third && self.number == HcNumber::Singular && self.voice == HcVoice::Active {
            local_ending = local_ending.replacen("(ν)", "", 1);
        }

        if self.verb.pps[0].ends_with("μι") || self.verb.pps[0].ends_with("αμαι") {
            if self.tense == HcTense::Present || self.tense == HcTense::Imperfect {
                if self.number == HcNumber::Plural || self.mood != HcMood::Indicative || self.voice != HcVoice::Active {
                    if self.verb.pps[0].ends_with("ωμι") {
                        local_stem.pop();
                        local_stem.push_str("ο");
                    }
                    else if self.verb.pps[0].ends_with("στημι") {
                        local_stem.pop();
                        local_stem.push_str("α");
                    }
                    else if self.verb.pps[0].ends_with("τίθημι") || self.verb.pps[0].ends_with("ῑ̔́ημι") {
                        local_stem.pop();
                        local_stem.push_str("ε");
                    }
                    else if self.verb.pps[0].ends_with("ῡμι") {
                        local_stem = local_stem.replacen("ῡ", "υ", 1);
                    }
                }
            }

            if self.tense == HcTense::Present {
                if self.voice == HcVoice::Active {
                    if self.mood == HcMood::Subjunctive {
                        if !decompose {
                            if self.verb.pps[0].ends_with("ωμι") {
                                // didwmi subjunctive contraction
                                if local_ending.contains("ῇ") {
                                    local_ending = local_ending.replacen("ῇ", "ῷ", 1);
                                }
                                else if local_ending.contains("ῆ") {
                                    local_ending = local_ending.replacen("ῆ", "ῶ", 1);
                                }
                            }

                            if !self.verb.pps[0].ends_with("ῡμι") {
                                local_stem.pop();
                            }
                        }
                        else {
                            //isthmi subjunctive stem
                            if self.verb.pps[0].ends_with("στημι") {
                                local_stem.pop();
                                local_stem.push_str("ε");
                            }
                        }
                    }
                    else if self.mood == HcMood::Imperative {
                        if decompose {
                            if !(self.person == HcPerson::Second && self.number == HcNumber::Singular) {
                                local_ending.remove(0);
                            }
                            else if self.verb.pps[0].ends_with("ῡμι") { 
                                local_stem = local_stem.replacen("υ", "ῡ", 1); //fix me
                                local_ending = String::from(""); // fix me
                            }
                        }
                        else {
                            if self.person == HcPerson::Second && self.number == HcNumber::Singular {
                                if self.verb.pps[0].ends_with("ωμι") {
                                    local_ending = String::from("υ");
                                }
                                else if self.verb.pps[0].ends_with("στημι") { 
                                    local_stem.pop();
                                    local_ending = String::from("η");
                                }
                                else if self.verb.pps[0].ends_with("ῡμι") { 
                                    local_stem = local_stem.replacen("υ", "ῡ", 1);
                                    local_ending = String::from("");
                                }
                                else {
                                    local_ending = String::from("ι");
                                }
                            }
                            else {
                                local_ending.remove(0);
                            }
                        }
                    }
                    else if self.verb.pps[0].ends_with("στημι") && self.person == HcPerson::Third && self.number == HcNumber::Plural &&self.mood == HcMood::Indicative && !decompose {
                        local_stem.pop();
                        local_ending = local_ending.replacen("ᾱ", "ᾶ", 1);
                    }
                }
                else { // middle/passive
                    if self.mood == HcMood::Subjunctive {
                        if !decompose {
                            if !self.verb.pps[0].ends_with("ῡμι") { 
                                local_stem.pop();
                            }
                            if self.verb.pps[0].ends_with("ωμι") {
                                // didwmi subjunctive contraction
                                if local_ending.contains("ῃ") {
                                    local_ending = local_ending.replacen("ῃ", "ῷ", 1);
                                }
                                else if local_ending.contains("η") {
                                    local_ending = local_ending.replacen("η", "ῶ", 1);
                                }
                            }
    
                            if local_ending != "ωμεθα" && !self.verb.pps[0].ends_with("ῡμι") {
                                local_ending = self.accent_syllable_start(&local_ending, 0, HGK_CIRCUMFLEX );
                            }
                        }
                        else {
                            //isthmi subjunctive stem
                            if self.verb.pps[0].ends_with("στημι") || self.verb.pps[0].ends_with("αμαι") {
                                local_stem.pop();
                                local_stem.push_str("ε");
                            }
                        }
                    }
                    else if self.mood == HcMood::Optative {
                        if !decompose {
                            if local_ending.starts_with("ο") && !self.verb.pps[0].ends_with("ῡμι") { //alt endings for tithhmi and ihmi
                                local_stem.pop();
                            }
                        }
                    }    
                }
            }
            else if self.tense == HcTense::Imperfect {
                if self.verb.pps[0].ends_with("ωμι") {
                    if self.number == HcNumber::Singular {
                        if decompose {
                            local_stem = local_stem.replacen("ω", "ο", 1); //use short stem when using thematic endings
                            if self.person == HcPerson::First && self.voice == HcVoice::Active{
                                local_ending = local_ending.replacen("ν", "ον", 1);
                            }
                            else {
                                local_ending = local_ending.replacen("ς", "ες", 1);
                                if self.person == HcPerson::Third && self.voice == HcVoice::Active {
                                    local_ending = String::from("ε");
                                }
                            }
                        }
                        else {
                            local_stem = local_stem.replacen("ω", "ου", 1);
                        }
                    }
                }
                else if self.verb.pps[0].ends_with("τίθημι") {
                    if (self.person == HcPerson::Second || self.person == HcPerson::Third) && self.number == HcNumber::Singular {
                        if decompose {
                            local_stem = local_stem.replacen("η", "ε", 1); //use short stem when using thematic endings
                            local_ending = local_ending.replacen("ς", "ες", 1);
                            if self.person == HcPerson::Third && self.voice == HcVoice::Active {
                                local_ending = String::from("ε");
                            }
                        }
                        else {
                            local_stem = local_stem.replacen("η", "ει", 1);
                        }
                    }
                }
            }
            else if self.tense == HcTense::Aorist {
                //mixed aorist
                if local_stem.ends_with("κ") && (self.number == HcNumber::Plural || self.mood != HcMood::Indicative || self.voice != HcVoice::Active) {
                        
                    if self.verb.pps[0].ends_with("δίδωμι") {
                        local_stem = local_stem.replacen("ωκ", "ο", 1);
                    }
                    else if self.verb.pps[0].ends_with("τίθημι") {
                        local_stem = local_stem.replacen("ηκ", "ε", 1);
                    }

                    if self.mood == HcMood::Subjunctive && !decompose && self.voice != HcVoice::Passive {
                        local_stem.pop();
                    }

                    if self.voice == HcVoice::Active {
                        if self.mood != HcMood::Indicative {
                            if !decompose {
                                if self.mood == HcMood::Subjunctive {
                                    if self.verb.pps[0].ends_with("ωμι") {
                                        // didwmi subjunctive contraction
                                        if local_ending.contains("ῃ") {
                                            local_ending = local_ending.replacen("ῃ", "ῷ", 1);
                                        }
                                        else if local_ending.contains("η") {
                                            local_ending = local_ending.replacen("η", "ῶ", 1);
                                        }
                                    }
                                    
                                    local_ending = self.accent_syllable_start(&local_ending, 0,  HGK_CIRCUMFLEX );
                                }
                                if self.mood == HcMood::Imperative {
                                    // ana/thes
                                    if self.verb.pps[0].ends_with("ἀνατίθημι") && self.person == HcPerson::Second && self.number == HcNumber::Singular {
                                        local_stem = self.accent_syllable(&local_stem, 2, HGK_ACUTE );
                                    }// apo/dos
                                    else if self.verb.pps[0].ends_with("ἀποδίδωμι") && self.person == HcPerson::Second && self.number == HcNumber::Singular {
                                        local_stem = self.accent_syllable(&local_stem, 2, HGK_ACUTE );
                                    }
                                    else if self.verb.pps[0].ends_with("μεταδίδωμι") && self.person == HcPerson::Second && self.number == HcNumber::Singular {
                                        local_stem = self.accent_syllable(&local_stem, 2, HGK_ACUTE );
                                    }
                                    else if self.verb.pps[0].ends_with("παραδίδωμι") && self.person == HcPerson::Second && self.number == HcNumber::Singular {
                                        local_stem = self.accent_syllable(&local_stem, 2, HGK_ACUTE );
                                    }
                                }
                            }
                            if self.mood == HcMood::Optative {
                                local_ending.remove(0);
                            }
                        }
                    }
                    else if self.voice == HcVoice::Middle {
                        if self.mood == HcMood::Indicative {
                            local_ending.remove(0);
                            if self.person == HcPerson::Second && self.number == HcNumber::Singular {
                                if decompose {
                                    local_ending = String::from("ο");
                                }
                                else if local_stem.ends_with("ε") {
                                    local_stem = local_stem.rreplacen("ε", "ο", 1);
                                }
                            }
                        }
                        else if self.mood == HcMood::Subjunctive {
                            if self.verb.pps[0].ends_with("ωμι") && !decompose {
                                // didwmi subjunctive contraction
                                if local_ending.contains("ῃ") {
                                    local_ending = local_ending.replacen("ῃ", "ῷ", 1);
                                }
                                else if local_ending.contains("η") {
                                    local_ending = local_ending.replacen("η", "ῶ", 1);
                                }
                            }
                            if !decompose && local_ending != "ωμεθα" {
                                local_ending = self.accent_syllable_start(&local_ending, 0,  HGK_CIRCUMFLEX );
                            }
                        }
                        else if self.mood == HcMood::Optative {
                            if !decompose {
                                if local_ending.starts_with("ο") {
                                    local_stem.pop();
                                }
                            }
                        }
                        else if self.mood == HcMood::Imperative {
                            if self.person == HcPerson::Second && self.number == HcNumber::Singular {
                                if decompose {
                                    local_ending.remove(0);
                                }
                                else {
                                    local_stem.pop();
                                    local_ending = local_ending.replacen("σο", "ου", 1);
                                }
                            }
                        }
                    }
                }

            }
            else if self.tense == HcTense::Perfect {
                if self.number == HcNumber::Plural && local_stem.ends_with("στηκ") {
                    local_stem = local_stem.replacen("ηκ", "α", 1);
                    if self.person == HcPerson::Third {
                        if decompose {

                        }
                        else {
                            local_stem.pop();
                            local_ending = local_ending.replacen("ᾱ", "ᾶ", 1);
                        }
                    }
                    else {
                        local_ending.remove(0);
                    }
                }
            }
            else if self.tense == HcTense::Pluperfect {
                if self.number == HcNumber::Plural && local_stem.ends_with("στηκ") {
                    local_stem = local_stem.replacen("ηκ", "α", 1);
                    local_ending.remove(0);
                }
            }
        }

        // root aorist
        if (self.tense == HcTense::Aorist && self.voice == HcVoice::Active) && local_stem.ends_with("στη") || local_stem.ends_with("φθη") || local_stem.ends_with("βη") {
            println!("stem {}", local_stem);
            if self.mood == HcMood::Subjunctive {
                if decompose {
                    local_stem.pop();
                    local_stem.push_str("ε")
                }
                else {
                    local_stem.pop();
                }
            }
            else if self.mood == HcMood::Optative {
                local_stem.pop();
                local_stem.push_str("α") 
            }
            else if self.mood == HcMood::Imperative {
                if self.person == HcPerson::Second && self.number == HcNumber::Singular && local_stem.ends_with("φθη") {
                    local_ending = local_ending.replacen("θ", "τ", 1);
                }
                else if self.person == HcPerson::Third && self.number == HcNumber::Plural {
                    local_stem.pop();
                    local_stem.push_str("α");
                }
            }
        }

        // consonant stem perfects and pluperfects
        if ((self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect) && (self.voice == HcVoice::Middle || self.voice == HcVoice::Passive)) && local_stem == "πεπεμ" || local_stem == "ἐπεπεμ" || local_stem == format!("ε {} πεπεμ", SEPARATOR) {
            if local_ending.starts_with("ντ") {
                return Ok(String::from(BLANK));
            }
            else if decompose {
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
            else if decompose {
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
            else if decompose {
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
        else if ((self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect) && 
            (self.voice == HcVoice::Middle || self.voice == HcVoice::Passive)) && local_stem.ends_with('σ') {
            
            if local_ending.starts_with("ντ") {
                return Ok(String::from(BLANK));
            }
            else if local_ending.starts_with('σ') && !decompose {
                local_ending.remove(0);
            }
        }
        else if ((self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect) && 
            (self.voice == HcVoice::Middle || self.voice == HcVoice::Passive)) && local_stem.ends_with('λ') {
            
            if local_ending.starts_with("ντ") {
                return Ok(String::from(BLANK));
            }
            else if local_ending.starts_with('σ') && !decompose && self.number == HcNumber::Plural {
                local_ending.remove(0);
            }
        }

        let future_passive_suffix = if self.tense == HcTense::Future && self.voice == HcVoice::Passive {
            if decompose {
                format!("ησ {} ", SEPARATOR)
            }
            else {
                String::from("ησ")
            }
        }
        else {
            String::from("")
        };

        if decompose {
            Ok(format!("{} {} {}{}", local_stem, SEPARATOR, future_passive_suffix, local_ending))
        }
        else { //come take seek say find
            if local_stem == "λαβ" && local_ending == "ε" {
                local_ending = "έ".to_string();
            }
            else if local_stem == "ἐλθ" && local_ending == "ε" {
                local_ending = "έ".to_string();
            }
            else if local_stem == "ἰδ" && local_ending == "ε" {
                local_ending = "έ".to_string();
            }
            Ok(format!("{}{}{}", local_stem, future_passive_suffix, local_ending))
        }
    }

    fn add_augment(&self, stem:&str, decompose:bool) -> String {
        let mut local_stem = stem.to_string();
        if decompose {
            if local_stem.starts_with('ἠ') || local_stem.starts_with('ἡ') || local_stem.starts_with("εἰ") {
                local_stem
            }
            else if local_stem.starts_with("ἀπο") {        
                local_stem.replacen("ἀπο", format!("ἀπο {} ε {} ", SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("δια") {        
                local_stem.replacen("δια", format!("δια {} ε {} ", SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("συνε") {        
                local_stem.replacen("συνε", format!("συν {} ε", SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("συμ") {        
                local_stem.replacen("συμ", format!("συν {} ε {} ", SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("διενη") {        
                local_stem.replacen("διενη", format!("διενη"/* FIX ME */).as_str(), 1)
            }
            else if local_stem.starts_with("ὑπο") {        
                local_stem.replacen("ὑπο", format!("ὑπο {} ε {} ", SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("ὑπα") {        
                local_stem.replacen("ὑπα", format!("ὑπο {} ε {} α" /* FIX ME ἀ */, SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("ἀνα") {     
                local_stem.replacen("ἀνα", format!("ἀνα {} ε {} ", SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("παρα") {     
                local_stem.replacen("παρα", format!("παρα {} ε {} ", SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("ἐπι") {     
                local_stem.replacen("ἐπι", format!("ἐπι {} ε {} ", SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("κατα") {     
                local_stem.replacen("κατα", format!("κατα {} ε {} ", SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("μετανι") {     
                local_stem.replacen("μετανι", format!("μετα {} ανα {} ε {} ἱ", SEPARATOR, SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("ἐπανι") {     
                local_stem.replacen("ἐπανι", format!("ἐπι {} ανα {} ε {} ἱ", SEPARATOR, SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("ἐπανε") {    
                if self.number == HcNumber::Singular { 
                    local_stem.replacen("ἐπανε", format!("ἐπι {} ανα {} ε {} ἑ", SEPARATOR, SEPARATOR, SEPARATOR).as_str(), 1)
                } 
                else {
                    local_stem.replacen("ἐπανε", format!("ἐπι {} ανα {} ἑ", SEPARATOR, SEPARATOR).as_str(), 1)
                }
            }
            else if local_stem.starts_with("μετανε") {    
                if self.number == HcNumber::Singular { 
                    local_stem.replacen("μετανε", format!("μετα {} ανα {} ε {} ἑ", SEPARATOR, SEPARATOR, SEPARATOR).as_str(), 1)
                } 
                else {
                    local_stem.replacen("μετανε", format!("μετα {} ανα {} ἑ", SEPARATOR, SEPARATOR).as_str(), 1)
                }
            }
            else if local_stem.starts_with("μετα") {     
                local_stem.replacen("μετα", format!("μετα {} ε {} ", SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("ἀφι") {     
                local_stem.replacen("ἀφι", format!("ἀπο {} ε {} ἱ", SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("καθι") {     
                local_stem.replacen("καθι", format!("κατα {} ε {} ἱ", SEPARATOR, SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("ἀφε") {    
                if self.number == HcNumber::Singular /*|| self.voice != HcVoice::Active FIX ME */ {
                    local_stem.replacen("ἀφε", format!("ἀπο {} ε {} ἑ", SEPARATOR, SEPARATOR).as_str(), 1)
                }
                else {
                    local_stem.replacen("ἀφε", format!("ἀπο {} ἑ", SEPARATOR).as_str(), 1)
                }
            }
            else if local_stem.starts_with("καθε") {    
                if self.number == HcNumber::Singular /*|| self.voice != HcVoice::Active FIX ME */ {
                    local_stem.replacen("καθε", format!("κατα {} ε {} ἑ", SEPARATOR, SEPARATOR).as_str(), 1)
                }
                else {
                    local_stem.replacen("καθε", format!("κατα {} ἑ", SEPARATOR).as_str(), 1)
                }
            }
            else if local_stem.starts_with("ἑσ") { //isthmi
                if self.number == HcNumber::Singular /*|| self.voice != HcVoice::Active FIX ME */ {
                    local_stem.replacen("ἑσ", format!("ε {} ἑσ", SEPARATOR).as_str(), 1)
                }
                else {
                    local_stem
                }
            }
            else if local_stem.starts_with("ἑ") { //epomai
                local_stem.replacen("ἑ", format!("ε {} ἑ", SEPARATOR).as_str(), 1)
            }
            else if local_stem.starts_with("ἐ") {    
                if self.tense != HcTense::Pluperfect { 
                    local_stem.replacen("ἐ", format!("ε {} ἐ", SEPARATOR).as_str(), 1)
                }
                else {
                    local_stem
                }
            }
            else if local_stem.starts_with("ᾐ") {    
                local_stem
            }
            else if local_stem.starts_with("ᾑ") {    
                local_stem
            }
            else {
                format!("ε {} {}", SEPARATOR, local_stem)
            }
        }
        else {
            if local_stem.starts_with("ἀπο") {
                local_stem.replacen("ἀπο", "ἀπε", 1)
            }
            else if local_stem.starts_with("ὁ") {
                local_stem.replacen("ὁ", "ἑω", 1)
            }
            else if local_stem.starts_with("δια") {
                local_stem.replacen("δια", "διε", 1)
            }
            else if local_stem.starts_with("συνε") {
                local_stem
            }
            else if local_stem.starts_with("συμ") {
                local_stem.replacen("συμ", "συνε", 1)
            }
            else if local_stem.starts_with("διε") {
                local_stem.replacen("διε", "διε", 1)
            }
            else if local_stem.starts_with("ὑπο") {
                local_stem.replacen("ὑπο", "ὑπε", 1)
            }
            else if local_stem.starts_with("ὑπα") {
                local_stem.replacen("ὑπα", "ὑπη", 1)
            }
            else if local_stem.starts_with("ἀνα") {
                local_stem.replacen("ἀνα", "ἀνε", 1)
            }
            else if local_stem.starts_with("παρα") {
                local_stem.replacen("παρα", "παρε", 1)
            }
            else if local_stem.starts_with("ἐπι") {
                local_stem.replacen("ἐπι", "ἐπε", 1)
            }
            else if local_stem.starts_with("κατα") {
                local_stem.replacen("κατα", "κατε", 1)
            }
            else if local_stem.starts_with("μετανε") {
                if self.number == HcNumber::Singular || self.voice != HcVoice::Active {
                    local_stem.replacen("μετανε", "μετανει", 1)
                }
                else {
                    local_stem
                }
            }
            else if local_stem.starts_with("μετανι") {
                local_stem.replacen("μετανι", "μετανῑ", 1)
            }
            else if local_stem.starts_with("ἐπανε") {
                if self.number == HcNumber::Singular || self.voice != HcVoice::Active {
                    local_stem.replacen("ἐπανε", "ἐπανει", 1)
                }
                else {
                    local_stem
                }
            }
            else if local_stem.starts_with("ἐπανι") {
                local_stem.replacen("ἐπανι", "ἐπανῑ", 1)
            }
            else if local_stem.starts_with("μετα") {
                local_stem.replacen("μετα", "μετε", 1)
            }
            else if local_stem.starts_with("ἀφι") {
                local_stem.replacen("ἀφι", "ἀφῑ", 1)
            }
            else if local_stem.starts_with("καθι") {
                local_stem.replacen("καθι", "καθῑ", 1)
            }
            else if local_stem.starts_with("ἀφε") {
                if self.number == HcNumber::Singular || self.voice != HcVoice::Active {
                    local_stem.replacen("ἀφε", "ἀφει", 1)
                }
                else {
                    local_stem
                }
            }
            else if local_stem.starts_with("καθε") {
                if self.number == HcNumber::Singular || self.voice != HcVoice::Active {
                    local_stem.replacen("καθε", "καθει", 1)
                }
                else {
                    local_stem
                }
            }
            else if local_stem.starts_with("ᾐ") {
                local_stem
            }  
            else if local_stem.starts_with("ᾑ") {
                local_stem
            }   
            else if local_stem.starts_with("αἰ") {
                local_stem.replacen("αἰ", "ᾐ", 1)
            }   
            else if local_stem.starts_with("αἱ") {
                local_stem.replacen("αἱ", "ᾑ", 1)
            }  
            else if local_stem.starts_with("ἑο") {
                local_stem.replacen("ἑο", "ἑο", 1)
            }   
            else if local_stem.starts_with("ἑω") {
                local_stem.replacen("ἑω", "ἑω", 1)
            }   
            else if local_stem.starts_with("ὠ") {
                local_stem
            }   
            else if local_stem.starts_with("ἑ") {
                if self.number == HcNumber::Singular || self.voice != HcVoice::Active {
                    local_stem.replacen("ἑ", "εἱ", 1)
                }
                else {
                    local_stem
                }
            }
            else if local_stem.starts_with("εἰ") {
                local_stem
            }
            else if local_stem.starts_with("ἐ") {
                if self.tense != HcTense::Pluperfect {
                    local_stem.replacen("ἐ", "ἠ", 1)
                }
                else {
                    local_stem
                }
            }
            else if local_stem.starts_with("ἱ") {
                local_stem.replacen("ἱ", "ῑ̔", 1)
            }
            else if self.verb.pps[0].starts_with('ἐ') || self.verb.pps[0].starts_with('ἄ') || self.verb.pps[0].starts_with('ἀ') {
                local_stem.remove(0);
                format!("ἠ{}", local_stem)
            }
            else if local_stem.starts_with('ἠ') || local_stem.starts_with('ἡ') {
                local_stem
            }
            else {
                format!("ἐ{}", local_stem)
            }
        }
    }

    fn deaugment(&self, a:&str, decompose:bool) -> String {
        let mut loc = a.to_string();

        if decompose {
            if loc.starts_with("ἀπε") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("ἀπε", format!("ἀπο {} ε {} ", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("ἀπε", format!("ἀπο {} ", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("συνη") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("συνη", format!("συν {} ε {} ε", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("συνη", format!("συν {} ε", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("διη") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("διη", format!("δια {} ε {} ε", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("διη", format!("δια {} ε", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("διε") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("διε", format!("δια {} ε {} ε", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("διε", format!("δια {} ε", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("ὑπε") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("ὑπε", format!("ὑπο {} ε {} ", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("ὑπε", format!("ὑπο {} ", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("ὑπη") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("ὑπη", format!("ὑπο {} ε {} α"/* FIX ME ἀ */, SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("ὑπη", format!("ὑπο {} α"/* FIX ME ἀ */, SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("ἐπεδ") {  //because pempw and epideiknumi
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("ἐπε", format!("ἐπι {} ε {} ", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("ἐπε", format!("ἐπι {} ", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("ἀνε") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("ἀνε", format!("ἀνα {} ε {} ", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("ἀνε", format!("ἀνα {} ", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("παρε") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("παρε", format!("παρα {} ε {} ", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("παρε", format!("παρα {} ", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("κατε") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("κατε", format!("κατα {} ε {} ", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("κατε", format!("κατα {} ", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("μετανε") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("μετανε", format!("μετα {} ανα {} ε {} ", SEPARATOR, SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("μετανε", format!("μετα {} ανα {} ", SEPARATOR, SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("ἐπανε") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("ἐπανε", format!("ἐπι {} ανα {} ε {} ", SEPARATOR, SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("ἐπανε", format!("ἐπι {} ανα {} ", SEPARATOR, SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("μετε") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("μετε", format!("μετα {} ε {} ", SEPARATOR, SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("μετε", format!("μετα {} ", SEPARATOR).as_str(), 1);
                }
            }
            else if loc.starts_with("εἱ") && self.verb.pps[0].starts_with("αἱ") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("εἱ", format!("ε {} ἑ", SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("εἱ", "ἑ", 1);
                }
            }
            else if loc.starts_with('ὠ') {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("ὠ", format!("ε {} ὀ", SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("ὠ", "ὀ", 1);
                }
            }
            else if loc.starts_with('ᾐ') && self.verb.pps[0].starts_with("αἰ") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("ᾐ", format!("ε {} αἰ", SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("ᾐ", "αἰ", 1);
                }
            }
            else if loc.starts_with('ᾑ') && self.verb.pps[0].starts_with("αἱ") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("ᾑ", format!("ε {} αἱ", SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("ᾑ", "αἱ", 1);
                }
            }
            else if loc.starts_with('ἠ') && (self.verb.pps[0].starts_with('ἐ') || self.verb.pps[0].starts_with("φέρω")) {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("ἠ", format!("ε {} ἐ", SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("ἠ", "ἐ", 1);
                }
            }
            else if loc.starts_with('ἠ') && self.verb.pps[0].starts_with('ἔ') {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("ἠ", format!("ε {} ἐ", SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("ἠ", "ἐ", 1);
                }
            }
            else if loc.starts_with('ἠ') && (self.verb.pps[0].starts_with('ἄ') || self.verb.pps[0].starts_with('ἀ')) {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("ἠ", format!("ε {} ἀ", SEPARATOR).as_str(), 1);
                }
                else {
                    loc = loc.replacen("ἠ", "ἀ", 1);
                }
            }
            else {
                loc.remove(0);
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = format!("ε {} {}", SEPARATOR, loc);
                }
            }
            loc
        }
        else {
            if loc.starts_with("ἀπε") {
                loc = loc.replacen("ἀπε", "ἀπο", 1);
            }
            else if loc.starts_with("διη") {
                loc = loc.replacen("διη", "διε", 1);
            }
            else if loc.starts_with("συνη") {
                loc = loc.replacen("συνη", "συνε", 1);
            }
            else if loc.starts_with("ὑπε") {
                loc = loc.replacen("ὑπε", "ὑπο", 1);
            }
            else if loc.starts_with("ὑπη") {
                loc = loc.replacen("ὑπη", "ὑπα", 1);
            }
            else if loc.starts_with("ἐπεδ") { //because pempw and epideiknumi
                loc = loc.replacen("ἐπε", "ἐπι", 1);
            }
            else if loc.starts_with("ἀνε") {
                loc = loc.replacen("ἀνε", "ἀνα", 1);
            }
            else if loc.starts_with("παρε") {
                loc = loc.replacen("παρε", "παρα", 1);
            }
            else if loc.starts_with("κατε") {
                loc = loc.replacen("κατε", "κατα", 1);
            }
            else if loc.starts_with("μετανε") {
                loc = loc.replacen("μετανε", "μετανα", 1);
            }
            else if loc.starts_with("ἐπανε") {
                loc = loc.replacen("ἐπανε", "ἐπανα", 1);
            }
            else if loc.starts_with("μετε") {
                loc = loc.replacen("μετε", "μετα", 1);
            }
            else if loc.starts_with("εἱ") {
                loc = loc.replacen("εἱ", "ἑ", 1);
            }
            else if loc.starts_with("ᾐ") {
                loc = loc.replacen("ᾐ", "αἰ", 1);
            }
            else if loc.starts_with("ᾑ") {
                loc = loc.replacen("ᾑ", "αἱ", 1);
            }
            else if loc.starts_with("ὠ") {
                loc = loc.replacen("ὠ", "ὀ", 1);
            }
            else if loc.starts_with('ἠ') && (self.verb.pps[0].starts_with('ἐ') || self.verb.pps[0].starts_with("φέρω")) {
                loc.remove(0);
                loc = format!("ἐ{}", loc);
            }
            else if loc.starts_with('ἠ') && self.verb.pps[0].starts_with('ἔ') {
                loc.remove(0);
                loc = format!("ἐ{}", loc);
            }
            else if loc.starts_with('ἠ') && (self.verb.pps[0].starts_with('ἄ') || self.verb.pps[0].starts_with('ἀ')) {
                loc.remove(0);
                loc = format!("ἀ{}", loc);
            }
            else {
                loc.remove(0);
            }
            loc
        }
    }

    fn separate_prefix(&self, stem:&str) ->String {
        if stem.starts_with("ἀπο") {
            return stem.replacen("ἀπο", format!("ἀπο {} ", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("συμ") {
            return stem.replacen("συμ", format!("συν {} ", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("συν") {
            return stem.replacen("συν", format!("συν {} ", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("διο") { // διοίσω
            return stem.replacen("διο", format!("δια {} ο", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("δια") {
            return stem.replacen("δια", format!("δια {} ", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("ὑπο") {
            return stem.replacen("ὑπο", format!("ὑπο {} ", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("ὑπα") {
            return stem.replacen("ὑπα", format!("ὑπο {} α"/* FIX ME ἀ */, SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("ἀνα") {
            return stem.replacen("ἀνα", format!("ἀνα {} ", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("παρα") {
            return stem.replacen("παρα", format!("παρα {} ", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("ἐπι") {
            return stem.replacen("ἐπι", format!("ἐπι {} ", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("ἀφι") {
            return stem.replacen("ἀφι", format!("ἀπο {} ἱ", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("καθι") {
            return stem.replacen("καθι", format!("κατα {} ἱ", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("καθε") {
            return stem.replacen("καθε", format!("κατα {} ἑ", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("κατα") {
            return stem.replacen("κατα", format!("κατα {} ", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("μετανα") {
            return stem.replacen("μετανα", format!("μετα {} ανα {} ", SEPARATOR, SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("μετανι") {
            return stem.replacen("μετανι", format!("μετα {} ανα {} ἱ", SEPARATOR, SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("μετανε") {
            return stem.replacen("μετανε", format!("μετα {} ανα {} ἑ", SEPARATOR, SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("ἐπανα") {
            return stem.replacen("ἐπανα", format!("ἐπι {} ανα {} ", SEPARATOR, SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("ἐπανι") {
            return stem.replacen("ἐπανι", format!("ἐπι {} ανα {} ἱ", SEPARATOR, SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("ἐπανε") {
            return stem.replacen("ἐπανε", format!("ἐπι {} ανα {} ἑ", SEPARATOR, SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("μετα") {
            return stem.replacen("μετα", format!("μετα {} ", SEPARATOR).as_str(), 1);
        }
        else if stem.starts_with("ἀφε") {
            return stem.replacen("ἀφε", format!("ἀπο {} ἑ", SEPARATOR).as_str(), 1);
        }
        stem.to_string()
    }

    fn get_form(&self, decompose:bool) -> Result<Vec<Step>, &str> {
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

        if self.voice == HcVoice::Active && self.is_deponent(f) {
            steps.push(Step{form:String::from(""), explanation:String::from("Deponent")});
            return Ok(steps);
        }

        //block future passive for passive deponents
        if self.verb.deponent_type() == HcDeponentType::PassiveDeponent && self.tense == HcTense::Future && self.voice == HcVoice::Passive {
            steps.push(Step{form:String::from(""), explanation:String::from("Deponent")});
            return Ok(steps);
        }
        
        //abd
        //no passive for middle deponent present or imperfect
        //this does not need to be done for future, aorist because from different pp,
        if self.voice == HcVoice::Passive && (self.tense == HcTense::Present || self.tense == HcTense::Imperfect) && self.verb.pps[0].ends_with("μαι")
        {
            steps.push(Step{form:String::from(""), explanation:String::from("Deponent")});
            return Ok(steps);
        }
        
        //for perfect and pluperfect we need to block passive if middle or passive deponent
        if self.voice == HcVoice::Passive && (self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect) && (self.verb.deponent_type() == HcDeponentType::MiddleDeponent || self.verb.deponent_type() == HcDeponentType::PassiveDeponent || self.verb.deponent_type() == HcDeponentType::MiddleDeponentHgeomai)
        {
            steps.push(Step{form:String::from(""), explanation:String::from("Deponent")});
            return Ok(steps);
        }
        
        //middle deponents do not have a passive voice.  H&Q page 316
        if self.voice == HcVoice::Passive && (self.verb.deponent_type() == HcDeponentType::MiddleDeponent || self.verb.deponent_type() == HcDeponentType::GignomaiDeponent) {
            steps.push(Step{form:String::from(""), explanation:String::from("Deponent")});
            return Ok(steps);
        }

        if self.voice == HcVoice::Active && (self.verb.deponent_type() == HcDeponentType::MiddleDeponent || self.verb.deponent_type() == HcDeponentType::PassiveDeponent)  && !self.verb.pps[2].ends_with("στην") {
            steps.push(Step{form:String::from(""), explanation:String::from("Deponent")});
            return Ok(steps);
        }

        if self.voice == HcVoice::Active && self.tense != HcTense::Perfect && self.tense != HcTense::Pluperfect && self.verb.deponent_type() == HcDeponentType::GignomaiDeponent {
            steps.push(Step{form:String::from(""), explanation:String::from("Deponent")});
            return Ok(steps);
        }
        /* 
        if (vf->voice == PASSIVE && deponentType(vf->verb) == PASSIVE_DEPONENT && (vf->tense == PRESENT || vf->tense == IMPERFECT || vf->tense == PERFECT || vf->tense == PLUPERFECT)) //aorist or future are ok
        {
            return 0;
        }
        */

    
        
        //let mut pps_without_ending = Vec::new();
        //strip accent: internally (not as a step)
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
                return Err("Illegal form ending");
            }
            for e in endings_for_form.unwrap() {

                let a = self.strip_ending(pp_num, a.to_string());
                if a.is_err() {
                    return Err("error stripping ending");
                }
                let a = a.unwrap();

                // let f = a.join(" / ");
                // let e = "Remove ending from Principal Part".to_string();
                // steps.push(Step{form:f, explanation:e});

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

                if (a.ends_with("στη")  || a.ends_with("φθη") || a.ends_with("βη")) && self.tense == HcTense::Aorist && self.voice == HcVoice::Middle {
                    continue;
                }

                if self.verb.pps[0].starts_with("βλάπτω") && a == "βλαφθ" && self.tense == HcTense::Future && self.voice == HcVoice::Passive {
                    continue;
                }

                //skip alternate here because same, could remove this now that we're removing duplicates later?
                if self.verb.pps[0].starts_with("σῴζω") && ((a.ends_with("σεσω") && self.person == HcPerson::Second) || (a.ends_with("σεσωσ") && self.person == HcPerson::Third && self.number == HcNumber::Plural)) {
                    continue;
                }
                
                let ending = if decompose { hgk_strip_diacritics(e, HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE) } else { e.to_string() };
                let stem = if decompose && self.tense == HcTense::Aorist && self.voice == HcVoice::Passive && self.mood == HcMood::Subjunctive { format!("{}ε", a.to_owned()) } else { a.to_owned() };
                let y = self.add_ending(&stem, &ending, decompose);
                let y = match y {
                    Ok(y) => y,
                    _ => return Err("Error adding ending")
                };

                if decompose && self.tense != HcTense::Imperfect && self.tense != HcTense::Pluperfect && self.tense != HcTense::Aorist && !(self.tense == HcTense::Future && self.voice == HcVoice::Passive) {
                    add_ending_collector.push( self.separate_prefix(&y) );
                }
                else {
                    add_ending_collector.push(y.to_string());
                }
                
                if !decompose {
                    let accented_form = if !hgk_has_diacritics(&y, HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE) { self.accent_verb(&y) } else { y };
                    if ((self.tense == HcTense::Imperfect || self.tense == HcTense::Present) && 
                        ( self.verb.pps[0].ends_with("άω") || self.verb.pps[0].ends_with("έω") || self.verb.pps[0].ends_with("όω") || self.verb.pps[0].ends_with("άομαι") || self.verb.pps[0].ends_with("έομαι") || self.verb.pps[0].ends_with("όομαι") )) || 
                        (self.tense == HcTense::Future && self.voice != HcVoice::Passive && (self.verb.pps[1].ends_with('ῶ') || self.verb.pps[1].ends_with("οῦμαι"))) {

                        add_accent_collector.push( self.contract_verb(&accented_form, e) );
                    }
                    else {
                        add_accent_collector.push( accented_form );
                    }
                }
                //println!("z1 {:?}", z1);
                //imperfect/pluperfect: add augment
                //aorist subj/opt/imper/inf/ptc: remove augment
                //contract contracted verbs
                //accent
            }
        }
        //let v: Vec<_> = v.into_iter().unique().collect();

        let f = add_ending_collector.join(" / ");
        let e = "Add ending".to_string();
        steps.push(Step{form:f, explanation:e});   
        
        if !decompose {
            //remove duplicate and then join alternates with /
            let f = add_accent_collector.into_iter().unique().collect::<Vec<String>>().join(" / ");
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
        let syl = analyze_syllable_quantities(word, self.person, self.number, self.mood);

        let esyl = analyze_syllable_quantities(ending, self.person, self.number, self.mood);
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
            if (orig_syllables[1].letters == "αι" || orig_syllables[1].letters == "οι") && self.mood == HcMood::Optative {
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
                            HcMood::Indicative => if self.verb.pps[0].ends_with("μι") { HcEndings::PresentActiveIndicativeMi } else { HcEndings::PresentActiveInd },
                            HcMood::Subjunctive => if self.verb.pps[0].ends_with("μι") && !self.verb.pps[0].ends_with("ῡμι") { HcEndings::AoristPassiveSubj } else { HcEndings::PresentActiveSubj },
                            HcMood::Optative => if self.verb.pps[0].ends_with("μι") && !self.verb.pps[0].ends_with("ῡμι") { HcEndings::PresentActiveOptMi } else { if self.verb.pps[0].ends_with("άω") || self.verb.pps[0].ends_with("έω") || self.verb.pps[0].ends_with("όω") { HcEndings::PresentActiveOptEContracted} else { HcEndings::PresentActiveOpt } },
                            HcMood::Imperative => HcEndings::PresentActiveImperative,
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    },
                    HcVoice::Middle | HcVoice::Passive => {
                        match self.mood {
                            HcMood::Indicative => if self.verb.pps[0].ends_with("μι") || self.verb.pps[0].ends_with("υμαι") || self.verb.pps[0].ends_with("αμαι")  { HcEndings::PerfectMidpassInd } else { HcEndings::PresentMidpassInd },
                            HcMood::Subjunctive => HcEndings::PresentMidpassSubj,
                            HcMood::Optative => if self.verb.pps[0].ends_with("ημι") && !self.verb.pps[0].ends_with("στημι") { HcEndings::PresentMidpassOptTithhmi } else if (self.verb.pps[0].ends_with("μι") && !self.verb.pps[0].ends_with("ῡμι")) || self.verb.pps[0].ends_with("αμαι") { HcEndings::MiddleOptMi } else { HcEndings::PresentMidpassOpt },
                            HcMood::Imperative => if self.verb.pps[0].ends_with("μι") || self.verb.pps[0].ends_with("υμαι") || self.verb.pps[0].ends_with("αμαι") { HcEndings::PresentMidpassImperativeMi } else { HcEndings::PresentMidpassImperative },
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
                            HcMood::Indicative => if self.verb.pps[0].ends_with("μι") { HcEndings::ImperfectActiveMi } else { HcEndings::ImperfectActiveInd },
                            HcMood::Subjunctive => HcEndings::NotImplemented,
                            HcMood::Optative => HcEndings::NotImplemented,
                            HcMood::Imperative => HcEndings::NotImplemented,
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    },
                    HcVoice::Middle | HcVoice::Passive => {
                        match self.mood {
                            HcMood::Indicative => if self.verb.pps[0].ends_with("μι") || self.verb.pps[0].ends_with("υμαι") || self.verb.pps[0].ends_with("αμαι") { HcEndings::PluperfectMidpassInd } else { HcEndings::ImperfectMidpassInd },
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
                            HcMood::Optative => if self.verb.pps[1].ends_with('ῶ') { HcEndings::PresentActiveOptEContracted} else { HcEndings::PresentActiveOpt },
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
                        if stem.ends_with("ον") {
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
                                
                                HcMood::Indicative => if stem.ends_with("στην") || stem.ends_with("φθην") || stem.ends_with("βην") { 
                                        HcEndings::AoristActiveIndicativeMiRoot } 
                                    else if self.verb.pps[0].ends_with("μι") && stem.ends_with("κα") {
                                        HcEndings::MixedAoristMi } 
                                    else { HcEndings::AoristActiveInd },
                                HcMood::Subjunctive => if stem.ends_with("στην") || stem.ends_with("φθην") || stem.ends_with("βην") { HcEndings::AoristPassiveSubj } else { HcEndings::PresentActiveSubj },
                                HcMood::Optative => if stem.ends_with("στην") || stem.ends_with("φθην") || stem.ends_with("βην") { HcEndings::PresentActiveOptMi } else if self.verb.pps[0].ends_with("μι") && stem.ends_with("κα") { HcEndings::AoristPassiveOpt } else { HcEndings::AoristActiveOpt },
                                HcMood::Imperative => if stem.ends_with("στην") || stem.ends_with("φθην") || stem.ends_with("βην") { HcEndings::AoristActiveImperativesMiRoot } else if self.verb.pps[0].ends_with("μι") && stem.ends_with("κα") { HcEndings::AoristActiveImperativesMi } else { HcEndings::AoristActiveImperative },
                                HcMood::Infinitive => HcEndings::NotImplemented,
                                HcMood::Participle => HcEndings::NotImplemented,
                            }                            
                        }
                    },
                    HcVoice::Middle => {
                        if stem.ends_with("ον") || stem.ends_with("ομην") {
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
                                HcMood::Indicative => if self.verb.pps[0].ends_with("μι") && stem.ends_with("κα") { HcEndings::ImperfectMidpassInd } else { HcEndings::AoristMidInd },
                                HcMood::Subjunctive => HcEndings::PresentMidpassSubj,
                                HcMood::Optative => if self.verb.pps[0].ends_with("ημι") && stem.ends_with("κα") { HcEndings::PresentMidpassOptTithhmi } else if self.verb.pps[0].ends_with("μι") && stem.ends_with("κα") { HcEndings::MiddleOptMi } else { HcEndings::AoristMiddleOpt },
                                HcMood::Imperative => if self.verb.pps[0].ends_with("μι") && stem.ends_with("κα") { HcEndings::PresentMidpassImperativeMi } else { HcEndings::AoristMiddleImperative },
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

static ENDINGS: &[[&str; 6]; 37] = &[
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
    
    ["οιμι,οιην", "οις,οιης", "οι,οιη", "οιμεν,οιημεν", "οιτε,οιητε", "οιεν,οιησαν"],//, "" },//pres act opt e
    
    ["μι", "ς", "σι(ν)", "μεν", "τε", "ᾱσι(ν)"],//, "" },   //mi
    
    ["", "ς", "τω", "", "τε", "ντων"],//, "" },//mi aorist active imperatives
    ["", "θι", "τω", "", "τε", "ντων"],//", "" },//mi root aorist active imperatives
    
    ["", "ο", "σθω", "", "σθε", "σθων"],//, "Root Aorist Middle Imperative" },//mi root aorist middle imperatives
    ["ν", "ς", "", "μεν", "τε", "σαν"],//, "Root Aorist Indicative" },//mi root aorist indicative
    
    ["", "οῦ", "εσθω", "", "εσθε", "εσθων"],//, "Present Middle/Passive Imperative" }, //second aorist middle/passive imperatives
    ["ιμην", "ῖο", "ῖτο,οῖτο", "ιμεθα,οιμεθα", "ῖσθε,οῖσθε", "ῖντο,οῖντο"],//, "Present Middle/Passive Optative Tithemi" }, //Exception: H&Q page 347
    //["ον", "ες", "ε", "ομεν", "ετε", "ον"],//***, "Imperfect Active Indicative" } //this is only for contracted verbs when decompose so the nu moveable doesn't show up
    ["", "σο", "σθω", "", "σθε", "σθων"],
    ["ν", "ς", "", "μεν", "τε", "σαν"],
    ["α", "ας", "ε(ν)", "μεν", "τε", "σαν"],
    ["ιμην", "ῖο", "ῖτο", "ιμεθα", "ῖσθε", "ῖντο"],
    ["ιην", "ιης", "ιη", "ῖμεν,ιημεν", "ῖτε,ιητε", "ῖεν,ιησαν"],//, "Aorist Passive Optative" },
    ];

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    //use unicode_normalization::UnicodeNormalization;

    #[test]
    fn test_rreplacen() {
        let s = "f2o f2o 123 foo".to_string();
        assert_eq!("f2o f2o 1new3 foo", s.rreplacen("2", "new", 1));
        assert_eq!("f2o fnewo 1new3 foo", s.rreplacen("2", "new", 2));
    }

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
        assert_eq!(b.get_endings("").unwrap()[0], "ω");

        let b = HcGreekVerbForm {verb:&a, person:HcPerson::First, number:HcNumber::Singular, tense:HcTense::Present, voice:HcVoice::Middle, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_form(false).unwrap()[3].form, "βλαπτομαι");
        let b = HcGreekVerbForm {verb:&a, person:HcPerson::Second, number:HcNumber::Singular, tense:HcTense::Present, voice:HcVoice::Middle, mood:HcMood::Indicative, gender:None, case:None};
        assert_eq!(b.get_endings("").unwrap()[0], "ει");
        assert_eq!(b.get_endings("").unwrap()[1], "ῃ");
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

                        let properties = if line.starts_with("θάπτω") || line.starts_with("κλέπτω") || line.starts_with("λείπω") || line.starts_with("ὁράω") {
                            CONSONANT_STEM_PERFECT_PI
                        }
                        else if line.starts_with("τάττω") || line.starts_with("πρᾱ́ττω") || line.starts_with("ἄγω") {
                            CONSONANT_STEM_PERFECT_GAMMA
                        }
                        else if line.starts_with("ἄρχω") || line.starts_with("ἀποδέχομαι") || line.starts_with("δέχομαι") {
                            CONSONANT_STEM_PERFECT_CHI
                        }
                        else if line.starts_with("βλάπτω") || line.starts_with("λαμβάνω") {
                            CONSONANT_STEM_PERFECT_BETA
                        }
                        else if line.starts_with("ἀγγέλλω") {
                            CONSONANT_STEM_PERFECT_LAMBDA
                        }
                        else {
                            REGULAR
                        };

                        let verb = HcGreekVerb::from_string(idx as u32, &line, properties).unwrap();

                        if paradigm_reader.read_line(&mut paradigm_line).unwrap() == 0 { return; }
                        paradigm_line.clear();

                        let partial = if verb.deponent_type() == HcDeponentType::PartialDeponent { " (Partial Deponent)" } 
                            else if verb.deponent_type() == HcDeponentType::MiddleDeponent { " (Middle Deponent)"} 
                            else if verb.deponent_type() == HcDeponentType::PassiveDeponent { " (Passive Deponent)"} 
                            else if verb.deponent_type() == HcDeponentType::GignomaiDeponent { " (Deponent gignomai)"} 
                            else { "" };
     
                        let verb_section = format!("Verb {}. {}{}", idx, verb.pps[0], partial);
                        println!("\n{}", verb_section);
                        if paradigm_reader.read_line(&mut paradigm_line).unwrap() != 0 && idx != 76 && idx != 77 && idx != 78 { 
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
                                    //if m == HcMood::Imperative { section = section.replacen(" (Middle/Passive)", "", 1)};
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

                                            let form_line = format!("{}{}: {} ; {}", y.value(), z.value(), 
                                                str::replace(&r, " /", ","),
                                                str::replace(&r_d, " /", ","));

                                            println!("{}", form_line);

                                            if paradigm_reader.read_line(&mut paradigm_line).unwrap() != 0 && idx != 76 && idx != 77 && idx != 78 { 
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
