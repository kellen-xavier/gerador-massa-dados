use std::char::from_digit;
use fake::faker::name::en::Name;
use fake::Fake;
use rand::{thread_rng, Rng};

fn gerar_nome() -> String {
    Name().fake()
}

fn gerar_cpf() -> String {
    let mut cpf: Vec<u8> = (0..9).map(|_| thread_rng().gen_range(0..=9) as u8).collect();

    let sum1: u32 = cpf.iter()
        .enumerate()
        .map(|(i, &digit)| digit as u32 * (10 - i as u32))
        .sum();

    let dv1 = (sum1 * 10 % 11) % 10;
        cpf.push(dv1 as u8);

    let sum2: u32 = cpf.iter()
        .enumerate()
        .map(|(i, &digit)| digit as u32 * (10 - i as u32))
        .sum();
    let dv1 = (sum1 * 10 % 11) % 10;
    cpf.push(dv1 as u8);

    cpf.iter()
        .enumerate()
        .map(|(i, &digit)| {
            if i == 3 || i == 6 {
                format!(".{}", digit)
            } else if i == 9 {
                format!("-{}", digit)
            } else {
                digit.to_string()
            }
        })
        .collect::<String>()
}


fn main() {
    let nome = gerar_nome();
    println!("Nome gerado: {}", nome);

    let cpf = gerar_cpf();
    println!("CPF gerado: {}", cpf);
}
