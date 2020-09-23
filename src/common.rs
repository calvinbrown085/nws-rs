
#[allow(dead_code)]
#[derive(Debug)]
pub enum StateAbbr {
    AL,
    AK,
    AZ,
    AR,
    CA,
    CO,
    CT,
    DE,
    FL,
    GA,
    HI,
    ID,
    IL,
    IN,
    IA,
    KS,
    KY,
    LA,
    ME,
    MD,
    MA,
    MI,
    MN,
    MS,
    MO,
    MT,
    NE,
    NV,
    NH,
    NJ,
    NM,
    NY,
    NC,
    ND,
    OH,
    OK,
    OR,
    PA,
    RI,
    SC,
    SD,
    TN,
    TX,
    UT,
    VT,
    VA,
    WA,
    WV,
    WI,
    WY
}

#[test]
fn test_display_impl() {
    let state_abbr = StateAbbr::IA;
    format!("{:?}", state_abbr);
    assert_eq!("IA", format!("{:?}", state_abbr))
}