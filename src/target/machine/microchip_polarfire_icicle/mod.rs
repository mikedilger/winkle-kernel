
// FIXME
global_asm!(include_str!("../sifive_hifive_unmatched/boot.S"));

compile_error!("Microchip PolarFire SoC Icicle Kit will be supported soon, but is not yet.");

pub fn display_machine_information() {
    println!("Build: Microchip PolarFire SoC Icicle Kit");
}
