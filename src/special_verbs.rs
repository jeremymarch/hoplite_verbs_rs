use super::*;

pub fn get_esti(vf: &HcGreekVerbForm, _decompose: bool) -> String {
    let mut s = String::from("");
    if vf.person != Some(HcPerson::Third) || vf.number != Some(HcNumber::Singular) {
        return s;
    }

    if vf.tense == HcTense::Present {
        if vf.mood == HcMood::Indicative && vf.voice == HcVoice::Active {
            s = String::from("ἔστι(ν)");
        }
    } else if vf.tense == HcTense::Imperfect && vf.voice == HcVoice::Active {
        if vf.mood == HcMood::Indicative {
            s = String::from("ἦν");
        }
    } else if vf.tense == HcTense::Future && vf.voice == HcVoice::Middle {
        if vf.mood == HcMood::Indicative {
            s = String::from("ἔσται");
        }
    }
    s
}

pub fn get_exesti(vf: &HcGreekVerbForm, decompose: bool) -> String {
    let mut s = String::from("");
    if vf.person != Some(HcPerson::Third) || vf.number != Some(HcNumber::Singular) {
        return s;
    }

    if vf.tense == HcTense::Present {
        if vf.mood == HcMood::Indicative && vf.voice == HcVoice::Active {
            s = if decompose {
                format!("ἐξ {} εστι(ν)", SEPARATOR)
            } else {
                String::from("ἔξεστι(ν)")
            };
        }
    } else if vf.tense == HcTense::Imperfect && vf.voice == HcVoice::Active {
        if vf.mood == HcMood::Indicative {
            s = if decompose {
                format!("ἐξ {} ην", SEPARATOR)
            } else {
                String::from("ἐξῆν")
            };
        }
    } else if vf.tense == HcTense::Future && vf.voice == HcVoice::Middle {
        if vf.mood == HcMood::Indicative {
            s = if decompose {
                format!("ἐξ {} εσεται", SEPARATOR)
            } else {
                String::from("ἐξέσται")
            };
        }
    }
    s
}

pub fn get_dei(vf: &HcGreekVerbForm, decompose: bool) -> String {
    let mut s = String::from("");
    if vf.voice != HcVoice::Active
        || vf.person != Some(HcPerson::Third)
        || vf.number != Some(HcNumber::Singular)
    {
        return s;
    }

    if vf.tense == HcTense::Present {
        if vf.mood == HcMood::Indicative {
            s = if decompose {
                format!("δε {} ει", SEPARATOR)
            } else {
                String::from("δεῖ")
            };
        } else if vf.mood == HcMood::Subjunctive {
            s = if decompose {
                format!("δε {} ῃ", SEPARATOR)
            } else {
                String::from("δέῃ")
            };
        } else if vf.mood == HcMood::Optative {
            s = if decompose {
                format!("δε {} οι", SEPARATOR)
            } else {
                String::from("δέοι")
            };
        }
    } else if vf.tense == HcTense::Imperfect {
        if vf.mood == HcMood::Indicative {
            s = if decompose {
                format!("ε {} δε {} ε", SEPARATOR, SEPARATOR)
            } else {
                String::from("ἔδει")
            };
        }
    } else if vf.tense == HcTense::Future {
        if vf.mood == HcMood::Indicative {
            s = if decompose {
                format!("δεησ {} ει", SEPARATOR)
            } else {
                String::from("δεήσει")
            };
        }
    } else if vf.tense == HcTense::Aorist {
        if vf.mood == HcMood::Indicative {
            s = if decompose {
                format!("ε {} δεησ {} ε(ν)", SEPARATOR, SEPARATOR)
            } else {
                String::from("ἐδέησε(ν)")
            };
        }
    }
    s
}

pub fn get_xrh(vf: &HcGreekVerbForm, decompose: bool) -> String {
    let mut s = String::from("");
    if vf.voice != HcVoice::Active
        || vf.person != Some(HcPerson::Third)
        || vf.number != Some(HcNumber::Singular)
    {
        return s;
    }

    if vf.tense == HcTense::Present {
        if vf.mood == HcMood::Indicative {
            s = if decompose {
                String::from("χρή")
            } else {
                String::from("χρή")
            };
        } else if vf.mood == HcMood::Subjunctive {
            s = if decompose {
                format!("χρή {} ᾖ", SEPARATOR)
            } else {
                String::from("χρῇ")
            };
        } else if vf.mood == HcMood::Optative {
            s = if decompose {
                format!("χρή {} εἴη", SEPARATOR)
            } else {
                String::from("χρείη")
            };
        }
    } else if vf.tense == HcTense::Imperfect {
        if vf.mood == HcMood::Indicative {
            s = if decompose {
                format!("ε {} χρή {} ἦν", SEPARATOR, SEPARATOR)
            } else {
                String::from("ἐχρῆν, χρῆν")
            };
        }
    } else if vf.tense == HcTense::Future {
        if vf.mood == HcMood::Indicative {
            s = if decompose {
                format!("χρή {} ἔσται", SEPARATOR)
            } else {
                String::from("χρῆσται")
            };
        }
    }
    s
}

pub fn get_eimi(vf: &HcGreekVerbForm, _decompose: bool) -> String {
    if vf.voice != HcVoice::Active {
        return String::from("");
    }
    let mut s = String::from("");
    if vf.tense == HcTense::Present {
        if vf.mood == HcMood::Indicative {
            if vf.person == Some(HcPerson::First) {
                if vf.number == Some(HcNumber::Singular) {
                    s = String::from("εἰμί");
                } else {
                    s = String::from("ἐσμέν");
                }
            } else if vf.person == Some(HcPerson::Second) {
                if vf.number == Some(HcNumber::Singular) {
                    s = String::from("εἶ");
                } else {
                    s = String::from("ἐστέ");
                }
            } else if vf.person == Some(HcPerson::Third) {
                if vf.number == Some(HcNumber::Singular) {
                    s = String::from("ἐστί(ν)");
                } else {
                    s = String::from("εἰσί(ν)");
                }
            }
        } else if vf.mood == HcMood::Subjunctive {
            if vf.person == Some(HcPerson::First) {
                if vf.number == Some(HcNumber::Singular) {
                    s = String::from("ὦ");
                } else {
                    s = String::from("ὦμεν");
                }
            } else if vf.person == Some(HcPerson::Second) {
                if vf.number == Some(HcNumber::Singular) {
                    s = String::from("ᾖς");
                } else {
                    s = String::from("ἦτε");
                }
            } else if vf.person == Some(HcPerson::Third) {
                if vf.number == Some(HcNumber::Singular) {
                    s = String::from("ᾖ");
                } else {
                    s = String::from("ὦσι(ν)");
                }
            }
        } else if vf.mood == HcMood::Optative {
            if vf.person == Some(HcPerson::First) {
                if vf.number == Some(HcNumber::Singular) {
                    s = String::from("εἴην");
                } else {
                    s = String::from("εἶμεν, εἴημεν");
                }
            } else if vf.person == Some(HcPerson::Second) {
                if vf.number == Some(HcNumber::Singular) {
                    s = String::from("εἴης");
                } else {
                    s = String::from("εἶτε, εἴητε");
                }
            } else if vf.person == Some(HcPerson::Third) {
                if vf.number == Some(HcNumber::Singular) {
                    s = String::from("εἴη");
                } else {
                    s = String::from("εἶεν, εἴησαν");
                }
            }
        } else if vf.mood == HcMood::Imperative {
            if vf.person == Some(HcPerson::First) {
                if vf.number == Some(HcNumber::Singular) {
                    s = String::from("");
                } else {
                    s = String::from("");
                }
            } else if vf.person == Some(HcPerson::Second) {
                if vf.number == Some(HcNumber::Singular) {
                    s = String::from("ἴσθι");
                } else {
                    s = String::from("ἔστε");
                }
            } else if vf.person == Some(HcPerson::Third) {
                if vf.number == Some(HcNumber::Singular) {
                    s = String::from("ἔστω");
                } else {
                    s = String::from("ἔστων, ὄντων");
                }
            }
        }
    } else if vf.tense == HcTense::Imperfect {
        if vf.person == Some(HcPerson::First) {
            if vf.number == Some(HcNumber::Singular) {
                s = String::from("ἦ, ἦν");
            } else {
                s = String::from("ἦμεν");
            }
        } else if vf.person == Some(HcPerson::Second) {
            if vf.number == Some(HcNumber::Singular) {
                s = String::from("ἦσθα");
            } else {
                s = String::from("ἦτε");
            }
        } else if vf.person == Some(HcPerson::Third) {
            if vf.number == Some(HcNumber::Singular) {
                s = String::from("ἦν");
            } else {
                s = String::from("ἦσαν");
            }
        }
    }
    s
}

pub fn get_keimai(vf: &HcGreekVerbForm, decompose: bool) -> String {
    let mut s = String::from("");
    if vf.tense == HcTense::Present {
        if vf.mood == HcMood::Indicative {
            if vf.person == Some(HcPerson::First) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("κει {} μαι", SEPARATOR)
                    } else {
                        String::from("κεῖμαι")
                    };
                } else {
                    s = if decompose {
                        format!("κει {} μεθα", SEPARATOR)
                    } else {
                        String::from("κείμεθα")
                    };
                }
            } else if vf.person == Some(HcPerson::Second) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("κει {} σαι", SEPARATOR)
                    } else {
                        String::from("κεῖσαι")
                    };
                } else {
                    s = if decompose {
                        format!("κει {} σθε", SEPARATOR)
                    } else {
                        String::from("κεῖσθε")
                    };
                }
            } else if vf.person == Some(HcPerson::Third) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("κει {} ται", SEPARATOR)
                    } else {
                        String::from("κεῖται")
                    };
                } else {
                    /*fix me?*/
                    s = if decompose {
                        format!("κει {} νται", SEPARATOR)
                    } else {
                        String::from("κεῖνται")
                    };
                }
            }
        } else if vf.mood == HcMood::Subjunctive {
            if vf.person == Some(HcPerson::First) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("κε {} ωμαι", SEPARATOR)
                    } else {
                        String::from("κέωμαι")
                    };
                } else {
                    s = if decompose {
                        format!("κε {} ωμεθα", SEPARATOR)
                    } else {
                        String::from("κεώμεθα")
                    };
                }
            } else if vf.person == Some(HcPerson::Second) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("κε {} ῃ", SEPARATOR)
                    } else {
                        String::from("κέῃ")
                    };
                } else {
                    s = if decompose {
                        format!("κε {} ησθε", SEPARATOR)
                    } else {
                        String::from("κέησθε")
                    };
                }
            } else if vf.person == Some(HcPerson::Third) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("κε {} ηται", SEPARATOR)
                    } else {
                        String::from("κέηται")
                    };
                } else {
                    s = if decompose {
                        format!("κε {} ωνται", SEPARATOR)
                    } else {
                        String::from("κέωνται")
                    };
                }
            }
        } else if vf.mood == HcMood::Optative {
            if vf.person == Some(HcPerson::First) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("κε {} οιμην", SEPARATOR)
                    } else {
                        String::from("κεοίμην")
                    };
                } else {
                    s = if decompose {
                        format!("κε {} οιμεθα", SEPARATOR)
                    } else {
                        String::from("κεοίμεθα")
                    };
                }
            } else if vf.person == Some(HcPerson::Second) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("κε {} οιο", SEPARATOR)
                    } else {
                        String::from("κέοιο")
                    };
                } else {
                    s = if decompose {
                        format!("κε {} οισθε", SEPARATOR)
                    } else {
                        String::from("κέοισθε")
                    };
                }
            } else if vf.person == Some(HcPerson::Third) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("κε {} οιτο", SEPARATOR)
                    } else {
                        String::from("κέοιτο")
                    };
                } else {
                    s = if decompose {
                        format!("κε {} οιντο", SEPARATOR)
                    } else {
                        String::from("κέοιντο")
                    };
                }
            }
        } else if vf.mood == HcMood::Imperative {
            if vf.person == Some(HcPerson::First) {
                if vf.number == Some(HcNumber::Singular) {
                    s = String::from("");
                } else {
                    s = String::from("");
                }
            } else if vf.person == Some(HcPerson::Second) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("κει {} σο", SEPARATOR)
                    } else {
                        String::from("κεῖσο")
                    };
                } else {
                    s = if decompose {
                        format!("κει {} σθε", SEPARATOR)
                    } else {
                        String::from("κεῖσθε")
                    };
                }
            } else if vf.person == Some(HcPerson::Third) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("κει {} σθω", SEPARATOR)
                    } else {
                        String::from("κείσθω")
                    };
                } else {
                    s = if decompose {
                        format!("κει {} σθων", SEPARATOR)
                    } else {
                        String::from("κείσθων")
                    };
                }
            }
        }
    } else if vf.tense == HcTense::Imperfect {
        if vf.person == Some(HcPerson::First) {
            if vf.number == Some(HcNumber::Singular) {
                s = if decompose {
                    format!("ε {} κει {} μην", SEPARATOR, SEPARATOR)
                } else {
                    String::from("ἐκείμην")
                };
            } else {
                s = if decompose {
                    format!("ε {} κει {} μεθα", SEPARATOR, SEPARATOR)
                } else {
                    String::from("ἐκείμεθα")
                };
            }
        } else if vf.person == Some(HcPerson::Second) {
            if vf.number == Some(HcNumber::Singular) {
                s = if decompose {
                    format!("ε {} κει {} σο", SEPARATOR, SEPARATOR)
                } else {
                    String::from("ἔκεισο")
                };
            } else {
                s = if decompose {
                    format!("ε {} κει {} σθε", SEPARATOR, SEPARATOR)
                } else {
                    String::from("ἔκεισθε")
                };
            }
        } else if vf.person == Some(HcPerson::Third) {
            if vf.number == Some(HcNumber::Singular) {
                s = if decompose {
                    format!("ε {} κει {} το", SEPARATOR, SEPARATOR)
                } else {
                    String::from("ἔκειτο")
                };
            } else {
                s = if decompose {
                    format!("ε {} κει {} ντο", SEPARATOR, SEPARATOR)
                } else {
                    String::from("ἔκειντο")
                };
            }
        }
    }
    s
}

pub fn get_fhmi(vf: &HcGreekVerbForm, decompose: bool) -> String {
    if vf.voice != HcVoice::Active {
        return String::from("");
    }
    let mut s = String::from("");
    if vf.tense == HcTense::Present {
        if vf.mood == HcMood::Indicative {
            if vf.person == Some(HcPerson::First) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("φη {} μι", SEPARATOR)
                    } else {
                        String::from("φημί")
                    };
                } else {
                    s = if decompose {
                        format!("φα {} μεν", SEPARATOR)
                    } else {
                        String::from("φαμέν")
                    };
                }
            } else if vf.person == Some(HcPerson::Second) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("φῃ {} ς", SEPARATOR)
                    } else {
                        String::from("φῄς")
                    };
                } else {
                    s = if decompose {
                        format!("φα {} τε", SEPARATOR)
                    } else {
                        String::from("φατέ")
                    };
                }
            } else if vf.person == Some(HcPerson::Third) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("φη {} σι(ν)", SEPARATOR)
                    } else {
                        String::from("φησί(ν)")
                    };
                } else {
                    /*fix me?*/
                    s = if decompose {
                        format!("φα {} ᾱσι(ν)", SEPARATOR)
                    } else {
                        String::from("φᾱσί(ν)")
                    };
                }
            }
        } else if vf.mood == HcMood::Subjunctive {
            if vf.person == Some(HcPerson::First) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("φε {} ω", SEPARATOR)
                    } else {
                        String::from("φῶ")
                    };
                } else {
                    s = if decompose {
                        format!("φε {} ωμεν", SEPARATOR)
                    } else {
                        String::from("φῶμεν")
                    };
                }
            } else if vf.person == Some(HcPerson::Second) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("φε {} ῃς", SEPARATOR)
                    } else {
                        String::from("φῇς")
                    };
                } else {
                    s = if decompose {
                        format!("φε {} ητε", SEPARATOR)
                    } else {
                        String::from("φῆτε")
                    };
                }
            } else if vf.person == Some(HcPerson::Third) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("φε {} ῃ", SEPARATOR)
                    } else {
                        String::from("φῇ")
                    };
                } else {
                    s = if decompose {
                        format!("φε {} ωσι(ν)", SEPARATOR)
                    } else {
                        String::from("φῶσι(ν)")
                    };
                }
            }
        } else if vf.mood == HcMood::Optative {
            if vf.person == Some(HcPerson::First) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("φα {} ιην", SEPARATOR)
                    } else {
                        String::from("φαίην")
                    };
                } else {
                    s = if decompose {
                        format!("φα {} ιμεν, φα {} ιημεν", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("φαῖμεν, φαίημεν")
                    };
                }
            } else if vf.person == Some(HcPerson::Second) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("φα {} ιης", SEPARATOR)
                    } else {
                        String::from("φαίης")
                    };
                } else {
                    s = if decompose {
                        format!("φα {} ιτε, φα {} ιητε", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("φαῖτε, φαίητε")
                    };
                }
            } else if vf.person == Some(HcPerson::Third) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("φα {} ιη", SEPARATOR)
                    } else {
                        String::from("φαίη")
                    };
                } else {
                    s = if decompose {
                        format!("φα {} ιεν, φα {} ιησαν", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("φαῖεν, φαίησαν")
                    };
                }
            }
        } else if vf.mood == HcMood::Imperative {
            if vf.person == Some(HcPerson::First) {
                if vf.number == Some(HcNumber::Singular) {
                    s = String::from("");
                } else {
                    s = String::from("");
                }
            } else if vf.person == Some(HcPerson::Second) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("φα {} θι", SEPARATOR)
                    } else {
                        String::from("φάθι")
                    };
                } else {
                    s = if decompose {
                        format!("φα {} τε", SEPARATOR)
                    } else {
                        String::from("φάτε")
                    };
                }
            } else if vf.person == Some(HcPerson::Third) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("φα {} τω", SEPARATOR)
                    } else {
                        String::from("φάτω")
                    };
                } else {
                    s = if decompose {
                        format!("φα {} ντων", SEPARATOR)
                    } else {
                        String::from("φάντων")
                    };
                }
            }
        }
    } else if vf.tense == HcTense::Imperfect {
        if vf.person == Some(HcPerson::First) {
            if vf.number == Some(HcNumber::Singular) {
                s = if decompose {
                    format!("ε {} φη {} ν", SEPARATOR, SEPARATOR)
                } else {
                    String::from("ἔφην")
                };
            } else {
                s = if decompose {
                    format!("ε {} φα {} μεν", SEPARATOR, SEPARATOR)
                } else {
                    String::from("ἔφαμεν")
                };
            }
        } else if vf.person == Some(HcPerson::Second) {
            if vf.number == Some(HcNumber::Singular) {
                s = if decompose {
                    format!(
                        "ε {} φη {} σθα, ε {} φη {} ς",
                        SEPARATOR, SEPARATOR, SEPARATOR, SEPARATOR
                    )
                } else {
                    String::from("ἔφησθα, ἔφης")
                };
            } else {
                s = if decompose {
                    format!("ε {} φα {} τε", SEPARATOR, SEPARATOR)
                } else {
                    String::from("ἔφατε")
                };
            }
        } else if vf.person == Some(HcPerson::Third) {
            if vf.number == Some(HcNumber::Singular) {
                s = if decompose {
                    format!("ε {} φη {} ", SEPARATOR, SEPARATOR)
                } else {
                    String::from("ἔφη")
                };
            } else {
                s = if decompose {
                    format!("ε {} φα {} σαν", SEPARATOR, SEPARATOR)
                } else {
                    String::from("ἔφασαν")
                };
            }
        }
    }
    s
}

pub fn get_eimi_ibo(vf: &HcGreekVerbForm, decompose: bool) -> String {
    if vf.voice != HcVoice::Active {
        return String::from("");
    }
    let mut s = String::from("");
    if vf.tense == HcTense::Present {
        if vf.mood == HcMood::Indicative {
            if vf.person == Some(HcPerson::First) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("εἰ {} μι", SEPARATOR)
                    } else {
                        String::from("εἶμι")
                    };
                } else {
                    s = if decompose {
                        format!("ἰ {} μεν", SEPARATOR)
                    } else {
                        String::from("ἴμεν")
                    };
                }
            } else if vf.person == Some(HcPerson::Second) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("εἰ {} ", SEPARATOR)
                    } else {
                        String::from("εἶ")
                    };
                } else {
                    s = if decompose {
                        format!("ἰ {} τε", SEPARATOR)
                    } else {
                        String::from("ἴτε")
                    };
                }
            } else if vf.person == Some(HcPerson::Third) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("εἰ {} σι(ν)", SEPARATOR)
                    } else {
                        String::from("εἶσι(ν)")
                    };
                } else {
                    /*fix me?*/
                    s = if decompose {
                        format!("ἰ {} ᾱσι(ν)", SEPARATOR)
                    } else {
                        String::from("ἴᾱσι(ν)")
                    };
                }
            }
        } else if vf.mood == HcMood::Subjunctive {
            if vf.person == Some(HcPerson::First) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("ἰ {} ω", SEPARATOR)
                    } else {
                        String::from("ἴω")
                    };
                } else {
                    s = if decompose {
                        format!("ἰ {} ωμεν", SEPARATOR)
                    } else {
                        String::from("ἴωμεν")
                    };
                }
            } else if vf.person == Some(HcPerson::Second) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("ἰ {} ῃς", SEPARATOR)
                    } else {
                        String::from("ἴῃς")
                    };
                } else {
                    s = if decompose {
                        format!("ἰ {} ητε", SEPARATOR)
                    } else {
                        String::from("ἴητε")
                    };
                }
            } else if vf.person == Some(HcPerson::Third) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("ἰ {} ῃ", SEPARATOR)
                    } else {
                        String::from("ἴῃ")
                    };
                } else {
                    s = if decompose {
                        format!("ἰ {} ωσι(ν)", SEPARATOR)
                    } else {
                        String::from("ἴωσι(ν)")
                    };
                }
            }
        } else if vf.mood == HcMood::Optative {
            if vf.person == Some(HcPerson::First) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("ἰ {} οιμι, ἰ {} οιην", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("ἴοιμι, ἰοίην")
                    };
                } else {
                    s = if decompose {
                        format!("ἰ {} οιμεν", SEPARATOR)
                    } else {
                        String::from("ἴοιμεν")
                    };
                }
            } else if vf.person == Some(HcPerson::Second) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("ἰ {} οις", SEPARATOR)
                    } else {
                        String::from("ἴοις")
                    };
                } else {
                    s = if decompose {
                        format!("ἰ {} οιτε", SEPARATOR)
                    } else {
                        String::from("ἴοιτε")
                    };
                }
            } else if vf.person == Some(HcPerson::Third) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("ἰ {} οι", SEPARATOR)
                    } else {
                        String::from("ἴοι")
                    };
                } else {
                    s = if decompose {
                        format!("ἰ {} οιεν", SEPARATOR)
                    } else {
                        String::from("ἴοιεν")
                    };
                }
            }
        } else if vf.mood == HcMood::Imperative {
            if vf.person == Some(HcPerson::First) {
                if vf.number == Some(HcNumber::Singular) {
                    s = String::from("");
                } else {
                    s = String::from("");
                }
            } else if vf.person == Some(HcPerson::Second) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("ἰ {} θι", SEPARATOR)
                    } else {
                        String::from("ἴθι")
                    };
                } else {
                    s = if decompose {
                        format!("ἰ {} τε", SEPARATOR)
                    } else {
                        String::from("ἴτε")
                    };
                }
            } else if vf.person == Some(HcPerson::Third) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("ἰ {} τω", SEPARATOR)
                    } else {
                        String::from("ἴτω")
                    };
                } else {
                    s = if decompose {
                        format!("ἰ {} οντων", SEPARATOR)
                    } else {
                        String::from("ἰόντων")
                    };
                }
            }
        }
    } else if vf.tense == HcTense::Imperfect {
        if vf.person == Some(HcPerson::First) {
            if vf.number == Some(HcNumber::Singular) {
                s = if decompose {
                    format!("ᾐ {} α, ᾐ {} ειν", SEPARATOR, SEPARATOR)
                } else {
                    String::from("ᾖα, ᾔειν")
                };
            } else {
                s = if decompose {
                    format!("ᾐ {} μεν", SEPARATOR)
                } else {
                    String::from("ᾖμεν")
                };
            }
        } else if vf.person == Some(HcPerson::Second) {
            if vf.number == Some(HcNumber::Singular) {
                s = if decompose {
                    format!("ᾐ {} εισθα, ᾐ {} εις", SEPARATOR, SEPARATOR)
                } else {
                    String::from("ᾔεισθα, ᾔεις")
                };
            } else {
                s = if decompose {
                    format!("ᾐ {} τε", SEPARATOR)
                } else {
                    String::from("ᾖτε")
                };
            }
        } else if vf.person == Some(HcPerson::Third) {
            if vf.number == Some(HcNumber::Singular) {
                s = if decompose {
                    format!("ᾐ {} ει(ν)", SEPARATOR)
                } else {
                    String::from("ᾔει(ν)")
                };
            } else {
                s = if decompose {
                    format!("ᾐ {} σαν, ᾐ {} εσαν", SEPARATOR, SEPARATOR)
                } else {
                    String::from("ᾖσαν, ᾔεσαν")
                };
            }
        }
    }
    s
}

pub fn get_oida(vf: &HcGreekVerbForm, decompose: bool) -> String {
    if vf.voice != HcVoice::Active {
        return String::from("");
    }
    let mut s = String::from("");
    if vf.tense == HcTense::Perfect {
        if vf.mood == HcMood::Indicative {
            if vf.person == Some(HcPerson::First) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("οἰδ {} α", SEPARATOR)
                    } else {
                        String::from("οἶδα")
                    };
                } else {
                    s = if decompose {
                        format!("ἰσ {} μεν", SEPARATOR)
                    } else {
                        String::from("ἴσμεν")
                    };
                }
            } else if vf.person == Some(HcPerson::Second) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("οἰσ {} θα", SEPARATOR)
                    } else {
                        String::from("οἶσθα")
                    };
                } else {
                    s = if decompose {
                        format!("ἰσ {} τε", SEPARATOR)
                    } else {
                        String::from("ἴστε")
                    };
                }
            } else if vf.person == Some(HcPerson::Third) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("οἰδ {} ε(ν)", SEPARATOR)
                    } else {
                        String::from("οἶδε(ν)")
                    };
                } else {
                    /*fix me?*/
                    s = if decompose {
                        format!("ἰσ {} ᾱσι(ν)", SEPARATOR)
                    } else {
                        String::from("ἴσᾱσι(ν)")
                    };
                }
            }
        } else if vf.mood == HcMood::Subjunctive {
            if vf.person == Some(HcPerson::First) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("εἰδε {} ω", SEPARATOR)
                    } else {
                        String::from("εἰδῶ")
                    };
                } else {
                    s = if decompose {
                        format!("εἰδε {} ωμεν", SEPARATOR)
                    } else {
                        String::from("εἰδῶμεν")
                    };
                }
            } else if vf.person == Some(HcPerson::Second) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("εἰδε {} ῃς", SEPARATOR)
                    } else {
                        String::from("εἰδῇς")
                    };
                } else {
                    s = if decompose {
                        format!("εἰδε {} ητε", SEPARATOR)
                    } else {
                        String::from("εἰδῆτε")
                    };
                }
            } else if vf.person == Some(HcPerson::Third) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("εἰδε {} ῃ", SEPARATOR)
                    } else {
                        String::from("εἰδῇ")
                    };
                } else {
                    s = if decompose {
                        format!("εἰδε {} ωσι(ν)", SEPARATOR)
                    } else {
                        String::from("εἰδῶσι(ν)")
                    };
                }
            }
        } else if vf.mood == HcMood::Optative {
            if vf.person == Some(HcPerson::First) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("εἰδ {} ειην", SEPARATOR)
                    } else {
                        String::from("εἰδείην")
                    };
                } else {
                    s = if decompose {
                        format!("εἰδ {} ειμεν, εἰδ {} ειημεν", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("εἰδεῖμεν, εἰδείημεν")
                    };
                }
            } else if vf.person == Some(HcPerson::Second) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("εἰδ {} ειης", SEPARATOR)
                    } else {
                        String::from("εἰδείης")
                    };
                } else {
                    s = if decompose {
                        format!("εἰδ {} ειτε, εἰδ {} ειητε", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("εἰδεῖτε, εἰδείητε")
                    };
                }
            } else if vf.person == Some(HcPerson::Third) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("εἰδ {} ειη", SEPARATOR)
                    } else {
                        String::from("εἰδείη")
                    };
                } else {
                    s = if decompose {
                        format!("εἰδ {} ειεν, εἰδ {} ειησαν", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("εἰδεῖεν, εἰδείησαν")
                    };
                }
            }
        } else if vf.mood == HcMood::Imperative {
            if vf.person == Some(HcPerson::First) {
                if vf.number == Some(HcNumber::Singular) {
                    s = String::from("");
                } else {
                    s = String::from("");
                }
            } else if vf.person == Some(HcPerson::Second) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("ἰσ {} θι", SEPARATOR)
                    } else {
                        String::from("ἴσθι")
                    };
                } else {
                    s = if decompose {
                        format!("ἰσ {} τε", SEPARATOR)
                    } else {
                        String::from("ἴστε")
                    };
                }
            } else if vf.person == Some(HcPerson::Third) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("ἰσ {} τω", SEPARATOR)
                    } else {
                        String::from("ἴστω")
                    };
                } else {
                    s = if decompose {
                        format!("ἰσ {} των", SEPARATOR)
                    } else {
                        String::from("ἴστων")
                    };
                }
            }
        }
    } else if vf.tense == HcTense::Pluperfect {
        if vf.person == Some(HcPerson::First) {
            if vf.number == Some(HcNumber::Singular) {
                s = if decompose {
                    format!(
                        "ε {} εἰδ {} η, ε {} εἰδ {} ειν",
                        SEPARATOR, SEPARATOR, SEPARATOR, SEPARATOR
                    )
                } else {
                    String::from("ᾔδη, ᾔδειν")
                };
            } else {
                s = if decompose {
                    format!(
                        "ε {} εἰσ {} μεν, ε {} εἰδ {} εμεν",
                        SEPARATOR, SEPARATOR, SEPARATOR, SEPARATOR
                    )
                } else {
                    String::from("ᾖσμεν, ᾔδεμεν")
                };
            }
        } else if vf.person == Some(HcPerson::Second) {
            if vf.number == Some(HcNumber::Singular) {
                s = if decompose {
                    format!(
                        "ε {} εἰδ {} ησθα, ε {} εἰδ {} εις",
                        SEPARATOR, SEPARATOR, SEPARATOR, SEPARATOR
                    )
                } else {
                    String::from("ᾔδησθα, ᾔδεις")
                };
            } else {
                s = if decompose {
                    format!(
                        "ε {} εἰσ {} τε, ε {} εἰδ {} ετε",
                        SEPARATOR, SEPARATOR, SEPARATOR, SEPARATOR
                    )
                } else {
                    String::from("ᾖστε, ᾔδετε")
                };
            }
        } else if vf.person == Some(HcPerson::Third) {
            if vf.number == Some(HcNumber::Singular) {
                s = if decompose {
                    format!("ε {} εἰδ {} ει(ν)", SEPARATOR, SEPARATOR)
                } else {
                    String::from("ᾔδει(ν)")
                };
            } else {
                s = if decompose {
                    format!(
                        "ε {} εἰσ {} αν, ε {} εἰδ {} εσαν",
                        SEPARATOR, SEPARATOR, SEPARATOR, SEPARATOR
                    )
                } else {
                    String::from("ᾖσαν, ᾔδεσαν")
                };
            }
        }
    }
    s
}

pub fn get_sunoida(vf: &HcGreekVerbForm, decompose: bool) -> String {
    if vf.voice != HcVoice::Active {
        return String::from("");
    }
    let mut s = String::from("");
    if vf.tense == HcTense::Perfect {
        if vf.mood == HcMood::Indicative {
            if vf.person == Some(HcPerson::First) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("συν {} οἰδ {} α", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("σύνοιδα")
                    };
                } else {
                    s = if decompose {
                        format!("συν {} ἰσ {} μεν", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("σύνισμεν")
                    };
                }
            } else if vf.person == Some(HcPerson::Second) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("συν {} οἰσ {} θα", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("σύνοισθα")
                    };
                } else {
                    s = if decompose {
                        format!("συν {} ἰσ {} τε", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("σύνιστε")
                    };
                }
            } else if vf.person == Some(HcPerson::Third) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("συν {} οἰδ {} ε(ν)", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("σύνοιδε(ν)")
                    };
                } else {
                    /*fix me?*/
                    s = if decompose {
                        format!("συν {} ἰσ {} ᾱσι(ν)", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("συνίσᾱσι(ν)")
                    };
                }
            }
        } else if vf.mood == HcMood::Subjunctive {
            if vf.person == Some(HcPerson::First) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("συν {} εἰδε {} ω", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("συνειδῶ")
                    };
                } else {
                    s = if decompose {
                        format!("συν {} εἰδε {} ωμεν", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("συνειδῶμεν")
                    };
                }
            } else if vf.person == Some(HcPerson::Second) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("συν {} εἰδε {} ῃς", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("συνειδῇς")
                    };
                } else {
                    s = if decompose {
                        format!("συν {} εἰδε {} ητε", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("συνειδῆτε")
                    };
                }
            } else if vf.person == Some(HcPerson::Third) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("συν {} εἰδε {} ῃ", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("συνειδῇ")
                    };
                } else {
                    s = if decompose {
                        format!("συν {} εἰδε {} ωσι(ν)", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("συνειδῶσι(ν)")
                    };
                }
            }
        } else if vf.mood == HcMood::Optative {
            if vf.person == Some(HcPerson::First) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("συν {} εἰδ {} ειην", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("συνειδείην")
                    };
                } else {
                    s = if decompose {
                        format!(
                            "συν {} εἰδ {} ειμεν, συν {} εἰδ {} ειημεν",
                            SEPARATOR, SEPARATOR, SEPARATOR, SEPARATOR
                        )
                    } else {
                        String::from("συνειδεῖμεν, συνειδείημεν")
                    };
                }
            } else if vf.person == Some(HcPerson::Second) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("συν {} εἰδ {} ειης", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("συνειδείης")
                    };
                } else {
                    s = if decompose {
                        format!(
                            "συν {} εἰδ {} ειτε, συν {} εἰδ {} ειητε",
                            SEPARATOR, SEPARATOR, SEPARATOR, SEPARATOR
                        )
                    } else {
                        String::from("συνειδεῖτε, συνειδείητε")
                    };
                }
            } else if vf.person == Some(HcPerson::Third) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("συν {} εἰδ {} ειη", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("συνειδείη")
                    };
                } else {
                    s = if decompose {
                        format!(
                            "συν {} εἰδ {} ειεν, συν {} εἰδ {} ειησαν",
                            SEPARATOR, SEPARATOR, SEPARATOR, SEPARATOR
                        )
                    } else {
                        String::from("συνειδεῖεν, συνειδείησαν")
                    };
                }
            }
        } else if vf.mood == HcMood::Imperative {
            if vf.person == Some(HcPerson::First) {
                if vf.number == Some(HcNumber::Singular) {
                    s = String::from("");
                } else {
                    s = String::from("");
                }
            } else if vf.person == Some(HcPerson::Second) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("συν {} ἰσ {} θι", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("σύνισθι")
                    };
                } else {
                    s = if decompose {
                        format!("συν {} ἰσ {} τε", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("σύνιστε")
                    };
                }
            } else if vf.person == Some(HcPerson::Third) {
                if vf.number == Some(HcNumber::Singular) {
                    s = if decompose {
                        format!("συν {} ἰσ {} τω", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("συνίστω")
                    };
                } else {
                    s = if decompose {
                        format!("συν {} ἰσ {} των", SEPARATOR, SEPARATOR)
                    } else {
                        String::from("συνίστων")
                    };
                }
            }
        }
    } else if vf.tense == HcTense::Pluperfect {
        if vf.person == Some(HcPerson::First) {
            if vf.number == Some(HcNumber::Singular) {
                s = if decompose {
                    format!(
                        "συν {} ε {} εἰδ {} η, συν {} ε {} εἰδ {} ειν",
                        SEPARATOR, SEPARATOR, SEPARATOR, SEPARATOR, SEPARATOR, SEPARATOR
                    )
                } else {
                    String::from("συνῄδη, συνῄδειν")
                };
            } else {
                s = if decompose {
                    format!(
                        "συν {} ε {} εἰσ {} μεν, συν {} ε {} εἰδ {} εμεν",
                        SEPARATOR, SEPARATOR, SEPARATOR, SEPARATOR, SEPARATOR, SEPARATOR
                    )
                } else {
                    String::from("συνῇσμεν, συνῄδεμεν")
                };
            }
        } else if vf.person == Some(HcPerson::Second) {
            if vf.number == Some(HcNumber::Singular) {
                s = if decompose {
                    format!(
                        "συν {} ε {} εἰδ {} ησθα, συν {} ε {} εἰδ {} εις",
                        SEPARATOR, SEPARATOR, SEPARATOR, SEPARATOR, SEPARATOR, SEPARATOR
                    )
                } else {
                    String::from("συνῄδησθα, συνῄδεις")
                };
            } else {
                s = if decompose {
                    format!(
                        "συν {} ε {} εἰσ {} τε, συν {} ε {} εἰδ {} ετε",
                        SEPARATOR, SEPARATOR, SEPARATOR, SEPARATOR, SEPARATOR, SEPARATOR
                    )
                } else {
                    String::from("συνῇστε, συνῄδετε")
                };
            }
        } else if vf.person == Some(HcPerson::Third) {
            if vf.number == Some(HcNumber::Singular) {
                s = if decompose {
                    format!("συν {} ε {} εἰδ {} ει(ν)", SEPARATOR, SEPARATOR, SEPARATOR)
                } else {
                    String::from("συνῄδει(ν)")
                };
            } else {
                s = if decompose {
                    format!(
                        "συν {} ε {} εἰσ {} αν, συν {} ε {} εἰδ {} εσαν",
                        SEPARATOR, SEPARATOR, SEPARATOR, SEPARATOR, SEPARATOR, SEPARATOR
                    )
                } else {
                    String::from("συνῇσαν, συνῄδεσαν")
                };
            }
        }
    }
    s
}
