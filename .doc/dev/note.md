
# Settings

## Entity

### Stats

Visible

- Health `points`
- Health Regen `points/sec`
- Absorption `points`
- Defense `points`
- Strength `points`
- Mana `points`
- Mana Regen `points/sec`
- Intelligence `points`
- Critical Chance `percentage`
- Critical Damage `percentage`
- Attack Speed `percentage`
- Movement Speed `unit/sec`

Hidden

- Add. Multiplier `percentage`
- Mul. Multiplier `percentage`
- Ability Scaling `percentage`
- Bouns Modifier `percentage`
- Damage Reduction `percentage`

### Calculation

`Base Damage` =
- For Physical damage: `(5 + Weapon ATK) x (1 + Strength x 0.01)`
- For Magical damage: `(5 + Weapon Magic Damage) x (1 + Intelligence x 0.01)`
- For Ability damage: `Ability damage`
(Actually, most of the ability damage are some term like "80% of Weapon ATK" or "80% of Weapon Magic Damage")

`Actual Damage` =
- `Normal Damage` = `Base Damage x (1 + Add. Multiplier) x (1 + Mul. Multiplier) x (1 + Bouns Modifier)`
- `Critical Damage` = `Normal Damage x (1 + Critical Damage)` If is a critical hit
- `Ability Damage` = `Base Damage x Ability Scaling` If is an ability

`Final Damage` = `Normal Damage x (1 - Damage Reduction)`
