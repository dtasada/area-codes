mod fzf;
use std::collections::hash_map::HashMap;

use rand::seq::IndexedRandom;
use termion::input::TermRead;

#[derive(Debug, Clone)]
struct Code {
    country_name: &'static str,
    iso_codes: [&'static str; 2],
    num_codes: Vec<&'static str>,
}

impl Code {
    fn new(
        country_name: &'static str,
        iso_codes: [&'static str; 2],
        num_codes: Vec<&'static str>,
    ) -> Self {
        Self {
            country_name,
            iso_codes,
            num_codes,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let codes: Vec<Code> = vec![
        Code::new("Afghanistan", ["AF", "AFG"], vec!["93"]),
        Code::new("Albania", ["AL", "ALB"], vec!["355"]),
        Code::new("Algeria", ["DZ", "DZA"], vec!["213"]),
        Code::new("American Samoa", ["AS", "ASM"], vec!["1 (684)"]),
        Code::new("Andorra", ["AD", "AND"], vec!["376"]),
        Code::new("Angola", ["AO", "AGO"], vec!["244"]),
        Code::new("Anguilla", ["AI", "AIA"], vec!["1 (264)"]),
        Code::new("Antarctica", ["AQ", "ATA"], vec!["672", "64"]),
        Code::new("Antigua and Barbuda", ["AG", "ATG"], vec!["1268"]),
        Code::new("Argentina", ["AR", "ARG"], vec!["54"]),
        Code::new("Armenia", ["AM", "ARM"], vec!["374"]),
        Code::new("Aruba", ["AW", "ABW"], vec!["297"]),
        Code::new("Ascension Island", ["AC", "ASC"], vec!["247"]),
        Code::new("Australia", ["AU", "AUS"], vec!["61"]),
        Code::new("Austria", ["AT", "AUT"], vec!["43"]),
        Code::new("Azerbaijan", ["AZ", "AZE"], vec!["994"]),
        Code::new("Bahamas", ["BS", "BHS"], vec!["1 (242)"]),
        Code::new("Bahrain", ["BH", "BHR"], vec!["973"]),
        Code::new("Bangladesh", ["BD", "BGD"], vec!["880"]),
        Code::new("Barbados", ["BB", "BRB"], vec!["1 (246)"]),
        Code::new("Belarus", ["BY", "BLR"], vec!["375"]),
        Code::new("Belgium", ["BE", "BEL"], vec!["32"]),
        Code::new("Belize", ["BZ", "BLZ"], vec!["501"]),
        Code::new("Benin", ["BJ", "BEN"], vec!["229"]),
        Code::new("Bermuda", ["BM", "BMU"], vec!["1 (441)"]),
        Code::new("Bhutan", ["BT", "BTN"], vec!["975"]),
        Code::new("Bolivia", ["BO", "BOL"], vec!["591"]),
        Code::new("Bosnia and Herzegovina", ["BA", "BIH"], vec!["387"]),
        Code::new("Botswana", ["BW", "BWA"], vec!["267"]),
        Code::new("Brazil", ["BR", "BRA"], vec!["55"]),
        Code::new("British Virgin Islands", ["VG", "VGB"], vec!["1 (284)"]),
        Code::new("Brunei", ["BN", "BRN"], vec!["673"]),
        Code::new("Bulgaria", ["BG", "BGR"], vec!["359"]),
        Code::new("Burkina Faso", ["BF", "BFA"], vec!["226"]),
        Code::new("Burma (Myanmar)", ["MM", "MMR"], vec!["95"]),
        Code::new("Burundi", ["BI", "BDI"], vec!["257"]),
        Code::new("Cambodia", ["KH", "KHM"], vec!["855"]),
        Code::new("Cameroon", ["CM", "CMR"], vec!["237"]),
        Code::new("Canada", ["CA", "CAN"], vec!["1"]),
        Code::new("Cape Verde", ["CV", "CPV"], vec!["238"]),
        Code::new("Cayman Islands", ["KY", "CYM"], vec!["1 345"]),
        Code::new("Central African Republic", ["CF", "CAF"], vec!["236"]),
        Code::new("Chad", ["TD", "TCD"], vec!["235"]),
        Code::new("Chile", ["CL", "CHL"], vec!["56"]),
        Code::new("China", ["CN", "CHN"], vec!["86"]),
        Code::new("Christmas Island", ["CX", "CXR"], vec!["61"]),
        Code::new("Cocos (Keeling) Islands", ["CC", "CCK"], vec!["61"]),
        Code::new("Colombia", ["CO", "COL"], vec!["57"]),
        Code::new("Comoros", ["KM", "COM"], vec!["269"]),
        Code::new("Congo", ["CG", "COG"], vec!["242"]),
        Code::new("Cook Islands", ["CK", "COK"], vec!["682"]),
        Code::new("Costa Rica", ["CR", "CRC"], vec!["506"]),
        Code::new("Croatia", ["HR", "HRV"], vec!["385"]),
        Code::new("Cuba", ["CU", "CUB"], vec!["53"]),
        Code::new("Cyprus", ["CY", "CYP"], vec!["357"]),
        Code::new("Czech Republic", ["CZ", "CZE"], vec!["420"]),
        Code::new(
            "Democratic Republic of the Congo",
            ["CD", "COD"],
            vec!["243"],
        ),
        Code::new("Denmark", ["DK", "DNK"], vec!["45"]),
        Code::new("Diego Garcia", ["DG", "DGA"], vec!["246"]),
        Code::new("Djibouti", ["DJ", "DJI"], vec!["253"]),
        Code::new("Dominica", ["DM", "DMA"], vec!["1 (767)"]),
        Code::new(
            "Dominican Republic",
            ["DO", "DOM"],
            vec!["1 (809)", "1 (829)", "(1 849)"],
        ),
        Code::new("Ecuador", ["EC", "ECU"], vec!["593"]),
        Code::new("Egypt", ["EG", "EGY"], vec!["20"]),
        Code::new("El Salvador", ["SV", "SLV"], vec!["503"]),
        Code::new("Equatorial Guinea", ["GQ", "GNQ"], vec!["240"]),
        Code::new("Eritrea", ["ER", "ERI"], vec!["291"]),
        Code::new("Estonia", ["EE", "EST"], vec!["372"]),
        Code::new("Ethiopia", ["ET", "ETH"], vec!["251"]),
        Code::new("Falkland Islands", ["FK", "FLK"], vec!["500"]),
        Code::new("Faroe Islands", ["FO", "FRO"], vec!["298"]),
        Code::new("Fiji", ["FJ", "FJI"], vec!["679"]),
        Code::new("Finland", ["FI", "FIN"], vec!["358"]),
        Code::new("France", ["FR", "FRA"], vec!["33"]),
        Code::new("French Guiana", ["GF", "GUF"], vec!["594"]),
        Code::new("French Polynesia", ["PF", "PYF"], vec!["689"]),
        Code::new("Gabon", ["GA", "GAB"], vec!["241"]),
        Code::new("Gambia", ["GM", "GMB"], vec!["220"]),
        Code::new("Georgia", ["GE", "GEO"], vec!["995"]),
        Code::new("Germany", ["DE", "DEU"], vec!["49"]),
        Code::new("Ghana", ["GH", "GHA"], vec!["233"]),
        Code::new("Gibraltar", ["GI", "GIB"], vec!["350"]),
        Code::new("Greece", ["GR", "GRC"], vec!["30"]),
        Code::new("Greenland", ["GL", "GRL"], vec!["299"]),
        Code::new("Grenada", ["GD", "GRD"], vec!["1 (473)"]),
        Code::new("Guadeloupe", ["GP", "GLP"], vec!["590"]),
        Code::new("Guam", ["GU", "GUM"], vec!["1 (671)"]),
        Code::new("Guatemala", ["GT", "GTM"], vec!["502"]),
        Code::new("Guinea", ["GN", "GIN"], vec!["224"]),
        Code::new("Guinea-Bissau", ["GW", "GNB"], vec!["245"]),
        Code::new("Guyana", ["GY", "GUY"], vec!["592"]),
        Code::new("Haiti", ["HT", "HTI"], vec!["509"]),
        Code::new("Holy See (Vatican City)", ["VA", "VAT"], vec!["39"]),
        Code::new("Honduras", ["HN", "HND"], vec!["504"]),
        Code::new("Hong Kong", ["HK", "HKG"], vec!["852"]),
        Code::new("Hungary", ["HU", "HUN"], vec!["36"]),
        Code::new("Iceland", ["IS", "IS"], vec![" 354"]),
        Code::new("India", ["IN", "IND"], vec!["91"]),
        Code::new("Indonesia", ["ID", "IDN"], vec!["62"]),
        Code::new("Iran", ["IR", "IRN"], vec!["98"]),
        Code::new("Iraq", ["IQ", "IRQ"], vec!["964"]),
        Code::new("Ireland", ["IE", "IRL"], vec!["353"]),
        Code::new("Isle of Man", ["IM", "IMN"], vec!["44"]),
        Code::new("Israel", ["IL", "ISR"], vec!["972"]),
        Code::new("Italy", ["IT", "ITA"], vec!["39"]),
        Code::new("Ivory Coast (Côte d'Ivoire)", ["CI", "CIV"], vec!["225"]),
        Code::new("Jamaica", ["JM", "JAM"], vec!["1 (876)"]),
        Code::new("Japan", ["JP", "JPN"], vec!["81"]),
        Code::new("Jersey", ["JE", "JEY"], vec!["44"]),
        Code::new("Jordan", ["JO", "JOR"], vec!["962"]),
        Code::new("Kazakhstan", ["KZ", "KAZ"], vec!["7"]),
        Code::new("Kenya", ["KE", "KEN"], vec!["254"]),
        Code::new("Kiribati", ["KI", "KIR"], vec!["686"]),
        Code::new("Kuwait", ["KW", "KWT"], vec!["965"]),
        Code::new("Kyrgyzstan", ["KG", "KGZ"], vec!["996"]),
        Code::new("Laos", ["LA", "LAO"], vec!["856"]),
        Code::new("Latvia", ["LV", "LVA"], vec!["371"]),
        Code::new("Lebanon", ["LB", "LBN"], vec!["961"]),
        Code::new("Lesotho", ["LS", "LSO"], vec!["266"]),
        Code::new("Liberia", ["LR", "LBR"], vec!["231"]),
        Code::new("Libya", ["LY", "LBY"], vec!["218"]),
        Code::new("Liechtenstein", ["LI", "LIE"], vec!["423"]),
        Code::new("Lithuania", ["LT", "LTU"], vec!["370"]),
        Code::new("Luxembourg", ["LU", "LUX"], vec!["352"]),
        Code::new("Macau", ["MO", "MAC"], vec!["853"]),
        Code::new("Macedonia", ["MK", "MKD"], vec!["389"]),
        Code::new("Madagascar", ["MG", "MDG"], vec!["261"]),
        Code::new("Malawi", ["MW", "MWI"], vec!["265"]),
        Code::new("Malaysia", ["MY", "MYS"], vec!["60"]),
        Code::new("Maldives", ["MV", "MDV"], vec!["960"]),
        Code::new("Mali", ["ML", "MLI"], vec!["223"]),
        Code::new("Malta", ["MT", "MLT"], vec!["356"]),
        Code::new("Marshall Islands", ["MH", "MHL"], vec!["692"]),
        Code::new("Martinique", ["MQ", "MTQ"], vec!["596"]),
        Code::new("Mauritania", ["MR", "MRT"], vec!["222"]),
        Code::new("Mauritius", ["MU", "MUS"], vec!["230"]),
        Code::new("Mayotte", ["YT", "MYT"], vec!["262"]),
        Code::new("Mexico", ["MX", "MEX"], vec!["52"]),
        Code::new("Micronesia", ["FM", "FSM"], vec!["691"]),
        Code::new("Moldova", ["MD", "MDA"], vec!["373"]),
        Code::new("Monaco", ["MC", "MCO"], vec!["377"]),
        Code::new("Mongolia", ["MN", "MNG"], vec!["976"]),
        Code::new("Montenegro", ["ME", "MNE"], vec!["382"]),
        Code::new("Montserrat", ["MS", "MSR"], vec!["1 (664)"]),
        Code::new("Morocco", ["MA", "MAR"], vec!["212"]),
        Code::new("Mozambique", ["MZ", "MOZ"], vec!["258"]),
        Code::new("Namibia", ["NA", "NAM"], vec!["264"]),
        Code::new("Nauru", ["NR", "NRU"], vec!["674"]),
        Code::new("Nepal", ["NP", "NPL"], vec!["977"]),
        Code::new("Netherlands", ["NL", "NLD"], vec!["31"]),
        Code::new("Netherlands Antilles", ["AN", "ANT"], vec!["599"]),
        Code::new("New Caledonia", ["NC", "NCL"], vec!["687"]),
        Code::new("New Zealand", ["NZ", "NZL"], vec!["64"]),
        Code::new("Nicaragua", ["NI", "NIC"], vec!["505"]),
        Code::new("Niger", ["NE", "NER"], vec!["227"]),
        Code::new("Nigeria", ["NG", "NGA"], vec!["234"]),
        Code::new("Niue", ["NU", "NIU"], vec!["683"]),
        Code::new("Norfolk Island", ["NF", "NFK"], vec!["672"]),
        Code::new("North Korea", ["KP", "PRK"], vec!["850"]),
        Code::new("Northern Mariana Islands", ["MP", "MNP"], vec!["1 (670)"]),
        Code::new("Norway", ["NO", "NOR"], vec!["47"]),
        Code::new("Oman", ["OM", "OMN"], vec!["968"]),
        Code::new("Pakistan", ["PK", "PAK"], vec!["92"]),
        Code::new("Palau", ["PW", "PLW"], vec!["680"]),
        Code::new("Palestine", ["PS", "PSE"], vec!["970"]),
        Code::new("Panama", ["PA", "PAN"], vec!["507"]),
        Code::new("Papua New Guinea", ["PG", "PNG"], vec!["675"]),
        Code::new("Paraguay", ["PY", "PRY"], vec!["595"]),
        Code::new("Peru", ["PE", "PER"], vec!["51"]),
        Code::new("Philippines", ["PH", "PHL"], vec!["63"]),
        Code::new("Pitcairn Islands", ["PN", "PCN"], vec!["870"]),
        Code::new("Poland", ["PL", "POL"], vec!["48"]),
        Code::new("Portugal", ["PT", "PRT"], vec!["351"]),
        Code::new("Puerto Rico", ["PR", "PRI"], vec!["1 (787)", "1 (939)"]),
        Code::new("Qatar", ["QA", "QAT"], vec!["974"]),
        Code::new("Republic of the Congo", ["CG", "COG"], vec!["242"]),
        Code::new("Reunion Island", ["RE", "REU"], vec!["262"]),
        Code::new("Romania", ["RO", "ROU"], vec!["40"]),
        Code::new("Russia", ["RU", "RUS"], vec!["7"]),
        Code::new("Rwanda", ["RW", "RWA"], vec!["250"]),
        Code::new("Saint Barthelemy", ["BL", "BLM"], vec!["590"]),
        Code::new("Saint Helena", ["SH", "SHN"], vec!["290"]),
        Code::new("Saint Kitts and Nevis", ["KN", "KNA"], vec!["1 (869)"]),
        Code::new("Saint Lucia", ["LC", "LCA"], vec!["1 (758)"]),
        Code::new("Saint Martin", ["MF", "MAF"], vec!["590"]),
        Code::new("Saint Pierre and Miquelon", ["PM", "SPM"], vec!["508"]),
        Code::new(
            "Saint Vincent and the Grenadines",
            ["VC", "VCT"],
            vec!["1 (784)"],
        ),
        Code::new("Samoa", ["WS", "WSM"], vec!["685"]),
        Code::new("San Marino", ["SM", "SMR"], vec!["378"]),
        Code::new("Sao Tome and Principe", ["ST", "STP"], vec!["239"]),
        Code::new("Saudi Arabia", ["SA", "SAU"], vec!["966"]),
        Code::new("Senegal", ["SN", "SEN"], vec!["221"]),
        Code::new("Serbia", ["RS", "SRB"], vec!["381"]),
        Code::new("Seychelles", ["SC", "SYC"], vec!["248"]),
        Code::new("Sierra Leone", ["SL", "SLE"], vec!["232"]),
        Code::new("Singapore", ["SG", "SGP"], vec!["65"]),
        Code::new("Sint Maarten", ["SX", "SXM"], vec!["1 (721)"]),
        Code::new("Slovakia", ["SK", "SVK"], vec!["421"]),
        Code::new("Slovenia", ["SI", "SVN"], vec!["386"]),
        Code::new("Solomon Islands", ["SB", "SLB"], vec!["677"]),
        Code::new("Somalia", ["SO", "SOM"], vec!["252"]),
        Code::new("South Africa", ["ZA", "ZAF"], vec!["27"]),
        Code::new("South Korea", ["KR", "KOR"], vec!["82"]),
        Code::new("South Sudan", ["SS", "SSD"], vec!["211"]),
        Code::new("Spain", ["ES", "ESP"], vec!["34"]),
        Code::new("Sri Lanka", ["LK", "LKA"], vec!["94"]),
        Code::new("Sudan", ["SD", "SDN"], vec!["249"]),
        Code::new("Suriname", ["SR", "SUR"], vec!["597"]),
        Code::new("Svalbard", ["SJ", "SJM"], vec!["47"]),
        Code::new("Swaziland", ["SZ", "SWZ"], vec!["268"]),
        Code::new("Sweden", ["SE", "SWE"], vec!["46"]),
        Code::new("Switzerland", ["CH", "CHE"], vec!["41"]),
        Code::new("Syria", ["SY", "SYR"], vec!["963"]),
        Code::new("Taiwan", ["TW", "TWN"], vec!["886"]),
        Code::new("Tajikistan", ["TJ", "TJK"], vec!["992"]),
        Code::new("Tanzania", ["TZ", "TZA"], vec!["255"]),
        Code::new("Thailand", ["TH", "THA"], vec!["66"]),
        Code::new("Timor-Leste (East Timor)", ["TL", "TLS"], vec!["670"]),
        Code::new("Togo", ["TG", "TGO"], vec!["228"]),
        Code::new("Tokelau", ["TK", "TKL"], vec!["690"]),
        Code::new("Tonga Islands", ["TO", "TON"], vec!["676"]),
        Code::new("Trinidad and Tobago", ["TT", "TTO"], vec!["1 (868)"]),
        Code::new("Tunisia", ["TN", "TUN"], vec!["216"]),
        Code::new("Turkey", ["TR", "TUR"], vec!["90"]),
        Code::new("Turkmenistan", ["TM", "TKM"], vec!["993"]),
        Code::new("Turks and Caicos Islands", ["TC", "TCA"], vec!["1 (649)"]),
        Code::new("Tuvalu", ["TV", "TUV"], vec!["688"]),
        Code::new("Uganda", ["UG", "UGA"], vec!["256"]),
        Code::new("Ukraine", ["UA", "UKR"], vec!["380"]),
        Code::new("United Arab Emirates", ["AE", "ARE"], vec!["971"]),
        Code::new("United Kingdom", ["GB", "GBR"], vec!["44"]),
        Code::new("United States", ["US", "USA"], vec!["1"]),
        Code::new("Uruguay", ["UY", "URY"], vec!["598"]),
        Code::new("US Virgin Islands", ["VI", "VIR"], vec!["1 (340)"]),
        Code::new("Uzbekistan", ["UZ", "UZB"], vec!["998"]),
        Code::new("Vanuatu", ["VU", "VUT"], vec!["678"]),
        Code::new("Venezuela", ["VE", "VEN"], vec!["58"]),
        Code::new("Vietnam", ["VN", "VNM"], vec!["84"]),
        Code::new("Wallis and Futuna", ["WF", "WLF"], vec!["681"]),
        Code::new("Western Sahara", ["EH", "ESH"], vec!["212"]),
        Code::new("Yemen", ["YE", "YEM"], vec!["967"]),
        Code::new("Zambia", ["ZM", "ZMB"], vec!["260"]),
        Code::new("Zimbabwe", ["ZW", "ZWE"], vec!["263"]),
        Code::new("idk", ["idk", "idk"], vec!["idk"]),
    ];

    let mut codes_fzf: Vec<fzf::Item<&Code>> = Vec::new();

    for code in &codes {
        codes_fzf.push(fzf::Item::new(
            code.country_name.to_string(),
            code.iso_codes.iter().map(|s| s.to_string()).collect(),
            code,
        ));
    }

    // maps a country's name to its code struct
    let name_map: HashMap<&'static str, &Code> =
        codes.iter().map(|code| (code.country_name, code)).collect();

    // maps each of a country's iso codes to its code struct
    let iso_map: HashMap<&'static str, &Code> = codes
        .iter()
        .flat_map(|code| {
            code.iso_codes
                .iter()
                .map(|iso| (*iso, code))
                .collect::<Vec<(&'static str, &Code)>>()
        })
        .collect();

    // maps each of a country's numerical codes to its code struct
    let code_map: HashMap<&'static str, &Code> = codes
        .iter()
        .flat_map(|code| {
            code.num_codes
                .iter()
                .map(|num| (*num, code))
                .collect::<Vec<(&'static str, &Code)>>()
        })
        .collect();

    println!(
        "Visit \x1b[4mhttps://en.wikipedia.org/wiki/List_of_telephone_country_codes#/media/File:Country_calling_codes_map.svg\x1b[0m for help."
    );

    let mut feedback = String::new();
    let mut stdin = termion::async_stdin().keys();
    loop {
        let random_country = codes.choose(&mut rand::rng()).unwrap();
        if random_country.country_name.eq("idk") {
            continue;
        }

        let question = format!(
            "What is the corresponding country for area code{} +{}?",
            if random_country.num_codes.len() > 1 {
                "s"
            } else {
                ""
            },
            random_country.num_codes.join(", "),
        );

        println!("{}", feedback);
        println!("{}", question);

        let fzf = fzf::find(codes_fzf.clone(), 10, &mut stdin)?;
        let answer = match fzf {
            Some(answer) => answer,
            None => return Ok(()),
        };

        if std::ptr::eq(answer, random_country) {
            feedback = "\x1b[1;32mcorrect answer!\x1b[0m".to_string();
        } else {
            feedback = format!(
                "\x1b[1;31mwrong answer. Area code{} +{} corresponds to {}.\x1b[0m",
                if random_country.num_codes.len() > 1 {
                    "s"
                } else {
                    ""
                },
                random_country.num_codes.join(", "),
                random_country.country_name
            );
        }
    }
}
