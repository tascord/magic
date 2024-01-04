pub mod cards;
type Effect = Box<dyn FnMut(Game, Action)>;
trait Targetable<T> {}

struct Game {}
struct Action {
    source: Target<Card>,
    target: Target<T>,
}
struct Target<T>(Box<dyn FnMut() -> dyn Targetable<T>>);

enum Zone {
    Battlefield,
    Hand,
    Stack,
    Graveyard,
    Exile,
    Library,
}

enum Colour {
    White,
    Blue,
    Black,
    Red,
    Green,
    Colourless
}

enum Cost {
    GenericMana(u8),
    Mana(u8, Colour),
}

enum Trigger {
    Cast { caster: Target<Player>, spell: Target<Card> },
    ZoneChang { card: Target<Card>, from: Zone, to: Zone },
    Damage { damaged: Target<Card, Player>, damager: Target<Card>, amount: u8 },
    Dies { card: Target<Card> },
}

enum Ability {
    Activated(ActivatedAbility),
    Triggered(TriggeredAbility),
    Static(StaticAbility),
}

struct ActivatedAbility {
    cost: Cost,
    effect: Effect
}

struct TriggeredAbility {
    trigger: Trigger,
    effect: Effect
}

struct StaticAbility(Effect);

struct Card {
    name: String,
    abilities: Vec<Ability>,
    cost: Cost,
}

struct Player {
    life: u8,
    hand: Vec<Card>,
    library: Vec<Card>,
    graveyard: Vec<Card>,
    battlefield: Vec<Card>,
}

//

lazy_static::lazy_static! {
    static ref PROWESS: TriggeredAbility = TriggeredAbility {
        trigger: Trigger::Cast { caster: Target::Player, spell: Target::Card },
        effect: Box::new(
            |game: Game, action: Action| {
                if action.target == Target::Card {
                    let card = action.target();
                    if card.abilities.contains(Ability::Prowess) {
                        card.buff(1, 0);
                    }
                }
            }
        )
    };
}

pub fn main() {

    let card = Card {
        name: "Monastery Swiftspear".to_string(),
        abilities: vec![PROWESS],
        cost: Cost::Mana(1, Colour::Red)
    };

}