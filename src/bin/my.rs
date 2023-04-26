use phie::emu::Emu;

pub fn main() {
    let mut emu: Emu = "
        ν0(𝜋) ↦ ⟦ 𝜑 ↦ ν3(𝜋) ⟧
        ν1(𝜋) ↦ ⟦ Δ ↦ 0x0007 ⟧
        ν2(𝜋) ↦ ⟦ λ ↦ int-add, ρ ↦ 𝜋.𝛼0, 𝛼0 ↦ 𝜋.𝛼1 ⟧
        ν3(𝜋) ↦ ⟦ 𝜑 ↦ ν2(ξ), 𝛼0 ↦ ν1(𝜋), 𝛼1 ↦ ν1(𝜋) ⟧
    ".parse().unwrap();
    let dtz = emu.dataize();
    print!("The result is: {}\n", dtz.0);
}