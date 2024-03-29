# rust_iso/iso639

A rust crate providing ISO 639 1, ISO 639 2, ISO 639 3 , ISO 15924 support.

## What is ISO 639

[ISO 639](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) is a standardized nomenclature used to classify languages. Each language is assigned a two-letter (639-1) and three-letter (639-2 and 639-3) lowercase abbreviation, amended in later versions of the nomenclature.

This table lists all of:

[ISO 639-1](https://en.wikipedia.org/wiki/ISO_639-1): two-letter codes, one per language for [ISO 639 macrolanguage](https://en.wikipedia.org/wiki/ISO_639_macrolanguage)
And some of:

[ISO 639-2/T](https://en.wikipedia.org/wiki/ISO_639-2): three-letter codes, for the same languages as 639-1

[ISO 639-2/B](https://en.wikipedia.org/wiki/ISO_639-2): three-letter codes, mostly the same as 639-2/T, but with some codes derived from English names rather than native names of languages (in the following table, these differing codes are highlighted in boldface)

[ISO 639-3](https://en.wikipedia.org/wiki/ISO_639-3): three-letter codes, the same as 639-2/T for languages, but with distinct codes for each variety of an ISO 639 macrolanguage

> _-- [Wikipedia](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes)_

## Installing

```sh
[dependencies]
rust_iso639 = "0.0.1"
```

## License

rust-iso/rust_iso639 is licensed under the Apache-2.0 license.

## Using

See [using](https://crates.io/crates/rust_iso639) section of the documentation.

Quick guide:

```rust
let lang = rust_iso639::from_code("zh");
let lang = rust_iso639::from_code_2t("zho");
let lang = rust_iso639::from_code_2b("chi");
let lang = rust_iso639::from_code_3("zho");

println!("{:?}", rust_iso639::ALL_CODE);
println!("{:?}", rust_iso639::ALL_CODE_2T);
println!("{:?}", rust_iso639::ALL_CODE_2B);
println!("{:?}", rust_iso639::ALL_CODE_3);

println!("{:?}", rust_iso639::CODE_MAP);
println!("{:?}", rust_iso639::CODE_2T_MAP);
println!("{:?}", rust_iso639::CODE_2B_MAP);
println!("{:?}", rust_iso639::CODE_3_MAP);

```

Data sample:

```rust
pub struct LanguageCode<'a> {
    ///ISO Language Name
    pub name: &'static str,
    ///639-1
    pub code: &'static str,
    ///639-2/T
    pub code_2t: &'static str,
    ///639-2/B
    pub code_2b: &'static str,
    //639-3 Macrolanguage
    pub code_3: &'static str,

    pub individual_languages: &'a [IndividualLanguages],
}

#[derive(Debug, Copy, Clone)]
pub struct IndividualLanguages {
    ///Name
    pub name: &'static str,
    ///Code
    pub code: &'static str,
}
```

## Source(s)

- https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes
- https://en.wikipedia.org/wiki/ISO_639_macrolanguage
- https://www.rfc-editor.org/rfc/bcp/bcp47.txt
- http://www.iana.org/assignments/language-subtag-registry/language-subtag-registry
- https://www.alchemysoftware.com/livedocs/ezscript/Topics/Catalyst/Language.htm
- [Codes for the representation of names of scripts](https://en.wikipedia.org/wiki/ISO_15924)
- https://www.zhihu.com/question/21980689
- https://en.wikipedia.org/wiki/Language_code
- https://iso639-3.sil.org/code_tables/download_tables
- http://unicode.org/iso15924/
