use crate::block::validator::Validator;
use crate::block::Block;
use crate::db::{Db, DbKind};
use crate::everything::Everything;
use crate::item::Item;
use crate::validate::validate_color;
use crate::token::Token;
use crate::tooltipped::Tooltipped;
use crate::trigger::validate_trigger;

#[derive(Clone, Debug)]
pub struct Ideas {}

impl Ideas {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::Ideas, key, block, Box::new(Self {}));
    }
}

impl DbKind for Ideas {
    fn validate(&self, key: &Token, block: &Block, data: &Everything) {
        let mut vd = Validator::new(block, data);

        data.verify_exists(Item::Localization, key);
        let loca = format!("idea_{key}_desc");
        data.verify_exists_implied(Item::Localization, &loca, key);

        vd.field_validated_block("trigger", |b, data| {
            validate_trigger(b, data, &mut sc, Tooltipped::No);
        });
        // TODO: confirm if this is correct?
        vd.field_item("modifier", Item::Modifier);
        vd.field_choice("group", &["military_ideas", "civic_ideas", "oratory_ideas", "religious_ideas"]);
        vd.field_item("soundeffect", Item::Sound);
    }
}