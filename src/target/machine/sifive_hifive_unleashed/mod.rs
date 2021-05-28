
// FIXME maybe (if different from unmatched)
global_asm!(include_str!("../sifive_hifive_unmatched/boot.S"));

compile_error!("SiFive HiFive Unleashed will be supported soon, but is not yet.");

// Missing UART0_ADDR
// Missing CONSOLE

pub fn pause() {
}

pub fn init() {
}

pub fn display_machine_information() {
    println!("Build: SiFive HiFive Unleashed");
}
