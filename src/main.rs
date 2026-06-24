mod fzf;
use std::collections::hash_map::HashMap;

use rand::seq::IndexedRandom;
use termion::input::TermRead;

#[derive(Debug, Clone, PartialEq)]
struct Code {
    country_name: &'static str,
    aliases: Vec<&'static str>,
    num_codes: Vec<&'static str>,
}

impl Code {
    fn new(
        country_name: &'static str,
        aliases: Vec<&'static str>,
        num_codes: Vec<&'static str>,
    ) -> Self {
        Self {
            country_name,
            aliases,
            num_codes,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let codes: Vec<Code> = vec![
        Code::new("Afghanistan", vec!["AF", "AFG"], vec!["93"]),
        Code::new("Albania", vec!["AL", "ALB"], vec!["355"]),
        Code::new("Algeria", vec!["DZ", "DZA"], vec!["213"]),
        Code::new("American Samoa", vec!["AS", "ASM"], vec!["1 (684)"]),
        Code::new("Andorra", vec!["AD", "AND"], vec!["376"]),
        Code::new("Angola", vec!["AO", "AGO"], vec!["244"]),
        Code::new("Anguilla", vec!["AI", "AIA"], vec!["1 (264)"]),
        Code::new("Antarctica", vec!["AQ", "ATA"], vec!["672", "64"]),
        Code::new("Antigua and Barbuda", vec!["AG", "ATG"], vec!["1 (268)"]),
        Code::new("Argentina", vec!["AR", "ARG"], vec!["54"]),
        Code::new("Armenia", vec!["AM", "ARM"], vec!["374"]),
        Code::new("Aruba", vec!["AW", "ABW"], vec!["297"]),
        Code::new("Ascension Island", vec!["AC", "ASC"], vec!["247"]),
        Code::new("Australia", vec!["AU", "AUS"], vec!["61"]),
        Code::new("Austria", vec!["AT", "AUT"], vec!["43"]),
        Code::new("Azerbaijan", vec!["AZ", "AZE"], vec!["994"]),
        Code::new("Bahamas", vec!["BS", "BHS"], vec!["1 (242)"]),
        Code::new("Bahrain", vec!["BH", "BHR"], vec!["973"]),
        Code::new("Bangladesh", vec!["BD", "BGD"], vec!["880"]),
        Code::new("Barbados", vec!["BB", "BRB"], vec!["1 (246)"]),
        Code::new("Belarus", vec!["BY", "BLR"], vec!["375"]),
        Code::new("Belgium", vec!["BE", "BEL"], vec!["32"]),
        Code::new("Belize", vec!["BZ", "BLZ"], vec!["501"]),
        Code::new("Benin", vec!["BJ", "BEN"], vec!["229"]),
        Code::new("Bermuda", vec!["BM", "BMU"], vec!["1 (441)"]),
        Code::new("Bhutan", vec!["BT", "BTN"], vec!["975"]),
        Code::new("Bolivia", vec!["BO", "BOL"], vec!["591"]),
        Code::new("Bosnia and Herzegovina", vec!["BA", "BIH"], vec!["387"]),
        Code::new("Botswana", vec!["BW", "BWA"], vec!["267"]),
        Code::new("Brazil", vec!["BR", "BRA"], vec!["55"]),
        Code::new("British Virgin Islands", vec!["VG", "VGB"], vec!["1 (284)"]),
        Code::new("Brunei", vec!["BN", "BRN"], vec!["673"]),
        Code::new("Bulgaria", vec!["BG", "BGR"], vec!["359"]),
        Code::new("Burkina Faso", vec!["BF", "BFA"], vec!["226"]),
        Code::new("Burma (Myanmar)", vec!["MM", "MMR"], vec!["95"]),
        Code::new("Burundi", vec!["BI", "BDI"], vec!["257"]),
        Code::new("Cambodia", vec!["KH", "KHM"], vec!["855"]),
        Code::new("Cameroon", vec!["CM", "CMR"], vec!["237"]),
        Code::new("Canada", vec!["CA", "CAN"], vec!["1"]),
        Code::new("Cape Verde", vec!["CV", "CPV"], vec!["238"]),
        Code::new("Cayman Islands", vec!["KY", "CYM"], vec!["1 (345)"]),
        Code::new("Central African Republic", vec!["CF", "CAF"], vec!["236"]),
        Code::new("Chad", vec!["TD", "TCD"], vec!["235"]),
        Code::new("Chile", vec!["CL", "CHL"], vec!["56"]),
        Code::new("China", vec!["CN", "CHN", "prc", "proc"], vec!["86"]),
        Code::new("Christmas Island", vec!["CX", "CXR"], vec!["61"]),
        Code::new("Cocos (Keeling) Islands", vec!["CC", "CCK"], vec!["61"]),
        Code::new("Colombia", vec!["CO", "COL"], vec!["57"]),
        Code::new("Comoros", vec!["KM", "COM"], vec!["269"]),
        Code::new("Congo", vec!["CG", "COG"], vec!["242"]),
        Code::new("Cook Islands", vec!["CK", "COK"], vec!["682"]),
        Code::new("Costa Rica", vec!["CR", "CRC"], vec!["506"]),
        Code::new("Croatia", vec!["HR", "HRV"], vec!["385"]),
        Code::new("Cuba", vec!["CU", "CUB"], vec!["53"]),
        Code::new("Cyprus", vec!["CY", "CYP"], vec!["357"]),
        Code::new("Czech Republic", vec!["CZ", "CZE"], vec!["420"]),
        Code::new(
            "Democratic Republic of the Congo",
            vec!["CD", "COD"],
            vec!["243"],
        ),
        Code::new("Denmark", vec!["DK", "DNK"], vec!["45"]),
        Code::new("Diego Garcia", vec!["DG", "DGA"], vec!["246"]),
        Code::new("Djibouti", vec!["DJ", "DJI"], vec!["253"]),
        Code::new("Dominica", vec!["DM", "DMA"], vec!["1 (767)"]),
        Code::new(
            "Dominican Republic",
            vec!["DO", "DOM"],
            vec!["1 (809)", "1 (829)", "1 (849)"],
        ),
        Code::new("Ecuador", vec!["EC", "ECU"], vec!["593"]),
        Code::new("Egypt", vec!["EG", "EGY"], vec!["20"]),
        Code::new("El Salvador", vec!["SV", "SLV"], vec!["503"]),
        Code::new("Equatorial Guinea", vec!["GQ", "GNQ"], vec!["240"]),
        Code::new("Eritrea", vec!["ER", "ERI"], vec!["291"]),
        Code::new("Estonia", vec!["EE", "EST"], vec!["372"]),
        Code::new("Ethiopia", vec!["ET", "ETH"], vec!["251"]),
        Code::new("Falkland Islands", vec!["FK", "FLK"], vec!["500"]),
        Code::new("Faroe Islands", vec!["FO", "FRO"], vec!["298"]),
        Code::new("Fiji", vec!["FJ", "FJI"], vec!["679"]),
        Code::new("Finland", vec!["FI", "FIN"], vec!["358"]),
        Code::new("France", vec!["FR", "FRA"], vec!["33"]),
        Code::new("French Guiana", vec!["GF", "GUF"], vec!["594"]),
        Code::new("French Polynesia", vec!["PF", "PYF"], vec!["689"]),
        Code::new("Gabon", vec!["GA", "GAB"], vec!["241"]),
        Code::new("Gambia", vec!["GM", "GMB"], vec!["220"]),
        Code::new("Georgia", vec!["GE", "GEO"], vec!["995"]),
        Code::new("Germany", vec!["DE", "DEU"], vec!["49"]),
        Code::new("Ghana", vec!["GH", "GHA"], vec!["233"]),
        Code::new("Gibraltar", vec!["GI", "GIB"], vec!["350"]),
        Code::new("Greece", vec!["GR", "GRC"], vec!["30"]),
        Code::new("Greenland", vec!["GL", "GRL"], vec!["299"]),
        Code::new("Grenada", vec!["GD", "GRD"], vec!["1 (473)"]),
        Code::new("Guadeloupe", vec!["GP", "GLP"], vec!["590"]),
        Code::new("Guam", vec!["GU", "GUM"], vec!["1 (671)"]),
        Code::new("Guatemala", vec!["GT", "GTM"], vec!["502"]),
        Code::new("Guinea", vec!["GN", "GIN"], vec!["224"]),
        Code::new("Guinea-Bissau", vec!["GW", "GNB"], vec!["245"]),
        Code::new("Guyana", vec!["GY", "GUY"], vec!["592"]),
        Code::new("Haiti", vec!["HT", "HTI"], vec!["509"]),
        Code::new("Holy See (Vatican City)", vec!["VA", "VAT"], vec!["39"]),
        Code::new("Honduras", vec!["HN", "HND"], vec!["504"]),
        Code::new("Hong Kong", vec!["HK", "HKG"], vec!["852"]),
        Code::new("Hungary", vec!["HU", "HUN"], vec!["36"]),
        Code::new("Iceland", vec!["IS", "IS"], vec!["354"]),
        Code::new("India", vec!["IN", "IND"], vec!["91"]),
        Code::new("Indonesia", vec!["ID", "IDN"], vec!["62"]),
        Code::new("Iran", vec!["IR", "IRN"], vec!["98"]),
        Code::new("Iraq", vec!["IQ", "IRQ"], vec!["964"]),
        Code::new("Ireland", vec!["IE", "IRL"], vec!["353"]),
        Code::new("Isle of Man", vec!["IM", "IMN"], vec!["44"]),
        Code::new("Israel", vec!["IL", "ISR"], vec!["972"]),
        Code::new("Italy", vec!["IT", "ITA"], vec!["39"]),
        Code::new(
            "Ivory Coast (Côte d'Ivoire)",
            vec!["CI", "CIV"],
            vec!["225"],
        ),
        Code::new("Jamaica", vec!["JM", "JAM"], vec!["1 (876)"]),
        Code::new("Japan", vec!["JP", "JPN"], vec!["81"]),
        Code::new("Jersey", vec!["JE", "JEY"], vec!["44"]),
        Code::new("Jordan", vec!["JO", "JOR"], vec!["962"]),
        Code::new("Kazakhstan", vec!["KZ", "KAZ"], vec!["7"]),
        Code::new("Kenya", vec!["KE", "KEN"], vec!["254"]),
        Code::new("Kiribati", vec!["KI", "KIR"], vec!["686"]),
        Code::new("Kuwait", vec!["KW", "KWT"], vec!["965"]),
        Code::new("Kyrgyzstan", vec!["KG", "KGZ"], vec!["996"]),
        Code::new("Laos", vec!["LA", "LAO"], vec!["856"]),
        Code::new("Latvia", vec!["LV", "LVA"], vec!["371"]),
        Code::new("Lebanon", vec!["LB", "LBN"], vec!["961"]),
        Code::new("Lesotho", vec!["LS", "LSO"], vec!["266"]),
        Code::new("Liberia", vec!["LR", "LBR"], vec!["231"]),
        Code::new("Libya", vec!["LY", "LBY"], vec!["218"]),
        Code::new("Liechtenstein", vec!["LI", "LIE"], vec!["423"]),
        Code::new("Lithuania", vec!["LT", "LTU"], vec!["370"]),
        Code::new("Luxembourg", vec!["LU", "LUX"], vec!["352"]),
        Code::new("Macau", vec!["MO", "MAC"], vec!["853"]),
        Code::new("Macedonia", vec!["MK", "MKD"], vec!["389"]),
        Code::new("Madagascar", vec!["MG", "MDG"], vec!["261"]),
        Code::new("Malawi", vec!["MW", "MWI"], vec!["265"]),
        Code::new("Malaysia", vec!["MY", "MYS"], vec!["60"]),
        Code::new("Maldives", vec!["MV", "MDV"], vec!["960"]),
        Code::new("Mali", vec!["ML", "MLI"], vec!["223"]),
        Code::new("Malta", vec!["MT", "MLT"], vec!["356"]),
        Code::new("Marshall Islands", vec!["MH", "MHL"], vec!["692"]),
        Code::new("Martinique", vec!["MQ", "MTQ"], vec!["596"]),
        Code::new("Mauritania", vec!["MR", "MRT"], vec!["222"]),
        Code::new("Mauritius", vec!["MU", "MUS"], vec!["230"]),
        Code::new("Mayotte", vec!["YT", "MYT"], vec!["262"]),
        Code::new("Mexico", vec!["MX", "MEX"], vec!["52"]),
        Code::new("Micronesia", vec!["FM", "FSM"], vec!["691"]),
        Code::new("Moldova", vec!["MD", "MDA"], vec!["373"]),
        Code::new("Monaco", vec!["MC", "MCO"], vec!["377"]),
        Code::new("Mongolia", vec!["MN", "MNG"], vec!["976"]),
        Code::new("Montenegro", vec!["ME", "MNE"], vec!["382"]),
        Code::new("Montserrat", vec!["MS", "MSR"], vec!["1 (664)"]),
        Code::new("Morocco", vec!["MA", "MAR"], vec!["212"]),
        Code::new("Mozambique", vec!["MZ", "MOZ"], vec!["258"]),
        Code::new("Namibia", vec!["NA", "NAM"], vec!["264"]),
        Code::new("Nauru", vec!["NR", "NRU"], vec!["674"]),
        Code::new("Nepal", vec!["NP", "NPL"], vec!["977"]),
        Code::new("Netherlands", vec!["NL", "NLD"], vec!["31"]),
        Code::new("Netherlands Antilles", vec!["AN", "ANT"], vec!["599"]),
        Code::new("New Caledonia", vec!["NC", "NCL"], vec!["687"]),
        Code::new("New Zealand", vec!["NZ", "NZL"], vec!["64"]),
        Code::new("Nicaragua", vec!["NI", "NIC"], vec!["505"]),
        Code::new("Niger", vec!["NE", "NER"], vec!["227"]),
        Code::new("Nigeria", vec!["NG", "NGA"], vec!["234"]),
        Code::new("Niue", vec!["NU", "NIU"], vec!["683"]),
        Code::new("Norfolk Island", vec!["NF", "NFK"], vec!["672"]),
        Code::new("North Korea", vec!["KP", "PRK", "dprk"], vec!["850"]),
        Code::new(
            "Northern Mariana Islands",
            vec!["MP", "MNP"],
            vec!["1 (670)"],
        ),
        Code::new("Norway", vec!["NO", "NOR"], vec!["47"]),
        Code::new("Oman", vec!["OM", "OMN"], vec!["968"]),
        Code::new("Pakistan", vec!["PK", "PAK"], vec!["92"]),
        Code::new("Palau", vec!["PW", "PLW"], vec!["680"]),
        Code::new("Palestine", vec!["PS", "PSE"], vec!["970"]),
        Code::new("Panama", vec!["PA", "PAN"], vec!["507"]),
        Code::new("Papua New Guinea", vec!["PG", "PNG"], vec!["675"]),
        Code::new("Paraguay", vec!["PY", "PRY"], vec!["595"]),
        Code::new("Peru", vec!["PE", "PER"], vec!["51"]),
        Code::new("Philippines", vec!["PH", "PHL"], vec!["63"]),
        Code::new("Pitcairn Islands", vec!["PN", "PCN"], vec!["870"]),
        Code::new("Poland", vec!["PL", "POL"], vec!["48"]),
        Code::new("Portugal", vec!["PT", "PRT"], vec!["351"]),
        Code::new("Puerto Rico", vec!["PR", "PRI"], vec!["1 (787)", "1 (939)"]),
        Code::new("Qatar", vec!["QA", "QAT"], vec!["974"]),
        Code::new("Republic of the Congo", vec!["CG", "COG"], vec!["242"]),
        Code::new("Reunion Island", vec!["RE", "REU"], vec!["262"]),
        Code::new("Romania", vec!["RO", "ROU"], vec!["40"]),
        Code::new("Russia", vec!["RU", "RUS"], vec!["7"]),
        Code::new("Rwanda", vec!["RW", "RWA"], vec!["250"]),
        Code::new("Saint Barthelemy", vec!["BL", "BLM"], vec!["590"]),
        Code::new("Saint Helena", vec!["SH", "SHN"], vec!["290"]),
        Code::new("Saint Kitts and Nevis", vec!["KN", "KNA"], vec!["1 (869)"]),
        Code::new("Saint Lucia", vec!["LC", "LCA"], vec!["1 (758)"]),
        Code::new("Saint Martin", vec!["MF", "MAF"], vec!["590"]),
        Code::new("Saint Pierre and Miquelon", vec!["PM", "SPM"], vec!["508"]),
        Code::new(
            "Saint Vincent and the Grenadines",
            vec!["VC", "VCT"],
            vec!["1 (784)"],
        ),
        Code::new("Samoa", vec!["WS", "WSM"], vec!["685"]),
        Code::new("San Marino", vec!["SM", "SMR"], vec!["378"]),
        Code::new("Sao Tome and Principe", vec!["ST", "STP"], vec!["239"]),
        Code::new("Saudi Arabia", vec!["SA", "SAU"], vec!["966"]),
        Code::new("Senegal", vec!["SN", "SEN"], vec!["221"]),
        Code::new("Serbia", vec!["RS", "SRB"], vec!["381"]),
        Code::new("Seychelles", vec!["SC", "SYC"], vec!["248"]),
        Code::new("Sierra Leone", vec!["SL", "SLE"], vec!["232"]),
        Code::new("Singapore", vec!["SG", "SGP"], vec!["65"]),
        Code::new("Sint Maarten", vec!["SX", "SXM"], vec!["1 (721)"]),
        Code::new("Slovakia", vec!["SK", "SVK"], vec!["421"]),
        Code::new("Slovenia", vec!["SI", "SVN"], vec!["386"]),
        Code::new("Solomon Islands", vec!["SB", "SLB"], vec!["677"]),
        Code::new("Somalia", vec!["SO", "SOM"], vec!["252"]),
        Code::new("South Africa", vec!["ZA", "ZAF"], vec!["27"]),
        Code::new("South Korea", vec!["KR", "KOR", "rok"], vec!["82"]),
        Code::new("South Sudan", vec!["SS", "SSD"], vec!["211"]),
        Code::new("Spain", vec!["ES", "ESP"], vec!["34"]),
        Code::new("Sri Lanka", vec!["LK", "LKA"], vec!["94"]),
        Code::new("Sudan", vec!["SD", "SDN"], vec!["249"]),
        Code::new("Suriname", vec!["SR", "SUR"], vec!["597"]),
        Code::new("Svalbard", vec!["SJ", "SJM"], vec!["47"]),
        Code::new("Eswatini", vec!["SZ", "SWZ"], vec!["268"]),
        Code::new("Sweden", vec!["SE", "SWE"], vec!["46"]),
        Code::new("Switzerland", vec!["CH", "CHE"], vec!["41"]),
        Code::new("Syria", vec!["SY", "SYR"], vec!["963"]),
        Code::new("Taiwan", vec!["TW", "TWN", "roc"], vec!["886"]),
        Code::new("Tajikistan", vec!["TJ", "TJK"], vec!["992"]),
        Code::new("Tanzania", vec!["TZ", "TZA"], vec!["255"]),
        Code::new("Thailand", vec!["TH", "THA"], vec!["66"]),
        Code::new("Timor-Leste (East Timor)", vec!["TL", "TLS"], vec!["670"]),
        Code::new("Togo", vec!["TG", "TGO"], vec!["228"]),
        Code::new("Tokelau", vec!["TK", "TKL"], vec!["690"]),
        Code::new("Tonga Islands", vec!["TO", "TON"], vec!["676"]),
        Code::new("Trinidad and Tobago", vec!["TT", "TTO"], vec!["1 (868)"]),
        Code::new("Tunisia", vec!["TN", "TUN"], vec!["216"]),
        Code::new("Turkey", vec!["TR", "TUR"], vec!["90"]),
        Code::new("Turkmenistan", vec!["TM", "TKM"], vec!["993"]),
        Code::new(
            "Turks and Caicos Islands",
            vec!["TC", "TCA"],
            vec!["1 (649)"],
        ),
        Code::new("Tuvalu", vec!["TV", "TUV"], vec!["688"]),
        Code::new("Uganda", vec!["UG", "UGA"], vec!["256"]),
        Code::new("Ukraine", vec!["UA", "UKR"], vec!["380"]),
        Code::new("United Arab Emirates", vec!["AE", "ARE"], vec!["971"]),
        Code::new("United Kingdom", vec!["GB", "GBR", "uk"], vec!["44"]),
        Code::new("United States", vec!["US", "USA"], vec!["1"]),
        Code::new("Uruguay", vec!["UY", "URY"], vec!["598"]),
        Code::new("US Virgin Islands", vec!["VI", "VIR"], vec!["1 (340)"]),
        Code::new("Uzbekistan", vec!["UZ", "UZB"], vec!["998"]),
        Code::new("Vanuatu", vec!["VU", "VUT"], vec!["678"]),
        Code::new("Venezuela", vec!["VE", "VEN"], vec!["58"]),
        Code::new("Vietnam", vec!["VN", "VNM"], vec!["84"]),
        Code::new("Wallis and Futuna", vec!["WF", "WLF"], vec!["681"]),
        Code::new("Western Sahara", vec!["EH", "ESH"], vec!["212"]),
        Code::new("Yemen", vec!["YE", "YEM"], vec!["967"]),
        Code::new("Zambia", vec!["ZM", "ZMB"], vec!["260"]),
        Code::new("Zimbabwe", vec!["ZW", "ZWE"], vec!["263"]),
        Code::new("idk", vec![], vec![]),
        Code::new("exit", vec![], vec![]),
    ];

    let mut codes_fzf: Vec<fzf::Item<&Code>> = Vec::new();

    for code in &codes {
        codes_fzf.push(fzf::Item::new(
            code.country_name.to_string(),
            code.aliases.iter().map(|s| s.to_string()).collect(),
            code,
        ));
    }

    // maps each of a country's numerical codes to its code struct
    let mut code_map: HashMap<Vec<&'static str>, Vec<&Code>> = HashMap::new();
    for code in &codes {
        code_map
            .entry(code.num_codes.clone())
            .or_default()
            .push(code);
    }

    println!(
        "Visit \x1b[4mhttps://en.wikipedia.org/wiki/List_of_telephone_country_codes#/media/File:Country_calling_codes_map.svg\x1b[0m for help."
    );

    let mut feedback = String::new();
    let mut stdin = termion::async_stdin().keys();
    loop {
        let random_country = codes.choose(&mut rand::rng()).unwrap();
        let question = format!(
            "What is the corresponding country for area code{} +{}?",
            if random_country.num_codes.len() > 1 {
                "s"
            } else {
                ""
            },
            random_country.num_codes.join(", +"),
        );

        println!("{}", feedback);
        println!("{}", question);

        let fzf = fzf::find(codes_fzf.clone(), 10, &mut stdin)?;
        let answer = match fzf {
            Some(answer) => answer,
            None => continue,
        };

        match answer.country_name {
            "idk" => continue,
            "exit" => return Ok(()),
            _ => {}
        }

        let correct_answers = code_map.get(&random_country.num_codes).unwrap();

        feedback = if correct_answers.contains(&answer) {
            "\x1b[1;32mcorrect answer!\x1b[0m".to_string()
        } else {
            format!(
                "\x1b[1;31mwrong answer. Area code{} +{} correspond{} to {}.\x1b[0m",
                if random_country.num_codes.len() > 1 {
                    "s"
                } else {
                    ""
                },
                random_country.num_codes.join(", +"),
                if random_country.num_codes.len() > 1 {
                    ""
                } else {
                    "s"
                },
                random_country.country_name
            )
        }
    }
}
