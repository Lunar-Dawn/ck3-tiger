use crate::block::validator::Validator;
use crate::block::Block;
use crate::db::{Db, DbKind};
use crate::everything::Everything;
use crate::item::Item;
use crate::modif::{validate_modifs, ModifKinds};
use crate::validate::validate_color;
use crate::token::Token;

#[derive(Clone, Debug)]
pub struct LegionDistinctions {}

impl LegionDistinctions {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::LegionDistinctions, key, block, Box::new(Self {}));
    }
}

impl DbKind for LegionDistinctions {
    fn validate(&self, key: &Token, block: &Block, data: &Everything) {
        let mut vd = Validator::new(block, data);

        data.verify_exists(Item::Localization, key);
        let loca = format!("{key}_desc");
        data.verify_exists_implied(Item::Localization, &loca, key);

        vd.field_item("icon", Item::File);

        vd.field_validated_block("commander", |block, data| {
            let vd = Validator::new(block, data);
            validate_modifs(block, data, ModifKinds::Character, vd);
        });
        vd.field_validated_block("unit", |block, data| {
            let vd = Validator::new(block, data);
            validate_modifs(block, data, ModifKinds::Unit, vd);
        });
    }
}