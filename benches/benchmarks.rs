use criterion::{/*black_box,*/ criterion_group, criterion_main, Criterion};
use hoplite_verbs_rs::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::Arc;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("check_forms benchmark", |b| b.iter(check_forms));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

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
                    //println!("\n{}", verb_section);
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

                                let _section = format!(
                                    "{} {} {}",
                                    x.value(),
                                    get_voice_label(x, v, m, verb.deponent_type()),
                                    m.value()
                                );
                                //if m == HcMood::Imperative { section = section.replacen(" (Middle/Passive)", "", 1)};
                                //println!("\n{}", section);
                                if paradigm_reader.read_line(&mut paradigm_line).unwrap() != 0 {
                                    //assert_eq!(paradigm_line[0..paradigm_line.len() - 1], section);
                                }
                                paradigm_line.clear();

                                for z in [HcNumber::Singular, HcNumber::Plural] {
                                    for y in [HcPerson::First, HcPerson::Second, HcPerson::Third] {
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

                                        //println!("{}", form_line);

                                        if paradigm_reader.read_line(&mut paradigm_line).unwrap()
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
