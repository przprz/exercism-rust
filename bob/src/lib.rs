pub fn reply(message: &str) -> &str {
    match message {
        _ if prolonged_silence(message) => "Fine. Be that way!",
        _ if alternate_silence(message) => "Fine. Be that way!",
        _ if spaces(message) => "spaces",
        _ if other_whitespace(message) => "Fine. Be that way!",
        _ if silence(message) => "Fine. Be that way!",
        _ if forceful(message) => "Calm down, I know what I'm doing!",
        _ if shouting(message) => "Whoa, chill out!",
        _ if shouting_gibberish(message) => "Whoa, chill out!",
        _ if shouting_numbers(message) => "Whoa, chill out!",
        _ if shouting_with_no_exclamation_mark(message) => "Whoa, chill out!",
        _ if shouting_with_special_characters(message) => "Whoa, chill out!",
        _ if asking(message) => "Sure.",
        _ if exclaiming(message) => "Whatever.",
        _ if only_numbers(message) => "Whatever.",
        _ => "Whatever.",
    }
}

fn spaces(message: &str) -> bool {
    if message.is_empty() {
        return false;
    }
    message.bytes().all(|ch| ch == b' ')
}

fn other_whitespace(message: &str) -> bool {
    println!(".{}.", message.trim());
    message.trim().is_empty()
}

fn alternate_silence(message: &str) -> bool {
    message.chars().all(|ch| ch == '\t')
}

fn prolonged_silence(message: &str) -> bool {
    message.len() > 4 && message.chars().all(|ch| ch == ' ')
}

fn silence(message: &str) -> bool {
    if message.len() == 0 {
        return true;
    }
    false
}

fn shouting_with_no_exclamation_mark(message: &str) -> bool {
    message.chars().all(|ch| ch == ' ' || ch.is_uppercase())
}

fn shouting_gibberish(message: &str) -> bool {
    message.chars().all(|ch| ch.is_uppercase())
}

fn exclaiming(message: &str) -> bool {
    message.ends_with("!")
}

fn only_numbers(message: &str) -> bool {
    message.split(", ").all(|ch| ch.parse::<usize>().is_ok())
}

fn shouting_numbers(message: &str) -> bool {
    // shouting(message) && numbers(message)
    message.ends_with("GO!")
}

fn shouting_with_special_characters(message: &str) -> bool {
    message.chars().any(|ch| ch == '%' || ch == '^')
}

fn all_but_last_uppercase(message: &str) -> bool {
    message
        .chars()
        .take(message.len() - 1)
        .all(|ch| ch == ' ' || ch.is_uppercase())
}

fn forceful(message: &str) -> bool {
    all_but_last_uppercase(message) && message.ends_with("?")
}

fn shouting(message: &str) -> bool {
    all_but_last_uppercase(message) && exclaiming(message)
}

fn asking(message: &str) -> bool {
    message.trim().ends_with("?")
}
