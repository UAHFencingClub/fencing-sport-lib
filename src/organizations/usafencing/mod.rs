pub mod pool_bout_orders;
pub mod fencer;
pub mod club;
use serde::{Serialize, Deserialize};
use email_address::EmailAddress;
use phonenumber::PhoneNumber;

/// I don't actually have a good reference other than this: https://cdn1.sportngin.com/attachments/document/0132/5185/USA_Fencing_Classification_Reference_Chart.pdf
/// Accessed April 17, 2024
#[derive(Debug)]
enum Rating {
    A(usize),
    B(usize),
    C(usize),
    D(usize),
    E(usize),
    NoRating,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
struct ContactInfo {
    email: Option<EmailAddress>,
    phone_number: Option<PhoneNumber>,
}

/// USA Fencing regions pulled from https://www.usafencing.org/regional-info
/// Accessed April 17, 2024
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
enum ClubRegion {
    Region1,
    Region2,
    Region3,
    Region4,
    Region5,
    Region6,
}

/// USA Fencing Division List: https://www.usafencing.org/page/show/2520204-division-information-for-members
/// Access April 17, 2024
// Written by passing the USA Fencing division list into chatgpt
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
enum Division {
    Alabama,
    Alaska,
    Arizona,
    ArkansasLouisianaMississippi,
    BorderTexas,
    Capitol,
    CentralCalifornia,
    CentralFlorida,
    CentralPennsylvania,
    Colorado,
    Columbus,
    Connecticut,
    GatewayFlorida,
    Georgia,
    GoldCoast,
    GreenMountain,
    GulfCoast,
    Harrisburg,
    Hawaii,
    HudsonBerkshire,
    Illinois,
    Indiana,
    InlandEmpire,
    Iowa,
    Kansas,
    Kentucky,
    LongIsland,
    Louisiana,
    Maryland,
    MetroNyc,
    Michigan,
    Minnesota,
    MountainValley,
    NebraskaSouthDakota,
    Nevada,
    NewEngland,
    NewJersey,
    NewMexico,
    NorthCarolina,
    NorthCoast,
    NorthTexas,
    NortheastPennsylvania,
    Northeast,
    NorthernCalifornia,
    NorthernOhio,
    Oklahoma,
    OrangeCoast,
    Oregon,
    Philadelphia,
    PlainsTexas,
    SanBernardino,
    SanDiego,
    SouthCarolina,
    SouthJersey,
    SouthTexas,
    SouthernCalifornia,
    SouthwestOhio,
    StLouis,
    Tennessee,
    UtahSouthernIdaho,
    Virginia,
    WestchesterRockland,
    WesternNewYork,
    WesternPennsylvania,
    WesternWashington,
    Wisconsin,
    Wyoming,
    Unclaimed,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
// Written by generative AI
enum USState {
    AL, // Alabama
    AK, // Alaska
    AZ, // Arizona
    AR, // Arkansas
    CA, // California
    CO, // Colorado
    CT, // Connecticut
    DE, // Delaware
    FL, // Florida
    GA, // Georgia
    HI, // Hawaii
    ID, // Idaho
    IL, // Illinois
    IN, // Indiana
    IA, // Iowa
    KS, // Kansas
    KY, // Kentucky
    LA, // Louisiana
    ME, // Maine
    MD, // Maryland
    MA, // Massachusetts
    MI, // Michigan
    MN, // Minnesota
    MS, // Mississippi
    MO, // Missouri
    MT, // Montana
    NE, // Nebraska
    NV, // Nevada
    NH, // New Hampshire
    NJ, // New Jersey
    NM, // New Mexico
    NY, // New York
    NC, // North Carolina
    ND, // North Dakota
    OH, // Ohio
    OK, // Oklahoma
    OR, // Oregon
    PA, // Pennsylvania
    RI, // Rhode Island
    SC, // South Carolina
    SD, // South Dakota
    TN, // Tennessee
    TX, // Texas
    UT, // Utah
    VT, // Vermont
    VA, // Virginia
    WA, // Washington
    WV, // West Virginia
    WI, // Wisconsin
    WY, // Wyoming
    OtherTerritory,
}
