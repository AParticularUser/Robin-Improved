# What if Robin was actually finished being coded.
--------------------------------------------------------------------------
Robin has always felt under-baked. And after looking through his scripts I can confirm this is the case. 
This mod aims to correct minor errors in vinilla while also adding new features.

## Change Log

### GENERAL
- Robin starts match with Levin-sword (not sure why this isn't already a thing)
- Tomes and Levin-Sword charge aren't consumed until the attack actually comes out
- Tomes and Levin-Sword have a passive recharge:
	- passive recharge is ~2.5x slower than the empty recharge
	- Tomes/Levin won't recharge if said Tome/Levin is being used
- Tomes and Levin-Sword have 20 total use points:
	- as long as their is at least 1 point left the attack will come out
- adjusted Tome and Levin uses:
	- Neutral-Air 1 & 2: -1
	- all other Aerials: -3
	- all Smashes: -3
	- Neutral-Special: -unchanged-
	- Side-Special: -4
	- Jab-3: -1
	- Up-Special 1 & 2: -1
	- Rapid-Jab start, hold, and finish: -1 (each)
	- Down-Special start: -6
	- Down-Special hold: -1
	- Grab-Pummel: -1
- Robin no longer tosses a depleted Levin-sword during the move and instead will wait till the move ends (like Tomes)
- fixed Levin attack effects not matching active frames

### AERIALS
- increased begining auto-cancel window on aerials:
	- n-air: 4-34 -> 6-34
	- f-air: 4-27 -> 10-27
	- b-air: 0-32 -> 7-32
	- u-air: 5-27 -> 9-27
	- d-air: 2-48 -> 11-48
- Robin only uses Levin-sword aerials with smash inputs
- A+B Smash is always on for Neutral-Air (otherwise there wouldn't be way to use Levin N-air without A+B-smash enabled)
- Bronze (normal) up-air active frames lasts one frame longer to better match the animation: 10-14 -> 10-15

### TILTS
- up-tilt starts one frame earlier to actually match the animation: 6 -> 5
- down-tilt adjusted kbg and angle for combos: kbg:40, angle:30 -> kbg:60, angle:68

### DASH ATTACK
- dash-attack increased range by changing animation to extend arm further out (less shank more thrust)

### SMASHES
- moved hit-box slightly forward on Forward-Smash to better match animation:
	- Bronze: units 8-14 -> units 8-16
	- Levin (late): units 2-8 -> units 2-9

### SPECIALS
- split Up-Special into two parts:
	- Up-Special-1 doesn't put Robin into free-fall and can only be used once per air time unless hit
	- Up-Special-2 behaves normally and will put Robin into free-fall
- Added missing animation for the grounded version of Up-Special failing
- if Side-Special fails, it can be acted out of sooner (faf:44)
- Down-Special comes out faster: frame 15 -> frame 8
- holding Special continues the grab and consumes a point every 20 frames
- if Down-Special is successful:
	- the enemy is inflicted with the "curse" effect
	- Robin fully recharges Levin-sword and Tomes excluding Rizaia
	- Robin can use Up-Special-1 again if it was already used
	- Robin is no longer put into free-fall (why was that a thing before)
- Down-Special revival time: 1800 -> 780

### OTHER
- Pummel requires and consumes dark magic
- increased Pummel damage: 1.5 -> 1.6
- holding Side-Taunt will deplete the current Tome and give it as an item
	- if Robin is already holding an item he will toss the Tome instead (like if he were to normally use up a Tome)
- Up-Taunt swaps the current sword if available (Levin <-> Bronze)

