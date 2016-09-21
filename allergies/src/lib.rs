#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

static ITEMS: [(u32, Allergen); 8] = [(1, Allergen::Eggs),
                                      (2, Allergen::Peanuts),
                                      (4, Allergen::Shellfish),
                                      (8, Allergen::Strawberries),
                                      (16, Allergen::Tomatoes),
                                      (32, Allergen::Chocolate),
                                      (64, Allergen::Pollen),
                                      (128, Allergen::Cats)];

const MAX_UID: u32 = 255;

pub struct Allergies {
    allergies_id: Vec<Allergen>,
}

fn get_allergies_from(allergen_uid: u32) -> Vec<Allergen> {

    let mut m_allergen_uid = allergen_uid;
    let mut allergies_id = Vec::with_capacity(8);
    for item in ITEMS.iter().rev() {
        let &(current_id, current_item) = item;
        if m_allergen_uid < current_id {
            continue;
        }
        m_allergen_uid -= current_id;
        allergies_id.push(current_item.clone());
    }
    allergies_id

}

impl Allergies {
    pub fn new(allergen_uid: u32) -> Self {
        let allergen_uid = allergen_uid & MAX_UID;
        let allergies_id = get_allergies_from(allergen_uid);
        Allergies {
            allergies_id: allergies_id,
        }
    }
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies_id.contains(allergen)
    }
    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies_id.clone()
    }
}
