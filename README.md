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

## What is ISO 15924

ISO 15924, Codes for the representation of names of scripts, is an international standard defining codes for writing systems or scripts (a "set of graphic characters used for the written form of one or more languages"). Each script is given both a four-letter code and a numeric code.[1]

Where possible the codes are derived from ISO 639-2, where the name of a script and the name of a language using the script are identical (example: Gujarātī ISO 639 guj, ISO 15924 Gujr). Preference is given to the 639-2 Bibliographical codes, which is different from the otherwise often preferred use of the Terminological codes.[1]

4-letter ISO 15924 codes are incorporated into the IANA Language Subtag Registry for IETF language tags and so can be used in file formats that make use of such language tags. For example, they can be used in HTML and XML to help Web browsers determine which typeface to use for foreign text. This way one could differentiate, for example, between Serbian written in the Cyrillic (sr-Cyrl) or Latin (sr-Latn) script, or mark romanized or transliterated text as such.

> _-- [Wikipedia](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes)_

## Installing

```sh
[dependencies]
rust_iso639 = "0.1.0"
```

## License

rust-iso/rust_iso39 is licensed under the Apache-2.0 license.

## Using

See [using](https://crates.io/crates/rust_iso3166) section of the documentation.

Quick guide:

```rust
let lang = rust_iso639::from_alpha2("AU");
let country = rust_iso639::from_alpha3("AUS");
let country = rust_iso639::from_numeric(036);
let country = rust_iso639::from_numeric_str("036");

println!("{:?}", rust_iso639::ALL);

println!("{:?}", rust_iso639::ALL_ALPHA2);
println!("{:?}", rust_iso639::ALL_ALPHA3);
println!("{:?}", rust_iso639::ALL_NAME);
println!("{:?}", rust_iso639::ALL_NUMERIC);
println!("{:?}", rust_iso639::ALL_NUMERIC_STR);

println!("{:?}", rust_iso639::NUMERIC_MAP);
println!("{:?}", rust_iso639::ALPHA3_MAP);
println!("{:?}", rust_iso639::ALPHA2_MAP);

// for ISO 3166-2
let country = rust_iso3166::from_alpha2("GB").unwrap();
let subdivisions = country.subdivisions();
assert!(subdivisions.unwrap().len() > 0);
let country = rust_iso3166::iso3166_2::from_code("GB-EDH");
assert_eq!("Edinburgh, City of", country.unwrap().name);

// for ISO 3166-3
let sub = rust_iso3166::iso3166_3::from_code("PZPA");
assert_eq!("Panama Canal Zone", sub.unwrap().name);
```

Data sample:

```rust
CountryCode {
    name: "Australia",
    alpha2: "AU",
    alpha3: "AUS",
    numeric: 36
}

 iso3166_2::Subdivision {
    name: "Bādghīs",
    code: "AF-BDG",
    subdivision_type: "Province",
    country_name: "Afghanistan",
    country_code: "AF",
    region_code: "AF-BDG",
}

iso3166_3::CountryCode3 {
    code: "BQAQ",
    name: "British Antarctic Territory",
    former: CountryCode {
        name: "British Antarctic Territory",
        alpha2: "BQ",
        alpha3: "ATB",
        numeric: 0
    },
    new_countries: [
        CountryCode {
            name: "Antarctica",
            alpha2: "AQ",
            alpha3: "ATA",
            numeric: 010
        },
    ],
    validity: [1974,1979],
    desc: "Merged into Antarctica",
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
