use atomic_counter::RelaxedCounter;

#[derive(Default)]
pub struct Stats {
    pub gens: RelaxedCounter,
    pub total_gens: RelaxedCounter,
    pub errors: RelaxedCounter,
    pub retries: RelaxedCounter,
    pub stop: RelaxedCounter,
}

pub fn set_title(elapsed: String, gened: usize, rpm: f64, retries: usize, errors: usize) {
    let title = format!(
        "OperaGX Gen By Myrddin - Status: [{}] | Generated: {} ~ RPM: {:.0?} ~ Retries: {} ~ Errors: {}",
            elapsed,
            gened,
            rpm,
            retries,
            errors,
        );

    winconsole::console::set_title(&title).expect("Unable to set title");
}
