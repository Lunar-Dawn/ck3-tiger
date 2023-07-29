use crate::block::validator::Validator;
use crate::block::Block;
use crate::db::{Db, DbKind};
use crate::everything::Everything;
use crate::context::ScopeContext;
use crate::item::Item;
use crate::token::Token;
use crate::tooltipped::Tooltipped;
use crate::trigger::validate_trigger;
use crate::effect::validate_effect;

#[derive(Clone, Debug)]
pub struct Ambition {}

impl Ambition {
    pub fn add(db: &mut Db, key: Token, block: Block) {
        db.add(Item::Ambition, key, block, Box::new(Self {}));
    }
}

impl DbKind for Ambition {
    fn validate(&self, key: &Token, block: &Block, data: &Everything) {
        let mut vd = Validator::new(block, data);


        data.verify_exists(Item::Localization, key);
        let loca = format!("{key}_desc");
        data.verify_exists_implied(Item::Localization, &loca, key);

        vd.field_validated_block("can_be_picked", |b, data| {
            validate_trigger(b, data, &mut sc, Tooltipped::No);
        });
        vd.field_validated_block("finished_when", |b, data| {
            validate_trigger(b, data, &mut sc, Tooltipped::Yes);
        });
        vd.field_validated_block("abort", |b, data| {
            validate_trigger(b, data, &mut sc, Tooltipped::No);
        });
        vd.field_validated_block("chance", |b, data| {
            validate_trigger(b, data, &mut sc, Tooltipped::No);
        });

        vd.field_value("duration");
        vd.field_bool("content");
    }
}