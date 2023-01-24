#!/usr/bin/env python3.9
import re
import sys
f = open(r"./iso639-1.txt", "r+")

f2 = open(r"./iso639-2.txt", "r+")

f3 = open(r"./iso-639-3.tab", "r+")

f31 = open(r"./iso-639-3-macrolanguages.tab", "r+")

f32 = open(r"./iso-639-3_Name_Index.tab", "r+")


codes = {}
code_2ts = {}
code_2bs = {}
code_3s = {}

all_codes = []
all_map = {}

name_3_map = {}

for x in f.readlines():
    x = x.strip()
    if x == "|-":
        continue
    ts = x.split("||")
    name = ts[0].split('|').pop()
    name = name.replace(']', '')
    name = name.replace('[', '').strip()
    code = ts[1].split('|').pop().replace('}}</code>', '').strip()
    code_2t = ts[2].replace('</code>', "").replace('<code>',
                                                   '').replace("'''", '').strip()
    code_2b = ts[3].replace('</code>', "").replace('<code>',
                                                   '').replace("'''", '').strip()
    code_3 = ts[4].split('</code>')[0].replace('</code>',
                                               "").replace('<code>', '').replace("'''", '').strip()

    c = {"code": code, "name": name, "code_2t": code_2t,
         "code_2b": code_2b, "code_3": code_3, "id": code}
         
    if id !="" and c["code"] not in all_map:
        all_codes.append(c)
        all_map[c["code"]] = c
        
    codes[c["code"]] = c
    code_2ts[c["code_2t"]] = c
    code_2bs[c["code_2b"]] = c
    code_3s[c["code_3"]] = c

        


for x in f2.readlines():
    x = x.strip()
    if x == "|-":
        continue
    if re.compile("\|\- id=\"\w\"").match(x):
        continue

    ts = x.split("||")

    code = ""
    code_2b = ""
    code_2t = ""
    code_3 = ""
    name = ""
    code_2 = ts[0].replace("| {{iso639-2|", "").replace("}}", "").split(" /")
    if len(code_2) == 1:
        code_2b = code_2t = code_2[0].strip()
    elif len(code_2) == 2:
        code_2b = code_2[1].replace("*", "").strip()
        code_2t = code_2[0].strip()

    code_3 = ts[1].strip()
    code = ts[3].strip()

    name = ts[4].replace("]]", "").replace("[[", "").strip()
    name = re.sub(re.compile("\w+ language\|"), "", name).strip()

    c = {"code": code, "name": name, "code_2t": code_2t,
         "code_2b": code_2b, "code_3": code_3}

    id = ""

    if c["code"] not in codes and c["code"] != "":
        codes[c["code"]] = c
        if id == "":
            id = c["code"]
            
    if c["code_2t"] not in code_2ts and c["code_2t"] != "":
        code_2ts[c["code_2t"]] = c
        if id == "":
            id = c["code_2t"]

    if c["code_2b"] not in code_2bs and c["code_2b"] != "":
        code_2bs[c["code_2b"]] = c
        if id == "":
            id = c["code_2b"]

    if c["code_3"] not in code_3s and c["code_3"] != "":
        code_3s[c["code_3"]] = c
        if id == "":
            id = c["code_3"]

    if id !="" and id not in all_map:
        all_codes.append(c)
        all_map[id] = c

#print(all_map)
#sys.exit(0)

for x in f3.readlines():
    x = x.strip()
    ts = x.split("\t")
    code = ts[3].strip()
    code_2b = ts[1].strip()
    code_2t = ts[2].strip()
    code_3 = ts[0].strip()
    if code_3 == "Id":
        continue
    name = ts[6].strip()

    c = {"code": code, "name": name, "code_2t": code_2t,
         "code_2b": code_2b, "code_3": code_3}


    id = ""

    if c["code"] not in codes and c["code"] != "":
        codes[c["code"]] = c
        if id == "":
            id = c["code"]
            
    if c["code_2t"] not in code_2ts and c["code_2t"] != "":
        code_2ts[c["code_2t"]] = c
        if id == "":
            id = c["code_2t"]
            
    if c["code_2b"] not in code_2bs and c["code_2b"] != "":
        code_2bs[c["code_2b"]] = c
        if id == "":
            id = c["code_2b"]

    if c["code_3"] not in code_3s and c["code_3"] != "":
        code_3s[c["code_3"]] = c
        if id == "":
            id = c["code_3"]

    if id !="" and id not in all_map:
        all_codes.append(c)
        all_map[id] = c


# print(code_2ts)
#   sys.exit(0)

for x in f31.readlines():
    x = x.strip()
    ts = x.split("\t")
    if ts[0] == "M_Id":
        continue
    code_3 = ts[0].strip()
    individual_language = ts[1].strip()
    for x in all_codes:
        if "individual_languages" not in x:
            x["individual_languages"] = []
        if code_3 == x["code_3"]:
            x["individual_languages"].append(individual_language)
    for key in all_map:
        if "individual_languages" not in all_map[key]:
            all_map[key]["individual_languages"] = []
        if code_3 == x["code_3"]:
            all_map[key]["individual_languages"].append(individual_language)
    for key in codes:
        if "individual_languages" not in codes[key]:
            codes[key]["individual_languages"] = []
        if code_3 == x["code_3"]:
            codes[key]["individual_languages"].append(individual_language)
    for key in code_2ts:
        if "individual_languages" not in code_2ts[key]:
            code_2ts[key]["individual_languages"] = []
        if code_3 == x["code_3"]:
            code_2ts[key]["individual_languages"].append(individual_language)
    for key in code_2bs:
        if "individual_languages" not in code_2bs[key]:
            code_2bs[key]["individual_languages"] = []
        if code_3 == x["code_3"]:
            code_2bs[key]["individual_languages"].append(individual_language)
    for key in code_3s:
        if "individual_languages" not in code_3s[key]:
            code_3s[key]["individual_languages"] = []
        if code_3 == x["code_3"]:
            code_3s[key]["individual_languages"].append(individual_language)

for x in f32.readlines():
    x = x.strip()
    ts = x.split("\t")
    if ts[0] == "Id":
        continue
    name_3_map[ts[0]] = ts[1]


for x in all_codes:
    if not "id" in x:
        x["id"] = ""
    if x["id"] == "":
        if x["code"] != "":
            x["id"] = x["code"]
        elif x["code_2t"] != "":
            x["id"] = x["code_2t"]
        elif x["code_2b"] != "":
            x["id"] = x["code_2b"]
        elif x["code_3"] != "":
            x["id"] = x["code_3"]


prefix = """use phf::{phf_map, Map};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use js_sys::Array;

#[cfg(test)]
mod tests {

    #[test]
    fn test_from_code_1() {
        let l = crate::from_code_1("zh");
        print!("test_from_code result {:?}", l)
    }

    #[test]
    fn test_from_code_2t() {
        let l = crate::from_code_2t("zho");
        print!("test_from_code_2t result {:?}", l)
    }

    #[test]
    fn test_from_code_2b() {
        let l = crate::from_code_2b("chi");
        print!("test_from_code_2b result {:?}", l)
    }

    #[test]
    fn test_from_code_3() {
        let l = crate::from_code_3("zho");
        print!("test_from_code_3 result {:?}", l)
    }

    #[test]
    fn test_all() {
        println!("{:?}", crate::ALL);
        println!("{:?}", crate::ALL_1);
        println!("{:?}", crate::ALL_2B);
        println!("{:?}", crate::ALL_2T);
        println!("{:?}", crate::ALL_3);


        println!("{:?}", crate::ALL_MAP);
        println!("{:?}", crate::ALL_1_MAP);
        println!("{:?}", crate::ALL_2B_MAP);
        println!("{:?}", crate::ALL_2T_MAP);
        println!("{:?}", crate::ALL_3_MAP);

    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct LanguageCode {
    ///ISO Language Name
    name: &'static str,
    ///639-1
    code: &'static str,
    ///639-2/T
    code_2t: &'static str,
    ///639-2/B
    code_2b: &'static str,
    //639-3 Macrolanguage
    code_3: &'static str,
    individual_languages: &'static[IndividualLanguages],
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl LanguageCode {
     #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.into()
    }
     #[wasm_bindgen(getter)]
    pub fn code(&self) -> String {
        self.code.into()
    }

    #[wasm_bindgen(getter)]
    pub fn code_2t(&self) -> String {
        self.code_2t.into()
    }

    #[wasm_bindgen(getter)]
    pub fn code_2b(&self) -> String {
        self.code_2b.into()
    }

    #[wasm_bindgen(getter)]
    pub fn code_3(&self) -> String {
        self.code_3.into()
    }
	
    #[wasm_bindgen(getter)]
    pub fn individual_languages(&self) -> Array {
		let mut vector: Vec<IndividualLanguages> = Vec::new(); 
        // self.individual_languages.into_serde().unwrap();
		for i in 0..self.individual_languages.len() {
			vector.push(self.individual_languages[i])
		}
		vector.into_iter().map(JsValue::from).collect()
    }
}


#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct IndividualLanguages {
    ///Name
    name: &'static str,
    ///Code
    code: &'static str,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct IndividualLanguages {
    ///Name
    pub name: &'static str,
    ///Code
    pub code: &'static str,
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl IndividualLanguages {
     #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.into()
    }
     #[wasm_bindgen(getter)]
    pub fn code(&self) -> String {
        self.code.into()
    }
}



#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn from_code_1(code: &str) -> Option<LanguageCode> {
    let up = code.to_lowercase();
    ALL_1_MAP.get(up.as_str()).cloned()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn from_code_2t(code: &str) -> Option<LanguageCode> {
    let up = code.to_lowercase();
    ALL_2T_MAP.get(up.as_str()).cloned()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn from_code_2b(code: &str) -> Option<LanguageCode> {
    let up = code.to_lowercase();
    ALL_2B_MAP.get(up.as_str()).cloned()
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn from_code_3(code: &str) -> Option<LanguageCode> {
    let up = code.to_lowercase();
    ALL_3_MAP.get(up.as_str()).cloned()
}
"""
print(prefix)


for x in all_codes:
    x["id"] = re.sub(re.compile("\-.*$"), "", x["id"]).strip()
    x["name"] = x["name"].replace('"', '\\"')
for x in all_codes:
    rs_code = """
pub const %s: LanguageCode = LanguageCode {
    name: "%s",
    code: "%s",
    code_2t: "%s",
    code_2b: "%s",
    code_3: "%s",
    individual_languages: &[
"""
    rs_code = rs_code % (x["id"].upper(), x["name"], x["code"].lower(),
                         x["code_2t"].lower(), x["code_2b"].lower(), x["code_3"].lower())
    if len(x["individual_languages"]) > 0:
        for individual_language in x["individual_languages"]:
            name = ""
            if individual_language in name_3_map:
                name = name_3_map[individual_language]

            rs_code += """        IndividualLanguages {
            name: "%s",
            code: "%s",
        },\n""" % (name, individual_language)
    rs_code += """    ],
};
"""
    print(rs_code)


rs_code = """
pub const ALL_CODES: & [LanguageCode] = &[
"""
for x in all_codes:
    rs_code += x["id"].upper() + ",\n"

rs_code += """
];
"""
print(rs_code)


rs_code = """
pub const ALL_1: & [LanguageCode] = &[
"""
for x in all_codes:
    if x["code"] in codes:
        rs_code += x["id"].upper() + ",\n"

rs_code += """
];
"""
print(rs_code)

rs_code = """
pub const ALL_2T: & [LanguageCode] = &[
"""
for x in all_codes:
    if x["code_2t"] in code_2ts:
        rs_code += x["id"].upper() + ",\n"

rs_code += """
];
"""
print(rs_code)


rs_code = """
pub const ALL_2B: & [LanguageCode] = &[
"""
for x in all_codes:
    if x["code_2b"] in code_2bs:
        rs_code += x["id"].upper() + ",\n"

rs_code += """
];
"""
print(rs_code)


rs_code = """
pub const ALL_3: & [LanguageCode] = &[
"""
for x in all_codes:
    if x["code_3"] in code_3s:
        rs_code += x["id"].upper() + ",\n"

rs_code += """
];
"""
print(rs_code)


rs_code = """
pub const ALL_MAP: Map<&str, LanguageCode> = phf_map! {
"""
for x in all_codes:
    rs_code += "\"" + x["id"].lower() + "\" => " + x["id"].upper() + ",\n"

rs_code += """
};
"""
print(rs_code)


rs_code = """
pub const ALL_1_MAP: Map<&str, LanguageCode> = phf_map! {
"""
for x in all_codes:
    if x["code"] in codes:
        rs_code += "\"" + x["id"].lower() + "\" => " + x["id"].upper() + ",\n"

rs_code += """
};
"""
print(rs_code)


rs_code = """
pub const ALL_2B_MAP: Map<&str, LanguageCode> = phf_map! {
"""
for x in all_codes:
    if x["code_2b"] in code_2bs:
        rs_code += "\"" + x["code_2b"].lower() + "\" => " + \
            x["id"].upper() + ",\n"

rs_code += """
};
"""
print(rs_code)


rs_code = """
pub const ALL_2T_MAP: Map<&str, LanguageCode> = phf_map! {
"""
for x in all_codes:
    if x["code_2t"] in code_2bs:
        rs_code += "\"" + x["code_2t"].lower() + "\" => " + \
            x["id"].upper() + ",\n"

rs_code += """
};
"""
print(rs_code)


rs_code = """
pub const ALL_3_MAP: Map<&str, LanguageCode> = phf_map! {
"""
for x in all_codes:
    if x["code_3"] in code_3s:
        rs_code += "\"" + x["code_3"].lower() + "\" => " + \
            x["id"].upper() + ",\n"

rs_code += """
};
"""
print(rs_code)
sys.exit(0)
