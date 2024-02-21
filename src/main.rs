use wrong_text_encoding::maptyping::Res;
use wrong_text_encoding::{
    get_sample, make_freq_map, make_replace_list, parse_bytes, print_bytes, replace_with_tags,
    ConversionMode, PrintMode,
};

fn main() -> Res<()> {
    let sample = get_sample("sample.txt")?;

    let (print_mode, conv_mode) = (PrintMode::Decimal, ConversionMode::Binary);
    let as_bytes = print_bytes(&parse_bytes(&sample, conv_mode)?, print_mode)?;
    println!("\n[sample: {print_mode:?}] -> {as_bytes}\n");

    let byte_list = as_bytes.split_whitespace().collect::<Vec<&str>>();
    let occurrences = make_freq_map(&byte_list)?;

    for (byte, occurred) in &occurrences {
        println!("{byte:>10} -> {occurred}");
    }

    let replace_map = make_replace_list('a', 'z');
    let tag_list = replace_with_tags(byte_list.as_slice(), replace_map)?;
    let letter_list = String::from_iter(&tag_list);

    println!("\nfull string: '{}'\n", letter_list);

    let occurrences = make_freq_map(&tag_list)?;

    for (ch, occurred) in &occurrences {
        println!("{ch:>10} -> {occurred}");
    }

    println!("\n[:: done ::]");

    Ok(())
}
