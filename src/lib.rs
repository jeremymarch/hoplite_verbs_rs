#![allow(dead_code)]
#![allow(clippy::if_same_then_else)] //for clarity let's leave these
#![allow(clippy::collapsible_if)]

// consonant stem infinitives
// υι diphthong accents circumflex (perf act nom, acc sing, nom pl.)
// fut. pass. ησ

pub extern crate polytonic_greek;
use polytonic_greek::*;
use std::sync::Arc;

use itertools::Itertools;
use rand::prelude::SliceRandom;
use std::collections::HashSet;
use tracing::error;

//mod latin;
mod special_verbs;

#[derive(Debug)]
pub struct Diagnostics {
    pub dash: usize,
    pub middle_passive: usize,
    pub blocked_for_unit: usize,
    pub filtered: usize,
    pub illegal: usize,
}

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

        let matches: Vec<(usize, &str)> = self.rmatch_indices(pat).take(count).collect();
        for (start, part) in matches.into_iter().rev() {
            //println!("start {}, part {}", start, part);
            result.push_str(unsafe { self.get_unchecked(last_end..start) });
            result.push_str(to);
            last_end = start + part.len();
        }
        result.push_str(unsafe { self.get_unchecked(last_end..self.len()) });
        result
    }
}

#[derive(Debug)]
pub struct VerbParameters {
    pub persons: Vec<HcPerson>,
    pub numbers: Vec<HcNumber>,
    pub tenses: Vec<HcTense>,
    pub voices: Vec<HcVoice>,
    pub moods: Vec<HcMood>,
}

impl VerbParameters {
    pub fn from_option(param_str: Option<String>) -> VerbParameters {
        match param_str {
            Some(s) => {
                let mut persons: Vec<HcPerson> = vec![];
                let mut numbers: Vec<HcNumber> = vec![];
                let mut tenses: Vec<HcTense> = vec![];
                let mut voices: Vec<HcVoice> = vec![];
                let mut moods: Vec<HcMood> = vec![];

                let param_vec: Vec<u32> = s
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty() && s.parse::<u32>().is_ok())
                    .map(|s| s.parse().unwrap_or(0))
                    .unique()
                    .collect();

                for p in param_vec {
                    match p {
                        1 => persons.push(HcPerson::First),
                        2 => persons.push(HcPerson::Second),
                        3 => persons.push(HcPerson::Third),
                        4 => numbers.push(HcNumber::Singular),
                        5 => numbers.push(HcNumber::Plural),
                        6 => tenses.push(HcTense::Present),
                        7 => tenses.push(HcTense::Imperfect),
                        8 => tenses.push(HcTense::Future),
                        9 => tenses.push(HcTense::Aorist),
                        10 => tenses.push(HcTense::Perfect),
                        11 => tenses.push(HcTense::Pluperfect),
                        12 => moods.push(HcMood::Indicative),
                        13 => moods.push(HcMood::Subjunctive),
                        14 => moods.push(HcMood::Optative),
                        15 => moods.push(HcMood::Imperative),
                        16 => voices.push(HcVoice::Active),
                        17 => voices.push(HcVoice::Middle),
                        18 => voices.push(HcVoice::Passive),
                        _ => (),
                    }
                }
                if persons.is_empty() {
                    persons.push(HcPerson::First);
                    persons.push(HcPerson::Second);
                    persons.push(HcPerson::Third);
                }
                if numbers.is_empty() {
                    numbers.push(HcNumber::Singular);
                    numbers.push(HcNumber::Plural);
                }
                if tenses.is_empty() {
                    tenses.push(HcTense::Present);
                    tenses.push(HcTense::Imperfect);
                    tenses.push(HcTense::Future);
                    tenses.push(HcTense::Aorist);
                    tenses.push(HcTense::Perfect);
                    tenses.push(HcTense::Pluperfect);
                }
                if voices.is_empty() {
                    voices.push(HcVoice::Active);
                    voices.push(HcVoice::Middle);
                    voices.push(HcVoice::Passive);
                }
                if moods.is_empty() {
                    moods.push(HcMood::Indicative);
                    moods.push(HcMood::Subjunctive);
                    moods.push(HcMood::Optative);
                    moods.push(HcMood::Imperative);
                }
                VerbParameters {
                    persons,
                    numbers,
                    tenses,
                    voices,
                    moods,
                }
            }
            None => VerbParameters {
                persons: vec![HcPerson::First, HcPerson::Second, HcPerson::Third],
                numbers: vec![HcNumber::Singular, HcNumber::Plural],
                tenses: vec![
                    HcTense::Present,
                    HcTense::Imperfect,
                    HcTense::Future,
                    HcTense::Aorist,
                    HcTense::Perfect,
                    HcTense::Pluperfect,
                ],
                voices: vec![HcVoice::Active, HcVoice::Middle, HcVoice::Passive],
                moods: vec![
                    HcMood::Indicative,
                    HcMood::Subjunctive,
                    HcMood::Optative,
                    HcMood::Imperative,
                ],
            },
        }
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

impl HcFormError {
    fn value(&self) -> &str {
        match *self {
            HcFormError::InternalError => "InternalError",
            HcFormError::BlankPrincipalPartForForm => "BlankPrincipalPart",
            HcFormError::UnexpectedPrincipalPartEnding => "InvalidPrincipalPart",
            HcFormError::Deponent => "DeponentNoFormForVoice",
            HcFormError::IllegalForm => "IllegalForm",
            HcFormError::DoesNotExist => "DoesNotExist",
            HcFormError::NotAvailableInUnit => "NoFormForUnit",
            HcFormError::NotImplemented => "NotImplemented",
        }
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
    AoristOptativeEchw,
    NotImplemented,
    //NumEndings,
}

#[derive(PartialEq)]
pub enum HcParameters {
    Person,
    Number,
    Tense,
    Mood,
    Voice,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum HcPerson {
    First,
    Second,
    Third,
}

impl HcPerson {
    pub fn value(&self) -> &str {
        match *self {
            HcPerson::First => "1",
            HcPerson::Second => "2",
            HcPerson::Third => "3",
        }
    }
    pub fn from_i16(value: i16) -> HcPerson {
        match value {
            0 => HcPerson::First,
            1 => HcPerson::Second,
            2 => HcPerson::Third,
            _ => panic!("Unknown value: {}", value),
        }
    }
    pub fn to_i16(&self) -> i16 {
        match *self {
            HcPerson::First => 0,
            HcPerson::Second => 1,
            HcPerson::Third => 2,
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
    pub fn value(&self) -> &str {
        match *self {
            HcNumber::Singular => "s",
            HcNumber::Dual => "d",
            HcNumber::Plural => "p",
        }
    }
    pub fn from_i16(value: i16) -> HcNumber {
        match value {
            0 => HcNumber::Singular,
            1 => HcNumber::Plural,
            _ => panic!("Unknown value: {}", value),
        }
    }
    pub fn to_i16(&self) -> i16 {
        match *self {
            HcNumber::Singular => 0,
            HcNumber::Dual => 2,
            HcNumber::Plural => 1,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum HcTense {
    Present,
    Imperfect,
    Future,
    Aorist,
    Perfect,
    Pluperfect,
}

impl HcTense {
    pub fn value(&self) -> &str {
        match *self {
            HcTense::Present => "Present",
            HcTense::Imperfect => "Imperfect",
            HcTense::Future => "Future",
            HcTense::Aorist => "Aorist",
            HcTense::Perfect => "Perfect",
            HcTense::Pluperfect => "Pluperfect",
        }
    }
    pub fn from_i16(value: i16) -> HcTense {
        match value {
            0 => HcTense::Present,
            1 => HcTense::Imperfect,
            2 => HcTense::Future,
            3 => HcTense::Aorist,
            4 => HcTense::Perfect,
            5 => HcTense::Pluperfect,
            _ => panic!("Unknown value: {}", value),
        }
    }
    pub fn to_i16(&self) -> i16 {
        match *self {
            HcTense::Present => 0,
            HcTense::Imperfect => 1,
            HcTense::Future => 2,
            HcTense::Aorist => 3,
            HcTense::Perfect => 4,
            HcTense::Pluperfect => 5,
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
    pub fn value(&self) -> &str {
        match *self {
            HcVoice::Active => "Active",
            HcVoice::Middle => "Middle",
            HcVoice::Passive => "Passive",
        }
    }
    pub fn from_i16(value: i16) -> HcVoice {
        match value {
            0 => HcVoice::Active,
            1 => HcVoice::Middle,
            2 => HcVoice::Passive,
            _ => panic!("Unknown value: {}", value),
        }
    }
    pub fn to_i16(&self) -> i16 {
        match *self {
            HcVoice::Active => 0,
            HcVoice::Middle => 1,
            HcVoice::Passive => 2,
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
    pub fn value(&self) -> &str {
        match *self {
            HcMood::Indicative => "Indicative",
            HcMood::Subjunctive => "Subjunctive",
            HcMood::Optative => "Optative",
            HcMood::Imperative => "Imperative",
            HcMood::Infinitive => "Infinitive",
            HcMood::Participle => "Participle",
        }
    }
    pub fn from_i16(value: i16) -> HcMood {
        match value {
            0 => HcMood::Indicative,
            1 => HcMood::Subjunctive,
            2 => HcMood::Optative,
            3 => HcMood::Imperative,
            4 => HcMood::Infinitive,
            5 => HcMood::Participle,
            _ => panic!("Unknown value: {}", value),
        }
    }
    pub fn to_i16(&self) -> i16 {
        match *self {
            HcMood::Indicative => 0,
            HcMood::Subjunctive => 1,
            HcMood::Optative => 2,
            HcMood::Imperative => 3,
            HcMood::Infinitive => 4,
            HcMood::Participle => 5,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum HcGender {
    Masculine,
    Feminine,
    Neuter,
}
impl HcGender {
    pub fn value(&self) -> &str {
        match *self {
            HcGender::Masculine => "Masculine",
            HcGender::Feminine => "Feminine",
            HcGender::Neuter => "Neuter",
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum HcCase {
    Nominative,
    Genitive,
    Dative,
    Accusative,
    Vocative,
}

impl HcCase {
    pub fn value(&self) -> &str {
        match *self {
            HcCase::Nominative => "Nominative",
            HcCase::Genitive => "Genitive",
            HcCase::Dative => "Dative",
            HcCase::Accusative => "Accusative",
            HcCase::Vocative => "Vocative",
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum HcGreekPrincipalParts {
    First = 1,
    Second = 2,
    Third = 3,
    Fourth = 4,
    Fifth = 5,
    Sixth = 6,
}

#[derive(PartialEq, Debug)]
pub enum HcDeponentType {
    NotDeponent,
    MiddleDeponent,
    PassiveDeponent,
    PartialDeponent,
    GignomaiDeponent,
    MiddleDeponentHgeomai,
}

impl HcDeponentType {
    pub fn value(&self) -> &str {
        match *self {
            HcDeponentType::NotDeponent => "Not Deponent",
            HcDeponentType::MiddleDeponent => "Middle Deponent",
            HcDeponentType::PassiveDeponent => "Passive Deponent",
            HcDeponentType::PartialDeponent => "Partial Deponent",
            HcDeponentType::GignomaiDeponent => "Deponent gignomai",
            HcDeponentType::MiddleDeponentHgeomai => "Middle Deponent with 6th pp",
        }
    }
}

pub const REGULAR: u32 = 0x0000;
pub const CONSONANT_STEM_PERFECT_PHI: u32 = 0x0001;
pub const CONSONANT_STEM_PERFECT_MU_PI: u32 = 0x0002;
pub const CONSONANT_STEM_PERFECT_KAPPA: u32 = 0x0004;
pub const CONSONANT_STEM_PERFECT_SIGMA: u32 = 0x0008;
pub const CONSONANT_STEM_PERFECT_SIGMA_2: u32 = 0x0010;
pub const CONSONANT_STEM_PERFECT_LAMBDA: u32 = 0x0020;
pub const CONSONANT_STEM_PERFECT_PI: u32 = 0x0040;
pub const CONSONANT_STEM_PERFECT_BETA: u32 = 0x0080;
pub const CONSONANT_STEM_PERFECT_GAMMA: u32 = 0x0100;
pub const CONSONANT_STEM_PERFECT_CHI: u32 = 0x0200;
pub const PREFIXED: u32 = 0x0400;
pub const CONTRACTED_FUTURE_ALPHA: u32 = 0x0800;
pub const CONSONANT_STEM_PERFECT_NU: u32 = 0x1000;
pub const MI_VERB: u32 = 0x2000;

#[derive(Eq, PartialEq, Debug)]
pub struct HcGreekVerb {
    pub id: u32,
    pub pps: Vec<String>,
    pub properties: u32,
    pub hq_unit: u32,
}

impl HcGreekVerb {
    pub fn get_verb_lemma(&self) -> String {
        if self.pps[0] == *"—" {
            format!("—, {}", self.pps[1]) //display 2nd pp if 1st is blank
        } else {
            self.pps[0].to_string()
        }
    }

    pub fn from_string(id: u32, pps: &str, props: u32, hq_unit: u32) -> Option<HcGreekVerb> {
        let x: Vec<String> = pps.split(',').map(|s| s.trim().to_owned()).collect();
        if x.len() == 6 {
            Some(HcGreekVerb {
                id,
                pps: x,
                properties: props,
                hq_unit,
            })
        } else {
            None
        }
    }

    pub fn from_string_with_properties(id: u32, ppstring: &str) -> Option<HcGreekVerb> {
        let mut properties = 0;
        let mut ll = ppstring.split('%');
        let mut hq_unit = 0;

        if let Some(pps) = ll.next() {
            if let Some(s) = ll.next() {
                hq_unit = s.trim().parse::<u32>().unwrap();
            }

            if let Some(s) = ll.next() {
                if s.contains("CONSONANT_STEM_PERFECT_PI") {
                    properties |= CONSONANT_STEM_PERFECT_PI;
                } else if s.contains("CONSONANT_STEM_PERFECT_MU_PI") {
                    properties |= CONSONANT_STEM_PERFECT_MU_PI;
                } else if s.contains("CONSONANT_STEM_PERFECT_GAMMA") {
                    properties |= CONSONANT_STEM_PERFECT_GAMMA;
                } else if s.contains("CONSONANT_STEM_PERFECT_KAPPA") {
                    properties |= CONSONANT_STEM_PERFECT_KAPPA;
                } else if s.contains("CONSONANT_STEM_PERFECT_CHI") {
                    properties |= CONSONANT_STEM_PERFECT_CHI;
                } else if s.contains("CONSONANT_STEM_PERFECT_BETA") {
                    properties |= CONSONANT_STEM_PERFECT_BETA;
                } else if s.contains("CONSONANT_STEM_PERFECT_LAMBDA") {
                    properties |= CONSONANT_STEM_PERFECT_LAMBDA;
                } else if s.contains("CONSONANT_STEM_PERFECT_NU") {
                    properties |= CONSONANT_STEM_PERFECT_NU;
                } else if s.contains("CONSONANT_STEM_PERFECT_SIGMA") {
                    properties |= CONSONANT_STEM_PERFECT_SIGMA;
                } else if s.contains("CONSONANT_STEM_PERFECT_PHI") {
                    properties |= CONSONANT_STEM_PERFECT_PHI;
                }
                if s.contains("PREFIXED") {
                    properties |= PREFIXED;
                }
            }
            return HcGreekVerb::from_string(id, pps, properties, hq_unit);
        }
        None
    }

    //page 316 in h&q
    pub fn deponent_type(&self) -> HcDeponentType {
        if self.pps[0].ends_with("γίγνομαι") {
            //and παραγίγνομαι
            //From Hardy: "I guess γίγνομαι is technically a partial deponent, though in practice I don't think we're in the habit of calling it that.  We simply say that's a deponent (i.e. a middle deponent) with one active PP."
            HcDeponentType::GignomaiDeponent //see H&Q page 382. fix me, there may be a better way to do this without separate case
        } else if self.pps[0].ends_with("μαι") && self.pps[1].ends_with("μαι") && self.pps[2].ends_with("μην") && self.pps[3] == "—" /* && utf8HasSuffix(v->perfmid, "μαι") */ && self.pps[5] == "—"
        {
            HcDeponentType::MiddleDeponent
        }
        //this gets μετανίσταμαι and ἐπανίσταμαι: middle deponents which happen to have an active perfect and root aorist
        else if self.pps[0].ends_with("μαι") && self.pps[1].ends_with("μαι") && self.pps[2].ends_with("ην") /* && utf8HasSuffix(v->perfmid, "μαι") */ && self.pps[5] == "—"
        {
            HcDeponentType::MiddleDeponent
        } else if self.pps[0].ends_with("μαι")
            && self.pps[1].ends_with("μαι")
            && self.pps[2] == "—"
            && self.pps[3] == "—"
            && self.pps[4].ends_with("μαι")
            && self.pps[5] != "—"
        {
            HcDeponentType::PassiveDeponent
        } else if self.pps[0].ends_with("ἐπίσταμαι") {
            HcDeponentType::PassiveDeponent //close enough
        } else if self.pps[0].ends_with("ἡγέομαι") {
            //doesn't seem to have future passive, though?
            HcDeponentType::MiddleDeponentHgeomai //we call it a middle deponent which happens to also have a 6th pp
        } else if self.pps[0].ends_with("μαι")
            || self.pps[1].ends_with("μαι")
            || self.pps[2].ends_with("μην")
        {
            HcDeponentType::PartialDeponent
        } else {
            HcDeponentType::NotDeponent
        }
    }

    fn is_consonant_stem(&self, pp: &str) -> bool {
        if !pp.is_empty() {
            pp.ends_with("γμαι")
                || pp.ends_with("σμαι")
                || pp.ends_with("μμαι")
                || pp.ends_with("λμαι")
        } else {
            self.properties & CONSONANT_STEM_PERFECT_PI == CONSONANT_STEM_PERFECT_PI
                || self.properties & CONSONANT_STEM_PERFECT_MU_PI == CONSONANT_STEM_PERFECT_MU_PI
                || self.properties & CONSONANT_STEM_PERFECT_GAMMA == CONSONANT_STEM_PERFECT_GAMMA
                || self.properties & CONSONANT_STEM_PERFECT_KAPPA == CONSONANT_STEM_PERFECT_KAPPA
                || self.properties & CONSONANT_STEM_PERFECT_CHI == CONSONANT_STEM_PERFECT_CHI
                || self.properties & CONSONANT_STEM_PERFECT_BETA == CONSONANT_STEM_PERFECT_BETA
                || self.properties & CONSONANT_STEM_PERFECT_LAMBDA == CONSONANT_STEM_PERFECT_LAMBDA
                || self.properties & CONSONANT_STEM_PERFECT_NU == CONSONANT_STEM_PERFECT_NU
                || self.properties & CONSONANT_STEM_PERFECT_SIGMA == CONSONANT_STEM_PERFECT_SIGMA
                || self.properties & CONSONANT_STEM_PERFECT_PHI == CONSONANT_STEM_PERFECT_PHI
        }
    }
}

#[derive(Default, PartialEq, Eq, Debug)]
pub struct Step {
    pub form: String,
    pub explanation: String,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct HcGreekVerbForm {
    pub verb: Arc<HcGreekVerb>,
    pub person: Option<HcPerson>,
    pub number: Option<HcNumber>,
    pub tense: HcTense,
    pub voice: HcVoice,
    pub mood: HcMood,
    pub gender: Option<HcGender>,
    pub case: Option<HcCase>,
}

static SEPARATOR: &str = "‐";
static BLANK: &str = "—";

pub trait HcVerbForms {
    fn is_contracted_verb(&self, form: &str) -> bool;
    fn is_legal_form(&self) -> bool;
    fn is_legal_deponent(&self, pp: &str) -> bool;
    fn get_description(&self, prev: &HcGreekVerbForm, start: &str, end: &str) -> String;
    fn get_form(&self, decompose: bool) -> Result<Vec<Step>, HcFormError>;
    fn get_pp_num(&self) -> HcGreekPrincipalParts;
    fn get_pp(&self) -> Option<String>;
    fn strip_ending(&self, pp_num: usize, form: String) -> Result<String, &str>;
    fn add_ending(
        &self,
        full_stem: &str,
        stem: &str,
        ending: &str,
        decompose: bool,
    ) -> Result<String, &str>;
    fn get_endings(&self, stem: &str) -> Option<Vec<&str>>;
    fn adjust_stem(&self, full_stem: &str, stem: &str, decompose: bool) -> Option<String>;
    fn get_participle_endings(&self, _stem: &str) -> Option<Vec<&str>>;
    fn get_infinitive_endings(&self, _stem: &str) -> Option<Vec<&str>>;
    fn get_label(&self) -> String;
    fn is_deponent(&self, stem: &str) -> bool;
    fn change_params(
        &mut self,
        n_params_to_change: u8,
        parameters: &VerbParameters,
        params_do_not_change: &mut [HcParameters],
    ) -> Vec<HcParameters>;
    fn random_form(
        &self,
        num_changes: u8,
        highest_unit: Option<i16>,
        parameters: &VerbParameters,
        filter_forms: Option<&HashSet<u32>>,
    ) -> (HcGreekVerbForm, Diagnostics);
    fn block_for_hq_unit(&self, unit: Option<i16>) -> bool;
    fn param_hash(&self) -> u32;
    fn extract_params_from_hash(&mut self, value: u32);
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

pub fn get_voice_label(
    tense: HcTense,
    voice: HcVoice,
    mood: HcMood,
    _deponent_type: HcDeponentType,
) -> String {
    if voice == HcVoice::Middle && mood == HcMood::Imperative {
        String::from("Middle")
    } else if voice == HcVoice::Passive && mood == HcMood::Imperative {
        String::from("Passive")
    } else if tense != HcTense::Future && tense != HcTense::Aorist && voice == HcVoice::Middle {
        String::from("Middle (Middle/Passive)")
    } else if tense != HcTense::Future && tense != HcTense::Aorist && voice == HcVoice::Passive {
        String::from("Passive (Middle/Passive)")
    } else {
        String::from(voice.value())
    }
}

trait ReplaceLast {
    fn replace_last(&self, r: char) -> String;
}

impl ReplaceLast for String {
    fn replace_last(&self, r: char) -> String {
        self.chars()
            .enumerate()
            .map(|(i, c)| if i == self.chars().count() - 1 { r } else { c })
            .collect::<String>()
    }
}

static CONSONANT_REPLACEMENTS: &[[&str; 4]; 26] = &[
    //phi
    ["φ", "σθ", "φσθ", "φθ"],
    ["φ", "μ", "φμ", "μμ"],
    ["φ", "σ", "φσ", "ψ"],
    ["φ", "τ", "φτ", "πτ"],
    //pi
    ["π", "σθ", "πσθ", "φθ"],
    ["μπ", "μ", "μπμ", "μμ"], //pempw
    ["π", "μ", "πμ", "μμ"],
    ["π", "σ", "πσ", "ψ"],
    //["π", "τ", "πτ", "πτ"], //no replace
    //beta
    ["β", "σθ", "βσθ", "φθ"],
    ["β", "μ", "βμ", "μμ"],
    ["β", "σ", "βσ", "ψ"],
    ["β", "τ", "βτ", "πτ"],
    //kappa
    ["κ", "σθ", "κσθ", "χθ"],
    ["κ", "μ", "κμ", "γμ"],
    ["κ", "σ", "κσ", "ξ"],
    //["κ", "τ", "κτ", "κτ"], //no replace
    //chi
    ["χ", "σθ", "χσθ", "χθ"],
    ["χ", "μ", "χμ", "γμ"],
    ["χ", "σ", "χσ", "ξ"],
    ["χ", "τ", "χτ", "κτ"],
    //gamma
    ["γ", "σθ", "γσθ", "χθ"],
    //["γ", "μ", "γμ", "γμ"],  //no replace
    ["γ", "σ", "γσ", "ξ"],
    ["γ", "τ", "γτ", "κτ"],
    //sigma
    ["σ", "σ", "σσ", "σ"],
    //lambda
    ["λ", "σθ", "λσθ", "λθ"],
    //nu
    ["ν", "σθ", "νσθ", "νθ"],
    ["ν", "μ", "νμ", "μμ"],
];
impl HcGreekVerbForm {
    fn contract_consonants(&self, unaccented_form: &str, ending: &str, decompose: bool) -> String {
        //3rd plural and account for non-consonant stem version of swzw
        if self.person == Some(HcPerson::Third)
            && self.number == Some(HcNumber::Plural)
            && unaccented_form != "σεσω"
            && unaccented_form != "ἐσεσω"
            && unaccented_form != "ε ‐ σεσω"
        {
            return String::from("—");
        }
        if self.person == Some(HcPerson::Second)
            && self.number == Some(HcNumber::Singular)
            && (unaccented_form == "ᾐσχυμ"
                || unaccented_form == "πεφασ"
                || unaccented_form == "ἐπεφασ"
                || unaccented_form == "ε ‐ πεφασ")
        {
            return String::from("—");
        }
        let mut form = unaccented_form.to_string();

        // add original consonant when remove ending
        if self.verb.properties & CONSONANT_STEM_PERFECT_MU_PI == CONSONANT_STEM_PERFECT_MU_PI {
            form.push('π');
        } else if self.verb.properties & CONSONANT_STEM_PERFECT_PHI == CONSONANT_STEM_PERFECT_PHI {
            form = form.replace_last('φ');
        } else if self.verb.properties & CONSONANT_STEM_PERFECT_KAPPA
            == CONSONANT_STEM_PERFECT_KAPPA
        {
            form = form.replace_last('κ');
        } else if self.verb.properties & CONSONANT_STEM_PERFECT_PI == CONSONANT_STEM_PERFECT_PI {
            form = form.replace_last('π');
        } else if self.verb.properties & CONSONANT_STEM_PERFECT_BETA == CONSONANT_STEM_PERFECT_BETA
        {
            form = form.replace_last('β');
        } else if self.verb.properties & CONSONANT_STEM_PERFECT_CHI == CONSONANT_STEM_PERFECT_CHI {
            form = form.replace_last('χ');
        } else if self.verb.properties & CONSONANT_STEM_PERFECT_NU == CONSONANT_STEM_PERFECT_NU
            && (unaccented_form == "ᾐσχυμ"
                || ((unaccented_form == "πεφασ"
                    || unaccented_form == "ἐπεφασ"
                    || unaccented_form == "ε ‐ πεφασ")
                    && (self.person != Some(HcPerson::First) || decompose)))
        {
            form = form.replace_last('ν');
        }

        if decompose {
            return format!("{} {} {}", form, SEPARATOR, ending);
        }

        let mut found = false;
        for r in CONSONANT_REPLACEMENTS {
            if form.ends_with(r[0]) && ending.starts_with(r[1]) {
                form.push_str(ending);
                form = form.replacen(r[2], r[3], 1);
                found = true;
                break;
            }
        }
        if !found {
            form.push_str(ending);
        }
        form
    }

    fn separate_prefix(&self, stem: &str) -> String {
        // let pre = vec![("ἀπο", vec!["ἀπο"], "")];
        // for p in pre {
        //     if stem.starts_with(p.0) {
        //         return stem.replacen(p.0, format!("{} {}", p.1.join(" - "), p.2).as_str(), 1);
        //     }
        // }

        if stem.starts_with("ἀπολ") {
            return stem.replacen("ἀπολ", format!("ἀπο {} ολ", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("ἀπο") {
            return stem.replacen("ἀπο", format!("ἀπο {} ", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("ἀφῑ") {
            return stem.replacen("ἀφῑ", format!("ἀπο {} ῑ̔", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("ἀφει") {
            return stem.replacen("ἀφει", format!("ἀπο {} εἱ", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("ἀφε") {
            return stem.replacen("ἀφε", format!("ἀπο {} ἑ", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("ἀφη") {
            return stem.replacen("ἀφη", format!("ἀπο {} ἡ", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("ἀπεκ") {
            return stem.replacen("ἀπεκ", format!("ἀπο {} εκ", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("ἀνε") {
            return stem.replacen("ἀνε", format!("ἀνα {} ε", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("ἐκ") {
            return stem.replacen("ἐκ", format!("ἐκ {} ", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("προ") {
            return stem.replacen("προ", format!("προ {} ", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("συμ") {
            return stem.replacen("συμ", format!("συν {} ", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("συνῑ") {
            return stem.replacen("συνῑ", format!("συν {} ῑ̔", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("συνη") && self.verb.pps[0].ends_with("ῑ́ημι") {
            return stem.replacen("συνη", format!("συν {} ἡ", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("συνει") && self.verb.pps[0].ends_with("δα") {
            return stem.replacen("συνει", format!("συν {} εἰ", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("συνει") {
            return stem.replacen("συνει", format!("συν {} εἱ", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("συν") {
            return stem.replacen("συν", format!("συν {} ", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("διο") {
            // διοίσω
            return stem.replacen("διο", format!("δια {} ο", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("δια") {
            return stem.replacen("δια", format!("δια {} ", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("ὑπο") {
            return stem.replacen("ὑπο", format!("ὑπο {} ", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("ὑπα") {
            return stem.replacen(
                "ὑπα",
                format!("ὑπο {} α" /* FIX ME ἀ */, SEPARATOR).as_str(),
                1,
            );
        } else if stem.starts_with("ἀνα") {
            return stem.replacen("ἀνα", format!("ἀνα {} ", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("παρα") {
            return stem.replacen("παρα", format!("παρα {} ", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("ἐπιστ") {
            //fix me
            return stem.to_string();
        } else if stem.starts_with("ἐπι") {
            return stem.replacen("ἐπι", format!("ἐπι {} ", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("ἀφι") {
            return stem.replacen("ἀφι", format!("ἀπο {} ἱ", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("καθι") {
            return stem.replacen("καθι", format!("κατα {} ἱ", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("καθε") {
            return stem.replacen("καθε", format!("κατα {} ἑ", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("κατα") {
            return stem.replacen("κατα", format!("κατα {} ", SEPARATOR).as_str(), 1);
        } else if stem.starts_with("μετανα") {
            return stem.replacen(
                "μετανα",
                format!("μετα {} ανα {} ", SEPARATOR, SEPARATOR).as_str(),
                1,
            );
        } else if stem.starts_with("μετανι") {
            return stem.replacen(
                "μετανι",
                format!("μετα {} ανα {} ἱ", SEPARATOR, SEPARATOR).as_str(),
                1,
            );
        } else if stem.starts_with("μετανε") {
            return stem.replacen(
                "μετανε",
                format!("μετα {} ανα {} ἑ", SEPARATOR, SEPARATOR).as_str(),
                1,
            );
        } else if stem.starts_with("ἐπανα") {
            return stem.replacen(
                "ἐπανα",
                format!("ἐπι {} ανα {} ", SEPARATOR, SEPARATOR).as_str(),
                1,
            );
        } else if stem.starts_with("ἐπανι") {
            return stem.replacen(
                "ἐπανι",
                format!("ἐπι {} ανα {} ἱ", SEPARATOR, SEPARATOR).as_str(),
                1,
            );
        } else if stem.starts_with("ἐπανε") {
            return stem.replacen(
                "ἐπανε",
                format!("ἐπι {} ανα {} ἑ", SEPARATOR, SEPARATOR).as_str(),
                1,
            );
        } else if stem.starts_with("μετα") {
            return stem.replacen("μετα", format!("μετα {} ", SEPARATOR).as_str(), 1);
        }
        stem.to_string()
    }

    fn accent_participle(&self, word: &str, stem: &str) -> String {
        let mut syllables = analyze_syllable_quantities(
            word,
            self.person,
            self.number,
            self.tense,
            self.mood,
            self.verb.properties,
        );
        syllables.reverse();

        const ULTIMA: usize = 0;
        const PENULT: usize = 1;
        const ANTEPENULT: usize = 2;
        let mut accent;
        let mut accent_position = match self.tense {
            HcTense::Present | HcTense::Future => match self.voice {
                HcVoice::Active => match self.number {
                    Some(HcNumber::Singular) => match self.case {
                        Some(HcCase::Nominative) | Some(HcCase::Vocative) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 2 {
                                    ANTEPENULT
                                } else if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => todo!(),
                        },
                        Some(HcCase::Genitive) | Some(HcCase::Dative) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                if syllables.len() > 2 {
                                    ANTEPENULT
                                } else if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => todo!(),
                        },
                        Some(HcCase::Accusative) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Feminine) => {
                                if syllables.len() > 2 {
                                    ANTEPENULT
                                } else if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Neuter) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => todo!(),
                        },
                        None => todo!(),
                    },
                    Some(HcNumber::Plural) => match self.case {
                        Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                            if syllables.len() > 2 {
                                ANTEPENULT
                            } else if syllables.len() > 1 {
                                PENULT
                            } else {
                                ULTIMA
                            }
                        }
                        Some(HcCase::Genitive) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => ULTIMA,
                            None => PENULT,
                        },
                        Some(HcCase::Dative) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                if syllables.len() > 2 {
                                    ANTEPENULT
                                } else if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => PENULT,
                        },
                        Some(HcCase::Accusative) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                if syllables.len() > 2 {
                                    ANTEPENULT
                                } else if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => PENULT,
                        },
                        None => todo!(),
                    },
                    Some(HcNumber::Dual) => todo!(),
                    None => todo!(),
                },
                HcVoice::Middle | HcVoice::Passive => match self.number {
                    Some(HcNumber::Singular) => match self.case {
                        Some(HcCase::Nominative) | Some(HcCase::Vocative) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                if syllables.len() > 2 {
                                    ANTEPENULT
                                } else if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => todo!(),
                        },
                        Some(HcCase::Genitive) | Some(HcCase::Dative) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => todo!(),
                        },
                        Some(HcCase::Accusative) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                if syllables.len() > 2 {
                                    ANTEPENULT
                                } else if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => todo!(),
                        },
                        None => todo!(),
                    },
                    Some(HcNumber::Plural) => match self.case {
                        Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                            if syllables.len() > 2 {
                                ANTEPENULT
                            } else if syllables.len() > 1 {
                                PENULT
                            } else {
                                ULTIMA
                            }
                        }
                        Some(HcCase::Genitive) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => PENULT,
                        },
                        Some(HcCase::Dative) => {
                            if syllables.len() > 1 {
                                PENULT
                            } else {
                                ULTIMA
                            }
                        }
                        Some(HcCase::Accusative) => match self.gender {
                            Some(HcGender::Masculine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Neuter) => {
                                if syllables.len() > 2 {
                                    ANTEPENULT
                                } else if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => PENULT,
                        },
                        None => todo!(),
                    },
                    Some(HcNumber::Dual) => todo!(),
                    None => todo!(),
                },
            },
            HcTense::Aorist => match self.voice {
                HcVoice::Active => {
                    if stem.ends_with("ον") || stem.ends_with("ομην") {
                        match self.number {
                            Some(HcNumber::Singular) => match self.case {
                                Some(HcCase::Nominative) | Some(HcCase::Vocative) => match self
                                    .gender
                                {
                                    Some(HcGender::Masculine) | Some(HcGender::Neuter) => ULTIMA,
                                    Some(HcGender::Feminine) => {
                                        if syllables.len() > 1 {
                                            PENULT
                                        } else {
                                            ULTIMA
                                        }
                                    }
                                    None => todo!(),
                                },
                                Some(HcCase::Genitive) | Some(HcCase::Dative) => {
                                    match self.gender {
                                        Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                            if syllables.len() > 1 {
                                                PENULT
                                            } else {
                                                ULTIMA
                                            }
                                        }
                                        Some(HcGender::Feminine) => {
                                            if syllables.len() > 1 {
                                                PENULT
                                            } else {
                                                ULTIMA
                                            }
                                        }
                                        None => todo!(),
                                    }
                                }
                                Some(HcCase::Accusative) => match self.gender {
                                    Some(HcGender::Masculine) | Some(HcGender::Feminine) => {
                                        if syllables.len() > 1 {
                                            PENULT
                                        } else {
                                            ULTIMA
                                        }
                                    }
                                    Some(HcGender::Neuter) => ULTIMA,
                                    None => todo!(),
                                },
                                None => todo!(),
                            },
                            Some(HcNumber::Plural) => match self.case {
                                Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                    if syllables.len() > 1 {
                                        PENULT
                                    } else {
                                        ULTIMA
                                    }
                                }
                                Some(HcCase::Genitive) => match self.gender {
                                    Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                        if syllables.len() > 1 {
                                            PENULT
                                        } else {
                                            ULTIMA
                                        }
                                    }
                                    Some(HcGender::Feminine) => ULTIMA,
                                    None => PENULT,
                                },
                                Some(HcCase::Dative) => match self.gender {
                                    Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                        if syllables.len() > 1 {
                                            PENULT
                                        } else {
                                            ULTIMA
                                        }
                                    }
                                    Some(HcGender::Feminine) => {
                                        if syllables.len() > 1 {
                                            PENULT
                                        } else {
                                            ULTIMA
                                        }
                                    }
                                    None => PENULT,
                                },
                                Some(HcCase::Accusative) => match self.gender {
                                    Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                        if syllables.len() > 1 {
                                            PENULT
                                        } else {
                                            ULTIMA
                                        }
                                    }
                                    Some(HcGender::Feminine) => {
                                        if syllables.len() > 1 {
                                            PENULT
                                        } else {
                                            ULTIMA
                                        }
                                    }
                                    None => PENULT,
                                },
                                None => todo!(),
                            },
                            Some(HcNumber::Dual) => todo!(),
                            None => todo!(),
                        }
                    } else {
                        match self.number {
                            Some(HcNumber::Singular) => match self.case {
                                Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                    match self.gender {
                                        Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                            if syllables.len() > 1 {
                                                PENULT
                                            } else {
                                                ULTIMA
                                            }
                                        }
                                        Some(HcGender::Feminine) => {
                                            if syllables.len() > 2 {
                                                ANTEPENULT
                                            } else if syllables.len() > 1 {
                                                PENULT
                                            } else {
                                                ULTIMA
                                            }
                                        }
                                        None => todo!(),
                                    }
                                }
                                Some(HcCase::Genitive) | Some(HcCase::Dative) => {
                                    match self.gender {
                                        Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                            if syllables.len() > 2 {
                                                ANTEPENULT
                                            } else if syllables.len() > 1 {
                                                PENULT
                                            } else {
                                                ULTIMA
                                            }
                                        }
                                        Some(HcGender::Feminine) => {
                                            if syllables.len() > 1 {
                                                PENULT
                                            } else {
                                                ULTIMA
                                            }
                                        }
                                        None => todo!(),
                                    }
                                }
                                Some(HcCase::Accusative) => match self.gender {
                                    Some(HcGender::Masculine) | Some(HcGender::Feminine) => {
                                        if syllables.len() > 2 {
                                            ANTEPENULT
                                        } else if syllables.len() > 1 {
                                            PENULT
                                        } else {
                                            ULTIMA
                                        }
                                    }
                                    Some(HcGender::Neuter) => {
                                        if syllables.len() > 1 {
                                            PENULT
                                        } else {
                                            ULTIMA
                                        }
                                    }
                                    None => todo!(),
                                },
                                None => todo!(),
                            },
                            Some(HcNumber::Plural) => match self.case {
                                Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                    if syllables.len() > 2 {
                                        ANTEPENULT
                                    } else if syllables.len() > 1 {
                                        PENULT
                                    } else {
                                        ULTIMA
                                    }
                                }
                                Some(HcCase::Genitive) => match self.gender {
                                    Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                        if syllables.len() > 1 {
                                            PENULT
                                        } else {
                                            ULTIMA
                                        }
                                    }
                                    Some(HcGender::Feminine) => ULTIMA,
                                    None => PENULT,
                                },
                                Some(HcCase::Dative) => match self.gender {
                                    Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                        if syllables.len() > 2 {
                                            ANTEPENULT
                                        } else if syllables.len() > 1 {
                                            PENULT
                                        } else {
                                            ULTIMA
                                        }
                                    }
                                    Some(HcGender::Feminine) => {
                                        if syllables.len() > 1 {
                                            PENULT
                                        } else {
                                            ULTIMA
                                        }
                                    }
                                    None => PENULT,
                                },
                                Some(HcCase::Accusative) => match self.gender {
                                    Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                        if syllables.len() > 2 {
                                            ANTEPENULT
                                        } else if syllables.len() > 1 {
                                            PENULT
                                        } else {
                                            ULTIMA
                                        }
                                    }
                                    Some(HcGender::Feminine) => {
                                        if syllables.len() > 1 {
                                            PENULT
                                        } else {
                                            ULTIMA
                                        }
                                    }
                                    None => PENULT,
                                },
                                None => todo!(),
                            },
                            Some(HcNumber::Dual) => todo!(),
                            None => todo!(),
                        }
                    }
                }
                HcVoice::Middle => match self.number {
                    Some(HcNumber::Singular) => match self.case {
                        Some(HcCase::Nominative) | Some(HcCase::Vocative) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                if syllables.len() > 2 {
                                    ANTEPENULT
                                } else if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => todo!(),
                        },
                        Some(HcCase::Genitive) | Some(HcCase::Dative) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => todo!(),
                        },
                        Some(HcCase::Accusative) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                if syllables.len() > 2 {
                                    ANTEPENULT
                                } else if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => todo!(),
                        },
                        None => todo!(),
                    },
                    Some(HcNumber::Plural) => match self.case {
                        Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                            if syllables.len() > 2 {
                                ANTEPENULT
                            } else if syllables.len() > 1 {
                                PENULT
                            } else {
                                ULTIMA
                            }
                        }
                        Some(HcCase::Genitive) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => PENULT,
                        },
                        Some(HcCase::Dative) => {
                            if syllables.len() > 1 {
                                PENULT
                            } else {
                                ULTIMA
                            }
                        }
                        Some(HcCase::Accusative) => match self.gender {
                            Some(HcGender::Masculine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Neuter) => {
                                if syllables.len() > 2 {
                                    ANTEPENULT
                                } else if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => PENULT,
                        },
                        None => todo!(),
                    },
                    Some(HcNumber::Dual) => todo!(),
                    None => todo!(),
                },
                HcVoice::Passive => match self.number {
                    Some(HcNumber::Singular) => match self.case {
                        Some(HcCase::Nominative) | Some(HcCase::Vocative) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Neuter) => ULTIMA,
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => todo!(),
                        },
                        Some(HcCase::Genitive) | Some(HcCase::Dative) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => todo!(),
                        },
                        Some(HcCase::Accusative) => match self.gender {
                            Some(HcGender::Masculine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Neuter) => ULTIMA,
                            None => todo!(),
                        },
                        None => todo!(),
                    },
                    Some(HcNumber::Plural) => match self.case {
                        Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                            if syllables.len() > 1 {
                                PENULT
                            } else {
                                ULTIMA
                            }
                        }
                        Some(HcCase::Genitive) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => ULTIMA,
                            None => PENULT,
                        },
                        Some(HcCase::Dative) => {
                            if syllables.len() > 1 {
                                PENULT
                            } else {
                                ULTIMA
                            }
                        }
                        Some(HcCase::Accusative) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => PENULT,
                        },
                        None => todo!(),
                    },
                    Some(HcNumber::Dual) => todo!(),
                    None => todo!(),
                },
            },
            HcTense::Perfect => match self.voice {
                HcVoice::Active => match self.number {
                    Some(HcNumber::Singular) => match self.case {
                        Some(HcCase::Nominative) | Some(HcCase::Vocative) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Neuter) => ULTIMA,
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => todo!(),
                        },
                        Some(HcCase::Genitive) | Some(HcCase::Dative) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => todo!(),
                        },
                        Some(HcCase::Accusative) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Neuter) => ULTIMA,
                            None => todo!(),
                        },
                        None => todo!(),
                    },
                    Some(HcNumber::Plural) => match self.case {
                        Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                            if syllables.len() > 1 {
                                PENULT
                            } else {
                                ULTIMA
                            }
                        }
                        Some(HcCase::Genitive) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => ULTIMA,
                            None => PENULT,
                        },
                        Some(HcCase::Dative) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => PENULT,
                        },
                        Some(HcCase::Accusative) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => PENULT,
                        },
                        None => todo!(),
                    },
                    Some(HcNumber::Dual) => todo!(),
                    None => todo!(),
                },
                HcVoice::Middle | HcVoice::Passive => match self.number {
                    Some(HcNumber::Singular) => match self.case {
                        Some(HcCase::Nominative) | Some(HcCase::Vocative) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => todo!(),
                        },
                        Some(HcCase::Genitive) | Some(HcCase::Dative) => {
                            if syllables.len() > 1 {
                                PENULT
                            } else {
                                ULTIMA
                            }
                        }
                        Some(HcCase::Accusative) => match self.gender {
                            Some(HcGender::Masculine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Neuter) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => todo!(),
                        },
                        None => todo!(),
                    },
                    Some(HcNumber::Plural) => match self.case {
                        Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                            if syllables.len() > 1 {
                                PENULT
                            } else {
                                ULTIMA
                            }
                        }
                        Some(HcCase::Genitive) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => PENULT,
                        },
                        Some(HcCase::Dative) => {
                            if syllables.len() > 1 {
                                PENULT
                            } else {
                                ULTIMA
                            }
                        }
                        Some(HcCase::Accusative) => match self.gender {
                            Some(HcGender::Masculine) | Some(HcGender::Neuter) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            Some(HcGender::Feminine) => {
                                if syllables.len() > 1 {
                                    PENULT
                                } else {
                                    ULTIMA
                                }
                            }
                            None => PENULT,
                        },
                        None => todo!(),
                    },
                    Some(HcNumber::Dual) => todo!(),
                    None => todo!(),
                },
            },
            _ => return String::new(),
        };

        if stem.ends_with("μι") || stem.ends_with("κα") {
            if (self.tense == HcTense::Present || self.tense == HcTense::Aorist)
                && self.voice == HcVoice::Active
            {
                if self.number == Some(HcNumber::Singular)
                    && (self.gender == Some(HcGender::Masculine)
                        && (self.case == Some(HcCase::Nominative)
                            || self.case == Some(HcCase::Vocative))
                        || (self.gender == Some(HcGender::Neuter)
                            && (self.case == Some(HcCase::Nominative)
                                || self.case == Some(HcCase::Accusative)
                                || self.case == Some(HcCase::Vocative))))
                {
                    accent_position = ULTIMA
                } else if syllables.len() > 1 {
                    accent_position = PENULT
                } else {
                    accent_position = ULTIMA
                }
            }
        }

        if syllables.len() > 2
            && !syllables.first().unwrap().is_long
            && syllables[1].is_long
            && accent_position == PENULT
        {
            accent = HGK_CIRCUMFLEX;
        } else if syllables.len() > 2 && syllables.first().unwrap().is_long {
            accent = HGK_ACUTE;
        } else if syllables.len() == 2
            && !syllables.first().unwrap().is_long
            && syllables[1].is_long
            && accent_position == PENULT
        {
            accent = HGK_CIRCUMFLEX;
        } else {
            accent = HGK_ACUTE;
        }

        if self.is_contracted_verb(word) && self.voice == HcVoice::Active {
            if self.gender == Some(HcGender::Masculine) || self.gender == Some(HcGender::Neuter) {
                if (self.case == Some(HcCase::Nominative)
                    && self.number == Some(HcNumber::Singular))
                    || (self.case == Some(HcCase::Vocative)
                        && self.number == Some(HcNumber::Singular))
                    || (self.gender == Some(HcGender::Neuter)
                        && self.case == Some(HcCase::Accusative)
                        && self.number == Some(HcNumber::Singular))
                {
                    accent_position = ULTIMA;
                    accent = HGK_CIRCUMFLEX;
                } else if self.number == Some(HcNumber::Plural)
                    && self.case == Some(HcCase::Genitive)
                {
                    accent_position = PENULT;
                    accent = HGK_ACUTE;
                } else {
                    accent_position = PENULT;
                    accent = HGK_CIRCUMFLEX;
                }
            } else if self.gender == Some(HcGender::Feminine) {
                if self.case == Some(HcCase::Nominative)
                    || self.case == Some(HcCase::Vocative)
                    || (self.case == Some(HcCase::Accusative)
                        && self.number == Some(HcNumber::Singular))
                {
                    accent_position = PENULT;
                    accent = HGK_CIRCUMFLEX;
                } else if self.case == Some(HcCase::Genitive)
                    && self.number == Some(HcNumber::Plural)
                {
                    accent_position = ULTIMA;
                    accent = HGK_CIRCUMFLEX;
                } else {
                    accent_position = PENULT;
                    accent = HGK_ACUTE;
                }
            }
        }

        let letter_index = syllables[accent_position].index;
        self.accent_syllable(word, letter_index, accent)
    }

    fn accent_infinitive(&self, word: &str) -> String {
        let mut syllables = analyze_syllable_quantities(
            word,
            self.person,
            self.number,
            self.tense,
            self.mood,
            self.verb.properties,
        );
        syllables.reverse();

        const ULTIMA: usize = 0;
        const PENULT: usize = 1;
        const ANTEPENULT: usize = 2;
        let accent;
        let accent_position = match self.tense {
            HcTense::Present | HcTense::Future => match self.voice {
                HcVoice::Active => {
                    if syllables.len() > 1 {
                        PENULT
                    } else {
                        ULTIMA
                    }
                }
                _ => {
                    if syllables.len() > 2 {
                        ANTEPENULT
                    } else {
                        PENULT
                    }
                }
            },
            HcTense::Aorist => match self.voice {
                HcVoice::Active => {
                    if syllables.len() > 1 {
                        PENULT
                    } else {
                        ULTIMA
                    }
                }
                HcVoice::Middle => {
                    if syllables.len() > 2 {
                        ANTEPENULT
                    } else {
                        PENULT
                    }
                }
                HcVoice::Passive => {
                    if syllables.len() > 1 {
                        PENULT
                    } else {
                        ULTIMA
                    }
                }
            },
            HcTense::Perfect => {
                if syllables.len() > 1 {
                    PENULT
                } else {
                    ULTIMA
                }
            }
            _ => return String::new(),
        };

        if syllables.len() > 2
            && !syllables.first().unwrap().is_long
            && syllables[1].is_long
            && accent_position == PENULT
        {
            accent = HGK_CIRCUMFLEX;
        } else if syllables.len() > 2 && syllables.first().unwrap().is_long {
            accent = HGK_ACUTE;
        } else if syllables.len() == 2
            && !syllables.first().unwrap().is_long
            && syllables[1].is_long
            && accent_position == PENULT
        {
            accent = HGK_CIRCUMFLEX;
        } else {
            accent = HGK_ACUTE;
        }

        let letter_index = syllables[accent_position].index;
        self.accent_syllable(word, letter_index, accent)
    }

    fn add_augment(&self, stem: &str, decompose: bool) -> String {
        let mut local_stem = stem.to_string();
        if decompose {
            if local_stem.starts_with('ἠ')
                || local_stem.starts_with('ἡ')
                || local_stem.starts_with("εἰ")
                || local_stem.starts_with("ῑ̔")
            {
                local_stem
            } else if local_stem.starts_with("ἀφι")
                && self.verb.pps[0].starts_with("ἀφικνέομαι")
                && self.tense == HcTense::Pluperfect
            {
                local_stem.replacen("ἀφι", format!("ἀπο {} ῑ̔", SEPARATOR).as_str(), 1)
            } else if local_stem.starts_with("ἀπολ") {
                local_stem.replacen(
                    "ἀπολ",
                    format!("ἀπο {} ε {} ολ", SEPARATOR, SEPARATOR).as_str(),
                    1,
                )
            } else if local_stem.starts_with("ἀπο") {
                local_stem.replacen(
                    "ἀπο",
                    format!("ἀπο {} ε {} ", SEPARATOR, SEPARATOR).as_str(),
                    1,
                )
            } else if local_stem.starts_with("ἀπεκ") {
                local_stem.replacen("ἀπεκ", format!("ἀπο {} εκ", SEPARATOR).as_str(), 1)
            } else if local_stem.starts_with("-ἐ") {
                local_stem
            } else if local_stem.starts_with("-εἱ") {
                local_stem
            } else if local_stem.starts_with("ηὑ") {
                local_stem
            } else if local_stem.starts_with("ἐκ") {
                local_stem.replacen(
                    "ἐκ",
                    format!("ἐκ {} ε {} ", SEPARATOR, SEPARATOR).as_str(),
                    1,
                )
            } else if local_stem.starts_with("προ") {
                local_stem.replacen(
                    "προ",
                    format!("προ {} ε {} ", SEPARATOR, SEPARATOR).as_str(),
                    1,
                )
            } else if local_stem.starts_with("δια") {
                local_stem.replacen(
                    "δια",
                    format!("δια {} ε {} ", SEPARATOR, SEPARATOR).as_str(),
                    1,
                )
            } else if local_stem.starts_with("συνῑ") {
                local_stem.replacen("συνῑ", format!("συν {} ῑ̔", SEPARATOR).as_str(), 1)
            } else if local_stem.starts_with("συνει") {
                local_stem.replacen("συνει", format!("συν {} εἱ", SEPARATOR).as_str(), 1)
            } else if local_stem.starts_with("συνε") {
                local_stem.replacen("συνε", format!("συν {} ε", SEPARATOR).as_str(), 1)
            } else if local_stem.starts_with("συμ") {
                local_stem.replacen(
                    "συμ",
                    format!("συν {} ε {} ", SEPARATOR, SEPARATOR).as_str(),
                    1,
                )
            } else if local_stem.starts_with("διενη") {
                local_stem //.replacen("διενη", "διενη".to_string().as_str(), 1) /* FIX ME */
            } else if local_stem.starts_with("ὑπο") {
                local_stem.replacen(
                    "ὑπο",
                    format!("ὑπο {} ε {} ", SEPARATOR, SEPARATOR).as_str(),
                    1,
                )
            } else if local_stem.starts_with("ὑπα") {
                local_stem.replacen(
                    "ὑπα",
                    format!("ὑπο {} ε {} α" /* FIX ME ἀ */, SEPARATOR, SEPARATOR).as_str(),
                    1,
                )
            } else if local_stem.starts_with("ἀνα") {
                local_stem.replacen(
                    "ἀνα",
                    format!("ἀνα {} ε {} ", SEPARATOR, SEPARATOR).as_str(),
                    1,
                )
            } else if local_stem.starts_with("παρα") {
                local_stem.replacen(
                    "παρα",
                    format!("παρα {} ε {} ", SEPARATOR, SEPARATOR).as_str(),
                    1,
                )
            } else if local_stem.starts_with("ἐπιστ") {
                local_stem.replacen("ἐπιστ", format!("ε {} ἐπιστ", SEPARATOR).as_str(), 1)
            } else if local_stem.starts_with("ἐπι") {
                local_stem.replacen(
                    "ἐπι",
                    format!("ἐπι {} ε {} ", SEPARATOR, SEPARATOR).as_str(),
                    1,
                )
            } else if local_stem.starts_with("κατα") {
                local_stem.replacen(
                    "κατα",
                    format!("κατα {} ε {} ", SEPARATOR, SEPARATOR).as_str(),
                    1,
                )
            } else if local_stem.starts_with("μετανι") {
                local_stem.replacen(
                    "μετανι",
                    format!("μετα {} ανα {} ε {} ἱ", SEPARATOR, SEPARATOR, SEPARATOR).as_str(),
                    1,
                )
            } else if local_stem.starts_with("ἐπανι") {
                local_stem.replacen(
                    "ἐπανι",
                    format!("ἐπι {} ανα {} ε {} ἱ", SEPARATOR, SEPARATOR, SEPARATOR).as_str(),
                    1,
                )
            } else if local_stem.starts_with("ἐπανε") {
                if self.number == Some(HcNumber::Singular) {
                    local_stem.replacen(
                        "ἐπανε",
                        format!("ἐπι {} ανα {} ε {} ἑ", SEPARATOR, SEPARATOR, SEPARATOR).as_str(),
                        1,
                    )
                } else {
                    local_stem.replacen(
                        "ἐπανε",
                        format!("ἐπι {} ανα {} ἑ", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    )
                }
            } else if local_stem.starts_with("μετανε") {
                if self.number == Some(HcNumber::Singular) {
                    local_stem.replacen(
                        "μετανε",
                        format!("μετα {} ανα {} ε {} ἑ", SEPARATOR, SEPARATOR, SEPARATOR).as_str(),
                        1,
                    )
                } else {
                    local_stem.replacen(
                        "μετανε",
                        format!("μετα {} ανα {} ἑ", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    )
                }
            } else if local_stem.starts_with("μετα") {
                local_stem.replacen(
                    "μετα",
                    format!("μετα {} ε {} ", SEPARATOR, SEPARATOR).as_str(),
                    1,
                )
            } else if local_stem.starts_with("ἀφῑ") {
                local_stem.replacen("ἀφῑ", format!("ἀπο {} ῑ̔", SEPARATOR).as_str(), 1)
            } else if local_stem.starts_with("ἀφι") {
                local_stem.replacen(
                    "ἀφι",
                    format!("ἀπο {} ε {} ἱ", SEPARATOR, SEPARATOR).as_str(),
                    1,
                )
            } else if local_stem.starts_with("καθι") {
                local_stem.replacen(
                    "καθι",
                    format!("κατα {} ε {} ἱ", SEPARATOR, SEPARATOR).as_str(),
                    1,
                )
            } else if local_stem.starts_with("ἀφει") {
                local_stem.replacen(
                    "ἀφει",
                    format!("ἀπο {} ἑι" /* FIX ME breathing position */, SEPARATOR).as_str(),
                    1,
                )
            } else if local_stem.starts_with("ἀφε") {
                if self.number == Some(HcNumber::Singular)
                /*|| self.voice != HcVoice::Active FIX ME */
                {
                    local_stem.replacen(
                        "ἀφε",
                        format!("ἀπο {} ε {} ἑ", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    )
                } else {
                    local_stem.replacen("ἀφε", format!("ἀπο {} ἑ", SEPARATOR).as_str(), 1)
                }
            } else if local_stem.starts_with("καθε") {
                if self.number == Some(HcNumber::Singular)
                /*|| self.voice != HcVoice::Active FIX ME */
                {
                    local_stem.replacen(
                        "καθε",
                        format!("κατα {} ε {} ἑ", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    )
                } else {
                    local_stem.replacen("καθε", format!("κατα {} ἑ", SEPARATOR).as_str(), 1)
                }
            } else if local_stem.starts_with("ἑσ") {
                //isthmi
                if self.number == Some(HcNumber::Singular)
                /*|| self.voice != HcVoice::Active FIX ME */
                {
                    local_stem.replacen("ἑσ", format!("ε {} ἑσ", SEPARATOR).as_str(), 1)
                } else {
                    local_stem
                }
            } else if local_stem.starts_with('ἑ') {
                //epomai
                local_stem.replacen('ἑ', format!("ε {} ἑ", SEPARATOR).as_str(), 1)
            } else if local_stem.starts_with('ἐ') {
                if self.tense != HcTense::Pluperfect {
                    local_stem.replacen('ἐ', format!("ε {} ἐ", SEPARATOR).as_str(), 1)
                } else {
                    local_stem
                }
            } else if local_stem.starts_with('ᾐ') {
                local_stem
            } else if local_stem.starts_with('ᾑ') {
                local_stem
            } else {
                format!("ε {} {}", SEPARATOR, local_stem)
            }
        } else {
            //else if not decompose
            if local_stem.starts_with("ἀπολ") {
                local_stem.replacen("ἀπολ", "ἀπωλ", 1)
            } else if local_stem.starts_with("ἀπο") {
                local_stem.replacen("ἀπο", "ἀπε", 1)
            } else if local_stem.starts_with("εὑ") {
                local_stem.replacen("εὑ", "ηὑ", 1)
            } else if local_stem.starts_with("ηὑ") {
                local_stem
            } else if local_stem.starts_with("ἀπεκ") {
                local_stem
            } else if local_stem.starts_with('ὁ') {
                local_stem.replacen('ὁ', "ἑω", 1)
            } else if local_stem.starts_with("προ") {
                local_stem.replacen("προ", "προε", 1)
            } else if local_stem.starts_with("δια") {
                local_stem.replacen("δια", "διε", 1)
            } else if local_stem.starts_with("-εἱ") {
                local_stem
            } else if local_stem.starts_with("ἐκ") {
                local_stem.replacen("ἐκ", "ἐξε", 1)
            } else if local_stem.starts_with("συνε") {
                local_stem
            } else if local_stem.starts_with("συνῑ") {
                local_stem
            } else if local_stem.starts_with("συμ") {
                local_stem.replacen("συμ", "συνε", 1)
            } else if local_stem.starts_with("διε") {
                local_stem
            } else if local_stem.starts_with("ὑπο") {
                local_stem.replacen("ὑπο", "ὑπε", 1)
            } else if local_stem.starts_with("ὑπα") {
                local_stem.replacen("ὑπα", "ὑπη", 1)
            } else if local_stem.starts_with("ἀνα") {
                local_stem.replacen("ἀνα", "ἀνε", 1)
            } else if local_stem.starts_with("παρα") {
                local_stem.replacen("παρα", "παρε", 1)
            } else if local_stem.starts_with("ἐπιστ") {
                local_stem.replacen("ἐπιστ", "ἠπιστ", 1)
            } else if local_stem.starts_with("ἐπι") {
                local_stem.replacen("ἐπι", "ἐπε", 1)
            } else if local_stem.starts_with("κατα") {
                local_stem.replacen("κατα", "κατε", 1)
            } else if local_stem.starts_with("μετανε") {
                if self.number == Some(HcNumber::Singular) || self.voice != HcVoice::Active {
                    local_stem.replacen("μετανε", "μετανει", 1)
                } else {
                    local_stem
                }
            } else if local_stem.starts_with("μετανι") {
                local_stem.replacen("μετανι", "μετανῑ", 1)
            } else if local_stem.starts_with("ἐπανε") {
                if self.number == Some(HcNumber::Singular) || self.voice != HcVoice::Active {
                    local_stem.replacen("ἐπανε", "ἐπανει", 1)
                } else {
                    local_stem
                }
            } else if local_stem.starts_with("ἐπανι") {
                local_stem.replacen("ἐπανι", "ἐπανῑ", 1)
            } else if local_stem.starts_with("μετα") {
                local_stem.replacen("μετα", "μετε", 1)
            } else if local_stem.starts_with("ἀφῑ") {
                local_stem
            } else if local_stem.starts_with("ἀφι") {
                local_stem.replacen("ἀφι", "ἀφῑ", 1)
            } else if local_stem.starts_with("καθι") {
                local_stem.replacen("καθι", "καθῑ", 1)
            } else if local_stem.starts_with("ἀφει") {
                local_stem
            } else if local_stem.starts_with("ἀφε") {
                if self.number == Some(HcNumber::Singular) || self.voice != HcVoice::Active {
                    local_stem.replacen("ἀφε", "ἀφει", 1)
                } else {
                    local_stem
                }
            } else if local_stem.starts_with("καθε") {
                if self.number == Some(HcNumber::Singular) || self.voice != HcVoice::Active {
                    local_stem.replacen("καθε", "καθει", 1)
                } else {
                    local_stem
                }
            } else if local_stem.starts_with('ᾐ') {
                local_stem
            } else if local_stem.starts_with('ᾑ') {
                local_stem
            } else if local_stem.starts_with('ἁ') {
                local_stem.replacen('ἁ', "ἡ", 1)
            } else if local_stem.starts_with("αἰ") {
                local_stem.replacen("αἰ", "ᾐ", 1)
            } else if local_stem.starts_with("αἱ") {
                local_stem.replacen("αἱ", "ᾑ", 1)
            } else if local_stem.starts_with("ἑο") {
                local_stem
            } else if local_stem.starts_with("ἑω") {
                local_stem
            } else if local_stem.starts_with("-ἐ") {
                local_stem
            } else if local_stem.starts_with('ὠ') {
                local_stem
            } else if local_stem.starts_with('ἑ') {
                if self.number == Some(HcNumber::Singular) || self.voice != HcVoice::Active {
                    local_stem.replacen('ἑ', "εἱ", 1)
                } else {
                    local_stem
                }
            } else if local_stem.starts_with("εἰ") {
                local_stem
            } else if local_stem.starts_with("ἐχ") {
                local_stem.replacen("ἐχ", "εἰχ", 1)
            } else if local_stem.starts_with('ἐ') {
                if self.tense != HcTense::Pluperfect {
                    local_stem.replacen('ἐ', "ἠ", 1)
                } else {
                    local_stem
                }
            } else if local_stem.starts_with("ῑ̔") {
                local_stem
            } else if local_stem.starts_with('ἱ') {
                local_stem.replacen('ἱ', "ῑ̔", 1)
            } else if (self.verb.pps[0].starts_with('ἐ')
                || self.verb.pps[0].starts_with('ἄ')
                || self.verb.pps[0].starts_with('ἀ'))
                && !self.verb.pps[0].starts_with("ἀποθνῄσκω")
            {
                local_stem.remove(0);
                format!("ἠ{}", local_stem)
            } else if local_stem.starts_with('ἠ') || local_stem.starts_with('ἡ') {
                local_stem
            } else {
                format!("ἐ{}", local_stem)
            }
        }
    }

    fn deaugment(&self, a: &str, decompose: bool) -> String {
        let mut loc = a.to_string();

        if decompose {
            if loc.starts_with("ἀπε") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen(
                        "ἀπε",
                        format!("ἀπο {} ε {} ", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                } else {
                    loc = loc.replacen("ἀπε", format!("ἀπο {} ", SEPARATOR).as_str(), 1);
                }
            } else if loc.starts_with("ἀπω") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen(
                        "ἀπω",
                        format!("ἀπο {} ε {} ο", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                } else {
                    loc = loc.replacen("ἀπω", format!("ἀπο {} ο", SEPARATOR).as_str(), 1);
                }
            } else if loc.starts_with("ηὑ") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("ηὑ", format!("ε {} εὑ", SEPARATOR).as_str(), 1);
                } else {
                    loc = loc.replacen("ηὑ", "εὑ".to_string().as_str(), 1);
                }
            } else if loc.starts_with("ἀφηκ")
                && (self.mood != HcMood::Indicative
                    || self.number == Some(HcNumber::Plural)
                    || self.voice != HcVoice::Active)
            {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen(
                        "ἀφηκ",
                        format!("ἀπο {} ε {} ἑ", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                } else {
                    loc = loc.replacen("ἀφηκ", format!("ἀπο {} ἑ", SEPARATOR).as_str(), 1);
                }
            } else if loc.starts_with("ἀφει") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen(
                        "ἀφει",
                        format!("ἀπο {} ε {} ἑ", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                } else {
                    loc = loc.replacen("ἀφει", format!("ἀπο {} ἑ", SEPARATOR).as_str(), 1);
                }
            } else if loc.starts_with("ἀφῑ") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen(
                        "ἀφῑ",
                        format!("ἀπο {} ε {} ἱ", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                } else {
                    loc = loc.replacen("ἀφῑ", format!("ἀπο {} ἱ", SEPARATOR).as_str(), 1);
                }
            } else if loc.starts_with("ἀνη") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen(
                        "ἀνη",
                        format!("ἀνα {} ε {} ε", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                } else {
                    loc = loc.replacen("ἀνη", format!("ἀνα {} ε", SEPARATOR).as_str(), 1);
                }
            } else if loc.starts_with("ἀφη") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("ἀφη", format!("ἀπο {} ἡ", SEPARATOR).as_str(), 1);
                } else {
                    loc = loc.replacen("ἀφη", format!("ἀπο {} ἡ", SEPARATOR).as_str(), 1);
                }
            } else if loc.starts_with("-εἱθ") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("-εἱθ", format!("- ε {} ἑθ", SEPARATOR).as_str(), 1);
                } else {
                    loc = loc.replacen("-εἱθ", "-ἑθ".to_string().as_str(), 1);
                }
            } else if loc.starts_with("-ἡκ") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    if self.number == Some(HcNumber::Plural) || self.voice != HcVoice::Active {
                        loc = loc.replacen("-ἡκ", format!("- ε {} ἑ", SEPARATOR).as_str(), 1);
                        //fix me cf -hka
                    }
                } else {
                    loc = loc.replacen("-ἡκ", "-ἑ".to_string().as_str(), 1);
                }
            } else if loc.starts_with("προε") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen(
                        "προε",
                        format!("προ {} ε {} ", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                } else {
                    loc = loc.replacen("προε", format!("προ {} ", SEPARATOR).as_str(), 1);
                }
            } else if loc.starts_with("πρου") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen(
                        "πρου",
                        format!("προ {} ε {} ", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                } else {
                    loc = loc.replacen("πρου", format!("προ {} ", SEPARATOR).as_str(), 1);
                }
            } else if loc.starts_with("ἐξε") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen(
                        "ἐξε",
                        format!("ἐκ {} ε {} ", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                } else {
                    loc = loc.replacen("ἐξε", format!("ἐκ {} ", SEPARATOR).as_str(), 1);
                }
            } else if loc.starts_with("-ἐ") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("-ἐ", format!("- ε {} ", SEPARATOR).as_str(), 1);
                } else {
                    loc = loc.replacen("-ἐ", "-".to_string().as_str(), 1);
                }
            } else if loc.starts_with("συνηκ")
                && (self.mood != HcMood::Indicative
                    || self.number == Some(HcNumber::Plural)
                    || self.voice != HcVoice::Active)
            {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen(
                        "συνηκ",
                        format!("συν {} ε {} ἑ", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                } else {
                    loc = loc.replacen("συνηκ", format!("συν {} ἑ", SEPARATOR).as_str(), 1);
                }
            } else if loc.starts_with("συνη")
                && self.verb.pps[0].ends_with("ῑ́ημι")
                && self.number == Some(HcNumber::Singular)
            {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("συνη", format!("συν {} ἡ", SEPARATOR).as_str(), 1);
                } else {
                    loc = loc.replacen("συνη", format!("συν {} ἡ", SEPARATOR).as_str(), 1);
                }
            } else if loc.starts_with("συνη") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen(
                        "συνη",
                        format!("συν {} ε {} ε", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                } else {
                    loc = loc.replacen("συνη", format!("συν {} ε", SEPARATOR).as_str(), 1);
                }
            } else if loc.starts_with("συνει") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen(
                        "συνει",
                        format!("συν {} ε {} ἑ", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                } else {
                    loc = loc.replacen("συνει", format!("συν {} ἑ", SEPARATOR).as_str(), 1);
                }
            } else if loc.starts_with("συνε") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen(
                        "συνε",
                        format!("συν {} ε {} ", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                } else {
                    loc = loc.replacen("συνε", format!("συν {} ", SEPARATOR).as_str(), 1);
                }
            } else if loc.starts_with("διη") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen(
                        "διη",
                        format!("δια {} ε {} ε", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                } else {
                    loc = loc.replacen("διη", format!("δια {} ε", SEPARATOR).as_str(), 1);
                }
            } else if loc.starts_with("διε") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen(
                        "διε",
                        format!("δια {} ε {} ε", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                } else {
                    loc = loc.replacen("διε", format!("δια {} ε", SEPARATOR).as_str(), 1);
                }
            } else if loc.starts_with("ὑπε") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen(
                        "ὑπε",
                        format!("ὑπο {} ε {} ", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                } else {
                    loc = loc.replacen("ὑπε", format!("ὑπο {} ", SEPARATOR).as_str(), 1);
                }
            } else if loc.starts_with("ὑπη") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen(
                        "ὑπη",
                        format!("ὑπο {} ε {} α" /* FIX ME ἀ */, SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                } else {
                    loc = loc.replacen(
                        "ὑπη",
                        format!("ὑπο {} α" /* FIX ME ἀ */, SEPARATOR).as_str(),
                        1,
                    );
                }
            } else if loc.starts_with("ἐπεδ") || loc.starts_with("ἐπεβ") {
                //because pempw and epideiknumi
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen(
                        "ἐπε",
                        format!("ἐπι {} ε {} ", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                } else {
                    loc = loc.replacen("ἐπε", format!("ἐπι {} ", SEPARATOR).as_str(), 1);
                }
            } else if loc.starts_with("ἀνε") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen(
                        "ἀνε",
                        format!("ἀνα {} ε {} ", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                } else {
                    loc = loc.replacen("ἀνε", format!("ἀνα {} ", SEPARATOR).as_str(), 1);
                }
            } else if loc.starts_with("παρε") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen(
                        "παρε",
                        format!("παρα {} ε {} ", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                } else {
                    loc = loc.replacen("παρε", format!("παρα {} ", SEPARATOR).as_str(), 1);
                }
            } else if loc.starts_with("κατε") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen(
                        "κατε",
                        format!("κατα {} ε {} ", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                } else {
                    loc = loc.replacen("κατε", format!("κατα {} ", SEPARATOR).as_str(), 1);
                }
            } else if loc.starts_with("μετανε") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen(
                        "μετανε",
                        format!("μετα {} ανα {} ε {} ", SEPARATOR, SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                } else {
                    loc = loc.replacen(
                        "μετανε",
                        format!("μετα {} ανα {} ", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                }
            } else if loc.starts_with("ἐπανε") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen(
                        "ἐπανε",
                        format!("ἐπι {} ανα {} ε {} ", SEPARATOR, SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                } else {
                    loc = loc.replacen(
                        "ἐπανε",
                        format!("ἐπι {} ανα {} ", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                }
            } else if loc.starts_with("μετε") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen(
                        "μετε",
                        format!("μετα {} ε {} ", SEPARATOR, SEPARATOR).as_str(),
                        1,
                    );
                } else {
                    loc = loc.replacen("μετε", format!("μετα {} ", SEPARATOR).as_str(), 1);
                }
            } else if loc.starts_with("εἱ") && self.verb.pps[0].starts_with("αἱ") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("εἱ", format!("ε {} ἑ", SEPARATOR).as_str(), 1);
                } else {
                    loc = loc.replacen("εἱ", "ἑ", 1);
                }
            } else if loc.starts_with("εἰ") && self.verb.pps[0].starts_with("λέ") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("εἰ", format!("ε {} εἰ", SEPARATOR).as_str(), 1);
                } else {
                    //loc = loc.replacen("εἰ", "εἰ", 1);
                }
            } else if loc.starts_with('ὠ') {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen('ὠ', format!("ε {} ὀ", SEPARATOR).as_str(), 1);
                } else {
                    loc = loc.replacen('ὠ', "ὀ", 1);
                }
            } else if loc.starts_with('ᾐ') && self.verb.pps[0].starts_with("αἰ") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen('ᾐ', format!("ε {} αἰ", SEPARATOR).as_str(), 1);
                } else {
                    loc = loc.replacen('ᾐ', "αἰ", 1);
                }
            } else if loc.starts_with('ᾑ') && self.verb.pps[0].starts_with("αἱ") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen('ᾑ', format!("ε {} αἱ", SEPARATOR).as_str(), 1);
                } else {
                    loc = loc.replacen('ᾑ', "αἱ", 1);
                }
            } else if loc.starts_with('ἠ')
                && (self.verb.pps[0].starts_with('ἐ')
                    || self.verb.pps[0].starts_with("φέρω")
                    || self.verb.pps[1].starts_with("ἐρήσομαι"))
            {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen('ἠ', format!("ε {} ἐ", SEPARATOR).as_str(), 1);
                } else {
                    loc = loc.replacen('ἠ', "ἐ", 1);
                }
            } else if loc.starts_with('ἠ') && self.verb.pps[0].starts_with('ἔ') {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen('ἠ', format!("ε {} ἐ", SEPARATOR).as_str(), 1);
                } else {
                    loc = loc.replacen('ἠ', "ἐ", 1);
                }
            } else if loc.starts_with('ἡ') && (self.verb.pps[0].starts_with('ἁ')) {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen('ἡ', format!("ε {} ἁ", SEPARATOR).as_str(), 1);
                } else {
                    loc = loc.replacen('ἡ', "ἁ", 1);
                }
            } else if loc.starts_with('ἠ')
                && (self.verb.pps[0].starts_with('ἄ') || self.verb.pps[0].starts_with('ἀ'))
            {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen('ἠ', format!("ε {} ἀ", SEPARATOR).as_str(), 1);
                } else {
                    loc = loc.replacen('ἠ', "ἀ", 1);
                }
            } else if loc.starts_with('ἡ') {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    //loc = loc.replacen("ἡ", "ἡ".to_string().as_str(), 1);
                } else {
                    return loc;
                }
            } else if loc.starts_with("ἐρρ") {
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = loc.replacen("ἐρρ", format!("ε {} ῥ", SEPARATOR).as_str(), 1);
                } else {
                    loc = loc.replacen("ἐρρ", "ῥ", 1);
                }
            } else {
                loc.remove(0);
                if self.tense == HcTense::Aorist && self.mood == HcMood::Indicative {
                    loc = format!("ε {} {}", SEPARATOR, loc);
                }
            }
            loc
        } else {
            if loc.starts_with("ἀπε") {
                loc = loc.replacen("ἀπε", "ἀπο", 1);
            } else if loc.starts_with("ἀπω") {
                loc = loc.replacen("ἀπω", "ἀπο", 1);
            } else if loc.starts_with("ἀφῑ") {
                loc = loc.replacen("ἀφῑ", "ἀφι", 1);
            } else if loc.starts_with("ηὑ") {
                loc = loc.replacen("ηὑ", "εὑ", 1);
            } else if loc.starts_with("ἀνη") {
                loc = loc.replacen("ἀνη", "ἀνε", 1);
            } else if loc.starts_with("ἀφηκ")
                && (self.mood != HcMood::Indicative
                    || self.number == Some(HcNumber::Plural)
                    || self.voice != HcVoice::Active)
            {
                loc = loc.replacen("ἀφηκ", "ἀφε", 1);
            } else if loc.starts_with("συνηκ")
                && (self.mood != HcMood::Indicative
                    || self.number == Some(HcNumber::Plural)
                    || self.voice != HcVoice::Active)
            {
                loc = loc.replacen("συνηκ", "συνε", 1);
            } else if loc.starts_with("ἀφει") {
                loc = loc.replacen("ἀφει", "ἀφε", 1);
            } else if loc.starts_with("-ἡκ") {
                loc = loc.replacen("-ἡκ", "-ἑ", 1);
            } else if loc.starts_with("-ἐ") {
                loc = loc.replacen("-ἐ", "-", 1);
            } else if loc.starts_with("-εἱθ") {
                loc = loc.replacen("-εἱθ", "-ἑθ", 1);
            } else if loc.starts_with("προε") {
                loc = loc.replacen("προε", "προ", 1);
            } else if loc.starts_with("πρου") {
                loc = loc.replacen("πρου", "προ", 1);
            } else if loc.starts_with("ἐξε") {
                loc = loc.replacen("ἐξε", "ἐκ", 1);
            } else if loc.starts_with("διη") {
                loc = loc.replacen("διη", "διε", 1);
            } else if loc.starts_with("συνη") {
                loc = loc.replacen("συνη", "συνε", 1);
            } else if loc.starts_with("συνει") {
                loc = loc.replacen("συνει", "συνε", 1);
            } else if loc.starts_with("συνε") {
                loc = loc.replacen("συνε", "συμ", 1);
            } else if loc.starts_with("ὑπε") {
                loc = loc.replacen("ὑπε", "ὑπο", 1);
            } else if loc.starts_with("ὑπη") {
                loc = loc.replacen("ὑπη", "ὑπα", 1);
            } else if loc.starts_with("ἐπεδ") || loc.starts_with("ἐπεβ") {
                //because pempw and epideiknumi, ἐπιβουλεύω
                loc = loc.replacen("ἐπε", "ἐπι", 1);
            } else if loc.starts_with("ἀνε") {
                loc = loc.replacen("ἀνε", "ἀνα", 1);
            } else if loc.starts_with("παρε") {
                loc = loc.replacen("παρε", "παρα", 1);
            } else if loc.starts_with("κατε") {
                loc = loc.replacen("κατε", "κατα", 1);
            } else if loc.starts_with("μετανε") {
                loc = loc.replacen("μετανε", "μετανα", 1);
            } else if loc.starts_with("ἐπανε") {
                loc = loc.replacen("ἐπανε", "ἐπανα", 1);
            } else if loc.starts_with("μετε") {
                loc = loc.replacen("μετε", "μετα", 1);
            } else if loc.starts_with("εἱ") {
                loc = loc.replacen("εἱ", "ἑ", 1);
            } else if loc.starts_with("εἰ") && self.verb.pps[0].starts_with("λέ") {
            } else if loc.starts_with("ἐρρ") {
                loc = loc.replacen("ἐρρ", "ῥ", 1);
            } else if loc.starts_with('ᾐ') {
                loc = loc.replacen('ᾐ', "αἰ", 1);
            } else if loc.starts_with('ᾑ') {
                loc = loc.replacen('ᾑ', "αἱ", 1);
            } else if loc.starts_with('ὠ') {
                loc = loc.replacen('ὠ', "ὀ", 1);
            } else if loc.starts_with('ἠ')
                && (self.verb.pps[0].starts_with('ἐ')
                    || self.verb.pps[0].starts_with("φέρω")
                    || self.verb.pps[1].starts_with("ἐρήσομαι"))
            {
                loc.remove(0);
                loc = format!("ἐ{}", loc);
            } else if loc.starts_with('ἠ') && self.verb.pps[0].starts_with('ἔ') {
                loc.remove(0);
                loc = format!("ἐ{}", loc);
            } else if loc.starts_with('ἡ') && (self.verb.pps[0].starts_with('ἁ')) {
                loc.remove(0);
                loc = format!("ἁ{}", loc);
            } else if loc.starts_with('ἠ')
                && (self.verb.pps[0].starts_with('ἄ') || self.verb.pps[0].starts_with('ἀ'))
            {
                loc.remove(0);
                loc = format!("ἀ{}", loc);
            } else if loc.starts_with('ἡ') {
                return loc;
            } else {
                loc.remove(0);
            }
            loc
        }
    }

    // if middle or passive do not change voice to passive or middle unless tense is aorist or future
    // true to block change, false to allow change
    // AND before OR
    fn block_middle_passive(&self, new_form: &HcGreekVerbForm) -> bool {
        (self.voice == HcVoice::Middle && new_form.voice == HcVoice::Passive
            || self.voice == HcVoice::Passive && new_form.voice == HcVoice::Middle)
            && new_form.tense != HcTense::Aorist
            && new_form.tense != HcTense::Future
            && self.tense != HcTense::Aorist
            && self.tense != HcTense::Future
    }

    fn accent_verb(&self, word: &str) -> String {
        let syllables = analyze_syllable_quantities(
            word,
            self.person,
            self.number,
            self.tense,
            self.mood,
            self.verb.properties,
        );

        let accent;
        let letter_index;
        if syllables.len() > 2 && !syllables.last().unwrap().is_long {
            //acute on antepenult (παιδεύομεν)
            accent = HGK_ACUTE;
            letter_index = syllables[0].index;
        } else if syllables.len() == 2 && syllables[0].is_long && !syllables[1].is_long {
            if (syllables[1].letters == "αι" || syllables[1].letters == "οι")
                && self.mood == HcMood::Optative
            {
                //***we never get here because analyze_syllable_quantities marks optative ai and oi as long
                accent = HGK_ACUTE; //exception to the exception for optative 3rd singular: acute on penult
            } else {
                accent = HGK_CIRCUMFLEX; //circumflex on penult (λῦε present active imperative)
            }
            letter_index = syllables[0].index;
        } else if syllables.len() > 1 {
            //acute on penult (παιδεύω)
            accent = HGK_ACUTE;
            letter_index = syllables[syllables.len() - 2].index;
        } else if syllables.len() == 1 {
            if syllables[0].is_long {
                accent = HGK_CIRCUMFLEX; //circumflex on ultima. e.g. (δοῦ)
            } else {
                accent = HGK_ACUTE; //acute on ultima. e.g. do/s (δός)
            }
            letter_index = syllables[0].index;
        } else {
            return String::from(word);
        }

        self.accent_syllable(word, letter_index, accent)
    }

    fn accent_verb_contracted(
        &self,
        word: &str,
        orig_syllables: Vec<SyllableAnalysis>,
        ending: &str,
    ) -> String {
        let syl = analyze_syllable_quantities(
            word,
            self.person,
            self.number,
            self.tense,
            self.mood,
            self.verb.properties,
        );

        let esyl = analyze_syllable_quantities(
            ending,
            self.person,
            self.number,
            self.tense,
            self.mood,
            self.verb.properties,
        );
        let accent;
        let letter_index;
        if orig_syllables.len() > 2 && !orig_syllables.last().unwrap().is_long {
            if esyl.len() > 2 {
                //has 3 or more syllables
                accent = HGK_ACUTE;
                letter_index = syl[syl.len() - 3].index; //accute on antepenult (ἀδικοιημεν)
            } else if syl.last().unwrap().is_long {
                accent = HGK_ACUTE;
                letter_index = syl[syl.len() - 2].index; //accute on penult (ἀδικει present active imperative)
            } else {
                accent = HGK_CIRCUMFLEX;
                letter_index = syl[syl.len() - 2].index; //circumflex on penult (ἀδικουμεν)
            }
        } else if orig_syllables.len() > 1 {
            //uncontracted word has 2 syllables
            if esyl.len() == 2 && esyl[1].is_long {
                accent = HGK_ACUTE;
                letter_index = syl[syl.len() - 2].index; //acute on penult (ἀδικοιην)
            } else {
                accent = HGK_CIRCUMFLEX;
                letter_index = syl[syl.len() - 1].index; //circumflex on ultima (ἀδικω)
            }
        } else {
            return String::from(word); //(nothing gets here)
        }

        self.accent_syllable(word, letter_index, accent)
    }

    fn accent_syllable(&self, word: &str, letter_index_from_end: u8, accent: u32) -> String {
        let v = word
            .gkletters()
            .rev()
            .enumerate()
            .map(|(x, mut a)| {
                if x == letter_index_from_end as usize {
                    a.toggle_diacritic(accent, true);
                    //println!("abc {:?} {:?} {:?}", a.letter, accent, letter_index_from_end);
                }
                a
            })
            .collect::<Vec<HGKLetter>>();

        let s = v
            .iter()
            .rev()
            .map(|a| a.to_string(HgkUnicodeMode::Precomposed))
            .collect::<String>();
        s
    }

    fn accent_syllable_start(&self, word: &str, letter_index_from_end: u8, accent: u32) -> String {
        let v = word
            .gkletters()
            .enumerate()
            .map(|(x, mut a)| {
                if x == letter_index_from_end as usize {
                    a.toggle_diacritic(accent, true);
                    //println!("abc {:?} {:?} {:?}", a.letter, accent, letter_index_from_end);
                }
                a
            })
            .collect::<Vec<HGKLetter>>();

        let s = v
            .iter()
            .map(|a| a.to_string(HgkUnicodeMode::Precomposed))
            .collect::<String>();
        s
    }

    fn contract_verb(&self, unaccented_form: &str, ending: &str) -> String {
        let mut form =
            hgk_strip_diacritics(unaccented_form, HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE);
        let orig_syl = analyze_syllable_quantities(
            &form,
            self.person,
            self.number,
            self.tense,
            self.mood,
            self.verb.properties,
        );

        if form.contains("εει") {
            // h&q p236
            form = form.replacen("εει", "ει", 1);
        } else if form.contains("εε") {
            form = form.replacen("εε", "ει", 1);
        } else if form.contains("εη") {
            form = form.replacen("εη", "η", 1);
        } else if form.contains("εῃ") {
            form = form.replacen("εῃ", "ῃ", 1);
        } else if form.contains("εοι") {
            form = form.replacen("εοι", "οι", 1);
        } else if form.contains("εου") {
            form = form.replacen("εου", "ου", 1);
        } else if form.contains("εο") {
            form = form.replacen("εο", "ου", 1);
        } else if form.contains("εω") {
            form = form.replacen("εω", "ω", 1);
        } else if form.contains("αει") {
            // h&q p232
            form = form.replacen("αει", "ᾱͅ", 1);
        } else if form.contains("αε") {
            form = form.replacen("αε", "ᾱ", 1);
        } else if form.contains("αη") {
            form = form.replacen("αη", "ᾱ", 1);
        } else if form.contains("αῃ") {
            form = form.replacen("αῃ", "ᾱͅ", 1);
        } else if form.contains("αοι") {
            form = form.replacen("αοι", "ῳ", 1);
        } else if form.contains("αου") {
            form = form.replacen("αου", "ω", 1);
        } else if form.contains("αο") {
            form = form.replacen("αο", "ω", 1);
        } else if form.contains("αω") {
            form = form.replacen("αω", "ω", 1);
        } else if form.contains("οει") {
            // h&q p264
            form = form.replacen("οει", "οι", 1);
        } else if form.contains("οε") {
            form = form.replacen("οε", "ου", 1);
        } else if form.contains("οη") {
            form = form.replacen("οη", "ω", 1);
        } else if form.contains("οῃ") {
            form = form.replacen("οῃ", "οι", 1);
        } else if form.contains("οοι") {
            form = form.replacen("οοι", "οι", 1);
        } else if form.contains("οου") {
            form = form.replacen("οου", "ου", 1);
        } else if form.contains("οο") {
            form = form.replacen("οο", "ου", 1);
        } else if form.contains("οω") {
            form = form.replacen("οω", "ω", 1);
        }

        if self.mood != HcMood::Participle && self.mood != HcMood::Infinitive {
            self.accent_verb_contracted(&form, orig_syl, ending)
        } else {
            form
        }

        //unaccented_form.to_string()
    }
}

impl HcVerbForms for HcGreekVerbForm {
    /*
    fn new() -> HcGreekVerbForm {

    }*/

    fn get_label(&self) -> String {
        "".to_string()
    }

    fn strip_ending(&self, pp_num: usize, form: String) -> Result<String, &str> {
        //println!("form: {}", form);
        match pp_num {
            1..=2 => {
                if form.ends_with('ω') {
                    if self.tense == HcTense::Future
                        && self.voice != HcVoice::Passive
                        && (self.verb.pps[1].ends_with('ῶ')
                            || (form.starts_with("ἐρ") && self.verb.pps[1].starts_with("ἐρῶ")))
                    {
                        if self.verb.pps[1].ends_with("ἐλῶ") {
                            // alpha contracted future: TODO add option to verb, so this is more general
                            if let Some(f) = form.strip_suffix('ω') {
                                return Ok(format!("{}α", f));
                            }
                        } else if let Some(f) = form.strip_suffix('ω') {
                            // epsilon contracted future
                            return Ok(format!("{}ε", f));
                        }
                    } else if let Some(f) = form.strip_suffix('ω') {
                        return Ok(f.to_string());
                    }
                } else if form.ends_with("ουμαι") && self.verb.pps[1].ends_with("οῦμαι")
                {
                    // contracted future
                    return Ok(form.replacen("ουμαι", "ε", 1));
                } else if let Some(f) = form.strip_suffix("ομαι") {
                    return Ok(f.to_string());
                } else if let Some(f) = form.strip_suffix("μαι") {
                    return Ok(f.to_string());
                } else if let Some(f) = form.strip_suffix("μι") {
                    return Ok(f.to_string());
                } else if let Some(f) = form.strip_suffix("τι(ν)") {
                    return Ok(f.to_string());
                } else if let Some(f) = form.strip_suffix("ται") {
                    return Ok(f.to_string());
                } else if form.ends_with("οἰδα") || form.ends_with("οιδα") {
                    return Ok("οἰδ".to_string());
                } else if form.ends_with("δει") {
                    return Ok("δε".to_string());
                } else if form.ends_with("δεησει") {
                    return Ok("δεησ".to_string());
                } else if form.ends_with("χρη") {
                    return Ok("χρ".to_string());
                }
            }
            3 => {
                if let Some(f) = form.strip_suffix("αμην") {
                    return Ok(f.to_string());
                } else if let Some(f) = form.strip_suffix('α') {
                    return Ok(f.to_string());
                } else if let Some(f) = form.strip_suffix("ον") {
                    return Ok(f.to_string());
                } else if let Some(f) = form.strip_suffix("ομην") {
                    return Ok(f.to_string());
                } else if let Some(f) = form.strip_suffix('ν') {
                    return Ok(f.to_string());
                } else if let Some(f) = form.strip_suffix("ε(ν)") {
                    return Ok(f.to_string());
                }
            }
            4 => {
                if let Some(f) = form.strip_suffix('α') {
                    return Ok(f.to_string());
                }
            }
            5 => {
                if let Some(f) = form.strip_suffix("μαι") {
                    return Ok(f.to_string());
                }
            }
            6 => {
                if let Some(f) = form.strip_suffix("ην") {
                    return Ok(f.to_string());
                }
            }
            _ => {
                return Err("error stripping ending 1");
            }
        }
        Err("error stripping ending 2")
    }

    fn is_deponent(&self, stem: &str) -> bool {
        #[allow(clippy::needless_bool)]
        if (self.tense == HcTense::Present
            || self.tense == HcTense::Imperfect
            || self.tense == HcTense::Future)
            && stem.ends_with("μαι")
        {
            true
        } else if self.tense == HcTense::Aorist
            && self.voice != HcVoice::Passive
            && stem.ends_with("άμην")
        {
            true
        } else {
            false
        }
    }

    fn add_ending(
        &self,
        full_stem: &str,
        stem: &str,
        ending: &str,
        decompose: bool,
    ) -> Result<String, &str> {
        let mut local_stem = stem.to_string();
        let mut local_ending = ending.to_string();

        //for contracted verbs remove nu movable for imperfect 3rd sing. active
        if self.tense == HcTense::Imperfect
            && (self.verb.pps[0].ends_with("άω")
                || self.verb.pps[0].ends_with("έω")
                || self.verb.pps[0].ends_with("όω"))
            && self.person == Some(HcPerson::Third)
            && self.number == Some(HcNumber::Singular)
            && self.voice == HcVoice::Active
        {
            local_ending = local_ending.replacen("(ν)", "", 1);
        }

        //add macron to ἀφικνέομαι perfect and pluperfect
        if self.verb.pps[0].ends_with("ἀφικνέομαι")
            && (self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect)
            && self.mood == HcMood::Indicative
            && self.voice != HcVoice::Active
        {
            local_stem = local_stem.replacen('ι', "ῑ", 1);
        }

        if self.verb.pps[0].ends_with("μι") || self.verb.pps[0].ends_with("αμαι") {
            if self.tense == HcTense::Present || self.tense == HcTense::Imperfect {
                if self.number == Some(HcNumber::Plural)
                    || self.mood != HcMood::Indicative
                    || self.voice != HcVoice::Active
                {
                    if self.verb.pps[0].ends_with("ωμι") {
                        local_stem.pop();
                        local_stem.push('ο');
                    } else if self.verb.pps[0].ends_with("στημι") {
                        local_stem.pop();
                        local_stem.push('α');
                    } else if self.verb.pps[0].ends_with("τίθημι")
                        || self.verb.pps[0].ends_with("ῑ̔́ημι")
                        || self.verb.pps[0].ends_with("ῑ́ημι")
                    {
                        local_stem.pop();
                        local_stem.push('ε');

                        if (self.verb.pps[0].ends_with("ῑ̔́ημι")
                            || self.verb.pps[0].ends_with("ῑ́ημι"))
                            && self.tense == HcTense::Present
                            && self.person == Some(HcPerson::Third)
                            && self.number == Some(HcNumber::Plural)
                            && self.voice == HcVoice::Active
                            && self.mood == HcMood::Indicative
                        {
                            if !decompose {
                                local_stem.pop();
                            }
                            local_ending = if decompose {
                                String::from("ᾱσι(ν)")
                            } else {
                                String::from("ᾶσι(ν)")
                            };
                        }
                    } else if self.verb.pps[0].ends_with("ῡμι") {
                        local_stem = local_stem.replacen('ῡ', "υ", 1);
                    }
                }
            }

            if self.tense == HcTense::Present {
                if self.voice == HcVoice::Active {
                    if self.mood == HcMood::Subjunctive {
                        if !decompose {
                            if self.verb.pps[0].ends_with("ωμι") {
                                // didwmi / gignwskw subjunctive contraction
                                if local_ending.contains('ῇ') {
                                    local_ending = local_ending.replacen('ῇ', "ῷ", 1);
                                } else if local_ending.contains('ῆ') {
                                    local_ending = local_ending.replacen('ῆ', "ῶ", 1);
                                }
                            }

                            if !self.verb.pps[0].ends_with("ῡμι") {
                                local_stem.pop();
                            }
                        } else {
                            //isthmi subjunctive stem
                            if self.verb.pps[0].ends_with("στημι") {
                                local_stem.pop();
                                local_stem.push('ε');
                            }
                        }
                    } else if self.mood == HcMood::Imperative {
                        if decompose {
                            if !(local_ending.is_empty()
                                || self.person == Some(HcPerson::Second)
                                    && self.number == Some(HcNumber::Singular))
                            {
                                local_ending.remove(0);
                            } else if self.verb.pps[0].ends_with("ῡμι") {
                                local_stem = local_stem.replacen('υ', "ῡ", 1); //fix me
                                local_ending = String::from(""); // fix me
                            }
                        } else if self.person == Some(HcPerson::Second)
                            && self.number == Some(HcNumber::Singular)
                        {
                            if self.verb.pps[0].ends_with("ωμι") {
                                local_ending = String::from("υ");
                            } else if self.verb.pps[0].ends_with("στημι") {
                                local_stem.pop();
                                local_ending = String::from("η");
                            } else if self.verb.pps[0].ends_with("ῡμι") {
                                local_stem = local_stem.replacen('υ', "ῡ", 1);
                                local_ending = String::from("");
                            } else {
                                local_ending = String::from("ι");
                            }
                        } else if !local_ending.is_empty() {
                            local_ending.remove(0);
                        }
                    } else if self.verb.pps[0].ends_with("στημι")
                        && self.person == Some(HcPerson::Third)
                        && self.number == Some(HcNumber::Plural)
                        && self.mood == HcMood::Indicative
                        && !decompose
                    {
                        local_stem.pop();
                        local_ending = local_ending.replacen("ᾱ", "ᾶ", 1);
                    }
                } else {
                    // middle/passive
                    if self.mood == HcMood::Subjunctive {
                        if !decompose {
                            if !self.verb.pps[0].ends_with("ῡμι") {
                                local_stem.pop();
                            }
                            if self.verb.pps[0].ends_with("ωμι") {
                                // didwmi / gignwskw subjunctive contraction
                                if local_ending.contains('ῃ') {
                                    local_ending = local_ending.replacen('ῃ', "ῷ", 1);
                                } else if local_ending.contains('η') {
                                    local_ending = local_ending.replacen('η', "ῶ", 1);
                                }
                            }

                            if local_ending != "ωμεθα"
                                && !self.verb.pps[0].ends_with("ῡμι")
                                && !self.verb.pps[0].ends_with("δύναμαι")
                                && !self.verb.pps[0].ends_with("ἐπίσταμαι")
                            {
                                local_ending =
                                    self.accent_syllable_start(&local_ending, 0, HGK_CIRCUMFLEX);
                            }
                        } else {
                            //isthmi subjunctive stem
                            if self.verb.pps[0].ends_with("δύναμαι")
                                || self.verb.pps[0].ends_with("ἐπίσταμαι")
                            {
                                local_stem.pop();
                            } else if self.verb.pps[0].ends_with("στημι")
                                || self.verb.pps[0].ends_with("αμαι")
                            {
                                local_stem.pop();
                                local_stem.push('ε');
                            }
                        }
                    } else if self.mood == HcMood::Optative {
                        if !decompose {
                            if self.verb.pps[0].ends_with("δύναμαι")
                                || self.verb.pps[0].ends_with("ἐπίσταμαι")
                            {
                                local_ending = hgk_strip_diacritics(
                                    &local_ending,
                                    HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE,
                                );
                            }
                            if local_ending.starts_with('ο') && !self.verb.pps[0].ends_with("ῡμι")
                            {
                                //alt endings for tithhmi and ihmi
                                local_stem.pop();
                            }
                        }
                    }
                }
            } else if self.tense == HcTense::Imperfect {
                if self.verb.pps[0].ends_with("ωμι") {
                    if self.number == Some(HcNumber::Singular) {
                        if decompose {
                            local_stem = local_stem.replacen('ω', "ο", 1); //use short stem when using thematic endings
                            if self.person == Some(HcPerson::First) && self.voice == HcVoice::Active
                            {
                                local_ending = local_ending.replacen('ν', "ον", 1);
                            } else {
                                local_ending = local_ending.replacen('ς', "ες", 1);
                                if self.person == Some(HcPerson::Third)
                                    && self.voice == HcVoice::Active
                                {
                                    local_ending = String::from("ε");
                                }
                            }
                        } else {
                            local_stem = local_stem.replacen('ω', "ου", 1);
                        }
                    }
                } else if self.verb.pps[0].ends_with("τίθημι")
                    || self.verb.pps[0].ends_with("ῑ̔́ημι")
                    || self.verb.pps[0].ends_with("ῑ́ημι")
                {
                    if (self.person == Some(HcPerson::Second)
                        || self.person == Some(HcPerson::Third))
                        && self.number == Some(HcNumber::Singular)
                    {
                        if decompose {
                            local_stem = local_stem.replacen('η', "ε", 1); //use short stem when using thematic endings
                            local_ending = local_ending.replacen('ς', "ες", 1);
                            if self.person == Some(HcPerson::Third) && self.voice == HcVoice::Active
                            {
                                local_ending = String::from("ε");
                            }
                        } else {
                            local_stem = local_stem.replacen('η', "ει", 1);
                        }
                    }
                }
                if (self.verb.pps[0] == "δύναμαι" || self.verb.pps[0] == "ἐπίσταμαι")
                    && self.tense == HcTense::Imperfect
                    && self.person == Some(HcPerson::Second)
                    && self.number == Some(HcNumber::Singular)
                {
                    if decompose {
                        local_ending = String::from("ο"); //fix me
                    } else {
                        local_stem.pop();
                        local_ending = String::from("ω");
                    }
                }
            } else if self.tense == HcTense::Aorist {
                //mixed aorist
                if self.verb.pps[2].ends_with("κα")
                    && (self.number == Some(HcNumber::Plural)
                        || self.mood != HcMood::Indicative
                        || self.voice != HcVoice::Active)
                {
                    if self.verb.pps[0].ends_with("δίδωμι") {
                        local_stem = local_stem.replacen("ωκ", "ο", 1);
                    } else if self.verb.pps[0].ends_with("τίθημι")
                        || self.verb.pps[0].ends_with("ῑ̔́ημι")
                        || self.verb.pps[0].ends_with("ῑ́ημι")
                    {
                        if self.verb.pps[0].ends_with("ῑ́ημι")
                            && !decompose
                            && (self.number == Some(HcNumber::Plural)
                                || self.voice != HcVoice::Active)
                        {
                            local_stem = local_stem.replacen("ηκ", "ει", 1);
                        } else if self.verb.pps[0].ends_with("ῑ̔́ημι") && !decompose {
                            local_stem = local_stem.replacen("ἡκ", "εἱ", 1);
                        } else {
                            local_stem = local_stem.replacen("ηκ", "ε", 1);
                        }
                    }

                    if self.mood == HcMood::Subjunctive
                        && !decompose
                        && self.voice != HcVoice::Passive
                    {
                        local_stem.pop();
                    }

                    if self.voice == HcVoice::Active {
                        if self.mood != HcMood::Indicative {
                            if !decompose {
                                if self.mood == HcMood::Subjunctive {
                                    if self.verb.pps[0].ends_with("ωμι") {
                                        // didwmi / gignwskw subjunctive contraction
                                        if local_ending.contains('ῃ') {
                                            local_ending = local_ending.replacen('ῃ', "ῷ", 1);
                                        } else if local_ending.contains('η') {
                                            local_ending = local_ending.replacen('η', "ῶ", 1);
                                        }
                                    } else if self.verb.pps[0].ends_with("ῑ̔́ημι") {
                                        let (stem, ending) = match (self.person, self.number) {
                                            (Some(HcPerson::First), Some(HcNumber::Singular)) => {
                                                ("-", "ὡ")
                                            }
                                            (Some(HcPerson::Second), Some(HcNumber::Singular)) => {
                                                ("-", "ᾑς")
                                            }
                                            (Some(HcPerson::Third), Some(HcNumber::Singular)) => {
                                                ("-", "ᾑ")
                                            }
                                            (Some(HcPerson::First), Some(HcNumber::Plural)) => {
                                                ("-", "ὡμεν")
                                            }
                                            (Some(HcPerson::Second), Some(HcNumber::Plural)) => {
                                                ("-", "ἡτε")
                                            }
                                            (Some(HcPerson::Third), Some(HcNumber::Plural)) => {
                                                ("-", "ὡσι(ν)")
                                            }
                                            _ => ("", ""),
                                        };
                                        local_stem = stem.to_string();
                                        local_ending = ending.to_string();
                                    }
                                    local_ending = self.accent_syllable_start(
                                        &local_ending,
                                        0,
                                        HGK_CIRCUMFLEX,
                                    );
                                } else if self.mood == HcMood::Imperative {
                                    // ana/thes
                                    if self.verb.pps[0].ends_with("ἀνατίθημι")
                                        && self.person == Some(HcPerson::Second)
                                        && self.number == Some(HcNumber::Singular)
                                    {
                                        local_stem =
                                            self.accent_syllable(&local_stem, 2, HGK_ACUTE);
                                    }
                                    // apo/dos
                                    else if self.verb.pps[0].ends_with("ἀποδίδωμι")
                                        && self.person == Some(HcPerson::Second)
                                        && self.number == Some(HcNumber::Singular)
                                    {
                                        local_stem =
                                            self.accent_syllable(&local_stem, 2, HGK_ACUTE);
                                    } else if self.verb.pps[0].ends_with("μεταδίδωμι")
                                        && self.person == Some(HcPerson::Second)
                                        && self.number == Some(HcNumber::Singular)
                                    {
                                        local_stem =
                                            self.accent_syllable(&local_stem, 2, HGK_ACUTE);
                                    } else if self.verb.pps[0].ends_with("παραδίδωμι")
                                        && self.person == Some(HcPerson::Second)
                                        && self.number == Some(HcNumber::Singular)
                                    {
                                        local_stem =
                                            self.accent_syllable(&local_stem, 2, HGK_ACUTE);
                                    }
                                }
                            }
                            if self.mood == HcMood::Optative {
                                local_ending.remove(0);
                                if self.verb.pps[0].ends_with("ῑ̔́ημι") && !decompose {
                                    local_ending.remove(0);
                                    local_stem = "-εἱ".to_string();
                                }
                            }
                        }
                    } else if self.voice == HcVoice::Middle {
                        if self.mood == HcMood::Indicative {
                            if (self.verb.pps[0].ends_with("ῑ̔́ημι")
                                || self.verb.pps[0].ends_with("ῑ́ημι"))
                                && self.person == Some(HcPerson::Second)
                                && self.number == Some(HcNumber::Singular)
                            {
                                local_ending = String::from("σο");
                            } else {
                                local_ending.remove(0);
                                if self.person == Some(HcPerson::Second)
                                    && self.number == Some(HcNumber::Singular)
                                {
                                    if decompose {
                                        local_ending = String::from("ο");
                                    } else if local_stem.ends_with('ε') {
                                        local_stem = local_stem.rreplacen("ε", "ο", 1);
                                    }
                                }
                            }
                        } else if self.mood == HcMood::Subjunctive {
                            if self.verb.pps[0].ends_with("ωμι") && !decompose {
                                // didwmi / gignwskw subjunctive contraction
                                if local_ending.contains('ῃ') {
                                    local_ending = local_ending.replacen('ῃ', "ῷ", 1);
                                } else if local_ending.contains('η') {
                                    local_ending = local_ending.replacen('η', "ῶ", 1);
                                }
                            } else if self.verb.pps[0].ends_with("ῑ̔́ημι") && !decompose {
                                let (stem, ending) = match (self.person, self.number) {
                                    (Some(HcPerson::First), Some(HcNumber::Singular)) => {
                                        ("-", "ὡμαι")
                                    }
                                    (Some(HcPerson::Second), Some(HcNumber::Singular)) => {
                                        ("-", "ᾑ")
                                    }
                                    (Some(HcPerson::Third), Some(HcNumber::Singular)) => {
                                        ("-", "ἡται")
                                    }
                                    (Some(HcPerson::First), Some(HcNumber::Plural)) => {
                                        ("-", "ὡμεθα")
                                    }
                                    (Some(HcPerson::Second), Some(HcNumber::Plural)) => {
                                        ("-", "ἡσθε")
                                    }
                                    (Some(HcPerson::Third), Some(HcNumber::Plural)) => {
                                        ("-", "ὡνται")
                                    }
                                    _ => ("", ""),
                                };

                                local_stem = stem.to_string();
                                local_ending = ending.to_string();
                            }
                            if !decompose && local_ending != "ωμεθα" && local_ending != "ὡμεθα"
                            {
                                local_ending =
                                    self.accent_syllable_start(&local_ending, 0, HGK_CIRCUMFLEX);
                            }
                        } else if self.mood == HcMood::Optative {
                            if !decompose {
                                if self.verb.pps[0].ends_with("ῑ̔́ημι") {
                                    if local_ending.starts_with('ο') {
                                        local_ending.remove(0);
                                        local_ending.remove(0);
                                        local_stem = "-οἱ".to_string();
                                    } else {
                                        local_ending.remove(0);
                                        local_stem = "-εἱ".to_string();
                                    }
                                } else if local_ending.starts_with('ο') {
                                    local_stem.pop();
                                }
                            }
                        } else if self.mood == HcMood::Imperative {
                            if self.person == Some(HcPerson::Second)
                                && self.number == Some(HcNumber::Singular)
                            {
                                if decompose {
                                    if !self.verb.pps[0].ends_with("ῑ́ημι")
                                        && !self.verb.pps[0].ends_with("ῑ̔́ημι")
                                    {
                                        local_ending.remove(0);
                                    } else {
                                        local_ending = local_ending.replacen("σο", "ου", 1);
                                    }
                                } else {
                                    local_stem.pop();

                                    if local_stem.starts_with("προ")
                                        || self.verb.pps[0].ends_with("ῑ́ημι")
                                    {
                                        local_ending = local_ending.replacen("σο", "οῦ", 1);
                                    } else if self.verb.pps[0].ends_with("ῑ̔́ημι") {
                                        local_ending = local_ending.replacen("σο", "οὗ", 1);
                                    } else {
                                        local_ending = local_ending.replacen("σο", "ου", 1);
                                    }
                                }
                            }
                        }
                    }
                }
            } else if self.tense == HcTense::Perfect {
                if self.number == Some(HcNumber::Plural) && local_stem.ends_with("στηκ") {
                    local_stem = local_stem.replacen("ηκ", "α", 1);
                    if self.person == Some(HcPerson::Third) {
                        if decompose {
                        } else {
                            local_stem.pop();
                            local_ending = local_ending.replacen("ᾱ", "ᾶ", 1);
                        }
                    } else {
                        local_ending.remove(0);
                    }
                }
            } else if self.tense == HcTense::Pluperfect {
                if self.number == Some(HcNumber::Plural) && local_stem.ends_with("στηκ") {
                    local_stem = local_stem.replacen("ηκ", "α", 1);
                    local_ending.remove(0);
                }
            }
        }

        // root aorist
        if (self.tense == HcTense::Aorist && self.voice == HcVoice::Active)
            && local_stem.ends_with("στη")
            || local_stem.ends_with("φθη")
            || local_stem.ends_with("βη")
            || local_stem.ends_with("γνω")
        {
            if self.mood == HcMood::Subjunctive {
                if decompose {
                    if local_stem.ends_with("γνω") {
                        local_stem.pop();
                        local_stem.push('ο');
                    } else {
                        local_stem.pop();
                        local_stem.push('ε');
                    }
                } else {
                    if local_stem.ends_with("γνω") {
                        // didwmi / gignwskw subjunctive contraction
                        if local_ending.contains('ῇ') {
                            local_ending = local_ending.replacen('ῇ', "ῷ", 1);
                        } else if local_ending.contains('ῆ') {
                            local_ending = local_ending.replacen('ῆ', "ῶ", 1);
                        }
                    }
                    local_stem.pop();
                }
            } else if self.mood == HcMood::Optative {
                if local_stem.ends_with("γνω") {
                    local_stem.pop();
                    local_stem.push('ο');
                } else {
                    local_stem.pop();
                    local_stem.push('α');
                }
            } else if self.mood == HcMood::Imperative {
                if self.person == Some(HcPerson::Second)
                    && self.number == Some(HcNumber::Singular)
                    && local_stem.ends_with("φθη")
                {
                    local_ending = local_ending.replacen('θ', "τ", 1);
                } else if self.person == Some(HcPerson::Third)
                    && self.number == Some(HcNumber::Plural)
                {
                    if local_stem.ends_with("γνω") {
                        local_stem.pop();
                        local_stem.push('ο');
                    } else {
                        local_stem.pop();
                        local_stem.push('α');
                    }
                }
            }
        }

        // consonant stem perfects and pluperfects
        if self.verb.is_consonant_stem(full_stem)
            && (self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect)
            && (self.voice == HcVoice::Middle || self.voice == HcVoice::Passive)
        {
            return Ok(self.contract_consonants(&local_stem, &local_ending, decompose));
        }

        //future passive
        let future_passive_suffix =
            if self.tense == HcTense::Future && self.voice == HcVoice::Passive {
                if decompose {
                    format!("ησ {} ", SEPARATOR)
                } else {
                    String::from("ησ")
                }
            } else {
                String::from("")
            };

        if self.verb.pps[0].ends_with("ἔχω")
            && self.person == Some(HcPerson::Second)
            && self.number == Some(HcNumber::Singular)
            && self.tense == HcTense::Aorist
            && self.mood == HcMood::Imperative
            && self.voice == HcVoice::Active
        {
            local_ending = String::from("ες");
        }

        if decompose {
            Ok(format!(
                "{} {} {}{}",
                local_stem, SEPARATOR, future_passive_suffix, local_ending
            ))
        } else {
            //come take see say find: elthe/ labe/ ide/ eipe/ eyre/
            if local_ending == "ε"
                && (local_stem == "ἐλθ"
                    || local_stem == "λαβ"
                    || local_stem == "ἰδ"
                    || local_stem == "εἰπ"
                    || local_stem == "εὑρ")
            {
                local_ending = "έ".to_string();
            }

            Ok(format!(
                "{}{}{}",
                local_stem, future_passive_suffix, local_ending
            ))
        }
    }

    fn block_for_hq_unit(&self, unit: Option<i16>) -> bool {
        match unit {
            Some(unit) => {
                let is_mi_verb = self.verb.pps[0].ends_with("μι");
                let is_isthmi = self.verb.pps[0].ends_with("στημι");

                let is_isthmi_perf = is_isthmi
                    && (self.tense == HcTense::Aorist
                        || self.tense == HcTense::Perfect
                        || self.tense == HcTense::Pluperfect);

                let is_future_optative =
                    self.tense == HcTense::Future && self.mood == HcMood::Optative;

                let is_consonant_stem_third_plural = self.verb.is_consonant_stem("")
                    && (self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect)
                    && (self.voice == HcVoice::Middle || self.voice == HcVoice::Passive)
                    && self.person == Some(HcPerson::Third)
                    && self.number == Some(HcNumber::Plural);

                if unit <= 2 {
                    //2 and under active indicative and not perfect or pluperfect
                    if self.tense == HcTense::Perfect
                        || self.tense == HcTense::Pluperfect
                        || self.voice != HcVoice::Active
                        || self.mood != HcMood::Indicative
                        || is_mi_verb
                    {
                        return true;
                    }
                } else if unit <= 4 {
                    //4 and under must be active, no imperatives, no future optative
                    if self.voice != HcVoice::Active
                        || self.mood == HcMood::Imperative
                        || is_mi_verb
                        || is_future_optative
                    {
                        return true;
                    }
                } else if unit <= 6 {
                    //6 and under can't be middle, no imperatives, no future optative
                    if self.voice == HcVoice::Middle
                        || self.mood == HcMood::Imperative
                        || is_mi_verb
                        || is_future_optative
                        || is_consonant_stem_third_plural
                    {
                        return true;
                    }
                } else if unit <= 10 {
                    //10 and under no imperatives, no future optative
                    if self.mood == HcMood::Imperative
                        || is_mi_verb
                        || is_future_optative
                        || is_consonant_stem_third_plural
                    {
                        return true;
                    }
                } else if unit <= 11 {
                    //11 and under no aorists of mi verbs, no perf/plup of isthmi, no future optative
                    if is_mi_verb || is_future_optative || is_consonant_stem_third_plural {
                        return true;
                    }
                } else if unit <= 12 {
                    //12 and under no aorists of mi verbs, no perf/plup of isthmi, no future optative
                    if (is_mi_verb && self.tense == HcTense::Aorist)
                        || is_isthmi_perf
                        || is_future_optative
                        || is_consonant_stem_third_plural
                    {
                        return true;
                    }
                    // todo deiknumi verbs?
                } else if unit <= 15 {
                    //15 and under no future optative
                    if is_future_optative || is_consonant_stem_third_plural {
                        return true;
                    }
                } else if unit <= 19 {
                    //19 and under no 3rd plural of consonant stem perf/plup mid/pass
                    if is_consonant_stem_third_plural {
                        return true;
                    }
                }
                false
            }
            None => false,
        }
    }

    //only call on finite verbs, maybe change to return Option<u32> to handle non-finites?
    fn param_hash(&self) -> u32 {
        let p_count = 3;
        let n_count = 2;
        let t_count = 6;
        let m_count = 4;

        let voice = self.voice.to_i16();
        let mood = self.mood.to_i16();
        let tense = self.tense.to_i16();
        let number = if self.number.is_some() {
            self.number.unwrap().to_i16()
        } else {
            2 //panic!() //add an extra number, in case of None: it just has to be unique
        };
        let person = if self.person.is_some() {
            self.person.unwrap().to_i16()
        } else {
            3 //panic!() //add an extra number, in case of None: it just has to be unique
        };

        //calculate unique hash from param values
        (voice * m_count * t_count * n_count * p_count
            + mood * t_count * n_count * p_count
            + tense * n_count * p_count
            + number * p_count
            + person)
            .try_into()
            .unwrap()
    }

    fn extract_params_from_hash(&mut self, value: u32) {
        let p_count = 3;
        let n_count = 2;
        let t_count = 6;
        let m_count = 4;

        let voice = value / (m_count * t_count * n_count * p_count);
        let remaining = value % (m_count * t_count * n_count * p_count);

        let mood = remaining / (t_count * n_count * p_count);
        let remaining = remaining % (t_count * n_count * p_count);

        let tense = remaining / (n_count * p_count);
        let remaining = remaining % (n_count * p_count);

        let number = remaining / p_count;
        let person = remaining % p_count;

        self.person = Some(HcPerson::from_i16(person.try_into().unwrap()));
        self.number = Some(HcNumber::from_i16(number.try_into().unwrap()));
        self.tense = HcTense::from_i16(tense.try_into().unwrap());
        self.mood = HcMood::from_i16(mood.try_into().unwrap());
        self.voice = HcVoice::from_i16(voice.try_into().unwrap());

        //(person, number, tense, mood, voice)
    }

    // add param for top unit
    fn random_form(
        &self,
        num_changes: u8,
        highest_unit: Option<i16>,
        parameters: &VerbParameters,
        filter_forms: Option<&HashSet<u32>>, //previously used forms we don't want to return
    ) -> (HcGreekVerbForm, Diagnostics) {
        let mut pf: HcGreekVerbForm;
        let mut num_skipped = 0;
        let mut ignore_filter = false;

        let mut diag = Diagnostics {
            dash: 0,
            middle_passive: 0,
            blocked_for_unit: 0,
            filtered: 0,
            illegal: 0,
        };

        loop {
            pf = self.clone();
            pf.change_params(
                num_changes,
                parameters,
                &mut [], //HcParameters::Person, HcParameters::Number
            );
            let vf = pf.get_form(false);
            if num_skipped > 2000 {
                // println!(
                //     "AAABBB error: {}",
                //     if filter_forms.is_some() {
                //         filter_forms.unwrap().len()
                //     } else {
                //         222
                //     }
                // );
                error!("random form 2000 cycles");
                ignore_filter = true;
            } else if num_skipped > 4000 {
                // println!(
                //     "AAABBB2 error: {}",
                //     if filter_forms.is_some() {
                //         filter_forms.unwrap().len()
                //     } else {
                //         222
                //     }
                // );
                error!("random form 4000 cycles");
                break;
            }
            num_skipped += 1;
            match vf {
                Ok(res) => {
                    if res.last().unwrap().form == "—"
                        || self.block_middle_passive(&pf)
                        || pf.block_for_hq_unit(highest_unit)
                        || (filter_forms.is_some()
                            && !ignore_filter
                            && filter_forms.unwrap().contains(&pf.param_hash()))
                    {
                        if self.block_middle_passive(&pf) {
                            diag.middle_passive += 1;
                        } else if pf.block_for_hq_unit(highest_unit) {
                            diag.blocked_for_unit += 1;
                        } else if filter_forms.is_some()
                            && !ignore_filter
                            && filter_forms.unwrap().contains(&pf.param_hash())
                        {
                            diag.filtered += 1;
                        } else if res.last().unwrap().form == "—" {
                            diag.dash += 1;
                        }

                        let _reason = if self.block_middle_passive(&pf) {
                            String::from("middle/passive just used")
                        } else if pf.block_for_hq_unit(highest_unit) {
                            format!("not in unit: {:?}", highest_unit)
                        } else if filter_forms.is_some()
                            && !ignore_filter
                            && filter_forms.unwrap().contains(&pf.param_hash())
                        {
                            "already used".to_string()
                        } else if res.last().unwrap().form == "—" {
                            format!("block bad form {:?}", res)
                        } else {
                            String::from("unknown reason")
                        };
                        // println!(
                        //     "\t{}: {:?} {:?}",
                        //     num_skipped,
                        //     res.last().unwrap().form,
                        //     reason
                        // );

                        continue;
                    } else {
                        //println!("{}", res.last().unwrap().form);
                        break;
                    }
                } //only 3rd pl consonant stem perfects/pluperfects return - now
                Err(_e) => {
                    diag.illegal += 1;
                    //println!("\t{}: {:?}", num_skipped, e);
                    continue;
                }
            }
        }
        (pf, diag)
    }

    // num params to change must be equal or less than num params with more than one value
    // params_do_not_change: pass in params from last change, so we don't change the same ones again
    fn change_params(
        &mut self,
        n_params_to_change: u8,
        parameters: &VerbParameters,
        params_do_not_change: &mut [HcParameters],
    ) -> Vec<HcParameters> {
        let mut possible_params = vec![
            HcParameters::Person,
            HcParameters::Number,
            HcParameters::Tense,
            HcParameters::Mood,
            HcParameters::Voice,
        ];

        if parameters.persons.len() == 1 {
            self.person = Some(parameters.persons[0]);
            possible_params.retain(|e| *e != HcParameters::Person);
        }
        if parameters.numbers.len() == 1 {
            self.number = Some(parameters.numbers[0]);
            possible_params.retain(|e| *e != HcParameters::Number);
        }
        if parameters.tenses.len() == 1 {
            self.tense = parameters.tenses[0];
            possible_params.retain(|e| *e != HcParameters::Tense);
        }
        if parameters.moods.len() == 1 {
            self.mood = parameters.moods[0];
            possible_params.retain(|e| *e != HcParameters::Mood);
        }
        if parameters.voices.len() == 1 {
            self.voice = parameters.voices[0];
            possible_params.retain(|e| *e != HcParameters::Voice);
        }

        if self.person.is_none() || self.number.is_none() || possible_params.is_empty() {
            return vec![];
        }

        let mut rng = rand::thread_rng();

        if !params_do_not_change.is_empty() && possible_params.len() > 1 {
            params_do_not_change.shuffle(&mut rng); //shuffle, so not always first param
            if let Some(aa) = params_do_not_change.first() {
                possible_params.retain(|e| *e != *aa);
            }
        }

        possible_params.shuffle(&mut rng);
        possible_params.truncate(n_params_to_change.into());

        for p in &possible_params {
            match p {
                HcParameters::Person => {
                    self.person = Some(
                        **parameters
                            .persons
                            .iter()
                            .filter(|x| **x != self.person.unwrap())
                            .collect::<Vec<_>>()
                            .choose(&mut rand::thread_rng())
                            .unwrap(),
                    );
                }
                HcParameters::Number => {
                    self.number = Some(
                        **parameters
                            .numbers
                            .iter()
                            .filter(|x| **x != self.number.unwrap())
                            .collect::<Vec<_>>()
                            .choose(&mut rand::thread_rng())
                            .unwrap(),
                    );
                }
                HcParameters::Tense => {
                    self.tense = **parameters
                        .tenses
                        .iter()
                        .filter(|x| **x != self.tense)
                        .collect::<Vec<_>>()
                        .choose(&mut rand::thread_rng())
                        .unwrap();
                }
                HcParameters::Voice => {
                    self.voice = **parameters
                        .voices
                        .iter()
                        .filter(|x| **x != self.voice)
                        .collect::<Vec<_>>()
                        .choose(&mut rand::thread_rng())
                        .unwrap();
                }
                HcParameters::Mood => {
                    self.mood = **parameters
                        .moods
                        .iter()
                        .filter(|x| **x != self.mood)
                        .collect::<Vec<_>>()
                        .choose(&mut rand::thread_rng())
                        .unwrap();
                }
            }
        }
        possible_params
    }

    fn get_description(&self, p: &HcGreekVerbForm, start: &str, end: &str) -> String {
        let mut desc = String::new();
        //let start = "<span foreground=\"red\"><b>";
        //let end = "</b></span>";
        if p.person != self.person {
            desc = format!("{} {}{:?}{}", desc, start, self.person, end);
        } else {
            desc = format!("{} {:?}", desc, self.person);
        }

        if p.number != self.number {
            desc = format!("{} {}{:?}{}", desc, start, self.number, end);
        } else {
            desc = format!("{} {:?}", desc, self.number);
        }

        if p.tense != self.tense {
            desc = format!("{} {}{:?}{}", desc, start, self.tense, end);
        } else {
            desc = format!("{} {:?}", desc, self.tense);
        }

        if p.mood != self.mood {
            desc = format!("{} {}{:?}{}", desc, start, self.mood, end);
        } else {
            desc = format!("{} {:?}", desc, self.mood);
        }

        if p.voice != self.voice {
            desc = format!("{} {}{:?}{}", desc, start, self.voice, end);
        } else {
            desc = format!("{} {:?}", desc, self.voice);
        }

        desc
    }

    fn is_legal_form(&self) -> bool {
        //eliminate first person imperatives
        //eliminate subjunctive and imperative outside of the present and aorist
        //and optative outside of the present and aorist and future
        //except for oida in perfect tense
        #[allow(clippy::needless_bool)]
        if self.number == Some(HcNumber::Dual) && self.person == Some(HcPerson::First) {
            false
        } else if self.mood == HcMood::Imperative && self.person == Some(HcPerson::First) {
            false
        } else if (self.mood == HcMood::Subjunctive || self.mood == HcMood::Imperative)
            && self.tense != HcTense::Present
            && self.tense != HcTense::Aorist
            && !(self.verb.pps[0].ends_with("δα") && self.tense == HcTense::Perfect)
        {
            false
        } else if self.mood == HcMood::Optative
            && self.tense != HcTense::Present
            && self.tense != HcTense::Aorist
            && self.tense != HcTense::Future
            && !(self.verb.pps[0].ends_with("δα") && self.tense == HcTense::Perfect)
        {
            false
        } else if self.mood == HcMood::Infinitive
            && (self.person.is_some()
                || self.number.is_some()
                || self.gender.is_some()
                || self.case.is_some())
        {
            false //infinitive must not have person, number, gender, or case
        } else if self.mood == HcMood::Participle
            && (self.person.is_some()
                || self.number.is_none()
                || self.gender.is_none()
                || self.case.is_none())
        {
            false //ptc must not have a person, but must have gender, number, case
        } else if self.mood != HcMood::Participle
            && self.mood != HcMood::Infinitive
            && (self.person.is_none()
                || self.number.is_none()
                || self.gender.is_some()
                || self.case.is_some())
        {
            false //finite must have a person and number, but must not have a gender or case
        } else {
            true
        }
    }

    fn is_legal_deponent(&self, pp: &str) -> bool {
        if self.voice == HcVoice::Active && self.is_deponent(pp) {
            return false;
        }

        //block future passive for passive deponents
        if self.verb.deponent_type() == HcDeponentType::PassiveDeponent
            && self.tense == HcTense::Future
            && self.voice == HcVoice::Passive
        {
            return false;
        }

        //abd
        //no passive for middle deponent present or imperfect
        //this does not need to be done for future, aorist because from different pp,
        if self.voice == HcVoice::Passive
            && (self.tense == HcTense::Present || self.tense == HcTense::Imperfect)
            && self.verb.pps[0].ends_with("μαι")
        {
            return false;
        }

        //for perfect and pluperfect we need to block passive if middle or passive deponent
        if self.voice == HcVoice::Passive
            && (self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect)
            && (self.verb.deponent_type() == HcDeponentType::MiddleDeponent
                || self.verb.deponent_type() == HcDeponentType::PassiveDeponent
                || self.verb.deponent_type() == HcDeponentType::MiddleDeponentHgeomai)
        {
            return false;
        }

        //middle deponents do not have a passive voice.  H&Q page 316
        if self.voice == HcVoice::Passive
            && (self.verb.deponent_type() == HcDeponentType::MiddleDeponent
                || self.verb.deponent_type() == HcDeponentType::GignomaiDeponent)
        {
            return false;
        }

        if self.voice == HcVoice::Active
            && (self.verb.deponent_type() == HcDeponentType::MiddleDeponent
                || self.verb.deponent_type() == HcDeponentType::PassiveDeponent)
            && !self.verb.pps[2].ends_with("στην")
        {
            return false;
        }

        if self.voice == HcVoice::Active
            && self.tense != HcTense::Perfect
            && self.tense != HcTense::Pluperfect
            && self.verb.deponent_type() == HcDeponentType::GignomaiDeponent
        {
            return false;
        }
        /*
        if (vf->voice == PASSIVE && deponentType(vf->verb) == PASSIVE_DEPONENT && (vf->tense == PRESENT || vf->tense == IMPERFECT || vf->tense == PERFECT || vf->tense == PLUPERFECT)) //aorist or future are ok
        {
            return 0;
        }
        */
        true
    }

    fn get_form(&self, decompose: bool) -> Result<Vec<Step>, HcFormError> {
        //0 is form valid?
        if !self.is_legal_form() {
            return Err(HcFormError::IllegalForm);
        }
        //first person dual is eliminated in is_legal_form() above
        if self.number == Some(HcNumber::Dual) && self.person != Some(HcPerson::First) {
            return Err(HcFormError::NotImplemented);
        }

        let mut steps = Vec::new();

        let f = self.verb.pps.join(", ");
        let e = "Principal Parts".to_string();
        steps.push(Step {
            form: f,
            explanation: e,
        });

        //1 get pp
        let pp_num = self.get_pp_num() as usize;
        let f = &self.verb.pps[pp_num - 1];
        let e = "Choose Principal Part".to_string();
        steps.push(Step {
            form: f.to_string(),
            explanation: e,
        });

        if f == BLANK {
            return Err(HcFormError::BlankPrincipalPartForForm);
        }

        //2 is legal deponent?
        if !self.is_legal_deponent(f) {
            return Err(HcFormError::Deponent);
        }

        //3 special verb?
        if self.verb.pps[0] == "δεῖ" {
            let fff = special_verbs::get_dei(self, decompose);
            if fff.is_empty() {
                return Err(HcFormError::IllegalForm);
            }
            steps.push(Step {
                form: fff,
                explanation: String::from("def"),
            });
            return Ok(steps);
        } else if self.verb.pps[0] == "χρή" {
            let fff = special_verbs::get_xrh(self, decompose);
            if fff.is_empty() {
                return Err(HcFormError::IllegalForm);
            }
            steps.push(Step {
                form: fff,
                explanation: String::from("def"),
            });
            return Ok(steps);
        } else if self.verb.pps[0] == "ἔστι(ν)" {
            let fff = special_verbs::get_esti(self, decompose);
            if fff.is_empty() {
                return Err(HcFormError::IllegalForm);
            }
            steps.push(Step {
                form: fff,
                explanation: String::from("def"),
            });
            return Ok(steps);
        } else if self.verb.pps[0] == "ἔξεστι(ν)" {
            let fff = special_verbs::get_exesti(self, decompose);
            if fff.is_empty() {
                return Err(HcFormError::IllegalForm);
            }
            steps.push(Step {
                form: fff,
                explanation: String::from("def"),
            });
            return Ok(steps);
        } else if self.verb.pps[0] == "εἰμί" {
            if self.tense != HcTense::Future {
                let fff = special_verbs::get_eimi(self, decompose);
                if fff.is_empty() {
                    return Err(HcFormError::IllegalForm);
                }
                steps.push(Step {
                    form: fff,
                    explanation: String::from("def"),
                });
                return Ok(steps);
            } else if self.person == Some(HcPerson::Third)
                && self.number == Some(HcNumber::Singular)
                && self.mood == HcMood::Indicative
            {
                if !decompose {
                    steps.push(Step {
                        form: String::from("ἔσται"),
                        explanation: String::from("def"),
                    });
                } else {
                    steps.push(Step {
                        form: format!("ἐσ {} εται", SEPARATOR),
                        explanation: String::from("def"),
                    });
                }
                return Ok(steps);
            }
        } else if self.verb.pps[0] == "φημί" {
            if self.tense == HcTense::Present || self.tense == HcTense::Imperfect {
                let fff = special_verbs::get_fhmi(self, decompose);
                if fff.is_empty() {
                    return Err(HcFormError::IllegalForm);
                }
                steps.push(Step {
                    form: fff,
                    explanation: String::from("def"),
                });
                return Ok(steps);
            } else if (self.tense == HcTense::Future || self.tense == HcTense::Aorist)
                && self.voice == HcVoice::Middle
            {
                /*fix me?*/
                return Err(HcFormError::IllegalForm);
            }
        } else if self.verb.pps[0] == "κεῖμαι" {
            if self.tense == HcTense::Present || self.tense == HcTense::Imperfect {
                let fff = special_verbs::get_keimai(self, decompose);
                if fff.is_empty() {
                    return Err(HcFormError::IllegalForm);
                }
                steps.push(Step {
                    form: fff,
                    explanation: String::from("def"),
                });
                return Ok(steps);
            }
        } else if self.verb.pps[0] == "εἶμι" {
            if self.tense == HcTense::Present || self.tense == HcTense::Imperfect {
                let fff = special_verbs::get_eimi_ibo(self, decompose);
                if fff.is_empty() {
                    return Err(HcFormError::IllegalForm);
                }
                steps.push(Step {
                    form: fff,
                    explanation: String::from("def"),
                });
                return Ok(steps);
            }
        } else if self.verb.pps[0] == "οἶδα" {
            if self.tense == HcTense::Present
                || self.tense == HcTense::Imperfect
                || self.tense == HcTense::Aorist
            {
                return Err(HcFormError::IllegalForm);
            } else if self.tense != HcTense::Future {
                let fff = special_verbs::get_oida(self, decompose);
                if fff.is_empty() {
                    return Err(HcFormError::IllegalForm);
                }
                steps.push(Step {
                    form: fff,
                    explanation: String::from("def"),
                });
                return Ok(steps);
            }
        } else if self.verb.pps[0] == "σύνοιδα" {
            if self.tense == HcTense::Present
                || self.tense == HcTense::Imperfect
                || self.tense == HcTense::Aorist
            {
                return Err(HcFormError::IllegalForm);
            } else if self.tense != HcTense::Future {
                let fff = special_verbs::get_sunoida(self, decompose);
                if fff.is_empty() {
                    return Err(HcFormError::IllegalForm);
                }
                steps.push(Step {
                    form: fff,
                    explanation: String::from("def"),
                });
                return Ok(steps);
            }
        }

        //let mut pps_without_ending = Vec::new();
        //strip accent: internally (not as a step)
        //let f = hgk_strip_diacritics_and_replace_circumflex_with_macron(f, HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE);
        let f = hgk_strip_diacritics(f, HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE);

        let mut pps_without_ending = f
            .split(" / ")
            .map(|e| e.to_string())
            .collect::<Vec<String>>();
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
        //add augment to imperfect or pluperfect
        if self.tense == HcTense::Imperfect || self.tense == HcTense::Pluperfect {
            for a in &pps_without_ending {
                pps_add_augment.push(self.add_augment(a, decompose));
            }
            pps_without_ending = pps_add_augment;
        }
        /* remove augment from non-indicative aorists and future passives */
        else if (self.tense == HcTense::Aorist && self.mood == HcMood::Indicative && decompose)
            || (self.tense == HcTense::Aorist && self.mood != HcMood::Indicative)
            || (self.tense == HcTense::Future && self.voice == HcVoice::Passive)
        {
            for a in &pps_without_ending {
                pps_add_augment.push(self.deaugment(a, decompose));
            }
            pps_without_ending = pps_add_augment;
        }

        let mut add_ending_collector = Vec::new();
        let mut add_accent_collector = Vec::new();

        for (alt_pp_idx, full_stem) in pps_without_ending.iter().enumerate() {
            //why not wait to strip ending in the loop?
            let endings_for_form = if self.mood == HcMood::Infinitive {
                match self.get_infinitive_endings(full_stem) {
                    Some(e) => e,
                    None => return Err(HcFormError::InternalError), //("Illegal form ending");,
                }
            } else if self.mood == HcMood::Participle {
                match self.get_participle_endings(full_stem) {
                    Some(e) => e,
                    None => return Err(HcFormError::InternalError), //("Illegal form ending");,
                }
            } else {
                match self.get_endings(full_stem) {
                    Some(e) => e,
                    None => return Err(HcFormError::InternalError), //("Illegal form ending");,
                }
            };

            for e in endings_for_form {
                //skip middle deponent pp if voice is active
                if full_stem.ends_with("ομην") && self.voice == HcVoice::Active {
                    continue;
                }

                let a = match self.strip_ending(pp_num, full_stem.to_string()) {
                    Ok(res) => res,
                    Err(_) => return Err(HcFormError::UnexpectedPrincipalPartEnding), //("error stripping ending");
                };

                // let f = a.join(" / ");
                // let e = "Remove ending from Principal Part".to_string();
                // steps.push(Step{form:f, explanation:e});

                if self.tense == HcTense::Aorist
                    && self.voice == HcVoice::Passive
                    && self.mood == HcMood::Imperative
                    && self.person == Some(HcPerson::Second)
                    && self.number == Some(HcNumber::Singular)
                {
                    if a.ends_with('θ') || a.ends_with('φ') || a.ends_with('χ') {
                        if e == "ηθι" {
                            continue;
                        }
                    } else if e == "ητι" {
                        continue;
                    }
                }

                // root aorist: skip middle voice
                if (a.ends_with("στη")
                    || a.ends_with("φθη")
                    || a.ends_with("βη")
                    || a.ends_with("γνω"))
                    && self.tense == HcTense::Aorist
                    && self.voice == HcVoice::Middle
                {
                    if pps_without_ending.len() > 1 {
                        continue; //if non-root alternate
                    } else {
                        return Err(HcFormError::InternalError); //only root, so no form
                    }
                }

                //attic greek does not form future passive from βλάπτω's βλαφθ 6th pp stem
                if self.verb.pps[0].starts_with("βλάπτω")
                    && a == "βλαφθ"
                    && self.tense == HcTense::Future
                    && self.voice == HcVoice::Passive
                {
                    continue;
                }

                // skip alternate here because same, could remove this now that we're removing duplicates later?
                if self.verb.pps[0].starts_with("σῴζω")
                    && ((a.ends_with("σεσω") && self.person == Some(HcPerson::Second))
                        || (a.ends_with("σεσωσ")
                            && self.person == Some(HcPerson::Third)
                            && self.number == Some(HcNumber::Plural)))
                {
                    continue;
                }

                let ending = if decompose {
                    hgk_strip_diacritics(e, HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE)
                } else {
                    e.to_string()
                };

                if self.mood == HcMood::Infinitive {
                    //println!("alt pp {}, {}, {}", alt_pp_idx, self.verb.pps[2], self.verb.pps[2].split('/').collect::<Vec<_>>()[alt_pp_idx]);
                    let mut new_stem = a.clone();
                    let infinitive = if self.tense == HcTense::Perfect
                        && self.voice != HcVoice::Active
                        && self.verb.is_consonant_stem(full_stem)
                    {
                        self.contract_consonants(&new_stem, e, decompose)
                    } else if self.tense == HcTense::Present && self.verb.pps[0].ends_with("άω") {
                        if self.voice == HcVoice::Active {
                            new_stem.pop();
                            format!("{}{}", new_stem, "ᾶν")
                        } else {
                            new_stem.pop();
                            format!("{}{}", new_stem, "ᾶσθαι")
                        }
                    } else if self.tense == HcTense::Present && self.verb.pps[0].ends_with("έω") {
                        if self.voice == HcVoice::Active {
                            new_stem.pop();
                            format!("{}{}", new_stem, "εῖν")
                        } else {
                            new_stem.pop();
                            format!("{}{}", new_stem, "εῖσθαι")
                        }
                    } else if self.tense == HcTense::Present && self.verb.pps[0].ends_with("όω") {
                        if self.voice == HcVoice::Active {
                            new_stem.pop();
                            format!("{}{}", new_stem, "οῦν")
                        } else {
                            new_stem.pop();
                            format!("{}{}", new_stem, "οῦσθαι")
                        }
                    } else if self.tense == HcTense::Present && self.verb.pps[0].ends_with("άομαι")
                    {
                        new_stem.pop();
                        format!("{}{}", new_stem, "ᾶσθαι")
                    } else if self.tense == HcTense::Present && self.verb.pps[0].ends_with("έομαι")
                    {
                        new_stem.pop();
                        format!("{}{}", new_stem, "εῖσθαι")
                    } else if self.tense == HcTense::Present && self.verb.pps[0].ends_with("όομαι")
                    {
                        new_stem.pop();
                        format!("{}{}", new_stem, "οῦσθαι")
                    } else if self.tense == HcTense::Present && self.verb.pps[0].ends_with("αμαι")
                    //δύναμαι
                    {
                        format!("{}{}", new_stem, "σθαι")
                    } else if self.tense == HcTense::Future
                        && self.voice != HcVoice::Passive
                        && self.verb.pps[1].ends_with('ῶ')
                    {
                        if self.verb.pps[1].ends_with("ἐλῶ") {
                            if self.voice == HcVoice::Active {
                                new_stem.pop();
                                format!("{}{}", new_stem, "ᾶν")
                            } else {
                                new_stem.pop();
                                format!("{}{}", new_stem, "ᾶσθαι")
                            }
                        } else if self.voice == HcVoice::Active {
                            new_stem.pop();
                            format!("{}{}", new_stem, "εῖν")
                        } else {
                            new_stem.pop();
                            format!("{}{}", new_stem, "εῖσθαι")
                        }
                    } else if self.tense == HcTense::Future
                        && self.voice != HcVoice::Passive
                        && self.verb.pps[1].ends_with("οῦμαι")
                    {
                        new_stem.pop();
                        format!("{}{}", new_stem, "εῖσθαι")
                    } else if self.tense == HcTense::Future
                        && self.verb.pps[1].starts_with("ἐρῶ")
                        && new_stem.starts_with("ἐρ")
                    {
                        if self.voice == HcVoice::Active {
                            new_stem.pop();
                            format!("{}{}", new_stem, "εῖν")
                        } else {
                            new_stem.pop();
                            format!("{}{}", new_stem, "εῖσθαι")
                        }
                    } else if self.tense == HcTense::Aorist
                        && self.voice != HcVoice::Passive
                        && (self.verb.pps[2].split('/').collect::<Vec<_>>()[alt_pp_idx]
                            .trim()
                            .ends_with("ον")
                            || self.verb.pps[2].trim().split('/').collect::<Vec<_>>()[alt_pp_idx]
                                .ends_with("όμην"))
                    {
                        if self.voice == HcVoice::Active {
                            format!("{}{}", new_stem, "εῖν")
                        } else {
                            format!("{}{}", new_stem, "έσθαι")
                        }
                    } else if self.tense == HcTense::Present && self.verb.pps[0].ends_with("ῑ̔́ημι")
                    {
                        if self.voice == HcVoice::Active {
                            String::from("ῑ̔έναι")
                        } else {
                            String::from("ῑ̔́εσθαι")
                        }
                    } else if self.tense == HcTense::Present && self.verb.pps[0].ends_with("ῑ́ημι")
                    {
                        new_stem.pop();
                        if self.voice == HcVoice::Active {
                            format!("{}{}", new_stem, "έναι")
                        } else {
                            format!("{}{}", new_stem, "εσθαι")
                        }
                    } else if self.tense == HcTense::Aorist
                        && self.voice != HcVoice::Passive
                        && self.verb.pps[0].ends_with("ῑ̔́ημι")
                    {
                        if self.voice == HcVoice::Active {
                            String::from("-εἷναι")
                        } else {
                            String::from("-ἕσθαι")
                        }
                    } else if self.tense == HcTense::Aorist
                        && self.voice != HcVoice::Passive
                        && self.verb.pps[0].ends_with("ῑ́ημι")
                    {
                        new_stem.pop();
                        if self.voice == HcVoice::Active {
                            format!("{}{}", new_stem, "εῖναι")
                        } else {
                            format!("{}{}", new_stem, "έσθαι")
                        }
                    } else if self.tense == HcTense::Present && self.verb.pps[0].ends_with("τίθημι")
                    {
                        new_stem.pop();
                        if self.voice == HcVoice::Active {
                            format!("{}{}", new_stem, "έναι")
                        } else {
                            format!("{}{}", new_stem, "εσθαι")
                        }
                    } else if self.tense == HcTense::Present && self.verb.pps[0].ends_with("δίδωμι")
                    {
                        new_stem.pop();
                        if self.voice == HcVoice::Active {
                            format!("{}{}", new_stem, "όναι")
                        } else {
                            format!("{}{}", new_stem, "οσθαι")
                        }
                    } else if self.tense == HcTense::Present && self.verb.pps[0].ends_with("στημι")
                    {
                        new_stem.pop();
                        if self.voice == HcVoice::Active {
                            format!("{}{}", new_stem, "άναι")
                        } else {
                            format!("{}{}", new_stem, "ασθαι")
                        }
                    } else if self.tense == HcTense::Present && self.verb.pps[0].ends_with("σταμαι")
                    {
                        format!("{}{}", new_stem, "σθαι")
                    } else if self.tense == HcTense::Aorist
                        && self.voice != HcVoice::Passive
                        && self.verb.pps[0].ends_with("τίθημι")
                    {
                        new_stem.pop(); //κ
                        new_stem.pop(); //η
                        if self.voice == HcVoice::Active {
                            format!("{}{}", new_stem, "εῖναι")
                        } else {
                            format!("{}{}", new_stem, "έσθαι")
                        }
                    } else if self.tense == HcTense::Aorist
                        && self.voice != HcVoice::Passive
                        && self.verb.pps[0].ends_with("δίδωμι")
                    {
                        new_stem.pop(); //κ
                        new_stem.pop(); //ο
                        if self.voice == HcVoice::Active {
                            format!("{}{}", new_stem, "οῦναι")
                        } else {
                            format!("{}{}", new_stem, "όσθαι")
                        }
                    } else if self.tense == HcTense::Aorist
                        && self.voice == HcVoice::Active
                        && self.verb.pps[2].ends_with("ην") //root aorists
                        && new_stem.ends_with('η')
                    //check each alt principal part in case not all root aorists
                    {
                        format!("{}{}", new_stem, "ναι")
                    } else if self.tense == HcTense::Aorist
                        && self.voice == HcVoice::Active
                        && self.verb.pps[2].ends_with("ων") //root aorists
                        && new_stem.ends_with('ω')
                    //check each alt principal part in case not all root aorists
                    {
                        format!("{}{}", new_stem, "ναι")
                    } else if self.tense == HcTense::Future && self.voice == HcVoice::Passive {
                        format!("{}ησ{}", new_stem, e)
                    } else if self.tense == HcTense::Perfect
                        && self.voice == HcVoice::Active
                        && (self.verb.pps[0].ends_with("στημι")
                            || self.verb.pps[0].ends_with("σταμαι"))
                    {
                        new_stem.pop(); //κ
                        new_stem.pop(); //η
                        format!("{}{}", new_stem, "άναι")
                    } else if self.tense == HcTense::Present && self.verb.pps[0].ends_with("ῡμι")
                    {
                        new_stem.pop();
                        if self.voice == HcVoice::Active {
                            format!("{}{}", new_stem, "υναι")
                        } else {
                            format!("{}{}", new_stem, "υσθαι")
                        }
                    } else if self.tense == HcTense::Present && self.verb.pps[0].ends_with("υμαι")
                    {
                        format!("{}{}", new_stem, "σθαι")
                    } else {
                        //everything else
                        format!("{}{}", new_stem, e)
                    };

                    let fff =
                        if !hgk_has_diacritics(&infinitive, HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE)
                        {
                            self.accent_infinitive(infinitive.as_str())
                        } else {
                            infinitive
                        };

                    add_accent_collector.push(fff);
                }
                //end handle infinitives
                else if self.mood == HcMood::Participle {
                    let new_stem = self.adjust_stem(full_stem, &a, true).unwrap(); //a.clone();

                    let mut e = e.to_string();
                    if (full_stem.ends_with("μι") || full_stem.ends_with("κα") || full_stem.ends_with("στην")) && !full_stem.ends_with("γκα") && !full_stem.ends_with("ῡμι") //enen
                        && (self.tense == HcTense::Present
                            || (self.tense == HcTense::Aorist && self.voice != HcVoice::Passive))
                    {
                        e.remove(0); //remove first character of ending

                        if self.voice == HcVoice::Active
                            && self.gender == Some(HcGender::Masculine)
                            && (self.case == Some(HcCase::Nominative)
                                || self.case == Some(HcCase::Vocative))
                            && self.number == Some(HcNumber::Singular)
                        {
                            if new_stem.ends_with('ο') {
                                e = String::from("υς");
                            } else if new_stem.ends_with('ε') {
                                e = String::from("ις");
                            } else if new_stem.ends_with('α') {
                                e = String::from("̄ς"); //0304 (macron) + sigma
                            }
                        } else if self.voice == HcVoice::Active
                            && self.number == Some(HcNumber::Plural)
                            && self.case == Some(HcCase::Dative)
                            && (self.gender == Some(HcGender::Masculine)
                                || self.gender == Some(HcGender::Neuter))
                        {
                            if new_stem.ends_with('ο') {
                                e = String::from("υσι(ν)");
                            } else if new_stem.ends_with('ε') {
                                e = String::from("ισι(ν)");
                            } else if new_stem.ends_with('α') {
                                e = String::from("̄σι(ν)"); //0304 (macron) + sigma
                            }
                        } else if self.gender == Some(HcGender::Feminine) {
                            if new_stem.ends_with('α') && self.tense == HcTense::Present {
                                e = e.replacen('υ', "̄", 1);
                            } else if new_stem.ends_with('ο') && self.tense == HcTense::Aorist {
                                e = e.replacen('̄', "υ", 1);
                            } else if new_stem.ends_with('ε') {
                                if self.tense == HcTense::Aorist {
                                    e = e.replacen('̄', "ι", 1);
                                } else {
                                    e = e.replace('υ', "ι");
                                }
                            }
                            // else if new_stem.ends_with("α") {
                            //     e = e.replace("υ", ""); //0304 (macron) + sigma
                            // }
                        }
                    }

                    let mut ptc = if self.tense == HcTense::Present {
                        if self.voice == HcVoice::Active {
                            format!("{}{}", new_stem, e)
                        } else if self.voice == HcVoice::Middle {
                            format!("{}{}", new_stem, e)
                        } else {
                            format!("{}{}", new_stem, e)
                        }
                    } else if self.tense == HcTense::Future {
                        if self.voice == HcVoice::Active {
                            format!("{}{}", new_stem, e)
                        } else if self.voice == HcVoice::Middle {
                            format!("{}{}", new_stem, e)
                        } else {
                            format!("{}ησ{}", new_stem, e)
                        }
                    } else if self.tense == HcTense::Aorist {
                        if self.voice == HcVoice::Active {
                            format!("{}{}", new_stem, e)
                        } else if self.voice == HcVoice::Middle {
                            format!("{}{}", new_stem, e)
                        } else {
                            format!("{}{}", new_stem, e)
                        }
                    } else if self.tense == HcTense::Perfect {
                        if self.voice == HcVoice::Active {
                            format!("{}{}", new_stem, e)
                        } else if self.voice == HcVoice::Middle {
                            format!("{}{}", new_stem, e)
                        } else {
                            format!("{}{}", new_stem, e)
                        }
                    } else {
                        String::from("")
                    };

                    if self.is_contracted_verb(&ptc) {
                        ptc = self.contract_verb(&ptc, &e);
                    }
                    let fff = if !hgk_has_diacritics(&ptc, HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE) {
                        self.accent_participle(ptc.as_str(), full_stem)
                    } else {
                        ptc
                    };

                    add_accent_collector.push(fff);
                } //end ptc

                let stem = if decompose
                    && self.tense == HcTense::Aorist
                    && self.voice == HcVoice::Passive
                    && self.mood == HcMood::Subjunctive
                {
                    format!("{}ε", a.to_owned())
                } else {
                    a.to_owned()
                };
                let y = self.add_ending(full_stem, &stem, &ending, decompose);

                let y = match y {
                    Ok(y) => y,
                    _ => return Err(HcFormError::InternalError), //("Error adding ending")
                };

                if decompose
                    && self.tense != HcTense::Imperfect
                    && self.tense != HcTense::Pluperfect
                    && self.tense != HcTense::Aorist
                    && !(self.tense == HcTense::Future && self.voice == HcVoice::Passive)
                    && self.mood != HcMood::Infinitive
                    && self.mood != HcMood::Participle
                {
                    add_ending_collector.push(self.separate_prefix(&y));
                } else if self.mood != HcMood::Infinitive && self.mood != HcMood::Participle {
                    add_ending_collector.push(y.to_string());
                }

                if !decompose {
                    let accented_form =
                        if !hgk_has_diacritics(&y, HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE) {
                            self.accent_verb(&y)
                        } else {
                            y
                        };
                    /* contracted future and present */
                    if self.mood != HcMood::Infinitive
                        && self.mood != HcMood::Participle
                        && self.is_contracted_verb(&accented_form)
                    {
                        add_accent_collector.push(self.contract_verb(&accented_form, e));
                    } else if self.mood != HcMood::Infinitive && self.mood != HcMood::Participle {
                        add_accent_collector.push(accented_form);
                    }
                    //println!("Here {} {}", a, e);
                }
            } //each ending loop
        } //each alt pp loop

        //remove duplicate decomposed forms for proe / prou
        if decompose
            && self.verb.pps[0] == "προδίδωμι"
            && ((self.tense == HcTense::Future && self.voice == HcVoice::Passive)
                || self.tense == HcTense::Aorist)
        {
            if add_ending_collector.len() == 2 {
                add_ending_collector.remove(1);
            } else if add_ending_collector.len() == 4 {
                add_ending_collector.remove(3);
                add_ending_collector.remove(2);
            }
        }

        //dynamai
        if self.verb.pps[0] == "δύναμαι"
            && decompose
            && self.mood == HcMood::Indicative
            && (self.tense == HcTense::Imperfect
                || self.tense == HcTense::Aorist
                || self.tense == HcTense::Pluperfect)
        {
            let alt = add_ending_collector[0].replacen('ε', "η", 1);
            add_ending_collector.push(alt);
        }

        //euriskw
        if self.verb.pps[0] == "εὑρίσκω" && decompose && self.mood == HcMood::Indicative {
            if self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect {
                let alt = add_ending_collector[0].replacen('η', "ε", 1);
                add_ending_collector.push(alt);
            } else if self.tense == HcTense::Imperfect
                || self.tense == HcTense::Aorist
                || self.tense == HcTense::Pluperfect
            {
                let alt = add_ending_collector[0].replacen("ε ‐ ", "", 1);
                add_ending_collector.push(alt);
            }
        }

        //aphihmi
        if self.verb.pps[0] == "ἀφῑ́ημι"
            && decompose
            && self.person == Some(HcPerson::Second)
            && self.number == Some(HcNumber::Singular)
            && self.tense == HcTense::Present
            && self.voice == HcVoice::Active
            && self.mood == HcMood::Indicative
        {
            let alt = String::from("ἀπο ‐ ῑ̔ε ‐ εις");
            add_ending_collector.push(alt);
        } else if self.verb.pps[0] == "συνῑ́ημι"
            && decompose
            && self.person == Some(HcPerson::Second)
            && self.number == Some(HcNumber::Singular)
            && self.tense == HcTense::Present
            && self.voice == HcVoice::Active
            && self.mood == HcMood::Indicative
        {
            let alt = String::from("συν ‐ ῑ̔ε ‐ εις");
            add_ending_collector.push(alt);
        } else if self.verb.pps[0] == "ῑ̔́ημι"
            && decompose
            && self.person == Some(HcPerson::Second)
            && self.number == Some(HcNumber::Singular)
            && self.tense == HcTense::Present
            && self.voice == HcVoice::Active
            && self.mood == HcMood::Indicative
        {
            let alt = String::from("ῑ̔ε ‐ εις");
            add_ending_collector.push(alt);
        }

        //add alts for ἀποθνῄσκω
        if self.verb.pps[0] == "ἀποθνῄσκω"
            && decompose
            && (self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect)
        {
            if !add_ending_collector.is_empty() && add_ending_collector[0] == "τεθνηκ ‐ αμεν"
            {
                let alt = String::from("τεθν ‐ αμεν");
                add_ending_collector.push(alt);
            } else if !add_ending_collector.is_empty() && add_ending_collector[0] == "τεθνηκ ‐ ατε"
            {
                let alt = String::from("τεθν ‐ ατε");
                add_ending_collector.push(alt);
            } else if !add_ending_collector.is_empty()
                && add_ending_collector[0] == "τεθνηκ ‐ ᾱσι(ν)"
            {
                let alt = String::from("τεθν ‐ ᾱσι(ν)");
                add_ending_collector.push(alt);
            } else if !add_ending_collector.is_empty()
                && add_ending_collector[0] == "ε ‐ τεθνηκ ‐ εσαν"
            {
                let alt = String::from("ε ‐ τεθν ‐ ασαν");
                add_ending_collector.push(alt);
            }
        }

        if add_ending_collector.is_empty()
            && self.mood != HcMood::Infinitive
            && self.mood != HcMood::Participle
        {
            //this catches meanesthn in aorist middle, etc.; fix me? should be better way to catch these
            return Err(HcFormError::InternalError);
        }
        let f = add_ending_collector.join(" / ");
        let e = "Add ending".to_string();
        steps.push(Step {
            form: f,
            explanation: e,
        });

        if !decompose {
            //add proe / prou forms for imperfect
            if self.verb.pps[0] == "προδίδωμι"
                && (self.tense == HcTense::Imperfect || self.tense == HcTense::Pluperfect)
            {
                let alt = add_accent_collector[0].replacen("προε", "πρου", 1);
                add_accent_collector.push(alt);
            }

            //aphihmi
            if self.verb.pps[0] == "ἀφῑ́ημι"
                && self.person == Some(HcPerson::Second)
                && self.number == Some(HcNumber::Singular)
                && self.tense == HcTense::Present
                && self.voice == HcVoice::Active
                && self.mood == HcMood::Indicative
            {
                let alt = String::from("ἀφῑεῖς");
                add_accent_collector.push(alt);
            } else if self.verb.pps[0] == "συνῑ́ημι"
                && self.person == Some(HcPerson::Second)
                && self.number == Some(HcNumber::Singular)
                && self.tense == HcTense::Present
                && self.voice == HcVoice::Active
                && self.mood == HcMood::Indicative
            {
                let alt = String::from("συνῑεῖς");
                add_accent_collector.push(alt);
            } else if self.verb.pps[0] == "ῑ̔́ημι"
                && self.person == Some(HcPerson::Second)
                && self.number == Some(HcNumber::Singular)
                && self.tense == HcTense::Present
                && self.voice == HcVoice::Active
                && self.mood == HcMood::Indicative
            {
                let alt = String::from("ῑ̔εῖς");
                add_accent_collector.push(alt);
            }

            //add alts for ἀποθνῄσκω
            if self.verb.pps[0] == "ἀποθνῄσκω"
                && (self.tense == HcTense::Perfect || self.tense == HcTense::Pluperfect)
            {
                if !add_accent_collector.is_empty() && add_accent_collector[0] == "τεθνήκαμεν"
                {
                    let alt = String::from("τέθναμεν");
                    add_accent_collector.push(alt);
                } else if !add_accent_collector.is_empty() && add_accent_collector[0] == "τεθνήκατε"
                {
                    let alt = String::from("τέθνατε");
                    add_accent_collector.push(alt);
                } else if !add_accent_collector.is_empty()
                    && add_accent_collector[0] == "τεθνήκᾱσι(ν)"
                {
                    let alt = String::from("τεθνᾶσι(ν)");
                    add_accent_collector.push(alt);
                } else if !add_accent_collector.is_empty()
                    && add_accent_collector[0] == "ἐτεθνήκεσαν"
                {
                    let alt = String::from("ἐτέθνασαν");
                    add_accent_collector.push(alt);
                } else if !add_accent_collector.is_empty()
                    && add_accent_collector[0] == "τεθνηκέναι"
                {
                    let alt = String::from("τεθνάναι");
                    add_accent_collector.push(alt);
                }
            }

            //dynamai
            if self.verb.pps[0] == "δύναμαι"
                && (self.tense == HcTense::Imperfect
                    || self.tense == HcTense::Aorist
                    || self.tense == HcTense::Pluperfect)
            {
                let alt = add_accent_collector[0].replacen('ἐ', "ἠ", 1);
                add_accent_collector.push(alt);
            }

            //euriskw
            if self.verb.pps[0] == "εὑρίσκω"
                && self.mood == HcMood::Indicative
                && (self.tense == HcTense::Perfect
                    || self.tense == HcTense::Imperfect
                    || self.tense == HcTense::Aorist
                    || self.tense == HcTense::Pluperfect)
            {
                let alt = add_accent_collector[0].replacen('η', "ε", 1);
                add_accent_collector.push(alt);
            }

            //remove duplicate and then join alternates with /
            let f = add_accent_collector
                .into_iter()
                .unique()
                .collect::<Vec<String>>()
                .join(" / ");
            let e = "Accent verb".to_string();
            steps.push(Step {
                form: f,
                explanation: e,
            });
        }

        Ok(steps)
    }

    fn get_pp(&self) -> Option<String> {
        let num = self.get_pp_num() as usize;
        if (1..=6).contains(&num) {
            Some(self.verb.pps[num - 1].to_string())
        } else {
            None
        }
    }

    //this needs to be refactored
    fn is_contracted_verb(&self, form: &str) -> bool {
        (self.tense == HcTense::Imperfect || self.tense == HcTense::Present)
            && (self.verb.pps[0].ends_with("άω")
                || self.verb.pps[0].ends_with("έω")
                || self.verb.pps[0].ends_with("όω")
                || self.verb.pps[0].ends_with("άομαι")
                || self.verb.pps[0].ends_with("έομαι")
                || self.verb.pps[0].ends_with("όομαι"))
            || (self.mood != HcMood::Infinitive
                && self.tense == HcTense::Future
                && self.voice != HcVoice::Passive
                && (self.verb.pps[1].ends_with('ῶ')
                    || (form.starts_with("ἐρ") && self.verb.pps[1].starts_with("ἐρῶ"))
                    || self.verb.pps[1].ends_with("οῦμαι")))
    }

    fn get_pp_num(&self) -> HcGreekPrincipalParts {
        match self.tense {
            HcTense::Present => HcGreekPrincipalParts::First,
            HcTense::Imperfect => HcGreekPrincipalParts::First,
            HcTense::Future => match self.voice {
                HcVoice::Active => HcGreekPrincipalParts::Second,
                HcVoice::Middle => HcGreekPrincipalParts::Second,
                HcVoice::Passive => HcGreekPrincipalParts::Sixth,
            },
            HcTense::Perfect => {
                if self.verb.pps[0].ends_with("δα") {
                    HcGreekPrincipalParts::First
                } else {
                    match self.voice {
                        HcVoice::Active => HcGreekPrincipalParts::Fourth,
                        HcVoice::Middle => HcGreekPrincipalParts::Fifth,
                        HcVoice::Passive => HcGreekPrincipalParts::Fifth,
                    }
                }
            }
            HcTense::Pluperfect => {
                if self.verb.pps[0].ends_with("δα") {
                    HcGreekPrincipalParts::First
                } else {
                    match self.voice {
                        HcVoice::Active => HcGreekPrincipalParts::Fourth,
                        HcVoice::Middle => HcGreekPrincipalParts::Fifth,
                        HcVoice::Passive => HcGreekPrincipalParts::Fifth,
                    }
                }
            }
            HcTense::Aorist => match self.voice {
                HcVoice::Active => HcGreekPrincipalParts::Third,
                HcVoice::Middle => HcGreekPrincipalParts::Third,
                HcVoice::Passive => HcGreekPrincipalParts::Sixth,
            },
        }
    }

    fn get_participle_endings(&self, stem: &str) -> Option<Vec<&str>> {
        match self.tense {
            HcTense::Present | HcTense::Future => match self.voice {
                HcVoice::Active => match self.number {
                    Some(HcNumber::Singular) => match self.gender {
                        Some(HcGender::Masculine) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => Some(vec!["ων"]),
                            Some(HcCase::Genitive) => Some(vec!["οντος"]),
                            Some(HcCase::Dative) => Some(vec!["οντι"]),
                            Some(HcCase::Accusative) => Some(vec!["οντα"]),
                            None => None,
                        },
                        Some(HcGender::Feminine) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => Some(vec!["ουσα"]),
                            Some(HcCase::Genitive) => Some(vec!["ουσης"]),
                            Some(HcCase::Dative) => Some(vec!["ουσῃ"]),
                            Some(HcCase::Accusative) => Some(vec!["ουσαν"]),
                            None => None,
                        },
                        Some(HcGender::Neuter) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => Some(vec!["ον"]),
                            Some(HcCase::Genitive) => Some(vec!["οντος"]),
                            Some(HcCase::Dative) => Some(vec!["οντι"]),
                            Some(HcCase::Accusative) => Some(vec!["ον"]),
                            None => None,
                        },
                        None => None,
                    },
                    Some(HcNumber::Plural) => match self.gender {
                        Some(HcGender::Masculine) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                Some(vec!["οντες"])
                            }
                            Some(HcCase::Genitive) => Some(vec!["οντων"]),
                            Some(HcCase::Dative) => Some(vec!["ουσι(ν)"]),
                            Some(HcCase::Accusative) => Some(vec!["οντας"]),
                            None => None,
                        },
                        Some(HcGender::Feminine) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                Some(vec!["ουσαι"])
                            }
                            Some(HcCase::Genitive) => Some(vec!["ουσῶν"]),
                            Some(HcCase::Dative) => Some(vec!["ουσαις"]),
                            Some(HcCase::Accusative) => Some(vec!["ουσᾱς"]),
                            None => None,
                        },
                        Some(HcGender::Neuter) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => Some(vec!["οντα"]),
                            Some(HcCase::Genitive) => Some(vec!["οντων"]),
                            Some(HcCase::Dative) => Some(vec!["ουσι(ν)"]),
                            Some(HcCase::Accusative) => Some(vec!["οντα"]),
                            None => None,
                        },
                        None => None,
                    },
                    Some(HcNumber::Dual) => todo!(),
                    None => None,
                },
                HcVoice::Middle | HcVoice::Passive => match self.number {
                    Some(HcNumber::Singular) => match self.gender {
                        Some(HcGender::Masculine) => match self.case {
                            Some(HcCase::Nominative) => Some(vec!["ομενος"]),
                            Some(HcCase::Genitive) => Some(vec!["ομενου"]),
                            Some(HcCase::Dative) => Some(vec!["ομενῳ"]),
                            Some(HcCase::Accusative) => Some(vec!["ομενον"]),
                            Some(HcCase::Vocative) => Some(vec!["ομενε"]),
                            None => None,
                        },
                        Some(HcGender::Feminine) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                Some(vec!["ομενη"])
                            }
                            Some(HcCase::Genitive) => Some(vec!["ομενης"]),
                            Some(HcCase::Dative) => Some(vec!["ομενῃ"]),
                            Some(HcCase::Accusative) => Some(vec!["ομενην"]),
                            None => None,
                        },
                        Some(HcGender::Neuter) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                Some(vec!["ομενον"])
                            }
                            Some(HcCase::Genitive) => Some(vec!["ομενου"]),
                            Some(HcCase::Dative) => Some(vec!["ομενῳ"]),
                            Some(HcCase::Accusative) => Some(vec!["ομενον"]),
                            None => None,
                        },
                        None => None,
                    },
                    Some(HcNumber::Plural) => match self.gender {
                        Some(HcGender::Masculine) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                Some(vec!["ομενοι"])
                            }
                            Some(HcCase::Genitive) => Some(vec!["ομενων"]),
                            Some(HcCase::Dative) => Some(vec!["ομενοις"]),
                            Some(HcCase::Accusative) => Some(vec!["ομενους"]),
                            None => None,
                        },
                        Some(HcGender::Feminine) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                Some(vec!["ομεναι"])
                            }
                            Some(HcCase::Genitive) => Some(vec!["ομενων"]),
                            Some(HcCase::Dative) => Some(vec!["ομεναις"]),
                            Some(HcCase::Accusative) => Some(vec!["ομενᾱς"]),
                            None => None,
                        },
                        Some(HcGender::Neuter) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                Some(vec!["ομενα"])
                            }
                            Some(HcCase::Genitive) => Some(vec!["ομενων"]),
                            Some(HcCase::Dative) => Some(vec!["ομενοις"]),
                            Some(HcCase::Accusative) => Some(vec!["ομενα"]),
                            None => None,
                        },
                        None => None,
                    },
                    Some(HcNumber::Dual) => todo!(),
                    None => None,
                },
            },
            HcTense::Aorist => match self.voice {
                HcVoice::Active => {
                    if stem.ends_with("ον") || stem.ends_with("ομην") {
                        match self.number {
                            Some(HcNumber::Singular) => match self.gender {
                                Some(HcGender::Masculine) => match self.case {
                                    Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                        Some(vec!["ων"])
                                    }
                                    Some(HcCase::Genitive) => Some(vec!["οντος"]),
                                    Some(HcCase::Dative) => Some(vec!["οντι"]),
                                    Some(HcCase::Accusative) => Some(vec!["οντα"]),
                                    None => None,
                                },
                                Some(HcGender::Feminine) => match self.case {
                                    Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                        Some(vec!["ουσα"])
                                    }
                                    Some(HcCase::Genitive) => Some(vec!["ουσης"]),
                                    Some(HcCase::Dative) => Some(vec!["ουσῃ"]),
                                    Some(HcCase::Accusative) => Some(vec!["ουσαν"]),
                                    None => None,
                                },
                                Some(HcGender::Neuter) => match self.case {
                                    Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                        Some(vec!["ον"])
                                    }
                                    Some(HcCase::Genitive) => Some(vec!["οντος"]),
                                    Some(HcCase::Dative) => Some(vec!["οντι"]),
                                    Some(HcCase::Accusative) => Some(vec!["ον"]),
                                    None => None,
                                },
                                None => None,
                            },
                            Some(HcNumber::Plural) => match self.gender {
                                Some(HcGender::Masculine) => match self.case {
                                    Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                        Some(vec!["οντες"])
                                    }
                                    Some(HcCase::Genitive) => Some(vec!["οντων"]),
                                    Some(HcCase::Dative) => Some(vec!["ουσι(ν)"]),
                                    Some(HcCase::Accusative) => Some(vec!["οντας"]),
                                    None => None,
                                },
                                Some(HcGender::Feminine) => match self.case {
                                    Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                        Some(vec!["ουσαι"])
                                    }
                                    Some(HcCase::Genitive) => Some(vec!["ουσῶν"]),
                                    Some(HcCase::Dative) => Some(vec!["ουσαις"]),
                                    Some(HcCase::Accusative) => Some(vec!["ουσᾱς"]),
                                    None => None,
                                },
                                Some(HcGender::Neuter) => match self.case {
                                    Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                        Some(vec!["οντα"])
                                    }
                                    Some(HcCase::Genitive) => Some(vec!["οντων"]),
                                    Some(HcCase::Dative) => Some(vec!["ουσι(ν)"]),
                                    Some(HcCase::Accusative) => Some(vec!["οντα"]),
                                    None => None,
                                },
                                None => None,
                            },
                            Some(HcNumber::Dual) => todo!(),
                            None => None,
                        }
                    } else {
                        match self.number {
                            Some(HcNumber::Singular) => match self.gender {
                                Some(HcGender::Masculine) => match self.case {
                                    Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                        Some(vec!["ᾱς"])
                                    }
                                    Some(HcCase::Genitive) => Some(vec!["αντος"]),
                                    Some(HcCase::Dative) => Some(vec!["αντι"]),
                                    Some(HcCase::Accusative) => Some(vec!["αντα"]),
                                    None => None,
                                },
                                Some(HcGender::Feminine) => match self.case {
                                    Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                        Some(vec!["ᾱσα"])
                                    }
                                    Some(HcCase::Genitive) => Some(vec!["ᾱσης"]),
                                    Some(HcCase::Dative) => Some(vec!["ᾱσῃ"]),
                                    Some(HcCase::Accusative) => Some(vec!["ᾱσαν"]),
                                    None => None,
                                },
                                Some(HcGender::Neuter) => match self.case {
                                    Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                        Some(vec!["αν"])
                                    }
                                    Some(HcCase::Genitive) => Some(vec!["αντος"]),
                                    Some(HcCase::Dative) => Some(vec!["αντι"]),
                                    Some(HcCase::Accusative) => Some(vec!["αν"]),
                                    None => None,
                                },
                                None => None,
                            },
                            Some(HcNumber::Plural) => match self.gender {
                                Some(HcGender::Masculine) => match self.case {
                                    Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                        Some(vec!["αντες"])
                                    }
                                    Some(HcCase::Genitive) => Some(vec!["αντων"]),
                                    Some(HcCase::Dative) => Some(vec!["ᾱσι(ν)"]),
                                    Some(HcCase::Accusative) => Some(vec!["αντας"]),
                                    None => None,
                                },
                                Some(HcGender::Feminine) => match self.case {
                                    Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                        Some(vec!["ᾱσαι"])
                                    }
                                    Some(HcCase::Genitive) => Some(vec!["ᾱσῶν"]),
                                    Some(HcCase::Dative) => Some(vec!["ᾱσαις"]),
                                    Some(HcCase::Accusative) => Some(vec!["ᾱσᾱς"]),
                                    None => None,
                                },
                                Some(HcGender::Neuter) => match self.case {
                                    Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                        Some(vec!["αντα"])
                                    }
                                    Some(HcCase::Genitive) => Some(vec!["αντων"]),
                                    Some(HcCase::Dative) => Some(vec!["ᾱσι(ν)"]),
                                    Some(HcCase::Accusative) => Some(vec!["αντα"]),
                                    None => None,
                                },
                                None => None,
                            },
                            Some(HcNumber::Dual) => todo!(),
                            None => None,
                        }
                    }
                }
                HcVoice::Middle => {
                    if stem.ends_with("ον") || stem.ends_with("ομην") {
                        match self.number {
                            Some(HcNumber::Singular) => match self.gender {
                                Some(HcGender::Masculine) => match self.case {
                                    Some(HcCase::Nominative) => Some(vec!["ομενος"]),
                                    Some(HcCase::Genitive) => Some(vec!["ομενου"]),
                                    Some(HcCase::Dative) => Some(vec!["ομενῳ"]),
                                    Some(HcCase::Accusative) => Some(vec!["ομενον"]),
                                    Some(HcCase::Vocative) => Some(vec!["ομενε"]),
                                    None => None,
                                },
                                Some(HcGender::Feminine) => match self.case {
                                    Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                        Some(vec!["ομενη"])
                                    }
                                    Some(HcCase::Genitive) => Some(vec!["ομενης"]),
                                    Some(HcCase::Dative) => Some(vec!["ομενῃ"]),
                                    Some(HcCase::Accusative) => Some(vec!["ομενην"]),
                                    None => None,
                                },
                                Some(HcGender::Neuter) => match self.case {
                                    Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                        Some(vec!["ομενον"])
                                    }
                                    Some(HcCase::Genitive) => Some(vec!["ομενου"]),
                                    Some(HcCase::Dative) => Some(vec!["ομενῳ"]),
                                    Some(HcCase::Accusative) => Some(vec!["ομενον"]),
                                    None => None,
                                },
                                None => None,
                            },
                            Some(HcNumber::Plural) => match self.gender {
                                Some(HcGender::Masculine) => match self.case {
                                    Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                        Some(vec!["ομενοι"])
                                    }
                                    Some(HcCase::Genitive) => Some(vec!["ομενων"]),
                                    Some(HcCase::Dative) => Some(vec!["ομενοις"]),
                                    Some(HcCase::Accusative) => Some(vec!["ομενους"]),
                                    None => None,
                                },
                                Some(HcGender::Feminine) => match self.case {
                                    Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                        Some(vec!["ομεναι"])
                                    }
                                    Some(HcCase::Genitive) => Some(vec!["ομενων"]),
                                    Some(HcCase::Dative) => Some(vec!["ομεναις"]),
                                    Some(HcCase::Accusative) => Some(vec!["ομενᾱς"]),
                                    None => None,
                                },
                                Some(HcGender::Neuter) => match self.case {
                                    Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                        Some(vec!["ομενα"])
                                    }
                                    Some(HcCase::Genitive) => Some(vec!["ομενων"]),
                                    Some(HcCase::Dative) => Some(vec!["ομενοις"]),
                                    Some(HcCase::Accusative) => Some(vec!["ομενα"]),
                                    None => None,
                                },
                                None => None,
                            },
                            Some(HcNumber::Dual) => todo!(),
                            None => None,
                        }
                    } else {
                        match self.number {
                            Some(HcNumber::Singular) => match self.gender {
                                Some(HcGender::Masculine) => match self.case {
                                    Some(HcCase::Nominative) => Some(vec!["αμενος"]),
                                    Some(HcCase::Genitive) => Some(vec!["αμενου"]),
                                    Some(HcCase::Dative) => Some(vec!["αμενῳ"]),
                                    Some(HcCase::Accusative) => Some(vec!["αμενον"]),
                                    Some(HcCase::Vocative) => Some(vec!["αμενε"]),
                                    None => None,
                                },
                                Some(HcGender::Feminine) => match self.case {
                                    Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                        Some(vec!["αμενη"])
                                    }
                                    Some(HcCase::Genitive) => Some(vec!["αμενης"]),
                                    Some(HcCase::Dative) => Some(vec!["αμενῃ"]),
                                    Some(HcCase::Accusative) => Some(vec!["αμενην"]),
                                    None => None,
                                },
                                Some(HcGender::Neuter) => match self.case {
                                    Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                        Some(vec!["αμενον"])
                                    }
                                    Some(HcCase::Genitive) => Some(vec!["αμενου"]),
                                    Some(HcCase::Dative) => Some(vec!["αμενῳ"]),
                                    Some(HcCase::Accusative) => Some(vec!["αμενον"]),
                                    None => None,
                                },
                                None => None,
                            },
                            Some(HcNumber::Plural) => match self.gender {
                                Some(HcGender::Masculine) => match self.case {
                                    Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                        Some(vec!["αμενοι"])
                                    }
                                    Some(HcCase::Genitive) => Some(vec!["αμενων"]),
                                    Some(HcCase::Dative) => Some(vec!["αμενοις"]),
                                    Some(HcCase::Accusative) => Some(vec!["αμενους"]),
                                    None => None,
                                },
                                Some(HcGender::Feminine) => match self.case {
                                    Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                        Some(vec!["αμεναι"])
                                    }
                                    Some(HcCase::Genitive) => Some(vec!["αμενων"]),
                                    Some(HcCase::Dative) => Some(vec!["αμεναις"]),
                                    Some(HcCase::Accusative) => Some(vec!["αμενᾱς"]),
                                    None => None,
                                },
                                Some(HcGender::Neuter) => match self.case {
                                    Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                        Some(vec!["αμενα"])
                                    }
                                    Some(HcCase::Genitive) => Some(vec!["αμενων"]),
                                    Some(HcCase::Dative) => Some(vec!["αμενοις"]),
                                    Some(HcCase::Accusative) => Some(vec!["αμενα"]),
                                    None => None,
                                },
                                None => None,
                            },
                            Some(HcNumber::Dual) => todo!(),
                            None => None,
                        }
                    }
                }
                HcVoice::Passive => match self.number {
                    Some(HcNumber::Singular) => match self.gender {
                        Some(HcGender::Masculine) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => Some(vec!["εις"]),
                            Some(HcCase::Genitive) => Some(vec!["εντος"]),
                            Some(HcCase::Dative) => Some(vec!["εντι"]),
                            Some(HcCase::Accusative) => Some(vec!["εντα"]),
                            None => None,
                        },
                        Some(HcGender::Feminine) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => Some(vec!["εισα"]),
                            Some(HcCase::Genitive) => Some(vec!["εισης"]),
                            Some(HcCase::Dative) => Some(vec!["εισῃ"]),
                            Some(HcCase::Accusative) => Some(vec!["εισαν"]),
                            None => None,
                        },
                        Some(HcGender::Neuter) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => Some(vec!["εν"]),
                            Some(HcCase::Genitive) => Some(vec!["εντος"]),
                            Some(HcCase::Dative) => Some(vec!["εντι"]),
                            Some(HcCase::Accusative) => Some(vec!["εν"]),
                            None => None,
                        },
                        None => None,
                    },
                    Some(HcNumber::Plural) => match self.gender {
                        Some(HcGender::Masculine) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                Some(vec!["εντες"])
                            }
                            Some(HcCase::Genitive) => Some(vec!["εντων"]),
                            Some(HcCase::Dative) => Some(vec!["εισι(ν)"]),
                            Some(HcCase::Accusative) => Some(vec!["εντας"]),
                            None => None,
                        },
                        Some(HcGender::Feminine) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                Some(vec!["εισαι"])
                            }
                            Some(HcCase::Genitive) => Some(vec!["εισῶν"]),
                            Some(HcCase::Dative) => Some(vec!["εισαις"]),
                            Some(HcCase::Accusative) => Some(vec!["εισᾱς"]),
                            None => None,
                        },
                        Some(HcGender::Neuter) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => Some(vec!["εντα"]),
                            Some(HcCase::Genitive) => Some(vec!["εντων"]),
                            Some(HcCase::Dative) => Some(vec!["εισι(ν)"]),
                            Some(HcCase::Accusative) => Some(vec!["εντα"]),
                            None => None,
                        },
                        None => None,
                    },
                    Some(HcNumber::Dual) => todo!(),
                    None => None,
                },
            },
            HcTense::Perfect => match self.voice {
                HcVoice::Active => match self.number {
                    Some(HcNumber::Singular) => match self.gender {
                        Some(HcGender::Masculine) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => Some(vec!["ως"]),
                            Some(HcCase::Genitive) => Some(vec!["οτος"]),
                            Some(HcCase::Dative) => Some(vec!["οτι"]),
                            Some(HcCase::Accusative) => Some(vec!["οτα"]),
                            None => None,
                        },
                        Some(HcGender::Feminine) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => Some(vec!["υια"]),
                            Some(HcCase::Genitive) => Some(vec!["υιᾱς"]),
                            Some(HcCase::Dative) => Some(vec!["υιᾱͅ"]),
                            Some(HcCase::Accusative) => Some(vec!["υιαν"]),
                            None => None,
                        },
                        Some(HcGender::Neuter) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => Some(vec!["ος"]),
                            Some(HcCase::Genitive) => Some(vec!["οτος"]),
                            Some(HcCase::Dative) => Some(vec!["οτι"]),
                            Some(HcCase::Accusative) => Some(vec!["ος"]),
                            None => None,
                        },
                        None => None,
                    },
                    Some(HcNumber::Plural) => match self.gender {
                        Some(HcGender::Masculine) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => Some(vec!["οτες"]),
                            Some(HcCase::Genitive) => Some(vec!["οτων"]),
                            Some(HcCase::Dative) => Some(vec!["οσι(ν)"]),
                            Some(HcCase::Accusative) => Some(vec!["οτας"]),
                            None => None,
                        },
                        Some(HcGender::Feminine) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => Some(vec!["υιαι"]),
                            Some(HcCase::Genitive) => Some(vec!["υιῶν"]),
                            Some(HcCase::Dative) => Some(vec!["υιαις"]),
                            Some(HcCase::Accusative) => Some(vec!["υιᾱς"]),
                            None => None,
                        },
                        Some(HcGender::Neuter) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => Some(vec!["οτα"]),
                            Some(HcCase::Genitive) => Some(vec!["οτων"]),
                            Some(HcCase::Dative) => Some(vec!["οσι(ν)"]),
                            Some(HcCase::Accusative) => Some(vec!["οτα"]),
                            None => None,
                        },
                        None => None,
                    },
                    Some(HcNumber::Dual) => todo!(),
                    None => None,
                },
                HcVoice::Middle | HcVoice::Passive => match self.number {
                    Some(HcNumber::Singular) => match self.gender {
                        Some(HcGender::Masculine) => match self.case {
                            Some(HcCase::Nominative) => Some(vec!["μενος"]),
                            Some(HcCase::Genitive) => Some(vec!["μενου"]),
                            Some(HcCase::Dative) => Some(vec!["μενῳ"]),
                            Some(HcCase::Accusative) => Some(vec!["μενον"]),
                            Some(HcCase::Vocative) => Some(vec!["μενε"]),
                            None => None,
                        },
                        Some(HcGender::Feminine) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => Some(vec!["μενη"]),
                            Some(HcCase::Genitive) => Some(vec!["μενης"]),
                            Some(HcCase::Dative) => Some(vec!["μενῃ"]),
                            Some(HcCase::Accusative) => Some(vec!["μενην"]),
                            None => None,
                        },
                        Some(HcGender::Neuter) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                Some(vec!["μενον"])
                            }
                            Some(HcCase::Genitive) => Some(vec!["μενου"]),
                            Some(HcCase::Dative) => Some(vec!["μενῳ"]),
                            Some(HcCase::Accusative) => Some(vec!["μενον"]),
                            None => None,
                        },
                        None => None,
                    },
                    Some(HcNumber::Plural) => match self.gender {
                        Some(HcGender::Masculine) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                Some(vec!["μενοι"])
                            }
                            Some(HcCase::Genitive) => Some(vec!["μενων"]),
                            Some(HcCase::Dative) => Some(vec!["μενοις"]),
                            Some(HcCase::Accusative) => Some(vec!["μενους"]),
                            None => None,
                        },
                        Some(HcGender::Feminine) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => {
                                Some(vec!["μεναι"])
                            }
                            Some(HcCase::Genitive) => Some(vec!["μενων"]),
                            Some(HcCase::Dative) => Some(vec!["μεναις"]),
                            Some(HcCase::Accusative) => Some(vec!["μενᾱς"]),
                            None => None,
                        },
                        Some(HcGender::Neuter) => match self.case {
                            Some(HcCase::Nominative) | Some(HcCase::Vocative) => Some(vec!["μενα"]),
                            Some(HcCase::Genitive) => Some(vec!["μενων"]),
                            Some(HcCase::Dative) => Some(vec!["μενοις"]),
                            Some(HcCase::Accusative) => Some(vec!["μενα"]),
                            None => None,
                        },
                        None => None,
                    },
                    Some(HcNumber::Dual) => todo!(),
                    None => None,
                },
            },
            _ => None,
        }
    }

    fn get_infinitive_endings(&self, _stem: &str) -> Option<Vec<&str>> {
        match self.tense {
            HcTense::Present | HcTense::Future => match self.voice {
                HcVoice::Active => Some(vec!["ειν"]),
                _ => Some(vec!["εσθαι"]),
            },
            HcTense::Aorist => match self.voice {
                HcVoice::Active => Some(vec!["αι"]),
                HcVoice::Middle => Some(vec!["ασθαι"]),
                _ => Some(vec!["ηναι"]),
            },
            HcTense::Perfect => match self.voice {
                HcVoice::Active => Some(vec!["εναι"]),
                _ => Some(vec!["σθαι"]),
            },
            _ => None,
        }
    }

    fn get_endings(&self, stem: &str) -> Option<Vec<&str>> {
        let ending = match self.tense {
            HcTense::Present => match self.voice {
                HcVoice::Active => match self.mood {
                    // HcMood::Infinitive => {
                    //     if self.verb.pps[0].ends_with("μι") {
                    //         HcEndings::PresentActiveIndicativeMi
                    //     } else {
                    //         HcEndings::PresentActiveInd
                    //     }
                    // }
                    HcMood::Indicative => {
                        if self.verb.pps[0].ends_with("μι") {
                            HcEndings::PresentActiveIndicativeMi
                        } else {
                            HcEndings::PresentActiveInd
                        }
                    }
                    HcMood::Subjunctive => {
                        if self.verb.pps[0].ends_with("μι") && !self.verb.pps[0].ends_with("ῡμι")
                        {
                            HcEndings::AoristPassiveSubj
                        } else {
                            HcEndings::PresentActiveSubj
                        }
                    }
                    HcMood::Optative => {
                        if self.verb.pps[0].ends_with("μι") && !self.verb.pps[0].ends_with("ῡμι")
                        {
                            HcEndings::PresentActiveOptMi
                        } else if self.verb.pps[0].ends_with("άω")
                            || self.verb.pps[0].ends_with("έω")
                            || self.verb.pps[0].ends_with("όω")
                        {
                            HcEndings::PresentActiveOptEContracted
                        } else {
                            HcEndings::PresentActiveOpt
                        }
                    }
                    HcMood::Imperative => HcEndings::PresentActiveImperative,
                    HcMood::Infinitive => HcEndings::NotImplemented,
                    HcMood::Participle => HcEndings::NotImplemented,
                },
                HcVoice::Middle | HcVoice::Passive => match self.mood {
                    HcMood::Indicative => {
                        if self.verb.pps[0].ends_with("μι")
                            || self.verb.pps[0].ends_with("υμαι")
                            || self.verb.pps[0].ends_with("αμαι")
                        {
                            HcEndings::PerfectMidpassInd
                        } else {
                            HcEndings::PresentMidpassInd
                        }
                    }
                    HcMood::Subjunctive => HcEndings::PresentMidpassSubj,
                    HcMood::Optative => {
                        if self.verb.pps[0].ends_with("ημι")
                            && !self.verb.pps[0].ends_with("στημι")
                            && !self.verb.pps[0].ends_with("ῑ́ημι")
                            && !self.verb.pps[0].ends_with("ῑ̔́ημι")
                        {
                            HcEndings::PresentMidpassOptTithhmi
                        } else if (self.verb.pps[0].ends_with("μι")
                            && !self.verb.pps[0].ends_with("ῡμι"))
                            || self.verb.pps[0].ends_with("αμαι")
                        {
                            HcEndings::MiddleOptMi
                        } else {
                            HcEndings::PresentMidpassOpt
                        }
                    }
                    HcMood::Imperative => {
                        if self.verb.pps[0].ends_with("μι")
                            || self.verb.pps[0].ends_with("υμαι")
                            || self.verb.pps[0].ends_with("αμαι")
                        {
                            HcEndings::PresentMidpassImperativeMi
                        } else {
                            HcEndings::PresentMidpassImperative
                        }
                    }
                    HcMood::Infinitive => HcEndings::NotImplemented,
                    HcMood::Participle => HcEndings::NotImplemented,
                },
            },
            HcTense::Imperfect => match self.voice {
                HcVoice::Active => match self.mood {
                    HcMood::Indicative => {
                        if self.verb.pps[0].ends_with("μι") {
                            HcEndings::ImperfectActiveMi
                        } else {
                            HcEndings::ImperfectActiveInd
                        }
                    }
                    HcMood::Subjunctive => HcEndings::NotImplemented,
                    HcMood::Optative => HcEndings::NotImplemented,
                    HcMood::Imperative => HcEndings::NotImplemented,
                    HcMood::Infinitive => HcEndings::NotImplemented,
                    HcMood::Participle => HcEndings::NotImplemented,
                },
                HcVoice::Middle | HcVoice::Passive => match self.mood {
                    HcMood::Indicative => {
                        if self.verb.pps[0].ends_with("μι")
                            || self.verb.pps[0].ends_with("υμαι")
                            || self.verb.pps[0].ends_with("αμαι")
                        {
                            HcEndings::PluperfectMidpassInd
                        } else {
                            HcEndings::ImperfectMidpassInd
                        }
                    }
                    HcMood::Subjunctive => HcEndings::NotImplemented,
                    HcMood::Optative => HcEndings::NotImplemented,
                    HcMood::Imperative => HcEndings::NotImplemented,
                    HcMood::Infinitive => HcEndings::NotImplemented,
                    HcMood::Participle => HcEndings::NotImplemented,
                },
            },
            HcTense::Future => {
                match self.voice {
                    HcVoice::Active => {
                        match self.mood {
                            HcMood::Indicative => HcEndings::PresentActiveInd,
                            HcMood::Subjunctive => HcEndings::NotImplemented,

                            HcMood::Optative => {
                                if
                                /* contracted future */
                                self.verb.pps[1].ends_with('ῶ')
                                    || (stem.starts_with("ἐρ")
                                        && self.verb.pps[1].starts_with("ἐρῶ"))
                                {
                                    HcEndings::PresentActiveOptEContracted
                                } else {
                                    HcEndings::PresentActiveOpt
                                }
                            }
                            HcMood::Imperative => HcEndings::NotImplemented,
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    }
                    HcVoice::Middle | HcVoice::Passive => match self.mood {
                        HcMood::Indicative => HcEndings::PresentMidpassInd,
                        HcMood::Subjunctive => HcEndings::NotImplemented,
                        HcMood::Optative => HcEndings::PresentMidpassOpt,
                        HcMood::Imperative => HcEndings::NotImplemented,
                        HcMood::Infinitive => HcEndings::NotImplemented,
                        HcMood::Participle => HcEndings::NotImplemented,
                    },
                }
            }
            HcTense::Aorist => match self.voice {
                HcVoice::Active => {
                    if stem.ends_with("ον") {
                        match self.mood {
                            HcMood::Indicative => HcEndings::ImperfectActiveInd,
                            HcMood::Subjunctive => HcEndings::PresentActiveSubj,
                            HcMood::Optative => {
                                if self.verb.pps[0].ends_with("ἔχω") {
                                    HcEndings::AoristOptativeEchw
                                } else {
                                    HcEndings::PresentActiveOpt
                                }
                            }
                            HcMood::Imperative => HcEndings::PresentActiveImperative,
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    } else {
                        match self.mood {
                            HcMood::Indicative => {
                                if stem.ends_with("στην")
                                    || stem.ends_with("φθην")
                                    || stem.ends_with("βην")
                                    || stem.ends_with("γνων")
                                {
                                    HcEndings::AoristActiveIndicativeMiRoot
                                } else if self.verb.pps[0].ends_with("μι")
                                    && self.verb.pps[2].ends_with("κα")
                                {
                                    HcEndings::MixedAoristMi
                                } else {
                                    HcEndings::AoristActiveInd
                                }
                            }
                            HcMood::Subjunctive => {
                                if stem.ends_with("στην")
                                    || stem.ends_with("φθην")
                                    || stem.ends_with("βην")
                                    || stem.ends_with("γνων")
                                {
                                    HcEndings::AoristPassiveSubj
                                } else {
                                    HcEndings::PresentActiveSubj
                                }
                            }
                            HcMood::Optative => {
                                if stem.ends_with("στην")
                                    || stem.ends_with("φθην")
                                    || stem.ends_with("βην")
                                    || stem.ends_with("γνων")
                                {
                                    HcEndings::PresentActiveOptMi
                                } else if self.verb.pps[0].ends_with("μι")
                                    && self.verb.pps[2].ends_with("κα")
                                {
                                    HcEndings::AoristPassiveOpt
                                } else {
                                    HcEndings::AoristActiveOpt
                                }
                            }
                            HcMood::Imperative => {
                                if stem.ends_with("στην")
                                    || stem.ends_with("φθην")
                                    || stem.ends_with("βην")
                                    || stem.ends_with("γνων")
                                {
                                    HcEndings::AoristActiveImperativesMiRoot
                                } else if self.verb.pps[0].ends_with("μι")
                                    && self.verb.pps[2].ends_with("κα")
                                {
                                    HcEndings::AoristActiveImperativesMi
                                } else {
                                    HcEndings::AoristActiveImperative
                                }
                            }
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    }
                }
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
                    } else {
                        match self.mood {
                            HcMood::Indicative => {
                                if self.verb.pps[0].ends_with("μι")
                                    && self.verb.pps[2].ends_with("κα")
                                {
                                    HcEndings::ImperfectMidpassInd
                                } else {
                                    HcEndings::AoristMidInd
                                }
                            }
                            HcMood::Subjunctive => HcEndings::PresentMidpassSubj,
                            HcMood::Optative => {
                                if self.verb.pps[0].ends_with("ημι")
                                    && self.verb.pps[2].ends_with("κα")
                                {
                                    HcEndings::PresentMidpassOptTithhmi
                                } else if self.verb.pps[0].ends_with("μι") && stem.ends_with("κα")
                                {
                                    HcEndings::MiddleOptMi
                                } else {
                                    HcEndings::AoristMiddleOpt
                                }
                            }
                            HcMood::Imperative => {
                                if self.verb.pps[0].ends_with("μι")
                                    && self.verb.pps[2].ends_with("κα")
                                {
                                    HcEndings::PresentMidpassImperativeMi
                                } else {
                                    HcEndings::AoristMiddleImperative
                                }
                            }
                            HcMood::Infinitive => HcEndings::NotImplemented,
                            HcMood::Participle => HcEndings::NotImplemented,
                        }
                    }
                }
                HcVoice::Passive => match self.mood {
                    HcMood::Indicative => HcEndings::AoristPassiveInd,
                    HcMood::Subjunctive => HcEndings::AoristPassiveSubj,
                    HcMood::Optative => HcEndings::AoristPassiveOpt,
                    HcMood::Imperative => HcEndings::AoristPassiveImperative,
                    HcMood::Infinitive => HcEndings::NotImplemented,
                    HcMood::Participle => HcEndings::NotImplemented,
                },
            },
            HcTense::Perfect => match self.voice {
                HcVoice::Active => match self.mood {
                    HcMood::Indicative => HcEndings::PerfectActiveInd,
                    HcMood::Subjunctive => HcEndings::NotImplemented,
                    HcMood::Optative => HcEndings::NotImplemented,
                    HcMood::Imperative => HcEndings::NotImplemented,
                    HcMood::Infinitive => HcEndings::NotImplemented,
                    HcMood::Participle => HcEndings::NotImplemented,
                },
                HcVoice::Middle | HcVoice::Passive => match self.mood {
                    HcMood::Indicative => HcEndings::PerfectMidpassInd,
                    HcMood::Subjunctive => HcEndings::NotImplemented,
                    HcMood::Optative => HcEndings::NotImplemented,
                    HcMood::Imperative => HcEndings::NotImplemented,
                    HcMood::Infinitive => HcEndings::NotImplemented,
                    HcMood::Participle => HcEndings::NotImplemented,
                },
            },
            HcTense::Pluperfect => match self.voice {
                HcVoice::Active => match self.mood {
                    HcMood::Indicative => HcEndings::PluperfectActiveInd,
                    HcMood::Subjunctive => HcEndings::NotImplemented,
                    HcMood::Optative => HcEndings::NotImplemented,
                    HcMood::Imperative => HcEndings::NotImplemented,
                    HcMood::Infinitive => HcEndings::NotImplemented,
                    HcMood::Participle => HcEndings::NotImplemented,
                },
                HcVoice::Middle | HcVoice::Passive => match self.mood {
                    HcMood::Indicative => HcEndings::PluperfectMidpassInd,
                    HcMood::Subjunctive => HcEndings::NotImplemented,
                    HcMood::Optative => HcEndings::NotImplemented,
                    HcMood::Imperative => HcEndings::NotImplemented,
                    HcMood::Infinitive => HcEndings::NotImplemented,
                    HcMood::Participle => HcEndings::NotImplemented,
                },
            },
        };

        if ending == HcEndings::NotImplemented {
            return None;
        }

        let person_number: usize = match self.person {
            Some(HcPerson::First) => match self.number {
                Some(HcNumber::Singular) => 0,
                Some(HcNumber::Dual) => 0,
                Some(HcNumber::Plural) => 3,
                _ => return None,
            },
            Some(HcPerson::Second) => match self.number {
                Some(HcNumber::Singular) => 1,
                Some(HcNumber::Dual) => 0,
                Some(HcNumber::Plural) => 4,
                _ => return None,
            },
            Some(HcPerson::Third) => match self.number {
                Some(HcNumber::Singular) => 2,
                Some(HcNumber::Dual) => 0,
                Some(HcNumber::Plural) => 5,
                _ => return None,
            },
            _ => return None,
        };

        Some(ENDINGS[ending as usize][person_number].split(',').collect())
    }

    fn adjust_stem(&self, full_stem: &str, stem: &str, decompose: bool) -> Option<String> {
        let mut local_stem = stem.to_string();
        //e.g full_stem: δωκα, stem: δωκ
        //println!("abc{}, {}", full_stem, stem);

        if self.tense == HcTense::Present {
            if full_stem.ends_with("διδωμι") {
                local_stem = local_stem.replace('ω', "ο");
            } else if full_stem.ends_with("τιθημι") {
                local_stem = local_stem.replace('η', "ε");
            } else if full_stem.ends_with("ἱστημι") {
                local_stem = local_stem.replace('η', "α");
            }
        } else if self.tense == HcTense::Aorist && self.voice != HcVoice::Passive {
            //mixed aorist
            if full_stem.ends_with("κα")
                && (self.number == Some(HcNumber::Plural)
                    || self.mood != HcMood::Indicative
                    || self.voice != HcVoice::Active)
            {
                if full_stem.ends_with("δωκα") {
                    local_stem = local_stem.replacen("ωκ", "ο", 1);
                } else if full_stem.ends_with("θηκα")
                    || full_stem.ends_with("ἡκα")
                    || full_stem.ends_with("ηκα")
                {
                    if full_stem.ends_with("ηκα")
                        && !decompose
                        && (self.number == Some(HcNumber::Plural) || self.voice != HcVoice::Active)
                    {
                        local_stem = local_stem.replacen("ηκ", "ε", 1);
                    } else if full_stem.ends_with("ἡκα") && !decompose {
                        local_stem = local_stem.replacen("ἡκ", "εἱ", 1);
                    } else {
                        local_stem = local_stem.replacen("ηκ", "ε", 1);
                    }
                }
            } else if full_stem.ends_with("στην") {
                local_stem = local_stem.replace('η', "α");
            }
        }
        Some(local_stem)
    }
}

#[derive(Debug, PartialEq)]
pub struct SyllableAnalysis {
    letters: String,
    is_long: bool,
    index: u8,
}

use unicode_segmentation::UnicodeSegmentation;
static PREFIXES: &[&str; 16] = &[
    "ἐκ",
    "ἀνα",
    "συμ",
    "συν",
    "δια",
    "διο",
    "ἀπο",
    "ἀπ",
    "ἀφ",
    "καθ",
    "κατα",
    "μετανα",
    "μεταν",
    "μετα",
    "ἐπαν",
    "ἐπι",
];

fn analyze_syllable_quantities(
    word: &str,
    p: Option<HcPerson>,
    n: Option<HcNumber>,
    t: HcTense,
    m: HcMood,
    props: u32,
) -> Vec<SyllableAnalysis> {
    let mut letters = word.gkletters();

    //    /*
    //  For prefixes, find where the prefix ends and don't look past that character
    //  */
    // if ((vf->verb->verbclass & PREFIXED) == PREFIXED && !utf8HasSuffix(vf->verb->present, "σύνοιδα") && ((vf->tense == AORIST && vf->mood == INDICATIVE) || vf->tense == PERFECT || vf->tense == PLUPERFECT))
    // {

    let mut area = word.len();
    if (props & PREFIXED) == PREFIXED
        && ((t == HcTense::Aorist && m == HcMood::Indicative)
            || t == HcTense::Perfect
            || t == HcTense::Pluperfect)
    {
        for p in PREFIXES {
            if word.starts_with(p) {
                area = p.graphemes(true).count();
                //println!("area: {} {}", p, area);
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
        //     else if (hasPrefix(tempUcs2String, *len, apo, 3) && !utf8HasSuffix(vf->verb->present, "ἀπόλλῡμι"))
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
    while let Some(x) = letters.next_back() {
        //println!("letter: {:?}", x);
        match x.letter_type() {
            HgkLetterType::HgkLongVowel => {
                if last_letter == 'υ' && x.letter == 'η' {
                    res.pop();
                    let mut s = String::from(x.letter);
                    s.push(last_letter);
                    res.push(SyllableAnalysis {
                        letters: s,
                        is_long: true,
                        index: letter_num - 1,
                    });
                } else {
                    last_letter = '\u{0000}';
                    res.push(SyllableAnalysis {
                        letters: x.to_string(HgkUnicodeMode::Precomposed),
                        is_long: true,
                        index: letter_num,
                    });
                }
            }
            HgkLetterType::HgkShortVowel => {
                if (x.letter == 'υ' && last_letter != 'ι')
                    || x.letter == 'ι' && (x.diacritics & HGK_DIAERESIS) != HGK_DIAERESIS
                {
                    last_letter = x.letter;
                    //res.push((x.letter.to_string(), false, letter_num)); //add short, might be replaced by diphthong
                    res.push(SyllableAnalysis {
                        letters: x.letter.to_string(),
                        is_long: false,
                        index: letter_num,
                    });
                } else {
                    if last_letter != '\u{0000}'
                        && (x.letter == 'ε'
                            || x.letter == 'α'
                            || x.letter == 'ο'
                            || x.letter == 'υ')
                    {
                        res.pop();
                        let mut s = String::from(x.letter);
                        s.push(last_letter);

                        let is_short = letter_num == 1
                            && (x.letter == 'α' || x.letter == 'ο')
                            && last_letter == 'ι'; //final diphthongs short accent
                        if is_short
                            && p == Some(HcPerson::Third)
                            && n == Some(HcNumber::Singular)
                            && m == HcMood::Optative
                        {
                            //exception to the exception for optative 3rd sing.
                            //res.push((s, true, letter_num - 1));
                            res.push(SyllableAnalysis {
                                letters: s,
                                is_long: true,
                                index: letter_num - 1,
                            });
                        } else {
                            //res.push((s, !is_short, letter_num - 1));
                            res.push(SyllableAnalysis {
                                letters: s,
                                is_long: !is_short,
                                index: letter_num - 1,
                            });
                        }
                    } else {
                        //res.push((x.letter.to_string(), false, letter_num));
                        res.push(SyllableAnalysis {
                            letters: x.letter.to_string(),
                            is_long: false,
                            index: letter_num,
                        });
                    }
                    last_letter = '\u{0000}';
                }
            }
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
    }
    res.reverse();
    res
}

#[derive(Eq, PartialEq, Debug)]
enum HcPtcEndings {
    PresentActiveMasc,
    PresentActiveFem,
    PresentActiveNeut,

    PresentMiddleMasc,
    PresentMiddleFem,
    PresentMiddleNeut,

    AoristActiveMasc,
    AoristActiveFem,
    AoristActiveNeut,

    AoristMiddleMasc,
    AoristMiddleFem,
    AoristMiddleNeut,

    AoristPassiveMasc,
    AoristPassiveFem,
    AoristPassiveNeut,

    PerfectActiveMasc,
    PerfectActiveFem,
    PerfectActiveNeut,

    PerfectMiddleMasc,
    PerfectMiddleFem,
    PerfectMiddleNeut,
}

static PTC_ENDINGS: &[[&str; 8]; 21] = &[
    [
        "ων",
        "οντος",
        "οντι",
        "οντα",
        "οντες",
        "οντων",
        "ουσι(ν)",
        "οντας",
    ],
    [
        "ουσα",
        "ουσης",
        "ουσῃ",
        "ουσαν",
        "ουσαι",
        "ουσῶν",
        "ουσαις",
        "ουσᾱς",
    ],
    [
        "oν",
        "οντος",
        "οντι",
        "ον",
        "οντa",
        "οντων",
        "ουσι(ν)",
        "οντα",
    ],
    [
        "ομενος",
        "ομενου",
        "ομενι",
        "ομενα",
        "ομενοι",
        "ομενων",
        "ομενοις",
        "ομενους",
    ],
    [
        "ομενη",
        "ομενης",
        "ομενῃ",
        "ομενην",
        "ομεναι",
        "ομενων",
        "ομεναις",
        "ομενᾱς",
    ],
    [
        "ομενον",
        "ομενου",
        "ομενι",
        "ομενον",
        "ομενα",
        "ομενων",
        "ομενοις",
        "ομενα",
    ],
    [
        "ᾱς",
        "αντος",
        "αντι",
        "αντα",
        "αντες",
        "αντων",
        "ᾱσι(ν)",
        "αντας",
    ],
    [
        "ᾱσα",
        "ᾱσης",
        "ᾱσῃ",
        "ᾱσαν",
        "ᾱσαι",
        "ᾱσῶν",
        "ᾱσαις",
        "ᾱσᾱς",
    ],
    [
        "αν",
        "αντος",
        "αντι",
        "αν",
        "αντa",
        "αντων",
        "ᾱσι(ν)",
        "αντα",
    ],
    [
        "αμενος",
        "αμενου",
        "αμενι",
        "αμενα",
        "αμενοι",
        "αμενων",
        "αμενοις",
        "αμενους",
    ],
    [
        "αμενη",
        "αμενης",
        "αμενῃ",
        "αμενην",
        "αμεναι",
        "αμενων",
        "αμεναις",
        "αμενᾱς",
    ],
    [
        "αμενον",
        "αμενου",
        "αμενι",
        "αμενον",
        "αμενα",
        "αμενων",
        "αμενοις",
        "αμενα",
    ],
    [
        "εις",
        " εντος",
        "εντι",
        "εντα",
        "εντες",
        "εντων",
        "εισι(ν)",
        "εντας",
    ],
    [
        "εισα",
        "εισης",
        "εισῃ",
        "εισαν",
        "εισαι",
        "εισῶν",
        "εισαις",
        "εισᾱς",
    ],
    [
        "εν",
        "εντος",
        "εντι",
        "εν",
        "εντa",
        "εντων",
        "εισι(ν)",
        "εντα",
    ],
    ["ως", "οτος", "οτι", "οτα", "οτες", "οτων", "οσι(ν)", "οτας"],
    [
        "υια",
        "υιᾱς",
        "υιᾱͅ",
        "υιαν",
        "υιαι",
        "υιῶν",
        "υιαις",
        "υιᾱς",
    ],
    ["oς", "οτος", "οτι", "ος", "οτa", "οτων", "οσι(ν)", "οτα"],
    [
        "μενος",
        "μενου",
        "μενι",
        "μενα",
        "μενοι",
        "μενων",
        "μενοις",
        "μενους",
    ],
    [
        "μενη",
        "μενης",
        "μενῃ",
        "μενην",
        "μεναι",
        "μενων",
        "μεναις",
        "μενᾱς",
    ],
    [
        "μενον",
        "μενου",
        "μενι",
        "μενον",
        "μενα",
        "μενων",
        "μενοις",
        "μενα",
    ],
];

static ENDINGS: &[[&str; 6]; 38] = &[
    ["ω", "εις", "ει", "ομεν", "ετε", "ουσι(ν)"], //, "Present Active Indicative" },
    ["ον", "ες", "ε(ν)", "ομεν", "ετε", "ον"],    //, "Imperfect Active Indicative" },
    ["α", "ας", "ε(ν)", "αμεν", "ατε", "αν"],     //, "Aorist Active Indicative" },
    ["α", "ας", "ε(ν)", "αμεν", "ατε", "ᾱσι(ν)"], //, "Perfect Active Indicative" },
    ["η", "ης", "ει(ν)", "εμεν", "ετε", "εσαν"],  //, "Pluperfect Active Indicative" },
    ["ω", "ῃς", "ῃ", "ωμεν", "ητε", "ωσι(ν)"],    //, "Present Active Subjunctive" },
    ["οιμι", "οις", "οι", "οιμεν", "οιτε", "οιεν"], //, "Present Active Optative" },
    [
        "αιμι",
        "αις,ειας",
        "αι,ειε(ν)",
        "αιμεν",
        "αιτε",
        "αιεν,ειαν",
    ], //, "Aorist Active Optative" },
    ["ομαι", "ει,ῃ", "εται", "ομεθα", "εσθε", "ονται"], //, "Present Middle/Passive Indicative" },
    ["ομην", "ου", "ετο", "ομεθα", "εσθε", "οντο"], //, "Imperfect Middle/Passive Indicative" },
    ["ην", "ης", "η", "ημεν", "ητε", "ησαν"],     //, "Aorist Passive Indicative" },
    ["αμην", "ω", "ατο", "αμεθα", "ασθε", "αντο"], //, "Aorist Middle Indicative" },
    ["ῶ", "ῇς", "ῇ", "ῶμεν", "ῆτε", "ῶσι(ν)"],    //***, "Aorist Passive Subjunctive" },
    [
        "ειην",
        "ειης",
        "ειη",
        "εῖμεν,ειημεν",
        "εῖτε,ειητε",
        "εῖεν,ειησαν",
    ], //, "Aorist Passive Optative" },
    ["αιμην", "αιο", "αιτο", "αιμεθα", "αισθε", "αιντο"], //, "Aorist Middle Optative" },
    ["μαι", "σαι", "ται", "μεθα", "σθε", "νται"], //, "Perfect Middle/Passive Indicative" },
    ["μην", "σο", "το", "μεθα", "σθε", "ντο"],    //, "Pluperfect Middle/Passive Indicative" },
    ["ωμαι", "ῃ", "ηται", "ωμεθα", "ησθε", "ωνται"], //, "Present Middle/Passive Subjunctive" },
    ["οιμην", "οιο", "οιτο", "οιμεθα", "οισθε", "οιντο"], //, "Present Middle/Passive Optative" },
    ["", "ε", "ετω", "", "ετε", "οντων"],         //, "Present Active Imperative" },
    ["", "ου", "εσθω", "", "εσθε", "εσθων"],      //, "Present Middle/Passive Imperative" },
    ["", "ον", "ατω", "", "ατε", "αντων"],        //, "Aorist Active Imperative" },
    ["", "αι", "ασθω", "", "ασθε", "ασθων"],      //, "Aorist Middle Imperative" },
    ["", "ητι,ηθι", "ητω", "", "ητε", "εντων"],   //, "Aorist Passive Imperative" },
    [
        "οιμι,οιην",
        "οις,οιης",
        "οι,οιη",
        "οιμεν,οιημεν",
        "οιτε,οιητε",
        "οιεν,οιησαν",
    ], //, "" },//pres act opt e
    ["μι", "ς", "σι(ν)", "μεν", "τε", "ᾱσι(ν)"],  //, "" },   //mi
    ["", "ς", "τω", "", "τε", "ντων"],            //, "" },//mi aorist active imperatives
    ["", "θι", "τω", "", "τε", "ντων"],           //", "" },//mi root aorist active imperatives
    ["", "ο", "σθω", "", "σθε", "σθων"], //, "Root Aorist Middle Imperative" },//mi root aorist middle imperatives
    ["ν", "ς", "", "μεν", "τε", "σαν"],  //, "Root Aorist Indicative" },//mi root aorist indicative
    ["", "οῦ", "εσθω", "", "εσθε", "εσθων"], //, "Present Middle/Passive Imperative" }, //second aorist middle/passive imperatives
    [
        "ιμην",
        "ῖο",
        "ῖτο,οῖτο",
        "ιμεθα,οιμεθα",
        "ῖσθε,οῖσθε",
        "ῖντο,οῖντο",
    ], //, "Present Middle/Passive Optative Tithemi" }, //Exception: H&Q page 347
    //["ον", "ες", "ε", "ομεν", "ετε", "ον"],//***, "Imperfect Active Indicative" } //this is only for contracted verbs when decompose so the nu moveable doesn't show up
    ["", "σο", "σθω", "", "σθε", "σθων"],
    ["ν", "ς", "", "μεν", "τε", "σαν"],
    ["α", "ας", "ε(ν)", "μεν", "τε", "σαν"],
    ["ιμην", "ῖο", "ῖτο", "ιμεθα", "ῖσθε", "ῖντο"],
    ["ιην", "ιης", "ιη", "ῖμεν,ιημεν", "ῖτε,ιητε", "ῖεν,ιησαν"], //, "Aorist Passive Optative" },
    ["οιην", "οιης", "οιη", "οιμεν", "οιτε", "οιεν"],
];

pub fn check_pps(input: &str, verb: &HcGreekVerb) -> Vec<bool> {
    let pps = input.split(',').collect::<Vec<_>>();
    if pps.len() != 6 || verb.pps.len() != 6 {
        //todo
        //if pps count is not 6, could check if there are 6 semi-colons or 6 slashes
        //and split on that instead of immediately failing
        return vec![false, false, false, false, false, false];
    }
    let mut is_correct_pps: Vec<bool> = Vec::new();
    for (i, p) in pps.iter().enumerate() {
        let d = &verb.pps[i];
        is_correct_pps.push(hgk_compare_multiple_forms(
            d.replace(['/', ';'], ",").trim(),
            p.replace("---", "—").replace(['/', ';'], ",").trim(),
            true,
        ));
    }
    is_correct_pps
}

fn separate_prefixes(form: &str) -> Option<Vec<&str>> {
    let prefixes = vec![
        Prefixes {
            prefix: "μεταν",
            separated: vec!["μετα", "ανα"],
        },
        Prefixes {
            prefix: "ἐπαν",
            separated: vec!["ἐπι", "ανα"],
        },
        Prefixes {
            prefix: "ἀπο",
            separated: vec!["ἀπο"],
        },
        Prefixes {
            prefix: "ἀπ",
            separated: vec!["ἀπο"],
        },
        Prefixes {
            prefix: "ἀφ",
            separated: vec!["ἀπο"],
        },
        Prefixes {
            prefix: "ἀνα",
            separated: vec!["ἀνα"],
        },
        Prefixes {
            prefix: "καθ",
            separated: vec!["κατα"],
        },
        Prefixes {
            prefix: "κατα",
            separated: vec!["κατα"],
        },
        Prefixes {
            prefix: "μετα",
            separated: vec!["μετα"],
        },
        Prefixes {
            prefix: "ἐπι",
            separated: vec!["ἐπι"],
        },
        Prefixes {
            prefix: "παρα",
            separated: vec!["παρα"],
        },
        Prefixes {
            prefix: "ὑπ",
            separated: vec!["ὑπο"],
        },
        Prefixes {
            prefix: "ὑπο",
            separated: vec!["ὑπο"],
        },
        Prefixes {
            prefix: "δια",
            separated: vec!["δια"],
        },
        Prefixes {
            prefix: "ἐξ",
            separated: vec!["ἐκ"],
        },
        Prefixes {
            prefix: "ἐκ",
            separated: vec!["ἐκ"],
        },
        Prefixes {
            prefix: "συμ",
            separated: vec!["συν"],
        },
        Prefixes {
            prefix: "συν",
            separated: vec!["συν"],
        },
        Prefixes {
            prefix: "προ",
            separated: vec!["προ"],
        },
    ];

    for p in prefixes {
        if form.starts_with(p.prefix) {
            return Some(p.separated);
        }
    }
    None
}

struct Prefixes<'a> {
    prefix: &'a str,
    separated: Vec<&'a str>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
    use quick_xml::writer::Writer;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::io::{BufWriter, Write};
    use unicode_normalization::UnicodeNormalization;

    #[test]
    fn as_test() {
        let luw_correct = "δίδωμι, δώσω, ἔδωκα, δέδωκα, δέδομαι, ἐδόθην";
        let verb = Arc::new(HcGreekVerb::from_string(1, luw_correct, 0x0000, 0).unwrap());
        let a = HcGreekVerbForm {
            verb: verb.clone(),
            person: None,
            number: Some(HcNumber::Singular),
            tense: HcTense::Aorist,
            voice: HcVoice::Active,
            mood: HcMood::Participle,
            gender: Some(HcGender::Masculine),
            case: Some(HcCase::Genitive),
        };
        assert_eq!(a.get_form(false).unwrap().last().unwrap().form, "δόντος");
    }

    #[test]
    fn adjust_stem_test() {
        let luw_correct = "λω, λσω, ἔλῡσα, λέλυκα, λέλυμαι, ἐλύθην";
        let verb = Arc::new(HcGreekVerb::from_string(1, luw_correct, 0x0000, 0).unwrap());
        let a = HcGreekVerbForm {
            verb: verb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Plural),
            tense: HcTense::Aorist,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        let stem = a.adjust_stem("ἔδωκα", "δωκ", false);
        assert_eq!(Some(String::from("δο")), stem);
        let stem = a.adjust_stem("ἔθηκα", "θηκ", false);
        assert_eq!(Some(String::from("θε")), stem);
        let stem = a.adjust_stem("ἡκα", "ἡκ", false);
        assert_eq!(Some(String::from("εἱ")), stem);

        let a = HcGreekVerbForm {
            verb: verb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Aorist,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        let stem = a.adjust_stem("ἔδωκα", "δωκ", false);
        assert_eq!(Some(String::from("δωκ")), stem);
        let stem = a.adjust_stem("ἔθηκα", "θηκ", false);
        assert_eq!(Some(String::from("θηκ")), stem);
        let stem = a.adjust_stem("ἡκα", "ἡκ", false);
        assert_eq!(Some(String::from("ἡκ")), stem);
    }

    #[test]
    fn separate_prefixes_test() {
        let stem = "μετανισταμαι";
        let prefixes = separate_prefixes(stem);
        assert_eq!(Some(vec!["μετα", "ανα"]), prefixes);

        let stem = "ἀποδίδωμι";
        let prefixes = separate_prefixes(stem);
        assert_eq!(Some(vec!["ἀπο"]), prefixes);

        let stem = "ἵσταμαι";
        let prefixes = separate_prefixes(stem);
        assert_eq!(None, prefixes);
    }

    #[test]
    fn test_check_pps() {
        let luw = "λω, λσω, ἔλῡσα, λέλυκ, λέλυμαι, ἐλύθη";
        let luw_correct = "λω, λσω, ἔλῡσα, λέλυκα, λέλυμαι, ἐλύθην";
        let verb = HcGreekVerb::from_string(1, luw_correct, 0x0000, 0).unwrap();
        assert_eq!(
            check_pps(luw, &verb),
            vec![true, true, true, false, true, false]
        );

        let blaptw = "βλάπτω, βλάψω, ἔβλαψα, βέβλαφα, βέβλαμμαι, ἐβλάβην / ἐβλάφθην";
        let blaptw_correct = "βλάπτω, βλάψω, ἔβλαψα, βέβλαφα, βέβλαμμαι, ἐβλάβην / ἐβλάφθην";
        let verb2 = HcGreekVerb::from_string(1, blaptw_correct, 0x0000, 0).unwrap();
        assert_eq!(
            check_pps(blaptw, &verb2),
            vec![true, true, true, true, true, true]
        );

        let blaptw2 = "βλάπτω, βλάψω, ἔβλαψα, βέβλαφα, βέβλαμμαι, ἐβλάφθην / ἐβλάβην";
        assert_eq!(
            check_pps(blaptw2, &verb2),
            vec![true, true, true, true, true, true]
        );

        let blaptw3 = "βλάπτω, βλάψω, ἔβλαψα, βέβλαφα, βέβλαμμαι, ἐβλάφθην ; ἐβλάβην";
        assert_eq!(
            check_pps(blaptw3, &verb2),
            vec![true, true, true, true, true, true]
        );
    }

    #[test]
    fn test_analyze_syllables() {
        let word = "αι";
        let syllables = analyze_syllable_quantities(
            word,
            Some(HcPerson::First),
            Some(HcNumber::Singular),
            HcTense::Present,
            HcMood::Indicative,
            0,
        );
        assert_eq!(
            syllables,
            vec![SyllableAnalysis {
                letters: String::from("αι"),
                is_long: false,
                index: 0
            }]
        );

        let word = "βαιβω";
        let syllables = analyze_syllable_quantities(
            word,
            Some(HcPerson::First),
            Some(HcNumber::Singular),
            HcTense::Present,
            HcMood::Indicative,
            0,
        );
        assert_eq!(
            syllables,
            vec![
                SyllableAnalysis {
                    letters: String::from("αι"),
                    is_long: true,
                    index: 2
                },
                SyllableAnalysis {
                    letters: String::from("ω"),
                    is_long: true,
                    index: 0
                }
            ]
        );

        let word = "βῡβω";
        let syllables = analyze_syllable_quantities(
            word,
            Some(HcPerson::First),
            Some(HcNumber::Singular),
            HcTense::Present,
            HcMood::Indicative,
            0,
        );
        assert_eq!(
            syllables,
            vec![
                SyllableAnalysis {
                    letters: String::from("ῡ"),
                    is_long: true,
                    index: 2
                },
                SyllableAnalysis {
                    letters: String::from("ω"),
                    is_long: true,
                    index: 0
                }
            ]
        );

        let word = "βυιβω";
        let syllables = analyze_syllable_quantities(
            word,
            Some(HcPerson::First),
            Some(HcNumber::Singular),
            HcTense::Present,
            HcMood::Indicative,
            0,
        );
        assert_eq!(
            syllables,
            vec![
                SyllableAnalysis {
                    letters: String::from("υι"),
                    is_long: true,
                    index: 2
                },
                SyllableAnalysis {
                    letters: String::from("ω"),
                    is_long: true,
                    index: 0
                }
            ]
        );
    }

    #[test]
    fn test_param_hash() {
        let luw = "λω, λσω, ἔλῡσα, λέλυκα, λέλυμαι, ἐλύθην";
        let verb = Arc::new(HcGreekVerb::from_string(1, luw, REGULAR, 0).unwrap());
        let a = HcGreekVerbForm {
            verb: verb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Future,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };

        //b has different params from a
        let mut b = HcGreekVerbForm {
            verb: verb.clone(),
            person: Some(HcPerson::Third),
            number: Some(HcNumber::Plural),
            tense: HcTense::Present,
            voice: HcVoice::Passive,
            mood: HcMood::Imperative,
            gender: None,
            case: None,
        };

        let hash = a.param_hash();
        b.extract_params_from_hash(hash); //this should set b's params to equal a: thus the forms are equal
                                          //test round trip to param hash to form again
        assert_eq!(a, b);
    }
    /*
        #[test]
        fn test_oida() {
            // All elements can be initialized to the same value.
            let mut results: [usize; 432] = [0; 432];

            let oida = "οἶδα, εἴσομαι, —, —, —, —";
            let verb = Arc::new(HcGreekVerb::from_string(1, oida, REGULAR, 0).unwrap());
            let a = HcGreekVerbForm {
                verb: verb.clone(),
                person: Some(HcPerson::First),
                number: Some(HcNumber::Singular),
                tense: HcTense::Perfect,
                voice: HcVoice::Active,
                mood: HcMood::Indicative,
                gender: None,
                case: None,
            };

            let first_hash = a.param_hash() as usize;
            let mut idx: usize = 0;

            let max_changes = 2;
            let highest_unit = 20;
            let verb_params = VerbParameters {
                persons: vec![HcPerson::First, HcPerson::Second, HcPerson::Third],
                numbers: vec![HcNumber::Singular, HcNumber::Plural],
                tenses: vec![
                    HcTense::Present,
                    HcTense::Imperfect,
                    HcTense::Future,
                    HcTense::Aorist,
                    HcTense::Perfect,
                    HcTense::Pluperfect,
                ],
                voices: vec![HcVoice::Active, HcVoice::Middle, HcVoice::Passive],
                moods: vec![
                    HcMood::Indicative,
                    HcMood::Subjunctive,
                    HcMood::Optative,
                    HcMood::Imperative,
                ],
            };

            //let mut form_filter:HashSet<u32> = HashSet::new();
            // form_filter.insert(b.param_hash());
            // form_filter.insert(c.param_hash());

            let count = 100_000;

            for i in 0..count {
                let (a, _diag) = a.random_form(max_changes, Some(highest_unit), &verb_params, None);
                println!(
                    "{} {}",
                    a.param_hash(),
                    a.get_form(false).unwrap().last().unwrap().form
                );
                idx = a.param_hash() as usize;
                results[idx] += 1;

                // if i % 10 == 0 {
                //     form_filter.clear();
                // }
                //form_filter.insert(d.param_hash());
                //assert!(!form_filter.contains(&d.param_hash()));
                //assert_ne!(d.param_hash(), c.param_hash()); //the random form should never equal c because c was added to filter HashSet
            }
            //assert_eq!(0, results[0]);

            let mut b = HcGreekVerbForm {
                verb: verb.clone(),
                person: Some(HcPerson::Second),
                number: Some(HcNumber::Singular),
                tense: HcTense::Perfect,
                voice: HcVoice::Active,
                mood: HcMood::Indicative,
                gender: None,
                case: None,
            };
            let m: Vec<(usize, f64)> = results
                .iter()
                .enumerate()
                .map(|(i, e)| (i, (*e as f64 / count as f64)))
                .filter(|e| e.1 != 0.0)
                .collect();

            //println!("count {} {:?}", results[b.param_hash() as usize], m);
            for i in m {
                b.extract_params_from_hash(i.0.try_into().unwrap());
                println!("{} {:?}", i.0, b);
            }
            assert!(
                results[b.param_hash() as usize] / count > 30 / count
                    && results[b.param_hash() as usize] / count < 34 / count,
                "b.param_hash() {}",
                results[b.param_hash() as usize] / count
            );
        }
    */

    #[test]
    fn test_random2() {
        let luw = "λω, λσω, ἔλῡσα, λέλυκα, λέλυμαι, ἐλύθην";
        let verb = Arc::new(HcGreekVerb::from_string(1, luw, REGULAR, 0).unwrap());
        let a = HcGreekVerbForm {
            verb: verb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Future,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };

        let max_changes = 2;
        let highest_unit = 2;
        let verb_params = VerbParameters {
            persons: vec![HcPerson::First, HcPerson::Second, HcPerson::Third],
            numbers: vec![HcNumber::Singular, HcNumber::Plural],
            tenses: vec![
                HcTense::Present,
                HcTense::Imperfect,
                HcTense::Future,
                HcTense::Aorist,
                HcTense::Perfect,
                HcTense::Pluperfect,
            ],
            voices: vec![HcVoice::Active, HcVoice::Middle, HcVoice::Passive],
            moods: vec![
                HcMood::Indicative,
                HcMood::Subjunctive,
                HcMood::Optative,
                HcMood::Imperative,
            ],
        };

        let mut form_filter = HashSet::new();
        // form_filter.insert(b.param_hash());
        // form_filter.insert(c.param_hash());

        for _i in 0..10 {
            let (d, _diag) = a.random_form(
                max_changes,
                Some(highest_unit),
                &verb_params,
                Some(&form_filter),
            );
            form_filter.insert(d.param_hash());
            //assert!(!form_filter.contains(&d.param_hash()));
            //assert_ne!(d.param_hash(), c.param_hash()); //the random form should never equal c because c was added to filter HashSet
        }
    }

    #[test]
    fn test_random() {
        let luw = "λω, λσω, ἔλῡσα, λέλυκα, λέλυμαι, ἐλύθην";
        let verb = Arc::new(HcGreekVerb::from_string(1, luw, REGULAR, 0).unwrap());
        let a = HcGreekVerbForm {
            verb: verb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Future,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        let b = HcGreekVerbForm {
            verb: verb.clone(),
            person: Some(HcPerson::Second),
            number: Some(HcNumber::Singular),
            tense: HcTense::Future,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        let c = HcGreekVerbForm {
            verb: verb.clone(),
            person: Some(HcPerson::Third),
            number: Some(HcNumber::Singular),
            tense: HcTense::Future,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };

        let max_changes = 1;
        let highest_unit = 2;
        let verb_params = VerbParameters {
            persons: vec![HcPerson::First, HcPerson::Second, HcPerson::Third],
            numbers: vec![HcNumber::Singular, HcNumber::Plural],
            tenses: vec![HcTense::Present, HcTense::Imperfect],
            voices: vec![HcVoice::Active],
            moods: vec![HcMood::Indicative],
        };

        let mut form_filter = HashSet::new();
        form_filter.insert(b.param_hash());
        form_filter.insert(c.param_hash());

        for _i in 0..10_000 {
            let (d, _diag) = a.random_form(
                max_changes,
                Some(highest_unit),
                &verb_params,
                Some(&form_filter),
            );
            assert!(!form_filter.contains(&d.param_hash()));
            assert_ne!(d.param_hash(), c.param_hash()); //the random form should never equal c because c was added to filter HashSet
        }
    }

    #[test]
    fn test_change_param_block_last_param_change() {
        let luw = "λω, λσω, ἔλῡσα, λέλυκα, λέλυμαι, ἐλύθην";
        let verb = Arc::new(HcGreekVerb::from_string(1, luw, REGULAR, 0).unwrap());
        let mut a = HcGreekVerbForm {
            verb: verb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };

        let num_changes = 2;
        let parameters = VerbParameters {
            persons: vec![HcPerson::First, HcPerson::Second, HcPerson::Third],
            numbers: vec![HcNumber::Singular, HcNumber::Plural],
            tenses: vec![
                HcTense::Present,
                HcTense::Imperfect,
                HcTense::Future,
                HcTense::Aorist,
                HcTense::Perfect,
                HcTense::Pluperfect,
            ],
            voices: vec![HcVoice::Active, HcVoice::Middle, HcVoice::Passive],
            moods: vec![
                HcMood::Indicative,
                HcMood::Subjunctive,
                HcMood::Optative,
                HcMood::Imperative,
            ],
        };

        let count = 10_000;
        for _i in 0..count {
            a.change_params(num_changes, &parameters, &mut [HcParameters::Tense]);
            assert_eq!(a.tense, HcTense::Present); //don't change tense if tense is passed in above
        }
    }

    #[test]
    fn test_random_param_change_distribution() {
        let mut persons = [0, 0, 0];
        let mut numbers = [0, 0];
        let mut tenses = [0, 0, 0, 0, 0, 0];
        let mut moods = [0, 0, 0, 0];
        let mut voices = [0, 0, 0];
        let mut param_hash = 0;

        let luw = "λω, λσω, ἔλῡσα, λέλυκα, λέλυμαι, ἐλύθην";
        let verb = Arc::new(HcGreekVerb::from_string(1, luw, REGULAR, 0).unwrap());
        let mut a = HcGreekVerbForm {
            verb: verb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Future,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };

        let num_changes = 2;
        let parameters = VerbParameters {
            persons: vec![HcPerson::First, HcPerson::Second, HcPerson::Third],
            numbers: vec![HcNumber::Singular, HcNumber::Plural],
            tenses: vec![
                HcTense::Present,
                HcTense::Imperfect,
                HcTense::Future,
                HcTense::Aorist,
                HcTense::Perfect,
                HcTense::Pluperfect,
            ],
            voices: vec![HcVoice::Active, HcVoice::Middle, HcVoice::Passive],
            moods: vec![
                HcMood::Indicative,
                HcMood::Subjunctive,
                HcMood::Optative,
                HcMood::Imperative,
            ],
        };

        let count = 100_000;
        for _i in 0..count {
            a.change_params(num_changes, &parameters, &mut []);
            persons[a.person.unwrap().to_i16() as usize] += 1;
            numbers[a.number.unwrap().to_i16() as usize] += 1;
            tenses[a.tense.to_i16() as usize] += 1;
            moods[a.mood.to_i16() as usize] += 1;
            voices[a.voice.to_i16() as usize] += 1;

            param_hash += a.param_hash();
        }
        //sum of hash divided by count should be half of total number of possible forms (432 = 216)
        assert!(
            (param_hash as f64 / count as f64) > 214.0
                && (param_hash as f64 / count as f64) < 218.0
        );

        //check distribution of each param:
        assert!(
            (persons[0] as f64 / count as f64) > 0.31 && (persons[0] as f64 / count as f64) < 0.35
        );
        assert!(
            (persons[1] as f64 / count as f64) > 0.31 && (persons[1] as f64 / count as f64) < 0.35
        );
        assert!(
            (persons[2] as f64 / count as f64) > 0.31 && (persons[2] as f64 / count as f64) < 0.35
        );

        assert!(
            (numbers[0] as f64 / count as f64) > 0.48 && (numbers[0] as f64 / count as f64) < 0.52
        );
        assert!(
            (numbers[1] as f64 / count as f64) > 0.48 && (numbers[1] as f64 / count as f64) < 0.52
        );

        assert!(
            (tenses[0] as f64 / count as f64) > 0.14 && (tenses[0] as f64 / count as f64) < 0.18
        );
        assert!(
            (tenses[1] as f64 / count as f64) > 0.14 && (tenses[1] as f64 / count as f64) < 0.18
        );
        assert!(
            (tenses[2] as f64 / count as f64) > 0.14 && (tenses[2] as f64 / count as f64) < 0.18
        );
        assert!(
            (tenses[3] as f64 / count as f64) > 0.14 && (tenses[3] as f64 / count as f64) < 0.18
        );
        assert!(
            (tenses[4] as f64 / count as f64) > 0.14 && (tenses[4] as f64 / count as f64) < 0.18
        );
        assert!(
            (tenses[5] as f64 / count as f64) > 0.14 && (tenses[5] as f64 / count as f64) < 0.18
        );

        assert!((moods[0] as f64 / count as f64) > 0.23 && (moods[0] as f64 / count as f64) < 0.27);
        assert!((moods[1] as f64 / count as f64) > 0.23 && (moods[1] as f64 / count as f64) < 0.27);
        assert!((moods[2] as f64 / count as f64) > 0.23 && (moods[2] as f64 / count as f64) < 0.27);
        assert!((moods[3] as f64 / count as f64) > 0.23 && (moods[3] as f64 / count as f64) < 0.27);

        assert!(
            (voices[0] as f64 / count as f64) > 0.31 && (voices[0] as f64 / count as f64) < 0.35
        );
        assert!(
            (voices[1] as f64 / count as f64) > 0.31 && (voices[1] as f64 / count as f64) < 0.35
        );
        assert!(
            (voices[2] as f64 / count as f64) > 0.31 && (voices[2] as f64 / count as f64) < 0.35
        );
    }

    //same as above, but with oida = same results
    #[test]
    fn test_random_param_change_distribution_oida() {
        let mut persons = [0, 0, 0];
        let mut numbers = [0, 0];
        let mut tenses = [0, 0, 0, 0, 0, 0];
        let mut moods = [0, 0, 0, 0];
        let mut voices = [0, 0, 0];
        let mut param_hash = 0;

        let oida = "οἶδα, εἴσομαι, —, —, —, —";
        let verb = Arc::new(HcGreekVerb::from_string(1, oida, REGULAR, 0).unwrap());
        let mut a = HcGreekVerbForm {
            verb: verb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Future,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };

        let num_changes = 2;
        let parameters = VerbParameters {
            persons: vec![HcPerson::First, HcPerson::Second, HcPerson::Third],
            numbers: vec![HcNumber::Singular, HcNumber::Plural],
            tenses: vec![
                HcTense::Present,
                HcTense::Imperfect,
                HcTense::Future,
                HcTense::Aorist,
                HcTense::Perfect,
                HcTense::Pluperfect,
            ],
            voices: vec![HcVoice::Active, HcVoice::Middle, HcVoice::Passive],
            moods: vec![
                HcMood::Indicative,
                HcMood::Subjunctive,
                HcMood::Optative,
                HcMood::Imperative,
            ],
        };

        let count = 100_000;
        for _i in 0..count {
            a.change_params(num_changes, &parameters, &mut []);
            persons[a.person.unwrap().to_i16() as usize] += 1;
            numbers[a.number.unwrap().to_i16() as usize] += 1;
            tenses[a.tense.to_i16() as usize] += 1;
            moods[a.mood.to_i16() as usize] += 1;
            voices[a.voice.to_i16() as usize] += 1;

            param_hash += a.param_hash();
        }
        //sum of hash divided by count should be half of total number of possible forms (432 = 216)
        assert!(
            (param_hash as f64 / count as f64) > 214.0
                && (param_hash as f64 / count as f64) < 218.0
        );

        //check distribution of each param:
        assert!(
            (persons[0] as f64 / count as f64) > 0.31 && (persons[0] as f64 / count as f64) < 0.35
        );
        assert!(
            (persons[1] as f64 / count as f64) > 0.31 && (persons[1] as f64 / count as f64) < 0.35
        );
        assert!(
            (persons[2] as f64 / count as f64) > 0.31 && (persons[2] as f64 / count as f64) < 0.35
        );

        assert!(
            (numbers[0] as f64 / count as f64) > 0.48 && (numbers[0] as f64 / count as f64) < 0.52
        );
        assert!(
            (numbers[1] as f64 / count as f64) > 0.48 && (numbers[1] as f64 / count as f64) < 0.52
        );

        assert!(
            (tenses[0] as f64 / count as f64) > 0.14 && (tenses[0] as f64 / count as f64) < 0.18
        );
        assert!(
            (tenses[1] as f64 / count as f64) > 0.14 && (tenses[1] as f64 / count as f64) < 0.18
        );
        assert!(
            (tenses[2] as f64 / count as f64) > 0.14 && (tenses[2] as f64 / count as f64) < 0.18
        );
        assert!(
            (tenses[3] as f64 / count as f64) > 0.14 && (tenses[3] as f64 / count as f64) < 0.18
        );
        assert!(
            (tenses[4] as f64 / count as f64) > 0.14 && (tenses[4] as f64 / count as f64) < 0.18
        );
        assert!(
            (tenses[5] as f64 / count as f64) > 0.14 && (tenses[5] as f64 / count as f64) < 0.18
        );

        assert!((moods[0] as f64 / count as f64) > 0.23 && (moods[0] as f64 / count as f64) < 0.27);
        assert!((moods[1] as f64 / count as f64) > 0.23 && (moods[1] as f64 / count as f64) < 0.27);
        assert!((moods[2] as f64 / count as f64) > 0.23 && (moods[2] as f64 / count as f64) < 0.27);
        assert!((moods[3] as f64 / count as f64) > 0.23 && (moods[3] as f64 / count as f64) < 0.27);

        assert!(
            (voices[0] as f64 / count as f64) > 0.31 && (voices[0] as f64 / count as f64) < 0.35
        );
        assert!(
            (voices[1] as f64 / count as f64) > 0.31 && (voices[1] as f64 / count as f64) < 0.35
        );
        assert!(
            (voices[2] as f64 / count as f64) > 0.31 && (voices[2] as f64 / count as f64) < 0.35
        );
    }

    #[test]
    fn test_participles() {
        let luw = "λω, λσω, ἔλῡσα, λέλυκα, λέλυμαι, ἐλύθην";
        let a = Arc::new(HcGreekVerb::from_string(1, luw, REGULAR, 0).unwrap());
        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Active,
            mood: HcMood::Participle,
            gender: Some(HcGender::Masculine),
            case: Some(HcCase::Nominative),
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "λῡ́ων");

        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Active,
            mood: HcMood::Participle,
            gender: Some(HcGender::Masculine),
            case: Some(HcCase::Genitive),
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "λῡ́οντος");

        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: Some(HcNumber::Singular),
            tense: HcTense::Future,
            voice: HcVoice::Active,
            mood: HcMood::Participle,
            gender: Some(HcGender::Masculine),
            case: Some(HcCase::Nominative),
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "λῡ́σων");

        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: Some(HcNumber::Singular),
            tense: HcTense::Aorist,
            voice: HcVoice::Active,
            mood: HcMood::Participle,
            gender: Some(HcGender::Masculine),
            case: Some(HcCase::Nominative),
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "λῡ́σᾱς");

        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: Some(HcNumber::Singular),
            tense: HcTense::Aorist,
            voice: HcVoice::Passive,
            mood: HcMood::Participle,
            gender: Some(HcGender::Masculine),
            case: Some(HcCase::Nominative),
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "λυθείς");

        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: Some(HcNumber::Singular),
            tense: HcTense::Perfect,
            voice: HcVoice::Active,
            mood: HcMood::Participle,
            gender: Some(HcGender::Masculine),
            case: Some(HcCase::Nominative),
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "λελυκώς");
    }

    #[test]
    fn test_infinitives() {
        //*consonant stem perfects
        //*contracted
        //*first aorist
        //mi verbs
        //exceptions apothnhskw alternates, etc
        let luw = "λω, λσω, ἔλῡσα, λέλυκα, λέλυμαι, ἐλύθην";
        let a = Arc::new(HcGreekVerb::from_string(1, luw, REGULAR, 0).unwrap());
        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: None,
            tense: HcTense::Aorist,
            voice: HcVoice::Passive,
            mood: HcMood::Infinitive,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "λυθῆναι");

        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: None,
            tense: HcTense::Present,
            voice: HcVoice::Active,
            mood: HcMood::Infinitive,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "λῡ́ειν");

        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: None,
            tense: HcTense::Present,
            voice: HcVoice::Middle,
            mood: HcMood::Infinitive,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "λῡ́εσθαι");

        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: None,
            tense: HcTense::Aorist,
            voice: HcVoice::Active,
            mood: HcMood::Infinitive,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "λῦσαι");

        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: None,
            tense: HcTense::Aorist,
            voice: HcVoice::Middle,
            mood: HcMood::Infinitive,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "λῡ́σασθαι");

        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: None,
            tense: HcTense::Perfect,
            voice: HcVoice::Active,
            mood: HcMood::Infinitive,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "λελυκέναι");

        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: None,
            tense: HcTense::Perfect,
            voice: HcVoice::Middle,
            mood: HcMood::Infinitive,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "λελύσθαι");

        let consonant_stem = "βλάπτω, βλάψω, ἔβλαψα, βέβλαφα, βέβλαμμαι, ἐβλάβην / ἐβλάφθην";
        let a = Arc::new(
            HcGreekVerb::from_string(1, consonant_stem, CONSONANT_STEM_PERFECT_BETA, 0).unwrap(),
        );
        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: None,
            tense: HcTense::Perfect,
            voice: HcVoice::Middle,
            mood: HcMood::Infinitive,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "βεβλάφθαι");

        let consonant_stem = "πέμπω, πέμψω, ἔπεμψα, πέπομφα, πέπεμμαι, ἐπέμφθην";
        let a = Arc::new(
            HcGreekVerb::from_string(1, consonant_stem, CONSONANT_STEM_PERFECT_MU_PI, 0).unwrap(),
        );
        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: None,
            tense: HcTense::Perfect,
            voice: HcVoice::Middle,
            mood: HcMood::Infinitive,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "πεπέμφθαι");

        let consonant_stem = "θάπτω, θάψω, ἔθαψα, —, τέθαμμαι, ἐτάφην";
        let a = Arc::new(
            HcGreekVerb::from_string(1, consonant_stem, CONSONANT_STEM_PERFECT_PI, 0).unwrap(),
        );
        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: None,
            tense: HcTense::Perfect,
            voice: HcVoice::Middle,
            mood: HcMood::Infinitive,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "τεθάφθαι");

        let consonant_stem = "τάττω, τάξω, ἔταξα, τέταχα, τέταγμαι, ἐτάχθην";
        let a = Arc::new(
            HcGreekVerb::from_string(1, consonant_stem, CONSONANT_STEM_PERFECT_GAMMA, 0).unwrap(),
        );
        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: None,
            tense: HcTense::Perfect,
            voice: HcVoice::Middle,
            mood: HcMood::Infinitive,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "τετάχθαι");

        let consonant_stem = "ἄρχω, ἄρξω, ἦρξα, ἦρχα, ἦργμαι, ἤρχθην";
        let a = Arc::new(
            HcGreekVerb::from_string(1, consonant_stem, CONSONANT_STEM_PERFECT_CHI, 0).unwrap(),
        );
        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: None,
            tense: HcTense::Perfect,
            voice: HcVoice::Middle,
            mood: HcMood::Infinitive,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "ἦρχθαι");

        let consonant_stem = "ἀγγέλλω, ἀγγελῶ, ἤγγειλα, ἤγγελκα, ἤγγελμαι, ἠγγέλθην";
        let a = Arc::new(
            HcGreekVerb::from_string(1, consonant_stem, CONSONANT_STEM_PERFECT_LAMBDA, 0).unwrap(),
        );
        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: None,
            tense: HcTense::Perfect,
            voice: HcVoice::Middle,
            mood: HcMood::Infinitive,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "ἠγγέλθαι");

        let consonant_stem = "φαίνω, φανῶ, ἔφηνα, πέφηνα, πέφασμαι, ἐφάνην";
        let a = Arc::new(
            HcGreekVerb::from_string(1, consonant_stem, CONSONANT_STEM_PERFECT_NU, 0).unwrap(),
        );
        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: None,
            tense: HcTense::Perfect,
            voice: HcVoice::Middle,
            mood: HcMood::Infinitive,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "πεφάνθαι");

        let consonant_stem = "κελεύω, κελεύσω, ἐκέλευσα, κεκέλευκα, κεκέλευσμαι, ἐκελεύσθην";
        let a = Arc::new(
            HcGreekVerb::from_string(1, consonant_stem, CONSONANT_STEM_PERFECT_SIGMA, 0).unwrap(),
        );
        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: None,
            tense: HcTense::Perfect,
            voice: HcVoice::Middle,
            mood: HcMood::Infinitive,
            gender: None,
            case: None,
        };
        assert_eq!(
            b.get_form(false).unwrap().last().unwrap().form,
            "κεκελεῦσθαι"
        );

        let consonant_stem = "πέμπω, πέμψω, ἔπεμψα, πέπομφα, πέπεμμαι, ἐπέμφθην";
        let a = Arc::new(
            HcGreekVerb::from_string(1, consonant_stem, CONSONANT_STEM_PERFECT_MU_PI, 0).unwrap(),
        );
        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: None,
            tense: HcTense::Perfect,
            voice: HcVoice::Middle,
            mood: HcMood::Infinitive,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "πεπέμφθαι");

        let consonant_stem = "τῑμάω, τῑμήσω, ἐτῑ́μησα, τετῑ́μηκα, τετῑ́μημαι, ἐτῑμήθην";
        let a = Arc::new(HcGreekVerb::from_string(1, consonant_stem, REGULAR, 0).unwrap());
        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: None,
            tense: HcTense::Present,
            voice: HcVoice::Active,
            mood: HcMood::Infinitive,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "τῑμᾶν");

        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: None,
            tense: HcTense::Present,
            voice: HcVoice::Middle,
            mood: HcMood::Infinitive,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "τῑμᾶσθαι");

        let consonant_stem = "λείπω, λείψω, ἔλιπον, λέλοιπα, λέλειμμαι, ἐλείφθην";
        let a = Arc::new(HcGreekVerb::from_string(1, consonant_stem, REGULAR, 0).unwrap());
        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: None,
            tense: HcTense::Aorist,
            voice: HcVoice::Active,
            mood: HcMood::Infinitive,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "λιπεῖν");

        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: None,
            tense: HcTense::Aorist,
            voice: HcVoice::Middle,
            mood: HcMood::Infinitive,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "λιπέσθαι");

        let consonant_stem = "γίγνομαι, γενήσομαι, ἐγενόμην, γέγονα, γεγένημαι, —";
        let a = Arc::new(HcGreekVerb::from_string(1, consonant_stem, REGULAR, 0).unwrap());
        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: None,
            tense: HcTense::Aorist,
            voice: HcVoice::Middle,
            mood: HcMood::Infinitive,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "γενέσθαι");

        let consonant_stem = "τίθημι, θήσω, ἔθηκα, τέθηκα, τέθειμαι, ἐτέθην";
        let a = Arc::new(HcGreekVerb::from_string(1, consonant_stem, REGULAR, 0).unwrap());
        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: None,
            number: None,
            tense: HcTense::Present,
            voice: HcVoice::Active,
            mood: HcMood::Infinitive,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "τιθέναι");
    }

    #[test]
    fn test_strip_ending() {
        let luw = "λω, λσωd, ἔλῡσα, λέλυκα, λέλυμαι, ἐλύθην";
        let a = Arc::new(HcGreekVerb::from_string(1, luw, REGULAR, 0).unwrap());
        let b = HcGreekVerbForm {
            verb: a,
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Future,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        assert_eq!(
            b.get_form(false),
            Err(HcFormError::UnexpectedPrincipalPartEnding)
        );
    }

    #[test]
    fn test_rreplacen() {
        let s = "f2o f2o 123 foo".to_string();
        assert_eq!("f2o f2o 1new3 foo", s.rreplacen("2", "new", 1));
        assert_eq!("f2o fnewo 1new3 foo", s.rreplacen("2", "new", 2));
    }

    #[test]
    fn accent_tests() {
        let luw = "λω, λσω, ἔλῡσα, λέλυκα, λέλυμαι, ἐλύθην";
        let a = Arc::new(HcGreekVerb::from_string(1, luw, REGULAR, 0).unwrap());
        let b = HcGreekVerbForm {
            verb: a,
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Aorist,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap()[1].form, "ἔλῡσα");
        assert_eq!(b.accent_verb("λελυμαι"), "λέλυμαι");
        assert_eq!(b.accent_verb("λυ\u{0304}ε"), "λῦε");
        assert_eq!(b.accent_verb("λ\u{1FE1}ε"), "λῦε");
        assert_eq!(b.accent_verb("ἐβλαβην"), "ἐβλάβην");
    }

    #[test]
    fn normalization_tests() {
        let alphamacron_acute = "\u{1FB1}\u{0301}"; //alpha with macron + acute
        assert_eq!(
            alphamacron_acute.nfc().collect::<String>(),
            alphamacron_acute
        );

        let alpha_macron_acute = "\u{03B1}\u{0304}\u{0301}"; //alpha + macron + acute
        assert_eq!(
            alpha_macron_acute.nfc().collect::<String>(),
            alphamacron_acute
        ); //composes to alpha with macron + acute
        assert_ne!(
            alpha_macron_acute.nfc().collect::<String>(),
            alpha_macron_acute
        ); //does not compose to alpha + macron + acute

        assert_eq!(
            alphamacron_acute.nfd().collect::<String>(),
            alpha_macron_acute
        ); //decomposes to alpha + macron + acute

        //order matters here
        let alpha_acute_macron = "\u{03B1}\u{0301}\u{0304}"; //alpha + acute + macron
        assert_ne!(
            alpha_acute_macron.nfc().collect::<String>(),
            alphamacron_acute
        ); //does not compose to alpha with macron + acute, = alpha with acute + macron

        //order matters here too
        let alpha_smooth_acute = "\u{03B1}\u{0313}\u{0301}";
        assert_eq!(alpha_smooth_acute.nfc().collect::<String>(), "\u{1F04}");
        let alpha_acute_smooth = "\u{03B1}\u{0301}\u{0313}";
        assert_ne!(alpha_acute_smooth.nfc().collect::<String>(), "\u{1F04}");
    }

    #[test]
    fn illegal_verb_forms() {
        let luw = "λω, λσω, ἔλῡσα, λέλυκα, λέλυμαι, ἐλύθην";
        let luwverb = Arc::new(HcGreekVerb::from_string(1, luw, REGULAR, 0).unwrap());
        let illegal = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Perfect,
            voice: HcVoice::Active,
            mood: HcMood::Subjunctive,
            gender: None,
            case: None,
        };
        assert!(!illegal.is_legal_form());

        let illegal = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Active,
            mood: HcMood::Imperative,
            gender: None,
            case: None,
        };
        assert!(!illegal.is_legal_form());

        let illegal = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Active,
            mood: HcMood::Infinitive,
            gender: None,
            case: None,
        };
        assert!(!illegal.is_legal_form());

        let illegal = HcGreekVerbForm {
            verb: luwverb,
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Future,
            voice: HcVoice::Active,
            mood: HcMood::Optative,
            gender: None,
            case: None,
        };
        assert!(illegal.is_legal_form());

        let oida = "οἶδα, εἴσομαι, —, —, —, —";
        let oidaverb = Arc::new(HcGreekVerb::from_string(1, oida, REGULAR, 0).unwrap());
        let legaloidaperf = HcGreekVerbForm {
            verb: oidaverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Perfect,
            voice: HcVoice::Active,
            mood: HcMood::Subjunctive,
            gender: None,
            case: None,
        };
        assert!(legaloidaperf.is_legal_form());
        let legaloidaperf = HcGreekVerbForm {
            verb: oidaverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Perfect,
            voice: HcVoice::Active,
            mood: HcMood::Optative,
            gender: None,
            case: None,
        };
        assert!(legaloidaperf.is_legal_form());
        let legaloidaperf = HcGreekVerbForm {
            verb: oidaverb.clone(),
            person: Some(HcPerson::Second),
            number: Some(HcNumber::Singular),
            tense: HcTense::Perfect,
            voice: HcVoice::Active,
            mood: HcMood::Imperative,
            gender: None,
            case: None,
        };
        assert!(legaloidaperf.is_legal_form());
        let legaloidaperf = HcGreekVerbForm {
            verb: oidaverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Future,
            voice: HcVoice::Active,
            mood: HcMood::Subjunctive,
            gender: None,
            case: None,
        };
        assert!(!legaloidaperf.is_legal_form());
        let legaloidaperf = HcGreekVerbForm {
            verb: oidaverb.clone(),
            person: Some(HcPerson::Second),
            number: Some(HcNumber::Singular),
            tense: HcTense::Future,
            voice: HcVoice::Active,
            mood: HcMood::Imperative,
            gender: None,
            case: None,
        };
        assert!(!legaloidaperf.is_legal_form());
        let illegaloidaplup = HcGreekVerbForm {
            verb: oidaverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Pluperfect,
            voice: HcVoice::Active,
            mood: HcMood::Subjunctive,
            gender: None,
            case: None,
        };
        assert!(!illegaloidaplup.is_legal_form());
        let illegaloidaplup = HcGreekVerbForm {
            verb: oidaverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Pluperfect,
            voice: HcVoice::Active,
            mood: HcMood::Optative,
            gender: None,
            case: None,
        };
        assert!(!illegaloidaplup.is_legal_form());
        let illegaloidaplup = HcGreekVerbForm {
            verb: oidaverb.clone(),
            person: Some(HcPerson::Second),
            number: Some(HcNumber::Singular),
            tense: HcTense::Pluperfect,
            voice: HcVoice::Active,
            mood: HcMood::Imperative,
            gender: None,
            case: None,
        };
        assert!(!illegaloidaplup.is_legal_form());

        let illegaloidaplup = HcGreekVerbForm {
            verb: oidaverb.clone(),
            person: Some(HcPerson::Second),
            number: Some(HcNumber::Singular),
            tense: HcTense::Future,
            voice: HcVoice::Active,
            mood: HcMood::Optative,
            gender: None,
            case: None,
        };
        assert!(illegaloidaplup.is_legal_form());

        let illegaloidaplup = HcGreekVerbForm {
            verb: oidaverb,
            person: Some(HcPerson::Second),
            number: Some(HcNumber::Singular),
            tense: HcTense::Future,
            voice: HcVoice::Active,
            mood: HcMood::Imperative,
            gender: None,
            case: None,
        };
        assert!(!illegaloidaplup.is_legal_form());
    }

    #[test]
    fn block_for_hq_unit() {
        let luw = "λω, λσω, ἔλῡσα, λέλυκα, λέλυμαι, ἐλύθην";
        let luwverb = Arc::new(HcGreekVerb::from_string(1, luw, REGULAR, 0).unwrap());

        let vf = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Perfect,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        // block perfects for unit 2
        assert!(vf.block_for_hq_unit(Some(2)));
        // allow perfects for unit 3
        assert!(!vf.block_for_hq_unit(Some(3)));

        // subjunctive/optative in unit 3
        let vf = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Active,
            mood: HcMood::Subjunctive,
            gender: None,
            case: None,
        };
        assert!(vf.block_for_hq_unit(Some(2)));
        assert!(!vf.block_for_hq_unit(Some(3)));

        // subjunctive/optative in unit 3
        let vf = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Active,
            mood: HcMood::Optative,
            gender: None,
            case: None,
        };
        assert!(vf.block_for_hq_unit(Some(2)));
        assert!(!vf.block_for_hq_unit(Some(3)));

        // passive voice in unit 5
        let vf = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Passive,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        assert!(vf.block_for_hq_unit(Some(4)));
        assert!(!vf.block_for_hq_unit(Some(5)));

        // middle voice in unit 7
        let vf = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Middle,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        assert!(vf.block_for_hq_unit(Some(6)));
        assert!(!vf.block_for_hq_unit(Some(7)));

        // imperatives in unit 11
        let vf = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Active,
            mood: HcMood::Imperative,
            gender: None,
            case: None,
        };
        assert!(vf.block_for_hq_unit(Some(10)));
        assert!(!vf.block_for_hq_unit(Some(11)));

        // block mi verbs until unit 12
        let isthmi = "ἵστημι, στήσω, ἔστησα / ἔστην, ἕστηκα, ἕσταμαι, ἐστάθην";
        let isthmi_verb = Arc::new(HcGreekVerb::from_string(1, isthmi, REGULAR, 0).unwrap());
        let vf = HcGreekVerbForm {
            verb: isthmi_verb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        assert!(vf.block_for_hq_unit(Some(2)));
        assert!(vf.block_for_hq_unit(Some(3)));
        assert!(vf.block_for_hq_unit(Some(4)));
        assert!(vf.block_for_hq_unit(Some(5)));
        assert!(vf.block_for_hq_unit(Some(6)));
        assert!(vf.block_for_hq_unit(Some(7)));
        assert!(vf.block_for_hq_unit(Some(8)));
        assert!(vf.block_for_hq_unit(Some(9)));
        assert!(vf.block_for_hq_unit(Some(10)));
        assert!(vf.block_for_hq_unit(Some(11)));
        assert!(!vf.block_for_hq_unit(Some(12)));

        // block aorist of mi verbs until unit 13
        let vf = HcGreekVerbForm {
            verb: isthmi_verb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Aorist,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        assert!(vf.block_for_hq_unit(Some(12)));
        assert!(!vf.block_for_hq_unit(Some(13)));

        // block perfect of isthmi until unit 13
        let vf = HcGreekVerbForm {
            verb: isthmi_verb,
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Perfect,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        assert!(vf.block_for_hq_unit(Some(12)));
        assert!(!vf.block_for_hq_unit(Some(13)));

        // future optative, not until unit 16
        let vf = HcGreekVerbForm {
            verb: luwverb,
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Future,
            voice: HcVoice::Active,
            mood: HcMood::Optative,
            gender: None,
            case: None,
        };
        assert!(vf.block_for_hq_unit(Some(2)));
        assert!(vf.block_for_hq_unit(Some(3)));
        assert!(vf.block_for_hq_unit(Some(4)));
        assert!(vf.block_for_hq_unit(Some(5)));
        assert!(vf.block_for_hq_unit(Some(6)));
        assert!(vf.block_for_hq_unit(Some(7)));
        assert!(vf.block_for_hq_unit(Some(8)));
        assert!(vf.block_for_hq_unit(Some(9)));
        assert!(vf.block_for_hq_unit(Some(10)));
        assert!(vf.block_for_hq_unit(Some(11)));
        assert!(vf.block_for_hq_unit(Some(12)));
        assert!(vf.block_for_hq_unit(Some(13)));
        assert!(vf.block_for_hq_unit(Some(14)));
        assert!(vf.block_for_hq_unit(Some(15)));
        assert!(!vf.block_for_hq_unit(Some(16)));

        // 3rd plural consonant stem perfects
        let blaptw = "βλάπτω, βλάψω, ἔβλαψα, βέβλαφα, βέβλαμμαι, ἐβλάβην / ἐβλάφθην";
        let cons_stem_verb =
            Arc::new(HcGreekVerb::from_string(1, blaptw, CONSONANT_STEM_PERFECT_PI, 0).unwrap());
        let vf = HcGreekVerbForm {
            verb: cons_stem_verb.clone(),
            person: Some(HcPerson::Third),
            number: Some(HcNumber::Plural),
            tense: HcTense::Perfect,
            voice: HcVoice::Middle,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        assert!(cons_stem_verb.is_consonant_stem(""));

        assert!(vf.block_for_hq_unit(Some(2)));
        assert!(vf.block_for_hq_unit(Some(3)));
        assert!(vf.block_for_hq_unit(Some(4)));
        assert!(vf.block_for_hq_unit(Some(5)));
        assert!(vf.block_for_hq_unit(Some(6)));
        assert!(vf.block_for_hq_unit(Some(7)));
        assert!(vf.block_for_hq_unit(Some(8)));
        assert!(vf.block_for_hq_unit(Some(10)));
        assert!(vf.block_for_hq_unit(Some(11)));
        assert!(vf.block_for_hq_unit(Some(12)));
        assert!(vf.block_for_hq_unit(Some(13)));
        assert!(vf.block_for_hq_unit(Some(14)));
        assert!(vf.block_for_hq_unit(Some(15)));
        assert!(vf.block_for_hq_unit(Some(16)));
        assert!(vf.block_for_hq_unit(Some(17)));
        assert!(vf.block_for_hq_unit(Some(18)));
        assert!(vf.block_for_hq_unit(Some(19)));
        assert!(!vf.block_for_hq_unit(None));
        assert!(!vf.block_for_hq_unit(Some(20)));

        // but perfect active of consonant stems is ok
        let vf = HcGreekVerbForm {
            verb: cons_stem_verb,
            person: Some(HcPerson::Third),
            number: Some(HcNumber::Plural),
            tense: HcTense::Perfect,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        assert!(vf.block_for_hq_unit(Some(2)));
        assert!(!vf.block_for_hq_unit(Some(3)));
    }

    #[test]
    fn block_middle_passive() {
        let luw = "λω, λσω, ἔλῡσα, λέλυκα, λέλυμαι, ἐλύθην";
        let luwverb = Arc::new(HcGreekVerb::from_string(1, luw, REGULAR, 0).unwrap());

        let a = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Aorist,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        let b = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Aorist,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        // no change of voice: not blocked
        assert!(!a.block_middle_passive(&b));

        let a = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Middle,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        let b = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Passive,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        // middle to passive, both present tense: blocked
        assert!(a.block_middle_passive(&b));

        let a = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Passive,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        let b = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Middle,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        // passive to middle, both present tense: blocked
        assert!(a.block_middle_passive(&b));

        let a = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Aorist,
            voice: HcVoice::Passive,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        let b = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Middle,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        // passive to middle, first one is aorist: not blocked
        assert!(!a.block_middle_passive(&b));

        let a = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Passive,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        let b = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Aorist,
            voice: HcVoice::Middle,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        // passive to middle, second one is aorist: not blocked
        assert!(!a.block_middle_passive(&b));

        let a = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Future,
            voice: HcVoice::Passive,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        let b = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Middle,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        // passive to middle, first one is future: not blocked
        assert!(!a.block_middle_passive(&b));

        let a = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Passive,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        let b = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Future,
            voice: HcVoice::Middle,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        // passive to middle, second one is future: not blocked
        assert!(!a.block_middle_passive(&b));

        let a = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Future,
            voice: HcVoice::Passive,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        let b = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Future,
            voice: HcVoice::Middle,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        // passive to middle, both future: not blocked
        assert!(!a.block_middle_passive(&b));

        let a = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Passive,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        let b = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        // passive to active, both present: not blocked
        assert!(!a.block_middle_passive(&b));

        let a = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        let b = HcGreekVerbForm {
            verb: luwverb,
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Passive,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        // active to passive, both present: not blocked
        assert!(!a.block_middle_passive(&b));
    }

    #[test]
    fn it_works() {
        let luw = "λω, λσω, ἔλῡσα, λέλυκα, λέλυμαι, ἐλύθην";
        let blaptw = "βλάπτω, βλάψω, ἔβλαψα, βέβλαφα, βέβλαμμαι, ἐβλάβην / ἐβλάφθην";

        let luwverb = Arc::new(HcGreekVerb::from_string(1, luw, REGULAR, 0).unwrap());
        let a1 = Arc::new(HcGreekVerb {
            id: 1,
            pps: vec![
                "λω".to_string(),
                "λσω".to_string(),
                "ἔλῡσα".to_string(),
                "λέλυκα".to_string(),
                "λέλυμαι".to_string(),
                "ἐλύθην".to_string(),
            ],
            properties: REGULAR,
            hq_unit: 0,
        });
        assert_eq!(luwverb, a1);

        let b = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Aorist,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        let c = HcGreekVerbForm {
            verb: luwverb.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Aorist,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        assert_eq!(b, c);

        assert_eq!(b.get_form(false).unwrap()[0].form, luw);
        assert_eq!(b.get_form(false).unwrap()[1].form, "ἔλῡσα");

        assert_eq!(b.get_form(false).unwrap()[2].form, "ἐλῡσα");
        assert_eq!(b.get_form(false).unwrap().last().unwrap().form, "ἔλῡσα");

        assert_eq!(b.get_pp_num(), HcGreekPrincipalParts::Third);
        assert_eq!(b.get_pp_num() as usize, 3);
        assert_eq!(b.verb.pps[b.get_pp_num() as usize - 1], "ἔλῡσα");
        assert_eq!(b.get_pp(), Some(String::from("ἔλῡσα")));

        let a = Arc::new(HcGreekVerb::from_string(1, blaptw, REGULAR, 0).unwrap());
        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Aorist,
            voice: HcVoice::Passive,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap()[2].form, "ἐβλαβην / ἐβλαφθην");
        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap()[2].form, "βλαπτω");
        assert_eq!(b.get_endings("").unwrap()[0], "ω");

        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Middle,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap()[3].form, "βλάπτομαι");
        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: Some(HcPerson::Second),
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Middle,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_endings("").unwrap()[0], "ει");
        assert_eq!(b.get_endings("").unwrap()[1], "ῃ");
        assert_eq!(b.get_form(false).unwrap()[3].form, "βλάπτει / βλάπτῃ");
        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: Some(HcPerson::Third),
            number: Some(HcNumber::Singular),
            tense: HcTense::Present,
            voice: HcVoice::Middle,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap()[3].form, "βλάπτεται");
        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Plural),
            tense: HcTense::Present,
            voice: HcVoice::Middle,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap()[3].form, "βλαπτόμεθα");
        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: Some(HcPerson::Second),
            number: Some(HcNumber::Plural),
            tense: HcTense::Present,
            voice: HcVoice::Middle,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap()[3].form, "βλάπτεσθε");
        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: Some(HcPerson::Third),
            number: Some(HcNumber::Plural),
            tense: HcTense::Present,
            voice: HcVoice::Middle,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap()[3].form, "βλάπτονται");

        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Future,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap()[2].form, "βλαψω");
        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Perfect,
            voice: HcVoice::Active,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap()[2].form, "βεβλαφα");
        let b = HcGreekVerbForm {
            verb: a.clone(),
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Perfect,
            voice: HcVoice::Passive,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        assert_eq!(b.get_form(false).unwrap()[2].form, "βεβλαμμαι");

        let b = HcGreekVerbForm {
            verb: a,
            person: Some(HcPerson::First),
            number: Some(HcNumber::Singular),
            tense: HcTense::Pluperfect,
            voice: HcVoice::Passive,
            mood: HcMood::Indicative,
            gender: None,
            case: None,
        };
        assert_eq!(
            b.get_form(false).unwrap().last().unwrap().form,
            "ἐβεβλάμμην"
        );

        for v in [HcVoice::Active, HcVoice::Middle, HcVoice::Passive] {
            for x in [
                HcTense::Present,
                HcTense::Imperfect,
                HcTense::Future,
                HcTense::Aorist,
                HcTense::Perfect,
                HcTense::Pluperfect,
            ] {
                for m in [
                    HcMood::Indicative,
                    HcMood::Subjunctive,
                    HcMood::Optative,
                    HcMood::Imperative,
                ] {
                    if ((m == HcMood::Subjunctive
                        || m == HcMood::Optative
                        || m == HcMood::Imperative)
                        && (x == HcTense::Imperfect
                            || x == HcTense::Perfect
                            || x == HcTense::Pluperfect))
                        || x == HcTense::Future
                            && (m == HcMood::Subjunctive || m == HcMood::Imperative)
                    {
                        continue;
                    }
                    let mut line = Vec::new();
                    for z in [HcNumber::Singular, HcNumber::Plural] {
                        for y in [HcPerson::First, HcPerson::Second, HcPerson::Third] {
                            if m == HcMood::Imperative && y == HcPerson::First {
                                line.push("---".to_string());
                                continue;
                            }
                            let b = HcGreekVerbForm {
                                verb: luwverb.clone(),
                                person: Some(y),
                                number: Some(z),
                                tense: x,
                                voice: v,
                                mood: m,
                                gender: None,
                                case: None,
                            };
                            line.push(b.get_form(false).unwrap().last().unwrap().form.to_string());
                        }
                    }
                    println!("{}", line.join(", "));
                }
            }
        }
    }

    #[test]
    fn check_infinitives() {
        if let Ok(pp_file) = File::open("testdata/pp.txt") {
            let pp_reader = BufReader::new(pp_file);

            for (idx, pp_line) in pp_reader.lines().enumerate() {
                if let Ok(line) = pp_line {
                    let verb = Arc::new(
                        HcGreekVerb::from_string_with_properties(idx as u32, &line).unwrap(),
                    );

                    for x in [
                        HcTense::Present,
                        /*HcTense::Imperfect,*/
                        HcTense::Future,
                        HcTense::Aorist,
                        HcTense::Perfect,
                        /*HcTense::Pluperfect,*/
                    ] {
                        //for v in [HcVoice::Active, HcVoice::Middle, HcVoice::Passive] {
                        let forma = HcGreekVerbForm {
                            verb: verb.clone(),
                            person: None,
                            number: None,
                            tense: x,
                            voice: HcVoice::Active,
                            mood: HcMood::Infinitive,
                            gender: None,
                            case: None,
                        };
                        let ra = match forma.get_form(false) {
                            Ok(res) => res.last().unwrap().form.to_string(),
                            Err(_a) => "NF".to_string(),
                        };

                        let formm = HcGreekVerbForm {
                            verb: verb.clone(),
                            person: None,
                            number: None,
                            tense: x,
                            voice: HcVoice::Middle,
                            mood: HcMood::Infinitive,
                            gender: None,
                            case: None,
                        };
                        let rm = match formm.get_form(false) {
                            Ok(res) => res.last().unwrap().form.to_string(),
                            Err(_a) => "NF".to_string(),
                        };

                        let formp = HcGreekVerbForm {
                            verb: verb.clone(),
                            person: None,
                            number: None,
                            tense: x,
                            voice: HcVoice::Passive,
                            mood: HcMood::Infinitive,
                            gender: None,
                            case: None,
                        };
                        let rp = match formp.get_form(false) {
                            Ok(res) => res.last().unwrap().form.to_string(),
                            Err(_a) => "NF".to_string(),
                        };
                        println!(
                            "{} {} {}\t{}: {}, {}, {}",
                            verb.hq_unit,
                            verb.id,
                            x.value(),
                            verb.pps[0],
                            ra,
                            rm,
                            rp
                        );
                        //}
                    }
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
                        // if line.chars().nth(0) != Some('#') {
                        //     continue;
                        // }
                        let verb = Arc::new(
                            HcGreekVerb::from_string_with_properties(idx as u32, &line).unwrap(),
                        );

                        if paradigm_reader.read_line(&mut paradigm_line).unwrap() == 0 {
                            return;
                        }
                        paradigm_line.clear();

                        let partial = if verb.deponent_type() == HcDeponentType::NotDeponent {
                            "".to_string()
                        } else {
                            format!(" ({})", verb.deponent_type().value())
                        };

                        let verb_section = format!(
                            "Verb {}. {}{}",
                            idx,
                            if verb.pps[0] != "—" {
                                verb.pps[0].clone()
                            } else {
                                verb.pps[1].clone()
                            },
                            partial
                        );
                        println!("\n{}", verb_section);
                        if paradigm_reader.read_line(&mut paradigm_line).unwrap() != 0
                        /*&& idx != 77 && idx != 78*/
                        {
                            assert_eq!(paradigm_line[0..paradigm_line.len() - 1], verb_section);
                        }
                        paradigm_line.clear();

                        for x in [
                            HcTense::Present,
                            HcTense::Imperfect,
                            HcTense::Future,
                            HcTense::Aorist,
                            HcTense::Perfect,
                            HcTense::Pluperfect,
                        ] {
                            for v in [HcVoice::Active, HcVoice::Middle, HcVoice::Passive] {
                                for m in [
                                    HcMood::Indicative,
                                    HcMood::Subjunctive,
                                    HcMood::Optative,
                                    HcMood::Imperative,
                                ] {
                                    if ((m == HcMood::Subjunctive
                                        || m == HcMood::Optative
                                        || m == HcMood::Imperative)
                                        && (x == HcTense::Imperfect
                                            || x == HcTense::Perfect
                                            || x == HcTense::Pluperfect))
                                        || x == HcTense::Future
                                            && (m == HcMood::Subjunctive || m == HcMood::Imperative)
                                    {
                                        //allow moods for oida, synoida
                                        if !((m == HcMood::Subjunctive
                                            || m == HcMood::Optative
                                            || m == HcMood::Imperative)
                                            && x == HcTense::Perfect
                                            && v == HcVoice::Active
                                            && (verb.pps[0] == "οἶδα" || verb.pps[0] == "σύνοιδα"))
                                        {
                                            continue;
                                        }
                                    }

                                    if paradigm_reader.read_line(&mut paradigm_line).unwrap() == 0 {
                                        return;
                                    }
                                    paradigm_line.clear();

                                    let section = format!(
                                        "{} {} {}",
                                        x.value(),
                                        get_voice_label(x, v, m, verb.deponent_type()),
                                        m.value()
                                    );
                                    //if m == HcMood::Imperative { section = section.replacen(" (Middle/Passive)", "", 1)};
                                    println!("\n{}", section);
                                    if paradigm_reader.read_line(&mut paradigm_line).unwrap() != 0 {
                                        //assert_eq!(paradigm_line[0..paradigm_line.len() - 1], section);
                                    }
                                    paradigm_line.clear();

                                    for z in [HcNumber::Singular, HcNumber::Plural] {
                                        for y in
                                            [HcPerson::First, HcPerson::Second, HcPerson::Third]
                                        {
                                            let form = HcGreekVerbForm {
                                                verb: verb.clone(),
                                                person: Some(y),
                                                number: Some(z),
                                                tense: x,
                                                voice: v,
                                                mood: m,
                                                gender: None,
                                                case: None,
                                            };
                                            let r = match form.get_form(false) {
                                                Ok(res) => res.last().unwrap().form.to_string(),
                                                Err(_a) => "NF".to_string(),
                                            };

                                            let r_d = match form.get_form(true) {
                                                Ok(res) => res.last().unwrap().form.to_string(),
                                                Err(_a) => "NDF".to_string(),
                                            };

                                            let form_line = format!(
                                                "{}{}: {} ; {}",
                                                y.value(),
                                                z.value(),
                                                str::replace(&r, " /", ","),
                                                str::replace(&r_d, " /", ",")
                                            );

                                            println!("{}", form_line);

                                            if paradigm_reader
                                                .read_line(&mut paradigm_line)
                                                .unwrap()
                                                != 0
                                            /*&& idx != 77 && idx != 78*/
                                            {
                                                assert_eq!(
                                                    paradigm_line[0..paradigm_line.len() - 1], /* .nfc().collect::<String>()*/
                                                    form_line
                                                );
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

    #[test]
    fn finite_write_xml() {
        let mut form_id = 0;
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        if let Ok(pp_file) = File::open("testdata/pp.txt") {
            let pp_reader = BufReader::new(pp_file);

            let elem = BytesStart::new("greek-finite-verbs");
            writer.write_event(Event::Start(elem)).unwrap();

            for (verb_idx, pp_line) in pp_reader.lines().enumerate() {
                if let Ok(line) = pp_line {
                    let verb = Arc::new(
                        HcGreekVerb::from_string_with_properties((verb_idx + 1) as u32, &line)
                            .unwrap(),
                    );

                    let mut elem = BytesStart::new("verb");
                    elem.push_attribute(("id", (verb_idx + 1).to_string().as_str()));
                    elem.push_attribute(("label", verb.get_verb_lemma().as_str()));
                    elem.push_attribute(("unit", verb.hq_unit.to_string().as_str()));
                    elem.push_attribute(("deponent", verb.deponent_type().value()));
                    elem.push_attribute(("pps", verb.pps.join(", ").as_str()));
                    writer.write_event(Event::Start(elem)).unwrap();

                    for x in [
                        HcTense::Present,
                        HcTense::Imperfect,
                        HcTense::Future,
                        HcTense::Aorist,
                        HcTense::Perfect,
                        HcTense::Pluperfect,
                    ] {
                        for v in [HcVoice::Active, HcVoice::Middle, HcVoice::Passive] {
                            for m in [
                                HcMood::Indicative,
                                HcMood::Subjunctive,
                                HcMood::Optative,
                                HcMood::Imperative,
                            ] {
                                for z in [
                                    None,
                                    Some(HcNumber::Singular),
                                    Some(HcNumber::Dual),
                                    Some(HcNumber::Plural),
                                ] {
                                    for y in [
                                        None,
                                        Some(HcPerson::First),
                                        Some(HcPerson::Second),
                                        Some(HcPerson::Third),
                                    ] {
                                        let form = HcGreekVerbForm {
                                            verb: verb.clone(),
                                            person: y,
                                            number: z,
                                            tense: x,
                                            voice: v,
                                            mood: m,
                                            gender: None,
                                            case: None,
                                        };
                                        let form_result = form.get_form(false);
                                        let form_result_decomposed = form.get_form(true);

                                        let person_label = if form.person.is_some() {
                                            form.person.unwrap().value().to_string()
                                        } else {
                                            String::from("None")
                                        };
                                        let number_label = if form.number.is_some() {
                                            form.number.unwrap().value().to_string()
                                        } else {
                                            String::from("None")
                                        };

                                        let mut elem = BytesStart::new("form");
                                        form_id += 1;
                                        elem.push_attribute(("id", form_id.to_string().as_str()));
                                        elem.push_attribute(("person", person_label.as_str()));
                                        elem.push_attribute(("number", number_label.as_str()));
                                        elem.push_attribute(("tense", form.tense.value()));
                                        elem.push_attribute(("mood", form.mood.value()));
                                        elem.push_attribute(("voice", form.voice.value()));

                                        if let Err(ref res) = form_result {
                                            elem.push_attribute(("status", res.value()));
                                        }
                                        if let Err(ref res) = form_result_decomposed {
                                            elem.push_attribute(("status-decomposed", res.value()));
                                        }
                                        elem.push_attribute((
                                            "voice-label",
                                            get_voice_label(x, v, m, verb.deponent_type()).as_str(),
                                        ));

                                        writer.write_event(Event::Start(elem)).unwrap();
                                        if let Ok(res) = form_result {
                                            let elemf = BytesStart::new("f");
                                            writer.write_event(Event::Start(elemf)).unwrap();
                                            writer
                                                .write_event(Event::Text(BytesText::new(
                                                    &res.last()
                                                        .unwrap()
                                                        .form
                                                        .to_string()
                                                        .replace(" /", ","),
                                                )))
                                                .unwrap();
                                            writer
                                                .write_event(Event::End(BytesEnd::new("f")))
                                                .unwrap();
                                        }
                                        if let Ok(res) = form_result_decomposed {
                                            let elemd = BytesStart::new("d");
                                            writer.write_event(Event::Start(elemd)).unwrap();
                                            writer
                                                .write_event(Event::Text(BytesText::new(
                                                    &res.last()
                                                        .unwrap()
                                                        .form
                                                        .to_string()
                                                        .replace(" /", ","),
                                                )))
                                                .unwrap();
                                            writer
                                                .write_event(Event::End(BytesEnd::new("d")))
                                                .unwrap();
                                        }
                                        writer
                                            .write_event(Event::End(BytesEnd::new("form")))
                                            .unwrap();
                                    }
                                }
                            }
                        }
                    }
                }
                writer
                    .write_event(Event::End(BytesEnd::new("verb")))
                    .unwrap();
            }
            writer
                .write_event(Event::End(BytesEnd::new("greek-finite-verbs")))
                .unwrap();
            let result = writer.into_inner();

            if let Ok(file) = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open("testdata/greek-finite-verbs.xml")
            {
                let mut f = BufWriter::new(file);
                f.write_all(result).unwrap();
            }
        }
    }

    #[test]
    fn infinitive_write_xml() {
        let mut form_id = 0;
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        if let Ok(pp_file) = File::open("testdata/pp.txt") {
            let pp_reader = BufReader::new(pp_file);

            let elem = BytesStart::new("greek-infinitives");
            writer.write_event(Event::Start(elem)).unwrap();

            for (verb_idx, pp_line) in pp_reader.lines().enumerate() {
                if let Ok(line) = pp_line {
                    let verb = Arc::new(
                        HcGreekVerb::from_string_with_properties((verb_idx + 1) as u32, &line)
                            .unwrap(),
                    );

                    let mut elem = BytesStart::new("verb");
                    elem.push_attribute(("id", (verb_idx + 1).to_string().as_str()));
                    elem.push_attribute(("label", verb.get_verb_lemma().as_str()));
                    elem.push_attribute(("unit", verb.hq_unit.to_string().as_str()));
                    elem.push_attribute(("deponent", verb.deponent_type().value()));
                    elem.push_attribute(("pps", verb.pps.join(", ").as_str()));
                    writer.write_event(Event::Start(elem)).unwrap();

                    for x in [
                        HcTense::Present,
                        //HcTense::Imperfect,
                        HcTense::Future,
                        HcTense::Aorist,
                        HcTense::Perfect,
                        //HcTense::Pluperfect,
                    ] {
                        for v in [HcVoice::Active, HcVoice::Middle, HcVoice::Passive] {
                            let form = HcGreekVerbForm {
                                verb: verb.clone(),
                                person: None,
                                number: None,
                                tense: x,
                                voice: v,
                                mood: HcMood::Infinitive,
                                gender: None,
                                case: None,
                            };
                            let form_result = form.get_form(false);
                            let form_result_decomposed = form.get_form(true);

                            let person_label = if form.person.is_some() {
                                form.person.unwrap().value().to_string()
                            } else {
                                String::from("None")
                            };
                            let number_label = if form.number.is_some() {
                                form.number.unwrap().value().to_string()
                            } else {
                                String::from("None")
                            };

                            let mut elem = BytesStart::new("form");
                            form_id += 1;
                            elem.push_attribute(("id", form_id.to_string().as_str()));
                            elem.push_attribute(("person", person_label.as_str()));
                            elem.push_attribute(("number", number_label.as_str()));
                            elem.push_attribute(("tense", form.tense.value()));
                            elem.push_attribute(("mood", form.mood.value()));
                            elem.push_attribute(("voice", form.voice.value()));

                            if let Err(ref res) = form_result {
                                elem.push_attribute(("status", res.value()));
                            }
                            if let Err(ref res) = form_result_decomposed {
                                elem.push_attribute(("status-decomposed", res.value()));
                            }
                            elem.push_attribute((
                                "voice-label",
                                get_voice_label(x, v, HcMood::Infinitive, verb.deponent_type())
                                    .as_str(),
                            ));

                            writer.write_event(Event::Start(elem)).unwrap();
                            if let Ok(res) = form_result {
                                let elemf = BytesStart::new("f");
                                writer.write_event(Event::Start(elemf)).unwrap();
                                writer
                                    .write_event(Event::Text(BytesText::new(
                                        &res.last().unwrap().form.to_string().replace(" /", ","),
                                    )))
                                    .unwrap();
                                writer.write_event(Event::End(BytesEnd::new("f"))).unwrap();
                            }
                            if let Ok(res) = form_result_decomposed {
                                let elemd = BytesStart::new("d");
                                writer.write_event(Event::Start(elemd)).unwrap();
                                writer
                                    .write_event(Event::Text(BytesText::new(
                                        &res.last().unwrap().form.to_string().replace(" /", ","),
                                    )))
                                    .unwrap();
                                writer.write_event(Event::End(BytesEnd::new("d"))).unwrap();
                            }

                            writer
                                .write_event(Event::End(BytesEnd::new("form")))
                                .unwrap();
                            //println!("{}", form_line);
                        }
                    }
                }
                writer
                    .write_event(Event::End(BytesEnd::new("verb")))
                    .unwrap();
            }
            writer
                .write_event(Event::End(BytesEnd::new("greek-infinitives")))
                .unwrap();
            let result = writer.into_inner();

            if let Ok(file) = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open("testdata/greek-infinitives.xml")
            {
                let mut f = BufWriter::new(file);
                f.write_all(result).unwrap();
            }
        }
    }

    #[test]
    fn participle_write_xml() {
        let mut form_id = 0;
        let mut buffer = Vec::new();
        let mut writer = Writer::new_with_indent(&mut buffer, b' ', 4);

        if let Ok(pp_file) = File::open("testdata/pp.txt") {
            let pp_reader = BufReader::new(pp_file);

            let elem = BytesStart::new("greek-participles");
            writer.write_event(Event::Start(elem)).unwrap();

            for (verb_idx, pp_line) in pp_reader.lines().enumerate() {
                if let Ok(line) = pp_line {
                    let verb = Arc::new(
                        HcGreekVerb::from_string_with_properties((verb_idx + 1) as u32, &line)
                            .unwrap(),
                    );

                    let mut elem = BytesStart::new("verb");
                    elem.push_attribute(("id", (verb_idx + 1).to_string().as_str()));
                    elem.push_attribute(("label", verb.get_verb_lemma().as_str()));
                    elem.push_attribute(("unit", verb.hq_unit.to_string().as_str()));
                    elem.push_attribute(("deponent", verb.deponent_type().value()));
                    elem.push_attribute(("pps", verb.pps.join(", ").as_str()));
                    writer.write_event(Event::Start(elem)).unwrap();

                    for x in [
                        HcTense::Present,
                        HcTense::Future,
                        HcTense::Aorist,
                        HcTense::Perfect,
                    ] {
                        for v in [HcVoice::Active, HcVoice::Middle, HcVoice::Passive] {
                            for z in [
                                Some(HcNumber::Singular),
                                //Some(HcNumber::Dual),
                                Some(HcNumber::Plural),
                            ] {
                                for c in [
                                    Some(HcCase::Nominative),
                                    Some(HcCase::Genitive),
                                    Some(HcCase::Dative),
                                    Some(HcCase::Accusative),
                                    Some(HcCase::Vocative),
                                ] {
                                    for g in [
                                        Some(HcGender::Masculine),
                                        Some(HcGender::Feminine),
                                        Some(HcGender::Neuter),
                                    ] {
                                        let form = HcGreekVerbForm {
                                            verb: verb.clone(),
                                            person: None,
                                            number: z,
                                            tense: x,
                                            voice: v,
                                            mood: HcMood::Participle,
                                            gender: g,
                                            case: c,
                                        };
                                        let form_result = form.get_form(false);
                                        let form_result_decomposed = form.get_form(true);

                                        let person_label = if form.person.is_some() {
                                            form.person.unwrap().value().to_string()
                                        } else {
                                            String::from("None")
                                        };
                                        let number_label = if form.number.is_some() {
                                            form.number.unwrap().value().to_string()
                                        } else {
                                            String::from("None")
                                        };

                                        let mut elem = BytesStart::new("form");
                                        form_id += 1;
                                        elem.push_attribute(("id", form_id.to_string().as_str()));
                                        elem.push_attribute(("person", person_label.as_str()));
                                        elem.push_attribute(("number", number_label.as_str()));
                                        elem.push_attribute(("tense", form.tense.value()));
                                        elem.push_attribute(("mood", form.mood.value()));
                                        elem.push_attribute(("voice", form.voice.value()));
                                        elem.push_attribute(("case", form.case.unwrap().value()));
                                        elem.push_attribute((
                                            "gender",
                                            form.gender.unwrap().value(),
                                        ));

                                        if let Err(ref res) = form_result {
                                            elem.push_attribute(("status", res.value()));
                                        }
                                        if let Err(ref res) = form_result_decomposed {
                                            elem.push_attribute(("status-decomposed", res.value()));
                                        }
                                        elem.push_attribute((
                                            "voice-label",
                                            get_voice_label(
                                                x,
                                                v,
                                                HcMood::Participle,
                                                verb.deponent_type(),
                                            )
                                            .as_str(),
                                        ));

                                        writer.write_event(Event::Start(elem)).unwrap();
                                        if let Ok(res) = form_result {
                                            let elemf = BytesStart::new("f");
                                            writer.write_event(Event::Start(elemf)).unwrap();
                                            writer
                                                .write_event(Event::Text(BytesText::new(
                                                    &res.last()
                                                        .unwrap()
                                                        .form
                                                        .to_string()
                                                        .replace(" /", ","),
                                                )))
                                                .unwrap();
                                            writer
                                                .write_event(Event::End(BytesEnd::new("f")))
                                                .unwrap();
                                        }
                                        if let Ok(res) = form_result_decomposed {
                                            let elemd = BytesStart::new("d");
                                            writer.write_event(Event::Start(elemd)).unwrap();
                                            writer
                                                .write_event(Event::Text(BytesText::new(
                                                    &res.last()
                                                        .unwrap()
                                                        .form
                                                        .to_string()
                                                        .replace(" /", ","),
                                                )))
                                                .unwrap();
                                            writer
                                                .write_event(Event::End(BytesEnd::new("d")))
                                                .unwrap();
                                        }
                                        writer
                                            .write_event(Event::End(BytesEnd::new("form")))
                                            .unwrap();
                                    }
                                }
                            }
                        }
                    }
                }
                writer
                    .write_event(Event::End(BytesEnd::new("verb")))
                    .unwrap();
            }
            writer
                .write_event(Event::End(BytesEnd::new("greek-participles")))
                .unwrap();
            let result = writer.into_inner();

            if let Ok(file) = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open("testdata/greek-participles.xml")
            {
                let mut f = BufWriter::new(file);
                f.write_all(result).unwrap();
            }
        }
    }
}
