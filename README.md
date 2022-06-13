# What if Robin had actually been finished being coded.
--------------------------------------------------------------------------
Change Log:
- spells and Levin-sword charge isn't consumed until the attack actually comes out
- spells and Levin-sword have a pasive recharge:
	* pasive recharge is ~2.5x slower than the empty recharge
	* spells/Levin won't recharge if said spell/Levin is being used
	* if thunder is stored, it won't pasively recharge
- spells and Levin-sword have 20 total use points:
	* as long as their is at least 1 point left the attack will come out
- adjusted spell and Levin uses:
	* n-air 1 & 2: -1
	* all other sword swings: -3
	* neutral-special: unchanged
	* side-special: -4
	* jab-gentleman: -1
	* up-special 1 & 2: -1
	* rapid-jab start, hold, and finish: -1 (each)
	* down-special: -7
	* grab-pummel: -1
- Robin starts match with Levin-sword (not sure why this isn't already a thing)
- Robin no longer tosses a depleted Levin-sword during the move and instead will wait till the move ends (like spell-books)
- Robin only uses Levin-sword aerials with smash inputs (this means the only way to use Levin N-air is with AB-smash enabled)
- up-tilt starts one frame earlier to actually match the animation: 6 -> 5
- down-tilt adjusted bkb and angle for combos: kbg:40, angle:30 -> kbg:60, angle:68
- dash-attack increased range by changing animation to extend arm further out (less shank more thrust)
- fixed Levin f-smash effects not matching active frames
- moved hit-box slightly forward on Bronze (normal) f-smash to better match animation: units 8-14 -> units 10-16
- fixed Levin up-smash lingering flare effect not being removed
- adjusted begining auto-cancel window on aerials to match new Levin usage:
	* f-air: 4-27 -> 10-27
	* b-air: 0-32 -> 7-32
	* u-air: 5-27 -> 9-27
	* d-air: 2-48 -> 11-48
	* n-air: 4-34 -> 6-34
- Bronze (normal) up-air active frames lasts one frame longer to better match the animation: 10-14 -> 10-15
- fixed Levin n-air's second hit lingering flare effect not matching active frames 
- increased pummel damage: 1.5 -> 1.6
- pummel requires and consumes dark magic (creates more opportunities for decision making)
- split up-special into two parts:
	* up-special-1 doesn't put Robin into free-fall and can only be used once per air time unless hit
	* up-special-2 behaves normally and will put Robin into free-fall
	* if up-special fails due to lack of wind-magic, Robin is not put into free-fall
- adjusted down-special active frames to comes out faster and lasts longer: 15-18 -> 8-24
- incresed down-special total frames: 49 -> 54
- if down-special is successful:
	* the enemy is inflicted with the "curse" effect
	* Robin fully recharges spells and Levin-sword excluding Rizaia
	* Robin can use up-special-1 again if it was already used
	* Robin is no longer put into special-fall (why was that a thing before)
- down-special revival time: 1800 -> 780
- holding side-taunt will deplete the current spell-book and give it as an item
	* if Robin is already holding an item he will toss the book instead (like if he were to normally use up a spell-book)
- up-taunt swaps the current sword if available (Levin <-> Bronze)
